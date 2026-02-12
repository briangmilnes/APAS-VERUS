//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for base ArraySeq implementation.

use apas_verus::Chap18::ArraySeq::ArraySeq::*;

#[test]
fn test_new() {
    let seq = ArraySeqS::<i32>::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 42);
    assert_eq!(*seq.nth(4), 42);
}

#[test]
fn test_set() {
    let mut seq = ArraySeqS::<i32>::new(3, 0);
    seq.set(0, 10).unwrap();
    seq.set(1, 20).unwrap();
    seq.set(2, 30).unwrap();
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_set_out_of_bounds() {
    let mut seq = ArraySeqS::<i32>::new(3, 0);
    assert!(seq.set(10, 42).is_err());
}

#[test]
fn test_length() {
    let seq = ArraySeqS::<i32>::new(10, 0);
    assert_eq!(seq.length(), 10);

    let empty = ArraySeqS::<i32>::empty();
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_nth() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(2), 3);
    assert_eq!(*seq.nth(4), 5);
}

#[test]
fn test_empty() {
    let seq = ArraySeqS::<i32>::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.is_empty());
}

#[test]
fn test_singleton() {
    let seq = ArraySeqS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
    assert!(seq.is_singleton());
}

#[test]
fn test_tabulate() {
    let seq = tabulate(&|i| i * 2, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(1), 2);
    assert_eq!(*seq.nth(2), 4);
    assert_eq!(*seq.nth(3), 6);
    assert_eq!(*seq.nth(4), 8);
}

#[test]
fn test_map() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let doubled = map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(1), 4);
    assert_eq!(*doubled.nth(2), 6);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_is_empty() {
    let empty = ArraySeqS::<i32>::empty();
    assert!(empty.is_empty());

    let not_empty = ArraySeqS::singleton(1);
    assert!(!not_empty.is_empty());
}

#[test]
fn test_is_singleton() {
    let single = ArraySeqS::singleton(42);
    assert!(single.is_singleton());

    let empty = ArraySeqS::<i32>::empty();
    assert!(!empty.is_singleton());

    let multi = ArraySeqS::from_vec(vec![1, 2]);
    assert!(!multi.is_singleton());
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 20, 30];
    let seq = ArraySeqS::from_vec(vec);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_partial_eq() {
    let a = ArraySeqS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqS::from_vec(vec![1, 2, 3]);
    let c = ArraySeqS::from_vec(vec![1, 2, 4]);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_display() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_into_iterator() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let sum: i32 = (&seq).into_iter().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_into_iterator_mut() {
    let mut seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    for x in &mut seq {
        *x *= 2;
    }
    assert_eq!(*seq.nth(0), 2);
    assert_eq!(*seq.nth(1), 4);
    assert_eq!(*seq.nth(2), 6);
}

#[test]
fn test_into_iterator_owned() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let collected = seq.into_iter().collect::<Vec<i32>>();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_clone() {
    let a = ArraySeqS::from_vec(vec![1, 2, 3]);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_iter() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let mut iter = seq.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    for x in seq.iter_mut() {
        *x *= 2;
    }
    assert_eq!(*seq.nth(0), 2);
    assert_eq!(*seq.nth(1), 4);
    assert_eq!(*seq.nth(2), 6);
}
