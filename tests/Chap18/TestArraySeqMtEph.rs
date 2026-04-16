// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for ArraySeqMtEph with parallel operations.

use apas_verus::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use vstd::prelude::Ghost;

#[test]
fn test_basic_operations() {
    let seq = ArraySeqMtEphS::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(4), 5);
}

#[test]
fn test_set_mutation() {
    let mut seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let _ = seq.set(2, 99);
    assert_eq!(*seq.nth(2), 99);
}

#[test]
fn test_map_sequential() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4]);
    let doubled = ArraySeqMtEphS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_reduce_sequential() {
    let seq = ArraySeqMtEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum = ArraySeqMtEphS::reduce(&seq, &|a, b| a + b, Ghost::assume_new(), 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_map_par() {
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let doubled = ArraySeqMtEphS::map_par(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 64);
    assert_eq!(*doubled.nth(0), 0);
    assert_eq!(*doubled.nth(32), 64);
    assert_eq!(*doubled.nth(63), 126);
}

#[test]
fn test_reduce_par() {
    let seq = ArraySeqMtEphS::from_vec((1..=64).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0);
    assert_eq!(sum, 64 * 65 / 2);
}

#[test]
fn test_reduce_par_medium() {
    let n = 256i64;
    let seq = ArraySeqMtEphS::from_vec((1..=n).collect());
    let sum = ArraySeqMtEphS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_map_par_medium() {
    let seq = ArraySeqMtEphS::from_vec((0..128).collect());
    let squared = ArraySeqMtEphS::map_par(&seq, |x| x * x);
    assert_eq!(squared.length(), 128);
    assert_eq!(*squared.nth(10), 100);
    assert_eq!(*squared.nth(11), 121);
}

#[test]
fn test_filter_par() {
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let evens = ArraySeqMtEphS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 32);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(1), 2);
    assert_eq!(*evens.nth(31), 62);
}

#[test]
fn test_filter_par_medium() {
    let seq = ArraySeqMtEphS::from_vec((0..256).collect());
    let multiples_of_7 = ArraySeqMtEphS::filter_par(&seq, |&x| x % 7 == 0);
    assert_eq!(multiples_of_7.length(), 37);
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
