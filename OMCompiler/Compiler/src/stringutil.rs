//! Translation of Util/StringUtil.mo
//!
//! This module provides string utility functions translated from MetaModelica,
//! including headline formatting, character search, word wrapping, and more.
//!
//! # Assumptions and Notes
//!
//! 1. **1-based vs 0-based indexing**: MetaModelica uses 1-based string indexing.
//!    All Rust equivalents use 0-based indexing.
//! 2. **UTF-8**: The MetaModelica `String` type handles Unicode. Rust `String` also
//!    uses UTF-8, so `chars()` is used for iteration. `stringLength` returns
//!    character count, not byte count.
//! 3. **System.strtok**: The MM `System.strtok(s, "\n")` splits a string on a
//!    delimiter and returns a list. The Rust equivalent uses `split('\n')`.
//! 4. **System.StringAllocator**: Used in `repeat` for efficient string building.
//!    The Rust version uses simple repetition with `str.repeat(n)`.
//! 5. **list<String>**: Maps to `im::Vector<String>`. The order is reversed when
//!    constructing via push_front and then reversed at the end via `list_reverse`.
//! 6. **MetaModelica.Dangerous functions**: `stringGetNoBoundsChecking` and
//!    `listReverseInPlace` are direct translations.
//! 7. **String(integer(x))**: `Integer` to `String` via `format!("{}", x)`.
//! 8. **String(Real, significantDigits=n)**: `Real` to `String` via
//!    format with precision.

use anyhow::{bail, Result};
use im::Vector;

// ============================================================================
// Constants
// ============================================================================

/// No position found sentinel value.
pub const NO_POS: i32 = 0;

/// ASCII code for newline character.
pub const CHAR_NEWLINE: i32 = 10;

/// ASCII code for space character.
pub const CHAR_SPACE: i32 = 32;

/// ASCII code for dash character.
pub const CHAR_DASH: i32 = 45;

/// ASCII code for dot character.
pub const CHAR_DOT: i32 = 46;

// ============================================================================
// Headline functions
// ============================================================================

/// Format a title as a headline with # borders.
pub fn headline_1(title: &str) -> String {
    let sep = "#".repeat(title.chars().count() + 8);
    format!("{}\n\n    {}\n\n{}\n", sep, title, sep)
}

/// Format a title as a headline with = borders.
pub fn headline_2(title: &str) -> String {
    let sep = "=".repeat(title.chars().count() + 4);
    format!("{}\n  {}\n{}\n", sep, title, sep)
}

/// Format a title as a headline with underline dashes.
pub fn headline_3(title: &str) -> String {
    format!("{}\n{}\n", title, "-".repeat(title.chars().count() + 2))
}

/// Format a title as a headline with underline asterisks.
pub fn headline_4(title: &str) -> String {
    format!("{}\n{}\n", title, "*".repeat(title.chars().count() + 2))
}

// ============================================================================
// Character search functions (1-based indexing, matching MM semantics)
// ============================================================================

/// Searches for a given character in the given string, returning the 1-based
/// index of the character if found. If not found returns NO_POS.
/// The start and end position determines the section of the string to search in.
///
/// `in_start_pos` is 1-based. `in_end_pos` is 1-based; 0 means scan to end.
pub fn find_char(
    in_string: &str,
    in_char: i32,
    in_start_pos: i32,
    in_end_pos: i32,
) -> i32 {
    let len = in_string.chars().count() as i32;
    let start_pos = in_start_pos.max(1);
    let end_pos = if in_end_pos > 0 {
        in_end_pos.min(len)
    } else {
        len
    };

    let chars: Vec<char> = in_string.chars().collect();
    for i in (start_pos - 1)..end_pos {
        if i < chars.len() as i32 && (chars[i as usize] as i32) == in_char {
            return i + 1; // 1-based index
        }
    }
    NO_POS
}

/// Searches backwards for a given character in the given string, returning the
/// 1-based index of the character if found. If not found returns NO_POS.
///
/// `in_start_pos` is 1-based; negative means scan from end.
/// `in_end_pos` is 1-based; negative means scan to start.
pub fn rfind_char(
    in_string: &str,
    in_char: i32,
    in_start_pos: i32,
    in_end_pos: i32,
) -> i32 {
    let len = in_string.chars().count() as i32;
    let start_pos = if in_start_pos > 0 {
        in_start_pos.min(len)
    } else {
        len
    };
    let end_pos = in_end_pos.max(1);

    let chars: Vec<char> = in_string.chars().collect();
    for i in (end_pos..=start_pos).rev() {
        if (chars[(i - 1) as usize] as i32) == in_char {
            return i; // 1-based index
        }
    }
    NO_POS
}

/// Searches for a character not matching the given character in the given
/// string, returning the 1-based index of the character if found. If not found
/// returns NO_POS.
pub fn find_char_not(
    in_string: &str,
    in_char: i32,
    in_start_pos: i32,
    in_end_pos: i32,
) -> i32 {
    let len = in_string.chars().count() as i32;
    let start_pos = in_start_pos.max(1);
    let end_pos = if in_end_pos > 0 {
        in_end_pos.min(len)
    } else {
        len
    };

    let chars: Vec<char> = in_string.chars().collect();
    for i in (start_pos - 1)..end_pos {
        if i < chars.len() as i32 && (chars[i as usize] as i32) != in_char {
            return i + 1; // 1-based index
        }
    }
    NO_POS
}

/// Searches backwards for a character not matching the given character in the
/// given string, returning the 1-based index of the character if found. If not found
/// returns NO_POS.
pub fn rfind_char_not(
    in_string: &str,
    in_char: i32,
    in_start_pos: i32,
    in_end_pos: i32,
) -> i32 {
    let len = in_string.chars().count() as i32;
    let start_pos = if in_start_pos > 0 {
        in_start_pos.min(len)
    } else {
        len
    };
    let end_pos = in_end_pos.max(1);

    let chars: Vec<char> = in_string.chars().collect();
    for i in (end_pos..=start_pos).rev() {
        if (chars[(i - 1) as usize] as i32) != in_char {
            return i; // 1-based index
        }
    }
    NO_POS
}

// ============================================================================
// Character classification
// ============================================================================

/// Returns true if the given character represented by its ASCII decimal number
/// is an alphabetic character.
pub fn is_alpha(in_char: i32) -> bool {
    (in_char >= 65 && in_char <= 90) || (in_char >= 97 && in_char <= 122)
}

// ============================================================================
// Word wrapping
// ============================================================================

/// Breaks the given string into lines which are no longer than the given wrap
/// length. The function tries to break lines at word boundaries, i.e. at spaces,
/// so that words are not split. It also wraps the string at any newline
/// characters it finds.
///
/// This function operates on ASCII strings, and does not handle UTF-8 strings
/// correctly.
pub fn word_wrap(
    in_string: &str,
    in_wrap_length: i32,
    in_delimiter: &str,
    in_raggedness: f64,
) -> Vector<String> {
    // Check that the wrap length is larger than the delimiter
    if (in_delimiter.chars().count() as i32) >= in_wrap_length - 1 {
        let mut result = Vector::new();
        result.push_back(in_string.to_string());
        return result;
    }

    // Split the string at newlines (equivalent to System.strtok(s, "\n"))
    let lines: Vec<String> = in_string
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    // Calculate the length of each line, excluding the delimiter
    let line_len = in_wrap_length - (in_delimiter.chars().count() as i32) - 1;

    // The gap size is how many characters a line may be shorter than the sought
    // after line length
    let gap_size = (line_len as f64 * in_raggedness).floor() as i32;
    let gap_size = if gap_size < 0 { 0 } else { gap_size };

    let mut out_strings: Vector<String> = Vector::new();

    for line_idx in 0..lines.len() {
        let line = &lines[line_idx];
        let line_chars: Vec<char> = line.chars().collect();
        let line_len_chars = line_chars.len() as i32;

        let mut start_pos: i32 = 1; // 1-based, like MM
        let mut end_pos: i32 = in_wrap_length;
        let mut delim = "";

        while end_pos < line_len_chars {
            // Get next character at position end_pos (1-based MM => 0-based Rust = end_pos)
            let next_char = if (end_pos as usize) < line_chars.len() {
                line_chars[end_pos as usize] as i32
            } else {
                -1
            };

            if next_char != CHAR_SPACE && next_char != CHAR_DASH {
                // Search backwards for a space
                let pos = find_char_internal(line, CHAR_SPACE, end_pos, (end_pos - gap_size).max(1));

                if pos != NO_POS {
                    // A space was found, break the string here
                    let str_part = get_substring(line, start_pos, pos - 1);
                    start_pos = pos + 1;
                    let entry = format!("{}{}", delim, str_part);
                    out_strings.push_front(entry);
                    delim = in_delimiter;
                } else {
                    // No space was found, search for a dash instead
                    let pos = find_char_internal(line, CHAR_DASH, end_pos, start_pos + gap_size);

                    if pos > 1 {
                        // A dash was found, check that the previous character is alphabetic
                        let prev_char_idx = ((pos - 1) - 1) as usize; // 1-based MM pos => 0-based char index
                        let prev_char = if prev_char_idx < line_chars.len() {
                            line_chars[prev_char_idx] as i32
                        } else {
                            -1
                        };
                        let pos = if is_alpha(prev_char) && is_alpha(next_char) {
                            pos
                        } else {
                            NO_POS
                        };

                        if pos != NO_POS {
                            let str_part = get_substring(line, start_pos, pos);
                            start_pos = pos + 1;
                            let entry = format!("{}{}", delim, str_part);
                            out_strings.push_front(entry);
                            delim = in_delimiter;
                        } else {
                            // No dash was found, break the word and hyphenate it
                            let str_part = format!("{}-", get_substring(line, start_pos, end_pos - 1));
                            start_pos = end_pos;
                            let entry = format!("{}{}", delim, str_part);
                            out_strings.push_front(entry);
                            delim = in_delimiter;
                        }
                    } else {
                        // No dash was found, break the word and hyphenate it
                        let str_part = format!("{}-", get_substring(line, start_pos, end_pos - 1));
                        start_pos = end_pos;
                        let entry = format!("{}{}", delim, str_part);
                        out_strings.push_front(entry);
                        delim = in_delimiter;
                    }
                }
            } else {
                // The next character is a space or dash, split the string here
                let str_part = get_substring(line, start_pos, end_pos);
                let skip = if next_char == CHAR_SPACE { 2 } else { 1 };
                start_pos = end_pos + skip;
                let entry = format!("{}{}", delim, str_part);
                out_strings.push_front(entry);
                delim = in_delimiter;
            }

            end_pos = start_pos + line_len;
        }

        // Add any remainder of the line to the list
        if start_pos <= line_len_chars && start_pos > 0 {
            let str_part = format!("{}{}", delim, get_substring(line, start_pos, line_len_chars));
            out_strings.push_front(str_part);
        }

    }

    // Reverse the list at the end (equivalent to listReverseInPlace)
    list_reverse(out_strings)
}

/// Internal character find using 1-based indexing (matching MM semantics).
/// Searches for target char between start_pos and end_pos (both 1-based).
fn find_char_internal(s: &str, target: i32, start_pos: i32, end_pos: i32) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    for i in ((start_pos - 1)..start_pos + (end_pos - start_pos)).take((end_pos - start_pos + 1) as usize) {
        if i < chars.len() as i32 && (chars[i as usize] as i32) == target {
            return i + 1;
        }
    }
    NO_POS
}

/// Internal substring extraction using 1-based indexing (matching MM semantics).
fn get_substring(s: &str, start: i32, end: i32) -> String {
    if start < 1 || end < start || s.is_empty() {
        return String::new();
    }
    let start_idx = (start - 1) as usize;
    let end_idx = end as usize;
    let chars: Vec<char> = s.chars().collect();
    if start_idx >= chars.len() {
        return String::new();
    }
    let actual_end = end_idx.min(chars.len());
    chars[start_idx..actual_end].iter().collect()
}

/// Reverses a vector (equivalent to listReverseInPlace).
fn list_reverse<T: Clone>(mut v: Vector<T>) -> Vector<T> {
    let mut result = Vector::new();
    while let Some(item) = v.pop_front() {
        result.push_back(item);
    }
    result
}

// ============================================================================
// Repeat
// ============================================================================

/// Repeat str n times.
pub fn repeat(s: &str, n: i32) -> String {
    if n <= 0 {
        return String::new();
    }
    s.repeat(n as usize)
}

// Deprecated: use `str.repeat(n)` instead.
/// Repeat str n times.
///
/// DEPRECATED: Use Rust's built-in `str.repeat(n)` which is more idiomatic.
pub fn repeat_str(s: &str, n: i32) -> String {
    repeat(s, n)
}

// ============================================================================
// Quote
// ============================================================================

/// Adds quotation marks to the beginning and end of a string.
pub fn quote(s: &str) -> String {
    format!("\"{}\"", s)
}

// ============================================================================
// Equal ignore space
// ============================================================================

/// Compares two strings, ignoring spaces.
pub fn equal_ignore_space(s1: &str, s2: &str) -> bool {
    let mut j = 0;
    let s2_chars: Vec<char> = s2.chars().collect();
    let s2_len = s2_chars.len();

    for ch in s1.chars() {
        if ch != ' ' {
            let mut found = false;
            while j < s2_len {
                if s2_chars[j] != ' ' {
                    found = true;
                    j += 1;
                    break;
                }
                j += 1;
            }
            if !found {
                return false;
            }
        }
    }

    while j < s2_len {
        if s2_chars[j] != ' ' {
            return false;
        }
        j += 1;
    }

    true
}

// ============================================================================
// Bytes to readable unit
// ============================================================================

/// Converts a byte count to a human-readable string (kB, MB, GB, TB).
pub fn bytes_to_readable_unit(bytes: f64, significant_digits: i32, max_size_in_unit: f64) -> String {
    let tb = 1024_f64.powi(4);
    let gb = 1024_f64.powi(3);
    let mb = 1024_f64.powi(2);
    let kb = 1024_f64;

    let format_with_precision = |val: f64, digits: i32| -> String {
        let precision: usize = if digits > 0 { (digits - 1) as usize } else { 0 };
        let raw = format!("{:.precision$}", val);
        raw.trim_end_matches('0').trim_end_matches('.').to_string()
    };

    if bytes > max_size_in_unit * gb {
        format!("{} TB", format_with_precision(bytes / tb, significant_digits))
    } else if bytes > max_size_in_unit * mb {
        format!("{} GB", format_with_precision(bytes / gb, significant_digits))
    } else if bytes > max_size_in_unit * kb {
        format!("{} MB", format_with_precision(bytes / mb, significant_digits))
    } else if bytes > max_size_in_unit {
        format!("{} kB", format_with_precision(bytes / kb, significant_digits))
    } else {
        format!("{} bytes", bytes as i64)
    }
}

// ============================================================================
// StartsWith
// ============================================================================

/// Returns true if the string starts with the given prefix.
pub fn starts_with(str: &str, prefix: &str) -> bool {
    str.starts_with(prefix)
}

// Deprecated: use `str.starts_with()` instead.
/// Returns true if the string starts with the given prefix.
///
/// DEPRECATED: Use Rust's built-in `str.starts_with()`.
pub fn starts_with_str(str: &str, prefix: &str) -> bool {
    starts_with(str, prefix)
}

// ============================================================================
// Ends with
// ============================================================================

/// Returns true if the string ends with the given suffix.
pub fn ends_with(str: &str, suffix: &str) -> bool {
    str.ends_with(suffix)
}

// Deprecated: use `str.ends_with()` instead.
/// Returns true if the string ends with the given suffix.
///
/// DEPRECATED: Use Rust's built-in `str.ends_with()`.
pub fn ends_with_str(str: &str, suffix: &str) -> bool {
    ends_with(str, suffix)
}

// ============================================================================
// Ends with newline
// ============================================================================

/// Returns true if the string ends with a newline character.
pub fn ends_with_newline(s: &str) -> bool {
    s.chars().next_back() == Some('\n')
}

// Deprecated: use `s.ends_with('\n')` instead.
/// Returns true if the string ends with a newline character.
///
/// DEPRECATED: Use `s.ends_with('\n')`.
pub fn ends_with_newline_str(s: &str) -> bool {
    ends_with_newline(s)
}

// ============================================================================
// Convert non-ASCII character to hex
// ============================================================================

/// Converts a single character string to a hex representation if it is not valid ASCII.
pub fn convert_char_non_ascii_to_hex(s: &mut String) {
    let hex_table = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"];

    if s.chars().count() != 1 {
        return;
    }

    let ch = s.chars().next().unwrap();
    let i = ch as u32;

    if i < 128 {
        return;
    }

    let hex_str = format!(
        "0x{}{}",
        hex_table[(i / 16) as usize],
        hex_table[(i % 16) as usize]
    );
    *s = hex_str;
}

// ============================================================================
// Strip BOM
// ============================================================================

/// Strips a UTF-8 BOM (Byte Order Mark) from the beginning of a string.
/// Returns the BOM that was stripped (or empty string if no BOM found).
pub fn strip_bom(s: &mut String) -> String {
    if s.len() < 3 {
        return String::new();
    }

    // UTF-8 BOM is 0xEF 0xBB 0xBF
    let bytes = s.as_bytes();
    if bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF {
        let bom = "\u{FEFF}".to_string();
        *s = String::from_utf8_lossy(&bytes[3..]).into_owned();
        return bom;
    }

    String::new()
}

// ============================================================================
// Strip file extension
// ============================================================================

/// Strips the file extension from a filename (by the last dot).
pub fn strip_file_extension(filename: &mut String) {
    if let Some(pos) = filename.rfind('.') {
        *filename = String::from(&filename[..pos]);
    }
}

// ============================================================================
// Rest
// ============================================================================

/// Returns all but the first character of a string.
pub fn rest(s: &str) -> String {
    s.chars().skip(1).collect()
}

// ============================================================================
// matchcontinue helpers
// ============================================================================

/// Helper for matchcontinue: succeeds if the condition is true.
fn match_true(cond: bool) -> Result<()> {
    if cond {
        return Ok(());
    }
    bail!("matchcontinue condition failed")
}

// ============================================================================
// Test
// ============================================================================

/// Tests basic string utility operations.
pub fn test_stringutil() -> Result<()> {
    // Test constants
    assert_eq!(NO_POS, 0);
    assert_eq!(CHAR_NEWLINE, 10);
    assert_eq!(CHAR_SPACE, 32);

    // Test headline functions
    let h1 = headline_1("Test");
    assert!(h1.contains("Test"));
    assert!(h1.starts_with('#'));

    let h2 = headline_2("Test");
    assert!(h2.contains("Test"));
    assert!(h2.starts_with('='));

    let h3 = headline_3("Test");
    assert!(h3.contains("Test"));

    let h4 = headline_4("Test");
    assert!(h4.contains("Test"));

    // Test find_char (1-based indexing)
    assert_eq!(find_char("hello world", b'w' as i32, 1, 0), 7);
    assert_eq!(find_char("hello world", b'z' as i32, 1, 0), NO_POS);

    // Test rfind_char (1-based indexing)
    // 'o' is at positions 5 and 8 in "hello world", searching backwards from 11 should find position 8
    assert_eq!(rfind_char("hello world", b'o' as i32, 0, 1), 8);

    // Test find_char_not
    assert_eq!(find_char_not("aaaa", b'b' as i32, 1, 0), 1);
    assert_eq!(find_char_not("    ", b' ' as i32, 1, 0), NO_POS);

    // Test isAlpha
    assert!(is_alpha(65)); // 'A'
    assert!(is_alpha(97)); // 'a'
    assert!(!is_alpha(48)); // '0'

    // Test repeat
    assert_eq!(repeat("ab", 3), "ababab");
    assert_eq!(repeat("ab", 0), "");

    // Test quote
    assert_eq!(quote("hello"), "\"hello\"");

    // Test starts_with
    assert!(starts_with("hello world", "hello"));
    assert!(!starts_with("hello world", "world"));

    // Test ends_with
    assert!(ends_with("hello world", "world"));
    assert!(!ends_with("hello world", "hello"));

    // Test ends_with_newline
    assert!(ends_with_newline("hello\n"));
    assert!(!ends_with_newline("hello"));

    // Test rest
    assert_eq!(rest("hello"), "ello");
    assert_eq!(rest("a"), "");
    assert_eq!(rest(""), "");

    // Test equal_ignore_space
    assert!(equal_ignore_space("hello world", "hello world"));
    assert!(equal_ignore_space("hello world", "hello  world"));
    assert!(!equal_ignore_space("hello world", "hello"));

    // Test convert_char_non_ascii_to_hex
    let mut s = "a".to_string();
    convert_char_non_ascii_to_hex(&mut s);
    assert_eq!(s, "a"); // ASCII stays the same

    let mut s2 = "\u{00E9}".to_string(); // é (233 = 0xE9)
    convert_char_non_ascii_to_hex(&mut s2);
    assert_eq!(s2, "0xE9");

    // Test strip_bom
    let mut s3 = "\u{FEFF}hello".to_string();
    let bom = strip_bom(&mut s3);
    assert_eq!(bom, "\u{FEFF}");
    assert_eq!(s3, "hello");

    // Test strip_file_extension
    let mut fname = "test.txt".to_string();
    strip_file_extension(&mut fname);
    assert_eq!(fname, "test");

    // Test rfind_char_not
    assert_eq!(rfind_char_not("aaaa", b'b' as i32, 0, 1), 4);
    assert_eq!(rfind_char_not("    ", b' ' as i32, 0, 1), NO_POS);

    // Test repeat_str (deprecated, same as repeat)
    assert_eq!(repeat_str("ab", 3), "ababab");

    // Test bytes_to_readable_unit
    assert!(bytes_to_readable_unit(100.0, 4, 500.0).contains("bytes"));
    assert!(bytes_to_readable_unit(1500.0, 4, 500.0).contains("kB"));
    // 1MB = 1048576 bytes, 500*kB = 512000, so 1MB > 500*kB => shows as MB
    assert!(bytes_to_readable_unit(1_048_576.0, 4, 500.0).contains("MB"));

    // Test starts_with_str (deprecated, same as starts_with)
    assert!(starts_with_str("hello world", "hello"));

    // Test ends_with_str (deprecated, same as ends_with)
    assert!(ends_with_str("hello world", "world"));

    // Test ends_with_newline_str (deprecated, same as ends_with_newline)
    assert!(ends_with_newline_str("hello\n"));

    // Test word_wrap
    let lines = word_wrap("hello world foo bar", 10, "", 0.3);
    assert!(!lines.is_empty());

    println!("testStringUtil succeeded\n");
    Ok(())
}
