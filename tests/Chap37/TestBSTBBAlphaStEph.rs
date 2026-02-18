//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTBBAlphaStEph.

use apas_verus::BSTBBAlphaStEphLit;
use apas_verus::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;

#[test]
fn test_bstbbalphastephlit_macro_empty() {
    let empty: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![];
    assert_eq!(bb_size(&empty), 0);
    assert!(bb_is_empty(&empty));
}

#[test]
fn test_bstbbalphastephlit_macro_with_elements() {
    let tree: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![5, 3, 7, 1, 9];
    assert_eq!(bb_size(&tree), 5);
    assert!(bb_contains(&tree, &5));
    assert!(bb_contains(&tree, &3));
    assert!(bb_contains(&tree, &7));
    assert!(!bb_contains(&tree, &10));
}

#[test]
fn test_bb_new_empty() {
    let tree = bb_new::<i32>();
    assert_eq!(bb_size(&tree), 0);
    assert!(bb_is_empty(&tree));
}

#[test]
fn test_bb_insert_and_find() {
    let tree = bb_new();
    let tree = bb_insert(tree, 5);
    let tree = bb_insert(tree, 3);
    let tree = bb_insert(tree, 7);

    assert_eq!(bb_find(&tree, &5), Some(&5));
    assert_eq!(bb_find(&tree, &3), Some(&3));
    assert_eq!(bb_find(&tree, &7), Some(&7));
    assert_eq!(bb_find(&tree, &10), None);
}

#[test]
fn test_bb_contains() {
    let tree = bb_new();
    let tree = bb_insert(tree, 10);
    let tree = bb_insert(tree, 5);
    let tree = bb_insert(tree, 15);

    assert!(bb_contains(&tree, &10));
    assert!(bb_contains(&tree, &5));
    assert!(bb_contains(&tree, &15));
    assert!(!bb_contains(&tree, &20));
}

#[test]
fn test_bb_height() {
    let tree = bb_new::<i32>();
    assert_eq!(bb_height(&tree), 0);

    let tree = bb_insert(tree, 5);
    assert!(bb_height(&tree) >= 1);

    let tree = bb_insert(tree, 3);
    let tree = bb_insert(tree, 7);
    assert!(bb_height(&tree) >= 2);
}

#[test]
fn test_bb_size() {
    let tree = bb_new::<i32>();
    assert_eq!(bb_size(&tree), 0);

    let tree = bb_insert(tree, 1);
    assert_eq!(bb_size(&tree), 1);

    let tree = bb_insert(tree, 2);
    let tree = bb_insert(tree, 3);
    assert_eq!(bb_size(&tree), 3);
}

#[test]
fn test_bb_duplicate_insert() {
    let tree = bb_new();
    let tree = bb_insert(tree, 5);
    let tree = bb_insert(tree, 5);

    assert_eq!(bb_size(&tree), 1);
    assert!(bb_contains(&tree, &5));
}

#[test]
fn test_bb_balancing_ascending() {
    let mut tree = bb_new();
    for i in 1..=64 {
        tree = bb_insert(tree, i);
    }
    assert_eq!(bb_size(&tree), 64);
    assert!(bb_height(&tree) <= 64);
}
