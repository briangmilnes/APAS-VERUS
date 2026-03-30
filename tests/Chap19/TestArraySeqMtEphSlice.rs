//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Runtime tests for Chap19 ArraySeqMtEphSlice.

use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

#[test]
fn test_empty() {
    let seq = ArraySeqMtEphSlice::<i32>::empty();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_singleton() {
    let seq = ArraySeqMtEphSlice::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.nth_cloned(0), 42);
}

#[test]
fn test_from_vec() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![10, 20, 30]);
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(2), 30);
}

#[test]
fn test_new() {
    let seq = ArraySeqMtEphSlice::<i32>::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(seq.nth_cloned(0), 42);
    assert_eq!(seq.nth_cloned(4), 42);
}

#[test]
fn test_slice_basic() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![10, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    assert_eq!(sliced.length(), 3);
    assert_eq!(sliced.nth_cloned(0), 20);
    assert_eq!(sliced.nth_cloned(1), 30);
    assert_eq!(sliced.nth_cloned(2), 40);
}

#[test]
fn test_slice_full() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3]);
    let sliced = seq.slice(0, 3);
    assert_eq!(sliced.length(), 3);
    assert_eq!(sliced.nth_cloned(0), 1);
    assert_eq!(sliced.nth_cloned(2), 3);
}

#[test]
fn test_slice_empty() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3]);
    let sliced = seq.slice(1, 0);
    assert_eq!(sliced.length(), 0);
}

#[test]
fn test_subseq_copy() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![10, 20, 30, 40, 50]);
    let sub = seq.subseq_copy(2, 2);
    assert_eq!(sub.length(), 2);
    assert_eq!(sub.nth_cloned(0), 30);
    assert_eq!(sub.nth_cloned(1), 40);
}

#[test]
fn test_to_vec() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3, 4]);
    let v = seq.to_vec();
    assert_eq!(v, vec![1, 2, 3, 4]);
}

#[test]
fn test_iter() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![10, 20, 30]);
    let mut it = seq.iter();
    assert_eq!(it.next(), Some(&10));
    assert_eq!(it.next(), Some(&20));
    assert_eq!(it.next(), Some(&30));
    assert_eq!(it.next(), None);
}

#[test]
fn test_iter_for_loop() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3, 4, 5]);
    let mut sum = 0;
    for v in &seq {
        sum += v;
    }
    assert_eq!(sum, 15);
}

#[test]
fn test_large_sequence() {
    let data: Vec<usize> = (0..200).collect();
    let seq = ArraySeqMtEphSlice::from_vec(data);
    assert_eq!(seq.length(), 200);
    assert_eq!(seq.nth_cloned(0), 0);
    assert_eq!(seq.nth_cloned(199), 199);
}

#[test]
fn test_slice_then_iter() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![10, 20, 30, 40, 50]);
    let sliced = seq.slice(1, 3);
    let collected: Vec<i32> = sliced.iter().map(|x| *x).collect();
    assert_eq!(collected, vec![20, 30, 40]);
}

#[test]
fn test_display() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = ArraySeqMtEphSlice::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}
