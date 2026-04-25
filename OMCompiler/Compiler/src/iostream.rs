//! Translation of Util/IOStream.mo
//!
//! This module provides stream utilities for file, list, and buffer-based I/O.
//! It wraps the iostream_ext FFI layer for file and buffer operations,
//! and uses im::Vector for list-stream operations.
//!
//! Stream types:
//! - File streams: write/read files via C FFI
//! - List streams: hold a list<String> in memory
//! - Buffer streams: write/read external C buffers via C FFI

use anyhow::{bail, Result};

use crate::iostreamext;

// Persistent list type (mapped to im::Vector since im 15.x has no List)
type List<T> = im::Vector<T>;

// ============================================================================
// Union types
// ============================================================================

/// Represents the type of a stream.
#[derive(Debug, Clone)]
pub enum IOStreamType {
    /// File stream - holds the file name
    File { name: String },
    /// List stream - in-memory list of strings
    List,
    /// Buffer stream - external C buffer
    Buffer,
}

/// The data carried by each stream variant.
#[derive(Debug, Clone)]
pub enum IOStreamData {
    /// File handle ID (returned by omcruntime)
    FileData(i32),
    /// In-memory list of strings
    ListData(List<String>),
    /// Buffer handle ID (returned by omcruntime)
    BufferData(i32),
}

/// The IOStream union type wrapping all stream kinds.
#[derive(Debug, Clone)]
pub enum IOStream {
    IOSTREAM {
        name: String,
        ty: IOStreamType,
        data: IOStreamData,
    },
}

impl IOStream {
    fn name(&self) -> &str {
        match self {
            IOStream::IOSTREAM { name, .. } => name,
        }
    }

    fn ty(&self) -> &IOStreamType {
        match self {
            IOStream::IOSTREAM { ty, .. } => ty,
        }
    }

    fn data(&self) -> &IOStreamData {
        match self {
            IOStream::IOSTREAM { data, .. } => data,
        }
    }
}

// ============================================================================
// Constants
// ============================================================================

/// Standard input (reserved, value 0).
pub const STD_INPUT: i32 = 0;

/// Standard output (value 1).
pub const STD_OUTPUT: i32 = 1;

/// Standard error (value 2).
pub const STD_ERROR: i32 = 2;

/// An empty stream of list type.
pub fn empty_stream_of_type_list() -> IOStream {
    IOStream::IOSTREAM {
        name: "emptyStreamOfTypeList".to_string(),
        ty: IOStreamType::List,
        data: IOStreamData::ListData(im::vector![]),
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Create a new stream of the given type.
///
/// - FILE: opens/creates a file stream with the given file name
/// - LIST: creates an in-memory list stream
/// - BUFFER: creates a buffer stream
///
/// # Errors
/// Returns an error if the underlying C resource creation fails.
pub fn create(stream_name: &str, stream_type: &IOStreamType) -> Result<IOStream> {
    match stream_type {
        IOStreamType::File { name } => {
            let file_id = iostreamext::create_file(name);
            if file_id < 0 {
                bail!("failed to create file stream: {}", name);
            }
            Ok(IOStream::IOSTREAM {
                name: stream_name.to_string(),
                ty: stream_type.clone(),
                data: IOStreamData::FileData(file_id),
            })
        }
        IOStreamType::List => Ok(IOStream::IOSTREAM {
            name: stream_name.to_string(),
            ty: stream_type.clone(),
            data: IOStreamData::ListData(im::vector![]),
        }),
        IOStreamType::Buffer => {
            let buffer_id = iostreamext::create_buffer();
            if buffer_id < 0 {
                bail!("failed to create buffer stream");
            }
            Ok(IOStream::IOSTREAM {
                name: stream_name.to_string(),
                ty: stream_type.clone(),
                data: IOStreamData::BufferData(buffer_id),
            })
        }
    }
}

/// Append a string to a stream.
///
/// - FILE: appends to the file
/// - LIST: prepends the string to the in-memory list
/// - BUFFER: appends to the buffer
///
/// # Errors
/// Returns an error if the underlying C operation fails.
pub fn append(stream: &IOStream, in_string: &str) -> Result<IOStream> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            iostreamext::append_file(*file_id, in_string);
            Ok(stream.clone())
        }
        IOStream::IOSTREAM {
            name,
            ty,
            data: IOStreamData::ListData(list_data),
        } => {
            // Prepends in_string to the list (MetaModelica :: operator)
            let mut new_list = list_data.clone();
            new_list.push_front(in_string.to_string());
            Ok(IOStream::IOSTREAM {
                name: name.clone(),
                ty: ty.clone(),
                data: IOStreamData::ListData(new_list),
            })
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            iostreamext::append_buffer(*buffer_id, in_string);
            Ok(stream.clone())
        }
    }
}

/// Append a list of strings to a stream by repeatedly calling `append`.
///
/// # Errors
/// Returns an error if any individual append fails.
pub fn append_list(stream: &IOStream, in_string_list: List<String>) -> Result<IOStream> {
    let mut result = stream.clone();
    for s in in_string_list {
        result = append(&result, &s)?;
    }
    Ok(result)
}

/// Append a list of strings to a stream, prepending them in reverse order
/// for list streams (for efficiency).
///
/// For list streams, this prepends the entire data list to the stream's
/// current list. For file/buffer streams, appends each string individually.
///
/// # Errors
/// Returns an error if the underlying C operation fails.
pub fn append_list_reverse(stream: &IOStream, data: List<String>) -> Result<IOStream> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            for s in data {
                iostreamext::append_file(*file_id, &s);
            }
            Ok(stream.clone())
        }
        IOStream::IOSTREAM {
            name,
            ty,
            data: IOStreamData::ListData(list_data),
        } => {
            // Prepend data to list_data (MetaModelica listAppend)
            let new_list = data.into_iter().chain(list_data.clone()).collect();
            Ok(IOStream::IOSTREAM {
                name: name.clone(),
                ty: ty.clone(),
                data: IOStreamData::ListData(new_list),
            })
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            for s in data {
                iostreamext::append_buffer(*buffer_id, &s);
            }
            Ok(stream.clone())
        }
    }
}

/// Transfer data from a source list stream to a destination stream.
///
/// Reads the list data from the source stream and appends it to the
/// destination stream using `append_list_reverse`.
///
/// # Errors
/// Returns an error if the destination append operation fails.
pub fn append_list_stream(src_stream: &IOStream, dst_stream: &IOStream) -> Result<IOStream> {
    let list_data = match src_stream {
        IOStream::IOSTREAM {
            data: IOStreamData::ListData(lst),
            ..
        } => lst.clone(),
        _ => bail!("source stream is not a list stream"),
    };
    append_list_reverse(dst_stream, list_data)
}

/// Close a stream.
///
/// For file streams, this closes the underlying file handle.
/// For list and buffer streams, this is a no-op.
///
/// # Errors
/// Returns an error if the close operation fails.
pub fn close(stream: &IOStream) -> Result<IOStream> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            iostreamext::close_file(*file_id);
            Ok(stream.clone())
        }
        // List and buffer streams: close does nothing
        _ => Ok(stream.clone()),
    }
}

/// Delete a stream's underlying resource.
///
/// For file streams, deletes the file.
/// For list streams, this is a no-op.
/// For buffer streams, deletes the buffer.
///
/// # Errors
/// Returns an error if the delete operation fails.
pub fn delete(stream: &IOStream) -> Result<()> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            iostreamext::delete_file(*file_id);
            Ok(())
        }
        IOStream::IOSTREAM {
            data: IOStreamData::ListData(_),
            ..
        } => {
            // List streams: delete does nothing
            Ok(())
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            iostreamext::delete_buffer(*buffer_id);
            Ok(())
        }
    }
}

/// Clear a stream's contents.
///
/// For file streams, truncates the file.
/// For list streams, returns a new stream with an empty list.
/// For buffer streams, empties the buffer.
///
/// # Errors
/// Returns an error if the clear operation fails.
pub fn clear(stream: &IOStream) -> Result<IOStream> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            iostreamext::clear_file(*file_id);
            Ok(stream.clone())
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            iostreamext::clear_buffer(*buffer_id);
            Ok(stream.clone())
        }
        IOStream::IOSTREAM { name, ty, .. } => {
            // List stream or default: return new stream with empty list
            Ok(IOStream::IOSTREAM {
                name: name.clone(),
                ty: ty.clone(),
                data: IOStreamData::ListData(im::vector![]),
            })
        }
    }
}

/// Check if a stream is empty.
///
/// For list streams, checks if the internal list is empty.
/// For file and buffer streams, always returns false
/// (since there's no efficient way to check without reading).
///
/// Note: This function only works with list streams in practice.
/// For file/buffer streams, use `to_string` to read content and check length.
pub fn is_empty(stream: &IOStream) -> bool {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::ListData(list_data),
            ..
        } => list_data.is_empty(),
        _ => false,
    }
}

/// Convert a stream to a string.
///
/// For file streams, reads the file content.
/// For list streams, concatenates the list in reverse order (MetaModelica
/// appendReversedList).
/// For buffer streams, reads the buffer content.
///
/// # Errors
/// Returns an error if the read operation fails.
pub fn to_string(stream: &IOStream) -> Result<String> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            let content = iostreamext::read_file(*file_id);
            Ok(content)
        }
        IOStream::IOSTREAM {
            data: IOStreamData::ListData(list_data),
            ..
        } => {
            // Concatenate reversed list (MetaModelica appendReversedList)
            let reversed: List<String> = list_data.clone().into_iter().rev().collect();
            let mut concatenated = String::new();
            for s in reversed {
                concatenated.push_str(&s);
            }
            Ok(concatenated)
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            let content = iostreamext::read_buffer(*buffer_id);
            Ok(content)
        }
    }
}

/// Print stream content to standard output or standard error.
///
/// For list streams, prints each string in the list (reversed order)
/// to the specified output destination.
///
/// # Errors
/// Returns an error if the print operation fails.
pub fn print(stream: &IOStream, where_to_print: i32) -> Result<()> {
    match stream {
        IOStream::IOSTREAM {
            data: IOStreamData::FileData(file_id),
            ..
        } => {
            iostreamext::print_file(*file_id, where_to_print);
            Ok(())
        }
        IOStream::IOSTREAM {
            data: IOStreamData::ListData(list_data),
            ..
        } => {
            // The C FFI printReversedList takes an opaque pointer to a C list,
            // not an im::Vector. Instead we print each element directly.
            let reversed: List<&str> = list_data
                .iter()
                .map(|s: &String| s.as_str())
                .rev()
                .collect();
            for s in reversed {
                match where_to_print {
                    2 => eprint!("{}", s),
                    _ => print!("{}", s),
                }
            }
            Ok(())
        }
        IOStream::IOSTREAM {
            data: IOStreamData::BufferData(buffer_id),
            ..
        } => {
            iostreamext::print_buffer(*buffer_id, where_to_print);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_list_stream() {
        let stream = create("test", &IOStreamType::List).unwrap();
        assert!(is_empty(&stream));
    }

    #[test]
    fn test_append_to_list_stream() {
        let stream = create("test", &IOStreamType::List).unwrap();
        let stream = append(&stream, "hello").unwrap();
        let s = to_string(&stream).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_append_multiple() {
        let stream = create("test", &IOStreamType::List).unwrap();
        let stream = append(&stream, "world").unwrap();
        let stream = append(&stream, "hello").unwrap();
        let s = to_string(&stream).unwrap();
        // prepend order: last append is first in list, reversed for concat
        assert_eq!(s, "helloworld");
    }

    #[test]
    fn test_empty_stream() {
        let stream = empty_stream_of_type_list();
        assert!(is_empty(&stream));
    }

    #[test]
    fn test_clear_list_stream() {
        let stream = create("test", &IOStreamType::List).unwrap();
        let stream = append(&stream, "data").unwrap();
        let stream = clear(&stream).unwrap();
        assert!(is_empty(&stream));
    }

    #[test]
    fn test_constants() {
        assert_eq!(STD_INPUT, 0);
        assert_eq!(STD_OUTPUT, 1);
        assert_eq!(STD_ERROR, 2);
    }
}
