//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for parallel scan using contraction (Chapter 27).

use std::sync::Arc;

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap27::ScanContractMtEph::ScanContractMtEph::*;

#[test]
fn test_scan_contract_parallel_empty() {
    let a = ArraySeqMtEphS::<usize>::empty();
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 0, "Empty sequence should produce empty result");
}

#[test]
fn test_scan_contract_parallel_single() {
    let a = ArraySeqMtEphS::singleton(42usize);
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 0, "Exclusive scan of [42] should be [0]");
}

#[test]
fn test_scan_contract_parallel_sum() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 5);
    assert_eq!(*result.nth(0), 0); // 0
    assert_eq!(*result.nth(1), 1); // 0+1
    assert_eq!(*result.nth(2), 3); // 0+1+2
    assert_eq!(*result.nth(3), 6); // 0+1+2+3
    assert_eq!(*result.nth(4), 10); // 0+1+2+3+4
}

#[test]
fn test_scan_contract_parallel_product() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 4); // [1, 2, 3, 4]
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x * y), 1);
    assert_eq!(result.length(), 4);
    assert_eq!(*result.nth(0), 1); // 1
    assert_eq!(*result.nth(1), 1); // 1*1
    assert_eq!(*result.nth(2), 2); // 1*1*2
    assert_eq!(*result.nth(3), 6); // 1*1*2*3
}

#[test]
fn test_scan_contract_parallel_odd_length() {
    let a = ArraySeqMtEphS::tabulate(&|_i| 1usize, 7); // [1, 1, 1, 1, 1, 1, 1]
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 7);
    for i in 0..7 {
        assert_eq!(*result.nth(i), i, "scan[{i}] should be {i}");
    }
}

#[test]
fn test_scan_contract_parallel_even_length() {
    let a = ArraySeqMtEphS::tabulate(&|_i| 1usize, 8); // [1, 1, 1, 1, 1, 1, 1, 1]
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 8);
    for i in 0..8 {
        assert_eq!(*result.nth(i), i, "scan[{i}] should be {i}");
    }
}

#[test]
fn test_scan_contract_parallel_large() {
    let a = ArraySeqMtEphS::tabulate(&|_i| 1usize, 100);
    let result = ArraySeqMtEphS::scan_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result.length(), 100);
    assert_eq!(*result.nth(0), 0);
    assert_eq!(*result.nth(50), 50);
    assert_eq!(*result.nth(99), 99);
}
