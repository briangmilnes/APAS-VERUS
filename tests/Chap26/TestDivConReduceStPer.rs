//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for sequential divide-and-conquer via reduce (Chapter 26).

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap26::DivConReduceStPer::DivConReduceStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_max_element_empty() {
    let seq = ArraySeqStPerS::<usize>::empty();
    let result = ArraySeqStPerS::max_element(&seq);
    assert_eq!(result, None);
}

#[test]
fn test_max_element() {
    let seq = ArraySeqStPerS::tabulate(&|i| (i * 7) % 23, 20);
    let result = ArraySeqStPerS::max_element(&seq);
    assert!(result.is_some());
}

#[test]
fn test_sum() {
    let seq = ArraySeqStPerS::tabulate(&|i| i, 11);
    let result = ArraySeqStPerS::sum(&seq);
    assert_eq!(result, 55); // 0+1+2+...+10 = 55
}

#[test]
fn test_product() {
    let seq = ArraySeqStPerS::tabulate(&|i| i + 1, 5);
    let result = ArraySeqStPerS::product(&seq);
    assert_eq!(result, 120); // 1*2*3*4*5 = 120
}

#[test]
fn test_any_true() {
    let seq = ArraySeqStPerS::tabulate(&|i| i == 5, 10);
    let result = ArraySeqStPerS::any(&seq);
    assert!(result);
}

#[test]
fn test_any_false() {
    let seq = ArraySeqStPerS::tabulate(&|_i| false, 10);
    let result = ArraySeqStPerS::any(&seq);
    assert!(!result);
}

#[test]
fn test_all_true() {
    let seq = ArraySeqStPerS::tabulate(&|_i| true, 10);
    let result = ArraySeqStPerS::all(&seq);
    assert!(result);
}

#[test]
fn test_all_false() {
    let seq = ArraySeqStPerS::tabulate(&|i| i != 5, 10);
    let result = ArraySeqStPerS::all(&seq);
    assert!(!result);
}
