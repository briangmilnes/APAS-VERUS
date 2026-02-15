//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for sequential divide-and-conquer scan (Chapter 26).

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap26::ScanDCStPer::ScanDCStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_prefix_sums_dc_empty() {
    let seq = ArraySeqStPerS::<usize>::empty();
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), 0);
    assert_eq!(total, 0);
}

#[test]
fn test_prefix_sums_dc_singleton() {
    let seq = ArraySeqStPerS::singleton(5);
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), 1);
    assert_eq!(*prefixes.nth(0), 0); // exclusive: prefix[0] = id = 0
    assert_eq!(total, 5);
}

#[test]
fn test_prefix_sums_dc_small() {
    // Input: [2, 1, 3, 2, 2, 5, 4, 1]
    // Expected exclusive prefixes: [0, 2, 3, 6, 8, 10, 15, 19]
    // Expected total: 20
    let seq = ArraySeqStPerS::from_vec(vec![2, 1, 3, 2, 2, 5, 4, 1]);
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), 8);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 2);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(*prefixes.nth(3), 6);
    assert_eq!(*prefixes.nth(4), 8);
    assert_eq!(*prefixes.nth(5), 10);
    assert_eq!(*prefixes.nth(6), 15);
    assert_eq!(*prefixes.nth(7), 19);
    assert_eq!(total, 20);
}

#[test]
fn test_prefix_sums_dc_consecutive() {
    // Input: [1, 2, 3, 4, 5]
    // Exclusive prefixes: [0, 1, 3, 6, 10]
    // Total: 15
    let seq = ArraySeqStPerS::tabulate(&|i| i + 1, 5);
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(*prefixes.nth(3), 6);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(total, 15);
}

#[test]
fn test_prefix_sums_dc_tabulate() {
    // Input: [0, 1, 2, ..., 9]
    // Total: 45
    let seq = ArraySeqStPerS::tabulate(&|i| i, 10);
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), 10);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 0);
    assert_eq!(*prefixes.nth(2), 1);
    assert_eq!(*prefixes.nth(3), 3);
    assert_eq!(total, 45);
}

#[test]
fn test_prefix_sums_dc_large() {
    let n = 1000;
    let seq = ArraySeqStPerS::tabulate(&|i| i + 1, n);
    let (prefixes, total) = ArraySeqStPerS::prefix_sums_dc(&seq);
    assert_eq!(prefixes.length(), n);
    // total = n*(n+1)/2 = 500500
    assert_eq!(total, 500500);
    // First prefix is always 0
    assert_eq!(*prefixes.nth(0), 0);
}
