// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Runtime tests for Chap19 ArraySeqMtEphSlice.

use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
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
fn test_subseq_copy() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30, 40, 50]);
    let sub = seq.subseq_copy(2, 2);
    assert_eq!(sub.length(), 2);
    assert_eq!(sub.nth_cloned(0), 30);
    assert_eq!(sub.nth_cloned(1), 40);
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
fn test_is_singleton_false_multi() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    assert!(!seq.is_singleton());
}

// set tests

#[test]
fn test_set_first() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    let updated = seq.set(0, 99);
    assert_eq!(updated.nth_cloned(0), 99);
    assert_eq!(updated.nth_cloned(1), 20);
    assert_eq!(updated.nth_cloned(2), 30);
}

#[test]
fn test_set_last() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    let updated = seq.set(2, 99);
    assert_eq!(updated.nth_cloned(0), 10);
    assert_eq!(updated.nth_cloned(1), 20);
    assert_eq!(updated.nth_cloned(2), 99);
}

#[test]
fn test_set_preserves_length() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4, 5]);
    let updated = seq.set(3, 0);
    assert_eq!(updated.length(), 5);
}

// append tests

#[test]
fn test_append_basic() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![4, 5]);
    let ab = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(ab.length(), 5);
    assert_eq!(ab.nth_cloned(0), 1);
    assert_eq!(ab.nth_cloned(2), 3);
    assert_eq!(ab.nth_cloned(3), 4);
    assert_eq!(ab.nth_cloned(4), 5);
}

#[test]
fn test_append_empty_left() {
    let a = ArraySeqMtEphSliceS::<i32>::empty();
    let b = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    let ab = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(ab.length(), 2);
    assert_eq!(ab.nth_cloned(0), 1);
}

#[test]
fn test_append_empty_right() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2]);
    let b = ArraySeqMtEphSliceS::<i32>::empty();
    let ab = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(ab.length(), 2);
    assert_eq!(ab.nth_cloned(1), 2);
}

#[test]
fn test_append_both_empty() {
    let a = ArraySeqMtEphSliceS::<i32>::empty();
    let b = ArraySeqMtEphSliceS::<i32>::empty();
    let ab = ArraySeqMtEphSliceS::append(&a, &b);
    assert_eq!(ab.length(), 0);
}

// update tests

#[test]
fn test_update_basic() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30]);
    let updated = ArraySeqMtEphSliceS::update(&a, 1, 99);
    assert_eq!(updated.length(), 3);
    assert_eq!(updated.nth_cloned(0), 10);
    assert_eq!(updated.nth_cloned(1), 99);
    assert_eq!(updated.nth_cloned(2), 30);
}

// inject tests

#[test]
fn test_inject_empty_updates() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let updates: Vec<(usize, i32)> = vec![];
    let injected = ArraySeqMtEphSliceS::inject(&a, &updates);
    assert_eq!(injected.length(), 3);
    assert_eq!(injected.nth_cloned(0), 1);
    assert_eq!(injected.nth_cloned(1), 2);
    assert_eq!(injected.nth_cloned(2), 3);
}

#[test]
fn test_inject_single_update() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![10, 20, 30, 40]);
    let updates = vec![(2, 99)];
    let injected = ArraySeqMtEphSliceS::inject(&a, &updates);
    assert_eq!(injected.length(), 4);
    assert_eq!(injected.nth_cloned(0), 10);
    assert_eq!(injected.nth_cloned(2), 99);
    assert_eq!(injected.nth_cloned(3), 40);
}

#[test]
fn test_inject_multiple_updates() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = vec![(0, 10), (4, 50)];
    let injected = ArraySeqMtEphSliceS::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected.nth_cloned(0), 10);
    assert_eq!(injected.nth_cloned(1), 2);
    assert_eq!(injected.nth_cloned(4), 50);
}

#[test]
fn test_inject_out_of_bounds_ignored() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3]);
    let updates = vec![(10, 99)];
    let injected = ArraySeqMtEphSliceS::inject(&a, &updates);
    assert_eq!(injected.length(), 3);
    assert_eq!(injected.nth_cloned(0), 1);
}

// ninject tests

#[test]
fn test_ninject_basic() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1, 2, 3, 4]);
    let updates = vec![(1, 20), (3, 40)];
    let injected = ArraySeqMtEphSliceS::ninject(&a, &updates);
    assert_eq!(injected.length(), 4);
    assert_eq!(injected.nth_cloned(0), 1);
    assert_eq!(injected.nth_cloned(2), 3);
}

#[test]
fn test_ninject_empty_updates() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![5, 6, 7]);
    let updates: Vec<(usize, i32)> = vec![];
    let injected = ArraySeqMtEphSliceS::ninject(&a, &updates);
    assert_eq!(injected.length(), 3);
    assert_eq!(injected.nth_cloned(0), 5);
    assert_eq!(injected.nth_cloned(1), 6);
    assert_eq!(injected.nth_cloned(2), 7);
}

// reduce tests

#[test]
fn test_reduce_empty() {
    let seq = ArraySeqMtEphSliceS::<i64>::empty();
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 0);
}

#[test]
fn test_reduce_singleton() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![42i64]);
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 42);
}

#[test]
fn test_reduce_small() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 15);
}

#[test]
fn test_reduce_medium() {
    let n = 100i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_reduce_large() {
    let n = 10000i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let sum = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_reduce_product() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let prod = seq.reduce(&|a: &i64, b: &i64| -> i64 { *a * *b }, Ghost::assume_new(), 1i64);
    assert_eq!(prod, 120);
}

// map tests

#[test]
fn test_map_empty() {
    let seq = ArraySeqMtEphSliceS::<i64>::empty();
    let mapped = seq.map(&|x: &i64| -> i64 { *x * 2 });
    assert_eq!(mapped.length(), 0);
}

#[test]
fn test_map_singleton() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![7i64]);
    let mapped = seq.map(&|x: &i64| -> i64 { *x * 3 });
    assert_eq!(mapped.length(), 1);
    assert_eq!(mapped.nth_cloned(0), 21);
}

#[test]
fn test_map_small() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let mapped = seq.map(&|x: &i64| -> i64 { *x + 10 });
    assert_eq!(mapped.length(), 5);
    for i in 0..5 {
        assert_eq!(mapped.nth_cloned(i), (i as i64) + 11);
    }
}

#[test]
fn test_map_large() {
    let n = 10000usize;
    let seq = ArraySeqMtEphSliceS::from_vec((0..n as i64).collect());
    let mapped = seq.map(&|x: &i64| -> i64 { *x * 2 });
    assert_eq!(mapped.length(), n);
    assert_eq!(mapped.nth_cloned(0), 0);
    assert_eq!(mapped.nth_cloned(n - 1), (n as i64 - 1) * 2);
}

// filter tests

#[test]
fn test_filter_empty() {
    let seq = ArraySeqMtEphSliceS::<i64>::empty();
    let filtered = seq.filter(&|x: &i64| -> bool { *x > 0 }, Ghost::assume_new());
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_all_pass() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let filtered = seq.filter(&|x: &i64| -> bool { *x > 0 }, Ghost::assume_new());
    assert_eq!(filtered.length(), 5);
}

#[test]
fn test_filter_none_pass() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let filtered = seq.filter(&|x: &i64| -> bool { *x > 100 }, Ghost::assume_new());
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_some_pass() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5, 6, 7, 8]);
    let filtered = seq.filter(&|x: &i64| -> bool { *x % 2 == 0 }, Ghost::assume_new());
    assert_eq!(filtered.length(), 4);
}

#[test]
fn test_filter_large() {
    let n = 10000i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let filtered = seq.filter(&|x: &i64| -> bool { *x % 3 == 0 }, Ghost::assume_new());
    assert_eq!(filtered.length(), (n / 3) as usize);
}

// tabulate tests

#[test]
fn test_tabulate_empty() {
    let tab = ArraySeqMtEphSliceS::<i64>::tabulate(&|i: usize| -> i64 { i as i64 }, 0);
    assert_eq!(tab.length(), 0);
}

#[test]
fn test_tabulate_singleton() {
    let tab = ArraySeqMtEphSliceS::tabulate(&|i: usize| -> i64 { (i as i64) * 10 }, 1);
    assert_eq!(tab.length(), 1);
    assert_eq!(tab.nth_cloned(0), 0);
}

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

#[test]
fn test_tabulate_identity() {
    let n = 100;
    let tab = ArraySeqMtEphSliceS::tabulate(&|i: usize| -> usize { i }, n);
    assert_eq!(tab.length(), n);
    for i in 0..n {
        assert_eq!(tab.nth_cloned(i), i);
    }
}

#[test]
fn test_tabulate_large() {
    let n = 10000;
    let tab = ArraySeqMtEphSliceS::tabulate(&|i: usize| -> i64 { (i as i64) + 1 }, n);
    assert_eq!(tab.length(), n);
    assert_eq!(tab.nth_cloned(0), 1);
    assert_eq!(tab.nth_cloned(n - 1), n as i64);
    let sum = tab.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, (n as i64) * (n as i64 + 1) / 2);
}

#[test]
fn test_tabulate_then_map() {
    let tab = ArraySeqMtEphSliceS::tabulate(&|i: usize| -> i64 { i as i64 }, 10);
    let doubled = tab.map(&|x: &i64| -> i64 { *x * 2 });
    assert_eq!(doubled.length(), 10);
    assert_eq!(doubled.nth_cloned(0), 0);
    assert_eq!(doubled.nth_cloned(5), 10);
    assert_eq!(doubled.nth_cloned(9), 18);
}

// combined operation tests

#[test]
fn test_map_then_reduce() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let doubled = seq.map(&|x: &i64| -> i64 { *x * 2 });
    let sum = doubled.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 30);
}

#[test]
fn test_filter_then_reduce() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let evens = seq.filter(&|x: &i64| -> bool { *x % 2 == 0 }, Ghost::assume_new());
    let sum = evens.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 30);
}

#[test]
fn test_slice_then_reduce() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    let sum = sliced.reduce(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 90);
}

// flatten tests

#[test]
fn test_flatten_empty() {
    let outer: ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<i64>> =
        ArraySeqMtEphSliceS::from_vec(vec![]);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 0);
}

#[test]
fn test_flatten_single() {
    let inner = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3]);
    let outer = ArraySeqMtEphSliceS::from_vec(vec![inner]);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 3);
}

#[test]
fn test_flatten_multiple() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2]);
    let b = ArraySeqMtEphSliceS::from_vec(vec![3i64]);
    let c = ArraySeqMtEphSliceS::from_vec(vec![4i64, 5, 6]);
    let outer = ArraySeqMtEphSliceS::from_vec(vec![a, b, c]);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 6);
}

#[test]
fn test_flatten_with_empties() {
    let a = ArraySeqMtEphSliceS::from_vec(vec![1i64]);
    let b: ArraySeqMtEphSliceS<i64> = ArraySeqMtEphSliceS::from_vec(vec![]);
    let c = ArraySeqMtEphSliceS::from_vec(vec![2i64, 3]);
    let outer = ArraySeqMtEphSliceS::from_vec(vec![a, b, c]);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 3);
}

#[test]
fn test_flatten_large() {
    let mut seqs: Vec<ArraySeqMtEphSliceS<i64>> = Vec::new();
    for i in 0..20 {
        seqs.push(ArraySeqMtEphSliceS::from_vec(vec![i as i64; 10]));
    }
    let outer = ArraySeqMtEphSliceS::from_vec(seqs);
    let flat = flatten(&outer);
    assert_eq!(flat.length(), 200);
}

// scan tests

#[test]
fn test_scan_empty() {
    let seq = ArraySeqMtEphSliceS::<i64>::empty();
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), 0);
    assert_eq!(total, 0);
}

#[test]
fn test_scan_singleton() {
    let seq = ArraySeqMtEphSliceS::from_vec(vec![42i64]);
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), 1);
    assert_eq!(prefixes.nth_cloned(0), 42);
    assert_eq!(total, 42);
}

#[test]
fn test_scan_small() {
    // Inclusive scan with +: [1,2,3,4] -> [1,3,6,10], total=10
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4]);
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), 4);
    assert_eq!(prefixes.nth_cloned(0), 1);
    assert_eq!(prefixes.nth_cloned(1), 3);
    assert_eq!(prefixes.nth_cloned(2), 6);
    assert_eq!(prefixes.nth_cloned(3), 10);
    assert_eq!(total, 10);
}

#[test]
fn test_scan_medium() {
    let n = 100i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), n as usize);
    assert_eq!(total, n * (n + 1) / 2);
    // First few prefixes: 1, 3, 6, 10, 15
    assert_eq!(prefixes.nth_cloned(0), 1);
    assert_eq!(prefixes.nth_cloned(1), 3);
    assert_eq!(prefixes.nth_cloned(2), 6);
    assert_eq!(prefixes.nth_cloned(3), 10);
    assert_eq!(prefixes.nth_cloned(4), 15);
    // Last prefix == total
    assert_eq!(prefixes.nth_cloned(n as usize - 1), total);
}

#[test]
fn test_scan_large() {
    let n = 10000i64;
    let seq = ArraySeqMtEphSliceS::from_vec((1..=n).collect());
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), n as usize);
    assert_eq!(total, n * (n + 1) / 2);
    assert_eq!(prefixes.nth_cloned(n as usize - 1), total);
}

#[test]
fn test_scan_product() {
    // Inclusive scan with *: [1,2,3,4,5] -> [1,2,6,24,120], total=120
    let seq = ArraySeqMtEphSliceS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let (prefixes, total) = seq.scan(&|a: &i64, b: &i64| -> i64 { *a * *b }, Ghost::assume_new(), 1i64);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(prefixes.nth_cloned(0), 1);
    assert_eq!(prefixes.nth_cloned(1), 2);
    assert_eq!(prefixes.nth_cloned(2), 6);
    assert_eq!(prefixes.nth_cloned(3), 24);
    assert_eq!(prefixes.nth_cloned(4), 120);
    assert_eq!(total, 120);
}

#[test]
fn test_scan_sliced() {
    // Scan on a slice of a larger sequence.
    let seq = ArraySeqMtEphSliceS::from_vec(vec![10i64, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3); // [20, 30, 40]
    let (prefixes, total) = sliced.scan(&|a: &i64, b: &i64| -> i64 { *a + *b }, Ghost::assume_new(), 0i64);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(prefixes.nth_cloned(0), 20);
    assert_eq!(prefixes.nth_cloned(1), 50);
    assert_eq!(prefixes.nth_cloned(2), 90);
    assert_eq!(total, 90);
}
