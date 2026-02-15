//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for parallel divide-and-conquer scan (Chapter 26).

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap26::ScanDCMtPer::ScanDCMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_prefix_sums_dc_parallel_empty() {
    let seq = ArraySeqMtPerS::<usize>::empty();
    let (prefixes, total) = ArraySeqMtPerS::prefix_sums_dc_parallel(&seq);
    assert_eq!(prefixes.length(), 0);
    assert_eq!(total, 0);
}

#[test]
fn test_prefix_sums_dc_parallel_singleton() {
    let seq = ArraySeqMtPerS::singleton(5);
    let (prefixes, total) = ArraySeqMtPerS::prefix_sums_dc_parallel(&seq);
    assert_eq!(prefixes.length(), 1);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(total, 5);
}

#[test]
fn test_prefix_sums_dc_parallel_small() {
    // Input: [2, 1, 3, 2, 2, 5, 4, 1]
    // Expected exclusive prefixes: [0, 2, 3, 6, 8, 10, 15, 19]
    // Expected total: 20
    let seq = ArraySeqMtPerS::from_vec(vec![2, 1, 3, 2, 2, 5, 4, 1]);
    let (prefixes, total) = ArraySeqMtPerS::prefix_sums_dc_parallel(&seq);
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
fn test_prefix_sums_dc_parallel_consecutive() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i + 1, 5);
    let (prefixes, total) = ArraySeqMtPerS::prefix_sums_dc_parallel(&seq);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(*prefixes.nth(3), 6);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(total, 15);
}

#[test]
fn test_prefix_sums_dc_parallel_large() {
    let n = 1000;
    let seq = ArraySeqMtPerS::tabulate(&|i| i + 1, n);
    let (prefixes, total) = ArraySeqMtPerS::prefix_sums_dc_parallel(&seq);
    assert_eq!(prefixes.length(), n);
    assert_eq!(total, 500500);
    assert_eq!(*prefixes.nth(0), 0);
}
