//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::BSTRBStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap37::BSTRBStEph::BSTRBStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstrbstephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTRBStEph<i32> = BSTRBStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTRBStEph<i32> = BSTRBStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn rb_insert_find_and_bounds() {
    let mut bst = BSTreeRB::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert!(bst.height() <= 5);
    assert_eq!(bst.find(&3), Some(&3));
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
fn rb_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeRB::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}

#[test]
fn rb_new_and_default() {
    let bst = BSTRBStEph::<i32>::new();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());

    let bst_default = BSTRBStEph::<i32>::default();
    assert_eq!(bst_default.size(), 0);
    assert!(bst_default.is_empty());
}

#[test]
fn rb_contains() {
    let mut bst = BSTreeRB::new();
    bst.insert(10);
    bst.insert(5);
    bst.insert(15);

    assert!(bst.contains(&10));
    assert!(bst.contains(&5));
    assert!(bst.contains(&15));
    assert!(!bst.contains(&20));
    assert!(!bst.contains(&0));
}

#[test]
fn rb_is_empty() {
    let mut bst = BSTreeRB::new();
    assert!(bst.is_empty());

    bst.insert(42);
    assert!(!bst.is_empty());
}

#[test]
fn rb_height() {
    let mut bst = BSTreeRB::new();
    assert_eq!(bst.height(), 0);

    bst.insert(50);
    assert_eq!(bst.height(), 1);

    bst.insert(30);
    bst.insert(70);
    assert!(bst.height() >= 2);

    bst.insert(20);
    bst.insert(40);
    bst.insert(60);
    bst.insert(80);
    assert!(bst.height() >= 3);
}

#[test]
fn rb_minimum_maximum() {
    let mut bst = BSTreeRB::new();
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);

    bst.insert(50);
    assert_eq!(bst.minimum().copied(), Some(50));
    assert_eq!(bst.maximum().copied(), Some(50));

    bst.insert(30);
    bst.insert(70);
    assert_eq!(bst.minimum().copied(), Some(30));
    assert_eq!(bst.maximum().copied(), Some(70));

    bst.insert(20);
    bst.insert(80);
    assert_eq!(bst.minimum().copied(), Some(20));
    assert_eq!(bst.maximum().copied(), Some(80));
}

#[test]
fn rb_in_order_traversal() {
    let mut bst = BSTreeRB::new();
    bst.insert(50);
    bst.insert(30);
    bst.insert(70);
    bst.insert(20);
    bst.insert(40);
    bst.insert(60);
    bst.insert(80);

    let seq = bst.in_order();
    let mut values = vec![];
    for i in 0..seq.length() {
        values.push(*seq.nth(i));
    }
    assert_eq!(values, vec![20, 30, 40, 50, 60, 70, 80]);
}

#[test]
fn rb_pre_order_traversal() {
    let mut bst = BSTreeRB::new();
    bst.insert(50);
    bst.insert(30);
    bst.insert(70);
    bst.insert(20);
    bst.insert(40);
    bst.insert(60);
    bst.insert(80);

    let seq = bst.pre_order();
    let mut values = vec![];
    for i in 0..seq.length() {
        values.push(*seq.nth(i));
    }
    assert_eq!(values[0], 50); // Root should be first
    assert!(values.contains(&30));
    assert!(values.contains(&70));
    assert_eq!(values.len(), 7);
}

#[test]
fn rb_balancing_sorted_input() {
    let mut bst = BSTreeRB::new();

    // Insert in sorted order - would create unbalanced tree without balancing
    for i in 1..=15 {
        bst.insert(i);
    }

    // Height should be logarithmic, not linear
    let height = bst.height();
    assert!(height <= 6); // log2(15) ≈ 4, RB trees can be slightly taller

    let seq = bst.in_order();
    for i in 0..15 {
        assert_eq!(*seq.nth(i), (i + 1) as i32);
    }
}

#[test]
fn rb_balancing_reverse_sorted() {
    let mut bst = BSTreeRB::new();

    // Insert in reverse sorted order
    for i in (1..=15).rev() {
        bst.insert(i);
    }

    let height = bst.height();
    assert!(height <= 6);

    let seq = bst.in_order();
    for i in 0..15 {
        assert_eq!(*seq.nth(i), (i + 1) as i32);
    }
}

#[test]
fn rb_empty_operations() {
    let bst: BSTRBStEph<i32> = BSTreeRB::new();

    assert_eq!(bst.find(&10), None);
    assert!(!bst.contains(&10));
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);
    assert_eq!(bst.height(), 0);

    let in_order = bst.in_order();
    assert_eq!(in_order.length(), 0);

    let pre_order = bst.pre_order();
    assert_eq!(pre_order.length(), 0);
}

#[test]
fn rb_single_element() {
    let mut bst = BSTreeRB::new();
    bst.insert(42);

    assert_eq!(bst.size(), 1);
    assert!(!bst.is_empty());
    assert_eq!(bst.height(), 1);
    assert_eq!(bst.minimum().copied(), Some(42));
    assert_eq!(bst.maximum().copied(), Some(42));
    assert_eq!(bst.find(&42), Some(&42));
    assert!(bst.contains(&42));

    let in_order = bst.in_order();
    assert_eq!(in_order.length(), 1);
    assert_eq!(*in_order.nth(0), 42);
}

#[test]
fn rb_large_tree() {
    let mut bst = BSTreeRB::new();

    for i in 1..=100 {
        bst.insert(i);
    }

    assert_eq!(bst.size(), 100);
    assert!(bst.height() <= 10); // log2(100) ≈ 6.6, RB can be up to 2*log2(n+1)

    for i in 1..=100 {
        assert!(bst.contains(&i));
        assert_eq!(bst.find(&i), Some(&i));
    }

    assert_eq!(bst.minimum().copied(), Some(1));
    assert_eq!(bst.maximum().copied(), Some(100));
}
