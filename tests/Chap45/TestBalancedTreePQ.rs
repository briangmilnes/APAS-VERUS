//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Test for Chapter 45: BalancedTreePQ - Priority Queue using AVL Trees

use apas_verus::{AVLTreeSeqStPerLit, ArraySeqStPerSLit, BalancedTreePQLit};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap45::BalancedTreePQ::BalancedTreePQ::*;
use apas_verus::Types::Types::*;

#[test]
fn test_balancedtreepqlit_macro_functionality() {
    // Test empty priority queue creation
    let empty: BalancedTreePQ<i32> = BalancedTreePQLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());

    // Test priority queue creation with elements
    let with_data: BalancedTreePQ<i32> = BalancedTreePQLit![3, 1, 4, 1, 5];
    assert_eq!(with_data.size(), 5);
    assert!(!with_data.is_empty());
}

#[test]
fn test_empty_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_singleton_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::singleton(42);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&42));
}

#[test]
fn test_insert_and_find_min() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10);
    let pq = pq.insert(5);
    let pq = pq.insert(15);
    let pq = pq.insert(3);
    let pq = pq.insert(8);

    assert_eq!(pq.find_min(), Some(&3));
    assert_eq!(pq.size(), 5);
}

#[test]
fn test_delete_min() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10);
    let pq = pq.insert(5);
    let pq = pq.insert(15);
    let pq = pq.insert(3);

    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(3));
    assert_eq!(pq.find_min(), Some(&5));
    assert_eq!(pq.size(), 3);

    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(5));
    assert_eq!(pq.find_min(), Some(&10));
    assert_eq!(pq.size(), 2);
}

#[test]
fn test_delete_min_empty() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, None);
    assert!(pq.is_empty());
}

#[test]
fn test_meld_two_priority_queues() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq1 = pq1.insert(10).insert(5).insert(15);

    let pq2: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq2 = pq2.insert(2).insert(8).insert(12);

    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&2));

    // Verify all elements are present by extracting in order
    let (melded, min1) = melded.delete_min();
    assert_eq!(min1, Some(2));
    let (melded, min2) = melded.delete_min();
    assert_eq!(min2, Some(5));
    let (_melded, min3) = melded.delete_min();
    assert_eq!(min3, Some(8));
}

#[test]
fn test_meld_with_empty() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq1 = pq1.insert(10).insert(5);

    let pq2: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();

    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 2);
    assert_eq!(melded.find_min(), Some(&5));
}

#[test]
fn test_from_seq() {
    let elements = AVLTreeSeqStPerLit![10, 5, 15, 3, 8, 12];
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::from_seq(&elements);

    assert_eq!(pq.size(), 6);
    assert_eq!(pq.find_min(), Some(&3));
}

#[test]
fn test_to_seq() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);

    let seq = pq.to_seq();
    assert_eq!(seq.length(), 3);
    // Elements should be in sorted order in the sequence
    assert_eq!(*seq.nth(0), 5);
    assert_eq!(*seq.nth(1), 10);
    assert_eq!(*seq.nth(2), 15);
}

#[test]
fn test_priority_queue_ordering() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let elements = [42, 17, 89, 3, 56, 23, 71, 8];

    let mut pq = pq;
    for &elem in &elements {
        pq = pq.insert(elem);
    }

    let mut sorted_elements = elements.to_vec();
    sorted_elements.sort();

    // Extract all elements and verify they come out in sorted order
    let mut extracted = ArraySeqStPerSLit![];
    let mut current_pq = pq;

    while !current_pq.is_empty() {
        let (new_pq, min_val) = current_pq.delete_min();
        if let Some(val) = min_val {
            extracted = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::append(&extracted, &ArraySeqStPerSLit![val]);
        }
        current_pq = new_pq;
    }

    for (i, &expected) in sorted_elements.iter().enumerate() {
        assert_eq!(*extracted.nth(i as N), expected);
    }
}

#[test]
fn test_duplicate_elements() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(5).insert(3).insert(5).insert(3).insert(5);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&3));

    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(3));
    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(3));
    let (_pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(5));
}

#[test]
fn test_string_priority_queue() {
    let pq: BalancedTreePQ<String> = BalancedTreePQTrait::empty();
    let pq = pq
        .insert("zebra".to_string())
        .insert("apple".to_string())
        .insert("banana".to_string())
        .insert("cherry".to_string());

    assert_eq!(pq.find_min(), Some(&"apple".to_string()));

    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some("apple".to_string()));
    assert_eq!(pq.find_min(), Some(&"banana".to_string()));
}

#[test]
fn test_large_priority_queue() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let mut pq = pq;

    // Insert 100 random-order elements
    let elements: ArraySeqStPerS<i32> =
        <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::tabulate(&|i| (i as i32 * 17 + 13) % 97, 100);
    for i in 0..elements.length() {
        pq = pq.insert(*elements.nth(i));
    }

    assert_eq!(pq.size(), 100);
    assert!(!pq.is_empty());

    // Extract first 10 elements to verify ordering
    let mut current_pq = pq;
    let mut prev_min = -1;

    for _ in 0..10 {
        let (new_pq, min_val) = current_pq.delete_min();
        if let Some(val) = min_val {
            assert!(val >= prev_min);
            prev_min = val;
        }
        current_pq = new_pq;
    }
}

#[test]
fn test_find_max() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert_eq!(pq.find_max(), None);

    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);
    assert_eq!(pq.find_max(), Some(&15));
}

#[test]
fn test_delete_max() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);

    let (pq, max_val) = pq.delete_max();
    assert_eq!(max_val, Some(15));
    assert_eq!(pq.find_max(), Some(&10));
    assert_eq!(pq.size(), 4);

    let (pq, max_val) = pq.delete_max();
    assert_eq!(max_val, Some(10));
    assert_eq!(pq.size(), 3);
}

#[test]
fn test_delete_max_empty() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let (pq, max_val) = pq.delete_max();
    assert_eq!(max_val, None);
    assert!(pq.is_empty());
}

#[test]
fn test_insert_all() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::singleton(5);
    let new_elements = AVLTreeSeqStPerLit![10, 2, 8, 12];

    let pq = pq.insert_all(&new_elements);
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&2));
    assert_eq!(pq.find_max(), Some(&12));
}

#[test]
fn test_extract_all_sorted() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted = pq.extract_all_sorted();
    assert_eq!(sorted.length(), 5);
    assert_eq!(*sorted.nth(0), 3);
    assert_eq!(*sorted.nth(1), 5);
    assert_eq!(*sorted.nth(2), 8);
    assert_eq!(*sorted.nth(3), 10);
    assert_eq!(*sorted.nth(4), 15);
}

#[test]
fn test_contains() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);

    assert!(pq.contains(&5));
    assert!(pq.contains(&10));
    assert!(pq.contains(&15));
    assert!(!pq.contains(&3));
    assert!(!pq.contains(&20));
}

#[test]
fn test_contains_empty() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert!(!pq.contains(&5));
}

#[test]
fn test_remove() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);

    let (pq, removed) = pq.remove(&8);
    assert!(removed);
    assert_eq!(pq.size(), 4);
    assert!(!pq.contains(&8));

    let (pq, removed) = pq.remove(&3);
    assert!(removed);
    assert_eq!(pq.size(), 3);
    assert_eq!(pq.find_min(), Some(&5));
}

#[test]
fn test_remove_not_found() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);

    let (pq, removed) = pq.remove(&20);
    assert!(!removed);
    assert_eq!(pq.size(), 3);
}

#[test]
fn test_range() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq
        .insert(10)
        .insert(5)
        .insert(15)
        .insert(3)
        .insert(8)
        .insert(12)
        .insert(20);

    let range_result = pq.range(&5, &12);
    assert_eq!(range_result.length(), 4);
    assert_eq!(*range_result.nth(0), 5);
    assert_eq!(*range_result.nth(1), 8);
    assert_eq!(*range_result.nth(2), 10);
    assert_eq!(*range_result.nth(3), 12);
}

#[test]
fn test_range_empty_result() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(20).insert(30);

    let range_result = pq.range(&5, &8);
    assert_eq!(range_result.length(), 0);
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 5, 15, 3, 8];
    let pq = BalancedTreePQ::from_vec(vec);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&3));
    assert_eq!(pq.find_max(), Some(&15));
}

#[test]
fn test_to_vec() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);

    let vec = pq.to_vec();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], 5);
    assert_eq!(vec[1], 10);
    assert_eq!(vec[2], 15);
}

#[test]
fn test_to_sorted_vec() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted_vec = pq.to_sorted_vec();
    assert_eq!(sorted_vec.len(), 5);
    for i in 1..sorted_vec.len() {
        assert!(sorted_vec[i - 1] <= sorted_vec[i]);
    }
}

#[test]
fn test_is_sorted() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15).insert(3).insert(8);

    assert!(pq.is_sorted());
}

#[test]
fn test_is_sorted_empty() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert!(pq.is_sorted());
}

#[test]
fn test_height() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    assert_eq!(pq.height(), 0);

    let pq = pq.insert(10);
    assert!(pq.height() >= 1);

    // Insert several elements to grow tree
    let mut pq = pq;
    for i in 0..15 {
        pq = pq.insert(i);
    }
    assert!(pq.height() > 1);
    assert!(pq.height() <= ((pq.size() as f64).log2().ceil() as N) + 1);
}

#[test]
fn test_split() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq
        .insert(10)
        .insert(5)
        .insert(15)
        .insert(3)
        .insert(8)
        .insert(12)
        .insert(20);

    let (left, found, right) = pq.split(&10);
    assert!(found);
    assert_eq!(left.size(), 3); // 3, 5, 8
    assert_eq!(right.size(), 4); // 10, 12, 15, 20
    assert_eq!(left.find_max(), Some(&8));
    assert_eq!(right.find_min(), Some(&10));
}

#[test]
fn test_split_not_found() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(10).insert(5).insert(15);

    let (left, found, right) = pq.split(&7);
    assert!(!found);
    assert_eq!(left.size(), 1); // 5
    assert_eq!(right.size(), 2); // 10, 15
}

#[test]
fn test_join() {
    let left: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let left = left.insert(3).insert(5).insert(7);

    let right: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let right = right.insert(10).insert(15).insert(20);

    let joined = BalancedTreePQ::join(&left, &right);
    assert_eq!(joined.size(), 6);
    assert_eq!(joined.find_min(), Some(&3));
    assert_eq!(joined.find_max(), Some(&20));
}

#[test]
fn test_filter() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq
        .insert(10)
        .insert(5)
        .insert(15)
        .insert(3)
        .insert(8)
        .insert(12)
        .insert(20);

    let filtered = pq.filter(|&x| x % 2 == 0);
    assert_eq!(filtered.size(), 4); // 10, 8, 12, 20
    assert!(filtered.contains(&10));
    assert!(filtered.contains(&8));
    assert!(!filtered.contains(&5));
    assert!(!filtered.contains(&15));
}

#[test]
fn test_filter_empty_result() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(1).insert(3).insert(5);

    let filtered = pq.filter(|&x| x > 10);
    assert!(filtered.is_empty());
}

#[test]
fn test_map() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(1).insert(2).insert(3).insert(4);

    let mapped = pq.map(|&x| x * 2);
    assert_eq!(mapped.size(), 4);
    assert_eq!(mapped.find_min(), Some(&2));
    assert_eq!(mapped.find_max(), Some(&8));
    assert!(mapped.contains(&4));
    assert!(mapped.contains(&6));
}

#[test]
fn test_map_to_string() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(1).insert(2).insert(3);

    let mapped: BalancedTreePQ<String> = pq.map(|&x| format!("num{x}"));
    assert_eq!(mapped.size(), 3);
    assert_eq!(mapped.find_min(), Some(&"num1".to_string()));
    assert_eq!(mapped.find_max(), Some(&"num3".to_string()));
}

#[test]
fn test_default() {
    let pq = BalancedTreePQ::<i32>::default();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
}

#[test]
fn test_persistent_semantics() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::singleton(5);
    let pq2 = pq1.insert(10);

    // pq1 should remain unchanged (persistent)
    assert_eq!(pq1.size(), 1);
    assert_eq!(pq1.find_min(), Some(&5));

    // pq2 should have both elements
    assert_eq!(pq2.size(), 2);
    assert_eq!(pq2.find_min(), Some(&5));
}

#[test]
fn test_sequential_delete_min_all() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(5).insert(3).insert(8).insert(1).insert(10);

    let mut current_pq = pq;
    let mut extracted = vec![];

    while !current_pq.is_empty() {
        let (new_pq, min_val) = current_pq.delete_min();
        if let Some(val) = min_val {
            extracted.push(val);
        }
        current_pq = new_pq;
    }

    assert_eq!(extracted, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_meld_multiple() {
    let pq1: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq1 = pq1.insert(1).insert(5);
    let pq2: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq2 = pq2.insert(3).insert(7);
    let pq3: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq3 = pq3.insert(2).insert(6);

    let melded = pq1.meld(&pq2).meld(&pq3);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&1));
    assert_eq!(melded.find_max(), Some(&7));
}

#[test]
fn test_display() {
    let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::empty();
    let pq = pq.insert(5).insert(3).insert(7);

    let display_str = format!("{pq}");
    assert!(display_str.contains("BalancedTreePQ"));
    assert!(display_str.contains("3"));
    assert!(display_str.contains("5"));
    assert!(display_str.contains("7"));
}
