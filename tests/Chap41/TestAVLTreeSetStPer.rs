//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetStPer

use apas_verus::{AVLTreeSeqStPerLit, AVLTreeSetStPerLit};
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_avltreesetstperlit_macro_functionality() {
    // Test empty set creation
    let empty: AVLTreeSetStPer<i32> = AVLTreeSetStPerLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));

    // Test set creation with elements
    let with_data: AVLTreeSetStPer<i32> = AVLTreeSetStPerLit![1, 2, 3];
    assert_eq!(with_data.size(), 3);
    assert!(with_data.find(&1));
    assert!(with_data.find(&2));
    assert!(with_data.find(&3));
    assert!(!with_data.find(&4));
}

#[test]
fn test_avl_tree_set_per_basic_operations() {
    // Test empty set
    let empty_set = AVLTreeSetStPer::<i32>::empty();
    assert_eq!(empty_set.size(), 0);
    assert!(!empty_set.find(&1));

    // Test singleton
    let singleton = AVLTreeSetStPer::singleton(42);
    assert_eq!(singleton.size(), 1);
    assert!(singleton.find(&42));
    assert!(!singleton.find(&1));

    // Test insert (persistent - returns new set)
    let set = AVLTreeSetStPer::empty();
    let set1 = set.insert(1);
    let set2 = set1.insert(2);
    let set3 = set2.insert(1); // duplicate

    assert_eq!(set3.size(), 2);
    assert!(set3.find(&1));
    assert!(set3.find(&2));

    // Original sets should be unchanged (persistent)
    assert_eq!(set.size(), 0);
    assert_eq!(set1.size(), 1);
    assert_eq!(set2.size(), 2);

    // Test delete
    let set4 = set3.delete(&1);
    assert_eq!(set4.size(), 1);
    assert!(!set4.find(&1));
    assert!(set4.find(&2));

    // Original set should be unchanged
    assert_eq!(set3.size(), 2);
    assert!(set3.find(&1));
}

#[test]
fn test_avl_tree_set_per_bulk_operations() {
    let set1 = AVLTreeSetStPerLit![1, 2, 3];
    let set2 = AVLTreeSetStPerLit![3, 4, 5];

    // Test union
    let union_result = set1.union(&set2);
    assert_eq!(union_result.size(), 5);
    for i in 1..=5 {
        assert!(union_result.find(&i));
    }

    // Original sets should be unchanged
    assert_eq!(set1.size(), 3);
    assert_eq!(set2.size(), 3);

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
fn test_avl_tree_set_per_from_seq() {
    // Test Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
    let seq_with_dups = AVLTreeSeqStPerLit![1, 2, 1, 3, 2, 4, 1];
    let set_result = AVLTreeSetStPer::from_seq(seq_with_dups);

    assert_eq!(set_result.size(), 4);
    assert!(set_result.find(&1));
    assert!(set_result.find(&2));
    assert!(set_result.find(&3));
    assert!(set_result.find(&4));
}

#[test]
fn test_avl_tree_set_per_filter() {
    let set = AVLTreeSetStPerLit![1, 2, 3, 4, 5, 6];
    let filtered = set.filter(|&x| x % 2 == 0);

    assert_eq!(filtered.size(), 3);
    assert!(filtered.find(&2));
    assert!(filtered.find(&4));
    assert!(filtered.find(&6));
    assert!(!filtered.find(&1));
    assert!(!filtered.find(&3));
    assert!(!filtered.find(&5));

    // Original set should be unchanged
    assert_eq!(set.size(), 6);
}

#[test]
fn test_avl_tree_set_per_to_seq() {
    let set = AVLTreeSetStPerLit![3, 1, 4, 1, 5]; // duplicates should be removed
    let seq = set.to_seq();

    // Should have 4 unique elements
    assert_eq!(seq.length(), 4);

    // Convert back to verify all elements are present
    let set_from_seq = AVLTreeSetStPer::from_seq(seq);
    assert_eq!(set_from_seq.size(), 4);
    assert!(set_from_seq.find(&1));
    assert!(set_from_seq.find(&3));
    assert!(set_from_seq.find(&4));
    assert!(set_from_seq.find(&5));
}

#[test]
fn test_avl_tree_set_per_macro() {
    let set = AVLTreeSetStPerLit![1, 2, 3];
    assert_eq!(set.size(), 3);
    assert!(set.find(&1));
    assert!(set.find(&2));
    assert!(set.find(&3));

    let empty: AVLTreeSetStPer<i32> = AVLTreeSetStPerLit![];
    assert_eq!(empty.size(), 0);
}

#[test]
fn test_avl_tree_set_per_persistence() {
    // Test that operations create new sets without modifying originals
    let original = AVLTreeSetStPerLit![1, 2, 3];

    let with_four = original.insert(4);
    assert_eq!(original.size(), 3);
    assert_eq!(with_four.size(), 4);
    assert!(!original.find(&4));
    assert!(with_four.find(&4));

    let without_two = original.delete(&2);
    assert_eq!(original.size(), 3);
    assert_eq!(without_two.size(), 2);
    assert!(original.find(&2));
    assert!(!without_two.find(&2));

    let filtered = original.filter(|&x| x > 1);
    assert_eq!(original.size(), 3);
    assert_eq!(filtered.size(), 2);
    assert!(original.find(&1));
    assert!(!filtered.find(&1));
}

#[test]
fn test_default_impl() {
    let set: AVLTreeSetStPer<i32> = Default::default();
    assert_eq!(set.size(), 0);
}

#[test]
fn test_clone_impl() {
    let set = AVLTreeSetStPerLit![1, 2, 3];
    let cloned = set.clone();
    assert_eq!(cloned.size(), 3);
    assert!(cloned.find(&2));
}

#[test]
fn test_display_impl() {
    let set = AVLTreeSetStPerLit![1, 2, 3];
    let display_str = format!("{}", set);
    assert!(display_str.contains("1"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("3"));
}
