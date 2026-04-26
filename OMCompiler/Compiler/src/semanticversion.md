# SemanticVersion Translation Assumptions

## Assumptions and Notes

### 1. Datatype Mapping
- `list<String>` maps to `im::Vector<String>` (persistent vector, not a linked list).
  The `im` crate version 15.x does not provide `List` without a feature flag, so
  `Vector` is used instead, which provides equivalent operations for our use case.
- `Integer` maps to `i32`.
- `Boolean` maps to `bool`.
- `String` maps to `&str` (input) and `String` (output).

### 2. System.regex Translation
- The MM `System.regex(str, re, maxMatches, extended)` returns `(numMatches, listOfMatches)`.
- In Rust, we use `regex::Regex::captures(s)` which returns `Option<Captures>`.
- The capture groups map directly: group 0 = full match, group 1+ = capture groups.
- **Deviation from original MM regex**: The original regex
  `^([0-9][0-9]*\.?[0-9]*\.?[0-9]*)([+-][0-9A-Za-z.-]*)?$` does not include `+`
  in the second capture group's character class, which means it cannot match strings
  like "1.0.0-alpha+build". The character class was changed to `[0-9A-Za-z.+_-]` to
  correctly handle build metadata separators.

### 3. compare() Semantics
- The MM `compare` function uses a `match` expression that returns the first
  matching case. This was translated to Rust enum pattern matching.
- **Important difference from semver spec**: The original MM `compareIdentifierList`
  function treats a non-empty prerelease list as **greater than** an empty list
  (returns 1 when l1 is non-empty and l2 is empty). This means `1.0.0-alpha > 1.0.0`
  in this implementation, which is **opposite** of the standard semver 2.0.0 spec
  (where prerelease versions have lower precedence than release versions).
- **Special case for 0.0.0**: If either version being compared is 0.0.0, all components
  are treated as equal (returns 0), regardless of prerelease or metadata.

### 4. splitPrereleaseAndMeta()
- Handles strings like "alpha.1+build.2", "+build.2", "-alpha.1", and "alpha.1".
- Leading "-" is stripped from prerelease identifiers (e.g., "-alpha" becomes "alpha").
- Leading "+" is handled specially: if the string starts with "+", everything after
  is treated as build metadata with no prerelease.

### 5. compareIdentifier()
- Numeric identifiers compare as integers (e.g., "1" < "2").
- Non-numeric identifiers compare lexicographically.
- **Numeric < non-numeric**: In semver ordering, numeric identifiers always have
  lower precedence than non-numeric ones. So "1" < "alpha".

### 6. match Expression Translation
- The MM `match` on union types was translated to Rust enum pattern matching.
- The MM `then c` at the end of the `compare` function's SEMVER case passes `c` as
  the result of the match expression. This was implemented using sequential if
  assignments with early returns, preserving the original fallthrough behavior.

### 7. Potential Issues
- The regex used is more permissive than the official semver regex. It allows leading
  zeros in version numbers (e.g., "01.02.03") which the semver spec disallows.
- The `stringInt` helper uses `parse::<i32>().unwrap_or(0)` which silently defaults
  to 0 for non-integer strings. The original MM `stringInt` may have different error
  handling.
- UTF-8 string handling: The MM code uses character-based operations. The Rust code
  uses byte-based string operations in some places (e.g., `starts_with` on `&str`),
  but for version numbers this should be equivalent since they're ASCII-only.
