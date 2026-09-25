// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for BSTRBMtEph.

use apas_verus::BSTRBMtEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap37::BSTRBMtEph::BSTRBMtEph::*;

#[test]
fn test_bstrbmtephlit_macro_functionality() {
    let empty: BSTRBMtEph<i32> = BSTRBMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTRBMtEph<i32> = BSTRBMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree = BSTRBMtEph::<i32>::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_find() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&7), Some(7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_contains() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(10).unwrap();
    tree.insert(5).unwrap();
    tree.insert(15).unwrap();

    assert!(tree.contains(&10));
    assert!(tree.contains(&5));
    assert!(tree.contains(&15));
    assert!(!tree.contains(&20));
}

#[test]
fn test_minimum_maximum() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    tree.insert(1).unwrap();
    tree.insert(9).unwrap();

    assert_eq!(tree.minimum(), Some(1));
    assert_eq!(tree.maximum(), Some(9));
}

#[test]
fn test_height() {
    let mut tree = BSTRBMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5).unwrap();
    assert!(tree.height() >= 1);

    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    assert!(bst_balance_check::height_within_two_lg(tree.size(), tree.height()));
}

#[test]
fn test_in_order_traversal() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();
    tree.insert(1).unwrap();
    tree.insert(9).unwrap();

    let values = tree.in_order();
    assert_eq!(values.length(), 5);
}

#[test]
fn test_pre_order_traversal() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(7).unwrap();

    let values = tree.pre_order();
    assert_eq!(values.length(), 3);
}

#[test]
fn test_size() {
    let mut tree = BSTRBMtEph::new();
    assert_eq!(tree.size(), 0);

    tree.insert(1).unwrap();
    assert_eq!(tree.size(), 1);

    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_is_empty() {
    let mut tree = BSTRBMtEph::<i32>::new();
    assert!(tree.is_empty());

    tree.insert(5).unwrap();
    assert!(!tree.is_empty());
}

#[test]
fn test_rb_balancing() {
    let mut tree = BSTRBMtEph::new();
    for i in 1..=15 {
        tree.insert(i).unwrap();
    }

    let height = tree.height();
    assert!(bst_balance_check::height_within_two_lg(tree.size(), height));
    assert_eq!(tree.size(), 15);
}

#[test]
fn test_duplicate_insert() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(5).unwrap();

    assert_eq!(tree.size(), 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_default() {
    let tree = BSTRBMtEph::<i32>::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_from_sorted_slice() {
    let values = vec![1, 2, 3, 4, 5, 6, 7];
    let tree = BSTRBMtEph::from_sorted_slice(&values);

    assert_eq!(tree.size(), 7);
    assert!(tree.contains(&1));
    assert!(tree.contains(&4));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));

    let height = tree.height();
    assert!(bst_balance_check::height_within_two_lg(tree.size(), height));
}

#[test]
fn test_from_sorted_slice_empty() {
    let values: Vec<i32> = vec![];
    let tree = BSTRBMtEph::from_sorted_slice(&values);

    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_filter() {
    let mut tree = BSTRBMtEph::new();
    for i in 1..=10 {
        tree.insert(i).unwrap();
    }

    let evens = tree.filter(|&x| x % 2 == 0);
    assert_eq!(evens.length(), 5);
}

#[test]
fn test_filter_all() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(1).unwrap();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();

    let all = tree.filter(|_| true);
    assert_eq!(all.length(), 3);
}

#[test]
fn test_filter_none() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(1).unwrap();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();

    let none = tree.filter(|_| false);
    assert_eq!(none.length(), 0);
}

#[test]
fn test_reduce_sum() {
    let mut tree = BSTRBMtEph::new();
    for i in 1..=10 {
        tree.insert(i).unwrap();
    }

    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 55);
}

#[test]
fn test_reduce_product() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(2).unwrap();
    tree.insert(3).unwrap();
    tree.insert(4).unwrap();

    let product = tree.reduce(|a, b| a * b, 1);
    assert_eq!(product, 24);
}

#[test]
fn test_reduce_max() {
    let mut tree = BSTRBMtEph::new();
    tree.insert(5).unwrap();
    tree.insert(3).unwrap();
    tree.insert(9).unwrap();
    tree.insert(1).unwrap();
    tree.insert(7).unwrap();

    let max = tree.reduce(|a, b| if a > b { a } else { b }, i32::MIN);
    assert_eq!(max, 9);
}

#[test]
fn test_reduce_empty() {
    let tree = BSTRBMtEph::<i32>::new();

    let sum = tree.reduce(|a, b| a + b, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_trait_new() {
    let tree = <BSTRBMtEph<i32> as BSTRBMtEphTrait<i32>>::new();
    assert!(tree.is_empty());
}

#[test]
fn test_trait_from_sorted() {
    let tree = <BSTRBMtEph<i32> as BSTRBMtEphTrait<i32>>::from_sorted_slice(&[1, 2, 3]);
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_trait_methods_direct() {
    let mut tree = <BSTRBMtEph<i32> as BSTRBMtEphTrait<i32>>::new();
    tree.insert(10).unwrap();
    assert!(tree.contains(&10));
}

// Balance workloads: the red-black shape is checked after every operation from
// the pre-order traversal alone (tests/Chap37/bst_balance_check.rs).

mod bst_balance_check;
use bst_balance_check::*;

fn mt_pre_order(tree: &BSTRBMtEph<i64>) -> Vec<i64> { tree.pre_order().seq }

/// Inserts `keys` one at a time, checking the shape after each insert.
/// Returns the maximum height observed.
fn mt_insert_checked(keys: &[i64]) -> usize {
    let mut tree = BSTRBMtEph::<i64>::new();
    let mut max_height = 0;
    for &k in keys {
        tree.insert(k).unwrap();
        let shape = check_red_black(&mt_pre_order(&tree))
            .unwrap_or_else(|e| panic!("after insert {}: {}", k, e));
        assert_eq!(shape.size, tree.size());
        max_height = max_height.max(shape.height);
    }
    max_height
}

#[test]
fn test_rb_balance_ascending() { mt_insert_checked(&ascending(500)); }

#[test]
fn test_rb_balance_descending() { mt_insert_checked(&descending(500)); }

#[test]
fn test_rb_balance_zigzag() { mt_insert_checked(&zigzag(500)); }

#[test]
fn test_rb_balance_random() {
    for seed in 1..=4 { mt_insert_checked(&random_permutation(500, seed)); }
}

#[test]
fn test_rb_balance_duplicates() {
    mt_insert_checked(&random_keys(500, 50, 7));
}

#[test]
fn test_rb_balance_from_sorted_slice() {
    for n in 0..=300 {
        let keys = ascending(n);
        let tree = BSTRBMtEph::from_sorted_slice(&keys);
        let shape = check_red_black(&mt_pre_order(&tree))
            .unwrap_or_else(|e| panic!("from_sorted_slice({}): {}", n, e));
        assert_eq!(shape.size, n as usize);
        assert_eq!(tree.size(), n as usize);
    }
}

/// Prints max height per workload (run with --no-capture for the report).
#[test]
fn test_rb_balance_report() {
    for (name, keys) in [
        ("ascending", ascending(500)),
        ("descending", descending(500)),
        ("zigzag", zigzag(500)),
        ("random", random_permutation(500, 1)),
    ] {
        let h = mt_insert_checked(&keys);
        println!("MtEph {:<11} n=500 max height {}", name, h);
    }
}

/// Checks the red-black shape, the size, and the height bound of `tree`.
fn mt_check(tree: &BSTRBMtEph<i64>, context: &str) {
    let shape = check_red_black(&mt_pre_order(tree))
        .unwrap_or_else(|e| panic!("{}: {}", context, e));
    assert_eq!(shape.size, tree.size(), "{}: size", context);
    assert_eq!(shape.height, tree.height(), "{}: height", context);
    assert!(height_within_two_lg(tree.size(), tree.height()), "{}: height bound", context);
}

/// Builds a tree from `keys` by insertion.
fn mt_build(keys: &[i64]) -> BSTRBMtEph<i64> {
    let mut tree = BSTRBMtEph::<i64>::new();
    for &k in keys { tree.insert(k).unwrap(); }
    tree
}

/// Deletes `keys` one at a time, checking the shape, the size, and the absence of the key.
fn mt_delete_checked(tree: &mut BSTRBMtEph<i64>, keys: &[i64]) {
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
fn test_rb_mt_delete_ascending_to_empty() {
    let mut tree = mt_build(&random_permutation(500, 3));
    mt_delete_checked(&mut tree, &ascending(500));
    assert!(tree.is_empty());
}

#[test]
fn test_rb_mt_delete_descending_to_empty() {
    let mut tree = mt_build(&ascending(500));
    mt_delete_checked(&mut tree, &descending(500));
    assert!(tree.is_empty());
}

#[test]
fn test_rb_mt_delete_random_to_empty() {
    for seed in 1..=3 {
        let mut tree = mt_build(&ascending(500));
        mt_delete_checked(&mut tree, &random_permutation(500, seed + 10));
        assert!(tree.is_empty());
    }
}

#[test]
fn test_rb_mt_delete_absent_keys() {
    let evens: Vec<i64> = (0..250).map(|i| 2 * i).collect();
    let odds: Vec<i64> = (0..250).map(|i| 2 * i + 1).collect();
    let mut tree = mt_build(&evens);
    mt_delete_checked(&mut tree, &odds);
    assert_eq!(tree.size(), 250);
    assert_eq!(tree.in_order().seq, evens);
}

#[test]
fn test_rb_mt_delete_from_empty() {
    let mut tree = BSTRBMtEph::<i64>::new();
    mt_delete_checked(&mut tree, &[0, 1, -1]);
    assert!(tree.is_empty());
}

#[test]
fn test_rb_mt_insert_delete_interleaved() {
    let mut rng = SeededRng::new(99);
    let mut tree = BSTRBMtEph::<i64>::new();
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
