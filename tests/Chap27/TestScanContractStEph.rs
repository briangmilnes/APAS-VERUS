//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for sequential scan using contraction (Chapter 27).

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap27::ScanContractStEph::ScanContractStEph::*;
use vstd::prelude::Ghost;

#[test]
fn test_scan_contract_empty() {
    let a = ArraySeqStEphS::<usize>::empty();
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 0, "Empty sequence should produce empty result");
}

#[test]
fn test_scan_contract_single() {
    let a = ArraySeqStEphS::singleton(42usize);
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 1);
    assert_eq!(result.nth(0), &0, "Exclusive scan of [42] should be [0]");
}

#[test]
fn test_scan_contract_sum() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 5);
    assert_eq!(result.nth(0), &0); // 0
    assert_eq!(result.nth(1), &1); // 0+1
    assert_eq!(result.nth(2), &3); // 0+1+2
    assert_eq!(result.nth(3), &6); // 0+1+2+3
    assert_eq!(result.nth(4), &10); // 0+1+2+3+4
}

#[test]
fn test_scan_contract_product() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 4); // [1, 2, 3, 4]
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x * y, Ghost::assume_new(), 1);
    assert_eq!(result.length(), 4);
    assert_eq!(result.nth(0), &1); // 1
    assert_eq!(result.nth(1), &1); // 1*1
    assert_eq!(result.nth(2), &2); // 1*1*2
    assert_eq!(result.nth(3), &6); // 1*1*2*3
}

#[test]
fn test_scan_contract_odd_length() {
    let a = ArraySeqStEphS::tabulate(&|_i| 1usize, 7); // [1, 1, 1, 1, 1, 1, 1]
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 7);
    for i in 0..7 {
        assert_eq!(result.nth(i), &i, "scan[{i}] should be {i}");
    }
}

#[test]
fn test_scan_contract_even_length() {
    let a = ArraySeqStEphS::tabulate(&|_i| 1usize, 8); // [1, 1, 1, 1, 1, 1, 1, 1]
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 8);
    for i in 0..8 {
        assert_eq!(result.nth(i), &i, "scan[{i}] should be {i}");
    }
}

#[test]
fn test_scan_contract_large() {
    let a = ArraySeqStEphS::tabulate(&|_i| 1usize, 100);
    let result = ArraySeqStEphS::scan_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result.length(), 100);
    assert_eq!(result.nth(0), &0);
    assert_eq!(result.nth(50), &50);
    assert_eq!(result.nth(99), &99);
}
