//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Runtime tests for LinkedListStPer (sequential persistent linked-list sequence).

use apas_verus::Chap18::LinkedListStPer::LinkedListStPer::*;

#[test]
fn test_new() {
    let seq = LinkedListStPerS::<i32>::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 42);
    assert_eq!(*seq.nth(4), 42);
}

#[test]
fn test_empty() {
    let seq = LinkedListStPerS::<i32>::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.is_empty());
}

#[test]
fn test_singleton() {
    let seq = LinkedListStPerS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
    assert!(seq.is_singleton());
}

#[test]
fn test_from_vec() {
    let seq = LinkedListStPerS::from_vec(vec![10, 20, 30]);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_append() {
    let a = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let b = LinkedListStPerS::from_vec(vec![4, 5]);
    let c = LinkedListStPerS::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(3), 4);
    assert_eq!(*c.nth(4), 5);
}

#[test]
fn test_subseq() {
    let seq = LinkedListStPerS::from_vec(vec![10, 20, 30, 40, 50]);
    let sub = seq.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 20);
    assert_eq!(*sub.nth(2), 40);
}

#[test]
fn test_update() {
    let seq = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let updated = LinkedListStPerS::update(&seq, 1, 99);
    assert_eq!(*updated.nth(1), 99);
    // Persistent: original unchanged.
    assert_eq!(*seq.nth(1), 2);
}

#[test]
fn test_partial_eq() {
    let a = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let b = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let c = LinkedListStPerS::from_vec(vec![1, 2, 4]);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_clone() {
    let a = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_display() {
    let seq = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_map() {
    let seq = LinkedListStPerS::from_vec(vec![1, 2, 3, 4]);
    let doubled = LinkedListStPerS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_tabulate() {
    let seq = LinkedListStPerS::tabulate(&|i| i * 3, 4);
    assert_eq!(seq.length(), 4);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(3), 9);
}

#[test]
fn test_iter() {
    let seq = LinkedListStPerS::from_vec(vec![1, 2, 3]);
    let mut it = seq.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}
