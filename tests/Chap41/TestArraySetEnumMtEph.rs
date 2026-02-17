//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySetEnumMtEph

use apas_verus::ArraySetEnumMtEphLit;

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_arraysetenummtephlit_macro_type_safety() {
    // Test capacity-only creation
    let capacity_only: ArraySetEnumMtEph = ArraySetEnumMtEphLit![10;];
    assert_eq!(capacity_only.size(), 0);
    assert!(!capacity_only.find(0));
    assert!(!capacity_only.find(5));

    // Test capacity with initial elements
    let with_elements: ArraySetEnumMtEph = ArraySetEnumMtEphLit![10; 1, 2, 3];
    assert_eq!(with_elements.size(), 3);
    assert!(with_elements.find(1));
    assert!(with_elements.find(2));
    assert!(with_elements.find(3));
    assert!(!with_elements.find(4));
    assert!(!with_elements.find(0));
}

#[test]
fn test_array_set_enum_mt_basic_operations() {
    // Test new and empty
    let empty_set = ArraySetEnumMtEph::new(10);
    assert_eq!(empty_set.size(), 0);
    assert!(!empty_set.find(0));
    assert!(!empty_set.find(5));
    assert!(!empty_set.find(9));

    let empty_set2 = ArraySetEnumMtEph::empty(10);
    assert_eq!(empty_set2.size(), 0);

    // Test singleton
    let singleton = ArraySetEnumMtEph::singleton(10, 5);
    assert_eq!(singleton.size(), 1);
    assert!(singleton.find(5));
    assert!(!singleton.find(0));
    assert!(!singleton.find(9));

    // Test insert
    let mut set = ArraySetEnumMtEph::empty(10);
    set.insert(1);
    set.insert(3);
    set.insert(1); // duplicate

    assert_eq!(set.size(), 2);
    assert!(set.find(1));
    assert!(set.find(3));
    assert!(!set.find(0));
    assert!(!set.find(2));

    // Test delete
    set.delete(1);
    assert_eq!(set.size(), 1);
    assert!(!set.find(1));
    assert!(set.find(3));

    // Test out of bounds
    let mut set_bounds = ArraySetEnumMtEph::empty(10);
    set_bounds.insert(15); // out of bounds
    assert_eq!(set_bounds.size(), 0);
    assert!(!set_bounds.find(15));
}

#[test]
fn test_array_set_enum_mt_bulk_operations() {
    let set1 = ArraySetEnumMtEphLit![10; 1, 2, 3];
    let set2 = ArraySetEnumMtEphLit![10; 3, 4, 5];

    // Test union
    let union_result = set1.union(&set2);
    assert_eq!(union_result.size(), 5);
    for i in 1..=5 {
        assert!(union_result.find(i));
    }
    assert!(!union_result.find(0));
    assert!(!union_result.find(6));

    // Test intersection
    let intersection_result = set1.intersection(&set2);
    assert_eq!(intersection_result.size(), 1);
    assert!(intersection_result.find(3));
    assert!(!intersection_result.find(1));
    assert!(!intersection_result.find(4));

    // Test difference
    let difference_result = set1.difference(&set2);
    assert_eq!(difference_result.size(), 2);
    assert!(difference_result.find(1));
    assert!(difference_result.find(2));
    assert!(!difference_result.find(3));
    assert!(!difference_result.find(4));
}

#[test]
fn test_array_set_enum_mt_from_seq() {
    let seq_data = vec![1, 3, 2, 3, 1, 4, 2]; // with duplicates
    let seq = ArraySeqMtEphS::from_vec(seq_data);
    let set_result = ArraySetEnumMtEph::from_seq(10, seq);

    assert_eq!(set_result.size(), 4);
    assert!(set_result.find(1));
    assert!(set_result.find(2));
    assert!(set_result.find(3));
    assert!(set_result.find(4));
    assert!(!set_result.find(0));
    assert!(!set_result.find(5));
}

#[test]
fn test_array_set_enum_mt_filter() {
    let set = ArraySetEnumMtEphLit![10; 1, 2, 3, 4, 5, 6];

    // Create a function that meets the Clone + 'static requirements
    fn is_even(x: usize) -> bool { x % 2 == 0 }
    let filtered = set.filter(is_even);

    assert_eq!(filtered.size(), 3);
    assert!(filtered.find(2));
    assert!(filtered.find(4));
    assert!(filtered.find(6));
    assert!(!filtered.find(1));
    assert!(!filtered.find(3));
    assert!(!filtered.find(5));
}

#[test]
fn test_array_set_enum_mt_to_seq() {
    let set = ArraySetEnumMtEphLit![10; 3, 1, 4, 5]; // order doesn't matter for enumerated sets
    let seq = set.to_seq();

    // Should have 4 elements in ascending order (0, 1, 3, 4, 5)
    assert_eq!(seq.length(), 4);

    // Convert back to verify all elements are present
    let set_from_seq = ArraySetEnumMtEph::from_seq(10, seq);
    assert_eq!(set_from_seq.size(), 4);
    assert!(set_from_seq.find(1));
    assert!(set_from_seq.find(3));
    assert!(set_from_seq.find(4));
    assert!(set_from_seq.find(5));
}

#[test]
fn test_array_set_enum_mt_macro() {
    let set = ArraySetEnumMtEphLit![10; 1, 2, 3];
    assert_eq!(set.size(), 3);
    assert!(set.find(1));
    assert!(set.find(2));
    assert!(set.find(3));

    let empty: ArraySetEnumMtEph = ArraySetEnumMtEphLit![10;];
    assert_eq!(empty.size(), 0);
}

#[test]
fn test_array_set_enum_mt_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let set = Arc::new(ArraySetEnumMtEphLit![100; 1, 2, 3, 4, 5]);
    let mut handles = Vec::new();

    // Spawn multiple threads that read from the set
    for i in 0..10 {
        let set_clone = Arc::clone(&set);
        let handle = thread::spawn(move || {
            let target = (i % 5) + 1;
            set_clone.find(target)
        });
        handles.push(handle);
    }

    // All threads should complete successfully
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result); // All values 1-5 should be found
    }
}

#[test]
fn test_array_set_enum_mt_universe_bounds() {
    let set = ArraySetEnumMtEph::new(5); // universe size 5 (0-4)

    // Insert valid elements
    let mut set1 = set;
    set1.insert(0);
    set1.insert(2);
    set1.insert(4);
    assert_eq!(set1.size(), 3);
    assert!(set1.find(0));
    assert!(set1.find(2));
    assert!(set1.find(4));

    // Try to insert out-of-bounds elements
    set1.insert(5);
    set1.insert(10);
    assert_eq!(set1.size(), 3); // Size shouldn't change
    assert!(!set1.find(5));
    assert!(!set1.find(10));

    // Try to find out-of-bounds elements
    assert!(!set1.find(5));
    assert!(!set1.find(100));
}
