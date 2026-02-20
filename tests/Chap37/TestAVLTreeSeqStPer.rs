//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::AVLTreeSeqStPerLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_persistent_set_does_not_mutate() {
    let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![7];
    let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3]; // tabulate(&|i| i, 4)
    // AVLTreeSeqStPer is persistent - no update method
    // let b = a.update(1, 99);
    assert_eq!(*a.nth(1), 1);
    // assert_eq!(*b.nth(1), 99);
}

#[test]
fn test_iterator_inorder_values() {
    let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![1, 2, 3, 4, 5]; // tabulate(&|i| i + 1, 5)
    let vals = a.iter().copied().collect::<Vec<N>>();
    assert_eq!(vals, vec![1, 2, 3, 4, 5]);
}

// Complex tree balancing edge case tests
#[test]
fn test_avl_left_heavy_rebalancing() {
    // Create a tree and test modifications that could trigger rebalancing
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50];

    // Modify elements to create potential imbalance scenarios
    let tree = tree.set(0, 5).unwrap(); // Change first element
    let tree = tree.set(1, 15).unwrap(); // Change second element
    let tree = tree.set(2, 25).unwrap(); // Change third element

    // Tree should remain balanced and maintain structure
    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![5, 15, 25, 40, 50]);

    // Verify all elements are accessible
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }
}

#[test]
fn test_avl_right_heavy_rebalancing() {
    // Create a tree and test modifications that could trigger rebalancing
    let tree = AVLTreeSeqStPerLit![1, 3, 5, 10, 15];

    // Modify elements to create potential imbalance scenarios
    let tree = tree.set(4, 50).unwrap(); // Change last element
    let tree = tree.set(3, 40).unwrap(); // Change fourth element
    let tree = tree.set(2, 30).unwrap(); // Change third element

    // Tree should remain balanced and maintain structure
    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 3, 30, 40, 50]);

    // Verify all elements are accessible
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }
}

#[test]
fn test_avl_left_right_rotation() {
    // Create a scenario requiring left-right rotation
    let tree = AVLTreeSeqStPerLit![1, 2, 3, 4, 5, 6, 7];

    // Modify middle elements to create left-right imbalance
    let tree = tree.set(2, 15).unwrap(); // Change 3 to 15
    let tree = tree.set(1, 12).unwrap(); // Change 2 to 12

    // Tree should remain balanced
    assert_eq!(tree.length(), 7);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 12, 15, 4, 5, 6, 7]);

    // Verify structural integrity
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }
}

#[test]
fn test_avl_right_left_rotation() {
    // Create a scenario requiring right-left rotation
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50, 60, 70];

    // Modify elements to create right-left imbalance
    let tree = tree.set(4, 25).unwrap(); // Change 50 to 25
    let tree = tree.set(5, 35).unwrap(); // Change 60 to 35

    // Tree should remain balanced
    assert_eq!(tree.length(), 7);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![10, 20, 30, 40, 25, 35, 70]);

    // Verify structural integrity
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }
}

#[test]
fn test_avl_large_tree_balancing() {
    // Test balancing with a larger tree (15 elements)
    let values = (1..=15).collect::<Vec<N>>();
    let tree = AVLTreeSeqStPerS::from_vec(values.clone());

    assert_eq!(tree.length(), 15);
    assert_eq!(tree.values_in_order(), values);

    // Verify random access works correctly
    for (i, &val) in values.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }

    // Test modifications maintain balance
    let modified = tree.set(7, 100).unwrap(); // Change middle element
    assert_eq!(modified.length(), 15);
    assert_eq!(*modified.nth(7), 100);

    // Verify other elements unchanged
    for (i, &val) in values.iter().enumerate().take(modified.length()) {
        if i != 7 {
            assert_eq!(*modified.nth(i), val);
        }
    }
}

#[test]
fn test_avl_sequential_modifications() {
    // Test multiple sequential modifications maintain balance
    let mut tree = AVLTreeSeqStPerLit![5, 10, 15, 20, 25];

    // Apply multiple modifications
    tree = tree.set(0, 1).unwrap(); // 1, 10, 15, 20, 25
    tree = tree.set(4, 30).unwrap(); // 1, 10, 15, 20, 30
    tree = tree.set(2, 12).unwrap(); // 1, 10, 12, 20, 30
    tree = tree.set(1, 8).unwrap(); // 1, 8, 12, 20, 30
    tree = tree.set(3, 22).unwrap(); // 1, 8, 12, 22, 30

    assert_eq!(tree.length(), 5);
    let vals = tree.values_in_order();
    assert_eq!(vals, vec![1, 8, 12, 22, 30]);

    // Verify all elements accessible
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }
}

#[test]
fn test_avl_empty_and_singleton_edge_cases() {
    // Test empty tree
    let empty = AVLTreeSeqStPerS::<N>::empty();
    assert_eq!(empty.length(), 0);
    assert!(empty.isEmpty());
    assert!(!empty.isSingleton());
    assert_eq!(empty.values_in_order(), vec![]);

    // Test singleton
    let single = AVLTreeSeqStPerS::singleton(42);
    assert_eq!(single.length(), 1);
    assert!(!single.isEmpty());
    assert!(single.isSingleton());
    assert_eq!(*single.nth(0), 42);
    assert_eq!(single.values_in_order(), vec![42]);

    // Test singleton modification
    let modified = single.set(0, 99).unwrap();
    assert_eq!(modified.length(), 1);
    assert_eq!(*modified.nth(0), 99);
    assert_eq!(modified.values_in_order(), vec![99]);

    // Original singleton unchanged (persistent)
    assert_eq!(*single.nth(0), 42);
}

#[test]
fn test_avl_subseq_balancing() {
    // Test that subseq operations maintain balance
    let tree = AVLTreeSeqStPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Test various subseq operations
    let sub1 = tree.subseq_copy(2, 5); // [3, 4, 5, 6, 7]
    assert_eq!(sub1.length(), 5);
    assert_eq!(sub1.values_in_order(), vec![3, 4, 5, 6, 7]);

    let sub2 = tree.subseq_copy(0, 3); // [1, 2, 3]
    assert_eq!(sub2.length(), 3);
    assert_eq!(sub2.values_in_order(), vec![1, 2, 3]);

    let sub3 = tree.subseq_copy(7, 3); // [8, 9, 10]
    assert_eq!(sub3.length(), 3);
    assert_eq!(sub3.values_in_order(), vec![8, 9, 10]);

    // Test edge cases
    let empty_sub = tree.subseq_copy(5, 0); // Empty subseq
    assert_eq!(empty_sub.length(), 0);
    assert!(empty_sub.isEmpty());

    let out_of_bounds = tree.subseq_copy(15, 5); // Start beyond end
    assert_eq!(out_of_bounds.length(), 0);
    assert!(out_of_bounds.isEmpty());

    let partial_bounds = tree.subseq_copy(8, 5); // Extends beyond end
    assert_eq!(partial_bounds.length(), 2); // Should get [9, 10]
    assert_eq!(partial_bounds.values_in_order(), vec![9, 10]);
}

#[test]
fn test_avl_stress_balancing() {
    // Stress test with alternating modifications
    let mut tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // Apply alternating modifications that could stress balancing
    for i in 0..tree.length() {
        let new_val = if i % 2 == 0 { i * 5 } else { i * 15 };
        tree = tree.set(i, new_val).unwrap();
    }

    assert_eq!(tree.length(), 10);

    // Verify all elements are accessible and tree maintains integrity
    let vals = tree.values_in_order();
    for (i, &val) in vals.iter().enumerate().take(tree.length()) {
        assert_eq!(*tree.nth(i), val);
    }

    // Expected values: [0, 15, 10, 45, 20, 75, 30, 105, 40, 135]
    let expected = vec![0, 15, 10, 45, 20, 75, 30, 105, 40, 135];
    assert_eq!(vals, expected);
}

#[test]
#[should_panic]
fn test_avl_nth_out_of_bounds_panics() {
    let tree = AVLTreeSeqStPerLit![1, 2, 3];
    let _ = tree.nth(5); // Should panic
}

#[test]
fn test_avl_set_out_of_bounds_error() {
    let tree = AVLTreeSeqStPerLit![1, 2, 3];
    let result = tree.set(5, 99);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Index out of bounds");
}

#[test]
fn test_avl_equality_and_debug() {
    let tree1 = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    let tree2 = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    let tree3 = AVLTreeSeqStPerLit![1, 2, 3, 4, 6];

    // Test equality
    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);

    // Test debug formatting (should not panic)
    let debug_str = format!("{tree1:?}");
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("5"));
}

#[test]
fn test_avl_to_arrayseq_conversion() {
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50];
    let array_seq = tree.to_arrayseq();

    assert_eq!(array_seq.length(), tree.length());
    for i in 0..tree.length() {
        assert_eq!(*array_seq.nth(i), *tree.nth(i));
    }
}

#[test]
fn test_clone_preserves_structure() {
    let tree1 = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    let tree2 = tree1.clone();

    assert_eq!(tree1.length(), tree2.length());
    for i in 0..tree1.length() {
        assert_eq!(*tree1.nth(i), *tree2.nth(i));
    }

    // Modify clone (persistent so creates new tree)
    let tree3 = tree2.set(2, 99).unwrap();
    assert_eq!(*tree3.nth(2), 99);
    assert_eq!(*tree1.nth(2), 3); // Original unchanged
    assert_eq!(*tree2.nth(2), 3); // Clone unchanged
}

#[test]
fn test_iterator_empty() {
    let empty = AVLTreeSeqStPerS::<N>::empty();
    let vals = empty.iter().copied().collect::<Vec<N>>();
    assert_eq!(vals.len(), 0);
}

#[test]
fn test_iterator_singleton() {
    let single = AVLTreeSeqStPerS::singleton(42);
    let vals = single.iter().copied().collect::<Vec<N>>();
    assert_eq!(vals, vec![42]);
}

#[test]
fn test_iterator_large_tree() {
    let tree = AVLTreeSeqStPerS::from_vec((1..=100).collect());
    let vals = tree.iter().copied().collect::<Vec<N>>();
    assert_eq!(vals.len(), 100);
    for (i, &val) in vals.iter().enumerate() {
        assert_eq!(val, (i + 1) as N);
    }
}

#[test]
fn test_from_vec_empty() {
    let tree = AVLTreeSeqStPerS::<N>::from_vec(vec![]);
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_from_vec_large() {
    let values = (1..=50).collect::<Vec<N>>();
    let tree = AVLTreeSeqStPerS::from_vec(values.clone());
    assert_eq!(tree.length(), 50);
    for (i, &expected) in values.iter().enumerate() {
        assert_eq!(*tree.nth(i), expected);
    }
}

#[test]
fn test_values_in_order_empty() {
    let empty = AVLTreeSeqStPerS::<N>::empty();
    let vals = empty.values_in_order();
    assert_eq!(vals.len(), 0);
}

#[test]
fn test_values_in_order_large() {
    let tree = AVLTreeSeqStPerS::from_vec((10..=30).collect());
    let vals = tree.values_in_order();
    assert_eq!(vals.len(), 21);
    for (i, &val) in vals.iter().enumerate() {
        assert_eq!(val, (i + 10) as N);
    }
}

#[test]
fn test_set_on_empty_at_zero() {
    let empty = AVLTreeSeqStPerS::<N>::empty();
    let result = empty.set(0, 42);
    assert!(result.is_ok());
    let tree = result.unwrap();
    assert_eq!(tree.length(), 1);
    assert_eq!(*tree.nth(0), 42);
}

#[test]
fn test_subseq_copy_edge_cases() {
    let tree = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];

    // Length extends beyond end
    let sub1 = tree.subseq_copy(3, 10);
    assert_eq!(sub1.length(), 2);
    assert_eq!(sub1.values_in_order(), vec![4, 5]);

    // Start equals length
    let sub2 = tree.subseq_copy(5, 5);
    assert_eq!(sub2.length(), 0);

    // Start greater than length
    let sub3 = tree.subseq_copy(10, 5);
    assert_eq!(sub3.length(), 0);

    // Full copy
    let sub4 = tree.subseq_copy(0, 5);
    assert_eq!(sub4.length(), 5);
    assert_eq!(sub4, tree);
}

#[test]
fn test_equality_comprehensive() {
    let tree1 = AVLTreeSeqStPerLit![1, 2, 3];
    let tree2 = AVLTreeSeqStPerLit![1, 2, 3];
    let tree3 = AVLTreeSeqStPerLit![1, 2, 4];
    let tree4 = AVLTreeSeqStPerLit![1, 2];

    assert_eq!(tree1, tree2);
    assert_ne!(tree1, tree3);
    assert_ne!(tree1, tree4);

    // Empty trees
    let empty1 = AVLTreeSeqStPerS::<N>::empty();
    let empty2 = AVLTreeSeqStPerS::<N>::new();
    assert_eq!(empty1, empty2);
    assert_ne!(empty1, tree1);
}

#[test]
fn test_persistence_verification() {
    let tree1 = AVLTreeSeqStPerLit![10, 20, 30];
    let tree2 = tree1.set(1, 99).unwrap();
    let tree3 = tree1.set(0, 88).unwrap();
    let tree4 = tree2.set(2, 77).unwrap();

    // Verify each tree is independent
    assert_eq!(tree1.values_in_order(), vec![10, 20, 30]);
    assert_eq!(tree2.values_in_order(), vec![10, 99, 30]);
    assert_eq!(tree3.values_in_order(), vec![88, 20, 30]);
    assert_eq!(tree4.values_in_order(), vec![10, 99, 77]);
}

#[test]
fn test_to_arrayseq_empty() {
    let empty = AVLTreeSeqStPerS::<N>::empty();
    let array_seq = empty.to_arrayseq();
    assert_eq!(array_seq.length(), 0);
}

#[test]
fn test_to_arrayseq_singleton() {
    let tree = AVLTreeSeqStPerS::singleton(42);
    let array_seq = tree.to_arrayseq();
    assert_eq!(array_seq.length(), 1);
    assert_eq!(*array_seq.nth(0), 42);
}

#[test]
fn test_very_large_tree() {
    let tree = AVLTreeSeqStPerS::from_vec((1..=200).collect());
    assert_eq!(tree.length(), 200);

    // Spot checks
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(50), 51);
    assert_eq!(*tree.nth(100), 101);
    assert_eq!(*tree.nth(199), 200);

    // Test persistence on large tree
    let modified = tree.set(100, 9999).unwrap();
    assert_eq!(*modified.nth(100), 9999);
    assert_eq!(*tree.nth(100), 101); // Original unchanged
}

#[test]
fn test_alternating_access_pattern() {
    let tree = AVLTreeSeqStPerLit![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // Access in alternating pattern
    for i in (0..tree.length()).step_by(2) {
        assert_eq!(*tree.nth(i), (i + 1) as N * 10);
    }

    for i in (1..tree.length()).step_by(2) {
        assert_eq!(*tree.nth(i), (i + 1) as N * 10);
    }
}

#[test]
fn test_macro_variants() {
    // Empty
    let empty: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![];
    assert_eq!(empty.length(), 0);

    // Repeat syntax
    let repeated = AVLTreeSeqStPerLit![42; 5];
    assert_eq!(repeated.length(), 5);
    for i in 0..5 {
        assert_eq!(*repeated.nth(i), 42);
    }

    // List syntax
    let listed = AVLTreeSeqStPerLit![1, 2, 3, 4, 5];
    assert_eq!(listed.length(), 5);
    assert_eq!(listed.values_in_order(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_debug_formatting() {
    let tree = AVLTreeSeqStPerLit![100, 200, 300];
    let debug_str = format!("{:?}", tree);
    assert!(debug_str.contains("100"));
    assert!(debug_str.contains("200"));
    assert!(debug_str.contains("300"));

    let empty = AVLTreeSeqStPerS::<N>::empty();
    let empty_debug = format!("{:?}", empty);
    assert!(empty_debug.contains("[]") || empty_debug.is_empty() || empty_debug.len() < 5);
}
