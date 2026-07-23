// OMShell omc Web Worker.
//
// On wasm the omc compiler is a *separate* wasm module (built from
// libopenmodelica_compiler, `web` target) that exports omc_init/omc_eval/
// omc_set_env (see libopenmodelica_compiler/src/wasm_api.rs). Running it on the
// UI thread froze the page for the duration of every command, so it is hosted
// here instead: a dedicated module Worker that the GUI talks to via postMessage
// (see omshell_core::driver, wasm). The UI thread stays free, so the spinner
// animates and input keeps responding while omc works.
//
// The build stages this file at the web root next to the omc module, so the
// `./omc/` import below resolves regardless of which OMShell page loads it.
//
// Downloads: omc_eval is synchronous and cannot await a fetch, so a command that
// needs a file (installPackage, updatePackageIndex, ...) does not fetch it
// itself — Curl_wasm records the missing file as "pending" and the command fails.
// evalWithDownloads() drains that pending list (omc_take_pending_downloads),
// fetches each file here with a streaming reader (posting {progress} so the GUI
// can show a real download bar), stages the bytes in the store (wasi_write_file), and
// re-runs the command, which now finds them. This needs no SharedArrayBuffer /
// cross-origin isolation. See openmodelica_script_util/src/Curl_wasm.rs.
//
// The init/version orchestration mirrors the omshell_omc native backend
// (init_session / eval_with_errors in lib.rs); keep the two in step.
import init, {
  omc_init,
  omc_eval,
  omc_set_env,
  wasi_path_open,
  wasi_fd_read,
  wasi_fd_close,
  wasi_path_filestat_get,
  wasi_readdir,
  wasi_write_file,
  omc_take_pending_downloads,
  omc_take_plot_commands,
} from "./omc/OpenModelicaCompiler.js";
// omc_abi exists only when the omc module is built with `scripting_api` (the
// OMEdit web client). A named import of a missing export would break the worker
// for OMShell/OMNotebook, so reach it through the namespace and feature-detect it.
import * as OmcModule from "./omc/OpenModelicaCompiler.js";

// Self-ID so a page console shows which omc_worker.js loaded (cache diagnosis).
console.log("omc_worker.js loaded (WASI file surface)");

// Cooperative cancel + live progress (OMEdit-wasm): a long omc call (simulate,
// installPackage, …) blocks here, so the main thread shares a SharedArrayBuffer
// "control block" the worker reads/writes with Atomics. Layout (Int32Array):
//   [0] cancel   main→worker  0 run / 1 cancel requested (omc polls per step)
//   [1] progress worker→main  permille 0..1000, or -1 indeterminate
//   [2] phase    worker→main  0 idle,1 download,2 parse,3 instantiate,4 backend,5 sim
//   [3] generation main→both  bumped per op so a stale read is ignored
// Null-safe until the main thread hands over the buffer (`controlBuf`/`cancelBuf`).
let controlView = null;
// OMEdit VFS-over-SAB (see the `vfsSab` setup message).
let vfsSab = null, vfsI32 = null;
function vfsSabReply(bytes) {
  // bytes: Uint8Array on success, null for "not found". Grow the SAB if the
  // reply doesn't fit (both agents see the same grown buffer), then publish.
  const payload = bytes || new Uint8Array(0);
  const need = 16 + payload.length;
  if (need > vfsSab.byteLength && vfsSab.grow) {
    try { vfsSab.grow(Math.min(vfsSab.maxByteLength || need, Math.max(need, vfsSab.byteLength * 2))); } catch (_) {}
  }
  const cap = Math.max(0, vfsSab.byteLength - 16);
  const n = Math.min(payload.length, cap);
  if (n > 0) new Uint8Array(vfsSab, 16, n).set(payload.subarray(0, n));
  Atomics.store(vfsI32, 2, n);
  Atomics.store(vfsI32, 1, bytes ? 1 : 0);
  Atomics.store(vfsI32, 0, 0);            // done
  Atomics.notify(vfsI32, 0);
}
globalThis.__omcPollCancel = () => (controlView ? Atomics.load(controlView, 0) : 0);
globalThis.__omcReportProgress = (permille, phase) => {
  // Guard length so an older 4-byte (cancel-only) buffer doesn't throw.
  if (controlView && controlView.length >= 3) {
    Atomics.store(controlView, 1, permille);
    Atomics.store(controlView, 2, phase);
  }
};
// Clear cancel + progress once an op finishes, so a cancel flag set during (or
// after) one op never leaks into the next and spuriously aborts it — the flag is
// strictly per-op. Called after each eval/abi completes.
function resetControl() {
  if (!controlView) return;
  Atomics.store(controlView, 0, 0);
  if (controlView.length >= 3) {
    Atomics.store(controlView, 1, -1);
    Atomics.store(controlView, 2, 0);
  }
}

// Read a whole file from the worker store through the WASI preview1 flow
// (path_open → fd_read → fd_close). Returns a Uint8Array or undefined if absent.
function wasiReadFile(path) {
  const fd = wasi_path_open(path);
  if (fd < 0) return undefined;
  try {
    return wasi_fd_read(fd) || undefined;
  } finally {
    wasi_fd_close(fd);
  }
}

// Instantiate the omc wasm module once. omc_set_env mirrors the old host page:
// builtins resolve by basename, so OPENMODELICAHOME only needs to be non-empty.
// Parse parallelism tops out early (best around 2-3), and every worker shares
// the one wasm memory (shared:true) — an extra worker only costs a thread + a
// 4 MiB talc segment — so a small pool keeps spawn cheap.
const OMC_POOL_THREADS = Math.min(3, globalThis.navigator?.hardwareConcurrency || 3);
const ready = (async () => {
  await init();
  omc_set_env("OPENMODELICAHOME", "/usr");
  // Spin up the rayon worker pool here, during the host's pre-main-loop init
  // wait, NOT lazily later: spawning workers once the Qt main loop is running
  // desyncs QWasmSuspendResumeControl's native-event queue (empty-shift trap).
  // Fast now (3 workers sharing one wasm memory). Absent in the single-threaded
  // build (feature-detected); omc_thread_pool_ready opens the serial-fallback gate.
  if (typeof OmcModule.initThreadPool === "function") {
    try {
      await OmcModule.initThreadPool(OMC_POOL_THREADS);
      OmcModule.omc_thread_pool_ready?.();
    } catch (_) { /* keep running serial-only */ }
  }
})();

const trim = (s) => (s ?? "").trim();
const unquote = (s) => s.replace(/^"+|"+$/g, "");

// Mirrors omshell_omc's trims: result is just trimmed; diagnostics and the
// version string also have their surrounding quotes stripped.
const cleanError = (s) => trim(unquote(trim(s)));

// Stream one URL into a Uint8Array, posting {progress} as bytes arrive. `total`
// is 0 when the server sends no Content-Length (indeterminate bar).
async function fetchWithProgress(url, label) {
  const resp = await fetch(url);
  if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
  const total = Number(resp.headers.get("content-length")) || 0;
  const reader = resp.body.getReader();
  const chunks = [];
  let received = 0;
  self.postMessage({ kind: "progress", file: label, done: 0, total });
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    chunks.push(value);
    received += value.length;
    self.postMessage({ kind: "progress", file: label, done: received, total });
  }
  const out = new Uint8Array(received);
  let off = 0;
  for (const c of chunks) {
    out.set(c, off);
    off += c.length;
  }
  return out;
}

// Fetch one pending file into the VFS, trying its mirrors in order. A file that
// no mirror serves is left absent: the command's re-run then reports the real
// download failure to the user.
async function fetchToVfs(urls, filename) {
  const label = filename.split("/").pop() || filename;
  for (const url of urls) {
    try {
      const bytes = await fetchWithProgress(url, label);
      wasi_write_file(filename, bytes);
      return;
    } catch (_) {
      // try the next mirror
    }
  }
}

// Run `src`, then satisfy any downloads it requested and run it again, until it
// needs nothing new. `attempted` stops an undownloadable file from looping
// forever (the file stays absent, so the final run surfaces omc's own error).
async function evalWithDownloads(src) {
  const attempted = new Set();
  for (;;) {
    const result = omc_eval(src);
    const pending = omc_take_pending_downloads() || [];
    const todo = pending.filter((p) => !attempted.has(p.filename));
    if (todo.length === 0) return result;
    omc_eval("getErrorString()"); // discard the aborted run's diagnostics
    omc_take_plot_commands(); // and any plots it recorded before aborting
    for (const item of todo) {
      attempted.add(item.filename);
      await fetchToVfs(item.urls, item.filename);
    }
  }
}

// One typed OMEdit ABI call (JSON request -> JSON reply), draining downloads the
// call triggers (installPackage, loadModel deps) and retrying, like evalWithDownloads.
async function abiWithDownloads(request) {
  const attempted = new Set();
  for (;;) {
    const response = OmcModule.omc_abi(request);
    const pending = omc_take_pending_downloads() || [];
    const todo = pending.filter((p) => !attempted.has(p.filename));
    if (todo.length === 0) return response;
    omc_eval("getErrorString()"); // discard the aborted call's diagnostics
    omc_take_plot_commands(); // and any plots it recorded before aborting
    for (const item of todo) {
      attempted.add(item.filename);
      await fetchToVfs(item.urls, item.filename);
    }
  }
}

async function doInit(installMsl) {
  if (!omc_init()) {
    return { kind: "ready", ok: false, error: "omc_init() failed" };
  }
  // Point the omc driver's cancel poll at __omcPollCancel (feature-detected).
  if (typeof OmcModule.omc_enable_cancel_poll === "function") {
    OmcModule.omc_enable_cancel_poll();
  }
  // Route omc progress reports into the control block (feature-detected).
  if (typeof OmcModule.omc_enable_progress_sink === "function") {
    OmcModule.omc_enable_progress_sink();
  }
  // The browser omc has no pre-installed library, so install the MSL to make the
  // shell immediately usable. Best-effort: a failure (e.g. no network) only
  // surfaces its diagnostics, it does not stop the shell from starting. A client
  // that wants its window up first (OMShell-qt) passes installMsl=false and runs
  // installPackage(Modelica) itself as an ordinary command after it is visible.
  let message = "";
  if (installMsl) {
    await evalWithDownloads("installPackage(Modelica)");
    message = cleanError(omc_eval("getErrorString()"));
  }
  const version = unquote(trim(omc_eval("getVersion()")));
  return { kind: "ready", ok: true, version, message };
}

// Plot commands the eval recorded, each with the bytes of its result file from
// the VFS, plus the transferable buffers for postMessage. `args` is the 18
// PlotCallback strings (result file at [0]). Clients that don't plot ignore it.
function collectPlots() {
  const cmds = omc_take_plot_commands() || [];
  const plots = [];
  const transfer = [];
  for (const args of cmds) {
    const file = args[0] || "";
    const bytes = file ? wasiReadFile(file) : undefined;
    plots.push({ args, file, bytes });
    if (bytes) transfer.push(bytes.buffer);
  }
  return { plots, transfer };
}

// `keepErrors` (OMEdit): leave omc's Error buffer intact instead of draining it
// into `error`. OMEdit reads diagnostics itself via the typed
// getMessagesStringInternal ABI (OMCProxy::printMessagesStringInternal), exactly
// as the native build does; draining here would empty the buffer first and the
// GUI would show nothing. OMShell/OMNotebook leave it unset and consume `error`.
async function doEval(src, keepErrors) {
  const result = trim(await evalWithDownloads(src));
  const error = keepErrors ? "" : cleanError(omc_eval("getErrorString()"));
  const { plots, transfer } = collectPlots();
  return {
    msg: { kind: "done", result, error, plots, keep: src.trim() !== "quit()" },
    transfer,
  };
}

// ---------------------------------------------------------------------------
// Sync-channel mode (multithread Qt clients: OMShell/OMNotebook/OMEdit-qt).
//
// A Qt secondary thread ("OMC thread") drives omc without Asyncify: it blocks on
// a futex in the *Qt* module's shared wasm heap while this worker does the work,
// then reads the reply straight out of that heap. The main thread hands us the Qt
// heap (`qtHeap`, a SharedArrayBuffer) and the byte offset (`base`) of an
// OmcChannel struct once (`syncChannel`); we then run syncLoop forever. Since it
// blocks on Atomics.wait, `onmessage` stops firing — so a threaded client sends
// `syncChannel` last and uses only the channel. See HANDOFF-asyncify-removal.md.
const S_IDLE = 0, S_REQUEST = 1, S_REPLY_SIZE = 2, S_REPLY_BUF = 3, S_READY = 4;
// Word indices into OmcChannel (int32), relative to base>>2.
const W_STATE = 0, W_REQKIND = 1, W_REQPTR = 2, W_REQLEN = 3, W_REPLYPTR = 4, W_REPLYLEN = 5;
const K_INIT = 0, K_EVAL = 1, K_ABI = 2, K_VFSGET = 3, K_VFSLIST = 4, K_VFSSTAT = 5, K_QUIT = 6;

const syncSet = (i32, w, s) => { Atomics.store(i32, w, s); Atomics.notify(i32, w, 1); };
const syncAwait = (i32, w, s) => { while (Atomics.load(i32, w) === s) Atomics.wait(i32, w, s); };

// One request -> reply-bytes (UTF-8 JSON). Reuses the same handlers as the
// postMessage path, so download-retry (evalWithDownloads) is identical.
async function handleSync(kind, reqBytes) {
  const req = reqBytes.length ? new TextDecoder().decode(reqBytes) : "{}";
  let out;
  if (kind === K_INIT) {
    const r = await doInit(JSON.parse(req).installMsl !== false);
    out = { ok: !!r.ok, version: r.version || "", message: r.message || "", error: r.error || "" };
  } else if (kind === K_EVAL) {
    const p = JSON.parse(req);
    const { msg } = await doEval(p.src, p.keepErrors);
    resetControl();
    // Plots carry binary result bytes that must land in the *client* module's FS
    // (OMPlot fopen's them there), so they can't ride the JSON channel. Post them
    // for the client's onmessage handler to stage; before the channel reply so
    // takePlotCommands() sees them by the time the OMC thread's evalDone fires.
    if (msg.plots && msg.plots.length) self.postMessage({ kind: "plots", plots: msg.plots });
    out = { result: msg.result || "", error: msg.error || "" };
  } else if (kind === K_ABI) {
    const response = typeof OmcModule.omc_abi === "function"
      ? await abiWithDownloads(JSON.parse(req).request)
      : JSON.stringify({ error: "omc_abi unavailable (omc built without scripting_api)" });
    resetControl();
    out = { response };
  } else if (kind === K_QUIT) {
    out = { ok: true };
  } else {
    // vfsGet/list/stat over the channel need a binary/typed framing; wired when
    // OMEdit lands. OMShell/OMNotebook only use init/eval.
    out = { error: "sync-channel request kind " + kind + " not implemented" };
  }
  return new TextEncoder().encode(JSON.stringify(out));
}

async function syncLoop(qtHeap, base) {
  const bw = base >> 2;
  for (;;) {
    // Re-derive views every iteration: a shared heap grow can hand back a larger
    // buffer object, and the reply region may sit in freshly grown pages.
    let i32 = new Int32Array(qtHeap);
    syncAwait(i32, bw + W_STATE, S_IDLE);            // OMC thread -> REQUEST
    const u8 = new Uint8Array(qtHeap);
    const kind = i32[bw + W_REQKIND];
    const reqPtr = i32[bw + W_REQPTR] >>> 0;
    const reqLen = i32[bw + W_REQLEN] >>> 0;
    const reqBytes = u8.slice(reqPtr, reqPtr + reqLen);
    let replyBytes;
    try { replyBytes = await handleSync(kind, reqBytes); }
    catch (e) { replyBytes = new TextEncoder().encode(JSON.stringify({ error: String(e) })); }
    // The reply travels via the channel, so no terminal postMessage is sent for
    // it — but the main thread clears the download-progress UI only on a
    // `done`/`ready` message. Post one now (post-download) so a finished command
    // doesn't leave "Downloading … 100%" stuck in the status bar.
    self.postMessage({ kind: "done" });
    i32 = new Int32Array(qtHeap);
    Atomics.store(i32, bw + W_REPLYLEN, replyBytes.length);
    syncSet(i32, bw + W_STATE, S_REPLY_SIZE);        // OMC thread mallocs, -> REPLY_BUF
    syncAwait(i32, bw + W_STATE, S_REPLY_SIZE);
    i32 = new Int32Array(qtHeap);
    new Uint8Array(qtHeap).set(replyBytes, i32[bw + W_REPLYPTR] >>> 0);
    syncSet(i32, bw + W_STATE, S_READY);             // OMC thread reads reply, -> IDLE
    syncAwait(i32, bw + W_STATE, S_READY);
  }
}

self.onmessage = async (e) => {
  const msg = e.data;
  // One-way setup message (no reply): store the shared control block. Before
  // `await ready` so it is in place before the first long call. `cancelBuf` is
  // the old name for the same message (a 4-byte cancel-only buffer).
  if (msg.cmd === "controlBuf" || msg.cmd === "cancelBuf") {
    try { controlView = msg.buf ? new Int32Array(msg.buf) : null; } catch (_) { controlView = null; }
    return;
  }
  // Growable SAB for VFS replies (OMEdit). vfsGet/List with `sab:true` write the
  // reply here + Atomics.notify instead of postMessage, so the main thread reads
  // it back with a busy-spin rather than an Asyncify suspend. Header (Int32):
  // [0] state 1=pending/0=done, [1] ok, [2] replyLen; bytes follow at offset 16.
  if (msg.cmd === "vfsSab") {
    try { vfsSab = msg.buf; vfsI32 = new Int32Array(msg.buf, 0, 4); }
    catch (_) { vfsSab = null; vfsI32 = null; }
    return;
  }
  // Enter sync-channel mode (multithread Qt clients). Must be the last message:
  // syncLoop blocks on Atomics.wait, so onmessage never fires again.
  if (msg.cmd === "syncChannel") {
    const qtHeap = msg.qtHeap, base = msg.base | 0;
    ready.then(() => syncLoop(qtHeap, base));
    return;
  }
  await ready;
  try {
    if (msg.cmd === "init") {
      self.postMessage(await doInit(msg.installMsl !== false));
    } else if (msg.cmd === "eval") {
      const { msg: reply, transfer } = await doEval(msg.src, msg.keepErrors);
      resetControl();
      // Transfer the result-file buffers (zero-copy) rather than clone them.
      self.postMessage(reply, transfer);
    } else if (msg.cmd === "abi") {
      // Typed OMEdit call. `id` correlates the reply with the page-side promise
      // (Module.omcAbiCall). `response` is the JSON string omc_abi_dispatch made.
      const response =
        typeof OmcModule.omc_abi === "function"
          ? await abiWithDownloads(msg.request)
          : JSON.stringify({ error: "omc_abi unavailable (omc built without the scripting_api feature)" });
      resetControl();
      self.postMessage({ kind: "abiResult", id: msg.id, response });
    } else if (msg.cmd === "vfsGet") {
      // OMEdit reads some files (library index, install manifests, visual.xml)
      // on the main thread; omc wrote them into this worker's store, so read them
      // back through the WASI surface and hand the bytes to the page's file engine.
      let bytes;
      try { bytes = wasiReadFile(msg.path); } catch (e) { bytes = undefined; }
      if (msg.sab) { vfsSabReply(bytes || null); }
      else {
        const transfer = bytes ? [bytes.buffer] : [];
        self.postMessage({ kind: "vfsResult", id: msg.id, bytes: bytes || null }, transfer);
      }
    } else if (msg.cmd === "vfsList") {
      // Directory enumeration for the page's QDir over worker-owned paths
      // (WASI fd_readdir). Returns [{ name, isDir }]; [] for a missing/empty dir.
      let entries;
      try { entries = wasi_readdir(msg.path) || []; } catch (e) { entries = []; }
      if (msg.sab) {
        // Names joined by '\n', dirs suffixed '/' (matches the postMessage decoder).
        const s = entries.map(en => en.name + (en.isDir ? "/" : "")).join("\n");
        vfsSabReply(new TextEncoder().encode(s));
      } else {
        self.postMessage({ kind: "vfsListResult", id: msg.id, entries });
      }
    } else if (msg.cmd === "vfsStat") {
      // WASI path_filestat_get's size (-1 if absent), for the file engine's size().
      let size;
      try { size = wasi_path_filestat_get(msg.path); } catch (e) { size = -1; }
      self.postMessage({ kind: "vfsStatResult", id: msg.id, size });
    }
  } catch (err) {
    // A trap inside omc must not silently wedge the shell: report it on the
    // channel the GUI is waiting on so it clears `busy`.
    if (msg.cmd === "init") {
      self.postMessage({ kind: "ready", ok: false, error: String(err) });
    } else if (msg.cmd === "abi") {
      self.postMessage({ kind: "abiResult", id: msg.id, response: JSON.stringify({ error: String(err) }) });
    } else {
      self.postMessage({ kind: "done", result: "", error: String(err), keep: true });
    }
  }
};
