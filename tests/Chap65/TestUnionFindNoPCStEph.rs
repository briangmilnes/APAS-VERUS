#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find (no path compression, HashMap-backed, generic) Tests

use apas_verus::Chap65::UnionFindNoPCStEph::UnionFindNoPCStEph::*;

#[test]
fn test_new_empty() {
    let uf = UnionFind::<usize>::new();
    assert_eq!(uf.size(), 0);
}

#[test]
fn test_insert_and_size() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(0);
    uf.insert(1);
    uf.insert(2);
    assert_eq!(uf.size(), 3);
}

#[test]
fn test_find_self() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(42);
    let root = uf.find(&42);
    assert_eq!(root, 42);
}

#[test]
fn test_equals_same_element() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(5);
    assert!(uf.equals(&5, &5));
}

#[test]
fn test_equals_different_sets() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(0);
    uf.insert(1);
    assert!(!uf.equals(&0, &1));
}

#[test]
fn test_union_basic() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(0);
    uf.insert(1);

    assert!(!uf.equals(&0, &1));
    uf.union_sets(&0, &1);
    assert!(uf.equals(&0, &1));
}

#[test]
fn test_union_transitivity() {
    let mut uf = UnionFind::<usize>::new();
    for i in 0..4usize {
        uf.insert(i);
    }

    uf.union_sets(&0, &1);
    uf.union_sets(&2, &3);
    uf.union_sets(&1, &2);

    assert!(uf.equals(&0, &1));
    assert!(uf.equals(&0, &2));
    assert!(uf.equals(&0, &3));
    assert!(uf.equals(&1, &2));
    assert!(uf.equals(&1, &3));
    assert!(uf.equals(&2, &3));
}

#[test]
fn test_union_chain() {
    let mut uf = UnionFind::<usize>::new();
    for i in 0..5usize {
        uf.insert(i);
    }

    uf.union_sets(&0, &1);
    uf.union_sets(&1, &2);
    uf.union_sets(&2, &3);
    uf.union_sets(&3, &4);

    let root = uf.find(&0);
    for i in 1..5usize {
        assert_eq!(uf.find(&i), root);
    }
}

#[test]
fn test_size_unchanged_after_union() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(0);
    uf.insert(1);
    uf.insert(2);
    assert_eq!(uf.size(), 3);

    uf.union_sets(&0, &1);
    assert_eq!(uf.size(), 3); // size tracks elements, not sets

    uf.union_sets(&1, &2);
    assert_eq!(uf.size(), 3);
}

#[test]
fn test_union_idempotent() {
    let mut uf = UnionFind::<usize>::new();
    uf.insert(0);
    uf.insert(1);

    uf.union_sets(&0, &1);
    assert!(uf.equals(&0, &1));
    uf.union_sets(&0, &1);
    assert!(uf.equals(&0, &1));
}

#[test]
fn test_multiple_disjoint_sets() {
    let mut uf = UnionFind::<usize>::new();
    for i in 0..6usize {
        uf.insert(i);
    }

    uf.union_sets(&0, &1);
    uf.union_sets(&2, &3);
    uf.union_sets(&4, &5);

    assert!(uf.equals(&0, &1));
    assert!(uf.equals(&2, &3));
    assert!(uf.equals(&4, &5));

    assert!(!uf.equals(&0, &2));
    assert!(!uf.equals(&0, &4));
    assert!(!uf.equals(&2, &4));
}
