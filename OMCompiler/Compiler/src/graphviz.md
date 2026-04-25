# Graphviz Module - Translation Assumptions

## Assumptions

### 1. `tick()` Implementation
- MetaModelica's `tick()` returns a globally unique integer. This is translated to an `AtomicI32` counter using `fetch_add`. This provides uniqueness within a single process but is NOT process-unique. For most graph generation use cases this should be sufficient.

### 2. `print()` Implementation
- MetaModelica's `print()` writes to stdout. This is translated to Rust's `print!()` macro which also writes to stdout.

### 3. `matchcontinue` Semantics
- The `matchcontinue` construct in MetaModelica tries cases sequentially, and if a case succeeds, continues to the next case (not breaking). This is translated to Rust using a series of `if` blocks with `bail!` to propagate failure.
- The pattern `if let Ok(v) = match_ok(&result) { return Ok(v); }` mimics the matchcontinue behavior: try the case, and if it succeeds, return the result; otherwise fall through to the next case.
- The `match_ok` helper returns `Ok(string)` for non-empty strings and `bail!("no match")` for empty ones, since in the original MetaModelica code, the `then` clause always produces a non-empty result when the pattern matches.

### 4. `make_label_req` three-case matchcontinue
- The original has three cases: `{s}` (single element), `{s1,s2}` (exactly two elements), and `s1 :: rest` (one or more elements).
- These are translated to helper functions `single_elem()`, `two_elem()`, and `cons_elem()` that check list lengths.
- The order matters: single and two-element cases are checked first (as in the original), then the general cons case.

### 5. `make_attr_req` two-case matchcontinue
- Similar to `make_label_req`, uses `single_attr_elem()` and `cons_attr_elem()` helpers.

### 6. `dump_children` matchcontinue with two parameters
- The original uses `match (inIdent, inChildren)` with cases `(_, {})` (empty children) and `(parent, (node :: rest))`.
- Translated to a direct check: if children is empty, return `Ok(())`; otherwise destructure the first element and recurse.

### 7. Type Mappings
- `Graphviz.Type` → `String` (type alias `Type_`, prefixed with underscore to avoid collision with Rust's `Type`)
- `Graphviz.Ident` → `String` (type alias `Ident`)
- `Graphviz.Label` → `String` (type alias `Label`)
- `list<Node>` → `im::Vector<Node>` (aliased as `Children`)
- `list<Attribute>` → `im::Vector<Attribute>` (aliased as `Attributes`)
- `list<Label>` → `im::Vector<Label>`

### 8. `box` Constant
- The `constant Attribute box = ATTR("shape","box")` is translated to a function `box_attr()` since Rust `const` cannot call `String::from()`. The `BOX` const is kept as a placeholder.

### 9. LNODE `labelLst` Field Ordering
- In the original, `lbl_1 = typ::lbl` prepends `typ` to the front of `lbl`. The `list_prepend` helper correctly implements this by creating a new list with the item first, followed by all elements of the original list.

## Things That Might Not Work As Expected

### 1. Thread Safety of `tick()`
- The `AtomicI32` counter is safe across threads, but the counter is shared globally. If multiple threads call `dump()` concurrently, node names will be unique but interleaved. This is consistent with the original behavior.

### 2. Integer Overflow of `tick()`
- `AtomicI32` will silently overflow (wrap around) at `i32::MAX`. This is extremely unlikely in practice but could theoretically produce duplicate node names after ~2.1 billion calls.

### 3. `make_attr_req` empty input
- If an empty attribute list is passed to `make_attr_req`, it will return an error (`bail!`). The caller `make_attr()` uses `unwrap_or_default()` which converts errors to an empty string, resulting in `"[]"`. This matches the expected Graphviz syntax for empty attributes.

### 4. Performance
- The `list_prepend` helper creates a new `Vec` and copies all elements. For large lists this could be slow. The original MetaModelica `::` (cons) is O(1), but the Rust equivalent using `im::Vector` is also O(1) for `push_front`. However, since `im::Vector 15.x` in this project does not have a working `push_front` that returns a new list (it appears to be the mutable API), the `list_prepend` helper uses a Vec allocation. Consider using `im::Deque` if O(1) prepend is needed.

### 5. `nodename` ignores input
- The `nodename` function generates a unique name regardless of the input string. The original function also does this: `s := stringAppend("GVNOD", intString(tick()))`. The input parameter is accepted but not used in the name generation.
