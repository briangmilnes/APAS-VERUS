#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Test suite for AllPairsResultStPerInt.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::AllPairsResultStPerInt::AllPairsResultStPerInt::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let result = AllPairsResultStPerInt::new(3);
    assert_eq!(result.get_distance(0, 0), 0);
    assert_eq!(result.get_distance(0, 1), i64::MAX);
    assert!(result.is_reachable(0, 0));
    assert!(!result.is_reachable(0, 1));
}

#[test]
fn test_set_distance() {
    let result = AllPairsResultStPerInt::new(3);
    let result = result.set_distance(0, 1, 5);
    let result = result.set_distance(1, 2, 10);
    assert_eq!(result.get_distance(0, 1), 5);
    assert_eq!(result.get_distance(1, 2), 10);
}

#[test]
fn test_set_predecessor() {
    let result = AllPairsResultStPerInt::new(3);
    let result = result.set_predecessor(0, 1, 0);
    let result = result.set_predecessor(1, 2, 1);
    assert_eq!(result.get_predecessor(0, 1), Some(0));
    assert_eq!(result.get_predecessor(1, 2), Some(1));
}

#[test]
fn test_extract_path() {
    let result = AllPairsResultStPerInt::new(4);
    let result = result.set_distance(0, 1, 1).set_predecessor(0, 1, 0);
    let result = result.set_distance(0, 2, 2).set_predecessor(0, 2, 1);
    let result = result.set_distance(0, 3, 3).set_predecessor(0, 3, 2);

    let path = result.extract_path(0, 3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_extract_path_unreachable() {
    let result = AllPairsResultStPerInt::new(3);
    assert_eq!(result.extract_path(0, 2), None);
}
