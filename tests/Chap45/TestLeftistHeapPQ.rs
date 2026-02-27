//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for LeftistHeapPQ.

use apas_verus::Chap45::LeftistHeapPQ::LeftistHeapPQ::*;
use apas_verus::LeftistHeapPQLit;
use apas_verus::Types::Types::*;

#[test]
fn test_leftistheappqlit_macro_functionality() {
    // Test empty heap creation
    let empty: LeftistHeapPQ<i32> = LeftistHeapPQLit![];
    assert_eq!(empty.size(), 0);

    // Test heap creation with elements
    let with_data: LeftistHeapPQ<i32> = LeftistHeapPQLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
}

#[test]
fn test_empty() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    assert!(heap.is_empty());
    assert_eq!(heap.size(), 0);
    assert_eq!(heap.find_min(), None);
}

#[test]
fn test_singleton() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::singleton(42);
    assert!(!heap.is_empty());
    assert_eq!(heap.size(), 1);
    assert_eq!(heap.find_min(), Some(&42));
}

#[test]
fn test_insert() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_find_min() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    assert_eq!(heap.find_min(), None);

    let heap = heap.insert(10);
    assert_eq!(heap.find_min(), Some(&10));

    let heap = heap.insert(5);
    assert_eq!(heap.find_min(), Some(&5));
}

#[test]
fn test_delete_min() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(3));
    assert_eq!(heap.find_min(), Some(&5));
    assert_eq!(heap.size(), 4);
    assert!(heap.is_valid_leftist_heap());

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(5));
    assert_eq!(heap.size(), 3);
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_delete_min_empty() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let (heap, min) = heap.delete_min();
    assert_eq!(min, None);
    assert!(heap.is_empty());
}

#[test]
fn test_meld() {
    let heap1: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap1 = heap1.insert(10).insert(5).insert(15);

    let heap2: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap2 = heap2.insert(2).insert(8).insert(12);

    let melded = heap1.meld(&heap2);
    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&2));
    assert!(melded.is_valid_leftist_heap());
}

#[test]
fn test_meld_with_empty() {
    let heap1: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap1 = heap1.insert(10).insert(5);

    let heap2: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();

    let melded = heap1.meld(&heap2);
    assert_eq!(melded.size(), 2);
    assert_eq!(melded.find_min(), Some(&5));
}

#[test]
fn test_from_seq() {
    let seq = vec![10, 5, 15, 3, 8, 12];
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::from_vec(seq);

    assert_eq!(heap.size(), 6);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_extract_all_sorted() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted = heap.extract_all_sorted();
    assert_eq!(sorted.len(), 5);
    assert_eq!(sorted, vec![3, 5, 8, 10, 15]);
}

#[test]
fn test_height() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    assert_eq!(heap.height(), 0);

    let heap = heap.insert(10);
    assert_eq!(heap.height(), 1);

    let heap = heap.insert(5).insert(15);
    assert!(heap.height() >= 1);
}

#[test]
fn test_root_rank() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    assert_eq!(heap.root_rank(), 0);

    let heap = heap.insert(10).insert(5).insert(15).insert(3);
    assert!(heap.root_rank() > 0);
}

#[test]
fn test_is_valid_leftist_heap() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    assert!(heap.is_valid_leftist_heap());

    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 5, 15, 3, 8];
    let heap = LeftistHeapPQ::from_vec(vec);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_to_vec() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15);

    let vec = heap.to_vec();
    assert_eq!(vec.len(), 3);
}

#[test]
fn test_to_sorted_vec() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(10).insert(5).insert(15).insert(3).insert(8);

    let sorted = heap.to_sorted_vec();
    assert_eq!(sorted, vec![3, 5, 8, 10, 15]);
}

#[test]
fn test_meld_multiple() {
    let heap1: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap1 = heap1.insert(1).insert(5);
    let heap2: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap2 = heap2.insert(3).insert(7);
    let heap3: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap3 = heap3.insert(2).insert(6);

    let heaps = vec![heap1, heap2, heap3];
    let melded = LeftistHeapPQ::meld_multiple(&heaps);

    assert_eq!(melded.size(), 6);
    assert_eq!(melded.find_min(), Some(&1));
    assert!(melded.is_valid_leftist_heap());
}

#[test]
fn test_meld_multiple_empty() {
    let heaps: Vec<LeftistHeapPQ<i32>> = vec![];
    let melded = LeftistHeapPQ::meld_multiple(&heaps);
    assert!(melded.is_empty());
}

#[test]
fn test_split() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap
        .insert(10)
        .insert(5)
        .insert(15)
        .insert(3)
        .insert(8)
        .insert(12)
        .insert(20);

    let (less, greater_eq) = heap.split(&10);

    // Verify less contains elements < 10
    assert!(less.size() > 0);
    if let Some(min) = less.find_min() {
        assert!(min < &10);
    }

    // Verify greater_eq contains elements >= 10
    assert!(greater_eq.size() > 0);
    if let Some(min) = greater_eq.find_min() {
        assert!(min >= &10);
    }
}

#[test]
fn test_persistent_semantics() {
    let heap1: LeftistHeapPQ<i32> = LeftistHeapPQTrait::singleton(5);
    let heap2 = heap1.insert(10);

    // heap1 should remain unchanged (persistent)
    assert_eq!(heap1.size(), 1);
    assert_eq!(heap1.find_min(), Some(&5));

    // heap2 should have both elements
    assert_eq!(heap2.size(), 2);
    assert_eq!(heap2.find_min(), Some(&5));
}

#[test]
fn test_duplicate_elements() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(5).insert(3).insert(5);

    assert_eq!(heap.size(), 5);
    assert_eq!(heap.find_min(), Some(&3));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_string_heap() {
    let heap: LeftistHeapPQ<String> = LeftistHeapPQTrait::empty();
    let heap = heap
        .insert("zebra".to_string())
        .insert("apple".to_string())
        .insert("banana".to_string())
        .insert("cherry".to_string());

    assert_eq!(heap.find_min(), Some(&"apple".to_string()));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_sequential_delete_all() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(8).insert(1).insert(10);

    let mut current_heap = heap;
    let mut extracted = vec![];

    while !current_heap.is_empty() {
        let (new_heap, min) = current_heap.delete_min();
        if let Some(val) = min {
            extracted.push(val);
        }
        current_heap = new_heap;
    }

    assert_eq!(extracted, vec![1, 3, 5, 8, 10]);
}

#[test]
fn test_large_heap() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let mut heap = heap;

    // Insert 100 elements
    for i in 0..100 {
        heap = heap.insert((i * 17 + 13) % 97);
    }

    assert_eq!(heap.size(), 100);
    assert!(heap.is_valid_leftist_heap());

    // Extract first 10 elements to verify ordering
    let mut current_heap = heap;
    let mut prev_min = -1;

    for _ in 0..10 {
        let (new_heap, min) = current_heap.delete_min();
        if let Some(val) = min {
            assert!(val >= prev_min);
            prev_min = val;
        }
        current_heap = new_heap;
        assert!(current_heap.is_valid_leftist_heap());
    }
}

#[test]
fn test_leftist_property_after_meld() {
    let heap1: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap1 = heap1.insert(1).insert(3).insert(5);

    let heap2: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap2 = heap2.insert(2).insert(4).insert(6);

    let melded = heap1.meld(&heap2);
    assert!(melded.is_valid_leftist_heap());

    // Verify leftist property maintained through operations
    let (melded, _) = melded.delete_min();
    assert!(melded.is_valid_leftist_heap());
}

#[test]
fn test_single_element_operations() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::singleton(42);

    assert_eq!(heap.find_min(), Some(&42));
    assert_eq!(heap.size(), 1);
    assert_eq!(heap.height(), 1);

    let (heap, min) = heap.delete_min();
    assert_eq!(min, Some(42));
    assert!(heap.is_empty());
}

#[test]
fn test_ascending_insertion() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(1).insert(2).insert(3).insert(4).insert(5);

    assert_eq!(heap.find_min(), Some(&1));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_descending_insertion() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(5).insert(4).insert(3).insert(2).insert(1);

    assert_eq!(heap.find_min(), Some(&1));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_meld_efficiency() {
    // Test that multiple melds maintain efficiency
    let mut heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();

    for i in 0..10 {
        let new_heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::singleton(i);
        heap = heap.meld(&new_heap);
    }

    assert_eq!(heap.size(), 10);
    assert_eq!(heap.find_min(), Some(&0));
    assert!(heap.is_valid_leftist_heap());
}

#[test]
fn test_default() {
    let heap = LeftistHeapPQ::<i32>::default();
    assert!(heap.is_empty());
    assert_eq!(heap.size(), 0);
}

#[test]
fn test_display() {
    let heap: LeftistHeapPQ<i32> = LeftistHeapPQTrait::empty();
    let heap = heap.insert(5).insert(3).insert(7);

    let display_str = format!("{heap}");
    assert!(display_str.contains("LeftistHeapPQ"));
}

#[test]
fn test_efficient_multi_way_merge() {
    let seq1 = vec![1, 4, 7];
    let seq2 = vec![2, 5, 8];
    let seq3 = vec![3, 6, 9];
    
    let result = efficient_multi_way_merge(vec![seq1, seq2, seq3]);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // Test with empty sequences
    let result_empty = efficient_multi_way_merge::<i32>(vec![]);
    assert_eq!(result_empty, vec![]);

    // Test with single sequence
    let result_single = efficient_multi_way_merge(vec![vec![5, 10, 15]]);
    assert_eq!(result_single, vec![5, 10, 15]);
}

#[test]
fn test_parallel_heap_construction() {
    let elements = vec![5, 2, 8, 1, 9, 3, 7];
    let heap = parallel_heap_construction(elements);
    
    assert_eq!(heap.size(), 7);
    assert_eq!(heap.find_min(), Some(&1));
    assert!(heap.is_valid_leftist_heap());
}
