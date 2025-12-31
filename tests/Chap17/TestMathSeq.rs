//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for MathSeq

use apas_verus::Chap17::MathSeq::MathSeq::*;
use apas_verus::MathSeqSLit;

#[test]
fn test_new() {
    let seq: MathSeqS<i32> = MathSeqS::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 42);
    assert_eq!(*seq.nth(4), 42);
}

#[test]
fn test_set() {
    let mut seq = MathSeqS::new(3, 0);
    assert!(seq.set(0, 10));
    assert!(seq.set(1, 20));
    assert!(seq.set(2, 30));
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
    assert!(!seq.set(10, 99));
}

#[test]
fn test_length() {
    let seq = MathSeqS::new(7, 1);
    assert_eq!(seq.length(), 7);
    let empty: MathSeqS<i32> = MathSeqS::empty();
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_nth() {
    let seq = MathSeqSLit![10, 20, 30, 40];
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
    assert_eq!(*seq.nth(3), 40);
}

#[test]
fn test_empty() {
    let seq: MathSeqS<i32> = MathSeqS::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.is_empty());
}

#[test]
fn test_singleton() {
    let seq: MathSeqS<i32> = MathSeqS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
    assert!(seq.is_singleton());
}

#[test]
fn test_subseq() {
    let seq = MathSeqSLit![10, 20, 30, 40, 50];
    let sub = seq.subseq(1, 3);
    assert_eq!(sub.len(), 3);
    assert_eq!(sub[0], 20);
    assert_eq!(sub[1], 30);
    assert_eq!(sub[2], 40);
}

#[test]
fn test_subseq_copy() {
    let seq = MathSeqSLit![10, 20, 30, 40, 50];
    let sub = seq.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 20);
    assert_eq!(*sub.nth(1), 30);
    assert_eq!(*sub.nth(2), 40);
}

#[test]
fn test_add_last() {
    let mut seq: MathSeqS<i32> = MathSeqS::empty();
    seq.add_last(10);
    seq.add_last(20);
    seq.add_last(30);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_delete_last() {
    let mut seq = MathSeqSLit![10, 20, 30];
    assert_eq!(seq.delete_last(), Some(30));
    assert_eq!(seq.length(), 2);
    assert_eq!(seq.delete_last(), Some(20));
    assert_eq!(seq.delete_last(), Some(10));
    assert_eq!(seq.delete_last(), None);
    assert!(seq.is_empty());
}

#[test]
fn test_is_empty() {
    let empty: MathSeqS<i32> = MathSeqS::empty();
    assert!(empty.is_empty());
    let non_empty: MathSeqS<i32> = MathSeqS::singleton(42);
    assert!(!non_empty.is_empty());
}

#[test]
fn test_is_singleton() {
    let singleton: MathSeqS<i32> = MathSeqS::singleton(42);
    assert!(singleton.is_singleton());
    let empty: MathSeqS<i32> = MathSeqS::empty();
    assert!(!empty.is_singleton());
    let two = MathSeqSLit![1, 2];
    assert!(!two.is_singleton());
}

#[test]
fn test_domain() {
    let seq = MathSeqSLit![10, 20, 30];
    let domain = seq.domain();
    assert_eq!(domain, vec![0, 1, 2]);
}

#[test]
fn test_range() {
    let seq = MathSeqSLit![10, 20, 10, 30, 20];
    let range = seq.range();
    assert_eq!(range.len(), 3);
    assert!(range.contains(&10));
    assert!(range.contains(&20));
    assert!(range.contains(&30));
}

#[test]
fn test_multiset_range() {
    let seq = MathSeqSLit![10, 20, 10, 30, 20, 10];
    let multiset = seq.multiset_range();
    assert_eq!(multiset.len(), 3);
    let counts: std::collections::HashMap<i32, usize> = multiset.into_iter().map(|(n, t)| (t, n)).collect();
    assert_eq!(counts.get(&10), Some(&3));
    assert_eq!(counts.get(&20), Some(&2));
    assert_eq!(counts.get(&30), Some(&1));
}

#[test]
fn test_iter() {
    let seq = MathSeqSLit![10, 20, 30];
    let mut iter = seq.iter();
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&20));
    assert_eq!(iter.next(), Some(&30));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut seq = MathSeqSLit![10, 20, 30];
    for x in seq.iter_mut() {
        *x += 1;
    }
    assert_eq!(*seq.nth(0), 11);
    assert_eq!(*seq.nth(1), 21);
    assert_eq!(*seq.nth(2), 31);
}

#[test]
fn test_from_vec() {
    let seq = MathSeqS::from_vec(vec![10, 20, 30]);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_with_len() {
    let seq = MathSeqS::with_len(5, 99);
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 99);
    }
}

#[test]
fn test_into_iterator_by_ref() {
    let seq = MathSeqSLit![10, 20, 30];
    let collected: Vec<&i32> = (&seq).into_iter().collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(*collected[0], 10);
    let sum: i32 = (&seq).into_iter().sum();
    assert_eq!(sum, 60);
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_into_iterator_by_mut_ref() {
    let mut seq = MathSeqSLit![10, 20, 30];
    for x in &mut seq {
        *x *= 2;
    }
    assert_eq!(*seq.nth(0), 20);
    assert_eq!(*seq.nth(1), 40);
    assert_eq!(*seq.nth(2), 60);
}

#[test]
fn test_into_iterator_by_value() {
    let seq = MathSeqSLit![10, 20, 30];
    let sum: i32 = seq.into_iter().sum();
    assert_eq!(sum, 60);
}

#[test]
fn test_partial_eq() {
    let seq1 = MathSeqSLit![10, 20, 30];
    let seq2 = MathSeqSLit![10, 20, 30];
    let seq3 = MathSeqSLit![10, 20, 99];
    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_display() {
    let seq = MathSeqSLit![10, 20, 30];
    let display_str = format!("{}", seq);
    assert!(display_str.contains("10"));
    assert!(display_str.contains("20"));
    assert!(display_str.contains("30"));
}

#[test]
fn test_debug() {
    let seq = MathSeqSLit![10, 20, 30];
    let debug_str = format!("{:?}", seq);
    assert!(debug_str.contains("10"));
    assert!(debug_str.contains("20"));
    assert!(debug_str.contains("30"));
}

#[test]
fn test_macro_empty() {
    let seq: MathSeqS<i32> = MathSeqSLit![];
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_macro_repeat() {
    let seq = MathSeqSLit![42; 5];
    assert_eq!(seq.length(), 5);
    for i in 0..5 {
        assert_eq!(*seq.nth(i), 42);
    }
}

#[test]
fn test_macro_list() {
    let seq = MathSeqSLit![1, 2, 3, 4, 5];
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(4), 5);
}
