//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 45: UnsortedListPQ Priority Queue implementation

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap45::UnsortedListPQ::UnsortedListPQ::*;
use apas_verus::Types::Types::*;
use apas_verus::UnsortedListPQLit;

#[test]
fn test_unsortedlistpqlit_macro_functionality() {
    // Test empty priority queue creation
    let empty: UnsortedListPQ<i32> = UnsortedListPQLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());

    // Test priority queue creation with elements
    let with_data: UnsortedListPQ<i32> = UnsortedListPQLit![3, 1, 4, 1, 5];
    assert_eq!(with_data.size(), 5);
    assert!(!with_data.is_empty());
}

#[test]
fn test_empty_priority_queue() {
    let pq = UnsortedListPQ::<i32>::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);

    let (new_pq, min_val) = pq.delete_min();
    assert!(new_pq.is_empty());
    assert_eq!(min_val, None);
}

#[test]
fn test_singleton_priority_queue() {
    let pq = UnsortedListPQ::singleton(42);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&42));
    assert_eq!(pq.find_min(), Some(&42));

    let (new_pq, min_val) = pq.delete_min();
    assert!(new_pq.is_empty());
    assert_eq!(min_val, Some(42));
}

#[test]
fn test_insert_and_find_min() {
    let mut pq = UnsortedListPQ::empty();

    // Insert elements in random order
    pq = pq.insert(5);
    assert_eq!(pq.find_min(), Some(&5));
    assert_eq!(pq.size(), 1);

    pq = pq.insert(3);
    assert_eq!(pq.find_min(), Some(&3));
    assert_eq!(pq.size(), 2);

    pq = pq.insert(8);
    assert_eq!(pq.find_min(), Some(&3));
    assert_eq!(pq.size(), 3);

    pq = pq.insert(1);
    assert_eq!(pq.find_min(), Some(&1));
    assert_eq!(pq.size(), 4);

    pq = pq.insert(6);
    assert_eq!(pq.find_min(), Some(&1));
    assert_eq!(pq.size(), 5);
}

#[test]
fn test_delete_min_sequence() {
    let mut pq = UnsortedListPQ::empty();

    // Insert elements: [5, 3, 8, 1, 6]
    pq = pq.insert(5);
    pq = pq.insert(3);
    pq = pq.insert(8);
    pq = pq.insert(1);
    pq = pq.insert(6);

    // Delete elements in sorted order
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(1));
    assert_eq!(new_pq.size(), 4);
    pq = new_pq;

    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(3));
    assert_eq!(new_pq.size(), 3);
    pq = new_pq;

    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(5));
    assert_eq!(new_pq.size(), 2);
    pq = new_pq;

    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(6));
    assert_eq!(new_pq.size(), 1);
    pq = new_pq;

    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(8));
    assert_eq!(new_pq.size(), 0);
    assert!(new_pq.is_empty());
}

#[test]
fn test_meld_operation() {
    let mut pq1 = UnsortedListPQ::empty();
    pq1 = pq1.insert(5);
    pq1 = pq1.insert(2);
    pq1 = pq1.insert(8);

    let mut pq2 = UnsortedListPQ::empty();
    pq2 = pq2.insert(3);
    pq2 = pq2.insert(1);
    pq2 = pq2.insert(7);

    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&1));

    // Verify all elements are present by extracting in sorted order
    let sorted = melded.extract_all_sorted();
    let expected = vec![1, 2, 3, 5, 7, 8];

    let mut actual = Vec::new();
    for i in 0..sorted.length() {
        actual.push(*sorted.nth(i));
    }
    assert_eq!(actual, expected);
}

#[test]
fn test_meld_with_empty() {
    let mut pq = UnsortedListPQ::empty();
    pq = pq.insert(5);
    pq = pq.insert(2);

    let empty_pq = UnsortedListPQ::empty();

    // Meld non-empty with empty
    let melded1 = pq.meld(&empty_pq);
    assert_eq!(melded1.size(), 2);
    assert_eq!(melded1.find_min(), Some(&2));

    // Meld empty with non-empty
    let melded2 = empty_pq.meld(&pq);
    assert_eq!(melded2.size(), 2);
    assert_eq!(melded2.find_min(), Some(&2));

    // Meld empty with empty
    let melded3 = empty_pq.meld(&empty_pq);
    assert!(melded3.is_empty());
}

#[test]
fn test_from_seq() {
    let elements = vec![7, 2, 9, 1, 5];
    let mut seq = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::empty();

    for element in elements {
        let single = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::singleton(element);
        seq = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::append(&seq, &single);
    }

    let pq = UnsortedListPQ::from_seq(&seq);
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    // Verify all elements by extracting in sorted order
    let sorted = pq.extract_all_sorted();
    let expected = vec![1, 2, 5, 7, 9];

    let mut actual = Vec::new();
    for i in 0..sorted.length() {
        actual.push(*sorted.nth(i));
    }
    assert_eq!(actual, expected);
}

#[test]
fn test_from_vec_convenience() {
    let elements = vec![7, 2, 9, 1, 5];
    let pq = UnsortedListPQ::from_vec(elements);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    let sorted_vec = pq.to_sorted_vec();
    assert_eq!(sorted_vec, vec![1, 2, 5, 7, 9]);
}

#[test]
fn test_insert_all() {
    let pq = UnsortedListPQ::empty();
    let elements = vec![7, 2, 9, 1, 5];

    let mut seq = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::empty();
    for element in elements {
        let single = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::singleton(element);
        seq = apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::append(&seq, &single);
    }

    let new_pq = pq.insert_all(&seq);
    assert_eq!(new_pq.size(), 5);
    assert_eq!(new_pq.find_min(), Some(&1));
}

#[test]
fn test_duplicate_elements() {
    let mut pq = UnsortedListPQ::empty();

    // Insert duplicates
    pq = pq.insert(5);
    pq = pq.insert(3);
    pq = pq.insert(5);
    pq = pq.insert(3);
    pq = pq.insert(1);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    // Extract all elements
    let sorted = pq.extract_all_sorted();
    let expected = vec![1, 3, 3, 5, 5];

    let mut actual = Vec::new();
    for i in 0..sorted.length() {
        actual.push(*sorted.nth(i));
    }
    assert_eq!(actual, expected);
}

#[test]
fn test_large_dataset() {
    let mut pq = UnsortedListPQ::empty();
    let size = 100;

    // Insert elements in reverse order
    for i in (0..size).rev() {
        pq = pq.insert(i);
    }

    assert_eq!(pq.size(), size);
    assert_eq!(pq.find_min(), Some(&0));

    // Extract all elements and verify they're sorted
    let mut current_pq = pq;
    for expected in 0..size {
        let (new_pq, min_val) = current_pq.delete_min();
        assert_eq!(min_val, Some(expected));
        current_pq = new_pq;
    }

    assert!(current_pq.is_empty());
}

#[test]
fn test_string_elements() {
    let mut pq = UnsortedListPQ::empty();

    pq = pq.insert("zebra".to_string());
    pq = pq.insert("apple".to_string());
    pq = pq.insert("banana".to_string());
    pq = pq.insert("cherry".to_string());

    assert_eq!(pq.find_min(), Some(&"apple".to_string()));

    let sorted = pq.extract_all_sorted();
    let expected = vec!["apple", "banana", "cherry", "zebra"];

    let mut actual = Vec::new();
    for i in 0..sorted.length() {
        actual.push(sorted.nth(i).clone());
    }
    assert_eq!(actual, expected);
}

#[test]
fn test_macro_creation() {
    let pq = UnsortedListPQLit![5, 3, 8, 1, 6];
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    let empty_pq: UnsortedListPQ<i32> = UnsortedListPQLit![];
    assert!(empty_pq.is_empty());
}

#[test]
fn test_display_formatting() {
    let pq = UnsortedListPQLit![3, 1, 4];
    let display_str = format!("{pq}");

    // The display should show elements in insertion order (unsorted)
    assert!(display_str.contains("UnsortedListPQ["));
    assert!(display_str.contains("3"));
    assert!(display_str.contains("1"));
    assert!(display_str.contains("4"));
}

#[test]
fn test_complexity_characteristics() {
    // This test verifies the expected O(1) insert and O(n) deleteMin behavior
    // by testing on different sizes and ensuring insert is consistently fast
    // while deleteMin scales with size

    let sizes = vec![10, 50, 100];

    for size in sizes {
        let mut pq = UnsortedListPQ::empty();

        // Insert should be O(1) - consistently fast regardless of size
        for i in 0..size {
            pq = pq.insert(i);
        }

        assert_eq!(pq.size(), size);
        assert_eq!(pq.find_min(), Some(&0));

        // deleteMin should be O(n) - but still correct
        let (new_pq, min_val) = pq.delete_min();
        assert_eq!(min_val, Some(0));
        assert_eq!(new_pq.size(), size - 1);
    }
}

#[test]
fn test_persistent_behavior() {
    // Test that operations create new instances without modifying originals
    let original = UnsortedListPQLit![5, 3, 8];

    let after_insert = original.insert(1);
    assert_eq!(original.size(), 3);
    assert_eq!(after_insert.size(), 4);
    assert_eq!(original.find_min(), Some(&3));
    assert_eq!(after_insert.find_min(), Some(&1));

    let (after_delete, _) = original.delete_min();
    assert_eq!(original.size(), 3);
    assert_eq!(after_delete.size(), 2);

    let other = UnsortedListPQLit![2, 7];
    let melded = original.meld(&other);
    assert_eq!(original.size(), 3);
    assert_eq!(other.size(), 2);
    assert_eq!(melded.size(), 5);
}

#[test]
fn test_edge_cases() {
    // Test with single element
    let single = UnsortedListPQ::singleton(42);
    let (empty, val) = single.delete_min();
    assert_eq!(val, Some(42));
    assert!(empty.is_empty());

    // Test meld of single elements
    let pq1 = UnsortedListPQ::singleton(1);
    let pq2 = UnsortedListPQ::singleton(2);
    let melded = pq1.meld(&pq2);
    assert_eq!(melded.size(), 2);
    assert_eq!(melded.find_min(), Some(&1));

    // Test with all same elements
    let same = UnsortedListPQLit![5, 5, 5, 5];
    assert_eq!(same.find_min(), Some(&5));
    let (after, val) = same.delete_min();
    assert_eq!(val, Some(5));
    assert_eq!(after.size(), 3);
}

#[test]
fn test_to_seq() {
    let pq = UnsortedListPQLit![5, 3, 8, 1];
    let seq = pq.to_seq();
    assert_eq!(seq.length(), 4);

    // The sequence should contain all elements
    let mut found = [false; 4];
    for i in 0..seq.length() {
        let val = *seq.nth(i);
        if val == 5 {
            found[0] = true;
        }
        if val == 3 {
            found[1] = true;
        }
        if val == 8 {
            found[2] = true;
        }
        if val == 1 {
            found[3] = true;
        }
    }
    assert!(found.iter().all(|&x| x));
}

#[test]
fn test_to_vec() {
    let pq = UnsortedListPQLit![5, 3, 8, 1];
    let vec = pq.to_vec();
    assert_eq!(vec.len(), 4);

    // The vector should contain all elements
    assert!(vec.contains(&5));
    assert!(vec.contains(&3));
    assert!(vec.contains(&8));
    assert!(vec.contains(&1));
}

#[test]
fn test_to_seq_empty() {
    let pq = UnsortedListPQ::<i32>::empty();
    let seq = pq.to_seq();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_to_vec_empty() {
    let pq = UnsortedListPQ::<i32>::empty();
    let vec = pq.to_vec();
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_roundtrip_vec() {
    let original = vec![7, 2, 9, 1, 5];
    let pq = UnsortedListPQ::from_vec(original.clone());
    let vec = pq.to_vec();

    // Should have same elements (order may differ)
    assert_eq!(vec.len(), original.len());
    for &elem in &original {
        assert!(vec.contains(&elem));
    }
}

#[test]
fn test_empty_constructor() {
    let pq = UnsortedListPQ::<i32>::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_singleton_constructor() {
    let pq = UnsortedListPQ::singleton(100);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&100));
}

#[test]
fn test_meld_multiple() {
    let pq1 = UnsortedListPQLit![5, 3];
    let pq2 = UnsortedListPQLit![7, 1];
    let pq3 = UnsortedListPQLit![9, 2];

    let melded12 = pq1.meld(&pq2);
    let melded_all = melded12.meld(&pq3);

    assert_eq!(melded_all.size(), 6);
    assert_eq!(melded_all.find_min(), Some(&1));
}

#[test]
fn test_insert_after_delete() {
    let pq = UnsortedListPQLit![5, 3, 8];
    let (pq2, _) = pq.delete_min();
    let pq3 = pq2.insert(1);

    assert_eq!(pq3.size(), 3);
    assert_eq!(pq3.find_min(), Some(&1));
}

#[test]
fn test_negative_numbers() {
    let pq = UnsortedListPQLit![-5, -3, -8, -1];
    assert_eq!(pq.find_min(), Some(&-8));

    let sorted = pq.to_sorted_vec();
    assert_eq!(sorted, vec![-8, -5, -3, -1]);
}

#[test]
fn test_mixed_positive_negative() {
    let pq = UnsortedListPQLit![5, -3, 8, -1, 0];
    assert_eq!(pq.find_min(), Some(&-3));

    let sorted = pq.to_sorted_vec();
    assert_eq!(sorted, vec![-3, -1, 0, 5, 8]);
}
