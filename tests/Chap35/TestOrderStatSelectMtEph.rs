// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

use apas_verus::ArraySeqMtEphChap19SLit;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphTrait, *};
use apas_verus::Chap35::OrderStatSelectMtEph::OrderStatSelectMtEph::*;

#[test]
fn test_empty() {
    let a: ArraySeqMtEphS<i32> = ArraySeqMtEphChap19SLit![];
    assert_eq!(ArraySeqMtEphS::select(&a, 0), None);
}

#[test]
fn test_single() {
    let a = ArraySeqMtEphChap19SLit![42];
    assert_eq!(ArraySeqMtEphS::select(&a, 0), Some(42));
    assert_eq!(ArraySeqMtEphS::select(&a, 1), None);
}

#[test]
fn test_small() {
    let a = ArraySeqMtEphChap19SLit![3, 1, 4, 1, 5, 9, 2, 6];
    let sorted = ArraySeqMtEphChap19SLit![1, 1, 2, 3, 4, 5, 6, 9];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(sorted.nth(k).clone()), "Failed at k={k}");
    }
}

#[test]
fn test_already_sorted() {
    let a = ArraySeqMtEphChap19SLit![1, 2, 3, 4, 5];
    for k in 0..5 {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_reverse_sorted() {
    let a = ArraySeqMtEphChap19SLit![5, 4, 3, 2, 1];
    for k in 0..5 {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(k as i32 + 1));
    }
}

#[test]
fn test_duplicates() {
    let a = ArraySeqMtEphChap19SLit![3, 3, 3, 3, 3];
    for k in 0..5 {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(3));
    }
}

#[test]
fn test_negative() {
    let a = ArraySeqMtEphChap19SLit![-5, -2, -8, -1, -9];
    let sorted = ArraySeqMtEphChap19SLit![-9, -8, -5, -2, -1];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_mixed() {
    let a = ArraySeqMtEphChap19SLit![-3, 7, -1, 0, 4, -5, 2];
    let sorted = ArraySeqMtEphChap19SLit![-5, -3, -1, 0, 2, 4, 7];

    for k in 0..sorted.length() {
        assert_eq!(ArraySeqMtEphS::select(&a, k), Some(sorted.nth(k).clone()));
    }
}

#[test]
fn test_two_elements() {
    let a = ArraySeqMtEphChap19SLit![5, 3];
    assert_eq!(ArraySeqMtEphS::select(&a, 0), Some(3));
    assert_eq!(ArraySeqMtEphS::select(&a, 1), Some(5));
    assert_eq!(ArraySeqMtEphS::select(&a, 2), None);
}

#[test]
fn test_large() {
    let n = 1000;
    let a = ArraySeqMtEphS::tabulate(&|i| (n as i32 - i as i32), n);
    assert_eq!(ArraySeqMtEphS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqMtEphS::select(&a, n / 2), Some(n as i32 / 2 + 1));
    assert_eq!(ArraySeqMtEphS::select(&a, n - 1), Some(n as i32));
}

#[test]
fn test_out_of_bounds() {
    for size in 0..10usize {
        let a = ArraySeqMtEphS::tabulate(&|i| i as i32, size);
        assert_eq!(ArraySeqMtEphS::select(&a, size), None);
        assert_eq!(ArraySeqMtEphS::select(&a, size + 1), None);
    }
}

#[test]
fn test_min_max_values() {
    let a = ArraySeqMtEphChap19SLit![i32::MAX, i32::MIN, 0, i32::MAX, i32::MIN];
    assert_eq!(ArraySeqMtEphS::select(&a, 0), Some(i32::MIN));
    assert_eq!(ArraySeqMtEphS::select(&a, 2), Some(0));
    assert_eq!(ArraySeqMtEphS::select(&a, 4), Some(i32::MAX));
}

#[test]
fn test_alternating_values() {
    let a = ArraySeqMtEphChap19SLit![1, 100, 1, 100, 1, 100];
    assert_eq!(ArraySeqMtEphS::select(&a, 0), Some(1));
    assert_eq!(ArraySeqMtEphS::select(&a, 2), Some(1));
    assert_eq!(ArraySeqMtEphS::select(&a, 3), Some(100));
    assert_eq!(ArraySeqMtEphS::select(&a, 5), Some(100));
}

#[test]
fn test_select_median() {
    let a = ArraySeqMtEphChap19SLit![7, 2, 9, 4, 5, 1, 8, 3, 6];
    assert_eq!(ArraySeqMtEphS::select(&a, 4), Some(5));
}

#[test]
fn test_concurrent_select() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(ArraySeqMtEphS::tabulate(&|i| (100 - i as i32), 100));
    let mut handles = vec![];

    for k in [0, 25, 49, 50, 75, 99] {
        let d = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            ArraySeqMtEphS::select(&d, k)
        }));
    }

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    assert_eq!(results[0], Some(1));
    assert_eq!(results[1], Some(26));
    assert_eq!(results[2], Some(50));
    assert_eq!(results[3], Some(51));
    assert_eq!(results[4], Some(76));
    assert_eq!(results[5], Some(100));
}
