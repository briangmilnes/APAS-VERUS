// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTBBAlphaStEph.

use apas_verus::BSTBBAlphaStEphLit;
use apas_verus::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;

#[test]
fn test_bstbbalphastephlit_macro_empty() {
    let empty: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());
}

#[test]
fn test_bstbbalphastephlit_macro_with_elements() {
    let tree: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![5, 3, 7, 1, 9];
    assert_eq!(tree.size(), 5);
    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTBBAlphaStEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_find() {
    let tree = BSTBBAlphaStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(3);
    let tree = tree.insert(7);

    assert_eq!(tree.find(&5), Some(&5));
    assert_eq!(tree.find(&3), Some(&3));
    assert_eq!(tree.find(&7), Some(&7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_contains() {
    let tree = BSTBBAlphaStEph::new();
    let tree = tree.insert(10);
    let tree = tree.insert(5);
    let tree = tree.insert(15);

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_height() {
    let tree = BSTBBAlphaStEph::<i32>::new();
    assert_eq!(tree.height(), 0);

    let tree = tree.insert(5);
    assert!(tree.height() >= 1);

    let tree = tree.insert(3);
    let tree = tree.insert(7);
    assert!(tree.height() >= 2);
}

#[test]
fn test_size() {
    let tree = BSTBBAlphaStEph::<i32>::new();
    assert_eq!(tree.size(), 0);

    let tree = tree.insert(1);
    assert_eq!(tree.size(), 1);

    let tree = tree.insert(2);
    let tree = tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_duplicate_insert() {
    let tree = BSTBBAlphaStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(5);

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_balancing_ascending() {
    let mut tree = BSTBBAlphaStEph::new();
    for i in 1..=64 {
        tree = tree.insert(i);
    }
    assert_eq!(tree.size(), 64);
    assert!(tree.height() <= 64);
}

#[test]
fn test_delete_leaf() {
    let tree = BSTBBAlphaStEph::new();
    let tree = tree.insert(5);
    let tree = tree.delete(&5);
    assert_eq!(tree.size(), 0);
    assert!(!tree.contains(&5));
}

#[test]
fn test_delete_nonexistent() {
    let tree = BSTBBAlphaStEph::new();
    let tree = tree.insert(5);
    let tree = tree.delete(&10);
    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_delete_with_children() {
    let tree = BSTBBAlphaStEphLit![5, 3, 7, 1, 4, 6, 9];
    assert_eq!(tree.size(), 7);

    let tree = tree.delete(&3);
    assert_eq!(tree.size(), 6);
    assert!(!tree.contains(&3));
    assert!(tree.contains(&1));
    assert!(tree.contains(&4));

    let tree = tree.delete(&7);
    assert_eq!(tree.size(), 5);
    assert!(!tree.contains(&7));
    assert!(tree.contains(&6));
    assert!(tree.contains(&9));
}

#[test]
fn test_delete_root() {
    let tree = BSTBBAlphaStEphLit![5, 3, 7];
    let tree = tree.delete(&5);
    assert_eq!(tree.size(), 2);
    assert!(!tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
}

#[test]
fn test_delete_all() {
    let tree = BSTBBAlphaStEphLit![5, 3, 7];
    let tree = tree.delete(&5);
    let tree = tree.delete(&3);
    let tree = tree.delete(&7);
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}
