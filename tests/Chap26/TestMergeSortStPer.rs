//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for sequential merge sort (Chapter 26).

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap26::MergeSortStPer::MergeSortStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_merge_empty() {
    let left = ArraySeqStPerS::<usize>::empty();
    let right = ArraySeqStPerS::<usize>::empty();
    let result = ArraySeqStPerS::merge(&left, &right);
    assert_eq!(result.length(), 0);
}

#[test]
fn test_merge_one_empty() {
    let left = ArraySeqStPerS::singleton(5);
    let right = ArraySeqStPerS::<usize>::empty();
    let result = ArraySeqStPerS::merge(&left, &right);
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 5);
}

#[test]
fn test_merge_sorted() {
    let left = ArraySeqStPerS::tabulate(&|i| i * 2, 5); // [0, 2, 4, 6, 8]
    let right = ArraySeqStPerS::tabulate(&|i| i * 2 + 1, 5); // [1, 3, 5, 7, 9]
    let result = ArraySeqStPerS::merge(&left, &right);
    assert_eq!(result.length(), 10);
    for i in 0..10 {
        assert_eq!(*result.nth(i), i);
    }
}

#[test]
fn test_merge_sort_empty() {
    let seq = ArraySeqStPerS::<usize>::empty();
    let sorted = ArraySeqStPerS::merge_sort(&seq);
    assert_eq!(sorted.length(), 0);
}

#[test]
fn test_merge_sort_single() {
    let seq = ArraySeqStPerS::singleton(42);
    let sorted = ArraySeqStPerS::merge_sort(&seq);
    assert_eq!(sorted.length(), 1);
    assert_eq!(*sorted.nth(0), 42);
}

#[test]
fn test_merge_sort_sorted() {
    let seq = ArraySeqStPerS::tabulate(&|i| i, 10);
    let sorted = ArraySeqStPerS::merge_sort(&seq);
    assert_eq!(sorted.length(), 10);
    for i in 0..10 {
        assert_eq!(*sorted.nth(i), i);
    }
}

#[test]
fn test_merge_sort_reverse() {
    let seq = ArraySeqStPerS::tabulate(&|i| 10 - i, 10);
    let sorted = ArraySeqStPerS::merge_sort(&seq);
    assert_eq!(sorted.length(), 10);
    for i in 0..10 {
        assert_eq!(*sorted.nth(i), i + 1);
    }
}

#[test]
fn test_merge_sort_duplicates() {
    let seq = ArraySeqStPerS::tabulate(&|i| i % 3, 12); // [0,1,2,0,1,2,0,1,2,0,1,2]
    let sorted = ArraySeqStPerS::merge_sort(&seq);
    assert_eq!(sorted.length(), 12);
    for i in 0..4 {
        assert_eq!(*sorted.nth(i), 0);
        assert_eq!(*sorted.nth(i + 4), 1);
        assert_eq!(*sorted.nth(i + 8), 2);
    }
}
