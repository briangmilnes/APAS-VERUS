//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTAVLStEph.

use apas_verus::BSTAVLStEphLit;
use apas_verus::Chap37::BSTAVLStEph::BSTAVLStEph::*;

#[test]
fn test_bstavlstephlit_macro_empty() {
    let empty: BSTAVLStEph<i32> = BSTAVLStEphLit![];
    assert_eq!(avl_size(&empty), 0);
    assert!(avl_is_empty(&empty));
}

#[test]
fn test_bstavlstephlit_macro_with_elements() {
    let tree: BSTAVLStEph<i32> = BSTAVLStEphLit![5, 3, 7, 1, 9];
    assert_eq!(avl_size(&tree), 5);
    assert!(avl_contains(&tree, &5));
    assert!(avl_contains(&tree, &3));
    assert!(avl_contains(&tree, &7));
    assert!(!avl_contains(&tree, &10));
}

#[test]
fn test_avl_new_empty() {
    let tree = avl_new::<i32>();
    assert_eq!(avl_size(&tree), 0);
    assert!(avl_is_empty(&tree));
}

#[test]
fn test_avl_insert_and_find() {
    let tree = avl_new();
    let tree = avl_insert(tree, 5);
    let tree = avl_insert(tree, 3);
    let tree = avl_insert(tree, 7);

    assert_eq!(avl_find(&tree, &5), Some(&5));
    assert_eq!(avl_find(&tree, &3), Some(&3));
    assert_eq!(avl_find(&tree, &7), Some(&7));
    assert_eq!(avl_find(&tree, &10), None);
}

#[test]
fn test_avl_contains() {
    let tree = avl_new();
    let tree = avl_insert(tree, 10);
    let tree = avl_insert(tree, 5);
    let tree = avl_insert(tree, 15);

    assert!(avl_contains(&tree, &10));
    assert!(avl_contains(&tree, &5));
    assert!(avl_contains(&tree, &15));
    assert!(!avl_contains(&tree, &20));
}

#[test]
fn test_avl_height() {
    let tree = avl_new::<i32>();
    assert_eq!(avl_height(&tree), 0);

    let tree = avl_insert(tree, 5);
    assert!(avl_height(&tree) >= 1);

    let tree = avl_insert(tree, 3);
    let tree = avl_insert(tree, 7);
    assert!(avl_height(&tree) >= 2);
}

#[test]
fn test_avl_size() {
    let tree = avl_new::<i32>();
    assert_eq!(avl_size(&tree), 0);

    let tree = avl_insert(tree, 1);
    assert_eq!(avl_size(&tree), 1);

    let tree = avl_insert(tree, 2);
    let tree = avl_insert(tree, 3);
    assert_eq!(avl_size(&tree), 3);
}

#[test]
fn test_avl_duplicate_insert() {
    let tree = avl_new();
    let tree = avl_insert(tree, 5);
    let tree = avl_insert(tree, 5);

    assert_eq!(avl_size(&tree), 1);
    assert!(avl_contains(&tree, &5));
}

#[test]
fn test_avl_balancing_ascending() {
    let mut tree = avl_new();
    for i in 1..=20 {
        tree = avl_insert(tree, i);
    }
    assert_eq!(avl_size(&tree), 20);
    assert!(avl_height(&tree) <= 20);
}
