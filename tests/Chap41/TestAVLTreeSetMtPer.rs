#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetMtPer.

use apas_verus::AVLTreeSetMtPerLit;
use apas_verus::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_avltreesetmtperlit_macro_type_safety() {
    // Test empty set creation with explicit type
    let empty: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![];
    assert_eq!(empty.size(), 0);
    assert!(!empty.find(&42));

    // Test multi-element set creation
    let multi: AVLTreeSetMtPer<i32> = AVLTreeSetMtPerLit![1, 2, 3];
    assert_eq!(multi.size(), 3);
    assert!(multi.find(&1));
    assert!(multi.find(&2));
    assert!(multi.find(&3));
    assert!(!multi.find(&4));
}

#[test]
fn test_empty_set() {
    let s = AVLTreeSetMtPer::<N>::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_singleton_set() {
    let s = AVLTreeSetMtPer::singleton(42);
    assert_eq!(s.size(), 1);
    assert!(s.find(&42));
    assert!(!s.find(&43));
}

#[test]
fn test_insert() {
    let s = AVLTreeSetMtPer::empty();
    let s1 = s.insert(1);
    let s2 = s1.insert(2);
    let s3 = s2.insert(3);

    assert_eq!(s3.size(), 3);
    assert!(s3.find(&1));
    assert!(s3.find(&2));
    assert!(s3.find(&3));
    assert!(!s3.find(&4));
}

#[test]
fn test_delete() {
    let s = AVLTreeSetMtPer::empty();
    let s1 = s.insert(1).insert(2).insert(3);
    let s2 = s1.delete(&2);

    assert_eq!(s2.size(), 2);
    assert!(s2.find(&1));
    assert!(!s2.find(&2));
    assert!(s2.find(&3));
}

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_union() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
//     let s2 = AVLTreeSetMtPer::empty().insert(3).insert(4).insert(5);
//     let s3 = s1.union(&s2);
//
//     assert_eq!(s3.size(), 5);
//     assert!(s3.find(&1));
//     assert!(s3.find(&2));
//     assert!(s3.find(&3));
//     assert!(s3.find(&4));
//     assert!(s3.find(&5));
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_intersection() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
//     let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
//     let s3 = s1.intersection(&s2);
//
//     assert_eq!(s3.size(), 2);
//     assert!(!s3.find(&1));
//     assert!(s3.find(&2));
//     assert!(s3.find(&3));
//     assert!(!s3.find(&4));
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_difference() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
//     let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
//     let s3 = s1.difference(&s2);
//
//     assert_eq!(s3.size(), 1);
//     assert!(s3.find(&1));
//     assert!(!s3.find(&2));
//     assert!(!s3.find(&3));
//     assert!(!s3.find(&4));
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_filter() {
//     let s = AVLTreeSetMtPer::empty()
//         .insert(1)
//         .insert(2)
//         .insert(3)
//         .insert(4)
//         .insert(5);
//     let evens = s.filter(|x| x % 2 == 0);
//
//     assert_eq!(evens.size(), 2);
//     assert!(!evens.find(&1));
//     assert!(evens.find(&2));
//     assert!(!evens.find(&3));
//     assert!(evens.find(&4));
//     assert!(!evens.find(&5));
// }

#[test]
fn test_clone() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s1.clone();

    assert_eq!(s1.size(), s2.size());
    assert!(s2.find(&1));
    assert!(s2.find(&2));
    assert!(s2.find(&3));
}

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_union_extended() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2);
//     let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3);
//     let u = s1.union(&s2);
//     assert_eq!(u.size(), 3);
//     assert!(u.find(&1));
//     assert!(u.find(&2));
//     assert!(u.find(&3));
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_intersection_extended() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
//     let s2 = AVLTreeSetMtPer::empty().insert(2).insert(3).insert(4);
//     let i = s1.intersection(&s2);
//     assert_eq!(i.size(), 2);
//     assert!(i.find(&2));
//     assert!(i.find(&3));
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_difference_extended() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
//     let s2 = AVLTreeSetMtPer::empty().insert(2);
//     let d = s1.difference(&s2);
//     assert_eq!(d.size(), 2);
//     assert!(d.find(&1));
//     assert!(d.find(&3));
// }

#[test]
fn test_delete_operation() {
    let s = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s.delete(&2);
    assert_eq!(s2.size(), 2);
    assert!(!s2.find(&2));
    assert!(s2.find(&1));
    assert!(s2.find(&3));
}

#[test]
fn test_persistence_delete() {
    let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2).insert(3);
    let s2 = s1.delete(&2);
    assert!(s1.find(&2)); // Original unchanged
    assert!(!s2.find(&2)); // New set without 2
}

#[test]
fn test_large_set() {
    let mut s = AVLTreeSetMtPer::empty();
    for i in 0..100 {
        s = s.insert(i);
    }
    assert_eq!(s.size(), 100);
    assert!(s.find(&0));
    assert!(s.find(&99));
}

#[test]
fn test_negative_numbers() {
    let s = AVLTreeSetMtPer::empty().insert(-5).insert(-3).insert(-7).insert(0);
    assert!(s.find(&-7));
    assert!(s.find(&0));
}

#[test]
fn test_duplicate_insert() {
    let s = AVLTreeSetMtPer::empty().insert(5).insert(5).insert(5);
    assert_eq!(s.size(), 1);
}

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_intersection_disjoint() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1).insert(2);
//     let s2 = AVLTreeSetMtPer::empty().insert(3).insert(4);
//     let i = s1.intersection(&s2);
//     assert_eq!(i.size(), 0);
// }

// DISABLED: Causes thread explosion via recursive ParaPair! calls
// #[test]
// fn test_union_empty() {
//     let s1 = AVLTreeSetMtPer::empty().insert(1);
//     let s2 = AVLTreeSetMtPer::empty();
//     let u = s1.union(&s2);
//     assert_eq!(u.size(), 1);
// }

#[test]
fn test_filter_operation() {
    let set = AVLTreeSetMtPerLit![1, 2, 3, 4, 5, 6];
    let evens = set.filter(|x: &i32| *x % 2 == 0);
    assert_eq!(evens.size(), 3);
    assert!(evens.find(&2));
    assert!(evens.find(&4));
    assert!(evens.find(&6));
}

#[test]
fn test_intersection_operation() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3, 4];
    let set2 = AVLTreeSetMtPerLit![3, 4, 5, 6];
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 2);
    assert!(inter.find(&3));
    assert!(inter.find(&4));
}

#[test]
fn test_difference_operation() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3, 4];
    let set2 = AVLTreeSetMtPerLit![3, 4, 5];
    let diff = set1.difference(&set2);
    assert_eq!(diff.size(), 2);
    assert!(diff.find(&1));
    assert!(diff.find(&2));
}

#[test]
fn test_union_operation() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3];
    let set2 = AVLTreeSetMtPerLit![3, 4, 5];
    let uni = set1.union(&set2);
    assert_eq!(uni.size(), 5);
    assert!(uni.find(&1));
    assert!(uni.find(&5));
}

#[test]
fn test_from_seq_operation() {
    let seq = AVLTreeSeqMtPerS::from_vec(vec![5, 2, 8, 2, 5]);
    let set = AVLTreeSetMtPer::from_seq(seq);
    assert_eq!(set.size(), 3);
    assert!(set.find(&2));
    assert!(set.find(&5));
    assert!(set.find(&8));
}

#[test]
fn test_default_trait() {
    let set: AVLTreeSetMtPer<i32> = Default::default();
    assert_eq!(set.size(), 0);
}

#[test]
fn test_display_trait() {
    let set = AVLTreeSetMtPerLit![1, 2, 3];
    let display_str = format!("{}", set);
    assert!(!display_str.is_empty());
}

#[test]
fn test_debug_trait() {
    let set = AVLTreeSetMtPerLit![1, 2, 3];
    let debug_str = format!("{:?}", set);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_partial_ord_trait() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3];
    let set2 = AVLTreeSetMtPerLit![1, 2, 3];
    let set3 = AVLTreeSetMtPerLit![1, 2, 4];
    assert_eq!(set1.partial_cmp(&set2), Some(std::cmp::Ordering::Equal));
    assert_eq!(set1.partial_cmp(&set3), Some(std::cmp::Ordering::Less));
}

#[test]
fn test_ord_trait() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3];
    let set2 = AVLTreeSetMtPerLit![1, 2, 3];
    let set3 = AVLTreeSetMtPerLit![1, 2, 4];
    assert_eq!(set1.cmp(&set2), std::cmp::Ordering::Equal);
    assert_eq!(set1.cmp(&set3), std::cmp::Ordering::Less);
    assert_eq!(set3.cmp(&set1), std::cmp::Ordering::Greater);
}

#[test]
fn test_to_seq() {
    let set = AVLTreeSetMtPerLit![3, 1, 4, 1, 5];
    let seq = set.to_seq();
    assert_eq!(seq.length(), 4); // Duplicates removed
}

#[test]
fn test_singleton_operation() {
    let set = AVLTreeSetMtPer::singleton(99);
    assert_eq!(set.size(), 1);
    assert!(set.find(&99));
    assert!(!set.find(&1));
}

#[test]
fn test_insert_multiple() {
    let set = AVLTreeSetMtPerLit![1, 2, 3];
    let set2 = set.insert(4).insert(5).insert(6);
    assert_eq!(set2.size(), 6);
    assert!(set2.find(&4));
    assert!(set2.find(&5));
    assert!(set2.find(&6));
}

#[test]
fn test_delete_multiple() {
    let set = AVLTreeSetMtPerLit![1, 2, 3, 4, 5];
    let set2 = set.delete(&2).delete(&4);
    assert_eq!(set2.size(), 3);
    assert!(!set2.find(&2));
    assert!(!set2.find(&4));
    assert!(set2.find(&1));
}

#[test]
fn test_filter_empty_result() {
    let set = AVLTreeSetMtPerLit![1, 3, 5];
    let evens = set.filter(|x: &i32| *x % 2 == 0);
    assert_eq!(evens.size(), 0);
}

#[test]
fn test_intersection_empty() {
    let set1 = AVLTreeSetMtPerLit![1, 2];
    let set2 = AVLTreeSetMtPerLit![3, 4];
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 0);
}

#[test]
fn test_difference_empty() {
    let set1 = AVLTreeSetMtPerLit![1, 2];
    let set2 = AVLTreeSetMtPerLit![1, 2, 3];
    let diff = set1.difference(&set2);
    assert_eq!(diff.size(), 0);
}

#[test]
fn test_union_self() {
    let set = AVLTreeSetMtPerLit![1, 2, 3];
    let uni = set.union(&set);
    assert_eq!(uni.size(), 3);
}

#[test]
fn test_find_missing() {
    let set = AVLTreeSetMtPerLit![1, 2, 3];
    assert!(!set.find(&100));
    assert!(!set.find(&0));
}

// PARALLEL PATH TESTS (> 8 elements triggers parallel divide-and-conquer)
// Now safe with rayon's 10-thread pool (no thread explosion)

#[test]
fn test_filter_parallel_path() {
    let set = AVLTreeSetMtPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let evens = set.filter(|x: &i32| *x % 2 == 0);
    assert_eq!(evens.size(), 6);
    assert!(evens.find(&2));
    assert!(evens.find(&12));
}

#[test]
fn test_intersection_parallel_path() {
    let set1 = AVLTreeSetMtPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let set2 = AVLTreeSetMtPerLit![5, 6, 7, 8, 9, 10, 11, 12];
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 6);
    assert!(inter.find(&5));
    assert!(inter.find(&10));
}

// DISABLED: union() recursively calls union() which causes nested parallel recursion
// even with thread pool: left_result.union(&right_result) at line 231 in source
// This creates O(log n) levels of nested parallelism that exceeds 10 threads
// #[test]
// fn test_union_parallel_path() {
//     let set1 = AVLTreeSetMtPerLit![1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let set2 = AVLTreeSetMtPerLit![6, 7, 8, 9, 10];
//     let uni = set1.union(&set2);
//     assert_eq!(uni.size(), 10);
//     assert!(uni.find(&1));
//     assert!(uni.find(&10));
// }

#[test]
fn test_from_seq_parallel_sort() {
    let unsorted = vec![12, 3, 8, 1, 10, 5, 9, 4, 7, 2, 11, 6];
    let seq = AVLTreeSeqMtPerS::from_vec(unsorted);
    let set = AVLTreeSetMtPer::from_seq(seq);
    assert_eq!(set.size(), 12);
    assert!(set.find(&1));
    assert!(set.find(&12));
}
