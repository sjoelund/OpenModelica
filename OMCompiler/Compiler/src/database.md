# Database Module - Translation Notes

## Source
- Original: `Util/Database.mo` (OpenModelica)
- Translation: `src/database.rs`

## Summary
Translated the OpenModelica Database package, which provides SQLite database functionality via C externs linked to the `omcruntime` library.

## C FFI Bindings

Two C functions are bound from the `omcruntime` library:

| MetaModelica | C Symbol | Signature |
|---|---|---|
| `Database_open(index, name)` | `Database_open` | `int Database_open(int index, const char* name)` |
| `Database_query(index, sql, result)` | `Database_query` | `int Database_query(int index, const char* sql, void** result)` |

## Assumptions

1. **C symbol names**: The MetaModelica code references `Database_open` and `Database_query` as external C symbols. The actual implementation in `runtime/Database.c` uses `DatabaseImpl_open` and `DatabaseImpl_query` internally, but the external C symbols are named `Database_open` and `Database_query` as specified in the `external "C"` annotations.

2. **Result parameter**: The `Database_query` C function takes a `void** result` parameter that is used by the SQLite callback mechanism. The safe Rust wrapper currently passes `null` for this parameter since the actual result processing requires complex SQLite callback infrastructure. This is a **limitation** - full result parsing is not implemented.

3. **Database index limit**: The C code uses a fixed array of 1024 databases (`DATABASES[1024]`). The Rust code respects this limit via the error codes defined in the C code.

4. **Error codes**: Custom error codes (500 for index overflow, 501 for not initialized) are defined in the C code and mapped to Rust constants.

5. **Thread safety**: The underlying C code is not thread-safe. The SQLite databases are stored in a global array, and concurrent access from multiple threads could cause data races.

6. **String encoding**: The C code expects null-terminated C strings (UTF-8). The Rust wrapper uses `CString` which guarantees null termination.

7. **In-memory databases**: The C code supports SQLite's `:memory:` special database name, which is passed through as-is.

## Potential Issues

1. **Missing FFI symbols**: The Rust code expects `Database_open` and `Database_query` symbols to be exported by the `omcruntime` library. If the C library exports them under different names (e.g., `DatabaseImpl_open`), linking will fail.

2. **No result parsing**: The `query` function currently only checks for success/failure but does not parse the query results. The original MetaModelica `query` function returns `list<tuple<String,String>>` (column names mapped to values). Implementing full result parsing would require:
   - Passing a callback struct to the FFI function
   - Collecting results in a thread-safe manner
   - Converting SQLite result data to Rust types

3. **No database handle management**: The Rust code does not track which database indices are currently open or provide a way to close databases. This is delegated to the C runtime.

4. **No SQLite version checking**: The C code calls `sqlite3_libversion()` in `DatabaseImpl_init` but the Rust code does not call this initialization function. If the C library needs initialization before use, it should be called.

5. **Edition 2024 safety**: This translation uses Rust 2024 edition which requires `unsafe extern "C"` blocks. If the project migrates editions, the FFI syntax may need adjustment.

## Datatype Mapping

| MetaModelica | Rust |
|---|---|
| `Integer` | `i32` |
| `String` | `&str` / `String` |
| `list<tuple<String,String>>` | Not fully implemented (see note above) |

## Dependencies

- `std::ffi::CString` - for safe C string conversion
- `std::os::raw` - for FFI-compatible types
