// Copyright (c) 2025 Brian G. Milnes
//! Tests for SetMtEph - parallel set operations.

use apas_verus::Chap05::SetMtEph::SetMtEph::*;

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
