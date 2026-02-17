//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySetStEph

use apas_verus::{ArraySeqStEphSLit, ArraySetStEphLit};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap41::Example41_3::Example41_3::*;
use apas_verus::Types::Types::*;

#[test]
fn test_arraysetstephlit_macro_functionality() {
    // Test empty set creation
    let empty: ArraySetStEph<i32> = ArraySetStEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));

    // Test set creation with elements
    let with_data: ArraySetStEph<i32> = ArraySetStEphLit![1, 2, 3];
    assert_eq!(with_data.size(), 3);
    assert!(with_data.find(&1));
    assert!(with_data.find(&2));
    assert!(with_data.find(&3));
    assert!(!with_data.find(&4));
}

#[test]
fn test_array_set_basic_operations() {
    // Test empty set
    let empty_set = ArraySetStEph::<i32>::empty();
    assert_eq!(empty_set.size(), 0);
    assert!(!empty_set.find(&1));

    // Test singleton
    let singleton = ArraySetStEph::singleton(42);
    assert_eq!(singleton.size(), 1);
    assert!(singleton.find(&42));
    assert!(!singleton.find(&1));

    // Test insert
    let mut set = ArraySetStEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(1); // duplicate

    assert_eq!(set.size(), 2);
    assert!(set.find(&1));
    assert!(set.find(&2));

    // Test delete
    set.delete(&1);
    assert_eq!(set.size(), 1);
    assert!(!set.find(&1));
    assert!(set.find(&2));
}

#[test]
fn test_array_set_bulk_operations() {
    let set1 = ArraySetStEphLit![1, 2, 3];
    let set2 = ArraySetStEphLit![3, 4, 5];

    // Test union
    let union_result = set1.union(&set2);
    assert_eq!(union_result.size(), 5);
    for i in 1..=5 {
        assert!(union_result.find(&i));
    }

    // Test intersection
    let intersection_result = set1.intersection(&set2);
    assert_eq!(intersection_result.size(), 1);
    assert!(intersection_result.find(&3));
    assert!(!intersection_result.find(&1));
    assert!(!intersection_result.find(&4));

    // Test difference
    let difference_result = set1.difference(&set2);
    assert_eq!(difference_result.size(), 2);
    assert!(difference_result.find(&1));
    assert!(difference_result.find(&2));
    assert!(!difference_result.find(&3));
}

#[test]
fn test_array_set_from_seq() {
    // Test Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
    let seq_with_dups = ArraySeqStEphSLit![1, 2, 1, 3, 2, 4, 1];
    let set_result = ArraySetStEph::from_seq(seq_with_dups);

    assert_eq!(set_result.size(), 4);
    assert!(set_result.find(&1));
    assert!(set_result.find(&2));
    assert!(set_result.find(&3));
    assert!(set_result.find(&4));
}

#[test]
fn test_array_set_filter() {
    let set = ArraySetStEphLit![1, 2, 3, 4, 5, 6];
    let filtered = set.filter(|&x| x % 2 == 0);

    assert_eq!(filtered.size(), 3);
    assert!(filtered.find(&2));
    assert!(filtered.find(&4));
    assert!(filtered.find(&6));
    assert!(!filtered.find(&1));
    assert!(!filtered.find(&3));
    assert!(!filtered.find(&5));
}

#[test]
fn test_array_set_to_seq() {
    let set = ArraySetStEphLit![3, 1, 4, 1, 5]; // duplicates should be removed
    let seq = set.to_seq();

    // Should have 4 unique elements
    assert_eq!(seq.length(), 4);

    // Convert back to verify all elements are present
    let set_from_seq = ArraySetStEph::from_seq(seq);
    assert_eq!(set_from_seq.size(), 4);
    assert!(set_from_seq.find(&1));
    assert!(set_from_seq.find(&3));
    assert!(set_from_seq.find(&4));
    assert!(set_from_seq.find(&5));
}

#[test]
fn test_array_set_macro() {
    let set = ArraySetStEphLit![1, 2, 3];
    assert_eq!(set.size(), 3);
    assert!(set.find(&1));
    assert!(set.find(&2));
    assert!(set.find(&3));

    let empty: ArraySetStEph<i32> = ArraySetStEphLit![];
    assert_eq!(empty.size(), 0);
}

#[test]
fn test_example_41_1_cases() { example_41_1_array_set(); }

#[test]
fn test_example_41_3_demonstration() { example_41_3_from_seq_demonstration(); }

#[test]
fn test_additional_operations() { additional_set_operations(); }
