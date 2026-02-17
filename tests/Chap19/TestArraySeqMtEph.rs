//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
