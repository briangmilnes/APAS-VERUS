//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap35::OrderStatSelectStPer::OrderStatSelectStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let a: ArraySeqStPerS<i32> = ArraySeqStPerS::from_vec(vec![]);
    assert_eq!(a.select(0), None);
}

#[test]
fn test_single() {
    let a = ArraySeqStPerS::from_vec(vec![42]);
    assert_eq!(a.select(0), Some(42));
    assert_eq!(a.select(1), None);
}

#[test]
fn test_small() {
    let a = ArraySeqStPerS::from_vec(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let sorted = ArraySeqStPerS::from_vec(vec![1, 1, 2, 3, 4, 5, 6, 9]);

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(sorted.nth(k).clone()), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArraySeqStPerS::from_vec(vec![1, 2, 3, 4, 5]);
    for k in 0..5 {
        assert_eq!(a.select(k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArraySeqStPerS::from_vec(vec![5, 4, 3, 2, 1]);
    for k in 0..5 {
        assert_eq!(a.select(k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArraySeqStPerS::from_vec(vec![3, 3, 3, 3, 3]);
    for k in 0..5 {
        assert_eq!(a.select(k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArraySeqStPerS::from_vec(vec![-5, -2, -8, -1, -9]);
    let sorted = ArraySeqStPerS::from_vec(vec![-9, -8, -5, -2, -1]);

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_mixed() {
    let a = ArraySeqStPerS::from_vec(vec![-3, 7, -1, 0, 4, -5, 2]);
    let sorted = ArraySeqStPerS::from_vec(vec![-5, -3, -1, 0, 2, 4, 7]);

    for k in 0..sorted.length() {
        assert_eq!(a.select(k), Some(sorted.nth(k).clone()));
    }
}
