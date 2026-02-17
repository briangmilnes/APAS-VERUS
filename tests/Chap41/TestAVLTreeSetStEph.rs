//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetStEph

use apas_verus::AVLTreeSetStEphLit;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;

use apas_verus::AVLTreeSeqStEphLit;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap41::Example41_3::Example41_3::*;
use apas_verus::Types::Types::*;

#[test]
fn test_avltreesetstephlit_macro_functionality() {
    // Test empty set creation
    let empty: AVLTreeSetStEph<i32> = AVLTreeSetStEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));

    // Test set creation with elements
    let with_data: AVLTreeSetStEph<i32> = AVLTreeSetStEphLit![1, 2, 3];
    assert_eq!(with_data.size(), 3);
    assert!(with_data.find(&1));
    assert!(with_data.find(&2));
    assert!(with_data.find(&3));
    assert!(!with_data.find(&4));
}

#[test]
fn test_avl_tree_set_basic_operations() {
    // Test empty set
    let empty_set = AVLTreeSetStEph::<i32>::empty();
    assert_eq!(empty_set.size(), 0);
    assert!(!empty_set.find(&1));

    // Test singleton
    let singleton = AVLTreeSetStEph::singleton(42);
    assert_eq!(singleton.size(), 1);
    assert!(singleton.find(&42));
    assert!(!singleton.find(&1));

    // Test insert
    let mut set = AVLTreeSetStEph::empty();
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
fn test_avl_tree_set_bulk_operations() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![3, 4, 5];

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
fn test_avl_tree_set_from_seq() {
    // Test Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
    let seq_with_dups = AVLTreeSeqStEphLit![1, 2, 1, 3, 2, 4, 1];
    let set_result = AVLTreeSetStEph::from_seq(seq_with_dups);

    assert_eq!(set_result.size(), 4);
    assert!(set_result.find(&1));
    assert!(set_result.find(&2));
    assert!(set_result.find(&3));
    assert!(set_result.find(&4));
}

#[test]
fn test_avl_tree_set_filter() {
    let set = AVLTreeSetStEphLit![1, 2, 3, 4, 5, 6];
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
fn test_avl_tree_set_to_seq() {
    let set = AVLTreeSetStEphLit![3, 1, 4, 1, 5]; // duplicates should be removed
    let seq = set.to_seq();

    // Should have 4 unique elements
    assert_eq!(seq.length(), 4);

    // Convert back to verify all elements are present
    let set_from_seq = AVLTreeSetStEph::from_seq(seq);
    assert_eq!(set_from_seq.size(), 4);
    assert!(set_from_seq.find(&1));
    assert!(set_from_seq.find(&3));
    assert!(set_from_seq.find(&4));
    assert!(set_from_seq.find(&5));
}

#[test]
fn test_avl_tree_set_macro() {
    let set = AVLTreeSetStEphLit![1, 2, 3];
    assert_eq!(set.size(), 3);
    assert!(set.find(&1));
    assert!(set.find(&2));
    assert!(set.find(&3));

    let empty: AVLTreeSetStEph<i32> = AVLTreeSetStEphLit![];
    assert_eq!(empty.size(), 0);
}

#[test]
fn test_avl_tree_set_ordering() {
    // AVL tree should maintain some ordering properties
    let set = AVLTreeSetStEphLit![5, 2, 8, 1, 3, 7, 9];
    assert_eq!(set.size(), 7);

    // All elements should be findable
    for i in [1, 2, 3, 5, 7, 8, 9] {
        assert!(set.find(&i));
    }

    // Non-existent elements should not be found
    for i in [0, 4, 6, 10] {
        assert!(!set.find(&i));
    }
}

#[test]
fn test_example_41_1_avl_cases() { example_41_1_avl_set(); }

#[test]
fn test_clone_set() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3, 4, 5];
    let mut set2 = set1.clone();

    assert_eq!(set1.size(), set2.size());
    for i in 1..=5 {
        assert!(set2.find(&i));
    }

    // Modify clone
    set2.insert(6);
    assert_eq!(set2.size(), 6);
    assert_eq!(set1.size(), 5); // Original unchanged (ephemeral, so this creates new backing)
}

#[test]
fn test_debug_trait() {
    let set = AVLTreeSetStEphLit![10, 20, 30];
    let debug_str = format!("{:?}", set);
    assert!(debug_str.contains("10"));
    assert!(debug_str.contains("20"));
    assert!(debug_str.contains("30"));
}

#[test]
fn test_large_set_operations() {
    let mut set = AVLTreeSetStEph::empty();
    for i in 0..100 {
        set.insert(i);
    }

    assert_eq!(set.size(), 100);
    for i in 0..100 {
        assert!(set.find(&i));
    }

    // Delete half
    for i in (0..100).step_by(2) {
        set.delete(&i);
    }

    assert_eq!(set.size(), 50);
    for i in (1..100).step_by(2) {
        assert!(set.find(&i));
    }
    for i in (0..100).step_by(2) {
        assert!(!set.find(&i));
    }
}

#[test]
fn test_union_empty_sets() {
    let empty1 = AVLTreeSetStEph::<i32>::empty();
    let empty2 = AVLTreeSetStEph::<i32>::empty();
    let result = empty1.union(&empty2);
    assert_eq!(result.size(), 0);

    let set = AVLTreeSetStEphLit![1, 2, 3];
    let result = set.union(&empty1);
    assert_eq!(result.size(), 3);
    assert!(result.find(&1));

    let result = empty1.union(&set);
    assert_eq!(result.size(), 3);
    assert!(result.find(&2));
}

#[test]
fn test_intersection_empty_sets() {
    let empty1 = AVLTreeSetStEph::<i32>::empty();
    let set = AVLTreeSetStEphLit![1, 2, 3];
    let result = set.intersection(&empty1);
    assert_eq!(result.size(), 0);

    let result = empty1.intersection(&set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_difference_empty_sets() {
    let empty1 = AVLTreeSetStEph::<i32>::empty();
    let set = AVLTreeSetStEphLit![1, 2, 3];

    let result = set.difference(&empty1);
    assert_eq!(result.size(), 3);

    let result = empty1.difference(&set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_filter_empty_set() {
    let empty = AVLTreeSetStEph::<i32>::empty();
    let filtered = empty.filter(|&x| x > 5);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_filter_all_match() {
    let set = AVLTreeSetStEphLit![2, 4, 6, 8];
    let filtered = set.filter(|&x| x % 2 == 0);
    assert_eq!(filtered.size(), 4);
    for i in [2, 4, 6, 8] {
        assert!(filtered.find(&i));
    }
}

#[test]
fn test_filter_none_match() {
    let set = AVLTreeSetStEphLit![1, 3, 5, 7];
    let filtered = set.filter(|&x| x % 2 == 0);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_from_seq_empty() {
    let seq: AVLTreeSeqStEphS<i32> = AVLTreeSeqStEphLit![];
    let set = AVLTreeSetStEph::from_seq(seq);
    assert_eq!(set.size(), 0);
}

#[test]
fn test_from_seq_singleton() {
    let seq = AVLTreeSeqStEphLit![42];
    let set = AVLTreeSetStEph::from_seq(seq);
    assert_eq!(set.size(), 1);
    assert!(set.find(&42));
}

#[test]
fn test_from_seq_all_duplicates() {
    let seq = AVLTreeSeqStEphLit![5, 5, 5, 5, 5];
    let set = AVLTreeSetStEph::from_seq(seq);
    assert_eq!(set.size(), 1);
    assert!(set.find(&5));
}

#[test]
fn test_to_seq_empty() {
    let empty = AVLTreeSetStEph::<i32>::empty();
    let seq = empty.to_seq();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_to_seq_singleton() {
    let set = AVLTreeSetStEph::singleton(42);
    let seq = set.to_seq();
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
}

#[test]
fn test_delete_nonexistent() {
    let mut set = AVLTreeSetStEphLit![1, 2, 3];
    set.delete(&99);
    assert_eq!(set.size(), 3); // Size unchanged
}

#[test]
fn test_delete_all_elements() {
    let mut set = AVLTreeSetStEphLit![1, 2, 3, 4, 5];
    for i in 1..=5 {
        set.delete(&i);
    }
    assert_eq!(set.size(), 0);
}

#[test]
fn test_insert_duplicate() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert(42);
    set.insert(42);
    set.insert(42);
    assert_eq!(set.size(), 1); // Should only have one copy
    assert!(set.find(&42));
}

#[test]
fn test_equality() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![1, 2, 3];
    let set3 = AVLTreeSetStEphLit![1, 2, 4];

    assert_eq!(set1, set2);
    assert_ne!(set1, set3);
}

#[test]
fn test_union_disjoint_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![4, 5, 6];
    let result = set1.union(&set2);

    assert_eq!(result.size(), 6);
    for i in 1..=6 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_union_identical_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![1, 2, 3];
    let result = set1.union(&set2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_intersection_disjoint_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![4, 5, 6];
    let result = set1.intersection(&set2);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_intersection_identical_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![1, 2, 3];
    let result = set1.intersection(&set2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_difference_identical_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![1, 2, 3];
    let result = set1.difference(&set2);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_difference_disjoint_sets() {
    let set1 = AVLTreeSetStEphLit![1, 2, 3];
    let set2 = AVLTreeSetStEphLit![4, 5, 6];
    let result = set1.difference(&set2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_mixed_operations_comprehensive() {
    let mut set1 = AVLTreeSetStEphLit![1, 2, 3, 4, 5];
    let set2 = AVLTreeSetStEphLit![4, 5, 6, 7];

    // Union
    let union_result = set1.union(&set2);
    assert_eq!(union_result.size(), 7);

    // Delete from original
    set1.delete(&3);
    assert_eq!(set1.size(), 4);

    // Insert new element
    set1.insert(10);
    assert_eq!(set1.size(), 5);

    // Intersection after modification
    let intersection_result = set1.intersection(&set2);
    assert_eq!(intersection_result.size(), 2); // 4, 5
    assert!(intersection_result.find(&4));
    assert!(intersection_result.find(&5));
}

#[test]
fn test_string_set() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());

    assert_eq!(set.size(), 3);
    assert!(set.find(&"apple".to_string()));
    assert!(set.find(&"banana".to_string()));
    assert!(set.find(&"cherry".to_string()));
    assert!(!set.find(&"date".to_string()));

    set.delete(&"banana".to_string());
    assert_eq!(set.size(), 2);
    assert!(!set.find(&"banana".to_string()));
}

#[test]
fn test_default_impl() {
    let set: AVLTreeSetStEph<i32> = Default::default();
    assert_eq!(set.size(), 0);
}

#[test]
fn test_clone_impl() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert(10);
    set.insert(20);
    let cloned = Clone::clone(&set);
    assert_eq!(cloned.size(), 2);
    assert!(cloned.find(&10));
}

#[test]
fn test_debug_impl() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert(10);
    set.insert(20);
    let debug_str = format!("{:?}", set);
    assert!(debug_str.contains("10"));
}

#[test]
fn test_display_impl() {
    let mut set = AVLTreeSetStEph::empty();
    set.insert(10);
    set.insert(20);
    let display_str = format!("{}", set);
    assert!(display_str.contains("10"));
}
