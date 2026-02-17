//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtPer.

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap49::SubsetSumMtPer::SubsetSumMtPer::*;
use apas_verus::SubsetSumMtPerLit;

#[test]
fn test_subset_sum_mt_per_basic() {
    let solver = SubsetSumMtPerLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));
    assert!(solver.subset_sum(0));
    assert!(solver.subset_sum(16));
}

#[test]
fn test_subset_sum_mt_per_example() {
    let solver = SubsetSumMtPerLit![1, 1, 1];
    assert!(solver.subset_sum(3));
    assert!(solver.subset_sum(2));
    assert!(!solver.subset_sum(4));
}

#[test]
fn test_subset_sum_mt_per_empty() {
    let solver: SubsetSumMtPerS<i32> = SubsetSumMtPerLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_mt_per_new() {
    let solver = SubsetSumMtPerS::<i32>::new();
    assert_eq!(solver.multiset().length(), 0);
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_mt_per_from_multiset() {
    let multiset = <ArraySeqMtPerS<i32> as ArraySeqMtPerBaseTrait<i32>>::from_vec(vec![5, 10, 15]);
    let solver = SubsetSumMtPerS::from_multiset(multiset);
    assert_eq!(solver.multiset().length(), 3);
    assert!(solver.subset_sum(15));
    assert!(solver.subset_sum(25));
    assert!(!solver.subset_sum(7));
}

#[test]
fn test_subset_sum_mt_per_multiset_getter() {
    let solver = SubsetSumMtPerLit![1, 2, 3];
    let multiset = solver.multiset();
    assert_eq!(multiset.length(), 3);
    assert_eq!(*multiset.nth(0), 1);
    assert_eq!(*multiset.nth(1), 2);
    assert_eq!(*multiset.nth(2), 3);
}

#[test]
fn test_subset_sum_mt_per_memo_size() {
    let solver = SubsetSumMtPerLit![1, 2, 3];
    assert_eq!(solver.memo_size(), 0);
    solver.subset_sum(5);
    assert!(solver.memo_size() > 0);
}

#[test]
fn test_subset_sum_mt_per_display() {
    let solver = SubsetSumMtPerLit![1, 2, 3];
    let display_str = format!("{}", solver);
    assert!(display_str.contains("SubsetSumMtPer"));
    assert!(display_str.contains("multiset"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_subset_sum_mt_per_clone() {
    let solver1 = SubsetSumMtPerLit![1, 2, 3];
    let solver2 = solver1.clone();
    assert_eq!(solver1.multiset().length(), solver2.multiset().length());
    assert!(solver2.subset_sum(6));
}

#[test]
fn test_subset_sum_mt_per_partial_eq() {
    let solver1 = SubsetSumMtPerLit![1, 2, 3];
    let solver2 = SubsetSumMtPerLit![1, 2, 3];
    let solver3 = SubsetSumMtPerLit![1, 2, 4];
    assert_eq!(solver1, solver2);
    assert_ne!(solver1, solver3);
}
