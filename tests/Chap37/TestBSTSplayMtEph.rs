// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTSplayMtEph.

use apas_verus::BSTSplayMtEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;

#[test]
fn test_bstsplaymtephlit_macro_functionality() {
    let empty: BSTSplayMtEph<i32> = BSTSplayMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTSplayMtEph<i32> = BSTSplayMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTSplayMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_find() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&7), Some(7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_contains() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(10).unwrap();
    tree.insert(5).unwrap();
    tree.insert(15).unwrap();

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_minimum_maximum() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    tree.insert(1).unwrap();
    tree.insert(9).unwrap();

    assert_eq!(tree.minimum(), Some(1));
    assert_eq!(tree.maximum(), Some(9));
}

#[test]
fn test_height() {
    let mut tree = BSTSplayMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5).unwrap();
    assert!(tree.height() >= 1);

    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    assert!(tree.height() >= 2);
}

#[test]
fn test_in_order_traversal() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    tree.insert(1).unwrap();
    tree.insert(9).unwrap();

    let values = tree.in_order();
    assert_eq!(values.length(), 5);
}

#[test]
fn test_pre_order_traversal() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    let values = tree.pre_order();
    assert_eq!(values.length(), 3);
}

#[test]
fn test_size() {
    let mut tree = BSTSplayMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1).unwrap();
    assert_eq!(tree.size(), 1);

    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let mut tree = BSTSplayMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5).unwrap();
    assert!(!tree.is_empty());
}

#[test]
fn test_duplicate_insert() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(5).unwrap();

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_default() {
    let tree = BSTSplayMtEph::<i32>::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_from_sorted_slice() {
    let values = vec![1, 2, 3, 4, 5, 6, 7];
    let tree = BSTSplayMtEph::from_sorted_slice(&values);

    assert_eq!(tree.size(), 7);
    assert!(tree.contains(&1));
    assert!(tree.contains(&4));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_from_sorted_slice_empty() {
    let values: Vec<i32> = vec![];
    let tree = BSTSplayMtEph::from_sorted_slice(&values);

    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_filter() {
    let mut tree = BSTSplayMtEph::new();
    for i in 1..=10 {
        tree.insert(i).unwrap();
    }

    let evens = tree.filter(|&x| x % 2 == 0);
    assert_eq!(evens.length(), 5);
}

#[test]
fn test_filter_all() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(1).unwrap();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();

    let all = tree.filter(|_| true);
    assert_eq!(all.length(), 3);
}

#[test]
fn test_filter_none() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(1).unwrap();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();

    let none = tree.filter(|_| false);
    assert_eq!(none.length(), 0);
}

#[test]
fn test_reduce_sum() {
    let mut tree = BSTSplayMtEph::new();
    for i in 1..=10 {
        tree.insert(i).unwrap();
    }

    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 55);
}

#[test]
fn test_reduce_product() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    tree.insert(4).unwrap();

    let product = tree.reduce(|a, b| a * b, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_reduce_max() {
    let mut tree = BSTSplayMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(9).unwrap();
    tree.insert(1).unwrap();
    tree.insert(7).unwrap();

    let max = tree.reduce(|a, b| if a > b { a } else { b }, i32::MIN);
    assert_eq!(max, 9);
}

#[test]
fn test_reduce_empty() {
    let tree = BSTSplayMtEph::<i32>::new();

    let sum = tree.reduce(|a, b| a + b, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_default_trait() {
    let tree: BSTSplayMtEph<i32> = Default::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_trait_new() {
    let tree = <BSTSplayMtEph<i32> as BSTSplayMtEphTrait<i32>>::new();
    assert!(tree.is_empty());
}
