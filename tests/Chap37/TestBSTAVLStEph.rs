//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::BSTAVLStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap37::BSTAVLStEph::BSTAVLStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstavlstephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTAVLStEph<i32> = BSTAVLStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTAVLStEph<i32> = BSTAVLStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn avl_insert_find_and_bounds() {
    let mut bst = BSTreeAVL::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert!(bst.height() <= 4);
    assert!(bst.contains(&3));
    assert_eq!(bst.find(&3), Some(&3));
    assert!(!bst.contains(&9));
    assert_eq!(bst.find(&9), None);
    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(7));
    let inorder = bst.in_order();
    let expected = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(inorder.length(), expected.len());
    for (exp, value) in expected.iter().zip(inorder.iter()) {
        assert_eq!(*value, *exp);
    }
}

#[test]
fn avl_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeAVL::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}

// Tests for remaining 25% untested BST variant methods
#[test]
fn test_bst_empty_constructor() {
    let bst = BSTreeAVL::<N>::new();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
    assert_eq!(bst.height(), 0);
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);
    assert_eq!(bst.find(&42), None);
    assert!(!bst.contains(&42));
}

#[test]
fn test_bst_single_element() {
    let mut bst = BSTreeAVL::new();
    bst.insert(42);

    assert_eq!(bst.size(), 1);
    assert!(!bst.is_empty());
    assert_eq!(bst.height(), 1);
    assert_eq!(bst.minimum(), Some(&42));
    assert_eq!(bst.maximum(), Some(&42));
    assert_eq!(bst.find(&42), Some(&42));
    assert!(bst.contains(&42));
    assert!(!bst.contains(&99));

    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 1);
    assert_eq!(*inorder.nth(0), 42);

    let preorder = bst.pre_order();
    assert_eq!(preorder.length(), 1);
    assert_eq!(*preorder.nth(0), 42);
}

#[test]
fn test_bst_is_empty_predicate() {
    let mut bst = BSTreeAVL::new();
    assert!(bst.is_empty());

    bst.insert(10);
    assert!(!bst.is_empty());

    // Create another empty tree to verify
    let empty_bst = BSTreeAVL::<N>::new();
    assert!(empty_bst.is_empty());
}

#[test]
fn test_bst_height_calculation() {
    let mut bst = BSTreeAVL::new();
    assert_eq!(bst.height(), 0);

    // Single node
    bst.insert(10);
    assert_eq!(bst.height(), 1);

    // Balanced tree
    bst.insert(5);
    bst.insert(15);
    assert!(bst.height() <= 2); // AVL tree should be balanced

    // Add more nodes
    bst.insert(3);
    bst.insert(7);
    bst.insert(12);
    bst.insert(18);

    // AVL tree should maintain logarithmic height
    assert!(bst.height() <= 4);
    assert_eq!(bst.size(), 7);
}

#[test]
fn test_bst_pre_order_traversal() {
    let mut bst = BSTreeAVL::new();

    // Test empty tree
    let empty_preorder = bst.pre_order();
    assert_eq!(empty_preorder.length(), 0);

    // Build a specific tree structure
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }

    let preorder = bst.pre_order();
    assert_eq!(preorder.length(), 7);

    // Pre-order should visit root first, then left subtree, then right subtree
    // The exact order depends on AVL balancing, but we can verify all elements are present
    let mut found_values = vec![];
    for i in 0..preorder.length() {
        found_values.push(*preorder.nth(i));
    }
    found_values.sort();
    assert_eq!(found_values, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_bst_minimum_maximum_edge_cases() {
    let mut bst = BSTreeAVL::new();

    // Empty tree
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);

    // Single element
    bst.insert(42);
    assert_eq!(bst.minimum(), Some(&42));
    assert_eq!(bst.maximum(), Some(&42));

    // Left-skewed insertions
    bst.insert(30);
    bst.insert(20);
    bst.insert(10);
    assert_eq!(bst.minimum(), Some(&10));
    assert_eq!(bst.maximum(), Some(&42));

    // Right-skewed insertions
    bst.insert(50);
    bst.insert(60);
    bst.insert(70);
    assert_eq!(bst.minimum(), Some(&10));
    assert_eq!(bst.maximum(), Some(&70));
}

#[test]
fn test_bst_contains_comprehensive() {
    let mut bst = BSTreeAVL::new();

    // Empty tree
    assert!(!bst.contains(&1));

    // Build tree with various values
    let values = [15, 10, 20, 8, 12, 25, 6, 11, 13, 27];
    for &value in &values {
        bst.insert(value);
    }

    // Test all inserted values
    for &value in &values {
        assert!(bst.contains(&value));
    }

    // Test non-existent values
    let non_existent = [1, 5, 7, 9, 14, 16, 19, 21, 24, 26, 30];
    for &value in &non_existent {
        assert!(!bst.contains(&value));
    }
}

#[test]
fn test_bst_find_comprehensive() {
    let mut bst = BSTreeAVL::new();

    // Empty tree
    assert_eq!(bst.find(&42), None);

    // Build tree
    let values = [50, 30, 70, 20, 40, 60, 80];
    for &value in &values {
        bst.insert(value);
    }

    // Test finding all values
    for &value in &values {
        assert_eq!(bst.find(&value), Some(&value));
    }

    // Test finding non-existent values
    assert_eq!(bst.find(&10), None);
    assert_eq!(bst.find(&35), None);
    assert_eq!(bst.find(&65), None);
    assert_eq!(bst.find(&90), None);
}

#[test]
fn test_bst_large_tree_operations() {
    let mut bst = BSTreeAVL::new();

    // Insert many values
    let values = (1..=100).collect::<Vec<N>>();
    for &value in &values {
        bst.insert(value);
    }

    assert_eq!(bst.size(), 100);
    assert!(!bst.is_empty());

    // AVL tree should maintain logarithmic height
    assert!(bst.height() <= 8); // log2(100) ≈ 6.6, AVL allows +1

    // Test minimum and maximum
    assert_eq!(bst.minimum(), Some(&1));
    assert_eq!(bst.maximum(), Some(&100));

    // Test random access
    for &value in &[1, 25, 50, 75, 100] {
        assert!(bst.contains(&value));
        assert_eq!(bst.find(&value), Some(&value));
    }

    // Test in-order traversal
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 100);
    for i in 0..100 {
        assert_eq!(*inorder.nth(i), (i + 1) as N);
    }
}

#[test]
fn test_bst_duplicate_handling() {
    let mut bst = BSTreeAVL::new();

    // Insert duplicates
    bst.insert(10);
    bst.insert(5);
    bst.insert(15);
    bst.insert(10); // Duplicate
    bst.insert(5); // Duplicate
    bst.insert(15); // Duplicate

    // Size should not increase for duplicates
    assert_eq!(bst.size(), 3);

    // All values should still be findable
    assert!(bst.contains(&10));
    assert!(bst.contains(&5));
    assert!(bst.contains(&15));

    // In-order should still be correct
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 3);
    assert_eq!(*inorder.nth(0), 5);
    assert_eq!(*inorder.nth(1), 10);
    assert_eq!(*inorder.nth(2), 15);
}

#[test]
fn test_bst_avl_balancing_stress() {
    let mut bst = BSTreeAVL::new();

    // Insert in ascending order (worst case for unbalanced BST)
    for i in 1..=20 {
        bst.insert(i);
    }

    // AVL tree should remain balanced
    assert_eq!(bst.size(), 20);
    assert!(bst.height() <= 6); // Should be much better than 20 (unbalanced height)

    // Verify all elements are present and in order
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 20);
    for i in 0..20 {
        assert_eq!(*inorder.nth(i), (i + 1) as N);
    }

    // Test descending order insertion
    let mut bst2 = BSTreeAVL::new();
    for i in (1..=20).rev() {
        bst2.insert(i);
    }

    assert_eq!(bst2.size(), 20);
    assert!(bst2.height() <= 6);

    // Both trees should have same in-order traversal
    let inorder2 = bst2.in_order();
    assert_eq!(inorder.length(), inorder2.length());
    for i in 0..inorder.length() {
        assert_eq!(*inorder.nth(i), *inorder2.nth(i));
    }
}

#[test]
fn test_bst_default_trait() {
    let bst: BSTreeAVL<N> = Default::default();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
    assert_eq!(bst.height(), 0);
}

#[test]
fn test_bst_clone_functionality() {
    let mut original = BSTreeAVL::new();
    for value in [10, 5, 15, 3, 7, 12, 18] {
        original.insert(value);
    }

    let cloned = original.clone();

    // Both should have same properties
    assert_eq!(original.size(), cloned.size());
    assert_eq!(original.height(), cloned.height());
    assert_eq!(original.minimum(), cloned.minimum());
    assert_eq!(original.maximum(), cloned.maximum());

    // Both should have same in-order traversal
    let orig_inorder = original.in_order();
    let clone_inorder = cloned.in_order();
    assert_eq!(orig_inorder.length(), clone_inorder.length());
    for i in 0..orig_inorder.length() {
        assert_eq!(*orig_inorder.nth(i), *clone_inorder.nth(i));
    }

    // Modifications to one should not affect the other
    let mut modified_clone = cloned;
    modified_clone.insert(100);
    assert_eq!(original.size(), 7);
    assert_eq!(modified_clone.size(), 8);
}
