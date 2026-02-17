//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq base trait functionality.

use apas_verus::Chap18::ArraySeq::ArraySeq::*;
use apas_verus::Chap37::AVLTreeSeq::AVLTreeSeq::*;
use apas_verus::Types::Types::*;

#[test]
fn test_avltreeseq_empty_constructor() {
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    assert_eq!(empty.length(), 0);
    assert!(empty.isEmpty());
    assert!(!empty.isSingleton());
}

#[test]
fn test_avltreeseq_new_constructor() {
    let new_tree: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::new();
    assert_eq!(new_tree.length(), 0);
    assert!(new_tree.isEmpty());
    assert!(!new_tree.isSingleton());
}

#[test]
fn test_avltreeseq_singleton_constructor() {
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    assert_eq!(single.length(), 1);
    assert!(!single.isEmpty());
    assert!(single.isSingleton());
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_avltreeseq_length_method() {
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    assert_eq!(empty.length(), 0);

    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(10);
    assert_eq!(single.length(), 1);
}

#[test]
fn test_avltreeseq_nth_method() {
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(99);
    assert_eq!(*single.nth(0), 99);
}

#[test]
#[should_panic]
fn test_avltreeseq_nth_panic_outofbounds() {
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    let _ = single.nth(1); // Index 1 is out of bounds for single element
}

#[test]
#[should_panic]
fn test_avltreeseq_nth_panic_empty() {
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    let _ = empty.nth(0); // Any index on empty tree should panic
}

#[test]
fn test_avltreeseq_set_method() {
    let mut single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    let result = single.set(0, 99);
    assert!(result.is_ok());
    assert_eq!(*single.nth(0), 99);
}

#[test]
fn test_avltreeseq_set_out_of_bounds_error() {
    let mut single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    let result = single.set(1, 99);
    assert!(result.is_err());
    if let Err(err_msg) = result {
        assert_eq!(err_msg, "Index out of bounds");
    }
}

#[test]
fn test_avltreeseq_predicates() {
    // Test isEmpty
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    assert!(empty.isEmpty());
    assert!(!empty.isSingleton());

    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    assert!(!single.isEmpty());
    assert!(single.isSingleton());
}

#[test]
fn test_avltreeseq_subseq_copy() {
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

    // Full subseq
    let full_subseq = single.subseq_copy(0, 1);
    assert_eq!(full_subseq.length(), 1);
    assert_eq!(*full_subseq.nth(0), 42);

    // Empty subseq
    let empty_subseq = single.subseq_copy(0, 0);
    assert_eq!(empty_subseq.length(), 0);
    assert!(empty_subseq.isEmpty());

    // Out of bounds subseq
    let oob_subseq = single.subseq_copy(1, 1);
    assert_eq!(oob_subseq.length(), 0);
    assert!(oob_subseq.isEmpty());
}

#[test]
fn test_avltreeseq_empty_operations_comprehensive() {
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();

    // Basic properties
    assert_eq!(empty.length(), 0);
    assert!(empty.isEmpty());
    assert!(!empty.isSingleton());

    // Subseq operations on empty tree
    let empty_subseq = empty.subseq_copy(0, 0);
    assert_eq!(empty_subseq.length(), 0);
    assert!(empty_subseq.isEmpty());

    let empty_subseq2 = empty.subseq_copy(0, 10);
    assert_eq!(empty_subseq2.length(), 0);
    assert!(empty_subseq2.isEmpty());
}

#[test]
fn test_avltreeseq_single_element_boundary() {
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

    // Basic properties
    assert_eq!(single.length(), 1);
    assert!(!single.isEmpty());
    assert!(single.isSingleton());

    // Access operations
    assert_eq!(*single.nth(0), 42);

    // Subseq operations
    let full_subseq = single.subseq_copy(0, 1);
    assert_eq!(full_subseq.length(), 1);
    assert_eq!(*full_subseq.nth(0), 42);

    let empty_subseq = single.subseq_copy(1, 1);
    assert_eq!(empty_subseq.length(), 0);

    let zero_length_subseq = single.subseq_copy(0, 0);
    assert_eq!(zero_length_subseq.length(), 0);

    // Set operations
    let mut single_mut = single;
    let result = single_mut.set(0, 99);
    assert!(result.is_ok());
    assert_eq!(*single_mut.nth(0), 99);

    // Out of bounds set should return error
    let result_oob = single_mut.set(1, 100);
    assert!(result_oob.is_err());
}

#[test]
fn test_avltreeseq_zero_length_operations() {
    // Test zero-length subseq operations
    let single: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);

    // Zero-length subseq at start
    let zero_start = single.subseq_copy(0, 0);
    assert_eq!(zero_start.length(), 0);
    assert!(zero_start.isEmpty());

    // Zero-length subseq at end
    let zero_end = single.subseq_copy(1, 0);
    assert_eq!(zero_end.length(), 0);
    assert!(zero_end.isEmpty());

    // Zero-length subseq beyond end should still return empty
    let zero_beyond = single.subseq_copy(10, 0);
    assert_eq!(zero_beyond.length(), 0);
    assert!(zero_beyond.isEmpty());

    // Test with empty tree
    let empty: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    let zero_empty = empty.subseq_copy(0, 0);
    assert_eq!(zero_empty.length(), 0);
    assert!(zero_empty.isEmpty());

    // All zero-length subsequences should be equivalent to empty
    assert_eq!(zero_start.length(), empty.length());
    assert_eq!(zero_end.length(), empty.length());
    assert_eq!(zero_beyond.length(), empty.length());
    assert_eq!(zero_empty.length(), empty.length());
}

#[test]
fn test_avltreeseq_equality_comparison() {
    let tree1: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    let tree2: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(42);
    let tree3: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::singleton(43);

    // Note: AVLTreeS may not implement PartialEq, so we compare properties
    assert_eq!(tree1.length(), tree2.length());
    assert_eq!(*tree1.nth(0), *tree2.nth(0));

    assert_eq!(tree1.length(), tree3.length());
    assert_ne!(*tree1.nth(0), *tree3.nth(0));

    let empty1: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    let empty2: AVLTreeS<N> = <AVLTreeS<N> as AVLTreeSeq<N>>::empty();
    assert_eq!(empty1.length(), empty2.length());
    assert_eq!(empty1.isEmpty(), empty2.isEmpty());
}

#[test]
fn test_new_root() {
    let tree = AVLTreeS::<i32>::new_root();
    assert_eq!(tree.length(), 0);
}

#[test]
fn test_update() {
    let mut tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    tree.update((1, 99));
    assert_eq!(*tree.nth(1), 99);
}

#[test]
fn test_from_vec() {
    let tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_to_arrayseq() {
    let tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    let seq = tree.to_arrayseq();
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_push_back() {
    let mut tree = AVLTreeS::<i32>::new();
    tree.push_back(1);
    tree.push_back(2);
    assert_eq!(tree.length(), 2);
    assert_eq!(*tree.nth(1), 2);
}

#[test]
fn test_contains_value() {
    let tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    assert!(tree.contains_value(&2));
    assert!(!tree.contains_value(&99));
}

#[test]
fn test_insert_value() {
    let mut tree = AVLTreeS::<i32>::new();
    tree.insert_value(5);
    assert!(tree.contains_value(&5));
}

#[test]
fn test_delete_value() {
    let mut tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    let deleted = tree.delete_value(&2);
    assert!(deleted);
    assert_eq!(tree.length(), 2);
}

#[test]
fn test_is_tree_empty() {
    let empty_tree = AVLTreeS::<i32>::new();
    assert!(empty_tree.is_tree_empty());

    let tree = AVLTreeS::<i32>::from_vec(vec![1]);
    assert!(!tree.is_tree_empty());
}

#[test]
fn test_values_in_order() {
    let tree = AVLTreeS::<i32>::from_vec(vec![1, 2, 3]);
    let values = tree.values_in_order();
    assert_eq!(values.len(), 3);
}

#[test]
fn test_avltreeseq_equality_comprehensive() {
    // Test equality of empty trees
    let empty1 = AVLTreeS::<i32>::empty();
    let empty2 = AVLTreeS::<i32>::empty();
    assert_eq!(empty1, empty2);

    // Test equality of singleton trees
    let single1 = AVLTreeS::from_vec(vec![42]);
    let single2 = AVLTreeS::from_vec(vec![42]);
    let single3 = AVLTreeS::from_vec(vec![43]);
    assert_eq!(single1, single2);
    assert_ne!(single1, single3);

    // Test equality of multi-element trees
    let tree1 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree2 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree3 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 6]);
    let tree4 = AVLTreeS::from_vec(vec![1, 2, 3, 4]);

    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3); // Different last element
    assert_ne!(tree1, tree4); // Different length
    assert_ne!(tree1, empty1); // Different from empty

    // Test with different orderings (should be equal if same sequence)
    let tree5 = AVLTreeS::from_vec(vec![5, 4, 3, 2, 1]);
    assert_ne!(tree1, tree5); // Different sequence order
}

#[test]
fn test_avltreeseq_debug_display() {
    // Test Debug formatting
    let tree = AVLTreeS::from_vec(vec![1, 2, 3]);
    let debug_str = format!("{:?}", tree);
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));

    // Test Display formatting
    let display_str = format!("{}", tree);
    assert!(display_str.contains("1"));
    assert!(display_str.contains("2"));
    assert!(display_str.contains("3"));

    // Test empty tree formatting
    let empty = AVLTreeS::<i32>::empty();
    let _empty_debug = format!("{:?}", empty);
    let empty_display = format!("{}", empty);
    assert_eq!(empty_display, "[]");
}

#[test]
fn test_avltreeseq_iterator() {
    // Test iterator on multi-element tree
    let tree = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);
    let collected = tree.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, vec![1, 2, 3, 4, 5]);

    // Test iterator on empty tree
    let empty = AVLTreeS::<i32>::empty();
    let empty_collected = empty.iter().copied().collect::<Vec<i32>>();
    assert_eq!(empty_collected, Vec::<i32>::new());

    // Test iterator on singleton
    let single = AVLTreeS::from_vec(vec![42]);
    let single_collected = single.iter().copied().collect::<Vec<i32>>();
    assert_eq!(single_collected, vec![42]);

    // Test iterator is consumed properly
    let tree2 = AVLTreeS::from_vec(vec![10, 20, 30]);
    let mut iter = tree2.iter();
    assert_eq!(*iter.next().unwrap(), 10);
    assert_eq!(*iter.next().unwrap(), 20);
    assert_eq!(*iter.next().unwrap(), 30);
    assert!(iter.next().is_none());
}

#[test]
fn test_avltreeseq_large_tree() {
    // Create a larger tree that will trigger rotations and rebalancing
    let values = (1..=20).collect::<Vec<i32>>();
    let tree = AVLTreeS::from_vec(values.clone());

    assert_eq!(tree.length(), 20);

    // Verify all elements are accessible
    for (i, &expected) in values.iter().enumerate() {
        assert_eq!(*tree.nth(i), expected);
    }

    // Verify iterator works correctly
    let collected = tree.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, values);
}

#[test]
fn test_avltreeseq_reverse_insertion_order() {
    // Insert in reverse order to test left-heavy rotations
    let values = (1..=10).rev().collect::<Vec<i32>>();
    let tree = AVLTreeS::from_vec(values);

    assert_eq!(tree.length(), 10);

    // Elements should still be in the order they were inserted
    for i in 0..10 {
        assert_eq!(*tree.nth(i), 10 - i as i32);
    }
}

#[test]
fn test_avltreeseq_set_multiple_elements() {
    let mut tree = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);

    // Set multiple elements
    assert!(tree.set(0, 10).is_ok());
    assert!(tree.set(2, 30).is_ok());
    assert!(tree.set(4, 50).is_ok());

    // Verify changes
    assert_eq!(*tree.nth(0), 10);
    assert_eq!(*tree.nth(1), 2);
    assert_eq!(*tree.nth(2), 30);
    assert_eq!(*tree.nth(3), 4);
    assert_eq!(*tree.nth(4), 50);

    // Verify length unchanged
    assert_eq!(tree.length(), 5);
}

#[test]
fn test_avltreeseq_subseq_multiple_elements() {
    let tree = AVLTreeS::from_vec(vec![10, 20, 30, 40, 50, 60, 70]);

    // Extract middle subsequence
    let sub1 = tree.subseq_copy(2, 3);
    assert_eq!(sub1.length(), 3);
    assert_eq!(*sub1.nth(0), 30);
    assert_eq!(*sub1.nth(1), 40);
    assert_eq!(*sub1.nth(2), 50);

    // Extract from start
    let sub2 = tree.subseq_copy(0, 2);
    assert_eq!(sub2.length(), 2);
    assert_eq!(*sub2.nth(0), 10);
    assert_eq!(*sub2.nth(1), 20);

    // Extract to end
    let sub3 = tree.subseq_copy(5, 10);
    assert_eq!(sub3.length(), 2);
    assert_eq!(*sub3.nth(0), 60);
    assert_eq!(*sub3.nth(1), 70);

    // Full tree copy
    let sub4 = tree.subseq_copy(0, 7);
    assert_eq!(sub4.length(), 7);
    assert_eq!(tree, sub4);
}

#[test]
fn test_avltreeseq_push_back_multiple() {
    let mut tree = AVLTreeS::<i32>::new();

    // Push multiple elements
    for i in 1..=10 {
        tree.push_back(i);
        assert_eq!(tree.length(), i as usize);
        assert_eq!(*tree.nth((i - 1) as usize), i);
    }

    // Verify all elements
    for i in 0..10 {
        assert_eq!(*tree.nth(i), (i + 1) as i32);
    }
}

#[test]
fn test_avltreeseq_delete_value_multiple() {
    let mut tree = AVLTreeS::from_vec(vec![10, 20, 30, 40, 50]);

    // Delete from middle
    assert!(tree.delete_value(&30));
    assert_eq!(tree.length(), 4);
    assert!(!tree.contains_value(&30));

    // Delete from start
    assert!(tree.delete_value(&10));
    assert_eq!(tree.length(), 3);
    assert!(!tree.contains_value(&10));

    // Delete from end
    assert!(tree.delete_value(&50));
    assert_eq!(tree.length(), 2);
    assert!(!tree.contains_value(&50));

    // Try to delete non-existent
    assert!(!tree.delete_value(&99));
    assert_eq!(tree.length(), 2);

    // Remaining elements
    assert_eq!(*tree.nth(0), 20);
    assert_eq!(*tree.nth(1), 40);
}

#[test]
fn test_avltreeseq_mixed_operations() {
    let mut tree = AVLTreeS::from_vec(vec![1, 2, 3]);

    // Mix of operations
    tree.push_back(4);
    assert_eq!(tree.length(), 4);

    tree.set(1, 20).unwrap();
    assert_eq!(*tree.nth(1), 20);

    tree.insert_value(5);
    assert_eq!(tree.length(), 5);

    assert!(tree.delete_value(&3));
    assert_eq!(tree.length(), 4);

    // Verify final state
    let final_values = tree.iter().copied().collect::<Vec<i32>>();
    assert_eq!(final_values, vec![1, 20, 4, 5]);
}

#[test]
fn test_avltreeseq_trigger_right_rotation() {
    // Insert in descending order to create left-heavy tree and trigger right rotations
    let mut tree = AVLTreeS::<i32>::new();
    tree.push_back(30);
    tree.push_back(20);
    tree.push_back(10); // This should trigger a right rotation

    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 30);
    assert_eq!(*tree.nth(1), 20);
    assert_eq!(*tree.nth(2), 10);

    // Continue adding to trigger more rotations
    tree.push_back(5);
    tree.push_back(1);
    assert_eq!(tree.length(), 5);
}

#[test]
fn test_avltreeseq_trigger_left_rotation() {
    // Insert in ascending order to create right-heavy tree and trigger left rotations
    let mut tree = AVLTreeS::<i32>::new();
    tree.push_back(10);
    tree.push_back(20);
    tree.push_back(30); // This should trigger a left rotation

    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 10);
    assert_eq!(*tree.nth(1), 20);
    assert_eq!(*tree.nth(2), 30);

    // Continue adding to trigger more rotations
    tree.push_back(40);
    tree.push_back(50);
    assert_eq!(tree.length(), 5);
}

#[test]
fn test_avltreeseq_trigger_left_right_rotation() {
    // Insert in zig-zag pattern to trigger left-right double rotation
    let mut tree = AVLTreeS::<i32>::new();
    tree.push_back(30);
    tree.push_back(10);
    tree.push_back(20); // This should trigger a left-right rotation

    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 30);
    assert_eq!(*tree.nth(1), 10);
    assert_eq!(*tree.nth(2), 20);
}

#[test]
fn test_avltreeseq_trigger_right_left_rotation() {
    // Insert in zig-zag pattern to trigger right-left double rotation
    let mut tree = AVLTreeS::<i32>::new();
    tree.push_back(10);
    tree.push_back(30);
    tree.push_back(20); // This should trigger a right-left rotation

    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 10);
    assert_eq!(*tree.nth(1), 30);
    assert_eq!(*tree.nth(2), 20);
}

#[test]
fn test_avltreeseq_very_large_tree() {
    // Create a very large tree to ensure all rotation paths are exercised
    let mut tree = AVLTreeS::<i32>::new();

    // Insert 100 elements in various patterns
    for i in 0..50 {
        tree.push_back(i);
    }
    assert_eq!(tree.length(), 50);

    // Insert in reverse
    for i in (50..75).rev() {
        tree.push_back(i);
    }
    assert_eq!(tree.length(), 75);

    // Insert in alternating pattern
    for i in 75..100 {
        if i % 2 == 0 {
            tree.push_back(i);
        } else {
            tree.push_back(1000 - i);
        }
    }
    assert_eq!(tree.length(), 100);

    // Verify we can access all elements
    for i in 0..100 {
        let _ = tree.nth(i);
    }
}

#[test]
fn test_avltreeseq_update_chain() {
    let mut tree = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);

    // Chain multiple updates
    tree.update((0, 10)).update((2, 30)).update((4, 50));

    assert_eq!(*tree.nth(0), 10);
    assert_eq!(*tree.nth(2), 30);
    assert_eq!(*tree.nth(4), 50);
}

#[test]
fn test_avltreeseq_default_trait() {
    let tree: AVLTreeS<i32> = Default::default();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_avltreeseq_from_vec_empty() {
    let tree = AVLTreeS::<i32>::from_vec(vec![]);
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_avltreeseq_to_arrayseq_empty() {
    let tree = AVLTreeS::<i32>::empty();
    let seq = tree.to_arrayseq();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_avltreeseq_values_in_order_comprehensive() {
    // Test with various tree sizes
    let tree1 = AVLTreeS::<i32>::from_vec(vec![]);
    assert_eq!(tree1.values_in_order(), vec![]);

    let tree2 = AVLTreeS::from_vec(vec![42]);
    assert_eq!(tree2.values_in_order(), vec![42]);

    let tree3 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(tree3.values_in_order(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_avltreeseq_contains_value_comprehensive() {
    let tree = AVLTreeS::from_vec(vec![5, 3, 7, 1, 9, 4, 6, 8, 2]);

    // Check all present values
    for i in 1..=9 {
        assert!(tree.contains_value(&i));
    }

    // Check absent values
    assert!(!tree.contains_value(&0));
    assert!(!tree.contains_value(&10));
    assert!(!tree.contains_value(&100));
}

#[test]
fn test_avltreeseq_delete_all_elements() {
    let mut tree = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);

    // Delete all elements one by one
    for i in 1..=5 {
        assert!(tree.delete_value(&i));
        assert_eq!(tree.length(), 5 - i as usize);
    }

    assert!(tree.is_tree_empty());
}

#[test]
fn test_avltreeseq_large_subseq_operations() {
    let tree = AVLTreeS::from_vec((1..=50).collect::<Vec<i32>>());

    // Extract large subsequence
    let sub1 = tree.subseq_copy(10, 20);
    assert_eq!(sub1.length(), 20);
    assert_eq!(*sub1.nth(0), 11);
    assert_eq!(*sub1.nth(19), 30);

    // Extract from middle to end
    let sub2 = tree.subseq_copy(25, 100);
    assert_eq!(sub2.length(), 25);
    assert_eq!(*sub2.nth(0), 26);
    assert_eq!(*sub2.nth(24), 50);
}

#[test]
fn test_avltreeseq_set_all_indices() {
    let mut tree = AVLTreeS::from_vec(vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

    // Set every single index
    for i in 0..10 {
        assert!(tree.set(i, (i + 1) * 100).is_ok());
    }

    // Verify all changes
    for i in 0..10 {
        assert_eq!(*tree.nth(i), (i + 1) * 100);
    }
}

#[test]
fn test_avltreeseq_complex_rotation_patterns() {
    // Create a complex insertion pattern that will trigger multiple rotation types
    let mut tree = AVLTreeS::<i32>::new();

    // This pattern should trigger various rotations
    let pattern = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 35, 55, 65, 77, 85];
    for val in pattern {
        tree.push_back(val);
    }

    assert_eq!(tree.length(), 15);

    // Verify tree maintains correct order
    let collected = tree.iter().copied().collect::<Vec<i32>>();
    assert_eq!(
        collected,
        vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 35, 55, 65, 77, 85]
    );
}

#[test]
fn test_avltreeseq_alternating_operations() {
    let mut tree = AVLTreeS::<i32>::new();

    // Alternate between insertions, deletions, and modifications
    tree.push_back(10);
    tree.push_back(20);
    tree.push_back(30);
    assert_eq!(tree.length(), 3);

    tree.delete_value(&20);
    assert_eq!(tree.length(), 2);

    tree.push_back(40);
    tree.push_back(50);
    assert_eq!(tree.length(), 4);

    tree.set(1, 99).unwrap();
    assert_eq!(*tree.nth(1), 99);

    tree.delete_value(&10);
    assert_eq!(tree.length(), 3);
}

#[test]
fn test_avltreeseq_push_back_to_empty() {
    let mut tree = AVLTreeS::<i32>::new();
    assert!(tree.is_tree_empty());

    tree.push_back(42);
    assert!(!tree.is_tree_empty());
    assert_eq!(tree.length(), 1);
    assert_eq!(*tree.nth(0), 42);
}

#[test]
fn test_avltreeseq_multiple_equality_checks() {
    let tree1 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree2 = AVLTreeS::from_vec(vec![1, 2, 3, 4, 5]);

    // Check multiple times to ensure equality is consistent
    for _ in 0..5 {
        assert_eq!(tree1, tree2);
    }

    // Check reflexivity
    assert_eq!(tree1, tree1);

    // Check symmetry
    assert_eq!(tree2, tree1);
}

#[test]
fn test_avltreeseq_insert_at_boundaries() {
    let mut tree = AVLTreeS::from_vec(vec![20, 30, 40]);

    // Insert at beginning
    tree.push_back(10);
    assert_eq!(tree.length(), 4);

    // Insert at end
    tree.push_back(50);
    assert_eq!(tree.length(), 5);

    // Verify order maintained
    let expected = vec![20, 30, 40, 10, 50];
    let collected = tree.iter().copied().collect::<Vec<i32>>();
    assert_eq!(collected, expected);
}

#[test]
fn test_avltreeseq_eq_trait() {
    let tree1 = AVLTreeS::from_vec(vec![1, 2, 3]);
    let tree2 = AVLTreeS::from_vec(vec![1, 2, 3]);
    assert_eq!(tree1, tree1); // Eq reflexive
    assert_eq!(tree1, tree2);
    assert_eq!(tree2, tree1);
}
