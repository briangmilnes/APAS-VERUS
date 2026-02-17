//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SubsetSumStEph.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap49::SubsetSumStEph::SubsetSumStEph::*;
use apas_verus::SubsetSumStEphLit;

#[test]
fn test_subset_sum_st_eph_basic() {
    let mut solver = SubsetSumStEphLit![1, 4, 2, 9];

    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(12));

    // Test ephemeral mutation
    solver.set(0, 3);
    assert!(!solver.subset_sum(8));
    assert!(solver.subset_sum(7));
}

#[test]
fn test_subset_sum_st_eph_mutation() {
    let mut solver = SubsetSumStEphLit![1, 1, 1];
    assert!(solver.subset_sum(3));

    solver.set(2, 5);
    assert!(!solver.subset_sum(3));
    assert!(solver.subset_sum(7));
}

#[test]
fn test_subset_sum_st_eph_empty() {
    let mut solver: SubsetSumStEphS<i32> = SubsetSumStEphLit![];
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_new() {
    let mut solver = SubsetSumStEphS::<i32>::new();
    assert_eq!(solver.multiset().length(), 0);
    assert!(solver.subset_sum(0));
    assert!(!solver.subset_sum(1));
}

#[test]
fn test_from_multiset() {
    let multiset = ArraySeqStEphSLit![1, 2, 3, 4];
    let mut solver = SubsetSumStEphS::from_multiset(multiset);
    assert_eq!(solver.multiset().length(), 4);
    assert!(solver.subset_sum(6)); // 2 + 4 = 6
}

#[test]
fn test_subset_sum_zero_target() {
    let mut solver = SubsetSumStEphLit![5, 10, 15];
    assert!(solver.subset_sum(0)); // Empty subset always sums to 0
}

#[test]
fn test_subset_sum_negative_target() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    assert!(!solver.subset_sum(-5)); // Negative targets return false
}

#[test]
fn test_subset_sum_exact_match() {
    let mut solver = SubsetSumStEphLit![7];
    assert!(solver.subset_sum(7)); // Single element equals target
}

#[test]
fn test_subset_sum_impossible() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    assert!(!solver.subset_sum(10)); // No subset can sum to 10
}

#[test]
fn test_subset_sum_multiple_solutions() {
    let mut solver = SubsetSumStEphLit![1, 2, 3, 4];
    // 1+2 = 3, or just 3
    assert!(solver.subset_sum(3));
    // 1+4 = 5, or 2+3 = 5
    assert!(solver.subset_sum(5));
}

#[test]
fn test_multiset_getter() {
    let solver = SubsetSumStEphLit![1, 2, 3];
    let multiset = solver.multiset();
    assert_eq!(multiset.length(), 3);
    assert_eq!(*multiset.nth(0), 1);
    assert_eq!(*multiset.nth(2), 3);
}

#[test]
fn test_multiset_mut_getter() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    {
        let multiset_mut = solver.multiset_mut();
        let _ = multiset_mut.set(1, 10);
    }
    assert_eq!(*solver.multiset().nth(1), 10);
}

#[test]
fn test_set_clears_memo() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    assert!(solver.subset_sum(6));

    // Memo should be populated now
    let memo_size_after_first = solver.memo_size();
    assert!(memo_size_after_first > 0);

    // Set clears memo
    solver.set(0, 10);
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_clear_memo() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    assert!(solver.subset_sum(5));

    // Memo should be populated
    assert!(solver.memo_size() > 0);

    // Clear memo
    solver.clear_memo();
    assert_eq!(solver.memo_size(), 0);
}

#[test]
fn test_memo_size() {
    let mut solver = SubsetSumStEphLit![1, 2, 3, 4];

    // Initially empty
    assert_eq!(solver.memo_size(), 0);

    // After computation, memo is cleared for next call
    assert!(solver.subset_sum(6));
    // subset_sum clears memo at start, so after call it should be populated
    let memo_size = solver.memo_size();
    assert!(memo_size > 0);
}

#[test]
fn test_display() {
    let solver = SubsetSumStEphLit![1, 2, 3];
    let display_str = format!("{}", solver);
    assert!(display_str.contains("SubsetSumStEph"));
    assert!(display_str.contains("multiset:"));
}

#[test]
fn test_into_iterator() {
    let solver = SubsetSumStEphLit![1, 2, 3];
    let values = solver.into_iter().collect::<Vec<_>>();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_into_iterator_ref() {
    let solver = SubsetSumStEphLit![1, 2, 3];
    let values = (&solver).into_iter().collect::<Vec<_>>();
    assert_eq!(values, vec![1, 2, 3]);

    // Original still usable
    assert_eq!(solver.multiset().length(), 3);
}

#[test]
fn test_into_iterator_mut() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    let values = (&mut solver).into_iter().collect::<Vec<_>>();
    assert_eq!(values, vec![1, 2, 3]);

    // Original still usable
    assert_eq!(solver.multiset().length(), 3);
}

#[test]
fn test_macro_empty() {
    let mut solver: SubsetSumStEphS<i32> = SubsetSumStEphLit!();
    assert_eq!(solver.multiset().length(), 0);
    assert!(solver.subset_sum(0));
}

#[test]
fn test_larger_multiset() {
    let mut solver = SubsetSumStEphLit![3, 34, 4, 12, 5, 2];
    assert!(solver.subset_sum(9)); // 4 + 5 = 9
    assert!(solver.subset_sum(19)); // 3 + 4 + 12 = 19
    assert!(!solver.subset_sum(100)); // Impossible
}

#[test]
fn test_all_same_value() {
    let mut solver = SubsetSumStEphLit![5, 5, 5, 5];
    assert!(solver.subset_sum(10)); // Two 5's
    assert!(solver.subset_sum(15)); // Three 5's
    assert!(!solver.subset_sum(7)); // Impossible
}

#[test]
fn test_mutation_preserves_functionality() {
    let mut solver = SubsetSumStEphLit![1, 2, 3];
    assert!(solver.subset_sum(6));

    // Mutate
    solver.set(0, 10);

    // Old sums don't work
    assert!(!solver.subset_sum(6));

    // New sums work
    assert!(solver.subset_sum(15)); // 10 + 2 + 3 = 15
}

#[test]
fn test_single_element() {
    let mut solver = SubsetSumStEphLit![42];
    assert!(solver.subset_sum(0)); // Empty subset
    assert!(solver.subset_sum(42)); // The element itself
    assert!(!solver.subset_sum(21)); // Impossible
}
