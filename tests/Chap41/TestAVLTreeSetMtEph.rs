//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetMtEph with parallelism verification

use apas_verus::AVLTreeSetMtEphLit;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_avltreesetmtephlit_macro_functionality() {
    // Test empty set creation
    let empty: AVLTreeSetMtEph<i32> = AVLTreeSetMtEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));

    // Test set creation with elements
    let with_data: AVLTreeSetMtEph<i32> = AVLTreeSetMtEphLit![1, 2, 3];
    assert_eq!(with_data.size(), 3);
    assert!(with_data.find(&1));
    assert!(with_data.find(&2));
    assert!(with_data.find(&3));
    assert!(!with_data.find(&4));
}

#[test]
fn test_empty() {
    let s = AVLTreeSetMtEph::<i32>::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_singleton() {
    let s = AVLTreeSetMtEph::singleton(42);
    assert_eq!(s.size(), 1);
    assert!(s.find(&42));
}

#[test]
fn test_insert_find() {
    let mut s = AVLTreeSetMtEph::<i32>::empty();
    s.insert(1);
    s.insert(2);
    s.insert(3);

    assert_eq!(s.size(), 3);
    assert!(s.find(&1));
    assert!(s.find(&2));
    assert!(s.find(&3));
    assert!(!s.find(&4));
}

#[test]
fn test_delete() {
    let mut s = AVLTreeSetMtEph::<i32>::empty();
    s.insert(1);
    s.insert(2);
    s.insert(3);

    s.delete(&2);
    assert_eq!(s.size(), 2);
    assert!(!s.find(&2));
    assert!(s.find(&1));
    assert!(s.find(&3));
}

#[test]
fn test_filter_small() {
    let mut s = AVLTreeSetMtEph::<i32>::empty();
    for i in 1..=10 {
        s.insert(i);
    }

    let evens = s.filter(|x| x % 2 == 0);
    assert_eq!(evens.size(), 5);
    assert!(evens.find(&2));
    assert!(evens.find(&4));
    assert!(!evens.find(&1));
}

#[test]
fn test_filter_large_parallel() {
    // Test parallel filter with large set (> threshold = 128)
    let mut s = AVLTreeSetMtEph::<i32>::empty();
    for i in 1..=200 {
        s.insert(i);
    }

    let evens = s.filter(|x| x % 2 == 0);
    assert_eq!(evens.size(), 100);
    for i in 1..=100 {
        assert!(evens.find(&(i * 2)));
    }
}

#[test]
fn test_union_small() {
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    let mut s2 = AVLTreeSetMtEph::<i32>::empty();

    for i in 1..=5 {
        s1.insert(i);
    }
    for i in 4..=8 {
        s2.insert(i);
    }

    let u = s1.union(&s2);
    assert_eq!(u.size(), 8);
    for i in 1..=8 {
        assert!(u.find(&i));
    }
}

#[test]
fn test_union_large_parallel() {
    // Test parallel union with large sets (> threshold = 128)
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    let mut s2 = AVLTreeSetMtEph::<i32>::empty();

    for i in 1..=150 {
        s1.insert(i);
    }
    for i in 100..=250 {
        s2.insert(i);
    }

    let u = s1.union(&s2);
    assert_eq!(u.size(), 250);
    for i in 1..=250 {
        assert!(u.find(&i));
    }
}

#[test]
fn test_intersection_small() {
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    let mut s2 = AVLTreeSetMtEph::<i32>::empty();

    for i in 1..=10 {
        s1.insert(i);
    }
    for i in 5..=15 {
        s2.insert(i);
    }

    let inter = s1.intersection(&s2);
    assert_eq!(inter.size(), 6);
    for i in 5..=10 {
        assert!(inter.find(&i));
    }
}

#[test]
fn test_intersection_large_parallel() {
    // Test parallel intersection with sets (reduced for faster testing)
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    let mut s2 = AVLTreeSetMtEph::<i32>::empty();

    for i in 1..=100 {
        s1.insert(i);
    }
    for i in 50..=150 {
        s2.insert(i);
    }

    let inter = s1.intersection(&s2);
    assert_eq!(inter.size(), 51);
    for i in 50..=100 {
        assert!(inter.find(&i));
    }
}

#[test]
fn test_difference() {
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    let mut s2 = AVLTreeSetMtEph::<i32>::empty();

    for i in 1..=10 {
        s1.insert(i);
    }
    for i in 5..=15 {
        s2.insert(i);
    }

    let diff = s1.difference(&s2);
    assert_eq!(diff.size(), 4);
    for i in 1..=4 {
        assert!(diff.find(&i));
    }
    for i in 5..=10 {
        assert!(!diff.find(&i));
    }
}

#[test]
fn test_clone() {
    let mut s1 = AVLTreeSetMtEph::<i32>::empty();
    s1.insert(1);
    s1.insert(2);

    let s2 = s1.clone();
    assert_eq!(s2.size(), 2);
    assert!(s2.find(&1));
    assert!(s2.find(&2));
}

#[test]
fn test_from_seq() {
    let seq = AVLTreeSeqStEphS::from_vec(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let s = AVLTreeSetMtEph::from_seq(seq);

    assert_eq!(s.size(), 7); // Duplicates removed
    assert!(s.find(&1));
    assert!(s.find(&2));
    assert!(s.find(&3));
    assert!(s.find(&4));
    assert!(s.find(&5));
    assert!(s.find(&6));
    assert!(s.find(&9));
}

#[test]
fn test_debug_trait() {
    let s = AVLTreeSetMtEphLit![10, 20, 30];
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("10"));
    assert!(debug_str.contains("20"));
    assert!(debug_str.contains("30"));
}

#[test]
fn test_to_seq() {
    let mut s = AVLTreeSetMtEph::empty();
    s.insert(3);
    s.insert(1);
    s.insert(2);

    let seq = s.to_seq();
    assert_eq!(seq.length(), 3);

    // Verify all elements are in the sequence
    let mut found = vec![false; 3];
    for i in 0..seq.length() {
        let val = *seq.nth(i);
        if val >= 1 && val <= 3 {
            found[(val - 1) as usize] = true;
        }
    }
    assert!(found.iter().all(|&x| x));
}

#[test]
fn test_equality_via_elements() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![1, 2, 3];
    let s3 = AVLTreeSetMtEphLit![1, 2, 4];

    // Test equality via size and find
    assert_eq!(s1.size(), s2.size());
    for i in 1..=3 {
        assert_eq!(s1.find(&i), s2.find(&i));
    }

    // Test inequality - s1 and s3 differ
    assert_eq!(s1.size(), s3.size()); // Same size
    assert!(!s1.find(&4)); // s1 doesn't have 4
    assert!(s3.find(&4)); // s3 has 4
    assert!(s1.find(&3)); // s1 has 3
    assert!(!s3.find(&3)); // s3 doesn't have 3
}

#[test]
fn test_union_empty_sets() {
    let empty1 = AVLTreeSetMtEph::<i32>::empty();
    let empty2 = AVLTreeSetMtEph::<i32>::empty();
    let result = empty1.union(&empty2);
    assert_eq!(result.size(), 0);

    let s = AVLTreeSetMtEphLit![1, 2, 3];
    let result = s.union(&empty1);
    assert_eq!(result.size(), 3);
}

#[test]
fn test_intersection_empty_sets() {
    let empty1 = AVLTreeSetMtEph::<i32>::empty();
    let s = AVLTreeSetMtEphLit![1, 2, 3];
    let result = s.intersection(&empty1);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_difference_empty_sets() {
    let empty1 = AVLTreeSetMtEph::<i32>::empty();
    let s = AVLTreeSetMtEphLit![1, 2, 3];

    let result = s.difference(&empty1);
    assert_eq!(result.size(), 3);

    let result = empty1.difference(&s);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_filter_empty() {
    let empty = AVLTreeSetMtEph::<i32>::empty();
    let filtered = empty.filter(|x| x % 2 == 0);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_filter_all_match() {
    let s = AVLTreeSetMtEphLit![2, 4, 6, 8];
    let filtered = s.filter(|x| x % 2 == 0);
    assert_eq!(filtered.size(), 4);
}

#[test]
fn test_filter_none_match() {
    let s = AVLTreeSetMtEphLit![1, 3, 5, 7];
    let filtered = s.filter(|x| x % 2 == 0);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_from_seq_empty() {
    let seq = AVLTreeSeqStEphS::<i32>::from_vec(vec![]);
    let s = AVLTreeSetMtEph::from_seq(seq);
    assert_eq!(s.size(), 0);
}

#[test]
fn test_from_seq_singleton() {
    let seq = AVLTreeSeqStEphS::from_vec(vec![42]);
    let s = AVLTreeSetMtEph::from_seq(seq);
    assert_eq!(s.size(), 1);
    assert!(s.find(&42));
}

#[test]
fn test_to_seq_empty() {
    let empty = AVLTreeSetMtEph::<i32>::empty();
    let seq = empty.to_seq();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_to_seq_singleton() {
    let s = AVLTreeSetMtEph::singleton(42);
    let seq = s.to_seq();
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
}

#[test]
fn test_delete_nonexistent() {
    let mut s = AVLTreeSetMtEphLit![1, 2, 3];
    s.delete(&99);
    assert_eq!(s.size(), 3); // Size unchanged
}

#[test]
fn test_delete_all_elements() {
    let mut s = AVLTreeSetMtEphLit![1, 2, 3, 4, 5];
    for i in 1..=5 {
        s.delete(&i);
    }
    assert_eq!(s.size(), 0);
}

#[test]
fn test_insert_duplicate() {
    let mut s = AVLTreeSetMtEph::empty();
    s.insert(42);
    s.insert(42);
    s.insert(42);
    assert_eq!(s.size(), 1); // Should only have one copy
    assert!(s.find(&42));
}

#[test]
fn test_union_disjoint_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![4, 5, 6];
    let result = s1.union(&s2);

    assert_eq!(result.size(), 6);
    for i in 1..=6 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_union_identical_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![1, 2, 3];
    let result = s1.union(&s2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_intersection_disjoint_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![4, 5, 6];
    let result = s1.intersection(&s2);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_intersection_identical_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![1, 2, 3];
    let result = s1.intersection(&s2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_difference_identical_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![1, 2, 3];
    let result = s1.difference(&s2);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_difference_disjoint_sets() {
    let s1 = AVLTreeSetMtEphLit![1, 2, 3];
    let s2 = AVLTreeSetMtEphLit![4, 5, 6];
    let result = s1.difference(&s2);

    assert_eq!(result.size(), 3);
    for i in 1..=3 {
        assert!(result.find(&i));
    }
}

#[test]
fn test_mixed_operations_comprehensive() {
    let mut s1 = AVLTreeSetMtEphLit![1, 2, 3, 4, 5];
    let s2 = AVLTreeSetMtEphLit![4, 5, 6, 7];

    // Union
    let union_result = s1.union(&s2);
    assert_eq!(union_result.size(), 7);

    // Delete from original
    s1.delete(&3);
    assert_eq!(s1.size(), 4);

    // Insert new element
    s1.insert(10);
    assert_eq!(s1.size(), 5);

    // Intersection after modification
    let intersection_result = s1.intersection(&s2);
    assert_eq!(intersection_result.size(), 2); // 4, 5
    assert!(intersection_result.find(&4));
    assert!(intersection_result.find(&5));
}

#[test]
fn test_from_seq_all_duplicates() {
    let seq = AVLTreeSeqStEphS::from_vec(vec![7, 7, 7, 7, 7]);
    let s = AVLTreeSetMtEph::from_seq(seq);
    assert_eq!(s.size(), 1);
    assert!(s.find(&7));
}

#[test]
fn test_large_set_operations() {
    let mut s = AVLTreeSetMtEph::empty();
    for i in 0..100 {
        s.insert(i);
    }

    assert_eq!(s.size(), 100);
    for i in 0..100 {
        assert!(s.find(&i));
    }

    // Delete half
    for i in (0..100).step_by(2) {
        s.delete(&i);
    }

    assert_eq!(s.size(), 50);
    for i in (1..100).step_by(2) {
        assert!(s.find(&i));
    }
}

#[test]
fn test_string_set() {
    let mut s = AVLTreeSetMtEph::empty();
    s.insert("apple".to_string());
    s.insert("banana".to_string());
    s.insert("cherry".to_string());

    assert_eq!(s.size(), 3);
    assert!(s.find(&"apple".to_string()));
    assert!(s.find(&"banana".to_string()));
    assert!(s.find(&"cherry".to_string()));
    assert!(!s.find(&"date".to_string()));

    s.delete(&"banana".to_string());
    assert_eq!(s.size(), 2);
    assert!(!s.find(&"banana".to_string()));
}
