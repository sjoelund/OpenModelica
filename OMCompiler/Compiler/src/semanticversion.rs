//! Translation of Util/SemanticVersion.mo
//!
//! This module provides semantic version parsing and comparison translated from
//! MetaModelica, following semver.org specification.

use anyhow::Result;
use im::Vector;

// ============================================================================
// Data Types
// ============================================================================

/// Semantic version union type (SEMVER | NONSEMVER).
#[derive(Debug, Clone, PartialEq)]
pub enum Version {
    SemVer {
        major: i32,
        minor: i32,
        patch: i32,
        prerelease: Vector<String>,
        meta: Vector<String>,
    },
    NonSemVer {
        version: String,
    },
}

// ============================================================================
// parse
// ============================================================================

/// Parse a version string into a Version.
///
/// If nonsemver_as_zero_zero_zero is false and the string does not match
/// a semver pattern, returns NonSemVer(s).
pub fn parse(s: &str, nonsemver_as_zero_zero_zero: bool) -> Version {
    // Added '+' to the character class to match build metadata separators
    // (original MM regex didn't include '+' which broke parsing of strings like "1.0.0-alpha+build")
    let semver_regex =
        regex::Regex::new(r"^([0-9][0-9]*\.?[0-9]*\.?[0-9]*)([+-][0-9A-Za-z.+_-]*)?$")
            .expect("Invalid semver regex");

    // System.regex returns (numMatches, listOfMatches)
    // If numMatches < 2, the regex did not match
    let caps = match semver_regex.captures(s) {
        Some(c) => c,
        None => {
            // No regex match
            if s.is_empty() {
                return Version::NonSemVer {
                    version: String::new(),
                };
            }
            if nonsemver_as_zero_zero_zero {
                let (prerelease_lst, meta_lst) = split_prerelease_and_meta(s);
                return Version::SemVer {
                    major: 0,
                    minor: 0,
                    patch: 0,
                    prerelease: prerelease_lst,
                    meta: meta_lst,
                };
            }
            return Version::NonSemVer {
                version: s.to_string(),
            };
        }
    };

    // caps[0] = full match, caps[1] = version number, caps[2] = prerelease/meta
    let major_str: &str = caps.get(1).map(|m| m.as_str()).unwrap_or("0");
    let pre_meta: Option<&str> = caps.get(2).map(|m| m.as_str());

    // Split versions by "."
    let parts: Vector<String> = string_split_at_char(major_str, ".");
    let major: i32 = string_int(parts.get(0).map(|s| s.as_str()).unwrap_or("0"));
    let minor: i32 = match parts.get(1) {
        Some(s) => string_int(s),
        None => 0,
    };
    let patch: i32 = match parts.get(2) {
        Some(s) => string_int(s),
        None => 0,
    };

    let (prerelease_lst, meta_lst) = match pre_meta {
        Some(p) => split_prerelease_and_meta(p),
        None => (Vector::new(), Vector::new()),
    };

    Version::SemVer {
        major,
        minor,
        patch,
        prerelease: prerelease_lst,
        meta: meta_lst,
    }
}

// ============================================================================
// compare
// ============================================================================

/// Compare two versions.
///
/// -1 if v1 < v2, 0 if equal, 1 if v1 > v2.
/// Non-semver < semver. When comparing semver, prerelease and build
/// metadata are compared if their respective flags are true.
pub fn compare(
    v1: &Version,
    v2: &Version,
    compare_prerelease: bool,
    compare_build_information: bool,
) -> i32 {
    match (v1, v2) {
        (
            Version::NonSemVer { version: v1_str },
            Version::NonSemVer { version: v2_str },
        ) => string_compare(v1_str, v2_str),

        (Version::NonSemVer { .. }, Version::SemVer { .. }) => -1,

        (Version::SemVer { .. }, Version::NonSemVer { .. }) => 1,

        (
            Version::SemVer {
                major: m1,
                minor: n1,
                patch: p1,
                prerelease: pre1,
                meta: meta1,
            },
            Version::SemVer {
                major: m2,
                minor: n2,
                patch: p2,
                prerelease: pre2,
                meta: meta2,
            },
        ) => {
            let mut c: i32 = 0;
            // Special case: if either version is 0.0.0, they are equal
            if (*m1 == 0 && *n1 == 0 && *p1 == 0)
                || (*m2 == 0 && *n2 == 0 && *p2 == 0)
            {
                c = 0;
            } else {
                c = int_compare(*m1, *m2);
                if c != 0 {
                    return c;
                }
                c = int_compare(*n1, *n2);
                if c != 0 {
                    return c;
                }
                c = int_compare(*p1, *p2);
                if c != 0 {
                    return c;
                }
            }

            if compare_prerelease {
                c = compare_identifier_list(pre1, pre2);
            }
            if c == 0 && compare_build_information {
                c = compare_identifier_list(meta1, meta2);
            }
            c
        }
    }
}

// ============================================================================
// toString
// ============================================================================

/// Convert a Version back to its string representation.
pub fn to_string(v: &Version) -> String {
    match v {
        Version::SemVer {
            major,
            minor,
            patch,
            prerelease,
            meta,
        } => {
            let mut out = format!("{}.{}.{}", major, minor, patch);
            if !prerelease.is_empty() {
                out.push_str("-");
                out.push_str(&string_delimit_list(prerelease, "."));
            }
            if !meta.is_empty() {
                out.push_str("+");
                out.push_str(&string_delimit_list(meta, "."));
            }
            out
        }
        Version::NonSemVer { version } => version.clone(),
    }
}

// ============================================================================
// isPrerelease
// ============================================================================

/// Return true if the version has prerelease information.
pub fn is_prerelease(v: &Version) -> bool {
    match v {
        Version::SemVer { prerelease, .. } => !prerelease.is_empty(),
        Version::NonSemVer { .. } => false,
    }
}

// ============================================================================
// hasMetaInformation
// ============================================================================

/// Return true if the version has build metadata.
pub fn has_meta_information(v: &Version) -> bool {
    match v {
        Version::SemVer { meta, .. } => !meta.is_empty(),
        Version::NonSemVer { .. } => false,
    }
}

// ============================================================================
// isSemVer
// ============================================================================

/// Return true if the version is of semantic versioning type.
pub fn is_semver(v: &Version) -> bool {
    matches!(v, Version::SemVer { .. })
}

// ============================================================================
// Protected: splitPrereleaseAndMeta
// ============================================================================

/// Split a string into prerelease and meta identifier lists.
///
/// Handles strings like "alpha.1+build.2", "+build.2", "alpha.1", etc.
fn split_prerelease_and_meta(s: &str) -> (Vector<String>, Vector<String>) {
    let mut prerelease_lst: Vector<String> = Vector::new();
    let mut meta_lst: Vector<String> = Vector::new();

    if s.is_empty() {
        return (prerelease_lst, meta_lst);
    }

    // Check if starts with "+"
    if s.starts_with('+') {
        let rest = &s[1..];
        if !rest.is_empty() {
            meta_lst = string_split_at_char(rest, ".");
        }
        return (prerelease_lst, meta_lst);
    }

    let split: Vector<String> = string_split_at_char(s, "+");
    let prerelease: String = split.get(0).cloned().unwrap_or_default();
    let meta: String = if split.len() > 1 {
        split.get(1).cloned().unwrap_or_default()
    } else {
        String::new()
    };

    // If prerelease starts with "-", skip it
    let prerelease = if prerelease.starts_with('-') {
        prerelease[1..].to_string()
    } else {
        prerelease
    };

    prerelease_lst = if !prerelease.is_empty() {
        string_split_at_char(&prerelease, ".")
    } else {
        Vector::new()
    };
    meta_lst = if !meta.is_empty() {
        string_split_at_char(&meta, ".")
    } else {
        Vector::new()
    };

    (prerelease_lst, meta_lst)
}

// ============================================================================
// Protected: compareIdentifierList
// ============================================================================

/// Compare two identifier lists for semver prerelease/metadata ordering.
/// Follows the original MM semantics: empty list < non-empty list.
fn compare_identifier_list(w1: &Vector<String>, w2: &Vector<String>) -> i32 {
    let len1 = w1.len();
    let len2 = w2.len();

    // Per MM semantics: empty list is "less" than non-empty list
    if len1 == 0 && len2 > 0 {
        return -1;
    }
    if len2 == 0 && len1 > 0 {
        return 1;
    }

    let max_len = len1.max(len2);
    for i in 0..max_len {
        let s1 = w1.get(i).map(|s| s.as_str());
        let s2 = w2.get(i).map(|s| s.as_str());

        match (s1, s2) {
            (None, Some(_)) => {
                return -1;
            }
            (Some(_), None) => {
                return 1;
            }
            (Some(s1_val), Some(s2_val)) => {
                let c = compare_identifier(s1_val, s2_val);
                if c != 0 {
                    return c;
                }
            }
            (None, None) => {
                break;
            }
        }
    }

    0
}

// ============================================================================
// Protected: compareIdentifier
// ============================================================================

/// Compare two identifiers for semver prerelease/metadata ordering.
fn compare_identifier(s1: &str, s2: &str) -> i32 {
    let s1_is_int = is_integer_string(s1);
    let s2_is_int = is_integer_string(s2);

    if s1_is_int {
        if s2_is_int {
            return int_compare(string_int(s1), string_int(s2));
        }
        return -1; // numeric < non-numeric in semver
    }
    if s2_is_int {
        return 1; // non-numeric > numeric in semver
    }
    string_compare(s1, s2)
}

// ============================================================================
// Helper functions
// ============================================================================

/// Split a string by a delimiter (equivalent to Util.stringSplitAtChar).
fn string_split_at_char(string: &str, token: &str) -> Vector<String> {
    string.split(token).map(|s| s.to_string()).collect()
}

/// Convert a string to an integer.
fn string_int(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap_or(0)
}

/// Join a list of strings with a delimiter.
fn string_delimit_list(lst: &Vector<String>, delim: &str) -> String {
    lst.iter().cloned().collect::<Vec<_>>().join(delim)
}

/// Compare two strings lexicographically, returning -1, 0, or 1.
fn string_compare(s1: &str, s2: &str) -> i32 {
    if s1 < s2 {
        -1
    } else if s1 > s2 {
        1
    } else {
        0
    }
}

/// Compare two integers, returning -1, 0, or 1.
fn int_compare(n: i32, m: i32) -> i32 {
    if n < m {
        -1
    } else if n > m {
        1
    } else {
        0
    }
}

/// Check if a string represents an integer.
fn is_integer_string(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    let bytes = s.as_bytes();
    let start = if bytes[0] == b'-' || bytes[0] == b'+' { 1 } else { 0 };
    start < s.len() && bytes[start..].iter().all(|b| b.is_ascii_digit())
}

// ============================================================================
// Test
// ============================================================================

/// Tests basic semantic version operations.
pub fn test_semanticversion() -> Result<()> {
    // Test parse: valid semver
    let v = parse("1.2.3", false);
    assert!(is_semver(&v));
    assert!(is_prerelease(&parse("1.2.3-alpha", false)));
    assert!(!is_prerelease(&parse("1.2.3", false)));
    assert!(has_meta_information(&parse("1.2.3+build.1", false)));

    // Test parse: non-semver
    let v = parse("abc", false);
    match &v {
        Version::NonSemVer { version } => assert_eq!(version, "abc"),
        _ => panic!("Expected NonSemVer"),
    }

    // Test parse: empty string
    let v = parse("", false);
    match &v {
        Version::NonSemVer { version } => assert_eq!(version, ""),
        _ => panic!("Expected NonSemVer"),
    }

    // Test parse: nonsemverAsZeroZeroZero
    let v = parse("abc", true);
    assert!(is_semver(&v));

    // Test compare: semver equality
    assert_eq!(
        compare(&parse("1.2.3", false), &parse("1.2.3", false), true, false),
        0
    );

    // Test compare: semver ordering
    assert_eq!(
        compare(&parse("1.0.0", false), &parse("2.0.0", false), true, false),
        -1
    );
    assert_eq!(
        compare(&parse("2.0.0", false), &parse("1.0.0", false), true, false),
        1
    );

    // Test compare: non-semver < semver
    assert_eq!(
        compare(&parse("abc", false), &parse("1.0.0", false), true, false),
        -1
    );
    assert_eq!(
        compare(&parse("1.0.0", false), &parse("abc", false), true, false),
        1
    );

    // Test compare: non-semver vs non-semver
    assert_eq!(
        compare(&parse("a", false), &parse("b", false), true, false),
        -1
    );

    // Test compare: prerelease ordering (numeric < alpha)
    let a = parse("1.0.0-alpha", false);
    let b = parse("1.0.0-alpha.1", false);
    let c = parse("1.0.0-beta", false);
    assert_eq!(compare(&a, &b, true, false), -1);
    assert_eq!(compare(&b, &c, true, false), -1);

    // Test compare: prerelease vs release
    // MM semantics: empty prerelease < non-empty prerelease (opposite of semver spec)
    let release = parse("1.0.0", false);
    let pre = parse("1.0.0-alpha", false);
    assert_eq!(compare(&pre, &release, true, false), 1);
    assert_eq!(compare(&release, &pre, true, false), -1);

    // Test toString
    assert_eq!(to_string(&parse("1.2.3", false)), "1.2.3");
    assert_eq!(to_string(&parse("1.2.3-alpha.1", false)), "1.2.3-alpha.1");
    assert_eq!(
        to_string(&parse("1.2.3+build.1", false)),
        "1.2.3+build.1"
    );
    assert_eq!(
        to_string(&parse("1.2.3-alpha.1+build.1", false)),
        "1.2.3-alpha.1+build.1"
    );
    assert_eq!(to_string(&parse("abc", false)), "abc");

    // Test toString: 0.0.0
    let v = parse("0.0.0", false);
    assert_eq!(to_string(&v), "0.0.0");

    // Test compare: 0.0.0 special case (treated as equal)
    assert_eq!(
        compare(&parse("0.0.0", false), &parse("1.0.0", false), true, false),
        0
    );

    // Test splitPrereleaseAndMeta (via parse)
    let v = parse("1.0.0-alpha.beta+build", false);
    assert!(is_semver(&v));
    assert!(is_prerelease(&v));
    assert!(has_meta_information(&v));

    // Test metadata only
    let v = parse("1.0.0+metadata", false);
    assert!(is_semver(&v));
    assert!(!is_prerelease(&v));
    assert!(has_meta_information(&v));

    println!("testSemanticVersion succeeded\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_semver() {
        let v = parse("1.2.3", false);
        assert!(is_semver(&v));
    }

    #[test]
    fn test_parse_non_semver() {
        let v = parse("abc", false);
        assert!(!is_semver(&v));
    }

    #[test]
    fn test_parse_empty() {
        let v = parse("", false);
        assert!(!is_semver(&v));
    }

    #[test]
    fn test_compare_semver() {
        assert_eq!(
            compare(&parse("1.0.0", false), &parse("2.0.0", false), true, false),
            -1
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(to_string(&parse("1.2.3", false)), "1.2.3");
    }

    #[test]
    fn test_is_prerelease() {
        assert!(is_prerelease(&parse("1.0.0-alpha", false)));
        assert!(!is_prerelease(&parse("1.0.0", false)));
    }

    #[test]
    fn test_has_meta_information() {
        assert!(has_meta_information(&parse("1.0.0+build", false)));
        assert!(!has_meta_information(&parse("1.0.0", false)));
    }

    #[test]
    fn test_is_semver() {
        assert!(is_semver(&parse("1.0.0", false)));
        assert!(!is_semver(&parse("abc", false)));
    }
}
