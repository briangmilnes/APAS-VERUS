//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTAVLStEph.

use apas_verus::BSTAVLStEphLit;
use apas_verus::Chap37::BSTAVLStEph::BSTAVLStEph::*;

#[test]
fn test_bstavlstephlit_macro_empty() {
    let empty: BSTAVLStEph<i32> = BSTAVLStEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());
}

#[test]
fn test_bstavlstephlit_macro_with_elements() {
    let tree: BSTAVLStEph<i32> = BSTAVLStEphLit![5, 3, 7, 1, 9];
    assert_eq!(tree.size(), 5);
    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_avl_new_empty() {
    let tree = BSTAVLStEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_avl_insert_and_find() {
    let tree = BSTAVLStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(3);
    let tree = tree.insert(7);

    assert_eq!(tree.find(&5), Some(&5));
    assert_eq!(tree.find(&3), Some(&3));
    assert_eq!(tree.find(&7), Some(&7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_avl_contains() {
    let tree = BSTAVLStEph::new();
    let tree = tree.insert(10);
    let tree = tree.insert(5);
    let tree = tree.insert(15);

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_avl_height() {
    let tree = BSTAVLStEph::<i32>::new();
    assert_eq!(tree.height(), 0);

    let tree = tree.insert(5);
    assert!(tree.height() >= 1);

    let tree = tree.insert(3);
    let tree = tree.insert(7);
    assert!(tree.height() >= 2);
}

#[test]
fn test_avl_size() {
    let tree = BSTAVLStEph::<i32>::new();
    assert_eq!(tree.size(), 0);

    let tree = tree.insert(1);
    assert_eq!(tree.size(), 1);

    let tree = tree.insert(2);
    let tree = tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_avl_duplicate_insert() {
    let tree = BSTAVLStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(5);

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_avl_balancing_ascending() {
    let mut tree = BSTAVLStEph::new();
    for i in 1..=20 {
        tree = tree.insert(i);
    }
    assert_eq!(tree.size(), 20);
    assert!(tree.height() <= 20);
}
