//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArrayMtPerSLit;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap35::OrderStatSelectMtPer::OrderStatSelectMtPer::*;

#[test]
fn test_empty() {
    let a: ArraySeqMtPerS<i32> = ArrayMtPerSLit![];
    assert_eq!(ArraySeqMtPerS::select(&a, 0), None);
}

#[test]
fn test_single() {
    let a = ArrayMtPerSLit![42];
    assert_eq!(ArraySeqMtPerS::select(&a, 0), Some(42));
    assert_eq!(ArraySeqMtPerS::select(&a, 1), None);
}

#[test]
fn test_small() {
    let a = ArrayMtPerSLit![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted = ArrayMtPerSLit![1, 1, 2, 3, 4, 5, 6, 9];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(sorted.nth(k).clone()), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArrayMtPerSLit![1, 2, 3, 4, 5];
    for k in 0..5 {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArrayMtPerSLit![5, 4, 3, 2, 1];
    for k in 0..5 {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArrayMtPerSLit![3, 3, 3, 3, 3];
    for k in 0..5 {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArrayMtPerSLit![-5, -2, -8, -1, -9];
    let sorted = ArrayMtPerSLit![-9, -8, -5, -2, -1];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_mixed() {
    let a = ArrayMtPerSLit![-3, 7, -1, 0, 4, -5, 2];
    let sorted = ArrayMtPerSLit![-5, -3, -1, 0, 2, 4, 7];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtPerS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_two_elements() {
    let a = ArrayMtPerSLit![5, 3];
    assert_eq!(ArraySeqMtPerS::select(&a, 0), Some(3));
    assert_eq!(ArraySeqMtPerS::select(&a, 1), Some(5));
    assert_eq!(ArraySeqMtPerS::select(&a, 2), None);
}

#[test]
fn test_large() {
    let n = 1000;
    let a = ArraySeqMtPerS::tabulate(&|i| (n as i32 - i as i32), n);
    assert_eq!(ArraySeqMtPerS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqMtPerS::select(&a, n / 2), Some(n as i32 / 2 + 1));
    assert_eq!(ArraySeqMtPerS::select(&a, n - 1), Some(n as i32));
}
