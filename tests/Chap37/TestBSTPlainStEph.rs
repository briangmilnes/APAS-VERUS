//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTPlainStEph.

use apas_verus::BSTPlainStEphLit;
use apas_verus::Chap37::BSTPlainStEph::BSTPlainStEph::*;

#[test]
fn test_bstplainstephlit_macro_empty() {
    let empty: BSTPlainStEph<i32> = BSTPlainStEphLit![];
    assert_eq!(bst_size(&empty), 0);
    assert!(bst_is_empty(&empty));
}

#[test]
fn test_bstplainstephlit_macro_with_elements() {
    let tree: BSTPlainStEph<i32> = BSTPlainStEphLit![5, 3, 7, 1, 9];
    assert_eq!(bst_size(&tree), 5);
    assert!(bst_contains(&tree, &5));
    assert!(bst_contains(&tree, &3));
    assert!(bst_contains(&tree, &7));
    assert!(!bst_contains(&tree, &10));
}

#[test]
fn test_new_empty() {
    let tree = bst_new::<i32>();
    assert_eq!(bst_size(&tree), 0);
    assert!(bst_is_empty(&tree));
}

#[test]
fn test_insert_and_find() {
    let tree = bst_new();
    let tree = bst_insert(tree, 5);
    let tree = bst_insert(tree, 3);
    let tree = bst_insert(tree, 7);

    assert_eq!(bst_find(&tree, &5), Some(&5));
    assert_eq!(bst_find(&tree, &3), Some(&3));
    assert_eq!(bst_find(&tree, &7), Some(&7));
    assert_eq!(bst_find(&tree, &10), None);
}

#[test]
fn test_contains() {
    let tree = bst_new();
    let tree = bst_insert(tree, 10);
    let tree = bst_insert(tree, 5);
    let tree = bst_insert(tree, 15);

    assert!(bst_contains(&tree, &10));
    assert!(bst_contains(&tree, &5));
    assert!(bst_contains(&tree, &15));
    assert!(!bst_contains(&tree, &20));
}

#[test]
fn test_height() {
    let tree = bst_new::<i32>();
    assert_eq!(bst_height(&tree), 0);

    let tree = bst_insert(tree, 5);
    assert!(bst_height(&tree) >= 1);

    let tree = bst_insert(tree, 3);
    let tree = bst_insert(tree, 7);
    assert!(bst_height(&tree) >= 2);
}

#[test]
fn test_size() {
    let tree = bst_new::<i32>();
    assert_eq!(bst_size(&tree), 0);

    let tree = bst_insert(tree, 1);
    assert_eq!(bst_size(&tree), 1);

    let tree = bst_insert(tree, 2);
    let tree = bst_insert(tree, 3);
    assert_eq!(bst_size(&tree), 3);
}

#[test]
fn test_is_empty() {
    let tree = bst_new::<i32>();
    assert!(bst_is_empty(&tree));

    let tree = bst_insert(tree, 5);
    assert!(!bst_is_empty(&tree));
}

#[test]
fn test_duplicate_insert() {
    let tree = bst_new();
    let tree = bst_insert(tree, 5);
    let tree = bst_insert(tree, 5);

    assert_eq!(bst_size(&tree), 1);
    assert!(bst_contains(&tree, &5));
}
