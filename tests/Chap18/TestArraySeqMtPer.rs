//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer with parallel operations.

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::Pool;

#[test]
fn test_basic_operations() {
    let seq = ArraySeqMtPerS::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(4), 5);
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
    let sum = ArraySeqMtPerS::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_map_par() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtPerS::from_vec((0..10000).collect());
    let doubled = ArraySeqMtPerS::map_par(&pool, &seq, |x| x * 2);
    assert_eq!(doubled.length(), 10000);
    assert_eq!(*doubled.nth(0), 0);
    assert_eq!(*doubled.nth(5000), 10000);
    assert_eq!(*doubled.nth(9999), 19998);
}

#[test]
fn test_reduce_par() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtPerS::from_vec((1..=1000).collect());
    let sum = ArraySeqMtPerS::reduce_par(&pool, &seq, |a, b| a + b, 0);
    assert_eq!(sum, 500500);
}

#[test]
fn test_reduce_par_large() {
    let pool = Pool::new(6);
    let n = 100_000i64;
    let seq = ArraySeqMtPerS::from_vec((1..=n).collect());
    let sum = ArraySeqMtPerS::reduce_par(&pool, &seq, |a, b| a + b, 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_map_par_complex() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtPerS::from_vec((0..5000).collect());
    let squared = ArraySeqMtPerS::map_par(&pool, &seq, |x| x * x);
    assert_eq!(squared.length(), 5000);
    assert_eq!(*squared.nth(10), 100);
    assert_eq!(*squared.nth(100), 10000);
}

