//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::BSTSizeStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap40::BSTSizeStEph::BSTSizeStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn size_bst_basic_operations() {
    let mut bst = BSTreeSize::new();
    assert!(bst.is_empty());
    assert_eq!(bst.size(), 0);

    // Insert elements
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(9);

    assert_eq!(bst.size(), 5);
    assert!(!bst.is_empty());

    // Test find operations
    assert_eq!(bst.find(&5), Some(&5));
    assert_eq!(bst.find(&10), None);

    // Test contains
    assert!(bst.contains(&7));
    assert!(!bst.contains(&10));

    // Test min/max
    assert_eq!(bst.minimum(), Some(&1));
    assert_eq!(bst.maximum(), Some(&9));
}

#[test]
fn size_bst_rank_operations() {
    let mut bst = BSTreeSize::new();
    let values = vec![5, 3, 7, 1, 9, 2, 6, 8, 4];

    for val in values {
        bst.insert(val);
    }

    // Test rank function (number of elements <= key)
    assert_eq!(bst.rank(&0), 0); // No elements <= 0
    assert_eq!(bst.rank(&1), 1); // 1 element <= 1: {1}
    assert_eq!(bst.rank(&2), 2); // 2 elements <= 2: {1, 2}
    assert_eq!(bst.rank(&3), 3); // 3 elements <= 3: {1, 2, 3}
    assert_eq!(bst.rank(&4), 4); // 4 elements <= 4: {1, 2, 3, 4}
    assert_eq!(bst.rank(&5), 5); // 5 elements <= 5: {1, 2, 3, 4, 5}
    assert_eq!(bst.rank(&6), 6); // 6 elements <= 6: {1, 2, 3, 4, 5, 6}
    assert_eq!(bst.rank(&7), 7); // 7 elements <= 7: {1, 2, 3, 4, 5, 6, 7}
    assert_eq!(bst.rank(&8), 8); // 8 elements <= 8: {1, 2, 3, 4, 5, 6, 7, 8}
    assert_eq!(bst.rank(&9), 9); // 9 elements <= 9: all elements
    assert_eq!(bst.rank(&10), 9); // 9 elements <= 10: all elements
}

#[test]
fn size_bst_select_operations() {
    let mut bst = BSTreeSize::new();
    let values = vec![5, 3, 7, 1, 9, 2, 6, 8, 4];

    for val in values {
        bst.insert(val);
    }

    // Test select function (1-indexed: get element with rank i)
    assert_eq!(bst.select(0), None); // Invalid rank
    assert_eq!(bst.select(1), Some(&1)); // 1st smallest: 1
    assert_eq!(bst.select(2), Some(&2)); // 2nd smallest: 2
    assert_eq!(bst.select(3), Some(&3)); // 3rd smallest: 3
    assert_eq!(bst.select(4), Some(&4)); // 4th smallest: 4
    assert_eq!(bst.select(5), Some(&5)); // 5th smallest: 5
    assert_eq!(bst.select(6), Some(&6)); // 6th smallest: 6
    assert_eq!(bst.select(7), Some(&7)); // 7th smallest: 7
    assert_eq!(bst.select(8), Some(&8)); // 8th smallest: 8
    assert_eq!(bst.select(9), Some(&9)); // 9th smallest: 9
    assert_eq!(bst.select(10), None); // Invalid rank
}

#[test]
fn size_bst_rank_select_consistency() {
    let mut bst = BSTreeSize::new();
    let values = vec![10, 5, 15, 3, 7, 12, 18, 1, 4, 6, 8, 11, 13, 16, 20];

    for val in values {
        bst.insert(val);
    }

    // For each element, rank(select(i)) should equal i
    for i in 1..=bst.size() {
        if let Some(key) = bst.select(i) {
            assert_eq!(bst.rank(key), i, "Rank-select inconsistency at rank {i}");
        }
    }

    // For each key, select(rank(key)) should equal key
    let in_order = bst.in_order();
    for i in 0..in_order.length() {
        let key = in_order.nth(i);
        let rank = bst.rank(key);
        assert_eq!(bst.select(rank), Some(key), "Select-rank inconsistency for key {key}");
    }
}

#[test]
fn size_bst_split_rank_operations() {
    let mut bst = BSTreeSize::new();
    let values = vec![5, 3, 7, 1, 9, 2, 6, 8, 4];

    for val in values {
        bst.insert(val);
    }

    // Split at rank 5: implementation puts ranks 1-5 in left, ranks 6-9 in right
    let (left, right) = bst.split_rank(5);

    assert_eq!(left.size(), 5);
    assert_eq!(right.size(), 4);

    // Check left tree contains {1,2,3,4,5}
    for i in 1..=5 {
        assert!(left.contains(&i), "Left tree missing {i}");
    }
    assert!(!left.contains(&6));

    // Check right tree contains {6,7,8,9}
    for i in 6..=9 {
        assert!(right.contains(&i), "Right tree missing {i}");
    }
    assert!(!right.contains(&5));

    // Test edge cases
    let (empty_left, all_right) = bst.split_rank(0);
    assert_eq!(empty_left.size(), 0);
    assert_eq!(all_right.size(), 9);

    let (all_left, empty_right) = bst.split_rank(10);
    assert_eq!(all_left.size(), 9);
    assert_eq!(empty_right.size(), 0);
}

#[test]
fn size_bst_macro_literal() {
    // Test empty macro
    let empty_bst: BSTSizeStEph<i32> = BSTSizeStEphLit![];
    assert!(empty_bst.is_empty());

    // Test non-empty macro
    let bst = BSTSizeStEphLit![5, 3, 7, 1, 9];
    assert_eq!(bst.size(), 5);
    // Elements are {1,3,5,7,9}, so rank(&5) = number of elements <= 5 = {1,3,5} = 3
    assert_eq!(bst.rank(&5), 3);
    assert_eq!(bst.select(3), Some(&5));
}

#[test]
fn size_bst_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeSize::new();
    bst.insert(5);
    bst.insert(5); // Duplicate

    assert_eq!(bst.size(), 1);
    assert_eq!(bst.rank(&5), 1);
    assert_eq!(bst.select(1), Some(&5));
}

#[test]
fn size_bst_large_dataset_performance() {
    let mut bst = BSTreeSize::new();

    // Insert 1000 elements
    for i in 0..1000 {
        bst.insert(i);
    }

    assert_eq!(bst.size(), 1000);

    // Test rank and select operations
    assert_eq!(bst.rank(&500), 501); // 501 elements <= 500: {0,1,2,...,500}
    assert_eq!(bst.select(501), Some(&500)); // 501st element is 500

    // Height should be logarithmic (allow some slack for unbalanced insertions)
    let height = bst.height();
    assert!(height <= 30, "Height {height} too large for 1000 elements");

    // Test split at middle
    let (left, right) = bst.split_rank(500);
    assert_eq!(left.size(), 500);
    assert_eq!(right.size(), 500);
}
