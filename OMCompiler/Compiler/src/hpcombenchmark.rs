//! Translation of BackEnd/HpcOmBenchmark.mo
//!
//! This module provides benchmark timing functions for measuring communication
//! and processing time in the OpenModelica runtime.
//!
//! All data access functions ultimately call into the `hpcombenchmarkext` module
//! which provides FFI bindings to the `omcruntime` C library.

use anyhow::{bail, Result};
use crate::hpcombenchmarkext;

type List<T> = im::Vector<T>;

// ============================================================================
// benchSystem
// ============================================================================

/// Returns the required time for <op, com> as nested tuples:
/// ((opCostM, opCostN), (comCostM, comCostN)).
/// Deprecated: external C dependency on omcruntime via hpcombenchmarkext.
pub fn bench_system() -> ((i32, i32), (i32, i32)) {
    let op_costs = hpcombenchmarkext::list_required_time_for_op();
    let op_m = *op_costs.get(0).unwrap_or(&0);
    let op_n = *op_costs.get(1).unwrap_or(&0);

    let com_costs = hpcombenchmarkext::list_required_time_for_comm();
    let com_m = *com_costs.get(0).unwrap_or(&0);
    let com_n = *com_costs.get(1).unwrap_or(&0);

    ((op_m, op_n), (com_m, com_n))
}

// ============================================================================
// readCalcTimesFromFile
// ============================================================================

/// Tries to find a file named `<iFileNamePrefix>.json` or
/// `<iFileNamePrefix>.xml`. If such a file exists, the calculation times are
/// read out. If not, the function will fail.
/// Deprecated: external C dependency on omcruntime via hpcombenchmarkext.
pub fn read_calc_times_from_file(i_file_name_prefix: &str) -> Result<List<(i32, i32, f64)>> {
    if let Ok(v) = read_calc_times_from_json_impl(i_file_name_prefix) {
        return Ok(v);
    }
    if let Ok(v) = read_calc_times_from_xml_impl(i_file_name_prefix) {
        return Ok(v);
    }
    println!("readCalcTimesFromFile: No valid profiling-file found.\n");
    bail!("No valid profiling-file found")
}

fn read_calc_times_from_json_impl(i_file_name_prefix: &str) -> Result<List<(i32, i32, f64)>> {
    let full_file_name = format!("{}.json", i_file_name_prefix);
    std::fs::metadata(&full_file_name).map(|_| ())?;
    println!("Using json-file\n");
    let tmp = hpcombenchmarkext::list_read_calc_times_from_json(&full_file_name);
    Ok(expand_calc_times(&tmp))
}

fn read_calc_times_from_xml_impl(i_file_name_prefix: &str) -> Result<List<(i32, i32, f64)>> {
    let full_file_name = format!("{}.xml", i_file_name_prefix);
    std::fs::metadata(&full_file_name).map(|_| ())?;
    let tmp = hpcombenchmarkext::list_read_calc_times_from_xml(&full_file_name);
    Ok(expand_calc_times(&tmp))
}

// ============================================================================
// expandCalcTimes
// ============================================================================

/// Takes a list of real vars and puts the first three entries into the tuple
/// list. Then cuts off these entries and repeats iteratively.
fn expand_calc_times(i_list: &List<f64>) -> List<(i32, i32, f64)> {
    if i_list.len() % 3 != 0 {
        // This case would trigger fail() in the original matchcontinue
        eprintln!("expandCalcTimes: Invalid number of list-entries\n");
        return im::vector![];
    }

    // The original list pattern is: numOfCalcs::calcTimeSum::eqIdx::rest
    // So elements are: [0]=numOfCalcs, [1]=calcTimeSum, [2]=eqIdx
    // The tuple is built as: (eqIdx, numOfCalcs, calcTimeSum)
    let mut result: List<(i32, i32, f64)> = im::vector![];

    let data: Vec<f64> = i_list.iter().cloned().collect();
    for chunk in data.chunks_exact(3) {
        let num_of_calcs = (chunk[0] as i64) as i32;
        let calc_time_sum = chunk[1];
        let eq_idx = (chunk[2] as i64) as i32;
        result.push_front((eq_idx, num_of_calcs, calc_time_sum));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_calc_times_basic() {
        // List: [numOfCalcs=1, calcTime=2.0, eqIdx=0, ...]
        let data: List<f64> = im::vector![1.0, 2.0, 0.0, 3.0, 4.0, 1.0];
        let result = expand_calc_times(&data);
        // First chunk: eqIdx=0, numOfCalcs=1, calcTime=2.0 -> pushed front first
        // Second chunk: eqIdx=1, numOfCalcs=3, calcTime=4.0 -> pushed front second (will be at front)
        assert_eq!(result.len(), 2);
        let (first, second) = (result.get(0).unwrap(), result.get(1).unwrap());
        assert_eq!(*first, (1, 3, 4.0));
        assert_eq!(*second, (0, 1, 2.0));
    }

    #[test]
    fn test_expand_calc_times_empty() {
        let data: List<f64> = im::vector![];
        let result = expand_calc_times(&data);
        assert!(result.is_empty());
    }

    #[test]
    fn test_expand_calc_times_single() {
        let data: List<f64> = im::vector![5.0, 10.0, 2.0];
        let result = expand_calc_times(&data);
        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), (2, 5, 10.0));
    }

    #[test]
    fn test_expand_calc_times_invalid_length() {
        let data: List<f64> = im::vector![1.0, 2.0]; // Not divisible by 3
        let result = expand_calc_times(&data);
        assert!(result.is_empty()); // Returns empty on invalid length
    }
}
