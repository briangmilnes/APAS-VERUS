#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Test suite for SSSPResultStPerFloat.

use ordered_float::OrderedFloat;

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::SSSPResultStPerFloat::SSSPResultStPerFloat::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let result = SSSPResultStPerFloat::new(5, 0);
    assert_eq!(result.get_distance(0), OrderedFloat(0.0));
    assert_eq!(result.get_distance(1), OrderedFloat(f64::INFINITY));
    assert!(result.is_reachable(0));
    assert!(!result.is_reachable(1));
}

#[test]
fn test_set_distance() {
    let result = SSSPResultStPerFloat::new(3, 0);
    let result = result.set_distance(1, OrderedFloat(5.5));
    let result = result.set_distance(2, OrderedFloat(10.5));
    assert_eq!(result.get_distance(1), OrderedFloat(5.5));
    assert_eq!(result.get_distance(2), OrderedFloat(10.5));
}

#[test]
fn test_set_predecessor() {
    let result = SSSPResultStPerFloat::new(3, 0);
    let result = result.set_predecessor(1, 0);
    let result = result.set_predecessor(2, 1);
    assert_eq!(result.get_predecessor(1), Some(0));
    assert_eq!(result.get_predecessor(2), Some(1));
}

#[test]
fn test_extract_path() {
    let result = SSSPResultStPerFloat::new(4, 0);
    let result = result.set_distance(1, OrderedFloat(1.0)).set_predecessor(1, 0);
    let result = result.set_distance(2, OrderedFloat(2.0)).set_predecessor(2, 1);
    let result = result.set_distance(3, OrderedFloat(3.0)).set_predecessor(3, 2);

    let path = result.extract_path(3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_extract_path_unreachable() {
    let result = SSSPResultStPerFloat::new(3, 0);
    assert_eq!(result.extract_path(2), None);
}
