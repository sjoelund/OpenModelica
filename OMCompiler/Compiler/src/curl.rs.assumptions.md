# Assumptions for curl.rs (Util/Curl.mo)

## Type Mapping

- `list<tuple<list<String>, String>>` mapped to `*mut c_void` (raw pointer)
  because the C function `om_curl_multi_download` operates on the MetaModelica
  MMC linked list representation internally.
- `Integer` mapped to `i32` (as per CLAUDE.md conventions).
- `Boolean` mapped to `bool`.

## FFI Binding

- The function `om_curl_multi_download` is declared as an external C function
  that lives in the OpenModelica runtime library (`libomcruntime` or similar).
  This function is implemented in `runtime/om_curl.c`.
- The FFI binding assumes the C function signature:
  `int om_curl_multi_download(void *url_path_list, int max_parallel)`
- The actual link to this function requires the OpenModelica runtime library
  to be available at link time.

## Default Value for `maxParallel`

- MetaModelica: `maxParallel = Config.noProc()` (number of CPU cores).
- Since `Config` has not been translated to Rust, a hardcoded default of 1
  is used when `maxParallel <= 0`. A proper Config translation would provide
  the number of available cores.

## Functionality

- The C implementation uses libcurl's multi interface for parallel downloads.
- Mirror retry is built-in: on failure, the next URL in the list is tried.
- Files are written to `.tmp` files first, then renamed to the target name
  on success.
- Error messages are sent via `c_add_message` in the C runtime (not exposed
  through this Rust wrapper).

## Things That Might Not Work as Expected

1. **Linking**: The `om_curl_multi_download` symbol must be available at link
   time. If building standalone without the OpenModelica runtime, this will
   fail to link.
2. **List pointer passing**: Callers must pass the linked list pointer in
   the exact format expected by the MMC runtime. Converting from Rust
   `Vec<(Vec<String>, String)>` would require implementing MMC list
   construction, which is non-trivial.
3. **Thread safety**: The C implementation uses `curl_global_init` and
   `curl_global_cleanup` which are not thread-safe if called concurrently
   from different Rust threads.
