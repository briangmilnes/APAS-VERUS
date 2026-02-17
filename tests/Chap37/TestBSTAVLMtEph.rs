#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTAVLMtEph.

use apas_verus::BSTAVLMtEphLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstavlmtephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTAVLMtEph<i32> = BSTAVLMtEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
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
fn test_insert_and_find() {
    let tree = BSTAVLMtEph::new();
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
    let tree = BSTAVLMtEph::new();
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
    let tree = BSTAVLMtEph::new();
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
    let tree = BSTAVLMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5);
    assert!(tree.height() >= 1);

    tree.insert(3);
    tree.insert(7);
    assert!(tree.height() <= 2); // AVL maintains balance
}

#[test]
fn test_in_order_traversal() {
    let tree = BSTAVLMtEph::new();
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
    let tree = BSTAVLMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    let values = tree.pre_order();
    assert_eq!(values.length(), 3);
}

#[test]
fn test_size() {
    let tree = BSTAVLMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1);
    assert_eq!(tree.size(), 1);

    tree.insert(2);
    tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let tree = BSTAVLMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5);
    assert!(!tree.is_empty());
}

#[test]
fn test_avl_balancing() {
    let tree = BSTAVLMtEph::new();
    // Insert in order to test rebalancing
    for i in 1..=7 {
        tree.insert(i);
    }

    // AVL tree should maintain height balance
    let height = tree.height();
    assert!(height <= 3); // Balanced tree with 7 nodes should have height ≤ 3
    assert_eq!(tree.size(), 7);
}

#[test]
fn test_duplicate_insert() {
    let tree = BSTAVLMtEph::new();
    tree.insert(5);
    tree.insert(5);

    assert_eq!(tree.size(), 1); // Duplicates are idempotent
    assert!(tree.contains(&5));
}

#[test]
fn test_default() {
    let tree = BSTAVLMtEph::<i32>::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_from_sorted_slice() {
    let values = vec![1, 2, 3, 4, 5, 6, 7];
    let tree = BSTAVLMtEph::from_sorted_slice(&values);
    
    assert_eq!(tree.size(), 7);
    assert!(tree.contains(&1));
    assert!(tree.contains(&4));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
    
    // Tree should be balanced
    let height = tree.height();
    assert!(height <= 3); // Balanced tree with 7 nodes should have height ≤ 3
}

#[test]
fn test_from_sorted_slice_empty() {
    let values: Vec<i32> = vec![];
    let tree = BSTAVLMtEph::from_sorted_slice(&values);
    
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_filter() {
    let tree = BSTAVLMtEph::new();
    for i in 1..=10 {
        tree.insert(i);
    }
    
    // Filter even numbers
    let evens = tree.filter(|&x| x % 2 == 0);
    assert_eq!(evens.length(), 5); // 2, 4, 6, 8, 10
    
    // Verify all filtered values are even
    for i in 0..evens.length() {
        let val = evens.nth(i);
        assert_eq!(val % 2, 0);
    }
}

#[test]
fn test_filter_all() {
    let tree = BSTAVLMtEph::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    
    // Filter that accepts all
    let all = tree.filter(|_| true);
    assert_eq!(all.length(), 3);
}

#[test]
fn test_filter_none() {
    let tree = BSTAVLMtEph::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    
    // Filter that accepts none
    let none = tree.filter(|_| false);
    assert_eq!(none.length(), 0);
}

#[test]
fn test_reduce_sum() {
    let tree = BSTAVLMtEph::new();
    for i in 1..=10 {
        tree.insert(i);
    }
    
    // Sum all values: 1+2+...+10 = 55
    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 55);
}

#[test]
fn test_reduce_product() {
    let tree = BSTAVLMtEph::new();
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    
    // Product: 2*3*4 = 24
    let product = tree.reduce(|a, b| a * b, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_reduce_max() {
    let tree = BSTAVLMtEph::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(9);
    tree.insert(1);
    tree.insert(7);
    
    // Find maximum using reduce
    let max = tree.reduce(|a, b| if a > b { a } else { b }, i32::MIN);
    assert_eq!(max, 9);
}

#[test]
fn test_reduce_empty() {
    let tree = BSTAVLMtEph::<i32>::new();
    
    // Reduce on empty tree should return identity
    let sum = tree.reduce(|a, b| a + b, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_trait_new_direct() {
    let tree = <BSTAVLMtEph<i32> as BSTAVLMtEphTrait<i32>>::new();
    assert!(tree.is_empty());
}

#[test]
fn test_trait_from_sorted_direct() {
    let tree = <BSTAVLMtEph<i32> as BSTAVLMtEphTrait<i32>>::from_sorted_slice(&[1, 2, 3]);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_trait_insert_direct() {
    let tree = <BSTAVLMtEph<i32> as BSTAVLMtEphTrait<i32>>::new();
    tree.insert(10);
    assert!(tree.contains(&10));
}
