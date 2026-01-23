//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPer with parallel operations.

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;

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
    let sum = ArraySeqMtPerS::reduce(&seq, &|a, b| a + b, 0);
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
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, 0);
    assert_eq!(sum, 64 * 65 / 2);
}

#[test]
fn test_reduce_par_medium() {
    let n = 256i64;
    let seq = ArraySeqMtPerS::from_vec((1..=n).collect());
    let sum = ArraySeqMtPerS::reduce_par(&seq, |a, b| a + b, 0i64);
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
