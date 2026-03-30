//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap35::OrderStatSelectStEph::OrderStatSelectStEph::*;

#[test]
fn test_empty() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    assert_eq!(ArraySeqStEphS::select(&a, 0), None);
}

#[test]
fn test_single() {
    let a = ArraySeqStEphSLit![42];
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(42));
    assert_eq!(ArraySeqStEphS::select(&a, 1), None);
}

#[test]
fn test_small() {
    let a = ArraySeqStEphSLit![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted = ArraySeqStEphSLit![1, 1, 2, 3, 4, 5, 6, 9];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(*sorted.nth(k)), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    for k in 0..5 {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArraySeqStEphSLit![5, 4, 3, 2, 1];
    for k in 0..5 {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArraySeqStEphSLit![3, 3, 3, 3, 3];
    for k in 0..5 {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArraySeqStEphSLit![-5, -2, -8, -1, -9];
    let sorted = ArraySeqStEphSLit![-9, -8, -5, -2, -1];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(*sorted.nth(k)));
    }
}

#[test]
fn test_mixed() {
    let a = ArraySeqStEphSLit![-3, 7, -1, 0, 4, -5, 2];
    let sorted = ArraySeqStEphSLit![-5, -3, -1, 0, 2, 4, 7];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqStEphS::select(&a, k), Some(*sorted.nth(k)));
    }
}

#[test]
fn test_two_elements() {
    let a = ArraySeqStEphSLit![5, 3];
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(3));
    assert_eq!(ArraySeqStEphS::select(&a, 1), Some(5));
    assert_eq!(ArraySeqStEphS::select(&a, 2), None);
}

#[test]
fn test_large() {
    let n = 1000;
    let a = ArraySeqStEphS::tabulate(&|i| (n as i32 - i as i32), n);
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqStEphS::select(&a, n / 2), Some(n as i32 / 2 + 1));
    assert_eq!(ArraySeqStEphS::select(&a, n - 1), Some(n as i32));
}

#[test]
fn test_out_of_bounds_various_sizes() {
    for size in 0..10usize {
        let a = ArraySeqStEphS::tabulate(&|i| i as i32, size);
        assert_eq!(ArraySeqStEphS::select(&a, size), None);
        assert_eq!(ArraySeqStEphS::select(&a, size + 1), None);
        assert_eq!(ArraySeqStEphS::select(&a, 1000), None);
    }
}

#[test]
fn test_min_max_values() {
    let a = ArraySeqStEphSLit![i32::MAX, i32::MIN, 0, i32::MAX, i32::MIN];
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(i32::MIN));
    assert_eq!(ArraySeqStEphS::select(&a, 1), Some(i32::MIN));
    assert_eq!(ArraySeqStEphS::select(&a, 2), Some(0));
    assert_eq!(ArraySeqStEphS::select(&a, 3), Some(i32::MAX));
    assert_eq!(ArraySeqStEphS::select(&a, 4), Some(i32::MAX));
}

#[test]
fn test_all_same_select_minimum() {
    let a = ArraySeqStEphSLit![42, 42, 42, 42, 42, 42, 42, 42, 42, 42];
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(42));
    assert_eq!(ArraySeqStEphS::select(&a, 9), Some(42));
}

#[test]
fn test_alternating_two_values() {
    let a = ArraySeqStEphSLit![1, 100, 1, 100, 1, 100];
    assert_eq!(ArraySeqStEphS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqStEphS::select(&a, 1), Some(1));
    assert_eq!(ArraySeqStEphS::select(&a, 2), Some(1));
    assert_eq!(ArraySeqStEphS::select(&a, 3), Some(100));
    assert_eq!(ArraySeqStEphS::select(&a, 4), Some(100));
    assert_eq!(ArraySeqStEphS::select(&a, 5), Some(100));
}

#[test]
fn test_select_median() {
    let a = ArraySeqStEphSLit![7, 2, 9, 4, 5, 1, 8, 3, 6];
    // Median of 9 elements is at index 4.
    assert_eq!(ArraySeqStEphS::select(&a, 4), Some(5));
}
