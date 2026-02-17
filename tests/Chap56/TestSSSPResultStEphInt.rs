#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Test suite for SSSPResultStEphInt.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::SSSPResultStEphInt::SSSPResultStEphInt::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let result = SSSPResultStEphInt::new(5, 0);
    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), i64::MAX);
    assert!(result.is_reachable(0));
    assert!(!result.is_reachable(1));
}

#[test]
fn test_set_distance() {
    let mut result = SSSPResultStEphInt::new(3, 0);
    result.set_distance(1, 5);
    result.set_distance(2, 10);
    assert_eq!(result.get_distance(1), 5);
    assert_eq!(result.get_distance(2), 10);
}

#[test]
fn test_set_predecessor() {
    let mut result = SSSPResultStEphInt::new(3, 0);
    result.set_predecessor(1, 0);
    result.set_predecessor(2, 1);
    assert_eq!(result.get_predecessor(1), Some(0));
    assert_eq!(result.get_predecessor(2), Some(1));
}

#[test]
fn test_extract_path() {
    let mut result = SSSPResultStEphInt::new(4, 0);
    result.set_distance(1, 1);
    result.set_predecessor(1, 0);
    result.set_distance(2, 2);
    result.set_predecessor(2, 1);
    result.set_distance(3, 3);
    result.set_predecessor(3, 2);

    let path = result.extract_path(3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_extract_path_unreachable() {
    let result = SSSPResultStEphInt::new(3, 0);
    assert_eq!(result.extract_path(2), None);
}
