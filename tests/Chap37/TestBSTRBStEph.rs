// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTRBStEph.

use apas_verus::BSTRBStEphLit;
use apas_verus::Chap37::BSTRBStEph::BSTRBStEph::*;

#[test]
fn test_bstrbstephlit_macro_empty() {
    let empty: BSTRBStEph<i32> = BSTRBStEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());
}

#[test]
fn test_bstrbstephlit_macro_with_elements() {
    let tree: BSTRBStEph<i32> = BSTRBStEphLit![5, 3, 7, 1, 9];
    assert_eq!(tree.size(), 5);
    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_rb_new_empty() {
    let tree = BSTRBStEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_rb_insert_and_find() {
    let tree = BSTRBStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(3);
    let tree = tree.insert(7);

    assert_eq!(tree.find(&5), Some(&5));
    assert_eq!(tree.find(&3), Some(&3));
    assert_eq!(tree.find(&7), Some(&7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_rb_contains() {
    let tree = BSTRBStEph::new();
    let tree = tree.insert(10);
    let tree = tree.insert(5);
    let tree = tree.insert(15);

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_rb_height() {
    let tree = BSTRBStEph::<i32>::new();
    assert_eq!(tree.height(), 0);

    let tree = tree.insert(5);
    assert!(tree.height() >= 1);

    let tree = tree.insert(3);
    let tree = tree.insert(7);
    assert!(tree.height() >= 2);
}

#[test]
fn test_rb_size() {
    let tree = BSTRBStEph::<i32>::new();
    assert_eq!(tree.size(), 0);

    let tree = tree.insert(1);
    assert_eq!(tree.size(), 1);

    let tree = tree.insert(2);
    let tree = tree.insert(3);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_rb_duplicate_insert() {
    let tree = BSTRBStEph::new();
    let tree = tree.insert(5);
    let tree = tree.insert(5);

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_rb_balancing_ascending() {
    let mut tree = BSTRBStEph::new();
    for i in 1..=20 {
        tree = tree.insert(i);
    }
    assert_eq!(tree.size(), 20);
    assert!(bst_balance_check::height_within_two_lg(tree.size(), tree.height()));
}

// Balance workloads: the red-black shape is checked after every operation from
// the pre-order traversal alone (tests/Chap37/bst_balance_check.rs).

mod bst_balance_check;
use bst_balance_check::*;

/// Checks the shape of `tree` against its size, panicking with `context` on failure.
fn st_check(tree: &BSTRBStEph<i64>, context: &str) -> Shape {
    let shape = check_red_black(&tree.pre_order())
        .unwrap_or_else(|e| panic!("{}: {}", context, e));
    assert_eq!(shape.size, tree.size(), "{}: size", context);
    assert!(height_within_two_lg(tree.size(), tree.height()), "{}: height", context);
    shape
}

/// Inserts `keys` one at a time into `tree`, checking the shape after each insert.
/// Returns the tree and the maximum height observed.
fn st_insert_checked(mut tree: BSTRBStEph<i64>, keys: &[i64]) -> (BSTRBStEph<i64>, usize) {
    let mut max_height = 0;
    for &k in keys {
        tree = tree.insert(k);
        assert!(tree.contains(&k));
        let shape = st_check(&tree, &format!("after insert {}", k));
        max_height = max_height.max(shape.height);
    }
    (tree, max_height)
}

/// Deletes `keys` one at a time from `tree`, checking the shape, the size, and
/// the absence of the key after each delete.
fn st_delete_checked(mut tree: BSTRBStEph<i64>, keys: &[i64]) -> BSTRBStEph<i64> {
    for &k in keys {
        let before = tree.size();
        let present = tree.contains(&k);
        tree = tree.delete(&k);
        assert!(!tree.contains(&k), "after delete {}: still present", k);
        assert_eq!(tree.size() + present as usize, before, "after delete {}: size", k);
        st_check(&tree, &format!("after delete {}", k));
    }
    tree
}

#[test]
fn test_rb_balance_ascending() { st_insert_checked(BSTRBStEph::new(), &ascending(2000)); }

#[test]
fn test_rb_balance_descending() { st_insert_checked(BSTRBStEph::new(), &descending(2000)); }

#[test]
fn test_rb_balance_zigzag() { st_insert_checked(BSTRBStEph::new(), &zigzag(2000)); }

#[test]
fn test_rb_balance_random() {
    for seed in 1..=4 { st_insert_checked(BSTRBStEph::new(), &random_permutation(2000, seed)); }
}

#[test]
fn test_rb_balance_duplicates() {
    st_insert_checked(BSTRBStEph::new(), &random_keys(2000, 100, 7));
}

#[test]
fn test_rb_delete_ascending_to_empty() {
    let (tree, _) = st_insert_checked(BSTRBStEph::new(), &random_permutation(1000, 3));
    let tree = st_delete_checked(tree, &ascending(1000));
    assert!(tree.is_empty());
}

#[test]
fn test_rb_delete_descending_to_empty() {
    let (tree, _) = st_insert_checked(BSTRBStEph::new(), &ascending(1000));
    let tree = st_delete_checked(tree, &descending(1000));
    assert!(tree.is_empty());
}

#[test]
fn test_rb_delete_random_to_empty() {
    for seed in 1..=3 {
        let (tree, _) = st_insert_checked(BSTRBStEph::new(), &ascending(1000));
        let tree = st_delete_checked(tree, &random_permutation(1000, seed + 10));
        assert!(tree.is_empty());
    }
}

#[test]
fn test_rb_delete_absent_keys() {
    let evens: Vec<i64> = (0..500).map(|i| 2 * i).collect();
    let odds: Vec<i64> = (0..500).map(|i| 2 * i + 1).collect();
    let (tree, _) = st_insert_checked(BSTRBStEph::new(), &evens);
    let tree = st_delete_checked(tree, &odds);
    assert_eq!(tree.size(), 500);
    assert_eq!(tree.in_order(), evens);
}

#[test]
fn test_rb_insert_delete_interleaved() {
    let mut rng = SeededRng::new(99);
    let mut tree = BSTRBStEph::<i64>::new();
    let mut model = std::collections::BTreeSet::new();
    for step in 0..6000 {
        let k = rng.below(400) as i64;
        if rng.below(3) == 0 {
            tree = tree.delete(&k);
            model.remove(&k);
        } else {
            tree = tree.insert(k);
            model.insert(k);
        }
        st_check(&tree, &format!("step {}", step));
        assert_eq!(tree.size(), model.len());
    }
    assert_eq!(tree.in_order(), model.into_iter().collect::<Vec<_>>());
}

/// Prints max height per workload (run with --no-capture for the report).
#[test]
fn test_rb_balance_report() {
    for (name, keys) in [
        ("ascending", ascending(2000)),
        ("descending", descending(2000)),
        ("zigzag", zigzag(2000)),
        ("random", random_permutation(2000, 1)),
    ] {
        let (_, h) = st_insert_checked(BSTRBStEph::new(), &keys);
        println!("StEph {:<11} n=2000 max height {}", name, h);
    }
}

#[test]
fn test_rb_minimum_maximum() {
    let tree = BSTRBStEph::<i64>::new();
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
    let (mut tree, _) = st_insert_checked(BSTRBStEph::new(), &random_permutation(300, 5));
    assert_eq!(tree.minimum(), Some(&0));
    assert_eq!(tree.maximum(), Some(&299));
    for k in 0..299 {
        tree = tree.delete(&k);
        assert_eq!(tree.minimum(), Some(&(k + 1)));
        assert_eq!(tree.maximum(), Some(&299));
    }
}
