//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer with parallel operations.

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use vstd::prelude::Ghost;

#[test]
fn test_basic_operations() {
    let seq = ArraySeqMtPerS::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(4), 5);
}

#[test]
fn test_subseq_copy() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sub = seq.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_map_sequential() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4]);
    let doubled = ArraySeqMtPerS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_reduce_sequential() {
    let seq = ArraySeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum = ArraySeqMtPerS::reduce(&seq, &|a, b| a + b, Ghost::assume_new(), 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_map_par() {
    let seq = ArraySeqMtPerS::from_vec((0..64).collect());
    let doubled = ArraySeqMtPerS::map_par(&seq, |x| x * 2);
    assert_eq!(doubled.length(), 64);
    assert_eq!(*doubled.nth(0), 0);
    assert_eq!(*doubled.nth(32), 64);
    assert_eq!(*doubled.nth(63), 126);
}

#[test]
fn test_reduce_par() {
    let seq = ArraySeqMtPerS::from_vec((1..=64).collect());
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0);
    assert_eq!(sum, 64 * 65 / 2);
}

#[test]
fn test_reduce_par_medium() {
    let n = 256i64;
    let seq = ArraySeqMtPerS::from_vec((1..=n).collect());
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_map_par_medium() {
    let seq = ArraySeqMtPerS::from_vec((0..128).collect());
    let squared = ArraySeqMtPerS::map_par(&seq, |x| x * x);
    assert_eq!(squared.length(), 128);
    assert_eq!(*squared.nth(10), 100);
    assert_eq!(*squared.nth(11), 121);
}

#[test]
fn test_filter_par() {
    let seq = ArraySeqMtPerS::from_vec((0..64).collect());
    let evens = ArraySeqMtPerS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 32);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(1), 2);
    assert_eq!(*evens.nth(31), 62);
}

#[test]
fn test_filter_par_medium() {
    let seq = ArraySeqMtPerS::from_vec((0..256).collect());
    let multiples_of_7 = ArraySeqMtPerS::filter_par(&seq, |&x| x % 7 == 0);
    assert_eq!(multiples_of_7.length(), 37);
}

// D&C parallel RTTs — empty, singleton, small, medium, large.

#[test]
fn test_map_par_empty() {
    let seq = ArraySeqMtPerS::<i64>::from_vec(vec![]);
    let mapped = ArraySeqMtPerS::map_par(&seq, |x| x * 2);
    assert_eq!(mapped.length(), 0);
}

#[test]
fn test_map_par_singleton() {
    let seq = ArraySeqMtPerS::from_vec(vec![7i64]);
    let mapped = ArraySeqMtPerS::map_par(&seq, |x| x * 3);
    assert_eq!(mapped.length(), 1);
    assert_eq!(*mapped.nth(0), 21);
}

#[test]
fn test_map_par_small() {
    let seq = ArraySeqMtPerS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let mapped = ArraySeqMtPerS::map_par(&seq, |x| x + 10);
    assert_eq!(mapped.length(), 5);
    for i in 0..5 {
        assert_eq!(*mapped.nth(i), (i as i64) + 11);
    }
}

#[test]
fn test_map_par_large() {
    let n = 10000usize;
    let seq = ArraySeqMtPerS::from_vec((0..n as i64).collect());
    let mapped = ArraySeqMtPerS::map_par(&seq, |x| x * 2);
    assert_eq!(mapped.length(), n);
    assert_eq!(*mapped.nth(0), 0);
    assert_eq!(*mapped.nth(n - 1), (n as i64 - 1) * 2);
    assert_eq!(*mapped.nth(5000), 10000);
}

#[test]
fn test_map_par_odd_size() {
    let seq = ArraySeqMtPerS::from_vec((0..77i64).collect());
    let mapped = ArraySeqMtPerS::map_par(&seq, |x| x + 1);
    assert_eq!(mapped.length(), 77);
    assert_eq!(*mapped.nth(76), 77);
}

#[test]
fn test_reduce_par_empty() {
    let seq = ArraySeqMtPerS::<i64>::from_vec(vec![]);
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 0);
}

#[test]
fn test_reduce_par_singleton() {
    let seq = ArraySeqMtPerS::from_vec(vec![42i64]);
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 42);
}

#[test]
fn test_reduce_par_small() {
    let seq = ArraySeqMtPerS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, 15);
}

#[test]
fn test_reduce_par_large() {
    let n = 10000i64;
    let seq = ArraySeqMtPerS::from_vec((1..=n).collect());
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_reduce_par_odd_size() {
    let n = 77i64;
    let seq = ArraySeqMtPerS::from_vec((1..=n).collect());
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, Ghost::assume_new(), 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_filter_par_empty() {
    let seq = ArraySeqMtPerS::<i64>::from_vec(vec![]);
    let filtered = ArraySeqMtPerS::filter_par(&seq, |_x| true);
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_par_singleton_keep() {
    let seq = ArraySeqMtPerS::from_vec(vec![4i64]);
    let filtered = ArraySeqMtPerS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(filtered.length(), 1);
    assert_eq!(*filtered.nth(0), 4);
}

#[test]
fn test_filter_par_singleton_drop() {
    let seq = ArraySeqMtPerS::from_vec(vec![3i64]);
    let filtered = ArraySeqMtPerS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(filtered.length(), 0);
}

#[test]
fn test_filter_par_small() {
    let seq = ArraySeqMtPerS::from_vec(vec![1i64, 2, 3, 4, 5]);
    let evens = ArraySeqMtPerS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 2);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
}

#[test]
fn test_filter_par_large() {
    let n = 10000usize;
    let seq = ArraySeqMtPerS::from_vec((0..n as i64).collect());
    let evens = ArraySeqMtPerS::filter_par(&seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), n / 2);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(evens.length() - 1), (n as i64) - 2);
}

#[test]
fn test_filter_par_all_pass() {
    let seq = ArraySeqMtPerS::from_vec((0..100i64).collect());
    let all = ArraySeqMtPerS::filter_par(&seq, |_x| true);
    assert_eq!(all.length(), 100);
}

#[test]
fn test_filter_par_none_pass() {
    let seq = ArraySeqMtPerS::from_vec((0..100i64).collect());
    let none = ArraySeqMtPerS::filter_par(&seq, |_x| false);
    assert_eq!(none.length(), 0);
}
