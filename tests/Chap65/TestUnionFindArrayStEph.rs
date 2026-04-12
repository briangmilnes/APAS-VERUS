#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Array (no path compression, integer-indexed) Tests

use apas_verus::Chap65::UnionFindArrayStEph::UnionFindArrayStEph::*;

#[test]
fn test_new() {
    let uf = UnionFindArray::new(5);
    assert_eq!(uf.size(), 5);
}

#[test]
fn test_new_zero() {
    let uf = UnionFindArray::new(0);
    assert_eq!(uf.size(), 0);
}

#[test]
fn test_find_self() {
    let mut uf = UnionFindArray::new(5);
    // Each element is its own representative initially.
    for i in 0..5 {
        assert_eq!(uf.find(i), i);
    }
}

#[test]
fn test_union_basic() {
    let mut uf = UnionFindArray::new(4);

    uf.union(0, 1);
    assert_eq!(uf.find(0), uf.find(1));

    uf.union(2, 3);
    assert_eq!(uf.find(2), uf.find(3));

    // 0 and 2 still in different sets.
    assert_ne!(uf.find(0), uf.find(2));
}

#[test]
fn test_union_merge_sets() {
    let mut uf = UnionFindArray::new(4);

    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(1, 2);

    // All four elements in the same set.
    let r0 = uf.find(0);
    assert_eq!(uf.find(1), r0);
    assert_eq!(uf.find(2), r0);
    assert_eq!(uf.find(3), r0);
}

#[test]
fn test_union_chain() {
    let mut uf = UnionFindArray::new(5);

    uf.union(0, 1);
    uf.union(1, 2);
    uf.union(2, 3);
    uf.union(3, 4);

    let r = uf.find(0);
    for i in 1..5 {
        assert_eq!(uf.find(i), r);
    }
}

#[test]
fn test_union_idempotent() {
    let mut uf = UnionFindArray::new(3);

    uf.union(0, 1);
    let r1 = uf.find(0);
    uf.union(0, 1); // same union again
    let r2 = uf.find(0);
    assert_eq!(r1, r2);
}

#[test]
fn test_size() {
    let uf = UnionFindArray::new(10);
    assert_eq!(uf.size(), 10);
}

#[test]
fn test_find_single() {
    let mut uf = UnionFindArray::new(1);
    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.size(), 1);
}

#[test]
fn test_union_by_rank_large() {
    let mut uf = UnionFindArray::new(8);

    // Build two trees and merge them.
    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(4, 5);
    uf.union(6, 7);

    uf.union(0, 2);
    uf.union(4, 6);
    uf.union(0, 4);

    let r = uf.find(0);
    for i in 1..8 {
        assert_eq!(uf.find(i), r);
    }
    assert_eq!(uf.size(), 8);
}
