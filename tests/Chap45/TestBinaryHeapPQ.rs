//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BinaryHeapPQ.

use apas_verus::BinaryHeapPQLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
use apas_verus::Types::Types::*;

#[test]
fn test_binaryheappqlit_macro_functionality() {
    // Test empty heap creation
    let empty: BinaryHeapPQ<i32> = BinaryHeapPQLit![];
    assert_eq!(empty.size(), 0);

    // Test heap creation with elements
    let with_data: BinaryHeapPQ<i32> = BinaryHeapPQLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
}

#[test]
fn test_empty() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    assert!(heap.is_empty());
    assert_eq!(heap.size(), 0);
    assert_eq!(heap.find_min(), None);
}

#[test]
fn test_singleton() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::singleton(42);
    assert!(!heap.is_empty());
    assert_eq!(heap.size(), 1);
    assert_eq!(heap.find_min(), Some(&42));
}

#[test]
fn test_insert_maintains_heap_property() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_find_min() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    assert_eq!(heap.find_min(), None);

    let heap = heap.insert(10);
    assert_eq!(heap.find_min(), Some(&10));

    let heap = heap.insert(5);
    assert_eq!(heap.find_min(), Some(&5));

    let heap = heap.insert(15);
    assert_eq!(heap.find_min(), Some(&5));
}

#[test]
fn test_delete_min() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(3));
    assert_eq!(heap.find_min(), Some(&5));
    assert_eq!(heap.size(), 4);
    assert!(heap.is_valid_heap());

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(5));
    assert_eq!(heap.size(), 3);
    assert!(heap.is_valid_heap());
}

#[test]
fn test_delete_min_empty() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let (heap, min) = heap.delete_min();
    assert_eq!(min, None);
    assert!(heap.is_empty());
}

#[test]
fn test_meld() {
    let heap1: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap1 = heap1.insert(10).insert(5).insert(15);

    let heap2: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap2 = heap2.insert(2).insert(8).insert(12);

    let melded = heap1.meld(&heap2);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&2));
    assert!(melded.is_valid_heap());
}

#[test]
fn test_meld_with_empty() {
    let heap1: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap1 = heap1.insert(10).insert(5);

    let heap2: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();

    let melded = heap1.meld(&heap2);
    assert_eq!(melded.size(), 2);
    assert_eq!(melded.find_min(), Some(&5));
}

#[test]
fn test_from_seq() {
    use apas_verus::ArraySeqStPerSLit;
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};

    let seq = ArraySeqStPerSLit![10, 5, 15, 3, 8, 12];
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::from_seq(&seq);

    assert_eq!(heap.size(), 6);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_to_seq() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15);

    let seq = heap.to_seq();
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_insert_all() {
    use apas_verus::ArraySeqStPerSLit;
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;

    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::singleton(5);
    let new_elements = ArraySeqStPerSLit![10, 2, 8, 12];

    let heap = heap.insert_all(&new_elements);
    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&2));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_extract_all_sorted() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted = heap.extract_all_sorted();
    assert_eq!(sorted.length(), 5);
    assert_eq!(*sorted.nth(0), 3);
    assert_eq!(*sorted.nth(1), 5);
    assert_eq!(*sorted.nth(2), 8);
    assert_eq!(*sorted.nth(3), 10);
    assert_eq!(*sorted.nth(4), 15);
}

#[test]
fn test_is_valid_heap() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    assert!(heap.is_valid_heap());

    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);
    assert!(heap.is_valid_heap());
}

#[test]
fn test_height() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    assert_eq!(heap.height(), 0);

    let heap = heap.insert(10);
    assert_eq!(heap.height(), 1);

    let heap = heap.insert(5).insert(15);
    assert!(heap.height() >= 1);
}

#[test]
fn test_level_elements() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap
        .insert(1)
        .insert(3)
        .insert(2)
        .insert(7)
        .insert(5)
        .insert(4)
        .insert(6);

    // After heapify, root should be minimum
    let level0 = heap.level_elements(0);
    assert_eq!(level0.length(), 1);
    assert_eq!(*level0.nth(0), 1);

    // Level 1 should have at most 2 elements
    let level1 = heap.level_elements(1);
    assert!(level1.length() <= 2);
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 5, 15, 3, 8];
    let heap = BinaryHeapPQ::from_vec(vec);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_to_vec() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15);

    let vec = heap.to_vec();
    assert_eq!(vec.len(), 3);
}

#[test]
fn test_to_sorted_vec() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted_vec = heap.to_sorted_vec();
    assert_eq!(sorted_vec.len(), 5);
    assert_eq!(sorted_vec, vec![3, 5, 8, 10, 15]);
}

#[test]
fn test_default() {
    let heap = BinaryHeapPQ::<i32>::default();
    assert!(heap.is_empty());
    assert_eq!(heap.size(), 0);
}

#[test]
fn test_duplicate_elements() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(5).insert(3).insert(5);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_string_heap() {
    let heap: BinaryHeapPQ<String> = BinaryHeapPQTrait::empty();
    let heap = heap
        .insert("zebra".to_string())
        .insert("apple".to_string())
        .insert("banana".to_string())
        .insert("cherry".to_string());

    assert_eq!(heap.find_min(), Some(&"apple".to_string()));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_heapsort_via_extract() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let elements = vec![42, 17, 89, 3, 56, 23, 71, 8];

    let mut heap = heap;
    for &elem in &elements {
        heap = heap.insert(elem);
    }

    let sorted = heap.extract_all_sorted();

    let mut expected = elements.clone();
    expected.sort();

    for (i, &expected_val) in expected.iter().enumerate() {
        assert_eq!(*sorted.nth(i as N), expected_val);
    }
}

#[test]
fn test_sequential_delete_all() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(8).insert(1).insert(10);

    let mut current_heap = heap;
    let mut extracted = vec![];

    while !current_heap.is_empty() {
        let (new_heap, min_val) = current_heap.delete_min();
        if let Some(val) = min_val {
            extracted.push(val);
        }
        current_heap = new_heap;
    }

    assert_eq!(extracted, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_large_heap() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let mut heap = heap;

    // Insert 100 elements
    let elements = ArraySeqStPerS::<i32>::tabulate(&|i| (i as i32 * 17 + 13) % 97, 100);
    for i in 0..elements.length() {
        heap = heap.insert(*elements.nth(i));
    }

    assert_eq!(heap.size(), 100);
    assert!(heap.is_valid_heap());

    // Extract first 10 elements to verify ordering
    let mut current_heap = heap;
    let mut prev_min = -1;

    for _ in 0..10 {
        let (new_heap, min_val) = current_heap.delete_min();
        if let Some(val) = min_val {
            assert!(val >= prev_min);
            prev_min = val;
        }
        current_heap = new_heap;
        assert!(current_heap.is_valid_heap());
    }
}

#[test]
fn test_persistent_semantics() {
    let heap1: BinaryHeapPQ<i32> = BinaryHeapPQTrait::singleton(5);
    let heap2 = heap1.insert(10);

    // heap1 should remain unchanged (persistent)
    assert_eq!(heap1.size(), 1);
    assert_eq!(heap1.find_min(), Some(&5));

    // heap2 should have both elements
    assert_eq!(heap2.size(), 2);
    assert_eq!(heap2.find_min(), Some(&5));
}

#[test]
fn test_meld_multiple() {
    let heap1: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap1 = heap1.insert(1).insert(5);
    let heap2: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap2 = heap2.insert(3).insert(7);
    let heap3: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap3 = heap3.insert(2).insert(6);

    let melded = heap1.meld(&heap2).meld(&heap3);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&1));
    assert!(melded.is_valid_heap());
}

#[test]
fn test_display() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(7);

    let display_str = format!("{heap}");
    assert!(display_str.contains("BinaryHeapPQ"));
}

#[test]
fn test_heap_property_after_operations() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();

    // Insert sequence
    let heap = heap.insert(50).insert(30).insert(70).insert(20).insert(40);
    assert!(heap.is_valid_heap());

    // Delete and check
    let (heap, _) = heap.delete_min();
    assert!(heap.is_valid_heap());

    // Meld and check
    let heap2: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap2 = heap2.insert(10).insert(60);
    let melded = heap.meld(&heap2);
    assert!(melded.is_valid_heap());
}

#[test]
fn test_single_element_operations() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::singleton(42);

    assert_eq!(heap.find_min(), Some(&42));
    assert_eq!(heap.size(), 1);

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(42));
    assert!(heap.is_empty());
}

#[test]
fn test_ascending_insertion() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(1).insert(2).insert(3).insert(4).insert(5);

    assert_eq!(heap.find_min(), Some(&1));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_descending_insertion() {
    let heap: BinaryHeapPQ<i32> = BinaryHeapPQTrait::empty();
    let heap = heap.insert(5).insert(4).insert(3).insert(2).insert(1);

    assert_eq!(heap.find_min(), Some(&1));
    assert!(heap.is_valid_heap());
}

#[test]
fn test_macro_type_check() { let _heap: BinaryHeapPQ<i32> = BinaryHeapPQLit![1, 2, 3]; }
