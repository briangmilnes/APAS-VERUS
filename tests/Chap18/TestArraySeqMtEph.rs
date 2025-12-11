//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtEph with parallel operations.

use apas_verus::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::Pool;

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
    let sum = ArraySeqMtEphS::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_map_par() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let doubled = ArraySeqMtEphS::map_par(&pool, &seq, |x| x * 2);
    assert_eq!(doubled.length(), 64);
    assert_eq!(*doubled.nth(0), 0);
    assert_eq!(*doubled.nth(32), 64);
    assert_eq!(*doubled.nth(63), 126);
}

#[test]
fn test_reduce_par() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtEphS::from_vec((1..=64).collect());
    let sum = ArraySeqMtEphS::reduce_par(&pool, &seq, |a, b| a + b, 0);
    assert_eq!(sum, 64 * 65 / 2);
}

#[test]
fn test_reduce_par_medium() {
    let pool = Pool::new(4);
    let n = 256i64;
    let seq = ArraySeqMtEphS::from_vec((1..=n).collect());
    let sum = ArraySeqMtEphS::reduce_par(&pool, &seq, |a, b| a + b, 0i64);
    assert_eq!(sum, n * (n + 1) / 2);
}

#[test]
fn test_map_par_medium() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtEphS::from_vec((0..128).collect());
    let squared = ArraySeqMtEphS::map_par(&pool, &seq, |x| x * x);
    assert_eq!(squared.length(), 128);
    assert_eq!(*squared.nth(10), 100);
    assert_eq!(*squared.nth(11), 121);
}

#[test]
fn test_filter_par() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtEphS::from_vec((0..64).collect());
    let evens = ArraySeqMtEphS::filter_par(&pool, &seq, |x| x % 2 == 0);
    assert_eq!(evens.length(), 32);
    assert_eq!(*evens.nth(0), 0);
    assert_eq!(*evens.nth(1), 2);
    assert_eq!(*evens.nth(31), 62);
}

#[test]
fn test_filter_par_medium() {
    let pool = Pool::new(4);
    let seq = ArraySeqMtEphS::from_vec((0..256).collect());
    let multiples_of_7 = ArraySeqMtEphS::filter_par(&pool, &seq, |&x| x % 7 == 0);
    assert_eq!(multiples_of_7.length(), 37);
}

