// Manually written file.
//
// Rust port of `OMCompiler/Compiler/Util/Print.mo`'s `external "C"`
// declarations into `OMCompiler/Compiler/runtime/printimpl.c`. The C
// runtime maintains two per-thread `std::string` buffers (a normal print
// buffer and an error buffer), plus a stack of saved buffers used by
// templates that want to render a subtree, capture the text, and then
// roll back to the prior buffer.
//
// We mirror that design with `thread_local!` `RefCell`s. The buffers are
// kept as `String` for cheap byte-level append; conversion to `ArcStr`
// happens only at the boundary functions `getString` / `getErrorString`
// which the MetaModelica callers consume.

#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write as _;

use anyhow::{Context, Result};
use arcstr::ArcStr;

#[derive(Default)]
struct PrintState {
    buf: String,
    err_buf: String,
    saved: HashMap<i32, String>,
    next_handle: i32,
}

thread_local! {
    static STATE: RefCell<PrintState> = RefCell::new(PrintState::default());
}

fn with<R>(f: impl FnOnce(&mut PrintState) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

pub fn clearBuf() {
    with(|s| s.buf.clear());
}

pub fn clearErrorBuf() {
    with(|s| s.err_buf.clear());
}

pub fn getBufLength() -> i32 {
    with(|s| s.buf.len() as i32)
}

pub fn getErrorString() -> Result<ArcStr> {
    Ok(with(|s| ArcStr::from(s.err_buf.as_str())))
}

pub fn getString() -> Result<ArcStr> {
    Ok(with(|s| ArcStr::from(s.buf.as_str())))
}

pub fn hasBufNewLineAtEnd() -> bool {
    with(|s| s.buf.ends_with('\n'))
}

pub fn printBuf(inString: ArcStr) -> Result<()> {
    with(|s| s.buf.push_str(&inString));
    Ok(())
}

pub fn printBufNewLine() -> Result<()> {
    with(|s| s.buf.push('\n'));
    Ok(())
}

pub fn printBufSpace(inNumOfSpaces: i32) -> Result<()> {
    if inNumOfSpaces > 0 {
        with(|s| {
            for _ in 0..inNumOfSpaces {
                s.buf.push(' ');
            }
        });
    }
    Ok(())
}

pub fn printErrorBuf(inString: ArcStr) -> Result<()> {
    with(|s| s.err_buf.push_str(&inString));
    Ok(())
}

/// Pop the saved buffer named by `handle` back into the active print
/// buffer. The C runtime swaps the slot in-place; we drop the saved
/// entry so handles are single-use.
pub fn restoreBuf(handle: i32) -> Result<()> {
    with(|s| {
        if let Some(prev) = s.saved.remove(&handle) {
            s.buf = prev;
        }
    });
    Ok(())
}

/// Save the active print buffer under a fresh handle and return that
/// handle to the caller. The buffer is reset to empty so the caller can
/// render a subtree in isolation before calling [`restoreBuf`] to splice
/// the saved text back.
pub fn saveAndClearBuf() -> Result<i32> {
    Ok(with(|s| {
        s.next_handle = s.next_handle.wrapping_add(1);
        if s.next_handle == 0 {
            s.next_handle = 1;
        }
        let h = s.next_handle;
        let prev = std::mem::take(&mut s.buf);
        s.saved.insert(h, prev);
        h
    }))
}

pub fn writeBuf(filename: ArcStr) -> Result<()> {
    let contents = with(|s| s.buf.clone());
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename.as_str())
        .with_context(|| format!("Print.writeBuf: cannot open {filename}"))?;
    f.write_all(contents.as_bytes())
        .with_context(|| format!("Print.writeBuf: write failed for {filename}"))?;
    Ok(())
}

/// Same as [`writeBuf`] but normalises any `\r\n` / lone `\r` line
/// terminators in the buffer to a single `\n` on disk. Mirrors the C
/// runtime's `Print_writeBufConvertLines`, which exists so that compiler
/// output stays in Unix-style line endings even when the buffer was
/// fed cross-platform text.
pub fn writeBufConvertLines(filename: ArcStr) -> Result<()> {
    let contents = with(|s| s.buf.clone());
    let mut normalised = String::with_capacity(contents.len());
    let mut chars = contents.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            normalised.push('\n');
            if matches!(chars.peek(), Some('\n')) {
                chars.next();
            }
        } else {
            normalised.push(c);
        }
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename.as_str())
        .with_context(|| format!("Print.writeBufConvertLines: cannot open {filename}"))?;
    f.write_all(normalised.as_bytes())
        .with_context(|| format!("Print.writeBufConvertLines: write failed for {filename}"))?;
    Ok(())
}
