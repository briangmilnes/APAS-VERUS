#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Tests (Sequential)

use std::hash::Hash;

use apas_verus::Chap65::UnionFindStEph::UnionFindStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_union_find_basic() {
    let mut uf = UnionFindStEph::<N>::new();

    // Insert vertices
    uf.insert(0);
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);

    // Initially each in own set
    assert!(!uf.equals(&0, &1));
    assert!(!uf.equals(&0, &2));
    assert!(!uf.equals(&1, &2));
    assert_eq!(uf.num_sets(), 4);

    // Union 0 and 1
    uf.union(&0, &1);
    assert!(uf.equals(&0, &1));
    assert!(!uf.equals(&0, &2));
    assert_eq!(uf.num_sets(), 3);

    // Union 2 and 3
    uf.union(&2, &3);
    assert!(uf.equals(&2, &3));
    assert!(!uf.equals(&0, &2));
    assert_eq!(uf.num_sets(), 2);

    // Union 1 and 2 (connects all)
    uf.union(&1, &2);
    assert!(uf.equals(&0, &1));
    assert!(uf.equals(&0, &2));
    assert!(uf.equals(&0, &3));
    assert!(uf.equals(&1, &2));
    assert!(uf.equals(&1, &3));
    assert!(uf.equals(&2, &3));
    assert_eq!(uf.num_sets(), 1);
}

#[test]
fn test_union_find_path_compression() {
    let mut uf = UnionFindStEph::<N>::new();

    // Create a long chain: 0-1-2-3-4
    for i in 0..5 {
        uf.insert(i);
    }

    uf.union(&0, &1);
    uf.union(&1, &2);
    uf.union(&2, &3);
    uf.union(&3, &4);

    // All should be in same set
    assert!(uf.equals(&0, &4));
    assert_eq!(uf.num_sets(), 1);

    // Path compression should have flattened the tree
    let root = uf.find(&4);
    assert!(uf.equals(&0, &root));
}

#[test]
fn test_union_find_empty() {
    let mut uf = UnionFindStEph::<N>::new();
    assert_eq!(uf.num_sets(), 0);
}

#[test]
fn test_union_find_single() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(42);
    assert_eq!(uf.num_sets(), 1);
    assert!(uf.equals(&42, &42));
}

#[test]
fn test_union_find_duplicate_insert() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(1);
    uf.insert(1);
    uf.insert(2);

    assert_eq!(uf.num_sets(), 2);
    uf.union(&1, &2);
    assert_eq!(uf.num_sets(), 1);
}

#[test]
fn test_union_find_string_vertices() {
    let mut uf = UnionFindStEph::<String>::new();

    uf.insert("A".to_string());
    uf.insert("B".to_string());
    uf.insert("C".to_string());

    assert_eq!(uf.num_sets(), 3);

    uf.union(&"A".to_string(), &"B".to_string());
    assert!(uf.equals(&"A".to_string(), &"B".to_string()));
    assert!(!uf.equals(&"A".to_string(), &"C".to_string()));
    assert_eq!(uf.num_sets(), 2);
}

#[test]
fn test_union_find_already_unioned() {
    let mut uf = UnionFindStEph::<N>::new();

    uf.insert(1);
    uf.insert(2);

    uf.union(&1, &2);
    assert_eq!(uf.num_sets(), 1);

    // Union again (no-op)
    uf.union(&1, &2);
    assert_eq!(uf.num_sets(), 1);
}

#[test]
fn test_find_root() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);

    uf.union(&1, &2);
    let root1 = uf.find(&1);
    let root2 = uf.find(&2);
    assert_eq!(root1, root2);

    let root3 = uf.find(&3);
    assert_ne!(root1, root3);
}

#[test]
fn test_large_union_find() {
    let mut uf = UnionFindStEph::<N>::new();

    for i in 0..100 {
        uf.insert(i);
    }
    assert_eq!(uf.num_sets(), 100);

    // Union evens together
    for i in (0..100).step_by(2) {
        if i > 0 {
            uf.union(&0, &i);
        }
    }

    // Union odds together
    for i in (1..100).step_by(2) {
        if i > 1 {
            uf.union(&1, &i);
        }
    }

    assert_eq!(uf.num_sets(), 2);

    // All evens should be equal
    assert!(uf.equals(&0, &50));
    assert!(uf.equals(&20, &80));

    // All odds should be equal
    assert!(uf.equals(&1, &51));
    assert!(uf.equals(&21, &81));

    // Evens and odds should not be equal
    assert!(!uf.equals(&0, &1));
}

#[test]
fn test_union_find_reflexive() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(5);

    assert!(uf.equals(&5, &5));
}

#[test]
fn test_union_find_symmetric() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(1);
    uf.insert(2);

    uf.union(&1, &2);
    assert!(uf.equals(&1, &2));
    assert!(uf.equals(&2, &1));
}

#[test]
fn test_union_find_transitive() {
    let mut uf = UnionFindStEph::<N>::new();
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);

    uf.union(&1, &2);
    uf.union(&2, &3);

    assert!(uf.equals(&1, &3));
}

#[test]
fn test_multiple_components() {
    let mut uf = UnionFindStEph::<N>::new();

    // Create 3 components
    for i in 0..9 {
        uf.insert(i);
    }

    // Component 1: {0,1,2}
    uf.union(&0, &1);
    uf.union(&1, &2);

    // Component 2: {3,4,5}
    uf.union(&3, &4);
    uf.union(&4, &5);

    // Component 3: {6,7,8}
    uf.union(&6, &7);
    uf.union(&7, &8);

    assert_eq!(uf.num_sets(), 3);

    assert!(uf.equals(&0, &2));
    assert!(uf.equals(&3, &5));
    assert!(uf.equals(&6, &8));

    assert!(!uf.equals(&0, &3));
    assert!(!uf.equals(&3, &6));
    assert!(!uf.equals(&0, &6));
}

#[test]
fn test_star_pattern() {
    let mut uf = UnionFindStEph::<N>::new();

    // Create star with center 0
    for i in 0..10 {
        uf.insert(i);
    }

    for i in 1..10 {
        uf.union(&0, &i);
    }

    assert_eq!(uf.num_sets(), 1);

    for i in 1..10 {
        for j in 1..10 {
            assert!(uf.equals(&i, &j));
        }
    }
}

#[test]
fn test_rank_based_union_smaller_to_larger() {
    let mut uf = UnionFindStEph::<N>::new();

    // Build tree with rank 2
    uf.insert(0);
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);
    uf.union(&0, &1);
    uf.union(&2, &3);
    uf.union(&0, &2); // Creates rank 2 tree

    // Build single node (rank 0)
    uf.insert(4);

    // Union rank-0 tree to rank-2 tree (should attach 4 under larger tree)
    uf.union(&4, &0);

    assert!(uf.equals(&0, &4));
    assert_eq!(uf.num_sets(), 1);
}

#[test]
fn test_default_trait() {
    let mut uf: UnionFindStEph<i32> = Default::default();
    uf.insert(1);
    assert_eq!(uf.num_sets(), 1);
}

// Trait-based tests

#[test]
fn test_trait_new() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    assert_eq!(uf.num_sets(), 0);
}

#[test]
fn test_trait_insert() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::insert(&mut uf, 1);
    <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::insert(&mut uf, 2);
    assert_eq!(uf.num_sets(), 2);
}

#[test]
fn test_trait_find() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.union(&1, &2);

    let root1 = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::find(&mut uf, &1);
    let root2 = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::find(&mut uf, &2);
    assert_eq!(root1, root2);
}

#[test]
fn test_trait_union() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::union(&mut uf, &1, &2);
    assert!(uf.equals(&1, &2));
}

#[test]
fn test_trait_equals() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.union(&1, &2);
    assert!(<UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::equals(
        &mut uf, &1, &2
    ));
}

#[test]
fn test_trait_num_sets() {
    let mut uf = <UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);
    assert_eq!(<UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::num_sets(&mut uf), 3);
    uf.union(&1, &2);
    assert_eq!(<UnionFindStEph<i32> as UnionFindStEphTrait<i32>>::num_sets(&mut uf), 2);
}

fn generic_union_find_ops<V: StT + Hash, UF: UnionFindStEphTrait<V>>(uf: &mut UF, a: V, b: V) {
    uf.insert(a.clone());
    uf.insert(b.clone());
    uf.union(&a, &b);
    assert!(uf.equals(&a, &b));
}

#[test]
fn test_generic_dispatch() {
    let mut uf = UnionFindStEph::<i32>::new();
    generic_union_find_ops(&mut uf, 10, 20);
    assert_eq!(uf.num_sets(), 1);
}
