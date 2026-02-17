//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStPer.

use apas_verus::ArraySeqStPerSLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap49::SubsetSumStPer::SubsetSumStPer::*;
use apas_verus::SubsetSumStPerLit;

#[test]
fn test_subset_sum_st_per_basic() {
    let solver = SubsetSumStPerLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(1));
    assert!(solver.subset_sum(5));
    assert!(solver.subset_sum(6));
    assert!(solver.subset_sum(16));
    assert!(!solver.subset_sum(17));
}

#[test]
fn test_subset_sum_st_per_example_49_2() {
    let solver = SubsetSumStPerLit![1, 1, 1];
    assert!(solver.subset_sum(3));
    assert!(solver.subset_sum(2));
    assert!(solver.subset_sum(1));
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(4));
}

#[test]
fn test_subset_sum_st_per_empty() {
    let solver: SubsetSumStPerS<i32> = SubsetSumStPerLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_st_per_singleton() {
    let solver = SubsetSumStPerLit![5];
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(5));
    assert!(!solver.subset_sum(1));
    assert!(!solver.subset_sum(10));
}

#[test]
fn test_subset_sum_st_per_large_values() {
    let solver = SubsetSumStPerLit![100, 200, 300];
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(100));
    assert!(solver.subset_sum(300));
    assert!(solver.subset_sum(500));
    assert!(solver.subset_sum(600));
    assert!(!solver.subset_sum(50));
    assert!(!solver.subset_sum(700));
}

#[test]
fn test_subset_sum_st_per_duplicates() {
    let solver = SubsetSumStPerLit![3, 3, 3];
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(3));
    assert!(solver.subset_sum(6));
    assert!(solver.subset_sum(9));
    assert!(!solver.subset_sum(1));
    assert!(!solver.subset_sum(10));
}

#[test]
fn test_subset_sum_st_per_all_combinations() {
    let solver = SubsetSumStPerLit![1, 2, 4];
    // All possible sums: 0, 1, 2, 3(1+2), 4, 5(1+4), 6(2+4), 7(1+2+4)
    for target in 0..=7 {
        assert!(solver.subset_sum(target), "Should find subset sum for {}", target);
    }
    assert!(!solver.subset_sum(8));
    assert!(!solver.subset_sum(9));
}

#[test]
fn test_subset_sum_st_per_new() {
    let solver = SubsetSumStPerS::<i32>::new();
    assert_eq!(solver.multiset().length(), 0);
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_st_per_from_multiset() {
    let multiset = ArraySeqStPerSLit![5, 10, 15];
    let solver = SubsetSumStPerS::from_multiset(multiset);
    assert_eq!(solver.multiset().length(), 3);
    assert!(solver.subset_sum(15));
    assert!(solver.subset_sum(25));
    assert!(!solver.subset_sum(7));
}

#[test]
fn test_subset_sum_st_per_multiset_getter() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    let multiset = solver.multiset();
    assert_eq!(multiset.length(), 3);
    assert_eq!(*multiset.nth(0), 1);
    assert_eq!(*multiset.nth(1), 2);
    assert_eq!(*multiset.nth(2), 3);
}

#[test]
fn test_subset_sum_st_per_memo_size() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    // Initially, memo should be empty before any queries
    assert_eq!(solver.memo_size(), 0);

    // After a query, memo should be populated (but we can't check it directly
    // because subset_sum() clones and uses a fresh memo each time)
    solver.subset_sum(5);
    // Note: memo_size() still returns 0 because subset_sum creates a temporary clone
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_subset_sum_st_per_display() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    let display_str = format!("{}", solver);
    assert!(display_str.contains("SubsetSumStPer"));
    assert!(display_str.contains("multiset"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_subset_sum_st_per_into_iter_owned() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    let values = solver.into_iter().collect::<Vec<i32>>();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_subset_sum_st_per_into_iter_ref() {
    let solver = SubsetSumStPerLit![5, 10, 15];
    let values = (&solver).into_iter().collect::<Vec<i32>>();
    assert_eq!(values, vec![5, 10, 15]);
    // Solver should still be usable after borrowing
    assert!(solver.subset_sum(15));
}

#[test]
fn test_subset_sum_st_per_clone() {
    let solver1 = SubsetSumStPerLit![1, 2, 3];
    let solver2 = solver1.clone();
    assert_eq!(solver1, solver2);
    assert!(solver1.subset_sum(6));
    assert!(solver2.subset_sum(6));
}

#[test]
fn test_subset_sum_st_per_negative_target() {
    let solver = SubsetSumStPerLit![1, 2, 3];
    assert!(!solver.subset_sum(-5));
    assert!(!solver.subset_sum(-1));
}
