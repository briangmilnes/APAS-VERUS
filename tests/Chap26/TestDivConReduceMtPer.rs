//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for parallel divide-and-conquer via reduce (Chapter 26).

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap26::DivConReduceMtPer::DivConReduceMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_max_element_parallel_empty() {
    let seq = ArraySeqMtPerS::<usize>::empty();
    let result = ArraySeqMtPerS::max_element_parallel(&seq);
    assert_eq!(result, None);
}

#[test]
fn test_max_element_parallel() {
    let seq = ArraySeqMtPerS::tabulate(&|i| (i * 7) % 23, 20);
    let result = ArraySeqMtPerS::max_element_parallel(&seq);
    assert!(result.is_some());
}

#[test]
fn test_sum_parallel() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i, 11);
    let result = ArraySeqMtPerS::sum_parallel(&seq);
    assert_eq!(result, 55); // 0+1+2+...+10 = 55
}

#[test]
fn test_product_parallel() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i + 1, 5);
    let result = ArraySeqMtPerS::product_parallel(&seq);
    assert_eq!(result, 120); // 1*2*3*4*5 = 120
}

#[test]
fn test_any_parallel_true() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i == 5, 10);
    let result = ArraySeqMtPerS::any_parallel(&seq);
    assert!(result);
}

#[test]
fn test_any_parallel_false() {
    let seq = ArraySeqMtPerS::tabulate(&|_i| false, 10);
    let result = ArraySeqMtPerS::any_parallel(&seq);
    assert!(!result);
}

#[test]
fn test_all_parallel_true() {
    let seq = ArraySeqMtPerS::tabulate(&|_i| true, 10);
    let result = ArraySeqMtPerS::all_parallel(&seq);
    assert!(result);
}

#[test]
fn test_all_parallel_false() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i != 5, 10);
    let result = ArraySeqMtPerS::all_parallel(&seq);
    assert!(!result);
}
