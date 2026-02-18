//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTRBStEph.

use apas_verus::BSTRBStEphLit;
use apas_verus::Chap37::BSTRBStEph::BSTRBStEph::*;

#[test]
fn test_bstrbstephlit_macro_empty() {
    let empty: BSTRBStEph<i32> = BSTRBStEphLit![];
    assert_eq!(rb_size(&empty), 0);
    assert!(rb_is_empty(&empty));
}

#[test]
fn test_bstrbstephlit_macro_with_elements() {
    let tree: BSTRBStEph<i32> = BSTRBStEphLit![5, 3, 7, 1, 9];
    assert_eq!(rb_size(&tree), 5);
    assert!(rb_contains(&tree, &5));
    assert!(rb_contains(&tree, &3));
    assert!(rb_contains(&tree, &7));
    assert!(!rb_contains(&tree, &10));
}

#[test]
fn test_rb_new_empty() {
    let tree = rb_new::<i32>();
    assert_eq!(rb_size(&tree), 0);
    assert!(rb_is_empty(&tree));
}

#[test]
fn test_rb_insert_and_find() {
    let tree = rb_new();
    let tree = rb_insert(tree, 5);
    let tree = rb_insert(tree, 3);
    let tree = rb_insert(tree, 7);

    assert_eq!(rb_find(&tree, &5), Some(&5));
    assert_eq!(rb_find(&tree, &3), Some(&3));
    assert_eq!(rb_find(&tree, &7), Some(&7));
    assert_eq!(rb_find(&tree, &10), None);
}

#[test]
fn test_rb_contains() {
    let tree = rb_new();
    let tree = rb_insert(tree, 10);
    let tree = rb_insert(tree, 5);
    let tree = rb_insert(tree, 15);

    assert!(rb_contains(&tree, &10));
    assert!(rb_contains(&tree, &5));
    assert!(rb_contains(&tree, &15));
    assert!(!rb_contains(&tree, &20));
}

#[test]
fn test_rb_height() {
    let tree = rb_new::<i32>();
    assert_eq!(rb_height(&tree), 0);

    let tree = rb_insert(tree, 5);
    assert!(rb_height(&tree) >= 1);

    let tree = rb_insert(tree, 3);
    let tree = rb_insert(tree, 7);
    assert!(rb_height(&tree) >= 2);
}

#[test]
fn test_rb_size() {
    let tree = rb_new::<i32>();
    assert_eq!(rb_size(&tree), 0);

    let tree = rb_insert(tree, 1);
    assert_eq!(rb_size(&tree), 1);

    let tree = rb_insert(tree, 2);
    let tree = rb_insert(tree, 3);
    assert_eq!(rb_size(&tree), 3);
}

#[test]
fn test_rb_duplicate_insert() {
    let tree = rb_new();
    let tree = rb_insert(tree, 5);
    let tree = rb_insert(tree, 5);

    assert_eq!(rb_size(&tree), 1);
    assert!(rb_contains(&tree, &5));
}

#[test]
fn test_rb_balancing_ascending() {
    let mut tree = rb_new();
    for i in 1..=20 {
        tree = rb_insert(tree, i);
    }
    assert_eq!(rb_size(&tree), 20);
    assert!(rb_height(&tree) <= 20);
}
