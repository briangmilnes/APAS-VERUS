// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTBBAlphaMtEph.

use apas_verus::BSTBBAlphaMtEphLit;
use apas_verus::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;

#[test]
fn test_bstbbalphamtephlit_macro_functionality() {
    let empty: BSTBBAlphaMtEph<i32> = BSTBBAlphaMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTBBAlphaMtEph<i32> = BSTBBAlphaMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTBBAlphaMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_contains() {
    let mut tree = BSTBBAlphaMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_contains() {
    let mut tree = BSTBBAlphaMtEph::new();
    tree.insert(10).unwrap();
    tree.insert(5).unwrap();
    tree.insert(15).unwrap();

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_height() {
    let mut tree = BSTBBAlphaMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5).unwrap();
    assert!(tree.height() >= 1);

    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    assert!(tree.height() <= 2);
}

#[test]
fn test_size() {
    let mut tree = BSTBBAlphaMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1).unwrap();
    assert_eq!(tree.size(), 1);

    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let mut tree = BSTBBAlphaMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5).unwrap();
    assert!(!tree.is_empty());
}

#[test]
fn test_duplicate_insert() {
    let mut tree = BSTBBAlphaMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(5).unwrap();

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_insert_sequential() {
    let mut tree = BSTBBAlphaMtEph::new();
    for i in 1..=7 {
        tree.insert(i).unwrap();
    }

    assert_eq!(tree.size(), 7);
}

#[test]
fn test_trait_new_direct() {
    let tree = <BSTBBAlphaMtEph<i32> as BSTBBAlphaMtEphTrait<i32>>::new();
    assert!(tree.is_empty());
}

#[test]
fn test_trait_insert_direct() {
    let mut tree = <BSTBBAlphaMtEph<i32> as BSTBBAlphaMtEphTrait<i32>>::new();
    tree.insert(10).unwrap();
    assert!(tree.contains(&10));
}

mod bst_balance_check;
use bst_balance_check::*;

/// Checks weight balance, the size field, and the height bound of `tree`.
fn mt_check(tree: &BSTBBAlphaMtEph<i64>, context: &str) -> Shape {
    let shape = check_weight_balanced(&tree.pre_order().seq)
        .unwrap_or_else(|e| panic!("{}: {}", context, e));
    assert_eq!(shape.size, tree.size(), "{}: size", context);
    assert_eq!(shape.height, tree.height(), "{}: height", context);
    assert!(height_within_wb_bound(tree.size(), tree.height()), "{}: height bound", context);
    shape
}

/// Inserts `keys` one at a time, checking the shape after each insert; returns the maximum height.
fn mt_insert_checked(tree: &mut BSTBBAlphaMtEph<i64>, keys: &[i64]) -> usize {
    let mut max_height = 0;
    for &k in keys {
        tree.insert(k).unwrap();
        assert!(tree.contains(&k));
        let shape = mt_check(tree, &format!("after insert {}", k));
        max_height = max_height.max(shape.height);
    }
    max_height
}

/// Deletes `keys` one at a time, checking the shape, the size, and the absence of the key.
fn mt_delete_checked(tree: &mut BSTBBAlphaMtEph<i64>, keys: &[i64]) {
    for &k in keys {
        let before = tree.size();
        let present = tree.contains(&k);
        tree.delete(&k).unwrap();
        assert!(!tree.contains(&k), "after delete {}: still present", k);
        assert_eq!(tree.size() + present as usize, before, "after delete {}: size", k);
        mt_check(tree, &format!("after delete {}", k));
    }
}

#[test]
fn test_mt_bb_balance_ascending() { mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &ascending(500)); }

#[test]
fn test_mt_bb_balance_descending() { mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &descending(500)); }

#[test]
fn test_mt_bb_balance_zigzag() { mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &zigzag(500)); }

#[test]
fn test_mt_bb_balance_random() {
    for seed in 1..=4 { mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &random_permutation(500, seed)); }
}

#[test]
fn test_mt_bb_balance_duplicates() {
    mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &random_keys(500, 50, 7));
}

#[test]
fn test_mt_bb_delete_ascending_to_empty() {
    let mut tree = BSTBBAlphaMtEph::new();
    mt_insert_checked(&mut tree, &random_permutation(500, 3));
    mt_delete_checked(&mut tree, &ascending(500));
    assert!(tree.is_empty());
}

#[test]
fn test_mt_bb_delete_descending_to_empty() {
    let mut tree = BSTBBAlphaMtEph::new();
    mt_insert_checked(&mut tree, &ascending(500));
    mt_delete_checked(&mut tree, &descending(500));
    assert!(tree.is_empty());
}

#[test]
fn test_mt_bb_delete_random_to_empty() {
    for seed in 1..=3 {
        let mut tree = BSTBBAlphaMtEph::new();
        mt_insert_checked(&mut tree, &ascending(500));
        mt_delete_checked(&mut tree, &random_permutation(500, seed + 10));
        assert!(tree.is_empty());
    }
}

#[test]
fn test_mt_bb_delete_absent_keys() {
    let evens: Vec<i64> = (0..250).map(|i| 2 * i).collect();
    let odds: Vec<i64> = (0..250).map(|i| 2 * i + 1).collect();
    let mut tree = BSTBBAlphaMtEph::new();
    mt_insert_checked(&mut tree, &evens);
    mt_delete_checked(&mut tree, &odds);
    assert_eq!(tree.size(), 250);
    assert_eq!(tree.in_order().seq, evens);
}

#[test]
fn test_mt_bb_insert_delete_interleaved() {
    let mut rng = SeededRng::new(99);
    let mut tree = BSTBBAlphaMtEph::<i64>::new();
    let mut model = std::collections::BTreeSet::new();
    for step in 0..2000 {
        let k = rng.below(200) as i64;
        if rng.below(3) == 0 {
            tree.delete(&k).unwrap();
            model.remove(&k);
        } else {
            tree.insert(k).unwrap();
            model.insert(k);
        }
        mt_check(&tree, &format!("step {}", step));
        assert_eq!(tree.size(), model.len());
    }
    assert_eq!(tree.in_order().seq, model.into_iter().collect::<Vec<_>>());
}

/// Prints max height per workload (run with --no-capture for the report).
#[test]
fn test_mt_bb_balance_report() {
    for (name, keys) in [
        ("ascending", ascending(500)),
        ("descending", descending(500)),
        ("zigzag", zigzag(500)),
        ("random", random_permutation(500, 1)),
    ] {
        let h = mt_insert_checked(&mut BSTBBAlphaMtEph::new(), &keys);
        println!("BB MtEph {:<11} n=500 max height {}", name, h);
    }
}
