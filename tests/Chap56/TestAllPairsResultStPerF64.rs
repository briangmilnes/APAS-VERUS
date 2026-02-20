//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Test suite for AllPairsResultStPerF64.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::AllPairsResultStPerF64::AllPairsResultStPerF64::*;
use apas_verus::vstdplus::float::float::*;

fn dist(v: f64) -> F64Dist { F64Dist { val: v } }

#[test]
fn test_new() {
    let result = AllPairsResultStPerF64::new(3);
    assert_eq!(result.get_distance(0, 0), dist(0.0));
    assert_eq!(result.get_distance(0, 1), dist(f64::INFINITY));
    assert!(result.is_reachable(0, 0));
    assert!(!result.is_reachable(0, 1));
}

#[test]
fn test_set_distance() {
    let result = AllPairsResultStPerF64::new(3);
    let result = result.set_distance(0, 1, dist(5.5));
    let result = result.set_distance(1, 2, dist(10.5));
    assert_eq!(result.get_distance(0, 1), dist(5.5));
    assert_eq!(result.get_distance(1, 2), dist(10.5));
}

#[test]
fn test_set_predecessor() {
    let result = AllPairsResultStPerF64::new(3);
    let result = result.set_predecessor(0, 1, 0);
    let result = result.set_predecessor(1, 2, 1);
    assert_eq!(result.get_predecessor(0, 1), Some(0));
    assert_eq!(result.get_predecessor(1, 2), Some(1));
}

#[test]
fn test_extract_path() {
    let result = AllPairsResultStPerF64::new(4);
    let result = result.set_distance(0, 1, dist(1.0)).set_predecessor(0, 1, 0);
    let result = result.set_distance(0, 2, dist(2.0)).set_predecessor(0, 2, 1);
    let result = result.set_distance(0, 3, dist(3.0)).set_predecessor(0, 3, 2);

    let path = result.extract_path(0, 3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_extract_path_unreachable() {
    let result = AllPairsResultStPerF64::new(3);
    assert_eq!(result.extract_path(0, 2), None);
}
