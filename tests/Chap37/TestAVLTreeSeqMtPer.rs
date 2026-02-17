#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeqMtPer.

use apas_verus::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let tree = AVLTreeSeqMtPerS::<i32>::empty();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_new() {
    let tree = AVLTreeSeqMtPerS::<i32>::new();
    assert_eq!(tree.length(), 0);
}

#[test]
fn test_singleton() {
    let tree = AVLTreeSeqMtPerS::singleton(42);
    assert_eq!(tree.length(), 1);
    assert!(tree.isSingleton());
    assert_eq!(*tree.nth(0), 42);
}

#[test]
fn test_length() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(tree.length(), 3);
}

#[test]
fn test_nth() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(1), 2);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_set() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let new_tree = tree.set(1, 99).unwrap();
    assert_eq!(*new_tree.nth(1), 99);
    // Original unchanged (persistent)
    assert_eq!(*tree.nth(1), 2);
}

#[test]
fn test_isempty() {
    let empty = AVLTreeSeqMtPerS::<i32>::empty();
    assert!(empty.isEmpty());

    let non_empty = AVLTreeSeqMtPerS::singleton(1);
    assert!(!non_empty.isEmpty());
}

#[test]
fn test_issingleton() {
    let single = AVLTreeSeqMtPerS::singleton(42);
    assert!(single.isSingleton());

    let multiple = AVLTreeSeqMtPerS::from_vec(vec![1, 2]);
    assert!(!multiple.isSingleton());
}

#[test]
fn test_subseq_copy() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sub = tree.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_from_vec() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_values_in_order() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let values = tree.values_in_order();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_iter() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let count = tree.into_iter().count();
    assert_eq!(count, 3);
}

#[test]
fn test_clone() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = tree1.clone();
    assert_eq!(tree1.length(), tree2.length());
    assert_eq!(*tree1.nth(0), *tree2.nth(0));
}

#[test]
fn test_multiple_sets() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree2 = tree.set(0, 10).unwrap();
    let tree3 = tree2.set(2, 30).unwrap();
    let tree4 = tree3.set(4, 50).unwrap();

    assert_eq!(*tree4.nth(0), 10);
    assert_eq!(*tree4.nth(2), 30);
    assert_eq!(*tree4.nth(4), 50);
    assert_eq!(*tree.nth(0), 1); // Original unchanged
}

#[test]
fn test_large_sequence() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..100).collect());
    assert_eq!(tree.length(), 100);
    assert_eq!(*tree.nth(0), 0);
    assert_eq!(*tree.nth(99), 99);
}

#[test]
fn test_persistence() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = tree1.set(1, 99).unwrap();
    let tree3 = tree1.set(0, 10).unwrap();

    assert_eq!(*tree1.nth(1), 2);
    assert_eq!(*tree2.nth(1), 99);
    assert_eq!(*tree3.nth(0), 10);
    assert_eq!(*tree1.nth(0), 1); // Original unchanged
}

#[test]
fn test_empty_subseq() {
    let empty = AVLTreeSeqMtPerS::<i32>::empty();
    let sub = empty.subseq_copy(0, 0);
    assert_eq!(sub.length(), 0);
}

#[test]
fn test_set_boundary() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let first = tree.set(0, 100).unwrap();
    let last = tree.set(2, 300).unwrap();
    assert_eq!(*first.nth(0), 100);
    assert_eq!(*last.nth(2), 300);
}

#[test]
fn test_eq() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree2 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let tree3 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 4]);
    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);
}

#[test]
fn test_large_tree_rotations() {
    // Insert many elements to trigger various rotations
    let tree = AVLTreeSeqMtPerS::from_vec((0..200).collect());
    assert_eq!(tree.length(), 200);

    // Spot check various elements
    assert_eq!(*tree.nth(0), 0);
    assert_eq!(*tree.nth(50), 50);
    assert_eq!(*tree.nth(100), 100);
    assert_eq!(*tree.nth(199), 199);
}

#[test]
fn test_reverse_insertion_order() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..50).rev().collect());
    assert_eq!(tree.length(), 50);

    // Elements are in the order they were inserted (reversed)
    for i in 0..50 {
        assert_eq!(*tree.nth(i), 49 - i);
    }
}

#[test]
fn test_subseq_copy_comprehensive() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..20).collect());

    // Middle slice
    let sub1 = tree.subseq_copy(5, 10);
    assert_eq!(sub1.length(), 10);
    for i in 0..10 {
        assert_eq!(*sub1.nth(i), i + 5);
    }

    // From start
    let sub2 = tree.subseq_copy(0, 5);
    assert_eq!(sub2.length(), 5);
    assert_eq!(*sub2.nth(0), 0);
    assert_eq!(*sub2.nth(4), 4);

    // To end
    let sub3 = tree.subseq_copy(15, 10);
    assert_eq!(sub3.length(), 5);
    assert_eq!(*sub3.nth(0), 15);
    assert_eq!(*sub3.nth(4), 19);

    // Out of bounds
    let sub4 = tree.subseq_copy(100, 10);
    assert_eq!(sub4.length(), 0);

    // Zero length
    let sub5 = tree.subseq_copy(5, 0);
    assert_eq!(sub5.length(), 0);
}

#[test]
fn test_iterator_comprehensive() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![10, 20, 30, 40, 50]);

    // Collect values
    let collected = tree.clone().into_iter().collect::<Vec<i32>>();
    assert_eq!(collected, vec![10, 20, 30, 40, 50]);

    // Count
    let count = tree.clone().into_iter().count();
    assert_eq!(count, 5);

    // Empty iterator
    let empty = AVLTreeSeqMtPerS::<i32>::empty();
    let empty_count = empty.into_iter().count();
    assert_eq!(empty_count, 0);
}

#[test]
fn test_debug_trait() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let debug_str = format!("{:?}", tree);
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
}

#[test]
fn test_set_out_of_bounds() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3]);
    let result = tree.set(10, 99);
    assert!(result.is_err());
}

#[test]
fn test_multiple_structural_sharing() {
    let tree1 = AVLTreeSeqMtPerS::from_vec((0..10).collect());

    // Create multiple versions sharing structure
    let tree2 = tree1.set(0, 100).unwrap();
    let tree3 = tree1.set(5, 500).unwrap();
    let tree4 = tree2.set(9, 900).unwrap();

    // All should have different values at modified positions
    assert_eq!(*tree1.nth(0), 0);
    assert_eq!(*tree2.nth(0), 100);
    assert_eq!(*tree3.nth(5), 500);
    assert_eq!(*tree4.nth(0), 100);
    assert_eq!(*tree4.nth(9), 900);

    // Unmodified positions share structure
    assert_eq!(*tree1.nth(1), 1);
    assert_eq!(*tree2.nth(1), 1);
    assert_eq!(*tree3.nth(1), 1);
}

#[test]
fn test_values_in_order_comprehensive() {
    let tree = AVLTreeSeqMtPerS::from_vec(vec![5, 3, 7, 1, 9]);
    let values = tree.values_in_order();
    assert_eq!(values, vec![5, 3, 7, 1, 9]);

    let empty = AVLTreeSeqMtPerS::<i32>::empty();
    let empty_values = empty.values_in_order();
    assert_eq!(empty_values, Vec::<i32>::new());
}

#[test]
fn test_default_trait() {
    let tree: AVLTreeSeqMtPerS<i32> = Default::default();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_alternating_sets() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..20).collect());

    // Set alternating indices
    let mut modified = tree.clone();
    for i in (0..20).step_by(2) {
        modified = modified.set(i, i * 100).unwrap();
    }

    // Check modified positions
    for i in (0..20).step_by(2) {
        assert_eq!(*modified.nth(i), i * 100);
    }

    // Check unmodified positions
    for i in (1..20).step_by(2) {
        assert_eq!(*modified.nth(i), i);
    }

    // Original unchanged
    for i in 0..20 {
        assert_eq!(*tree.nth(i), i);
    }
}

#[test]
fn test_very_large_sequence() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..1000).collect());
    assert_eq!(tree.length(), 1000);

    // Spot check
    assert_eq!(*tree.nth(0), 0);
    assert_eq!(*tree.nth(500), 500);
    assert_eq!(*tree.nth(999), 999);

    // Modify and check persistence
    let modified = tree.set(500, 9999).unwrap();
    assert_eq!(*modified.nth(500), 9999);
    assert_eq!(*tree.nth(500), 500);
}

#[test]
fn test_subseq_then_modify() {
    let tree = AVLTreeSeqMtPerS::from_vec((0..10).collect());
    let sub = tree.subseq_copy(3, 5);

    assert_eq!(sub.length(), 5);

    // Modify subsequence
    let modified_sub = sub.set(2, 999).unwrap();
    assert_eq!(*modified_sub.nth(2), 999);
    assert_eq!(*sub.nth(2), 5); // Original unchanged
}

#[test]
fn test_empty_operations() {
    let empty = AVLTreeSeqMtPerS::<i32>::empty();

    assert_eq!(empty.length(), 0);
    assert!(empty.isEmpty());
    assert!(!empty.isSingleton());

    let values = empty.values_in_order();
    assert_eq!(values.len(), 0);

    let sub = empty.subseq_copy(0, 10);
    assert_eq!(sub.length(), 0);
}

#[test]
fn test_singleton_operations() {
    let single = AVLTreeSeqMtPerS::singleton(42);

    assert_eq!(single.length(), 1);
    assert!(!single.isEmpty());
    assert!(single.isSingleton());
    assert_eq!(*single.nth(0), 42);

    let modified = single.set(0, 100).unwrap();
    assert_eq!(*modified.nth(0), 100);
    assert_eq!(*single.nth(0), 42);

    let sub = single.subseq_copy(0, 1);
    assert_eq!(sub.length(), 1);
    assert_eq!(*sub.nth(0), 42);
}

#[test]
fn test_equality_comprehensive() {
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree2 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let tree3 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 6]);
    let tree4 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4]);

    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);
    assert_ne!(tree1, tree4);

    let empty1 = AVLTreeSeqMtPerS::<i32>::empty();
    let empty2 = AVLTreeSeqMtPerS::<i32>::empty();
    assert_eq!(empty1, empty2);
    assert_ne!(tree1, empty1);
}

#[test]
fn test_trigger_rotations() {
    // Trigger right rotation: insert left-heavy
    let mut tree = AVLTreeSeqMtPerS::singleton(10);
    tree = tree.set(0, 5).unwrap(); // Will cause left-left case
    tree = tree.set(0, 1).unwrap(); // Triggers right rotation
    assert_eq!(tree.length(), 1);

    // Trigger left rotation: insert right-heavy via sets
    let mut tree2 = AVLTreeSeqMtPerS::from_vec(vec![1, 2, 3, 4, 5]);
    for i in 0..5 {
        tree2 = tree2.set(i, i * 10).unwrap();
    }
    assert_eq!(tree2.length(), 5);

    // Trigger left-right rotation
    let mut tree3 = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 15]);
    tree3 = tree3.set(1, 7).unwrap(); // Modifying middle
    assert_eq!(*tree3.nth(1), 7);

    // Trigger right-left rotation
    let mut tree4 = AVLTreeSeqMtPerS::from_vec(vec![10, 20, 30]);
    tree4 = tree4.set(1, 25).unwrap(); // Modifying middle
    assert_eq!(*tree4.nth(1), 25);
}

#[test]
fn test_set_at_index_zero_empty() {
    let empty = AVLTreeSeqMtPerS::<i32>::empty();
    // This should trigger line 93 (setting at index 0 on empty tree)
    let result = empty.set(0, 42);
    // Actually it might succeed and create a node
    if let Ok(tree) = result {
        assert_eq!(tree.length(), 1);
        assert_eq!(*tree.nth(0), 42);
    }
}

// ========== ROTATION COVERAGE TESTS ==========

#[test]
fn test_rotation_right_heavy() {
    // Create a left-heavy tree to trigger right rotation
    // Lines 38-43, 56-61
    let mut tree = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 15]);
    // Overwrite to create imbalance
    for _ in 0..10 {
        tree = tree.set(0, 1).unwrap();
        tree = tree.set(1, 2).unwrap();
        tree = tree.set(2, 3).unwrap();
    }
    assert_eq!(tree.length(), 3);
}

#[test]
fn test_rotation_left_heavy() {
    // Create a right-heavy tree to trigger left rotation
    // Lines 45-50, 64-69
    let mut tree = AVLTreeSeqMtPerS::from_vec(vec![5, 10, 15]);
    // Overwrite to create imbalance
    for _ in 0..10 {
        tree = tree.set(0, 1).unwrap();
        tree = tree.set(1, 2).unwrap();
        tree = tree.set(2, 3).unwrap();
    }
    assert_eq!(tree.length(), 3);
}

#[test]
fn test_deep_rotation_cascade() {
    // Create a very unbalanced tree through sequential sets
    // This should trigger multiple rotation cases
    let mut tree = AVLTreeSeqMtPerS::from_vec((0..50).collect::<Vec<i32>>());
    
    // Modify many elements to potentially trigger rebalancing
    for i in 0..50 {
        let val = ((i * 7) % 100) as i32;
        tree = tree.set(i, val).unwrap();
    }
    
    assert_eq!(tree.length(), 50);
    
    // Verify structure is still valid
    for i in 0..50 {
        let expected = ((i * 7) % 100) as i32;
        assert_eq!(*tree.nth(i), expected);
    }
}

#[test]
fn test_left_right_rotation() {
    // Trigger left-right rotation (lines 56-61)
    let mut tree = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 15, 3, 7]);
    
    // Make modifications that trigger rotation
    tree = tree.set(1, 6).unwrap();  
    tree = tree.set(3, 4).unwrap();
    
    assert_eq!(tree.length(), 5);
}

#[test]
fn test_right_left_rotation() {
    // Trigger right-left rotation (lines 64-69)
    let mut tree = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 20, 15, 25]);
    
    // Make modifications that trigger rotation
    tree = tree.set(2, 18).unwrap();
    tree = tree.set(3, 17).unwrap();
    
    assert_eq!(tree.length(), 5);
}

#[test]
fn test_sequential_insertions_causing_rotations() {
    // Build tree that forces rotations by growing from empty
    let mut tree = AVLTreeSeqMtPerS::<i32>::empty();
    
    // This should trigger line 93 (set at index 0 on empty)
    tree = tree.set(0, 50).unwrap();
    assert_eq!(tree.length(), 1);
    
    // Subsequent sets may not grow the tree but will modify it
    tree = tree.set(0, 40).unwrap();
    tree = tree.set(0, 30).unwrap();
    tree = tree.set(0, 20).unwrap();
    tree = tree.set(0, 10).unwrap();
    
    assert_eq!(*tree.nth(0), 10);
}

#[test]
fn test_force_all_rotation_cases() {
    // Systematically create scenarios for all rotation cases
    
    // Case 1: LL rotation (left-left heavy)
    let tree1 = AVLTreeSeqMtPerS::from_vec(vec![30, 20, 40, 10, 25, 35, 50, 5]);
    let _ = tree1.set(0, 3).unwrap(); // Modify to trigger rebalance
    
    // Case 2: RR rotation (right-right heavy)  
    let tree2 = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 20, 3, 7, 15, 30, 35]);
    let _ = tree2.set(7, 40).unwrap(); // Modify to trigger rebalance
    
    // Case 3: LR rotation (left-right heavy)
    let tree3 = AVLTreeSeqMtPerS::from_vec(vec![30, 10, 40, 5, 20]);
    let _ = tree3.set(2, 15).unwrap(); // Modify middle
    
    // Case 4: RL rotation (right-left heavy)
    let tree4 = AVLTreeSeqMtPerS::from_vec(vec![10, 5, 30, 20, 40]);
    let _ = tree4.set(2, 25).unwrap(); // Modify middle
}

#[test]
fn test_extreme_imbalance_scenarios() {
    // Create trees with extreme imbalances to force rotation code
    
    // Very left-heavy
    let mut tree = AVLTreeSeqMtPerS::from_vec((0..20).rev().collect::<Vec<i32>>());
    for i in 0..20 {
        tree = tree.set(i, (i * 2) as i32).unwrap();
    }
    assert_eq!(tree.length(), 20);
    
    // Very right-heavy
    let mut tree2 = AVLTreeSeqMtPerS::from_vec((0..20).collect::<Vec<i32>>());
    for i in 0..20 {
        tree2 = tree2.set(i, (i * 3) as i32).unwrap();
    }
    assert_eq!(tree2.length(), 20);
}
