//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapStEph.

use apas_verus::BSTTreapStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap39::BSTTreapStEph::BSTTreapStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bsttreapstephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTTreapStEph<i32> = BSTTreapStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTTreapStEph<i32> = BSTTreapStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_empty() {
    let tree: BSTTreapStEph<i32> = Default::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_size() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    assert_eq!(tree.size(), 1);
    tree.insert(3);
    tree.insert(7);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_contains() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_find() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);

    assert_eq!(tree.find(&5), Some(&5));
    assert_eq!(tree.find(&3), Some(&3));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_minimum_maximum() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);

    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    assert_eq!(tree.minimum(), Some(&1));
    assert_eq!(tree.maximum(), Some(&9));
}

#[test]
fn test_height() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    assert_eq!(tree.height(), 0);

    tree.insert(5);
    assert!(tree.height() >= 1);

    tree.insert(3);
    tree.insert(7);
    assert!(tree.height() >= 1);
}

#[test]
fn test_in_order() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    let _seq = tree.in_order();
    // Just verify it returns without panicking
}

#[test]
fn test_pre_order() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    let _seq = tree.pre_order();
    // Just verify it returns without panicking
}

#[test]
fn test_is_empty() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    assert!(tree.is_empty());

    tree.insert(5);
    assert!(!tree.is_empty());
}

#[test]
fn test_duplicate_insert() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(5);
    tree.insert(5);

    // Treaps may allow duplicates depending on implementation
    assert!(tree.size() >= 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_single_element() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(42);

    assert_eq!(tree.size(), 1);
    assert_eq!(tree.minimum(), Some(&42));
    assert_eq!(tree.maximum(), Some(&42));
    assert_eq!(tree.height(), 1);
}

#[test]
fn test_large_tree() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    for i in 0..100 {
        tree.insert(i);
    }

    assert_eq!(tree.size(), 100);
    assert_eq!(tree.minimum(), Some(&0));
    assert_eq!(tree.maximum(), Some(&99));
}

#[test]
fn test_reverse_order_insert() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);
    tree.insert(2);
    tree.insert(1);

    assert_eq!(tree.size(), 5);
    assert_eq!(tree.minimum(), Some(&1));
    assert_eq!(tree.maximum(), Some(&5));
}

#[test]
fn test_negative_numbers() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(-5);
    tree.insert(-10);
    tree.insert(-3);
    tree.insert(-1);

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.minimum(), Some(&-10));
    assert_eq!(tree.maximum(), Some(&-1));
    assert!(tree.contains(&-5));
}

#[test]
fn test_mixed_positive_negative() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(-3);
    tree.insert(0);
    tree.insert(-10);
    tree.insert(15);

    assert_eq!(tree.minimum(), Some(&-10));
    assert_eq!(tree.maximum(), Some(&15));
    assert!(tree.contains(&0));
}

#[test]
fn test_find_missing() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    assert_eq!(tree.find(&5), Some(&5));
    assert_eq!(tree.find(&100), None);
    assert_eq!(tree.find(&-100), None);
}

#[test]
fn test_in_order_traversal() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);

    let seq = tree.in_order();
    assert_eq!(seq.length(), 5);

    // In-order should give sorted sequence
    let expected = [1, 3, 5, 7, 9];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(*seq.nth(i), exp);
    }
}

#[test]
fn test_pre_order_traversal() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    let seq = tree.pre_order();
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_sequential_inserts() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    for i in 0..50 {
        tree.insert(i);
    }

    assert_eq!(tree.size(), 50);
    assert_eq!(tree.minimum(), Some(&0));
    assert_eq!(tree.maximum(), Some(&49));

    // Verify all elements are present
    for i in 0..50 {
        assert!(tree.contains(&i));
    }
}

#[test]
fn test_random_order_inserts() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    let values = vec![15, 3, 22, 8, 1, 45, 12, 30, 7, 19];

    for &v in &values {
        tree.insert(v);
    }

    assert_eq!(tree.size(), 10);
    assert_eq!(tree.minimum(), Some(&1));
    assert_eq!(tree.maximum(), Some(&45));

    for &v in &values {
        assert!(tree.contains(&v));
    }
}

#[test]
fn test_height_balanced() {
    let mut tree: BSTTreapStEph<i32> = Default::default();

    // Insert many elements
    for i in 0..64 {
        tree.insert(i);
    }

    let height = tree.height();
    // Treap should maintain reasonable balance (not degenerate)
    // For 64 elements, expect O(log n) height, so should be < 20
    assert!(height < 20, "Height {height} is too large for 64 elements");
}

#[test]
fn test_empty_operations() {
    let tree: BSTTreapStEph<i32> = Default::default();

    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
    assert_eq!(tree.find(&5), None);
    assert!(!tree.contains(&5));

    let seq = tree.in_order();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_contains_after_many_inserts() {
    let mut tree: BSTTreapStEph<i32> = Default::default();

    for i in (0..100).step_by(3) {
        tree.insert(i);
    }

    // Check present values
    for i in (0..100).step_by(3) {
        assert!(tree.contains(&i));
    }

    // Check absent values
    assert!(!tree.contains(&1));
    assert!(!tree.contains(&2));
    assert!(!tree.contains(&100));
}

#[test]
fn test_string_treap() {
    let mut tree: BSTTreapStEph<String> = Default::default();

    tree.insert("dog".to_string());
    tree.insert("cat".to_string());
    tree.insert("elephant".to_string());
    tree.insert("ant".to_string());

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.minimum(), Some(&"ant".to_string()));
    assert_eq!(tree.maximum(), Some(&"elephant".to_string()));
    assert!(tree.contains(&"dog".to_string()));
}

#[test]
fn test_traversal_empty_tree() {
    let tree: BSTTreapStEph<i32> = Default::default();

    let in_seq = tree.in_order();
    assert_eq!(in_seq.length(), 0);

    let pre_seq = tree.pre_order();
    assert_eq!(pre_seq.length(), 0);
}

#[test]
fn test_singleton_traversal() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(42);

    let in_seq = tree.in_order();
    assert_eq!(in_seq.length(), 1);
    assert_eq!(*in_seq.nth(0), 42);

    let pre_seq = tree.pre_order();
    assert_eq!(pre_seq.length(), 1);
    assert_eq!(*pre_seq.nth(0), 42);
}

#[test]
fn test_zero_value() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(0);
    tree.insert(-5);
    tree.insert(5);

    assert_eq!(tree.size(), 3);
    assert!(tree.contains(&0));
    assert_eq!(tree.find(&0), Some(&0));
}

#[test]
fn test_extremes() {
    let mut tree: BSTTreapStEph<i32> = Default::default();
    tree.insert(i32::MAX);
    tree.insert(i32::MIN);
    tree.insert(0);

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.minimum(), Some(&i32::MIN));
    assert_eq!(tree.maximum(), Some(&i32::MAX));
}
