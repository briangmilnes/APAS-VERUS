// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Runtime tests for Chap18 ArraySeqMtEphSlice.

use apas_verus::Chap18::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
use vstd::prelude::Ghost;

#[test]
fn test_empty() {
    let seq = ArraySeqMtEphSliceS::<i32>::empty();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_singleton() {
    let seq = ArraySeqMtEphSliceS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth_cloned(0), 42);
}

#[test]
fn test_from_vec() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(2), 30);
}

#[test]
fn test_new() {
    let seq = ArraySeqMtEphSliceS::<i32>::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 42);
    assert_eq!(seq.nth_cloned(4), 42);
}

#[test]
fn test_slice_basic() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    assert_eq!(sliced.length(), 3);
    assert_eq!(sliced.nth_cloned(0), 20);
    assert_eq!(sliced.nth_cloned(1), 30);
    assert_eq!(sliced.nth_cloned(2), 40);
}

#[test]
fn test_slice_full() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let sliced = seq.slice(0, 3);
    assert_eq!(sliced.length(), 3);
    assert_eq!(sliced.nth_cloned(0), 1);
    assert_eq!(sliced.nth_cloned(2), 3);
}

#[test]
fn test_slice_empty() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let sliced = seq.slice(1, 0);
    assert_eq!(sliced.length(), 0);
}

#[test]
fn test_to_vec() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let v = seq.to_vec();
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_iter() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    let mut it = seq.iter();
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.next(), Some(&20));
    assert_eq!(it.next(), Some(&30));
    assert_eq!(it.next(), None);
}

#[test]
fn test_iter_for_loop() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4, 5]);
    let mut sum = 0;
    for v in &seq {
        sum += v;
    }
    assert_eq!(sum, 15);
}

#[test]
fn test_large_sequence() {
    let data: Vec<usize> = (0..200).collect();
    let seq = ArraySeqMtEphSliceS::from_vec(data);
    assert_eq!(seq.length(), 200);
    assert_eq!(seq.nth_cloned(0), 0);
    assert_eq!(seq.nth_cloned(199), 199);
}

#[test]
fn test_slice_then_iter() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    let collected: Vec<i32> = sliced.iter().map(|x| *x).collect();
    assert_eq!(collected, vec![20, 30, 40]);
}

#[test]
fn test_display() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}

// is_empty / is_singleton tests

#[test]
fn test_is_empty_true() {
    let seq = ArraySeqMtEphSliceS::<i32>::empty();
    assert!(seq.is_empty());
}

#[test]
fn test_is_empty_false() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1]);
    assert!(!seq.is_empty());
}

#[test]
fn test_is_singleton_true() {
    let seq = ArraySeqMtEphSliceS::singleton(42);
    assert!(seq.is_singleton());
}

#[test]
fn test_is_singleton_false_empty() {
    let seq = ArraySeqMtEphSliceS::<i32>::empty();
    assert!(!seq.is_singleton());
}

#[test]
fn test_is_singleton_false_many() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    assert!(!seq.is_singleton());
}

// append tests

#[test]
fn test_append_basic() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![4i64, 5]);
    let appended = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(appended.length(), 5);
    assert_eq!(appended.nth_cloned(0), 1);
    assert_eq!(appended.nth_cloned(2), 3);
    assert_eq!(appended.nth_cloned(3), 4);
    assert_eq!(appended.nth_cloned(4), 5);
}

#[test]
fn test_append_empty_left() {
    let a = ArraySeqMtEphSliceS::<i64>::empty();
    let b = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2]);
    let appended = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(appended.length(), 2);
    assert_eq!(appended.nth_cloned(0), 1);
}

#[test]
fn test_append_empty_right() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2]);
    let b = ArraySeqMtEphSliceS::<i64>::empty();
    let appended = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(appended.length(), 2);
    assert_eq!(appended.nth_cloned(1), 2);
}

// update tests

#[test]
fn test_update_basic() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30]);
    let updated = seq.update(1, 99);
    assert_eq!(updated.length(), 3);
    assert_eq!(updated.nth_cloned(0), 10);
    assert_eq!(updated.nth_cloned(1), 99);
    assert_eq!(updated.nth_cloned(2), 30);
}

#[test]
fn test_update_first() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3]);
    let updated = seq.update(0, 100);
    assert_eq!(updated.nth_cloned(0), 100);
    assert_eq!(updated.nth_cloned(1), 2);
}

#[test]
fn test_update_last() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3]);
    let updated = seq.update(2, 100);
    assert_eq!(updated.nth_cloned(2), 100);
    assert_eq!(updated.nth_cloned(0), 1);
}

// inject tests

#[test]
fn test_inject_basic() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30, 40, 50]);
    let updates = vec![(1, 99i64), (3, 77)];
    let injected = seq.inject(&updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected.nth_cloned(0), 10);
    assert_eq!(injected.nth_cloned(1), 99);
    assert_eq!(injected.nth_cloned(2), 30);
    assert_eq!(injected.nth_cloned(3), 77);
    assert_eq!(injected.nth_cloned(4), 50);
}

#[test]
fn test_inject_empty_updates() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3]);
    let updates: Vec<(usize, i64)> = vec![];
    let injected = seq.inject(&updates);
    assert_eq!(injected.length(), 3);
    assert_eq!(injected.nth_cloned(0), 1);
    assert_eq!(injected.nth_cloned(1), 2);
    assert_eq!(injected.nth_cloned(2), 3);
}

// ninject tests

#[test]
fn test_ninject_basic() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30]);
    let updates = vec![(0, 99i64)];
    let injected = seq.ninject(&updates);
    assert_eq!(injected.length(), 3);
    // Position 0 should be either 10 or 99.
    let v0 = injected.nth_cloned(0);
    assert!(v0 == 10 || v0 == 99);
}

// reduce tests

#[test]
fn test_reduce_empty() {
    let seq = ArraySeqMtEphSliceS::<i64>::empty();
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 0);
}

#[test]
fn test_reduce_small() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 15);
}

#[test]
fn test_reduce_large() {
    let n = 10000i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

// map tests

#[test]
fn test_map_small() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let mapped = seq.map(&|x: &i64| -> i64 { *x + 10 });
    assert_eq!(mapped.length(), 5);
    for i in 0..5 {
        assert_eq!(mapped.nth_cloned(i), (i as i64) + 11);
    }
}

// filter tests

#[test]
fn test_filter_some_pass() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5, 6, 7, 8]);
    let filtered = seq.filter(&|x: &i64| -> bool { *x % 2 == 0 }, Ghost::assume_new());
    assert_eq!(filtered.length(), 4);
}

// tabulate tests

#[test]
fn test_tabulate_small() {
    let tab = ArraySeqMtEphSliceS::tabulate(&|i: usize| -> i64 { (i as i64) * (i as i64) }, 5);
    assert_eq!(tab.length(), 5);
    assert_eq!(tab.nth_cloned(0), 0);
    assert_eq!(tab.nth_cloned(1), 1);
    assert_eq!(tab.nth_cloned(2), 4);
    assert_eq!(tab.nth_cloned(3), 9);
    assert_eq!(tab.nth_cloned(4), 16);
}

// scan tests

#[test]
fn test_scan_small() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4]);
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), 4);
    assert_eq!(prefixes.nth_cloned(0), 1);
    assert_eq!(prefixes.nth_cloned(1), 3);
    assert_eq!(prefixes.nth_cloned(2), 6);
    assert_eq!(prefixes.nth_cloned(3), 10);
    assert_eq!(total, 10);
}

// flatten tests

#[test]
fn test_flatten_multiple() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![3i64]);
    let c = ArraySeqMtEphSliceS::from_vec(vec![4i64, 5, 6]);
    let outer = ArraySeqMtEphSliceS::from_vec(vec![a, b, c]);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 6);
}

// combined operation tests

#[test]
fn test_slice_then_reduce() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    let sum = sliced.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 90);
}
