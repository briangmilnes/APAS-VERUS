//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 45: SortedListPQ - Priority Queue using Sorted List

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap45::SortedListPQ::SortedListPQ::*;
use apas_verus::SortedListPQLit;
use apas_verus::Types::Types::*;

#[test]
fn test_sortedlistpqlit_macro_functionality() {
    // Test empty priority queue creation
    let empty: SortedListPQ<i32> = SortedListPQLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());

    // Test priority queue creation with elements
    let with_data: SortedListPQ<i32> = SortedListPQLit![3, 1, 4, 1, 5];
    assert_eq!(with_data.size(), 5);
    assert!(!with_data.is_empty());
}

#[test]
fn test_empty_priority_queue() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_singleton_priority_queue() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::singleton(42);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.find_min(), Some(&42));
}

#[test]
fn test_insert_and_find_min() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(10);
    pq = pq.insert(5);
    pq = pq.insert(20);
    assert_eq!(pq.size(), 3);
    assert_eq!(pq.find_min(), Some(&5));

    // Verify sorted order is maintained
    let seq = pq.to_seq();
    assert_eq!(seq.nth(0), &5);
    assert_eq!(seq.nth(1), &10);
    assert_eq!(seq.nth(2), &20);
}

#[test]
fn test_delete_min() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(10);
    pq = pq.insert(5);
    pq = pq.insert(20);

    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(5));
    assert_eq!(new_pq.size(), 2);
    assert_eq!(new_pq.find_min(), Some(&10));
}

#[test]
fn test_delete_min_from_singleton() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::singleton(42);
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some(42));
    assert!(new_pq.is_empty());
}

#[test]
fn test_delete_min_from_empty() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let (new_pq, min_val) = pq.delete_min();
    assert_eq!(min_val, None);
    assert!(new_pq.is_empty());
}

#[test]
fn test_meld_priority_queues() {
    let pq1 = {
        let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
        p = p.insert(10);
        p = p.insert(20);
        p
    };

    let pq2 = {
        let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
        p = p.insert(5);
        p = p.insert(15);
        p = p.insert(25);
        p
    };

    let merged_pq = pq1.meld(&pq2);
    assert_eq!(merged_pq.size(), 5);
    assert_eq!(merged_pq.find_min(), Some(&5));

    // Verify sorted order after meld
    let seq = merged_pq.to_seq();
    assert_eq!(seq.nth(0), &5);
    assert_eq!(seq.nth(1), &10);
    assert_eq!(seq.nth(2), &15);
    assert_eq!(seq.nth(3), &20);
    assert_eq!(seq.nth(4), &25);
}

#[test]
fn test_meld_with_empty() {
    let pq1 = {
        let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
        p = p.insert(10);
        p = p.insert(5);
        p
    };
    let pq2: SortedListPQ<i32> = SortedListPQTrait::empty();

    let merged = pq1.meld(&pq2);
    assert_eq!(merged.size(), 2);
    assert_eq!(merged.find_min(), Some(&5));

    let merged_reverse = pq2.meld(&pq1);
    assert_eq!(merged_reverse.size(), 2);
    assert_eq!(merged_reverse.find_min(), Some(&5));
}

#[test]
fn test_from_seq() {
    let seq = ArraySeqStPerS::from_vec(vec![5, 2, 8, 1, 9]);
    let pq: SortedListPQ<i32> = SortedListPQTrait::from_seq(&seq);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    // Verify elements are sorted
    let sorted_seq = pq.to_seq();
    assert_eq!(sorted_seq.nth(0), &1);
    assert_eq!(sorted_seq.nth(1), &2);
    assert_eq!(sorted_seq.nth(2), &5);
    assert_eq!(sorted_seq.nth(3), &8);
    assert_eq!(sorted_seq.nth(4), &9);
}

#[test]
fn test_from_empty_seq() {
    let empty_seq = <ArraySeqStPerS<i32> as ArraySeqStPerTrait<i32>>::empty();
    let pq: SortedListPQ<i32> = SortedListPQTrait::from_seq(&empty_seq);
    assert!(pq.is_empty());
    assert_eq!(pq.find_min(), None);
}

#[test]
fn test_multiple_inserts_and_deletes() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();

    // Insert elements in random order
    pq = pq.insert(30);
    pq = pq.insert(10);
    pq = pq.insert(40);
    pq = pq.insert(20);
    pq = pq.insert(5);

    assert_eq!(pq.find_min(), Some(&5));

    // Delete in sorted order
    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(5));
    assert_eq!(pq.find_min(), Some(&10));

    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(10));
    assert_eq!(pq.find_min(), Some(&20));

    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(20));
    assert_eq!(pq.find_min(), Some(&30));

    let (pq, min4) = pq.delete_min();
    assert_eq!(min4, Some(30));
    assert_eq!(pq.find_min(), Some(&40));

    let (pq, min5) = pq.delete_min();
    assert_eq!(min5, Some(40));
    assert!(pq.is_empty());
}

#[test]
fn test_string_elements() {
    let mut pq: SortedListPQ<String> = SortedListPQTrait::empty();
    pq = pq.insert("banana".to_string());
    pq = pq.insert("apple".to_string());
    pq = pq.insert("zebra".to_string());

    assert_eq!(pq.find_min(), Some(&"apple".to_string()));

    let (pq, min_val) = pq.delete_min();
    assert_eq!(min_val, Some("apple".to_string()));
    assert_eq!(pq.find_min(), Some(&"banana".to_string()));
}

#[test]
fn test_duplicate_elements() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(5);
    pq = pq.insert(5);
    pq = pq.insert(3);
    pq = pq.insert(5);

    assert_eq!(pq.size(), 4);
    assert_eq!(pq.find_min(), Some(&3));

    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(3));

    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(5));

    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(5));

    let (pq, min4) = pq.delete_min();
    assert_eq!(min4, Some(5));

    assert!(pq.is_empty());
}

#[test]
fn test_large_sequence() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();

    // Insert many elements
    for i in (1..=100).rev() {
        pq = pq.insert(i);
    }

    assert_eq!(pq.size(), 100);
    assert_eq!(pq.find_min(), Some(&1));

    // Verify sorted order
    for expected in 1..=100 {
        let (new_pq, min_val) = pq.delete_min();
        assert_eq!(min_val, Some(expected));
        pq = new_pq;
    }

    assert!(pq.is_empty());
}

#[test]
fn test_meld_large_queues() {
    let pq1 = {
        let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
        for i in (1..=50).step_by(2) {
            // Odd numbers
            p = p.insert(i);
        }
        p
    };

    let pq2 = {
        let mut p: SortedListPQ<i32> = SortedListPQTrait::empty();
        for i in (2..=50).step_by(2) {
            // Even numbers
            p = p.insert(i);
        }
        p
    };

    let merged = pq1.meld(&pq2);
    assert_eq!(merged.size(), 50);
    assert_eq!(merged.find_min(), Some(&1));

    // Verify all numbers 1-50 are present in order
    let mut current_pq = merged;
    for expected in 1..=50 {
        let (new_pq, min_val) = current_pq.delete_min();
        assert_eq!(min_val, Some(expected));
        current_pq = new_pq;
    }

    assert!(current_pq.is_empty());
}

#[test]
fn test_persistent_nature() {
    let pq1: SortedListPQ<i32> = SortedListPQTrait::singleton(10);
    let pq2 = pq1.insert(5);
    let pq3 = pq2.insert(15);

    // Original queues should be unchanged
    assert_eq!(pq1.size(), 1);
    assert_eq!(pq1.find_min(), Some(&10));

    assert_eq!(pq2.size(), 2);
    assert_eq!(pq2.find_min(), Some(&5));

    assert_eq!(pq3.size(), 3);
    assert_eq!(pq3.find_min(), Some(&5));

    // Test delete_min persistence
    let (pq4, deleted) = pq3.delete_min();
    assert_eq!(deleted, Some(5));
    assert_eq!(pq4.size(), 2);
    assert_eq!(pq4.find_min(), Some(&10));

    // pq3 should be unchanged
    assert_eq!(pq3.size(), 3);
    assert_eq!(pq3.find_min(), Some(&5));
}

#[test]
fn test_to_seq_conversion() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let values = vec![30, 10, 20, 5, 25];

    for val in values {
        pq = pq.insert(val);
    }

    let seq = pq.to_seq();
    assert_eq!(seq.length(), 5);

    // Should be in sorted order
    let expected = [5, 10, 20, 25, 30];
    for (i, &expected_val) in expected.iter().enumerate() {
        assert_eq!(seq.nth(i as N), &expected_val);
    }
}

#[test]
fn test_negative_numbers() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(-5);
    pq = pq.insert(-3);
    pq = pq.insert(-8);
    pq = pq.insert(-1);

    assert_eq!(pq.find_min(), Some(&-8));

    let (pq, min1) = pq.delete_min();
    assert_eq!(min1, Some(-8));

    let (pq, min2) = pq.delete_min();
    assert_eq!(min2, Some(-5));

    let (pq, min3) = pq.delete_min();
    assert_eq!(min3, Some(-3));

    let (pq, min4) = pq.delete_min();
    assert_eq!(min4, Some(-1));

    assert!(pq.is_empty());
}

#[test]
fn test_mixed_positive_negative() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(5);
    pq = pq.insert(-3);
    pq = pq.insert(0);
    pq = pq.insert(-7);
    pq = pq.insert(2);

    assert_eq!(pq.find_min(), Some(&-7));

    let seq = pq.to_seq();
    assert_eq!(seq.nth(0), &-7);
    assert_eq!(seq.nth(1), &-3);
    assert_eq!(seq.nth(2), &0);
    assert_eq!(seq.nth(3), &2);
    assert_eq!(seq.nth(4), &5);
}

#[test]
fn test_to_seq_empty() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let seq = pq.to_seq();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_meld_same_elements() {
    let mut pq1: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq1 = pq1.insert(5);
    pq1 = pq1.insert(10);

    let mut pq2: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq2 = pq2.insert(5);
    pq2 = pq2.insert(10);

    let merged = pq1.meld(&pq2);
    assert_eq!(merged.size(), 4);
    assert_eq!(merged.find_min(), Some(&5));

    let seq = merged.to_seq();
    assert_eq!(seq.nth(0), &5);
    assert_eq!(seq.nth(1), &5);
    assert_eq!(seq.nth(2), &10);
    assert_eq!(seq.nth(3), &10);
}

#[test]
fn test_insert_maintains_order() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();

    // Insert in various orders
    pq = pq.insert(50);
    let seq1 = pq.to_seq();
    assert_eq!(seq1.nth(0), &50);

    pq = pq.insert(30);
    let seq2 = pq.to_seq();
    assert_eq!(seq2.nth(0), &30);
    assert_eq!(seq2.nth(1), &50);

    pq = pq.insert(70);
    let seq3 = pq.to_seq();
    assert_eq!(seq3.nth(0), &30);
    assert_eq!(seq3.nth(1), &50);
    assert_eq!(seq3.nth(2), &70);

    pq = pq.insert(40);
    let seq4 = pq.to_seq();
    assert_eq!(seq4.nth(0), &30);
    assert_eq!(seq4.nth(1), &40);
    assert_eq!(seq4.nth(2), &50);
    assert_eq!(seq4.nth(3), &70);
}

#[test]
fn test_from_seq_with_duplicates() {
    let seq = ArraySeqStPerS::from_vec(vec![5, 2, 5, 1, 2]);
    let pq: SortedListPQ<i32> = SortedListPQTrait::from_seq(&seq);

    assert_eq!(pq.size(), 5);
    let sorted = pq.to_seq();
    assert_eq!(sorted.nth(0), &1);
    assert_eq!(sorted.nth(1), &2);
    assert_eq!(sorted.nth(2), &2);
    assert_eq!(sorted.nth(3), &5);
    assert_eq!(sorted.nth(4), &5);
}

#[test]
fn test_zero_value() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(0);
    pq = pq.insert(5);
    pq = pq.insert(-5);

    assert_eq!(pq.find_min(), Some(&-5));

    let (pq, _) = pq.delete_min();
    assert_eq!(pq.find_min(), Some(&0));
}

#[test]
fn test_macro_with_values() {
    let pq = SortedListPQLit![3, 1, 4, 1, 5];
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.find_min(), Some(&1));

    let seq = pq.to_seq();
    assert_eq!(seq.nth(0), &1);
    assert_eq!(seq.nth(1), &1);
    assert_eq!(seq.nth(2), &3);
    assert_eq!(seq.nth(3), &4);
    assert_eq!(seq.nth(4), &5);
}

#[test]
fn test_insert_all() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let seq = ArraySeqStPerS::from_vec(vec![5, 3, 7, 1]);
    let pq = pq.insert_all(&seq);

    assert_eq!(pq.size(), 4);
    assert_eq!(pq.find_min(), Some(&1));
}

#[test]
fn test_extract_all_sorted() {
    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(5);
    pq = pq.insert(2);
    pq = pq.insert(8);

    let sorted = pq.extract_all_sorted();
    assert_eq!(sorted.length(), 3);
    assert_eq!(sorted.nth(0), &2);
    assert_eq!(sorted.nth(1), &5);
    assert_eq!(sorted.nth(2), &8);
}

#[test]
fn test_find_max() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    assert_eq!(pq.find_max(), None);

    let pq = pq.insert(5);
    assert_eq!(pq.find_max(), Some(&5));

    let pq = pq.insert(10);
    assert_eq!(pq.find_max(), Some(&10));

    let pq = pq.insert(3);
    assert_eq!(pq.find_max(), Some(&10));
}

#[test]
fn test_delete_max() {
    let pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    let (pq2, max) = pq.delete_max();
    assert_eq!(max, None);
    assert!(pq2.is_empty());

    let mut pq: SortedListPQ<i32> = SortedListPQTrait::empty();
    pq = pq.insert(5);
    pq = pq.insert(10);
    pq = pq.insert(3);

    let (pq2, max) = pq.delete_max();
    assert_eq!(max, Some(10));
    assert_eq!(pq2.size(), 2);
    assert_eq!(pq2.find_max(), Some(&5));
}

#[test]
fn test_default_trait() {
    let pq: SortedListPQ<i32> = Default::default();
    assert!(pq.is_empty());
}

#[test]
fn test_display_trait() {
    let pq = SortedListPQLit![3, 1, 2];
    let s = format!("{}", pq);
    assert!(s.contains("1"));
    assert!(s.contains("2"));
    assert!(s.contains("3"));
}

#[test]
fn test_debug_trait() {
    let pq = SortedListPQLit![3, 1, 2];
    let s = format!("{:?}", pq);
    assert!(s.contains("SortedListPQ"));
}

#[test]
fn test_equality() {
    let pq1 = SortedListPQLit![1, 2, 3];
    let pq2 = SortedListPQLit![1, 2, 3];
    let pq3 = SortedListPQLit![1, 2, 4];

    assert_eq!(pq1, pq2);
    assert_ne!(pq1, pq3);
}

#[test]
fn test_from_vec() {
    let pq = SortedListPQ::from_vec(vec![5, 2, 8, 1]);
    assert_eq!(pq.size(), 4);
    assert_eq!(pq.find_min(), Some(&1));
}

#[test]
fn test_to_vec() {
    let pq = SortedListPQLit![3, 1, 2];
    let vec = pq.to_vec();
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_to_sorted_vec() {
    let pq = SortedListPQLit![3, 1, 2];
    let vec = pq.to_sorted_vec();
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_is_sorted() {
    let pq = SortedListPQLit![3, 1, 2];
    assert!(pq.is_sorted());

    let pq_empty: SortedListPQ<i32> = SortedListPQTrait::empty();
    assert!(pq_empty.is_sorted());
}
