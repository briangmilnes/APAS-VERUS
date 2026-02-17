#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTBBAlphaMtEph.

use apas_verus::BSTBBAlphaMtEphLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstbbalphamtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTBBAlphaMtEph<i32> = BSTBBAlphaMtEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTBBAlphaMtEph<i32> = BSTBBAlphaMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTBBAlphaMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_find() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&7), Some(7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_contains() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_minimum_maximum() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    assert_eq!(tree.minimum(), Some(1));
    assert_eq!(tree.maximum(), Some(9));
}

#[test]
fn test_height() {
    let tree = BSTBBAlphaMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5);
    assert!(tree.height() >= 1);

    tree.insert(3);
    tree.insert(7);
    assert!(tree.height() >= 2);
}

#[test]
fn test_in_order_traversal() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    let values = tree.in_order();
    assert_eq!(values.length(), 5);
}

#[test]
fn test_pre_order_traversal() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    let values = tree.pre_order();
    assert_eq!(values.length(), 3);
}

#[test]
fn test_size() {
    let tree = BSTBBAlphaMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1);
    assert_eq!(tree.size(), 1);

    tree.insert(2);
    tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let tree = BSTBBAlphaMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5);
    assert!(!tree.is_empty());
}

#[test]
fn test_balancing() {
    let tree = BSTBBAlphaMtEph::new();
    // Insert in order to test rebalancing
    for i in 1..=7 {
        tree.insert(i);
    }

    // BB-Alpha tree should maintain reasonable height
    let height = tree.height();
    assert!(height <= 4);
    assert_eq!(tree.size(), 7);
}

#[test]
fn test_duplicate_insert() {
    let tree = BSTBBAlphaMtEph::new();
    tree.insert(5);
    tree.insert(5);

    assert_eq!(tree.size(), 1); // Duplicates are idempotent
    assert!(tree.contains(&5));
}

#[test]
fn test_default() {
    let tree = BSTBBAlphaMtEph::<i32>::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}
