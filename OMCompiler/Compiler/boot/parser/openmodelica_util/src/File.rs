// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/File.mo` and its companion C runtime
// `OMCompiler/Compiler/Util/omc_file_ext.h`.
//
// The MetaModelica source declares `File` as an ExternalObject whose runtime
// representation in C is `__OMC_FILE { FILE* file; mmc_sint_t cnt; const char*
// name; }` with manual reference counting. In Rust we model the same
// invariants more idiomatically:
//
//   * `pub struct File` is a clonable handle whose contents are shared via
//     `Arc<Mutex<FileInner>>`. Cloning the `Arc` *is* the reference count.
//   * `FileInner` owns an optional `std::fs::File` plus the on-disk file name
//     and current write escape mode. When the last `Arc` is dropped the
//     `std::fs::File` is dropped too — the C destructor's `fclose` happens
//     for free.
//
// Note on the constructor's `fromID: Option<Integer>` parameter: in the C
// runtime this is a void* pretending to be `Option<Integer>` — either NULL
// (meaning "make a new file") or a raw `__OMC_FILE*` to clone-by-refcount.
// MetaModelica callers only ever produce that pointer via `getReference` /
// `noReference`, both of which we cannot faithfully implement without C-side
// pointer punning. We therefore stub `getReference` / `noReference` /
// `releaseReference` with `todo!()` and treat the constructor's `fromID` as
// always-NULL (the new-file path), which is what `File()` with the default
// `fromID = noReference()` always was anyway.

#![allow(non_snake_case)]

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use arcstr::{literal, ArcStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Mode {
    Read = 1,
    Write = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Whence {
    Set = 1,
    Current = 2,
    End = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum Escape {
    None = 1,
    C = 2,
    JSON = 3,
    XML = 4,
}

pub mod Examples {
    use super::*;
}

struct FileInner {
    file: Option<std::fs::File>,
    name: ArcStr,
}

impl FileInner {
    fn write_bytes(&mut self, bytes: &[u8], what: &str) -> Result<()> {
        match self.file.as_mut() {
            Some(f) => {
                f.write_all(bytes)
                    .map_err(|e| anyhow::anyhow!("File.{what}: write to {}: {}", self.name, e))
            }
            None => bail!("File.{what}: Failed to write to file: {} (not open)", self.name),
        }
    }
}

/// A handle to an opaque MetaModelica `File` external object.
#[derive(Clone)]
pub struct File {
    inner: Arc<Mutex<FileInner>>,
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.lock() {
            Ok(g) => write!(f, "File({:?})", g.name),
            Err(_) => write!(f, "File(<poisoned>)"),
        }
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}
impl Eq for File {}

impl File {
    /// Constructor. `fromID` mirrors the MM signature and, in C, is a void*
    /// that's either NULL ("new file") or a raw pointer to clone by refcount.
    /// We only model the NULL path; see the module-level comment for why.
    pub fn new(_fromID: Option<i32>) -> Result<File> {
        Ok(File {
            inner: Arc::new(Mutex::new(FileInner {
                file: None,
                name: literal!("[no open file]"),
            })),
        })
    }
}

/// Free-function shim with the class name so MetaModelica call sites that
/// spell the constructor as `File.File(...)` (i.e. `File::File(...)` in
/// generated Rust) resolve. Functions and types share no namespace in Rust,
/// so this happily coexists with the `File` struct.
pub fn File(fromID: Option<i32>) -> Result<File> {
    File::new(fromID)
}

/// Destructor stub. The MM destructor's job is to release the underlying
/// resource; in Rust that happens automatically when the last `Arc` is
/// dropped, so this is intentionally a no-op.
pub fn destructor(_file: File) {}

pub fn open(file: File, filename: ArcStr, mode: Mode) -> Result<()> {
    let mut guard = file.inner.lock().unwrap();
    // If a file is already open, close it (drop it) before opening a new one,
    // matching the C runtime's behavior.
    guard.file = None;
    let handle = match mode {
        Mode::Read => OpenOptions::new().read(true).open(filename.as_str()),
        Mode::Write => OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename.as_str()),
    }
    .map_err(|e| anyhow::anyhow!("File.open: Failed to open file {filename} with mode {mode:?}: {e}"))?;
    guard.file = Some(handle);
    guard.name = filename;
    Ok(())
}

pub fn write(file: File, data: ArcStr) -> Result<()> {
    let mut guard = file.inner.lock().unwrap();
    guard.write_bytes(data.as_bytes(), "write")
}

pub fn writeInt(file: File, data: i32, format: ArcStr) -> Result<()> {
    // The C runtime uses `fprintf` with a user-supplied format string. We
    // honor the common `%d` default with a fast path and fall through to a
    // simple substitution otherwise. Full printf-compatibility would require
    // a printf parser; callers in the OMC sources only use `%d` and `%ld`.
    let mut guard = file.inner.lock().unwrap();
    let s = match format.as_str() {
        "%d" | "%i" | "%ld" => data.to_string(),
        other => other.replace("%d", &data.to_string()).replace("%ld", &data.to_string()),
    };
    guard.write_bytes(s.as_bytes(), "writeInt")
}

pub fn writeReal(file: File, data: f64, format: ArcStr) -> Result<()> {
    let mut guard = file.inner.lock().unwrap();
    let s = match format.as_str() {
        "%.15g" => format!("{:.15e}", data).replace("e0", ""),
        // Generic fallback: just print using Rust's default Display.
        _ => data.to_string(),
    };
    guard.write_bytes(s.as_bytes(), "writeReal")
}

pub fn writeEscape(file: File, data: ArcStr, escape: Escape) -> Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(data.len());
    match escape {
        Escape::None => buf.extend_from_slice(data.as_bytes()),
        Escape::C => {
            for &b in data.as_bytes() {
                match b {
                    b'\n' => buf.extend_from_slice(b"\\n"),
                    b'"' => buf.extend_from_slice(b"\\\""),
                    _ => buf.push(b),
                }
            }
        }
        Escape::JSON => {
            for &b in data.as_bytes() {
                match b {
                    b'"' => buf.extend_from_slice(b"\\\""),
                    b'\\' => buf.extend_from_slice(b"\\\\"),
                    b'\n' => buf.extend_from_slice(b"\\n"),
                    0x08 => buf.extend_from_slice(b"\\b"),
                    0x0C => buf.extend_from_slice(b"\\f"),
                    b'\r' => buf.extend_from_slice(b"\\r"),
                    b'\t' => buf.extend_from_slice(b"\\t"),
                    b if b < b' ' => {
                        // Other control characters are emitted as \uXXXX.
                        buf.extend_from_slice(format!("\\u{:04x}", b).as_bytes());
                    }
                    b => buf.push(b),
                }
            }
        }
        Escape::XML => {
            for &b in data.as_bytes() {
                match b {
                    b'<' => buf.extend_from_slice(b"&lt;"),
                    b'>' => buf.extend_from_slice(b"&gt;"),
                    b'"' => buf.extend_from_slice(b"&#34;"),
                    b'&' => buf.extend_from_slice(b"&amp;"),
                    b'\'' => buf.extend_from_slice(b"&#39;"),
                    b => buf.push(b),
                }
            }
        }
    }
    let mut guard = file.inner.lock().unwrap();
    guard.write_bytes(&buf, "writeEscape")
}

pub fn seek(file: File, offset: i32, whence: Whence) -> Result<bool> {
    let mut guard = file.inner.lock().unwrap();
    let Some(f) = guard.file.as_mut() else { return Ok(false) };
    let from = match whence {
        Whence::Set => SeekFrom::Start(offset as u64),
        Whence::Current => SeekFrom::Current(offset as i64),
        Whence::End => SeekFrom::End(offset as i64),
    };
    Ok(f.seek(from).is_ok())
}

pub fn tell(file: File) -> Result<i32> {
    let mut guard = file.inner.lock().unwrap();
    match guard.file.as_mut() {
        Some(f) => match f.stream_position() {
            Ok(p) => Ok(p as i32),
            Err(_) => Ok(-1),
        },
        None => Ok(-1),
    }
}

pub fn getFilename(file: Option<i32>) -> Result<ArcStr> {
    // The MM signature is a lie — `file: Option<Integer>` is really an
    // opaque `__OMC_FILE*` returned by `getReference`. Honest Rust cannot
    // reconstruct the original `File` from a bare `Option<i32>`. The only
    // honest implementations are (a) thread an actual handle through the
    // API, or (b) FFI to the C runtime. Until one of those is done, fail
    // loudly rather than silently returning a bogus string.
    let _ = file;
    todo!("File.getFilename: opaque-pointer punning via Option<Integer> is not yet bridged in the Rust runtime")
}

pub fn noReference() -> Result<Option<i32>> {
    // In C this returns NULL — a void* that the constructor recognizes as
    // "make a new file". The Rust constructor's `_fromID` is ignored, so
    // `None` is a faithful stand-in for the default-value path.
    Ok(None)
}

pub fn getReference(file: File) -> Result<Option<i32>> {
    // Same caveat as `getFilename`: the return value is really a pointer,
    // not an integer. Anyone who consumes the result expects to round-trip
    // it back to the constructor, which we cannot do without C runtime help.
    let _ = file;
    todo!("File.getReference: opaque-pointer punning via Option<Integer> is not yet bridged in the Rust runtime")
}

pub fn releaseReference(file: File) -> Result<()> {
    // The C runtime decrements a reference count here. In Rust the Arc
    // refcount tracks lifetime automatically — dropping the `file`
    // parameter at function exit decrements it for us.
    let _ = file;
    Ok(())
}

pub fn writeSpace(file: File, n: i32) -> Result<()> {
    for _ in 0..n {
        write(file.clone(), literal!(" "))?;
    }
    Ok(())
}
