//! Translation of FrontEnd/MMath.mo
//!
//! This module provides rational number arithmetic in Rust.
//! It is a direct translation of the MMath package from MetaModelica,
//! supporting addition, subtraction, multiplication, division, GCD,
//! and comparison of rational numbers.

use anyhow::{bail, Result};
use std::fmt;

// ============================================================================
// Rational uniontype
// ============================================================================

/// Represents a rational number, e.g. 6/7.
#[derive(Debug, Clone, PartialEq, Copy)]
#[allow(non_camel_case_types)]
pub enum Rational {
    RATIONAL { nom: i64, denom: i64 },
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rational::RATIONAL { nom, denom } => write!(f, "{}/{}", nom, denom),
        }
    }
}

// ============================================================================
// Constants
// ============================================================================

/// Rational number 0/1
pub const RAT0: Rational = Rational::RATIONAL {
    nom: 0,
    denom: 1,
};

/// Rational number 1/1
pub const RAT1: Rational = Rational::RATIONAL {
    nom: 1,
    denom: 1,
};

// ============================================================================
// Helper: normalize zero (protected)
// ============================================================================

/// If numerator is zero, set denominator to 1.
fn normalize_zero(r: Rational) -> Rational {
    match r {
        Rational::RATIONAL { nom: 0, .. } => Rational::RATIONAL { nom: 0, denom: 1 },
        other => other,
    }
}

// ============================================================================
// Functions
// ============================================================================

/// Comparison: returns true if r1 > r2 (converted to f64 for comparison).
pub fn is_greater_than(r1: Rational, r2: Rational) -> bool {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: n1, denom: d1 },
            Rational::RATIONAL { nom: n2, denom: d2 },
        ) => (n1 as f64 / d1 as f64) > (n2 as f64 / d2 as f64),
    }
}

/// Adds two rational numbers: r1 + r2, result simplified by GCD.
pub fn add_rational(r1: Rational, r2: Rational) -> Rational {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: i1, denom: i2 },
            Rational::RATIONAL { nom: i3, denom: i4 },
        ) => {
            let ri1 = i1 * i4 + i3 * i2;
            let ri2 = i2 * i4;
            let d = int_gcd(ri1, ri2);
            let ri1 = ri1 / d;
            let ri2 = ri2 / d;
            normalize_zero(Rational::RATIONAL { nom: ri1, denom: ri2 })
        }
    }
}

/// Converts a rational to a string "n/d".
pub fn rational_string(r: Rational) -> String {
    match r {
        Rational::RATIONAL { nom: n, denom: d } => format!("{}/{}", n, d),
    }
}

/// Equality: returns true if r1 == r2 (using cross-multiplication).
pub fn equals(r1: Rational, r2: Rational) -> bool {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: i1, denom: i2 },
            Rational::RATIONAL { nom: i3, denom: i4 },
        ) => i1 * i4 - i3 * i2 == 0,
    }
}

/// Subtracts two rational numbers: r1 - r2, result simplified by GCD.
pub fn sub_rational(r1: Rational, r2: Rational) -> Rational {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: i1, denom: i2 },
            Rational::RATIONAL { nom: i3, denom: i4 },
        ) => {
            let ri1 = i1 * i4 - i3 * i2;
            let ri2 = i2 * i4;
            let d = int_gcd(ri1, ri2);
            let ri1 = ri1 / d;
            let ri2 = ri2 / d;
            normalize_zero(Rational::RATIONAL { nom: ri1, denom: ri2 })
        }
    }
}

/// Multiplies two rational numbers: r1 * r2, result simplified by GCD.
pub fn mult_rational(r1: Rational, r2: Rational) -> Rational {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: i1, denom: i2 },
            Rational::RATIONAL { nom: i3, denom: i4 },
        ) => {
            let ri1 = i1 * i3;
            let ri2 = i2 * i4;
            let d = int_gcd(ri1, ri2);
            let ri1 = ri1 / d;
            let ri2 = ri2 / d;
            normalize_zero(Rational::RATIONAL { nom: ri1, denom: ri2 })
        }
    }
}

/// Division of two rationals: r1 / r2 = (n1/d1) / (n2/d2) = (n1*d2) / (n2*d1), result simplified by GCD.
pub fn div_rational(r1: Rational, r2: Rational) -> Rational {
    match (r1, r2) {
        (
            Rational::RATIONAL { nom: i1, denom: i2 },
            Rational::RATIONAL { nom: i3, denom: i4 },
        ) => {
            let ri1 = i1 * i4;
            let ri2 = i3 * i2;
            let d = int_gcd(ri1, ri2);
            let ri1 = ri1 / d;
            let ri2 = ri2 / d;
            normalize_zero(Rational::RATIONAL { nom: ri1, denom: ri2 })
        }
    }
}

/// Returns the greatest common divisor of two integers using Euclidean algorithm.
pub fn int_gcd(i1: i64, i2: i64) -> i64 {
    match (i1, i2) {
        (_, 0) => i1,
        _ => {
            let rem = i1.rem_euclid(i2);
            int_gcd(i2, rem)
        }
    }
}

// ============================================================================
// matchcontinue helper functions
// ============================================================================

/// Helper for matchcontinue: succeeds if the boolean is true.
fn match_true(b: bool) -> Result<()> {
    if b {
        return Ok(());
    }
    bail!("assertion failed")
}

// ============================================================================
// Test
// ============================================================================

/// Tests the rational arithmetic operations using matchcontinue pattern.
/// Each assertion is checked; if all pass, prints success. If any fails, prints failure.
pub fn test_rational() -> Result<()> {
    let result = do_test();
    match result {
        Ok(()) => {
            println!("testRational succeeded\n");
            Ok(())
        }
        Err(_) => {
            println!("testRationals failed\n");
            Ok(())
        }
    }
}

fn do_test() -> Result<()> {
    // matchcontinue: case() checks all these equations; if any fails, falls to else
    match_true(equals(RATIONAL(7, 6), add_rational(RATIONAL(1, 2), RATIONAL(2, 3))))?;
    match_true(equals(RATIONAL(2, 1), add_rational(RATIONAL(1, 2), RATIONAL(3, 2))))?;

    match_true(equals(RATIONAL(1, 1), sub_rational(RATIONAL(3, 2), RATIONAL(1, 2))))?;
    match_true(equals(RATIONAL(1, 3), sub_rational(RATIONAL(1, 2), RATIONAL(1, 6))))?;

    match_true(equals(RATIONAL(4, 3), mult_rational(RATIONAL(2, 3), RATIONAL(4, 2))))?;
    match_true(equals(RATIONAL(1, 1), mult_rational(RATIONAL(1, 1), RATIONAL(1, 1))))?;

    match_true(equals(RATIONAL(1, 2), div_rational(RATIONAL(1, 3), RATIONAL(2, 3))))?;
    Ok(())
}

/// Helper for testRational: constructs a Rational from literal arguments.
fn RATIONAL(nom: i64, denom: i64) -> Rational {
    Rational::RATIONAL { nom, denom }
}
