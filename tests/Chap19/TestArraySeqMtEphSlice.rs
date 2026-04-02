//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
