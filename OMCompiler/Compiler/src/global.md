# Global Package Translation Notes

## Assumptions and Potential Issues

### Runtime Stubs
The `setGlobalRoot` and `none()` (NONE()) functions are translated as stubs because they
require OpenModelica runtime bindings that are not yet available. The `initialize()` function
uses these stubs and will not perform actual initialization until the runtime bindings are
provided.

- `setGlobalRoot` currently takes a generic `T` and discards both arguments - real implementation
  requires the OpenModelica global root storage system.
- `none()` returns `None` (Option::None) as a placeholder for the OpenModelica NONE value.
- Empty sets `{}` are represented as `Vec::<()>::new()` - this may differ from the actual
  OpenModelica empty set representation.

### Type Mappings
- All `Integer` constants map directly to `i32`.
- Constant names converted from `camelCase` to `SCREAMING_SNAKE_CASE` following Rust conventions.

### Unused Constants
Most constants are not yet used by other translated modules. They will be referenced as those
modules are translated. The dead code warnings should disappear as more modules are translated.

### Thread-Local Roots vs Global Roots
The original code distinguishes between thread-local roots (indices 0-3) and global roots
(indices 9+). This distinction is preserved in the constant names but not in the Rust types,
as both use `i32`.

### System.tick Indexes
The indexes under "indexes in System.tick" (tmpVariableIndex, iteratorIndex, etc.) are
included in the same module for organizational clarity, matching the original Global.mo
structure.
