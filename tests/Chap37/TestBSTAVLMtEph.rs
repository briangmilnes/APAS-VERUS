// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTAVLMtEph.

use apas_verus::BSTAVLMtEphLit;
use apas_verus::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;

#[test]
fn test_bstavlmtephlit_macro_functionality() {
    let empty: BSTAVLMtEph<i32> = BSTAVLMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTAVLMtEph<i32> = BSTAVLMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTAVLMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_contains() {
    let mut tree = BSTAVLMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_contains() {
    let mut tree = BSTAVLMtEph::new();
    tree.insert(10).unwrap();
    tree.insert(5).unwrap();
    tree.insert(15).unwrap();

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_height() {
    let mut tree = BSTAVLMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5).unwrap();
    assert!(tree.height() >= 1);

    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    assert!(tree.height() <= 2);
}

#[test]
fn test_size() {
    let mut tree = BSTAVLMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1).unwrap();
    assert_eq!(tree.size(), 1);

    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let mut tree = BSTAVLMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5).unwrap();
    assert!(!tree.is_empty());
}

#[test]
fn test_duplicate_insert() {
    let mut tree = BSTAVLMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(5).unwrap();

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_insert_sequential() {
    let mut tree = BSTAVLMtEph::new();
    for i in 1..=7 {
        tree.insert(i).unwrap();
    }

    let height = tree.height();
    assert!(height <= 7, "height was {}", height);
    assert_eq!(tree.size(), 7);
}

#[test]
fn test_trait_new_direct() {
    let tree = <BSTAVLMtEph<i32> as BSTAVLMtEphTrait<i32>>::new();
    assert!(tree.is_empty());
}

#[test]
fn test_trait_insert_direct() {
    let mut tree = <BSTAVLMtEph<i32> as BSTAVLMtEphTrait<i32>>::new();
    tree.insert(10).unwrap();
    assert!(tree.contains(&10));
}
