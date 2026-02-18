//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap35::OrderStatSelectStPer::OrderStatSelectStPer::*;

#[test]
fn test_empty() {
    let a: ArraySeqStPerS<i32> = ArraySeqStPerS::from_vec(vec![]);
    assert_eq!(ArraySeqStPerS::select(&a, 0), None);
}

#[test]
fn test_single() {
    let a = ArraySeqStPerS::from_vec(vec![42]);
    assert_eq!(ArraySeqStPerS::select(&a, 0), Some(42));
    assert_eq!(ArraySeqStPerS::select(&a, 1), None);
}

#[test]
fn test_small() {
    let a = ArraySeqStPerS::from_vec(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let sorted = ArraySeqStPerS::from_vec(vec![1, 1, 2, 3, 4, 5, 6, 9]);

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(sorted.nth(k).clone()), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArraySeqStPerS::from_vec(vec![1, 2, 3, 4, 5]);
    for k in 0..5 {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArraySeqStPerS::from_vec(vec![5, 4, 3, 2, 1]);
    for k in 0..5 {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArraySeqStPerS::from_vec(vec![3, 3, 3, 3, 3]);
    for k in 0..5 {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArraySeqStPerS::from_vec(vec![-5, -2, -8, -1, -9]);
    let sorted = ArraySeqStPerS::from_vec(vec![-9, -8, -5, -2, -1]);

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_mixed() {
    let a = ArraySeqStPerS::from_vec(vec![-3, 7, -1, 0, 4, -5, 2]);
    let sorted = ArraySeqStPerS::from_vec(vec![-5, -3, -1, 0, 2, 4, 7]);

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStPerS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_two_elements() {
    let a = ArraySeqStPerS::from_vec(vec![5, 3]);
    assert_eq!(ArraySeqStPerS::select(&a, 0), Some(3));
    assert_eq!(ArraySeqStPerS::select(&a, 1), Some(5));
    assert_eq!(ArraySeqStPerS::select(&a, 2), None);
}

#[test]
fn test_large() {
    let n = 1000;
    let a = ArraySeqStPerS::tabulate(&|i| (n as i32 - i as i32), n);
    assert_eq!(ArraySeqStPerS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqStPerS::select(&a, n / 2), Some(n as i32 / 2 + 1));
    assert_eq!(ArraySeqStPerS::select(&a, n - 1), Some(n as i32));
}
