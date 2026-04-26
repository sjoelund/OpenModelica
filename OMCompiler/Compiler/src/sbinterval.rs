//! Translation of Util/SBInterval.mo
//!
//! Interval type for set-based graphs. Provides interval arithmetic including
//! construction, intersection, complement, affine transformation, and membership testing.

use anyhow::Result;
use im::Vector;

type List<T> = Vector<T>;

// ============================================================================
// Type definitions
// ============================================================================

/// INTERVAL record: defines a discrete interval [lo, hi] with given step.
/// lo = lower bound, step = stride, hi = upper bound (inclusive, aligned to step).
#[derive(Clone, Debug, PartialEq)]
pub struct INTERVAL {
    pub lo: i32,
    pub step: i32,
    pub hi: i32,
}

/// Empty interval sentinel (step == 0 indicates empty).
fn empty_interval() -> INTERVAL {
    INTERVAL {
        lo: -1,
        step: 0,
        hi: -1,
    }
}

/// Full interval covering [1, i32::MAX] with step 1.
fn full_interval() -> INTERVAL {
    INTERVAL {
        lo: 1,
        step: 1,
        hi: i32::MAX,
    }
}

/// Single-element interval [1, 1] with step 1.
fn unit_interval() -> INTERVAL {
    INTERVAL {
        lo: 1,
        step: 1,
        hi: 1,
    }
}

// ============================================================================
// euclid (protected)
// ============================================================================

/// Extended Euclidean algorithm.
/// Returns (d, m, ua, vb) where:
///   d  = gcd(a, b)
///   m  = lcm(a, b) = a * (b / d)
///   ua, vb = Bezout coefficients: ua * a + vb * b = d
fn euclid(a: i32, b: i32) -> (i32, i32, i32, i32) {
    let (mut r1, mut r2) = (a, b);
    let (mut s1, mut s2) = (a, 0);

    while r2 != 0 {
        let q = r1 / r2;

        let tmp = r2;
        r2 = r1 - q * r2;
        r1 = tmp;

        let tmp = s2;
        s2 = s1 - q * s2;
        s1 = tmp;
    }

    let d = r1;
    let m = s2.abs();
    let ua = s1;
    let vb = d - s1;
    (d, m, ua, vb)
}

// ============================================================================
// Constructors
// ============================================================================

/// Create a new interval from lo, step, hi.
/// Handles edge cases: non-negative lo, positive step, intMax overflow.
pub fn new(lo: i32, step: i32, hi: i32) -> INTERVAL {
    if lo >= 0 && step > 0 && hi >= 0 {
        if lo <= hi && hi < i32::MAX {
            let adjusted_hi = hi - (hi - lo).rem_euclid(step);
            INTERVAL { lo, step, hi: adjusted_hi }
        } else if lo <= hi && hi == i32::MAX {
            INTERVAL { lo, step, hi: i32::MAX }
        } else {
            // Wrong values for subscript (check low <= hi).
            INTERVAL { lo, step: 0, hi }
        }
    } else if lo >= 0 && step == 0 && hi == lo {
        INTERVAL { lo, step: 1, hi }
    } else {
        new_empty()
    }
}

/// Create an empty interval.
pub fn new_empty() -> INTERVAL {
    empty_interval()
}

/// Create a unit interval [1, 1] with step 1.
pub fn new_unit() -> INTERVAL {
    unit_interval()
}

/// Create a full interval [1, i32::MAX] with step 1.
pub fn new_full() -> INTERVAL {
    full_interval()
}

// ============================================================================
// Accessors
// ============================================================================

/// Get the lower bound of the interval.
pub fn lower_bound(int: &INTERVAL) -> i32 {
    int.lo
}

/// Get the step value of the interval.
pub fn step_value(int: &INTERVAL) -> i32 {
    int.step
}

/// Get the upper bound of the interval.
pub fn upper_bound(int: &INTERVAL) -> i32 {
    int.hi
}

// ============================================================================
// crop
// ============================================================================

/// Crop the interval's upper bound to align with the step.
/// Only adjusts if hi < i32::MAX.
pub fn crop(int: &INTERVAL) -> INTERVAL {
    let mut result = int.clone();
    if result.hi < i32::MAX {
        result.hi = result.hi - (result.hi - result.lo).rem_euclid(result.step);
    }
    result
}

// ============================================================================
// intersection
// ============================================================================

/// Compute the intersection of two intervals.
/// Returns an empty interval if they do not intersect or step through each other.
pub fn intersection(int1: &INTERVAL, int2: &INTERVAL) -> Result<INTERVAL> {
    if int1.hi < int2.lo || int2.hi < int1.lo {
        return Ok(new_empty());
    }

    let (gcd_, new_step, ua, vb) = euclid(int1.step, int2.step);

    if (int1.lo - int2.lo) % gcd_ != 0 {
        // The intervals step through each other without touching.
        return Ok(new_empty());
    }

    // x is an integer on both intervals (modulo new_step).
    let x = int1.lo / gcd_ * vb + int2.lo / gcd_ * ua + (int1.lo % gcd_);

    let new_lo = int1.lo.max(int2.lo);
    let new_hi = int1.hi.min(int2.hi);
    let new_lo = new_lo + (x - new_lo).rem_euclid(new_step);
    let new_hi = if new_hi < i32::MAX {
        new_hi - (new_hi - x).rem_euclid(new_step)
    } else {
        new_hi
    };

    if new_hi < new_lo {
        Ok(new_empty())
    } else {
        Ok(new(new_lo, new_step, new_hi))
    }
}

// ============================================================================
// complement
// ============================================================================

/// Returns a list of intervals corresponding to the removal of int2 from int1.
/// Note: MetaModelica returns UnorderedSet<SBInterval>. Rust uses List<INTERVAL>
/// since UnorderedSet has no Rust translation yet.
pub fn complement(int1: &INTERVAL, int2: &INTERVAL) -> Result<List<INTERVAL>> {
    let mut ints: List<INTERVAL> = List::new();

    let i2 = intersection(int1, int2)?;

    if is_empty(&i2) {
        // No intersection, nothing to remove.
        ints.push_front(int1.clone());
    } else if !int_equals(int1, &i2) {
        // Rightmost interval.
        if i2.hi < int1.hi {
            let add = new(i2.hi + int1.step, int1.step, int1.hi);
            ints.push_front(add);
        }

        let count_r = i2.step / int1.step - 1;
        let count_s = if i2.hi < i32::MAX {
            (i2.hi - i2.lo) / i2.step
        } else {
            i32::MAX
        };

        if count_r < count_s {
            // Create an interval for every residue class not equal to i2.lo.
            if count_s < i32::MAX {
                let mut i = count_r;
                while i >= 1 {
                    let add = INTERVAL {
                        lo: i2.lo + i * int1.step,
                        step: i2.step,
                        hi: i2.hi - i2.step + i * int1.step,
                    };
                    ints.push_front(add);
                    i -= 1;
                }
            } else {
                let mut i = count_r;
                while i >= 1 {
                    let add = INTERVAL {
                        lo: i2.lo + i * int1.step,
                        step: i2.step,
                        hi: i32::MAX,
                    };
                    ints.push_front(add);
                    i -= 1;
                }
            }
        } else {
            // Create an interval for every space between removed points.
            let mut i = count_s;
            while i >= 1 {
                let add = INTERVAL {
                    lo: i2.lo + int1.step + (i - 1) * i2.step,
                    step: int1.step,
                    hi: i2.lo - int1.step + i * i2.step,
                };
                ints.push_front(add);
                i -= 1;
            }
        }

        // Leftmost interval.
        if i2.lo > int1.lo {
            let add = new(int1.lo, int1.step, i2.lo - int1.step);
            ints.push_front(add);
        }
    }

    Ok(ints)
}

// ============================================================================
// affine
// ============================================================================

/// Affine function for scaling and offsetting an interval.
pub fn affine(int: &INTERVAL, gain: f64, offset: i32) -> Result<INTERVAL> {
    if gain > 0.0 {
        let mut lo = int.lo as f64;
        let mut step = int.step as f64;
        let mut hi = int.hi as f64;

        lo = lo * gain + offset as f64;
        hi = hi * gain + offset as f64;
        step = step * gain;

        if step < 1.0 {
            step = 1.0;
            lo = lo.ceil();
            hi = hi.floor();
        }

        if lo < 0.0 {
            lo = lo + step * (1.0 + (lo.abs() / step).floor());
        }

        if hi < lo {
            return Ok(new_empty());
        }

        let ilo = lo.floor() as i32;
        let ihi = hi.floor() as i32;
        let istep = if ilo == ihi { 1 } else { step.floor() as i32 };

        let res = new(ilo, istep, ihi);
        Ok(res)
    } else {
        if offset > 0 {
            Ok(new(offset, 1, offset))
        } else {
            Ok(new_empty())
        }
    }
}

// ============================================================================
// cardinality
// ============================================================================

/// Cardinality: number of elements as a real-valued count.
/// Maps: realInt(intReal(hi - lo) / intReal(step)).
pub fn cardinality(int: &INTERVAL) -> f64 {
    (int.hi - int.lo) as f64 / int.step as f64
}

// ============================================================================
// contains
// ============================================================================

/// Returns true if c belongs to the interval.
pub fn contains(c: i32, int: &INTERVAL) -> bool {
    !is_empty(int)
        && c >= int.lo
        && c <= int.hi
        && (c - int.lo).rem_euclid(int.step) == 0
}

// ============================================================================
// isEmpty
// ============================================================================

/// Returns true if the interval is empty (step == 0).
pub fn is_empty(int: &INTERVAL) -> bool {
    int.step == 0
}

// ============================================================================
// size
// ============================================================================

/// Returns the number of elements in the interval.
pub fn size(int: &INTERVAL) -> i32 {
    (int.hi - int.lo) / int.step + 1
}

// ============================================================================
// isEqual (int_equals)
// ============================================================================

/// Returns true if two intervals are equal.
pub fn int_equals(int1: &INTERVAL, int2: &INTERVAL) -> bool {
    int1.lo == int2.lo && int1.step == int2.step && int1.hi == int2.hi
}

// ============================================================================
// hash
// ============================================================================

/// Hash function for intervals.
/// Simply returns the lower bound.
pub fn hash(int: &INTERVAL) -> i32 {
    int.lo
}

// ============================================================================
// toString
// ============================================================================

/// String representation: "[lo:step:hi]".
pub fn to_string(int: &INTERVAL) -> String {
    format!("[{}:{}:{}]", int.lo, int.step, int.hi)
}
