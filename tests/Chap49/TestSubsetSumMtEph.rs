//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumMtEph.

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphTrait, *};
use apas_verus::Chap49::SubsetSumMtEph::SubsetSumMtEph::*;
use apas_verus::SubsetSumMtEphLit;

#[test]
fn test_subset_sum_mt_eph_basic() {
    let mut solver = SubsetSumMtEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(1, 8);
    assert!(solver.subset_sum(8));
}

#[test]
fn test_subset_sum_mt_eph_mutation() {
    let mut solver = SubsetSumMtEphLit![5, 5, 5];
    assert!(solver.subset_sum(15));
    assert!(solver.subset_sum(10));

    solver.set(0, 10);
    assert!(solver.subset_sum(20));
}

#[test]
fn test_subset_sum_mt_eph_empty() {
    let mut solver: SubsetSumMtEphS<i32> = SubsetSumMtEphLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_mt_eph_new() {
    let mut solver = SubsetSumMtEphS::<i32>::new();
    assert_eq!(solver.multiset().length(), 0);
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_subset_sum_mt_eph_from_multiset() {
    let multiset = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::from_vec(vec![5, 10, 15]);
    let mut solver = SubsetSumMtEphS::from_multiset(multiset);
    assert_eq!(solver.multiset().length(), 3);
    assert!(solver.subset_sum(15));
    assert!(solver.subset_sum(25));
    assert!(!solver.subset_sum(7));
}

#[test]
fn test_subset_sum_mt_eph_multiset_getter() {
    let solver = SubsetSumMtEphLit![1, 2, 3];
    let multiset = solver.multiset();
    assert_eq!(multiset.length(), 3);
    assert_eq!(*multiset.nth(0), 1);
    assert_eq!(*multiset.nth(1), 2);
    assert_eq!(*multiset.nth(2), 3);
}

#[test]
fn test_subset_sum_mt_eph_multiset_mut() {
    let mut solver = SubsetSumMtEphLit![1, 2, 3];
    {
        let multiset_mut = solver.multiset_mut();
        assert_eq!(multiset_mut.length(), 3);
    }
    // Verify mutation works
    solver.set(0, 10);
    assert_eq!(*solver.multiset().nth(0), 10);
}

#[test]
fn test_subset_sum_mt_eph_clear_memo() {
    let mut solver = SubsetSumMtEphLit![1, 2, 3];

    // Run a query to populate memo
    solver.subset_sum(6);
    assert!(solver.memo_size() > 0);

    // Clear memo
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_subset_sum_mt_eph_memo_size() {
    let mut solver = SubsetSumMtEphLit![1, 2, 3];
    // Before computation, memo should be empty
    assert_eq!(solver.memo_size(), 0);

    // After computation, memo should contain subproblem results
    solver.subset_sum(5);
    assert!(solver.memo_size() > 0);
}

#[test]
fn test_subset_sum_mt_eph_display() {
    let solver = SubsetSumMtEphLit![1, 2, 3];
    let display_str = format!("{}", solver);
    assert!(display_str.contains("SubsetSumMtEph"));
    assert!(display_str.contains("multiset"));
    assert!(display_str.contains("memo_entries"));
}

#[test]
fn test_subset_sum_mt_eph_clone() {
    let solver1 = SubsetSumMtEphLit![1, 2, 3];
    let mut solver2 = solver1.clone();

    assert_eq!(solver1.multiset().length(), solver2.multiset().length());
    assert!(solver2.subset_sum(6));
}

#[test]
fn test_subset_sum_mt_eph_set_mutation() {
    let mut solver = SubsetSumMtEphLit![1, 2, 3];

    // Verify initial state
    assert_eq!(*solver.multiset().nth(0), 1);

    // Mutate and verify
    solver.set(0, 100);
    assert_eq!(*solver.multiset().nth(0), 100);

    // Verify subset sum reflects mutation
    let result = solver.subset_sum(102);
    assert!(result);
}
