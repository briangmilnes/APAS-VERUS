// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//!
//! Test suite for SSSPResultStEphI64.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let result = SSSPResultStEphI64::new(5, 0);
    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), i64::MAX);
    assert!(result.is_reachable(0));
    assert!(!result.is_reachable(1));
}

#[test]
fn test_set_distance() {
    let mut result = SSSPResultStEphI64::new(3, 0);
    result.set_distance(1, 5);
    result.set_distance(2, 10);
    assert_eq!(result.get_distance(1), 5);
    assert_eq!(result.get_distance(2), 10);
}

#[test]
fn test_set_predecessor() {
    let mut result = SSSPResultStEphI64::new(3, 0);
    result.set_predecessor(1, 0);
    result.set_predecessor(2, 1);
    assert_eq!(result.get_predecessor(1), Some(0));
    assert_eq!(result.get_predecessor(2), Some(1));
}

#[test]
fn test_extract_path() {
    let mut result = SSSPResultStEphI64::new(4, 0);
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
    let result = SSSPResultStEphI64::new(3, 0);
    assert_eq!(result.extract_path(2), None);
}

#[test]
fn test_extract_path_source() {
    let result = SSSPResultStEphI64::new(3, 0);
    let path = result.extract_path(0).unwrap();
    assert_eq!(path.length(), 1);
    assert_eq!(*path.nth(0), 0);
}

#[test]
fn test_is_reachable() {
    let mut result = SSSPResultStEphI64::new(4, 0);
    assert!(result.is_reachable(0));
    assert!(!result.is_reachable(1));

    result.set_distance(1, 5);
    assert!(result.is_reachable(1));

    result.set_distance(2, 0);
    assert!(result.is_reachable(2));
}

#[test]
fn test_distances_array() {
    let mut result = SSSPResultStEphI64::new(5, 0);
    result.set_distance(1, 10);
    result.set_distance(2, 20);
    result.set_distance(3, 30);
    result.set_distance(4, 40);

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 10);
    assert_eq!(result.get_distance(2), 20);
    assert_eq!(result.get_distance(3), 30);
    assert_eq!(result.get_distance(4), 40);
}

#[test]
fn test_overwrite_distance() {
    let mut result = SSSPResultStEphI64::new(3, 0);
    result.set_distance(1, 100);
    assert_eq!(result.get_distance(1), 100);
    result.set_distance(1, 50);
    assert_eq!(result.get_distance(1), 50);
}
