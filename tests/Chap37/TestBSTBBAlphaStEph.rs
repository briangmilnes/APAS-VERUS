//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::BSTBBAlphaStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstbbalphastephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn bbalpha_insert_find_balances() {
    let mut bst = BSTreeBBAlpha::new();
    for value in 0..64 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 64);
    let height = bst.height();
    assert!(height <= 12, "height {height} too large");
    assert_eq!(bst.find(&10), Some(&10));
    assert_eq!(bst.find(&128), None);
    assert_eq!(bst.minimum().copied(), Some(0));
    assert_eq!(bst.maximum().copied(), Some(63));
    let inorder = bst.in_order();
    assert_eq!(inorder.length(), 64);
    for (expected, value) in (0..64).zip(inorder.iter()) {
        assert_eq!(*value, expected);
    }
}

#[test]
fn bbalpha_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}

#[test]
fn test_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
}

#[test]
fn test_singleton() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    assert_eq!(bst.size(), 1);
    assert!(bst.contains(&42));
}

#[test]
fn test_large_tree() {
    let mut bst = BSTreeBBAlpha::new();
    for i in 0..100 {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 100);
}

#[test]
fn test_reverse_insert() {
    let mut bst = BSTreeBBAlpha::new();
    for i in (0..50).rev() {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 50);
    assert_eq!(bst.minimum().copied(), Some(0));
}

#[test]
fn test_contains_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    assert!(!bst.contains(&5));
}

#[test]
fn test_negative_numbers() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(-5);
    bst.insert(-3);
    bst.insert(-7);
    assert!(bst.contains(&-5));
    assert_eq!(bst.minimum().copied(), Some(-7));
}

#[test]
fn test_find() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    assert_eq!(bst.find(&5), Some(&5));
    assert_eq!(bst.find(&10), None);
}

#[test]
fn test_maximum() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(9);
    assert_eq!(bst.maximum().copied(), Some(9));
}

#[test]
fn test_minimum_maximum_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);
}

#[test]
fn test_minimum_maximum_single() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    assert_eq!(bst.minimum().copied(), Some(42));
    assert_eq!(bst.maximum().copied(), Some(42));
}

#[test]
fn test_height_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    assert_eq!(bst.height(), 0);
}

#[test]
fn test_height_single() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    assert_eq!(bst.height(), 1);
}

#[test]
fn test_height_balanced() {
    let mut bst = BSTreeBBAlpha::new();
    // Insert in balanced order
    bst.insert(50);
    bst.insert(25);
    bst.insert(75);
    bst.insert(10);
    bst.insert(30);
    bst.insert(60);
    bst.insert(80);

    assert_eq!(bst.size(), 7);
    let h = bst.height();
    assert!(h >= 3 && h <= 4, "height should be 3 or 4, got {}", h);
}

#[test]
fn test_in_order_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    let result = bst.in_order();
    assert_eq!(result.length(), 0);
}

#[test]
fn test_in_order_single() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    let result = bst.in_order();
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 42);
}

#[test]
fn test_in_order_multiple() {
    let mut bst = BSTreeBBAlpha::new();
    let values = vec![5, 3, 7, 1, 9, 4, 6, 8, 2];
    for &v in &values {
        bst.insert(v);
    }

    let result = bst.in_order();
    assert_eq!(result.length(), 9);

    // Verify sorted order
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (i, &expected_val) in expected.iter().enumerate() {
        assert_eq!(*result.nth(i), expected_val);
    }
}

#[test]
fn test_pre_order_empty() {
    let bst = BSTreeBBAlpha::<i32>::new();
    let result = bst.pre_order();
    assert_eq!(result.length(), 0);
}

#[test]
fn test_pre_order_single() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(42);
    let result = bst.pre_order();
    assert_eq!(result.length(), 1);
    assert_eq!(*result.nth(0), 42);
}

#[test]
fn test_pre_order_multiple() {
    let mut bst = BSTreeBBAlpha::new();
    // Insert in specific order: root first
    bst.insert(50);
    bst.insert(25);
    bst.insert(75);
    bst.insert(10);
    bst.insert(30);
    bst.insert(60);
    bst.insert(80);

    let result = bst.pre_order();
    assert_eq!(result.length(), 7);
    // First element should be root-like value
    assert_eq!(*result.nth(0), 50);
}

#[test]
fn test_default_trait() {
    let bst: BSTreeBBAlpha<i32> = Default::default();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
}

#[test]
fn test_insert_ascending_rebalancing() {
    let mut bst = BSTreeBBAlpha::new();
    // Insert in strictly ascending order to test rebalancing
    for i in 1..=100 {
        bst.insert(i);
    }

    assert_eq!(bst.size(), 100);
    let h = bst.height();
    // BB[α] should keep height bounded (not degenerate to n)
    assert!(h <= 50, "height {} should not be degenerate", h);

    // Verify order preserved
    let inorder = bst.in_order();
    for i in 1..=100 {
        assert_eq!(*inorder.nth(i - 1), i);
    }
}

#[test]
fn test_insert_descending_rebalancing() {
    let mut bst = BSTreeBBAlpha::new();
    // Insert in strictly descending order to test rebalancing
    for i in (1..=100).rev() {
        bst.insert(i);
    }

    assert_eq!(bst.size(), 100);
    let h = bst.height();
    // BB[α] should keep height bounded (not degenerate to n)
    assert!(h <= 50, "height {} should not be degenerate", h);

    // Verify order preserved
    let inorder = bst.in_order();
    for i in 1..=100 {
        assert_eq!(*inorder.nth(i - 1), i);
    }
}

#[test]
fn test_insert_alternating_pattern() {
    let mut bst = BSTreeBBAlpha::new();
    // Alternating high-low pattern
    for i in 0..50 {
        if i % 2 == 0 {
            bst.insert(i);
        } else {
            bst.insert(100 - i);
        }
    }

    assert_eq!(bst.size(), 50);
    let h = bst.height();
    assert!(h <= 40, "height {} should not be degenerate", h);
}

#[test]
fn test_find_all_inserted() {
    let mut bst = BSTreeBBAlpha::new();
    let values = vec![15, 10, 20, 8, 12, 17, 25, 6, 11, 16, 27];

    for &v in &values {
        bst.insert(v);
    }

    // Find all inserted values
    for &v in &values {
        assert_eq!(bst.find(&v), Some(&v));
        assert!(bst.contains(&v));
    }

    // Find non-existent values
    for &v in &[1, 2, 3, 4, 5, 7, 9, 13, 14, 18, 19, 21, 22, 23, 24, 26, 28, 100] {
        assert_eq!(bst.find(&v), None);
        assert!(!bst.contains(&v));
    }
}

#[test]
fn test_minimum_maximum_comprehensive() {
    let mut bst = BSTreeBBAlpha::new();
    let values = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 35, 55, 65, 77, 85];

    for &v in &values {
        bst.insert(v);
    }

    assert_eq!(bst.minimum().copied(), Some(5));
    assert_eq!(bst.maximum().copied(), Some(85));
}

#[test]
fn test_very_large_tree() {
    let mut bst = BSTreeBBAlpha::new();

    // Insert 500 elements
    for i in 0..500 {
        bst.insert(i);
    }

    assert_eq!(bst.size(), 500);

    // Height should be bounded (not fully degenerate to n=500)
    let h = bst.height();
    assert!(h <= 400, "height {} should not be fully degenerate", h);

    // Spot check some values
    assert!(bst.contains(&0));
    assert!(bst.contains(&250));
    assert!(bst.contains(&499));
    assert!(!bst.contains(&500));
}

#[test]
fn test_duplicate_insertions_comprehensive() {
    let mut bst = BSTreeBBAlpha::new();

    // Insert same values multiple times
    for _ in 0..5 {
        for i in 0..10 {
            bst.insert(i);
        }
    }

    // Size should still be 10 (duplicates ignored)
    assert_eq!(bst.size(), 10);

    // Verify all present
    for i in 0..10 {
        assert!(bst.contains(&i));
    }
}

#[test]
fn test_mixed_operations_comprehensive() {
    let mut bst = BSTreeBBAlpha::new();

    // Complex sequence of operations
    bst.insert(50);
    assert_eq!(bst.size(), 1);

    bst.insert(25);
    bst.insert(75);
    assert_eq!(bst.size(), 3);
    assert_eq!(bst.minimum().copied(), Some(25));
    assert_eq!(bst.maximum().copied(), Some(75));

    bst.insert(10);
    bst.insert(30);
    bst.insert(60);
    bst.insert(80);
    assert_eq!(bst.size(), 7);

    let inorder = bst.in_order();
    let expected = vec![10, 25, 30, 50, 60, 75, 80];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(*inorder.nth(i), exp);
    }
}

#[test]
fn test_string_type() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert("dog".to_string());
    bst.insert("cat".to_string());
    bst.insert("elephant".to_string());
    bst.insert("ant".to_string());

    assert_eq!(bst.size(), 4);
    assert!(bst.contains(&"cat".to_string()));
    assert!(!bst.contains(&"zebra".to_string()));

    assert_eq!(bst.minimum().map(|s| s.as_str()), Some("ant"));
    assert_eq!(bst.maximum().map(|s| s.as_str()), Some("elephant"));
}

#[test]
fn test_clone_trait() {
    let mut bst1 = BSTreeBBAlpha::new();
    bst1.insert(1);
    bst1.insert(2);
    bst1.insert(3);

    let bst2 = bst1.clone();
    assert_eq!(bst2.size(), 3);
    assert!(bst2.contains(&1));
    assert!(bst2.contains(&2));
    assert!(bst2.contains(&3));
}

#[test]
fn test_pre_order_vs_in_order() {
    let mut bst = BSTreeBBAlpha::new();
    bst.insert(50);
    bst.insert(25);
    bst.insert(75);

    let pre = bst.pre_order();
    let ino = bst.in_order();

    assert_eq!(pre.length(), 3);
    assert_eq!(ino.length(), 3);

    // Pre-order should visit root first
    assert_eq!(*pre.nth(0), 50);

    // In-order should be sorted
    assert_eq!(*ino.nth(0), 25);
    assert_eq!(*ino.nth(1), 50);
    assert_eq!(*ino.nth(2), 75);
}
