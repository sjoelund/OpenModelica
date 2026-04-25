# BaseAvlSet Translation Assumptions

## Generic Types (replaceable keyword)

The MetaModelica `replaceable` keyword is translated to Rust generics:
- `Key` → `K: Ord + Clone` — Any type implementing `Ord` and `Clone`
- `ValueNode` → `K` — Same as Key since `ValueNode = Key` in the original
- `Tree` → `Tree<K>` — Generic over the key type

## keyCompare / keyStr

- `keyCompare` is replaced by `Ord::cmp`, converted to -1/0/1 via `key_compare()` function
- `keyStr` / `printNodeStr` uses `std::fmt::Display` trait bound — consumers must implement `Display` for their key types if they want to use printing functions

## List Type

- MetaModelica `list<Key>` maps to `im::Vector<K>` (used as a persistent list)
- The `im::List` type is not available in the im crate at the crate root; `im::Vector` is used instead

## listKeys Order

The original MO `listKeys` function has a suspicious implementation where the `then` clause returns the result of `listKeys(left, lst)` rather than `lst`. The Rust translation implements the semantically correct in-order traversal (left → key → right) producing ascending sorted order. This matches the documented intent: "Converts the tree to a flat list of keys (in order)."

## listKeysReverse Order

The MO `listKeysReverse` traverses left-to-right but still prepends, producing descending order. The Rust translation does the same, producing descending sorted order.

## smallestKey Direction

The original MO `smallestKey` finds the "smallest" by going right (`NODE(right = EMPTY())` is the base case). This is unusual since typically the smallest element is found by going left. This implementation matches the MO source exactly — the naming is misleading but the behavior is preserved.

## referenceEq

The original MO references `referenceEq(t1, t2)` in `referenceEqOrEmpty`, which checks if two trees are the same reference. In Rust, this is translated to structural equality (`t1 == t2`) since Rust enums don't have reference identity in the same way. This is a semantic difference that should not affect correctness for AVL sets.

## Option<ValueNode> in printTreeStr2

The MO declares `Option<ValueNode> val_node;` as a protected variable but never uses it. This appears to be a type hint for the match statement. In Rust, this variable is omitted entirely.

## setTreeLeftRight Optimization

The original has an optimization: if the new children are reference-equal to the old children, return the original tree unchanged. In Rust, this uses structural equality (`==`) instead of reference equality. The behavior is the same for most cases.

## Balance Algorithm

The AVL balancing algorithm uses standard single/double rotation patterns:
- Left-heavy (diff > 1): right rotation (or right-left double rotation)
- Right-heavy (diff < -1): left rotation (or left-right double rotation)

## addList

The MO `addList` function uses `input output Tree tree` — it mutates the input tree parameter. In Rust, this is translated to taking ownership of the tree and returning a new tree, which is the idiomatic Rust equivalent.

## printTreeStr / printTreeStr2

The visual tree printing uses UTF-8 box-drawing characters (┌, └, ─, │). The `Display` trait bound on the key type is required for these functions.

## No matchcontinue Needed

None of the functions in BaseAvlSet.mo use `matchcontinue`, so the `matchcontinue` translation pattern is not used in this module.

## Compilation

- All 13 unit tests pass
- The code compiles with `cargo check` and `cargo test`
- Warnings about unused code are expected since main.rs only imports `mod baseavlset`
