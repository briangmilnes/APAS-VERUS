//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for parallel reduce using contraction (Chapter 27).

use std::sync::Arc;

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap27::ReduceContractMtEph::ReduceContractMtEph::*;

#[test]
fn test_reduce_contract_parallel_empty() {
    let a = ArraySeqMtEphS::<usize>::empty();
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 0, "Empty sequence should return identity");
}

#[test]
fn test_reduce_contract_parallel_single() {
    let a = ArraySeqMtEphS::singleton(42usize);
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 42, "Single element should return that element");
}

#[test]
fn test_reduce_contract_parallel_sum() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 10); // [1, 2, 3, ..., 10]
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 55, "Sum of 1..=10 should be 55");
}

#[test]
fn test_reduce_contract_parallel_product() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x * y), 1);
    assert_eq!(result, 120, "Product of 1..=5 should be 120");
}

#[test]
fn test_reduce_contract_parallel_max() {
    let a = ArraySeqMtEphS::tabulate(&|i| i * 2, 8); // [0, 2, 4, 6, 8, 10, 12, 14]
    let result =
        ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| if x > y { *x } else { *y }), 0);
    assert_eq!(result, 14, "Max should be 14");
}

#[test]
fn test_reduce_contract_parallel_odd_length() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 7); // [1, 2, 3, 4, 5, 6, 7]
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 28, "Sum of 1..=7 should be 28");
}

#[test]
fn test_reduce_contract_parallel_even_length() {
    let a = ArraySeqMtEphS::tabulate(&|i| i + 1, 8); // [1, 2, 3, 4, 5, 6, 7, 8]
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 36, "Sum of 1..=8 should be 36");
}

#[test]
fn test_reduce_contract_parallel_large() {
    let a = ArraySeqMtEphS::tabulate(&|_i| 1usize, 1000);
    let result = ArraySeqMtEphS::reduce_contract_parallel(&a, Arc::new(|x: &usize, y: &usize| x + y), 0);
    assert_eq!(result, 1000, "Sum of 1000 ones should be 1000");
}
