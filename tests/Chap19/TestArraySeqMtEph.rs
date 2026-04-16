// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Runtime tests for Chap19 ArraySeqMtEph (multi-threaded ephemeral parametric array sequence).

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use vstd::prelude::Ghost;

#[test]
fn test_new() {
    let seq = ArraySeqMtEphS::<i32>::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 42);
    assert_eq!(*seq.nth(4), 42);
}

#[test]
fn test_empty() {
    let seq = ArraySeqMtEphS::<i32>::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.is_empty());
}

#[test]
fn test_singleton() {
    let seq = ArraySeqMtEphS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
    assert!(seq.is_singleton());
}

#[test]
fn test_from_vec() {
    let seq = ArraySeqMtEphS::from_vec(vec![10, 20, 30]);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_set() {
    let mut seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let _ = seq.set(1, 99);
    assert_eq!(*seq.nth(1), 99);
}

#[test]
fn test_append() {
    let a = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqMtEphS::from_vec(vec![4, 5]);
    let c = ArraySeqMtEphS::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(3), 4);
}

#[test]
fn test_subseq() {
    let seq = ArraySeqMtEphS::from_vec(vec![10, 20, 30, 40, 50]);
    let sub = ArraySeqMtEphS::subseq(&seq, 1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 20);
}

#[test]
fn test_update() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let updated = ArraySeqMtEphS::update(&seq, 1, 99);
    assert_eq!(*updated.nth(1), 99);
}

#[test]
fn test_map() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4]);
    let doubled = ArraySeqMtEphS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_map_par() {
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let doubled = ArraySeqMtEphS::map_par(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 64);
    assert_eq!(*doubled.nth(0), 0);
    assert_eq!(*doubled.nth(32), 64);
}

#[test]
fn test_reduce_par() {
    let seq = ArraySeqMtEphS::from_vec((1..=64).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0);
    assert_eq!(sum, 64 * 65 / 2);
}

#[test]
fn test_filter_par() {
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let evens = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 32);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(1), 2);
}

#[test]
fn test_tabulate() {
    let seq = ArraySeqMtEphS::tabulate(&|i| i * 3, 4);
    assert_eq!(seq.length(), 4);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(3), 9);
}

#[test]
fn test_partial_eq() {
    let a = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let c = ArraySeqMtEphS::from_vec(vec![1, 2, 4]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_clone() {
    let a = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_display() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_iter() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3]);
    let mut it = seq.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}

#[test]
fn test_is_empty_singleton() {
    let empty = ArraySeqMtEphS::<i32>::empty();
    assert!(empty.is_empty());
    assert!(!empty.is_singleton());

    let single = ArraySeqMtEphS::singleton(42);
    assert!(!single.is_empty());
    assert!(single.is_singleton());
}

#[test]
fn test_inject() {
    let seq = ArraySeqMtEphS::from_vec(vec![10, 20, 30, 40, 50]);
    let updates = vec![(1, 99), (3, 77)];
    let injected = ArraySeqMtEphS::inject(&seq, &updates);
    assert_eq!(*injected.nth(1), 99);
    assert_eq!(*injected.nth(3), 77);
}

#[test]
fn test_large_sequence() {
    let data: Vec<usize> = (0..500).collect();
    let seq = ArraySeqMtEphS::from_vec(data);
    assert_eq!(seq.length(), 500);
    assert_eq!(*seq.nth(499), 499);
}

#[test]
fn test_for_loop() {
    let seq = ArraySeqMtEphS::from_vec(vec![10, 20, 30]);
    let mut sum = 0;
    for v in &seq {
        sum += v;
    }
    assert_eq!(sum, 60);
}

#[test]
fn test_flatten() {
    let inner1 = ArraySeqMtEphS::from_vec(vec![1, 2]);
    let inner2 = ArraySeqMtEphS::from_vec(vec![3, 4, 5]);
    let outer = ArraySeqMtEphS::from_vec(vec![inner1, inner2]);
    let flat = ArraySeqMtEphS::flatten(&outer);
    assert_eq!(flat.length(), 5);
    assert_eq!(*flat.nth(0), 1);
    assert_eq!(*flat.nth(4), 5);
}

// D&C parallel RTTs — empty, singleton, small, medium, large.

#[test]
fn test_map_par_empty() {
    let seq = ArraySeqMtEphS::<i64>::from_vec(vec![]);
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x * 2);
    assert_eq!(mapped.length(), 0);
}

#[test]
fn test_map_par_singleton() {
    let seq = ArraySeqMtEphS::from_vec(vec![7i64]);
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x * 3);
    assert_eq!(mapped.length(), 1);
    assert_eq!(*mapped.nth(0), 21);
}

#[test]
fn test_map_par_small() {
    let seq = ArraySeqMtEphS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x + 10);
    assert_eq!(mapped.length(), 5);
    for i in 0..5 {
        assert_eq!(*mapped.nth(i), (i as i64) + 11);
    }
}

#[test]
fn test_map_par_medium() {
    let seq = ArraySeqMtEphS::from_vec((0..100i64).collect());
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x * 2);
    assert_eq!(mapped.length(), 100);
    for i in 0..100 {
        assert_eq!(*mapped.nth(i), (i as i64) * 2);
    }
}

#[test]
fn test_map_par_large() {
    let n = 10000usize;
    let seq = ArraySeqMtEphS::from_vec((0..n as i64).collect());
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x * 2);
    assert_eq!(mapped.length(), n);
    assert_eq!(*mapped.nth(0), 0);
    assert_eq!(*mapped.nth(n - 1), (n as i64 - 1) * 2);
    assert_eq!(*mapped.nth(5000), 10000);
}

#[test]
fn test_map_par_odd_size() {
    let seq = ArraySeqMtEphS::from_vec((0..77i64).collect());
    let mapped = ArraySeqMtEphS::map_par(&seq, |x| x + 1);
    assert_eq!(mapped.length(), 77);
    assert_eq!(*mapped.nth(76), 77);
}

#[test]
fn test_reduce_par_empty() {
    let seq = ArraySeqMtEphS::<i64>::from_vec(vec![]);
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 0);
}

#[test]
fn test_reduce_par_singleton() {
    let seq = ArraySeqMtEphS::from_vec(vec![42i64]);
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 42);
}

#[test]
fn test_reduce_par_small() {
    let seq = ArraySeqMtEphS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 15);
}

#[test]
fn test_reduce_par_medium() {
    let n = 100i64;
    let seq = ArraySeqMtEphS::from_vec((1..=n).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_reduce_par_large() {
    let n = 10000i64;
    let seq = ArraySeqMtEphS::from_vec((1..=n).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_reduce_par_odd_size() {
    let n = 77i64;
    let seq = ArraySeqMtEphS::from_vec((1..=n).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_filter_par_empty() {
    let seq = ArraySeqMtEphS::<i64>::from_vec(vec![]);
    let filtered = ArraySeqMtEphS::filter_par(&seq, |_x| true);
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_par_singleton_keep() {
    let seq = ArraySeqMtEphS::from_vec(vec![4i64]);
    let filtered = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(filtered.length(), 1);
    assert_eq!(*filtered.nth(0), 4);
}

#[test]
fn test_filter_par_singleton_drop() {
    let seq = ArraySeqMtEphS::from_vec(vec![3i64]);
    let filtered = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_par_small() {
    let seq = ArraySeqMtEphS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let evens = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 2);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
}

#[test]
fn test_filter_par_medium() {
    let seq = ArraySeqMtEphS::from_vec((0..100i64).collect());
    let evens = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 50);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(49), 98);
}

#[test]
fn test_filter_par_large() {
    let n = 10000usize;
    let seq = ArraySeqMtEphS::from_vec((0..n as i64).collect());
    let evens = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), n / 2);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(evens.length() - 1), (n as i64) - 2);
}

#[test]
fn test_filter_par_all_pass() {
    let seq = ArraySeqMtEphS::from_vec((0..100i64).collect());
    let all = ArraySeqMtEphS::filter_par(&seq, |_x| true);
    assert_eq!(all.length(), 100);
}

#[test]
fn test_filter_par_none_pass() {
    let seq = ArraySeqMtEphS::from_vec((0..100i64).collect());
    let none = ArraySeqMtEphS::filter_par(&seq, |_x| false);
    assert_eq!(none.length(), 0);
}
