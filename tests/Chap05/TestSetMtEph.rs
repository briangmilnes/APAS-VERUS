// Copyright (c) 2025 Brian G. Milnes
//! Tests for SetMtEph - parallel set operations.

use apas_verus::Chap05::SetMtEph::SetMtEph::*;
use apas_verus::Types::Types::Pair;

#[test]
fn test_set_basic() {
    let set: SetMtEph<i32> = SetMtEph::from_vec(vec![1, 2, 3]);
    assert_eq!(set.size(), 3);
    assert!(set.mem(&1));
    assert!(set.mem(&2));
    assert!(set.mem(&3));
    assert!(!set.mem(&4));
}

#[test]
fn test_set_union() {
    let s1: SetMtEph<i32> = SetMtEph::from_vec(vec![1, 2, 3]);
    let s2: SetMtEph<i32> = SetMtEph::from_vec(vec![3, 4, 5]);
    let union = s1.union(&s2);
    assert_eq!(union.size(), 5);
}

#[test]
fn test_cartesian_product() {
    let s1: SetMtEph<i32> = SetMtEph::from_vec(vec![1, 2, 3]);
    let s2: SetMtEph<i32> = SetMtEph::from_vec(vec![10, 20]);
    let prod = s1.cartesian_product(&s2);
    // |s1| * |s2| = 3 * 2 = 6
    assert_eq!(prod.size(), 6);
    assert!(prod.mem(&Pair(1, 10)));
    assert!(prod.mem(&Pair(1, 20)));
    assert!(prod.mem(&Pair(2, 10)));
    assert!(prod.mem(&Pair(2, 20)));
    assert!(prod.mem(&Pair(3, 10)));
    assert!(prod.mem(&Pair(3, 20)));
    assert!(!prod.mem(&Pair(4, 10)));
}

#[test]
fn test_cartesian_product_larger() {
    // Test with more elements to exercise parallelism
    let s1: SetMtEph<i32> = SetMtEph::from_vec((0..20).collect());
    let s2: SetMtEph<i32> = SetMtEph::from_vec((0..10).collect());
    let prod = s1.cartesian_product(&s2);
    assert_eq!(prod.size(), 200);
}

#[test]
fn test_set_empty() {
    let set: SetMtEph<i32> = SetMtEph::empty();
    assert_eq!(set.size(), 0);
    assert!(!set.mem(&0));
}

#[test]
fn test_set_insert() {
    let mut set: SetMtEph<i32> = SetMtEph::empty();
    assert!(set.insert(1));
    assert!(set.insert(2));
    assert!(!set.insert(1)); // Duplicate.
    assert_eq!(set.size(), 2);
}

#[test]
fn test_set_intersection() {
    let s1 = SetMtEph::from_vec(vec![1, 2, 3, 4, 5]);
    let s2 = SetMtEph::from_vec(vec![3, 4, 5, 6, 7]);
    let inter = s1.intersection(&s2);
    assert_eq!(inter.size(), 3);
    assert!(inter.mem(&3));
    assert!(inter.mem(&4));
    assert!(inter.mem(&5));
    assert!(!inter.mem(&1));
    assert!(!inter.mem(&7));
}

#[test]
fn test_set_union_with_empty() {
    let s1 = SetMtEph::from_vec(vec![1, 2, 3]);
    let s2: SetMtEph<i32> = SetMtEph::empty();
    let union = s1.union(&s2);
    assert_eq!(union.size(), 3);
}

#[test]
fn test_set_intersection_disjoint() {
    let s1 = SetMtEph::from_vec(vec![1, 2, 3]);
    let s2 = SetMtEph::from_vec(vec![4, 5, 6]);
    let inter = s1.intersection(&s2);
    assert_eq!(inter.size(), 0);
}

#[test]
fn test_set_from_vec_deduplicates() {
    let set = SetMtEph::from_vec(vec![1, 1, 2, 2, 3, 3]);
    assert_eq!(set.size(), 3);
}

#[test]
fn test_set_iter() {
    let set = SetMtEph::from_vec(vec![10, 20, 30]);
    let mut collected: Vec<i32> = set.iter().cloned().collect();
    collected.sort();
    assert_eq!(collected, vec![10, 20, 30]);
}

#[test]
fn test_set_large_union() {
    let s1 = SetMtEph::from_vec((0..100).collect());
    let s2 = SetMtEph::from_vec((50..150).collect());
    let union = s1.union(&s2);
    assert_eq!(union.size(), 150);
}

#[test]
fn test_set_large_intersection() {
    let s1 = SetMtEph::from_vec((0..100).collect());
    let s2 = SetMtEph::from_vec((50..150).collect());
    let inter = s1.intersection(&s2);
    assert_eq!(inter.size(), 50);
}

#[test]
fn test_cartesian_product_empty() {
    let s1 = SetMtEph::from_vec(vec![1, 2, 3]);
    let s2: SetMtEph<i32> = SetMtEph::empty();
    let prod = s1.cartesian_product(&s2);
    assert_eq!(prod.size(), 0);
}
