#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTPlainMtEph.

use apas_verus::BSTPlainMtEphLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstplainmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTPlainMtEph<i32> = BSTPlainMtEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTPlainMtEph<i32> = BSTPlainMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTPlainMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_find() {
    let tree = BSTPlainMtEph::new();
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
    let tree = BSTPlainMtEph::new();
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
    let tree = BSTPlainMtEph::new();
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
    let tree = BSTPlainMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5);
    assert!(tree.height() >= 1);

    tree.insert(3);
    tree.insert(7);
    assert!(tree.height() >= 2);
}

#[test]
fn test_in_order_traversal() {
    let tree = BSTPlainMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    let values = tree.in_order();
    assert_eq!(values.length(), 5);
}

#[test]
fn test_size() {
    let tree = BSTPlainMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1);
    assert_eq!(tree.size(), 1);

    tree.insert(2);
    tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let tree = BSTPlainMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5);
    assert!(!tree.is_empty());
}

#[test]
fn test_duplicate_insert() {
    let tree = BSTPlainMtEph::new();
    tree.insert(5);
    tree.insert(5);

    assert_eq!(tree.size(), 1); // Duplicates are idempotent
    assert!(tree.contains(&5));
}
