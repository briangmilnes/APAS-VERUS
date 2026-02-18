//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for sequential reduce using contraction (Chapter 27).

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap27::ReduceContractStEph::ReduceContractStEph::*;
use vstd::prelude::Ghost;

#[test]
fn test_reduce_contract_empty() {
    let a = ArraySeqStEphS::<usize>::empty();
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 0, "Empty sequence should return identity");
}

#[test]
fn test_reduce_contract_single() {
    let a = ArraySeqStEphS::singleton(42usize);
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 42, "Single element should return that element");
}

#[test]
fn test_reduce_contract_sum() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 10); // [1, 2, 3, ..., 10]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 55, "Sum of 1..=10 should be 55");
}

#[test]
fn test_reduce_contract_product() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 5); // [1, 2, 3, 4, 5]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x * y, Ghost::assume_new(), 1);
    assert_eq!(result, 120, "Product of 1..=5 should be 120");
}

#[test]
fn test_reduce_contract_max() {
    let a = ArraySeqStEphS::tabulate(&|i| i * 2, 8); // [0, 2, 4, 6, 8, 10, 12, 14]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| if x > y { *x } else { *y }, Ghost::assume_new(), 0);
    assert_eq!(result, 14, "Max should be 14");
}

#[test]
fn test_reduce_contract_odd_length() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 7); // [1, 2, 3, 4, 5, 6, 7]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 28, "Sum of 1..=7 should be 28");
}

#[test]
fn test_reduce_contract_even_length() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 8); // [1, 2, 3, 4, 5, 6, 7, 8]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 36, "Sum of 1..=8 should be 36");
}

#[test]
fn test_reduce_contract_two_elements() {
    let a = ArraySeqStEphS::tabulate(&|i| i + 1, 2); // [1, 2]
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 3, "Sum of [1, 2] should be 3");
}

#[test]
fn test_reduce_contract_power_of_2() {
    let a = ArraySeqStEphS::tabulate(&|_i| 1usize, 16);
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 16, "Sum of 16 ones should be 16");

    let a32 = ArraySeqStEphS::tabulate(&|_i| 1usize, 32);
    let result32 = ArraySeqStEphS::reduce_contract(&a32, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result32, 32, "Sum of 32 ones should be 32");
}

#[test]
fn test_reduce_contract_large() {
    let a = ArraySeqStEphS::tabulate(&|_i| 1usize, 1000);
    let result = ArraySeqStEphS::reduce_contract(&a, &|x, y| x + y, Ghost::assume_new(), 0);
    assert_eq!(result, 1000, "Sum of 1000 ones should be 1000");
}
