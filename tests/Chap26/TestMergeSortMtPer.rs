//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for parallel merge sort (Chapter 26).

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap26::MergeSortMtPer::MergeSortMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_merge_sort_parallel_empty() {
    let seq = ArraySeqMtPerS::<usize>::empty();
    let sorted = ArraySeqMtPerS::merge_sort_parallel(&seq);
    assert_eq!(sorted.length(), 0);
}

#[test]
fn test_merge_sort_parallel_single() {
    let seq = ArraySeqMtPerS::singleton(42);
    let sorted = ArraySeqMtPerS::merge_sort_parallel(&seq);
    assert_eq!(sorted.length(), 1);
    assert_eq!(*sorted.nth(0), 42);
}

#[test]
fn test_merge_sort_parallel_sorted() {
    let seq = ArraySeqMtPerS::tabulate(&|i| i, 20);
    let sorted = ArraySeqMtPerS::merge_sort_parallel(&seq);
    assert_eq!(sorted.length(), 20);
    for i in 0..20 {
        assert_eq!(*sorted.nth(i), i);
    }
}

#[test]
fn test_merge_sort_parallel_reverse() {
    let seq = ArraySeqMtPerS::tabulate(&|i| 20 - i, 20);
    let sorted = ArraySeqMtPerS::merge_sort_parallel(&seq);
    assert_eq!(sorted.length(), 20);
    for i in 0..20 {
        assert_eq!(*sorted.nth(i), i + 1);
    }
}

#[test]
fn test_merge_parallel() {
    let left = ArraySeqMtPerS::tabulate(&|i| i * 2, 10);
    let right = ArraySeqMtPerS::tabulate(&|i| i * 2 + 1, 10);
    let result = ArraySeqMtPerS::merge_parallel(&left, &right);
    assert_eq!(result.length(), 20);
    for i in 0..20 {
        assert_eq!(*result.nth(i), i);
    }
}
