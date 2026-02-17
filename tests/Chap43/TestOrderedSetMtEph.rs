//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Comprehensive tests for OrderedSetMtEph - multi-threaded ephemeral ordered set implementation.

use std::sync::Arc;
use std::thread;

use apas_verus::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
use apas_verus::OrderedSetMtEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let set = OrderedSetMtEph::<i32>::empty();
    assert_eq!(set.size(), 0);
    assert_eq!(set.first(), None);
    assert_eq!(set.last(), None);
}

#[test]
fn test_singleton() {
    let set = OrderedSetMtEph::singleton(42);
    assert_eq!(set.size(), 1);
    assert_eq!(set.first(), Some(42));
    assert_eq!(set.last(), Some(42));
    assert!(set.find(&42));
    assert!(!set.find(&0));
}

#[test]
fn test_insert_and_find() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(5);
    set.insert(2);
    set.insert(8);
    set.insert(1);
    set.insert(7);

    assert_eq!(set.size(), 5);
    assert!(set.find(&1));
    assert!(set.find(&2));
    assert!(set.find(&5));
    assert!(set.find(&7));
    assert!(set.find(&8));
    assert!(!set.find(&0));
    assert!(!set.find(&10));
}

#[test]
fn test_delete() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(5);
    set.insert(2);
    set.insert(8);

    set.delete(&2);
    assert_eq!(set.size(), 2);
    assert!(!set.find(&2));
    assert!(set.find(&5));
    assert!(set.find(&8));
}

#[test]
fn test_first_and_last() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(5);
    set.insert(2);
    set.insert(8);
    set.insert(1);
    set.insert(7);

    assert_eq!(set.first(), Some(1));
    assert_eq!(set.last(), Some(8));

    let empty_set = OrderedSetMtEph::<i32>::empty();
    assert_eq!(empty_set.first(), None);
    assert_eq!(empty_set.last(), None);
}

#[test]
fn test_previous() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    assert_eq!(set.previous(&0), None); // Before first
    assert_eq!(set.previous(&1), None); // At first
    assert_eq!(set.previous(&2), Some(1)); // Between elements
    assert_eq!(set.previous(&5), Some(3)); // At element
    assert_eq!(set.previous(&6), Some(5)); // Between elements
    assert_eq!(set.previous(&10), Some(9)); // After last
}

#[test]
fn test_next() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    assert_eq!(set.next(&0), Some(1)); // Before first
    assert_eq!(set.next(&1), Some(3)); // At first
    assert_eq!(set.next(&2), Some(3)); // Between elements
    assert_eq!(set.next(&5), Some(7)); // At element
    assert_eq!(set.next(&8), Some(9)); // Between elements
    assert_eq!(set.next(&9), None); // At last
    assert_eq!(set.next(&10), None); // After last
}

#[test]
fn test_split() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    // Split at existing element
    let (left, found, right) = set.split(&5);
    assert!(found);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 2);
    assert!(left.find(&1));
    assert!(left.find(&3));
    assert!(!left.find(&5));
    assert!(right.find(&7));
    assert!(right.find(&9));
    assert!(!right.find(&5));

    // Original set is now empty (ephemeral behavior)
    assert_eq!(set.size(), 0);
}

#[test]
fn test_join() {
    let mut left = OrderedSetMtEph::empty();
    left.insert(1);
    left.insert(3);

    let mut right = OrderedSetMtEph::empty();
    right.insert(7);
    right.insert(9);

    left.join(right);
    assert_eq!(left.size(), 4);
    assert!(left.find(&1));
    assert!(left.find(&3));
    assert!(left.find(&7));
    assert!(left.find(&9));
}

#[test]
fn test_get_range() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    let range = set.get_range(&3, &7);
    assert_eq!(range.size(), 3);
    assert!(range.find(&3));
    assert!(range.find(&5));
    assert!(range.find(&7));
    assert!(!range.find(&1));
    assert!(!range.find(&9));

    // Empty range
    let empty_range = set.get_range(&10, &20);
    assert_eq!(empty_range.size(), 0);
}

#[test]
fn test_rank() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    assert_eq!(set.rank(&0), 0); // Before first
    assert_eq!(set.rank(&1), 0); // At first
    assert_eq!(set.rank(&2), 1); // Between elements
    assert_eq!(set.rank(&5), 2); // At element
    assert_eq!(set.rank(&6), 3); // Between elements
    assert_eq!(set.rank(&10), 5); // After last
}

#[test]
fn test_select() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    assert_eq!(set.select(0), Some(1));
    assert_eq!(set.select(1), Some(3));
    assert_eq!(set.select(2), Some(5));
    assert_eq!(set.select(3), Some(7));
    assert_eq!(set.select(4), Some(9));
    assert_eq!(set.select(5), None); // Out of bounds
}

#[test]
fn test_split_rank() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    set.insert(9);

    let (left, right) = set.split_rank(2);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 3);
    assert!(left.find(&1));
    assert!(left.find(&3));
    assert!(right.find(&5));
    assert!(right.find(&7));
    assert!(right.find(&9));

    // Original set is now empty (ephemeral behavior)
    assert_eq!(set.size(), 0);
}

#[test]
fn test_filter() {
    let mut set = OrderedSetMtEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    set.filter(|x| *x % 2 == 0);
    assert_eq!(set.size(), 2);
    assert!(set.find(&2));
    assert!(set.find(&4));
    assert!(!set.find(&1));
    assert!(!set.find(&3));
    assert!(!set.find(&5));
}

#[test]
fn test_intersection() {
    let mut set1 = OrderedSetMtEph::empty();
    set1.insert(1);
    set1.insert(3);
    set1.insert(5);
    set1.insert(7);

    let mut set2 = OrderedSetMtEph::empty();
    set2.insert(3);
    set2.insert(4);
    set2.insert(5);
    set2.insert(6);

    set1.intersection(&set2);
    assert_eq!(set1.size(), 2);
    assert!(set1.find(&3));
    assert!(set1.find(&5));
    assert!(!set1.find(&1));
    assert!(!set1.find(&4));
}

#[test]
fn test_union() {
    let mut set1 = OrderedSetMtEph::empty();
    set1.insert(1);
    set1.insert(3);
    set1.insert(5);

    let mut set2 = OrderedSetMtEph::empty();
    set2.insert(3);
    set2.insert(4);
    set2.insert(6);

    set1.union(&set2);
    assert_eq!(set1.size(), 5);
    assert!(set1.find(&1));
    assert!(set1.find(&3));
    assert!(set1.find(&4));
    assert!(set1.find(&5));
    assert!(set1.find(&6));
}

#[test]
fn test_difference() {
    let mut set1 = OrderedSetMtEph::empty();
    set1.insert(1);
    set1.insert(3);
    set1.insert(5);
    set1.insert(7);

    let mut set2 = OrderedSetMtEph::empty();
    set2.insert(3);
    set2.insert(5);

    set1.difference(&set2);
    assert_eq!(set1.size(), 2);
    assert!(set1.find(&1));
    assert!(set1.find(&7));
    assert!(!set1.find(&3));
    assert!(!set1.find(&5));
}

#[test]
fn test_ephemeral_semantics() {
    let mut original = OrderedSetMtEph::empty();
    original.insert(1);
    original.insert(2);
    original.insert(3);

    // Test that operations modify the original set
    let original_size = original.size();
    original.insert(4);
    assert_eq!(original.size(), original_size + 1);
    assert!(original.find(&4));

    original.delete(&2);
    assert_eq!(original.size(), original_size);
    assert!(!original.find(&2));

    // Test split empties original
    let mut test_set = OrderedSetMtEph::empty();
    test_set.insert(1);
    test_set.insert(2);
    test_set.insert(3);

    let (_left, _found, _right) = test_set.split(&2);
    assert_eq!(test_set.size(), 0);
}

#[test]
fn test_parallel_operations() {
    // Test that parallel operations work correctly with larger datasets
    let mut set1 = OrderedSetMtEph::empty();
    let mut set2 = OrderedSetMtEph::empty();

    // Insert many elements to trigger parallel paths
    for i in 0..20 {
        set1.insert(i * 2); // Even numbers
        set2.insert(i * 2 + 1); // Odd numbers
    }

    // Test parallel union
    set1.union(&set2);
    assert_eq!(set1.size(), 40);

    // Verify all elements are present
    for i in 0..40 {
        assert!(set1.find(&i));
    }

    // Test parallel filter
    set1.filter(|x| *x < 20);
    assert_eq!(set1.size(), 20);

    for i in 0..20 {
        assert!(set1.find(&i));
    }
    for i in 20..40 {
        assert!(!set1.find(&i));
    }
}

#[test]
fn test_thread_safety() {
    // Test that the data structure can be safely shared between threads
    let mut set = OrderedSetMtEph::empty();
    for i in 0..10 {
        set.insert(i);
    }

    let set = Arc::new(set);
    let mut handles = vec![];

    // Spawn multiple threads that read from the set
    for _ in 0..4 {
        let set_clone = Arc::clone(&set);
        let handle = thread::spawn(move || {
            let mut found_count = 0;
            for i in 0..10 {
                if set_clone.find(&i) {
                    found_count += 1;
                }
            }
            found_count
        });
        handles.push(handle);
    }

    // All threads should find all 10 elements
    for handle in handles {
        let count = handle.join().unwrap();
        assert_eq!(count, 10);
    }
}

#[test]
fn test_ordered_set_mt_eph_lit_macro() {
    let set: OrderedSetMtEph<i32> = OrderedSetMtEphLit![1, 3, 5, 7, 9];
    assert_eq!(set.size(), 5);
    assert!(set.find(&1));
    assert!(set.find(&3));
    assert!(set.find(&5));
    assert!(set.find(&7));
    assert!(set.find(&9));
    assert!(!set.find(&2));

    let empty_set: OrderedSetMtEph<i32> = OrderedSetMtEphLit![];
    assert_eq!(empty_set.size(), 0);
}

#[test]
fn test_string_ordering() {
    let mut set = OrderedSetMtEph::empty();
    set.insert("charlie".to_string());
    set.insert("alice".to_string());
    set.insert("bob".to_string());

    assert_eq!(set.first(), Some("alice".to_string()));
    assert_eq!(set.last(), Some("charlie".to_string()));
    assert_eq!(set.next(&"alice".to_string()), Some("bob".to_string()));
    assert_eq!(set.previous(&"charlie".to_string()), Some("bob".to_string()));
}

#[test]
fn test_large_dataset_performance() {
    // Test with larger dataset to ensure parallel operations are beneficial
    let mut set = OrderedSetMtEph::empty();

    // Insert 100 elements
    for i in 0..100 {
        set.insert(i);
    }

    assert_eq!(set.size(), 100);
    assert_eq!(set.first(), Some(0));
    assert_eq!(set.last(), Some(99));

    // Test parallel filter on large dataset
    set.filter(|x| *x % 10 == 0);
    assert_eq!(set.size(), 10);

    for i in 0..10 {
        assert!(set.find(&(i * 10)));
    }
}
