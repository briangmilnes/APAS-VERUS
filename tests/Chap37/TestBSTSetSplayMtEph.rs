#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTSetSplayMtEph.

use apas_verus::BSTSetSplayMtEphLit;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::*;
use apas_verus::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstsetsplaymtephlit_macro_functionality() {
    let empty: BSTSetSplayMtEph<i32> = BSTSetSplayMtEphLit![];
    assert_eq!(empty.size(), 0);

    let with_data: BSTSetSplayMtEph<i32> = BSTSetSplayMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert_eq!(with_data.find(&5), Some(5));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&7), Some(7));
    assert_eq!(with_data.find(&10), None);
}

#[test]
fn test_empty() {
    let set = BSTSetSplayMtEph::<i32>::empty();
    assert_eq!(set.size(), 0);
    assert!(set.is_empty());
}

#[test]
fn test_singleton() {
    let set = BSTSetSplayMtEph::singleton(42);
    assert_eq!(set.size(), 1);
    assert!(set.contains(&42));
}

#[test]
fn test_insert_and_contains() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    assert_eq!(set.size(), 3);
    assert!(set.contains(&5));
    assert!(set.contains(&3));
    assert!(!set.contains(&10));
}

#[test]
fn test_delete() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.delete(&3);
    assert_eq!(set.size(), 2);
    assert!(!set.contains(&3));
}

#[test]
fn test_minimum_maximum() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.insert(1);
    set.insert(9);
    assert_eq!(set.minimum(), Some(1));
    assert_eq!(set.maximum(), Some(9));
}

#[test]
fn test_union() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);
    set2.insert(3);
    let union = set1.union(&set2);
    assert_eq!(union.size(), 3);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
}

#[test]
fn test_intersection() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 2);
    assert!(inter.contains(&2));
    assert!(inter.contains(&3));
}

#[test]
fn test_difference() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);
    let diff = set1.difference(&set2);
    assert_eq!(diff.size(), 2);
    assert!(diff.contains(&1));
    assert!(diff.contains(&3));
}

#[test]
fn test_split() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    let (left, found, right) = set.split(&5);
    assert!(found);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 1);
}

#[test]
fn test_filter() {
    let mut set = BSTSetSplayMtEph::empty();
    for i in 1..=10 {
        set.insert(i);
    }
    let evens = set.filter(|x| x % 2 == 0);
    assert_eq!(evens.size(), 5);
    assert!(evens.contains(&2));
    assert!(evens.contains(&4));
}

#[test]
fn test_reduce() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let sum = set.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 6);
}

#[test]
fn test_iter_in_order() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    set.insert(1);
    let seq = set.iter_in_order();
    assert_eq!(seq.length(), 4);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(1), 3);
    assert_eq!(*seq.nth(2), 5);
    assert_eq!(*seq.nth(3), 7);
}

#[test]
fn test_large_set() {
    let mut set = BSTSetSplayMtEph::empty();
    for i in 0..100 {
        set.insert(i);
    }
    assert_eq!(set.size(), 100);
    assert_eq!(set.minimum(), Some(0));
    assert_eq!(set.maximum(), Some(99));
}

#[test]
fn test_delete_multiple() {
    let mut set = BSTSetSplayMtEph::empty();
    for i in 0..10 {
        set.insert(i);
    }
    set.delete(&5);
    set.delete(&3);
    set.delete(&7);
    assert_eq!(set.size(), 7);
    assert!(!set.contains(&5));
}

#[test]
fn test_duplicate_insert() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(5);
    set.insert(5);
    assert_eq!(set.size(), 1);
}

#[test]
fn test_balanced_after_inserts() {
    let mut set = BSTSetSplayMtEph::empty();
    for i in 1..=100 {
        set.insert(i);
    }
    // Splay tree should stay functional
    assert_eq!(set.size(), 100);
    assert!(set.contains(&50));
}

#[test]
fn test_union_empty() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    let set2 = BSTSetSplayMtEph::empty();
    let union = set1.union(&set2);
    assert_eq!(union.size(), 1);
}

#[test]
fn test_intersection_disjoint() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(3);
    set2.insert(4);
    let inter = set1.intersection(&set2);
    assert_eq!(inter.size(), 0);
}

#[test]
fn test_negative_numbers() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(-5);
    set.insert(-3);
    set.insert(-7);
    set.insert(0);
    assert_eq!(set.minimum(), Some(-7));
    assert_eq!(set.maximum(), Some(0));
}

#[test]
fn test_join_pair() {
    let mut left = BSTSetSplayMtEph::empty();
    left.insert(1);
    left.insert(2);
    let mut right = BSTSetSplayMtEph::empty();
    right.insert(5);
    right.insert(6);
    let joined = BSTSetSplayMtEph::join_pair(left, right);
    assert_eq!(joined.size(), 4);
}

#[test]
fn test_join_m() {
    let mut left = BSTSetSplayMtEph::empty();
    left.insert(1);
    left.insert(2);

    let mut right = BSTSetSplayMtEph::empty();
    right.insert(6);
    right.insert(7);

    let joined = BSTSetSplayMtEph::join_m(left, 5, right);
    assert_eq!(joined.size(), 5);
    assert!(joined.contains(&1));
    assert!(joined.contains(&2));
    assert!(joined.contains(&5));
    assert!(joined.contains(&6));
    assert!(joined.contains(&7));
}

#[test]
fn test_join_m_with_empty() {
    let empty = BSTSetSplayMtEph::empty();
    let mut right = BSTSetSplayMtEph::empty();
    right.insert(6);
    right.insert(7);

    let joined = BSTSetSplayMtEph::join_m(empty, 5, right);
    assert_eq!(joined.size(), 3);
    assert!(joined.contains(&5));
    assert!(joined.contains(&6));
    assert!(joined.contains(&7));
}

#[test]
fn test_as_tree() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);

    let tree = set.as_tree();
    assert_eq!(tree.size(), 3);
    assert!(tree.contains(&5));
}

#[test]
fn test_trait_impl_empty() {
    let set: BSTSetSplayMtEph<i32> = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::empty();
    assert_eq!(set.size(), 0);
}

#[test]
fn test_trait_impl_singleton() {
    let set = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::singleton(42);
    assert_eq!(set.size(), 1);
    assert!(set.contains(&42));
}

#[test]
fn test_trait_impl_contains() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(10);
    assert!(<BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::contains(
        &set, &10
    ));
}

#[test]
fn test_trait_impl_find() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(20);
    assert_eq!(
        <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::find(&set, &20),
        Some(20)
    );
}

#[test]
fn test_trait_impl_minimum() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    assert_eq!(
        <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::minimum(&set),
        Some(3)
    );
}

#[test]
fn test_trait_impl_maximum() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);
    set.insert(3);
    set.insert(7);
    assert_eq!(
        <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::maximum(&set),
        Some(7)
    );
}

#[test]
fn test_trait_impl_insert() {
    let mut set = BSTSetSplayMtEph::empty();
    <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::insert(&mut set, 15);
    assert_eq!(set.size(), 1);
}

#[test]
fn test_trait_impl_delete() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(10);
    set.insert(20);
    <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::delete(&mut set, &10);
    assert_eq!(set.size(), 1);
    assert!(!set.contains(&10));
}

#[test]
fn test_trait_impl_union() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::union(&set1, &set2);
    assert_eq!(result.size(), 2);
}

#[test]
fn test_trait_impl_intersection() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);
    set2.insert(3);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::intersection(&set1, &set2);
    assert_eq!(result.size(), 1);
    assert!(result.contains(&2));
}

#[test]
fn test_trait_impl_difference() {
    let mut set1 = BSTSetSplayMtEph::empty();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BSTSetSplayMtEph::empty();
    set2.insert(2);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::difference(&set1, &set2);
    assert_eq!(result.size(), 1);
    assert!(result.contains(&1));
}

#[test]
fn test_trait_impl_split() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(1);
    set.insert(5);
    set.insert(10);

    let (left, found, right) = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::split(&set, &5);
    assert!(found);
    assert_eq!(left.size(), 1);
    assert_eq!(right.size(), 1);
}

#[test]
fn test_trait_impl_join_pair() {
    let mut left = BSTSetSplayMtEph::empty();
    left.insert(1);
    let mut right = BSTSetSplayMtEph::empty();
    right.insert(5);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::join_pair(left, right);
    assert_eq!(result.size(), 2);
}

#[test]
fn test_trait_impl_join_m() {
    let mut left = BSTSetSplayMtEph::empty();
    left.insert(1);
    let mut right = BSTSetSplayMtEph::empty();
    right.insert(5);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::join_m(left, 3, right);
    assert_eq!(result.size(), 3);
    assert!(result.contains(&3));
}

#[test]
fn test_trait_impl_filter() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let result = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::filter(&set, |x| x % 2 == 1);
    assert_eq!(result.size(), 2);
}

#[test]
fn test_trait_impl_reduce() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let sum = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::reduce(&set, |a, b| a + b, 0);
    assert_eq!(sum, 6);
}

#[test]
fn test_trait_impl_iter_in_order() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    let seq = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::iter_in_order(&set);
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_trait_impl_as_tree() {
    let mut set = BSTSetSplayMtEph::empty();
    set.insert(5);

    let tree = <BSTSetSplayMtEph<i32> as BSTSetSplayMtEphTrait<i32>>::as_tree(&set);
    assert_eq!(tree.size(), 1);
}
