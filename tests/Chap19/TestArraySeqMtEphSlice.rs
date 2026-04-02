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
