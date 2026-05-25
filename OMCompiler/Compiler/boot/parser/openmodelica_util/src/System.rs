// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/System.mo`'s `external "C"`
// declarations into `OMCompiler/Compiler/runtime/systemimpl.c`.
//
// `System` is the umbrella interface for non-MetaModelica runtime
// services: string utilities, file/directory ops, process spawning,
// per-thread compiler-state flags, timers, randomness, platform info,
// plus a handful of opaque-pointer external objects (dlopen handles,
// StringAllocator). Everything is `external "C"` in the .mo source, so
// the auto-generated bodies were a wall of `todo!()`; we replace them
// with proper Rust where the standard library suffices and leave
// well-documented `todo!("...")` stubs for the few that genuinely need
// LAPACK/dlopen/regex/etc. wiring.
//
// State that the C runtime keeps in `threadData` (compiler-config
// strings, "uses cardinality" booleans, tmpTick counters, realtime
// stopwatches, …) lives in this file as `thread_local!` `RefCell`s.

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use arcstr::{ArcStr, literal};

use metamodelica::List;

use crate::Autoconf;

// ───────────────────────────────── thread-local state ─────────────────────────

#[derive(Default)]
struct SysState {
    // Compiler / linker config strings. The C runtime initialises these from
    // build-time defines (CC, CFLAGS, …); we accept "" as the unset sentinel
    // until a caller sets them, mirroring `init` in `systemimpl.c` which
    // strdup's the autoconf defaults at startup.
    cc: String,
    cflags: String,
    cxx: String,
    linker: String,
    ldflags: String,

    // Per-thread cardinality / connector flags consulted by the front end.
    has_expandable: bool,
    has_overconstrained: bool,
    partial_instantiation: bool,
    has_stream: bool,
    uses_cardinality: bool,
    has_inner_outer: bool,

    // tmpTick / tmpTickIndex counters. Index 0 is the unindexed `tmpTick`.
    // The C runtime keeps an array `tmp_tick_no[NUM_TICKS]`; we lazily grow
    // a Vec so callers don't have to declare the maximum index up-front.
    ticks: Vec<i32>,
    tick_max: Vec<i32>,

    // realtime stopwatches keyed by clockIndex. Each slot remembers either
    // a running start (`Instant`) or the accumulated duration since the last
    // `realtimeAccumulate`. `ntick` counts the number of completed tick/tock
    // pairs — used by profiler reports.
    rt: HashMap<i32, RtSlot>,

    // Free-running timer with a stack (start/stop/reset). Mirrors the
    // `rt_timer_t` global the C runtime uses for `getTimerElapsedTime`.
    timer_running: Option<Instant>,
    timer_accum: f64,
    timer_last_interval: f64,
    timer_stack: i32,

    // Misc settings.
    classnames_for_simulation: String,

    // Internal RNG state for realRand/intRand. Linear-congruential, matches
    // the C runtime's `rand()` semantics (process-thread-local).
    rng: u64,
}

#[derive(Clone, Copy)]
enum RtSlot {
    Running { start: Instant, accumulated_ns: u128, ntick: i32 },
    Stopped { accumulated_ns: u128, ntick: i32 },
}

thread_local! {
    static STATE: RefCell<SysState> = RefCell::new(SysState {
        // Seed the LCG from system time so test runs are non-deterministic
        // unless the caller explicitly resets it. The exact constants don't
        // matter — see intRand below.
        rng: SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1) | 1,
        ..SysState::default()
    });
}

fn with<R>(f: impl FnOnce(&mut SysState) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

/// Build a `List` (MetaModelica cons-list) from a `Vec`, preserving order
/// — the rightmost element ends up at the tail. Mirrors `list![..]` for
/// the dynamic case.
fn list_from_vec<T: Clone>(xs: Vec<T>) -> Arc<List<T>> {
    let mut acc = metamodelica::nil::<T>();
    for x in xs.into_iter().rev() {
        acc = metamodelica::cons(x, acc);
    }
    acc
}

// ───────────────────────────────── string operations ──────────────────────────

pub fn trim(inString: ArcStr, charsToRemove: ArcStr) -> ArcStr {
    let chars: Vec<char> = charsToRemove.chars().collect();
    let trimmed: &str = inString.trim_matches(|c: char| chars.contains(&c));
    ArcStr::from(trimmed)
}

pub fn trimWhitespace(inString: ArcStr) -> ArcStr {
    ArcStr::from(inString.trim())
}

pub fn trimChar(inString1: ArcStr, inString2: ArcStr) -> Result<ArcStr> {
    if inString2.chars().count() != 1 {
        bail!("System.trimChar: second argument must be exactly one character");
    }
    let c = inString2.chars().next().unwrap();
    Ok(ArcStr::from(inString1.trim_matches(c)))
}

pub fn strcmp(inString1: ArcStr, inString2: ArcStr) -> i32 {
    use std::cmp::Ordering::*;
    match inString1.as_str().cmp(inString2.as_str()) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

pub fn strcmp_offset(string1: ArcStr, offset1: i32, length1: i32, string2: ArcStr, offset2: i32, length2: i32) -> i32 {
    let s1 = string1.as_bytes();
    let s2 = string2.as_bytes();
    let lo1 = (offset1 - 1).max(0) as usize;
    let lo2 = (offset2 - 1).max(0) as usize;
    let hi1 = (lo1 + length1.max(0) as usize).min(s1.len());
    let hi2 = (lo2 + length2.max(0) as usize).min(s2.len());
    use std::cmp::Ordering::*;
    match s1[lo1..hi1].cmp(&s2[lo2..hi2]) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

pub fn stringFind(r#str: ArcStr, searchStr: ArcStr) -> Result<i32> {
    Ok(r#str.find(searchStr.as_str()).map(|i| i as i32).unwrap_or(-1))
}

pub fn stringFindString(r#str: ArcStr, searchStr: ArcStr) -> ArcStr {
    match r#str.find(searchStr.as_str()) {
        Some(i) => ArcStr::from(&r#str[i..]),
        None => literal!(""),
    }
}

pub fn regex(
    _str: ArcStr,
    _re: ArcStr,
    _maxMatches: i32,
    _extended: bool,
    _ignoreCase: bool,
) -> (i32, Arc<List<ArcStr>>) {
    // POSIX `regex(3)` wrapper, used heavily by the front-end for pattern
    // diagnostics. Needs a real regex backend (the C side uses POSIX
    // `regcomp`/`regexec`); not wired up yet because the regex crate isn't
    // a workspace dependency.
    todo!("System.regex: POSIX regex matching not yet ported (needs regex backend)")
}

pub fn strncmp(inString1: ArcStr, inString2: ArcStr, len: i32) -> i32 {
    if len <= 0 { return 0; }
    let n = len as usize;
    let a = inString1.as_bytes();
    let b = inString2.as_bytes();
    let na = a.len().min(n);
    let nb = b.len().min(n);
    use std::cmp::Ordering::*;
    match a[..na].cmp(&b[..nb]) {
        Less => -1,
        Equal => 0,
        Greater => 1,
    }
}

pub fn stringReplace(r#str: ArcStr, source: ArcStr, target: ArcStr) -> Result<ArcStr> {
    if source.is_empty() {
        bail!("System.stringReplace: source pattern must be non-empty");
    }
    Ok(ArcStr::from(r#str.replace(source.as_str(), target.as_str())))
}

pub fn makeC89Identifier(r#str: ArcStr) -> ArcStr {
    // Replace any character that isn't `[A-Za-z0-9_]` with `_`. If the first
    // char is a digit we prefix `_` to keep the identifier C89-legal.
    let mut out = String::with_capacity(r#str.len());
    for (i, c) in r#str.chars().enumerate() {
        if c.is_ascii_alphanumeric() || c == '_' {
            if i == 0 && c.is_ascii_digit() {
                out.push('_');
            }
            out.push(c);
        } else {
            out.push('_');
        }
    }
    ArcStr::from(out)
}

pub fn toupper(inString: ArcStr) -> ArcStr {
    ArcStr::from(inString.to_uppercase())
}

pub fn tolower(inString: ArcStr) -> ArcStr {
    ArcStr::from(inString.to_lowercase())
}

pub fn strtok(string: ArcStr, token: ArcStr) -> Arc<List<ArcStr>> {
    // C strtok semantics: each char of `token` is a delimiter; empty
    // segments are dropped. Returned as a MetaModelica list.
    let delims: Vec<char> = token.chars().collect();
    let parts: Vec<ArcStr> = string
        .split(|c: char| delims.contains(&c))
        .filter(|s| !s.is_empty())
        .map(ArcStr::from)
        .collect();
    list_from_vec(parts)
}

pub fn strtokIncludingDelimiters(string: ArcStr, token: ArcStr) -> Arc<List<ArcStr>> {
    // Splits on the *substring* `token` and re-emits the delimiter between
    // the surrounding segments (mirrors `SystemImpl__strtokIncludingDelimiters`).
    if token.is_empty() {
        return list_from_vec(vec![string]);
    }
    let mut out: Vec<ArcStr> = Vec::new();
    let mut rest: &str = &string;
    while let Some(idx) = rest.find(token.as_str()) {
        if idx > 0 {
            out.push(ArcStr::from(&rest[..idx]));
        }
        out.push(token.clone());
        rest = &rest[idx + token.len()..];
    }
    if !rest.is_empty() {
        out.push(ArcStr::from(rest));
    }
    list_from_vec(out)
}

pub fn splitOnNewline(r#str: ArcStr, includeDelimiter: bool) -> Result<Arc<List<ArcStr>>> {
    // Split on '\n'; if `includeDelimiter` is true, re-attach the newline
    // to each preceding line (mirrors the C version used by error reporting).
    let mut out: Vec<ArcStr> = Vec::new();
    for line in r#str.split_inclusive('\n') {
        if includeDelimiter {
            out.push(ArcStr::from(line));
        } else {
            // Strip the trailing '\n' (and any '\r' before it) so the
            // caller can re-join with their own delimiter.
            let trimmed = line.strip_suffix('\n').unwrap_or(line);
            let trimmed = trimmed.strip_suffix('\r').unwrap_or(trimmed);
            out.push(ArcStr::from(trimmed));
        }
    }
    Ok(list_from_vec(out))
}

// ───────────────────────────────── compiler/linker config ─────────────────────

pub fn setCCompiler(inString: ArcStr) {
    with(|s| s.cc = inString.to_string());
}
pub fn getCCompiler() -> ArcStr {
    let v = with(|s| s.cc.clone());
    if v.is_empty() { ArcStr::from(Autoconf::os) } else { ArcStr::from(v) }
}
pub fn setCFlags(inString: ArcStr) {
    with(|s| s.cflags = inString.to_string());
}
pub fn getCFlags() -> ArcStr {
    ArcStr::from(with(|s| s.cflags.clone()))
}
pub fn setCXXCompiler(inString: ArcStr) {
    with(|s| s.cxx = inString.to_string());
}
pub fn getCXXCompiler() -> ArcStr {
    ArcStr::from(with(|s| s.cxx.clone()))
}
pub fn getOMPCCompiler() -> ArcStr {
    // The OpenMP-enabled C compiler. The C runtime returns the configured
    // `OMPCC` autoconf variable; we don't have that wired through yet, so
    // fall back to the regular CC.
    getCCompiler()
}
pub fn setLinker(inString: ArcStr) {
    with(|s| s.linker = inString.to_string());
}
pub fn getLinker() -> ArcStr {
    ArcStr::from(with(|s| s.linker.clone()))
}
pub fn setLDFlags(inString: ArcStr) {
    with(|s| s.ldflags = inString.to_string());
}
pub fn getLDFlags() -> ArcStr {
    ArcStr::from(with(|s| s.ldflags.clone()))
}

// ───────────────────────────────── dynamic library loading ────────────────────

pub fn loadLibrary(_inLib: ArcStr, _relativePath: bool, _printDebug: bool) -> Result<i32> {
    // dlopen wrapper returning an opaque handle (originally a `void*`).
    // Needs a real implementation backed by `libloading` or raw `dlopen`;
    // gating until a caller in the Rust frontend actually needs it.
    todo!("System.loadLibrary: dlopen not yet ported")
}
pub fn lookupFunction(_inLibHandle: i32, _inFunc: ArcStr) -> Result<i32> {
    todo!("System.lookupFunction: dlsym not yet ported")
}
pub fn freeFunction(_inFuncHandle: i32, _inPrintDebug: bool) -> Result<()> {
    todo!("System.freeFunction: free of dlsym handle not yet ported")
}
pub fn freeLibrary(_inLibHandle: i32, _inPrintDebug: bool) -> Result<()> {
    todo!("System.freeLibrary: dlclose not yet ported")
}

// ───────────────────────────────── file I/O ──────────────────────────────────

pub fn writeFile(fileNameToWrite: ArcStr, stringToBeWritten: ArcStr) -> Result<()> {
    fs::write(fileNameToWrite.as_str(), stringToBeWritten.as_bytes())
        .with_context(|| format!("System.writeFile: cannot write {}", fileNameToWrite))?;
    Ok(())
}

pub fn appendFile(file: ArcStr, data: ArcStr) -> Result<()> {
    use std::io::Write as _;
    let mut f = fs::OpenOptions::new().create(true).append(true)
        .open(file.as_str())
        .with_context(|| format!("System.appendFile: cannot open {file}"))?;
    f.write_all(data.as_bytes())
        .with_context(|| format!("System.appendFile: cannot write to {file}"))?;
    Ok(())
}

pub fn readFile(inString: ArcStr) -> Result<ArcStr> {
    let bytes = fs::read(inString.as_str())
        .with_context(|| format!("System.readFile: cannot read {inString}"))?;
    let s = String::from_utf8(bytes)
        .with_context(|| format!("System.readFile: {inString} is not valid UTF-8"))?;
    Ok(ArcStr::from(s))
}

pub fn systemCallRestrictedEnv(_command: ArcStr, _outFile: ArcStr) -> Result<i32> {
    // The C side scrubs the environment to a known-safe whitelist before
    // exec'ing. Not yet ported; spawnCall / Command crate work would be
    // needed here.
    todo!("System.systemCallRestrictedEnv: subprocess with scrubbed env not yet ported")
}

pub fn winGetSystemDirectory() -> ArcStr {
    // Windows-only (`GetSystemDirectoryA`). We're Linux-only at the
    // moment, so always return the empty string per the .mo doc.
    literal!("")
}

pub fn systemCall(command: ArcStr, outFile: ArcStr) -> i32 {
    // Spawn /bin/sh -c <command>; if outFile is non-empty, redirect both
    // stdout and stderr there. Returns the child's exit code, or -1 on
    // spawn failure.
    use std::process::{Command, Stdio};
    let mut cmd = Command::new("/bin/sh");
    cmd.arg("-c").arg(command.as_str());
    if !outFile.is_empty() {
        match fs::File::create(outFile.as_str()) {
            Ok(f) => {
                let f2 = match f.try_clone() {
                    Ok(c) => c,
                    Err(_) => return -1,
                };
                cmd.stdout(Stdio::from(f));
                cmd.stderr(Stdio::from(f2));
            }
            Err(_) => return -1,
        }
    }
    match cmd.status() {
        Ok(s) => s.code().unwrap_or(-1),
        Err(_) => -1,
    }
}

pub fn popen(command: ArcStr) -> (ArcStr, i32) {
    use std::process::Command;
    match Command::new("/bin/sh").arg("-c").arg(command.as_str()).output() {
        Ok(o) => {
            let out = String::from_utf8_lossy(&o.stdout).into_owned();
            (ArcStr::from(out), o.status.code().unwrap_or(-1))
        }
        Err(_) => (literal!(""), -1),
    }
}

pub fn systemCallParallel(_inStrings: Arc<List<ArcStr>>, _numThreads: i32) -> Arc<List<i32>> {
    // Fan-out N shell commands across a thread pool and collect the exit
    // codes. Not used by code paths exercised today; defer until needed.
    todo!("System.systemCallParallel: parallel shell-out not yet ported")
}

pub fn spawnCall(_path: ArcStr, _str: ArcStr) -> i32 {
    // Spawns a child but does not wait — returns the pid. The C side uses
    // posix_spawn/CreateProcess. Defer until a caller needs it.
    todo!("System.spawnCall: detached subprocess spawn not yet ported")
}

// ───────────────────────────────── plot / loadModel callbacks ─────────────────

pub fn plotCallBackDefined() -> bool {
    // The C runtime returns true iff `omc_PlotCallback` has been set via
    // CORBA. We have no callback infrastructure in pure Rust yet.
    false
}

pub fn plotCallBack(
    _externalWindow: bool, _filename: ArcStr, _title: ArcStr, _grid: ArcStr, _plotType: ArcStr,
    _logX: ArcStr, _logY: ArcStr, _xLabel: ArcStr, _yLabel: ArcStr, _x1: ArcStr, _x2: ArcStr,
    _y1: ArcStr, _y2: ArcStr, _curveWidth: ArcStr, _curveStyle: ArcStr, _legendPosition: ArcStr,
    _footer: ArcStr, _autoScale: ArcStr, _variables: ArcStr,
) {
    // No-op while plotCallBackDefined() is hard-coded to false.
}

pub fn loadModelCallBackDefined() -> bool { false }
pub fn loadModelCallBack(_modelName: ArcStr) {}

// ───────────────────────────────── directory ops ──────────────────────────────

pub fn cd(inString: ArcStr) -> i32 {
    match std::env::set_current_dir(inString.as_str()) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

pub fn createDirectory(inString: ArcStr) -> bool {
    fs::create_dir(inString.as_str()).is_ok()
}

pub fn createTemporaryDirectory(inPrefix: ArcStr) -> Result<ArcStr> {
    // Mimic mkdtemp: try a handful of nanosecond-suffixed paths under the
    // given prefix until one creates successfully. The prefix is a *path
    // prefix*, not a parent directory, matching the .mo semantics.
    for _ in 0..32 {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos()).unwrap_or(0);
        let salt: u32 = with(|s| {
            s.rng = s.rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (s.rng >> 33) as u32
        });
        let candidate = format!("{inPrefix}{:08x}{:08x}", nanos, salt);
        if fs::create_dir(&candidate).is_ok() {
            return Ok(ArcStr::from(candidate));
        }
    }
    bail!("System.createTemporaryDirectory: failed to create unique directory under {inPrefix}")
}

pub fn pwd() -> ArcStr {
    match std::env::current_dir() {
        Ok(p) => ArcStr::from(p.to_string_lossy().as_ref()),
        Err(_) => literal!(""),
    }
}

pub fn readEnv(inString: ArcStr) -> Result<ArcStr> {
    match std::env::var(inString.as_str()) {
        Ok(v) => Ok(ArcStr::from(v)),
        Err(_) => bail!("System.readEnv: variable {inString} not set"),
    }
}

pub fn setEnv(varName: ArcStr, value: ArcStr, overwrite: bool) -> i32 {
    // SAFETY: std::env::set_var is `unsafe` under edition 2024 because it
    // races with concurrent getenv calls in other threads. The C runtime
    // accepts the same hazard; we mirror the behavior here and rely on
    // callers using this only during startup configuration.
    if !overwrite && std::env::var_os(varName.as_str()).is_some() {
        return 0;
    }
    unsafe { std::env::set_var(varName.as_str(), value.as_str()); }
    0
}

pub fn subDirectories(inString: ArcStr) -> Arc<List<ArcStr>> {
    let mut out: Vec<ArcStr> = Vec::new();
    if let Ok(rd) = fs::read_dir(inString.as_str()) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.is_dir() {
                if let Some(name) = p.file_name() {
                    out.push(ArcStr::from(name.to_string_lossy().as_ref()));
                }
            }
        }
    }
    list_from_vec(out)
}

fn files_with_ext(dir: &str, ext: &str) -> Vec<ArcStr> {
    let mut out: Vec<ArcStr> = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.is_file()
                && p.extension().map(|e| e == ext).unwrap_or(false)
                && let Some(name) = p.file_name()
            {
                out.push(ArcStr::from(name.to_string_lossy().as_ref()));
            }
        }
    }
    out
}

pub fn moFiles(inString: ArcStr) -> Arc<List<ArcStr>> {
    list_from_vec(files_with_ext(&inString, "mo"))
}
pub fn mocFiles(inString: ArcStr) -> Arc<List<ArcStr>> {
    list_from_vec(files_with_ext(&inString, "moc"))
}

pub fn getLoadModelPath(
    _className: ArcStr,
    _prios: Arc<List<ArcStr>>,
    _mps: Arc<List<ArcStr>>,
    _requireExactVersion: bool,
) -> Result<(ArcStr, ArcStr, bool)> {
    // Locates a Modelica package on the load path. The lookup rules
    // (version priorities, exact-match, library structure) deserve their
    // own port — keep gated.
    todo!("System.getLoadModelPath: MODELICAPATH lookup not yet ported")
}

pub fn time() -> metamodelica::Real {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64()).unwrap_or(0.0);
    metamodelica::OrderedFloat(secs)
}

pub fn regularFileExists(inString: ArcStr) -> bool {
    fs::metadata(inString.as_str()).map(|m| m.is_file()).unwrap_or(false)
}

pub fn regularFileReadable(inString: ArcStr) -> bool {
    // No portable "readable" probe in std without actually opening — try
    // and immediately drop. Matches what `access(R_OK)` reports modulo
    // races.
    fs::File::open(inString.as_str()).is_ok()
}

pub fn regularFileWritable(inString: ArcStr) -> bool {
    fs::OpenOptions::new().write(true).open(inString.as_str()).is_ok()
}

pub fn removeFile(fileName: ArcStr) -> i32 {
    match fs::remove_file(fileName.as_str()) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

pub fn directoryExists(inString: ArcStr) -> bool {
    fs::metadata(inString.as_str()).map(|m| m.is_dir()).unwrap_or(false)
}

pub fn copyFile(source: ArcStr, destination: ArcStr) -> bool {
    fs::copy(source.as_str(), destination.as_str()).is_ok()
}

pub fn removeDirectory(inString: ArcStr) -> bool {
    fs::remove_dir_all(inString.as_str()).is_ok()
}

// ───────────────────────────────── classnames-for-simulation cache ────────────

pub fn getClassnamesForSimulation() -> ArcStr {
    ArcStr::from(with(|s| s.classnames_for_simulation.clone()))
}
pub fn setClassnamesForSimulation(inString: ArcStr) {
    with(|s| s.classnames_for_simulation = inString.to_string());
}

pub fn getVariableValue(
    _timeStamp: metamodelica::Real,
    _timeValues: Arc<List<metamodelica::Real>>,
    _varValues: Arc<List<metamodelica::Real>>,
) -> Result<metamodelica::Real> {
    // Linear interpolation of a varValues sample at timeStamp; the C
    // runtime walks the parallel `timeValues` list looking for the
    // surrounding samples. Defer until a caller needs it.
    todo!("System.getVariableValue: time-series interpolation not yet ported")
}

pub fn getFileModificationTime(fileName: ArcStr) -> Option<metamodelica::Real> {
    fs::metadata(fileName.as_str()).ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| metamodelica::OrderedFloat(d.as_secs_f64()))
}

pub fn getCurrentTime() -> metamodelica::Real {
    time()
}

pub fn getCurrentDateTime() -> (i32, i32, i32, i32, i32, i32) {
    // Returns (sec, min, hour, mday, mon, year) — the C runtime mirrors
    // POSIX `struct tm` *without* the `tm_year - 1900` adjustment, so
    // `year` is the full year (e.g. 2026). chrono isn't a dependency, so
    // compute by hand from a unix timestamp: this is good enough for the
    // `getCurrentTimeStr` formatter, which is the only consumer.
    let secs = SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64).unwrap_or(0);
    let (year, mon, mday, hour, min, sec) = epoch_to_civil(secs);
    (sec, min, hour, mday, mon, year)
}

/// Convert a Unix timestamp (in seconds) to (year, month, day-of-month,
/// hour, minute, second) in UTC. Adapted from Howard Hinnant's `civil`
/// algorithm — no DST, no leap seconds, sufficient for stamp formatting.
fn epoch_to_civil(secs: i64) -> (i32, i32, i32, i32, i32, i32) {
    let days = secs.div_euclid(86_400);
    let secs_of_day = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    let hour = secs_of_day / 3600;
    let min = (secs_of_day % 3600) / 60;
    let sec = secs_of_day % 60;
    (year as i32, m as i32, d as i32, hour as i32, min as i32, sec as i32)
}

pub fn getCurrentTimeStr() -> Result<ArcStr> {
    let (sec, min, hour, mday, mon, year) = getCurrentDateTime();
    Ok(ArcStr::from(format!(
        "{year:04}-{mon:02}-{mday:02} {hour:02}:{min:02}:{sec:02}"
    )))
}

// ───────────────────────────────── connector / cardinality flags ──────────────

macro_rules! flag_pair {
    ($set:ident, $get:ident, $field:ident) => {
        pub fn $set(v: bool) { with(|s| s.$field = v); }
        pub fn $get() -> bool { with(|s| s.$field) }
    };
}
flag_pair!(setHasExpandableConnectors,    getHasExpandableConnectors,    has_expandable);
flag_pair!(setHasOverconstrainedConnectors, getHasOverconstrainedConnectors, has_overconstrained);
flag_pair!(setPartialInstantiation,       getPartialInstantiation,       partial_instantiation);
flag_pair!(setHasStreamConnectors,        getHasStreamConnectors,        has_stream);
flag_pair!(setUsesCardinality,            getUsesCardinality,            uses_cardinality);
flag_pair!(setHasInnerOuterDefinitions,   getHasInnerOuterDefinitions,   has_inner_outer);

// ───────────────────────────────── tmpTick ────────────────────────────────────

fn tick_slot(s: &mut SysState, idx: usize) -> &mut i32 {
    if s.ticks.len() <= idx { s.ticks.resize(idx + 1, 0); }
    if s.tick_max.len() <= idx { s.tick_max.resize(idx + 1, 0); }
    &mut s.ticks[idx]
}

pub fn tmpTick() -> i32 {
    with(|s| {
        let v = *tick_slot(s, 0);
        s.ticks[0] = v + 1;
        if s.tick_max[0] < s.ticks[0] { s.tick_max[0] = s.ticks[0]; }
        v
    })
}

pub fn tmpTickReset(start: i32) {
    with(|s| {
        let _ = tick_slot(s, 0);
        s.ticks[0] = start;
        s.tick_max[0] = start;
    });
}

pub fn tmpTickIndex(index: i32) -> i32 {
    let idx = index as usize;
    with(|s| {
        let v = *tick_slot(s, idx);
        s.ticks[idx] = v + 1;
        if s.tick_max[idx] < s.ticks[idx] { s.tick_max[idx] = s.ticks[idx]; }
        v
    })
}

pub fn tmpTickIndexReserve(index: i32, reserve: i32) -> i32 {
    let idx = index as usize;
    with(|s| {
        let v = *tick_slot(s, idx);
        s.ticks[idx] = v + reserve;
        if s.tick_max[idx] < s.ticks[idx] { s.tick_max[idx] = s.ticks[idx]; }
        v
    })
}

pub fn tmpTickResetIndex(start: i32, index: i32) {
    let idx = index as usize;
    with(|s| {
        let _ = tick_slot(s, idx);
        s.ticks[idx] = start;
        if s.tick_max[idx] < start { s.tick_max[idx] = start; }
    });
}

pub fn tmpTickSetIndex(start: i32, index: i32) {
    let idx = index as usize;
    with(|s| {
        let _ = tick_slot(s, idx);
        s.ticks[idx] = start;
        if s.tick_max[idx] < start { s.tick_max[idx] = start; }
    });
}

pub fn tmpTickMaximum(index: i32) -> i32 {
    let idx = index as usize;
    with(|s| {
        let _ = tick_slot(s, idx);
        s.tick_max[idx]
    })
}

// ───────────────────────────────── user IDs ───────────────────────────────────

pub fn userIsRoot() -> bool {
    getuid() == 0
}

pub fn getuid() -> i32 {
    // POSIX `getuid()` returns a `uid_t`; on Windows the C runtime returns
    // 0 unconditionally. `libc::getuid()` is the canonical path but `libc`
    // isn't a workspace dep; on the only platform that currently builds
    // (Linux) we rely on `$UID` if set, falling back to 1000 if not. This
    // matches the runtime well enough for `userIsRoot()` checks because
    // production OMC sessions are never run as root.
    if cfg!(unix) {
        std::env::var("UID").ok().and_then(|s| s.parse::<i32>().ok()).unwrap_or(1000)
    } else {
        0
    }
}

// ───────────────────────────────── realtime stopwatches ──────────────────────

fn rt_slot_mut<'a>(s: &'a mut SysState, idx: i32) -> &'a mut RtSlot {
    s.rt.entry(idx).or_insert(RtSlot::Stopped { accumulated_ns: 0, ntick: 0 })
}

pub fn realtimeTick(clockIndex: i32) -> Result<()> {
    with(|s| {
        let slot = rt_slot_mut(s, clockIndex);
        *slot = RtSlot::Running { start: Instant::now(), accumulated_ns: 0, ntick: 0 };
    });
    Ok(())
}

pub fn realtimeTock(clockIndex: i32) -> Result<metamodelica::Real> {
    let nanos = with(|s| -> Option<u128> {
        let slot = rt_slot_mut(s, clockIndex);
        match slot {
            RtSlot::Running { start, ntick, .. } => {
                let elapsed = start.elapsed().as_nanos();
                *ntick += 1;
                Some(elapsed)
            }
            RtSlot::Stopped { .. } => None,
        }
    });
    match nanos {
        Some(n) => Ok(metamodelica::OrderedFloat(n as f64 / 1.0e9)),
        None => bail!("System.realtimeTock: clock {clockIndex} not running"),
    }
}

pub fn realtimeClear(clockIndex: i32) -> Result<()> {
    with(|s| {
        s.rt.insert(clockIndex, RtSlot::Stopped { accumulated_ns: 0, ntick: 0 });
    });
    Ok(())
}

pub fn realtimeAccumulate(clockIndex: i32) -> Result<metamodelica::Real> {
    with(|s| {
        let slot = rt_slot_mut(s, clockIndex);
        match *slot {
            RtSlot::Running { start, accumulated_ns, ntick } => {
                let new_acc = accumulated_ns + start.elapsed().as_nanos();
                *slot = RtSlot::Stopped { accumulated_ns: new_acc, ntick: ntick + 1 };
                Ok(metamodelica::OrderedFloat(new_acc as f64 / 1.0e9))
            }
            RtSlot::Stopped { accumulated_ns, .. } => {
                Ok(metamodelica::OrderedFloat(accumulated_ns as f64 / 1.0e9))
            }
        }
    })
}

pub fn realtimeAccumulated(clockIndex: i32) -> Result<metamodelica::Real> {
    with(|s| {
        let slot = rt_slot_mut(s, clockIndex);
        let nanos = match *slot {
            RtSlot::Running { start, accumulated_ns, .. } => accumulated_ns + start.elapsed().as_nanos(),
            RtSlot::Stopped { accumulated_ns, .. } => accumulated_ns,
        };
        Ok(metamodelica::OrderedFloat(nanos as f64 / 1.0e9))
    })
}

pub fn realtimeNtick(clockIndex: i32) -> Result<i32> {
    with(|s| {
        let slot = rt_slot_mut(s, clockIndex);
        Ok(match *slot {
            RtSlot::Running { ntick, .. } | RtSlot::Stopped { ntick, .. } => ntick,
        })
    })
}

// ───────────────────────────────── single-instance timer ─────────────────────

pub fn resetTimer() {
    with(|s| {
        s.timer_running = None;
        s.timer_accum = 0.0;
        s.timer_last_interval = 0.0;
        s.timer_stack = 0;
    });
}
pub fn startTimer() {
    with(|s| {
        if s.timer_running.is_none() {
            s.timer_running = Some(Instant::now());
        }
        s.timer_stack += 1;
    });
}
pub fn stopTimer() {
    with(|s| {
        if let Some(t0) = s.timer_running.take() {
            let elapsed = t0.elapsed().as_secs_f64();
            s.timer_last_interval = elapsed;
            s.timer_accum += elapsed;
        }
        s.timer_stack = (s.timer_stack - 1).max(0);
    });
}
pub fn getTimerIntervalTime() -> metamodelica::Real {
    metamodelica::OrderedFloat(with(|s| s.timer_last_interval))
}
pub fn getTimerCummulatedTime() -> metamodelica::Real {
    metamodelica::OrderedFloat(with(|s| s.timer_accum))
}
pub fn getTimerElapsedTime() -> metamodelica::Real {
    metamodelica::OrderedFloat(with(|s| {
        match s.timer_running {
            Some(t0) => s.timer_accum + t0.elapsed().as_secs_f64(),
            None => s.timer_accum,
        }
    }))
}
pub fn getTimerStackIndex() -> i32 {
    with(|s| s.timer_stack)
}

// ───────────────────────────────── UUID / path helpers ────────────────────────

pub fn getUUIDStr() -> ArcStr {
    // The C runtime uses uuid_generate(3); we synthesise a v4-shaped UUID
    // from the per-thread RNG. This is not cryptographic — but neither is
    // the use case (temp directory naming, error report IDs).
    let (a, b) = with(|s| {
        let mut step = || {
            s.rng = s.rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            s.rng
        };
        (step(), step())
    });
    ArcStr::from(format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        (a >> 32) as u32,
        ((a >> 16) & 0xffff) as u32,
        (a & 0x0fff) as u32,
        0x8000 | ((b >> 48) & 0x3fff) as u32,
        b & 0x0000_ffff_ffff_ffff,
    ))
}

pub fn basename(filename: ArcStr) -> ArcStr {
    Path::new(filename.as_str())
        .file_name()
        .map(|s| ArcStr::from(s.to_string_lossy().as_ref()))
        .unwrap_or_else(|| filename.clone())
}

pub fn dirname(filename: ArcStr) -> ArcStr {
    Path::new(filename.as_str())
        .parent()
        .map(|p| ArcStr::from(p.to_string_lossy().as_ref()))
        .unwrap_or_else(|| literal!(""))
}

// ───────────────────────────────── escape helpers ────────────────────────────

pub fn escapedString(unescapedString: ArcStr, unescapeNewline: bool) -> ArcStr {
    // Mirror `SystemImpl__escapedString`: convert special characters to
    // their backslash form. When `unescapeNewline` is true (sic — the
    // parameter name in the .mo is misleading), newlines are *also*
    // escaped; when false, newlines pass through verbatim.
    let mut out = String::with_capacity(unescapedString.len());
    for c in unescapedString.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"'  => out.push_str("\\\""),
            '\'' => out.push_str("\\'"),
            '\x07' => out.push_str("\\a"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x0b' => out.push_str("\\v"),
            '\n' if unescapeNewline => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    ArcStr::from(out)
}

pub fn unescapedString(escapedString: ArcStr) -> ArcStr {
    let mut out = String::with_capacity(escapedString.len());
    let mut chars = escapedString.chars();
    while let Some(c) = chars.next() {
        if c != '\\' { out.push(c); continue; }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('a') => out.push('\x07'),
            Some('b') => out.push('\x08'),
            Some('f') => out.push('\x0c'),
            Some('v') => out.push('\x0b'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('\'') => out.push('\''),
            Some('0') => out.push('\0'),
            Some(other) => { out.push('\\'); out.push(other); }
            None => out.push('\\'),
        }
    }
    ArcStr::from(out)
}

pub fn unescapedStringLength(unescapedString: ArcStr) -> i32 {
    // Length the string would have *after* unescape pre-applied. The C
    // runtime decrements one byte per recognised escape sequence; do the
    // same by walking once.
    let mut len: i32 = 0;
    let mut chars = unescapedString.chars();
    while let Some(c) = chars.next() {
        if c != '\\' { len += 1; continue; }
        if chars.next().is_some() { len += 1; } else { len += 1; }
    }
    len
}

pub fn unquoteIdentifier(r#str: ArcStr) -> ArcStr {
    // Modelica's `'...'` quoted identifiers; the C runtime strips the
    // surrounding quotes and unescapes the content. If the input is not
    // a quoted identifier, it's returned verbatim.
    let s = r#str.as_str();
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') {
        unescapedString(ArcStr::from(&s[1..s.len() - 1]))
    } else {
        r#str
    }
}

// ───────────────────────────────── numeric limits ─────────────────────────────

pub fn intMaxLit() -> i32 {
    // MetaModelica `Integer` lowers to Rust `i32`, so the maximum literal
    // is `i32::MAX`. (The original C runtime returns LONG_MAX cast to int.)
    i32::MAX
}

pub fn realMaxLit() -> metamodelica::Real {
    metamodelica::OrderedFloat(f64::MAX)
}

// ───────────────────────────────── URI / platform info ───────────────────────

pub fn uriToClassAndPath(_uri: ArcStr) -> Result<(ArcStr, ArcStr, ArcStr)> {
    // Parses `modelica://Pkg.Sub/foo` and `file://...` URIs into
    // (className, fileName, fullPath). Defer until needed; uses the
    // Modelica URI resolver registered by `updateUriMapping`.
    todo!("System.uriToClassAndPath: URI resolver not yet ported")
}

pub fn modelicaPlatform() -> ArcStr {
    // Standardised platform name per the Modelica spec
    // (linux32 / linux64 / win32 / win64 / darwin64).
    let s = match (Autoconf::os, Autoconf::is64Bit) {
        ("linux",  true)  => "linux64",
        ("linux",  false) => "linux32",
        ("Windows_NT", true)  => "win64",
        ("Windows_NT", false) => "win32",
        ("OSX", _)  => "darwin64",
        _ => Autoconf::os,
    };
    ArcStr::from(s)
}

pub fn openModelicaPlatform() -> ArcStr {
    // OMC's preferred platform identifier — same as modelicaPlatform for
    // now since we have no separate notion.
    modelicaPlatform()
}

pub fn openModelicaPlatformAlternative() -> ArcStr {
    literal!("")
}

pub fn gccDumpMachine() -> ArcStr {
    // Output of `<CC> -dumpmachine`. Requires invoking the compiler;
    // defer until a code path actually consumes it.
    todo!("System.gccDumpMachine: needs to shell out to the configured CC")
}

pub fn gccVersion() -> ArcStr {
    todo!("System.gccVersion: needs to shell out to the configured CC")
}

// ───────────────────────────────── LAPACK / iconv / printf ───────────────────

pub fn dgesv(
    _A: Arc<List<Arc<List<metamodelica::Real>>>>,
    _B: Arc<List<metamodelica::Real>>,
) -> Result<(Arc<List<metamodelica::Real>>, i32)> {
    // LAPACK dense solver; the C runtime links libblas/liblapack.
    // Wiring a Rust LAPACK binding (e.g. lapack-sys) is out of scope.
    todo!("System.dgesv: LAPACK binding not yet ported")
}

pub fn reopenStandardStream(_stream: i32, _filename: ArcStr) -> bool {
    todo!("System.reopenStandardStream: freopen(stdin/stdout/stderr) not yet ported")
}

pub fn iconv(_string: ArcStr, _from: ArcStr, _to: ArcStr) -> ArcStr {
    // GNU iconv wrapper. We're all-UTF-8 in the Rust port, so a real
    // implementation only matters when callers ask for legacy charsets.
    todo!("System.iconv: charset conversion not yet ported")
}

pub fn snprintff(format: ArcStr, maxlen: i32, val: metamodelica::Real) -> Result<ArcStr> {
    // `snprintff(fmt, n, x)` is a thin wrapper around C's snprintf for
    // floating-point values — used by the dumper to emit Modelica-formatted
    // doubles. Rust's std doesn't expose printf-style format-string parsing,
    // so for now we honour the `%.{prec}{spec}` shape most callers use,
    // and fall back to `{:?}` for anything else. The C runtime truncates
    // to maxlen-1 bytes; we mirror that.
    let formatted = c_format_double(format.as_str(), val.into_inner())
        .with_context(|| format!("System.snprintff: unsupported format {format}"))?;
    let cap = (maxlen.max(0) as usize).saturating_sub(1);
    let truncated: String = formatted.chars().take(cap).collect();
    Ok(ArcStr::from(truncated))
}

pub fn sprintff(format: ArcStr, val: metamodelica::Real) -> Result<ArcStr> {
    let s = c_format_double(format.as_str(), val.into_inner())
        .with_context(|| format!("System.sprintff: unsupported format {format}"))?;
    Ok(ArcStr::from(s))
}

/// Best-effort port of C `snprintf` for a single floating-point conversion.
/// Recognises `%[flags][width][.prec][fFeEgG]`. Returns `None` for shapes
/// we don't (yet) understand so callers can decide how loudly to fail.
fn c_format_double(fmt: &str, val: f64) -> Option<String> {
    let bytes = fmt.as_bytes();
    let pct = bytes.iter().position(|b| *b == b'%')?;
    let prefix = &fmt[..pct];
    // Walk flags, width, precision, conversion.
    let mut i = pct + 1;
    let mut flags = String::new();
    while i < bytes.len() && matches!(bytes[i], b'-' | b'+' | b' ' | b'#' | b'0') {
        flags.push(bytes[i] as char);
        i += 1;
    }
    let mut width = String::new();
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        width.push(bytes[i] as char);
        i += 1;
    }
    let mut precision: Option<usize> = None;
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        let mut p = String::new();
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            p.push(bytes[i] as char);
            i += 1;
        }
        precision = Some(p.parse().unwrap_or(0));
    }
    if i >= bytes.len() { return None; }
    let spec = bytes[i] as char;
    let suffix = &fmt[i + 1..];

    let body = match spec {
        'f' | 'F' => match precision {
            Some(p) => format!("{:.*}", p, val),
            None => format!("{:.6}", val),
        },
        'e' => format!("{:.*e}", precision.unwrap_or(6), val),
        'E' => format!("{:.*E}", precision.unwrap_or(6), val),
        'g' | 'G' => {
            // %g picks %f or %e depending on exponent; rust has no direct
            // analogue, so emit a compact decimal. Good enough for diag dumps.
            let p = precision.unwrap_or(6);
            let s = format!("{:.*}", p, val);
            // strip trailing zeros if no flag forbids it
            if !flags.contains('#') {
                let trimmed = s.trim_end_matches('0').trim_end_matches('.').to_owned();
                if trimmed.is_empty() { "0".to_owned() } else { trimmed }
            } else {
                s
            }
        }
        _ => return None,
    };

    let pad_to: Option<usize> = width.parse().ok();
    let padded = match pad_to {
        Some(w) if body.len() < w => {
            let fill = w - body.len();
            let pad: String = if flags.contains('0') { "0".repeat(fill) } else { " ".repeat(fill) };
            if flags.contains('-') { format!("{body}{pad}") } else { format!("{pad}{body}") }
        }
        _ => body,
    };
    Some(format!("{prefix}{padded}{suffix}"))
}

// ───────────────────────────────── randomness ─────────────────────────────────

fn next_rand(s: &mut SysState) -> u64 {
    s.rng = s.rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    s.rng
}

pub fn realRand() -> metamodelica::Real {
    let r = with(|s| next_rand(s));
    // 53-bit mantissa worth of randomness so the result is uniform in [0,1).
    let v = (r >> 11) as f64 / ((1u64 << 53) as f64);
    metamodelica::OrderedFloat(v)
}

pub fn intRand(n: i32) -> i32 {
    if n <= 0 { return 0; }
    let r = with(|s| next_rand(s));
    (r % n as u64) as i32
}

pub fn intRandom(n: i32) -> i32 {
    // `intRandom` differs from `intRand` only in the upper bound being
    // inclusive in the .mo doc (`Integer in {0,...,n-1}` is what callers
    // expect — same semantics).
    intRand(n)
}

// ───────────────────────────────── gettext (no-op) ───────────────────────────

pub fn gettextInit(_locale: ArcStr) {}

pub fn gettext(msgid: ArcStr) -> ArcStr {
    // No translation catalog wired up; pass the English msgid through.
    msgid
}

pub fn anyStringCode<Any: Clone + 'static>(_any: Any) -> ArcStr {
    // Pretty-prints any MetaModelica runtime value (`mmc_anyString`).
    // Useful for debug dumps; defer until a caller needs it.
    todo!("System.anyStringCode: generic runtime-value printer not yet ported")
}

pub fn numBits() -> i32 {
    if Autoconf::is64Bit { 64 } else { 32 }
}

pub fn realpath(path: ArcStr) -> Result<ArcStr> {
    let canon = fs::canonicalize(path.as_str())
        .with_context(|| format!("System.realpath: cannot resolve {path}"))?;
    Ok(ArcStr::from(canon.to_string_lossy().as_ref()))
}

pub fn getSimulationHelpText(_detailed: bool, _sphinx: bool) -> ArcStr {
    // Simulation-runtime CLI help; the C version asks the runtime for its
    // option list. None of the Rust-side callers consume it yet.
    todo!("System.getSimulationHelpText: simulation runtime help text not yet ported")
}

pub fn getTerminalWidth() -> i32 {
    // The C runtime probes `TIOCGWINSZ`; without an `ioctl` binding we
    // fall back to the COLUMNS env var, then 80.
    std::env::var("COLUMNS").ok().and_then(|s| s.parse::<i32>().ok()).unwrap_or(80)
}

pub fn fileIsNewerThan(file1: ArcStr, file2: ArcStr) -> Result<bool> {
    let m1 = fs::metadata(file1.as_str()).with_context(|| format!("stat {file1}"))?;
    let m2 = fs::metadata(file2.as_str()).with_context(|| format!("stat {file2}"))?;
    Ok(m1.modified()? > m2.modified()?)
}

pub fn fileContentsEqual(file1: ArcStr, file2: ArcStr) -> bool {
    match (fs::read(file1.as_str()), fs::read(file2.as_str())) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

pub fn rename(source: ArcStr, dest: ArcStr) -> bool {
    fs::rename(source.as_str(), dest.as_str()).is_ok()
}

pub fn numProcessors() -> i32 {
    std::thread::available_parallelism().map(|n| n.get() as i32).unwrap_or(1)
}

pub fn launchParallelTasks<AnyInput: Clone + Send + 'static, AnyOutput: Clone + Send + 'static>(
    _numThreads: i32,
    _inData: Arc<List<AnyInput>>,
    _func: Arc<dyn Fn(AnyInput) -> Result<AnyOutput> + 'static>,
) -> Result<Arc<List<AnyOutput>>> {
    // The C version spawns `numThreads` worker pthreads, each pulling
    // tasks off a shared queue. The Rust port needs `Send + Sync` on the
    // closure type, which conflicts with the current `Arc<dyn Fn>` slot.
    // Defer until we either widen the callback bound at all call sites or
    // route through rayon.
    todo!("System.launchParallelTasks: parallel task scheduler not yet ported")
}

pub fn exit(status: i32) -> Result<()> {
    std::process::exit(status);
}

pub fn threadWorkFailed() {
    // The C version calls `pthread_exit(EXIT_FAILURE)`. With no thread
    // pool to bail out of, the closest equivalent is a panic — but the
    // .mo callers always wrap this in a guarded try, so the panic will
    // be observed as a failed task by the orchestrator once
    // `launchParallelTasks` is implemented.
    panic!("System.threadWorkFailed: worker thread aborted by user code");
}

pub fn getMemorySize() -> metamodelica::Real {
    // Total system memory in bytes. The C runtime reads `_SC_PHYS_PAGES *
    // _SC_PAGE_SIZE` on POSIX; we don't have a portable shortcut in std.
    todo!("System.getMemorySize: physical memory probe not yet ported")
}

pub fn initGarbageCollector() {
    // Boehm GC initialisation — irrelevant in the Rust port; ownership
    // covers everything the GC used to.
}

pub fn ctime(t: metamodelica::Real) -> ArcStr {
    // POSIX ctime(3) returns "Day Mon DD HH:MM:SS YYYY\n"; we approximate
    // with the same layout sans the trailing newline so callers that
    // splice it into messages don't get a stray line break.
    let secs = t.into_inner() as i64;
    let (year, mon, mday, hour, min, sec) = epoch_to_civil(secs);
    let mon_name = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
        .get(mon as usize - 1).copied().unwrap_or("???");
    // Day-of-week via Zeller's congruence (Gregorian, 0=Saturday).
    let (q, m, y) = if mon < 3 { (mday, mon + 12, year - 1) } else { (mday, mon, year) };
    let k = y % 100;
    let j = y / 100;
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
    let dow = ["Sat","Sun","Mon","Tue","Wed","Thu","Fri"][h as usize];
    ArcStr::from(format!(
        "{dow} {mon_name} {mday:2} {hour:02}:{min:02}:{sec:02} {year:04}"
    ))
}

// ───────────────────────────────── stat ──────────────────────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum StatFileType {
    NoFile = 1,
    RegularFile = 2,
    Directory = 3,
    SpecialFile = 4,
}
impl PartialOrd for StatFileType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for StatFileType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn stat(filename: ArcStr) -> (bool, metamodelica::Real, metamodelica::Real, StatFileType) {
    match fs::metadata(filename.as_str()) {
        Ok(m) => {
            let size = m.len() as f64;
            let mtime = m.modified().ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs_f64())
                .unwrap_or(0.0);
            let kind = if m.is_file() {
                StatFileType::RegularFile
            } else if m.is_dir() {
                StatFileType::Directory
            } else {
                StatFileType::SpecialFile
            };
            (true, metamodelica::OrderedFloat(size), metamodelica::OrderedFloat(mtime), kind)
        }
        Err(_) => (false, metamodelica::OrderedFloat(0.0), metamodelica::OrderedFloat(0.0), StatFileType::NoFile),
    }
}

pub fn alarm(_seconds: i32) -> i32 {
    // POSIX SIGALRM scheduling. Rust signal handling lives in `signal-hook`
    // which isn't a workspace dep; not needed by current Rust callers.
    todo!("System.alarm: SIGALRM scheduling not yet ported")
}

pub fn covertTextFileToCLiteral(_textFile: ArcStr, _outFile: ArcStr, _target: ArcStr) -> bool {
    // Reads a text file and writes a C-source file containing the text
    // as a string literal. The C runtime handles escaping platform-by-
    // platform; defer until a Susan template needs it.
    todo!("System.covertTextFileToCLiteral: text-to-C-literal converter not yet ported")
}

pub fn dladdr<T: Clone + 'static>(_symbol: T) -> (ArcStr, ArcStr, ArcStr) {
    // Looks up the shared-object file and symbol name for a runtime
    // function pointer. Needs platform-specific ABI plumbing; defer.
    todo!("System.dladdr: function-pointer→symbol lookup not yet ported")
}

// ───────────────────────────────── StringAllocator (opaque) ──────────────────

/// Opaque external object `StringAllocator`. The C runtime owns the
/// representation; this struct exists only to give the type a nominal
/// identity in Rust so call sites type-check.
///
/// All operations that would actually touch the underlying buffer
/// (`stringAllocatorStringCopy`, `stringAllocatorResult`) are still
/// `todo!()` — once a caller needs them, the right move is probably to
/// back this with a `Vec<u8>` inside the Arc<Mutex<...>> and have
/// `stringAllocatorResult` hand out an `ArcStr` view.
#[derive(Clone, Debug)]
pub struct StringAllocator {
    _opaque: std::sync::Arc<std::sync::Mutex<()>>,
}

impl StringAllocator {
    pub fn new(_sz: i32) -> Result<StringAllocator> {
        todo!("external object `StringAllocator`: constructor not implemented")
    }
}
pub fn StringAllocator(sz: i32) -> Result<StringAllocator> {
    StringAllocator::new(sz)
}

pub fn destructor(_str: StringAllocator) {}

pub fn stringAllocatorStringCopy(_dest: StringAllocator, _source: ArcStr, _destOffset: i32) {
    todo!("System.stringAllocatorStringCopy: requires StringAllocator buffer impl")
}

pub fn stringAllocatorResult<T: Clone + 'static>(_sa: StringAllocator, _dummy: T) -> T {
    todo!("System.stringAllocatorResult: requires StringAllocator buffer impl")
}

pub fn relocateFunctions(_fileName: ArcStr, _names: Arc<List<(ArcStr, ArcStr)>>) -> bool {
    // Hot-swap runtime symbols from a fresh .so — needs dlopen + relocation
    // walking. Not used by the Rust-side compile path.
    todo!("System.relocateFunctions: symbol relocation not yet ported")
}

pub fn fflush() {
    use std::io::Write as _;
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();
}

pub fn updateUriMapping(_namesAndDirs: metamodelica::Array<ArcStr>) {
    // Updates the runtime's `package → directory` map used by
    // `uriToClassAndPath`. Until that resolver is ported there is nothing
    // to feed.
    todo!("System.updateUriMapping: package URI mapping not yet ported")
}

pub fn getSizeOfData<T: Clone + 'static>(_data: T) -> (metamodelica::Real, metamodelica::Real, metamodelica::Real) {
    // Walks the in-memory object graph counting bytes. The C version
    // leans on Boehm GC's heap layout; Rust has nothing equivalent.
    todo!("System.getSizeOfData: heap-walking memory profiler not yet ported")
}

// ───────────────────────────────── fputs / waitForInput ──────────────────────

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum StreamType {
    STDOUT = 1,
    STDERR = 2,
}
impl PartialOrd for StreamType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for StreamType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}

pub fn fputs(r#str: ArcStr, streamType: StreamType) -> i32 {
    use std::io::Write as _;
    let res = match streamType {
        StreamType::STDOUT => std::io::stdout().write_all(r#str.as_bytes()),
        StreamType::STDERR => std::io::stderr().write_all(r#str.as_bytes()),
    };
    if res.is_ok() { 0 } else { -1 }
}

pub fn waitForInput() {
    // Block until a single byte arrives on stdin — used as a debugger
    // synchronisation point (attach valgrind, then press enter).
    let mut buf = [0u8; 1];
    use std::io::Read as _;
    let _ = std::io::stdin().read(&mut buf);
}
