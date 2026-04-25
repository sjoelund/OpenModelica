# IOStream Translation Assumptions

## Datatype Mappings

- `list<String>` -> `im::Vector<String>` with type alias `List<String>`
  - The `im` crate v15.1.0 does not have a `List` type. The CLAUDE.md
    instructions say to use `im::List`, but `im::Vector<T>` is used here
    (consistent with all other translations in this project).
  - `im::Vector` is a random-access persistent vector (O(log N) access).
    `im::List` would be a cons-list (O(1) cons at front). This may have
    minor performance implications for code that relies on O(1) prepending,
    though `push_front` on `im::Vector` is copy-on-write and still efficient.
- `Integer` -> `i32`
- `Boolean` -> `bool`
- `String` -> `String`

## FFI Dependencies

- This module depends on `iostreamext` for file and buffer operations.
- The `iostreamext` module wraps C FFI functions from the `omcruntime` library.
- The `print_reversedList` C FFI function takes an opaque `*mut c_void`
  pointer to a C list structure, not an `im::Vector`. Since Rust
  `im::Vector` cannot be passed to this C function directly, list stream
  printing iterates over elements individually instead.

## Behavior Differences

- **clear for list streams**: The original MetaModelica code uses
  `IOStream(name, ty, LIST_DATA({}))` to create a new stream with an empty
  list. In Rust, this returns a new `IOStream` with the same name and type
  but empty `ListData`. This is correct behavior.

- **clear for file/buffer streams**: Uses `clearFile`/`clearBuffer` from the
  C FFI. These truncate/clear the underlying resource in place.

- **close for list/buffer streams**: The original code uses `matchcontinue`
  with an `else` clause that returns the stream unchanged for list and
  buffer streams. In Rust, we simply return the stream as-is for these cases.

- **empty for file/buffer streams**: The original code has a `match` that
  only handles `LIST_DATA`. For file and buffer streams, there is no match
  case, so the function would fall through without returning a value (a
  compile error in MetaModelica). In Rust, we return `false` for
  non-list streams as a reasonable default (though strictly this is
  undefined behavior in the original).

## matchcontinue Handling

The original code uses `matchcontinue` in `close` and `clear` functions.
These have been translated to sequential `match` expressions since:
- `close`: Only `FILE` has special handling; list/buffer are no-ops (handled
  by the `else` clause).
- `clear`: Three distinct cases (file, list, buffer) that are mutually
  exclusive, so regular `match` works correctly.

## Thread Safety

The `im::Vector` type uses reference counting internally (Arc-backed), so
cloned vectors share structure. This is safe across threads.

## Constants

- `stdInput = 0`, `stdOutput = 1`, `stdError = 2` are exported as `STD_INPUT`,
  `STD_OUTPUT`, `STD_ERROR` constants.

## Notes

- The `print` function for list streams does NOT use the C `printReversedList`
  FFI because it expects a C list pointer, not an `im::Vector`. Instead,
  it iterates over the list in reverse and prints each element directly
  using `print!`/`eprint!` macros.
- The `to_string` function for list streams reverses the list and
  concatenates all strings, replicating the behavior of the C
  `appendReversedList` function.
- The `append_list_stream` function requires the source stream to be a list
  stream. If a non-list stream is passed, it returns an error.
