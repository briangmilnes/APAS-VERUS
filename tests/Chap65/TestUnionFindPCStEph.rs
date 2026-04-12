#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find with Path Compression Tests (Sequential)

use apas_verus::Chap65::UnionFindPCStEph::UnionFindPCStEph::*;

#[test]
fn test_union_find_basic() {
    let mut uf = UnionFindPC::<usize>::new();

    uf.insert(0);
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);

    assert!(!uf.equals(&0, &1));
    assert!(!uf.equals(&0, &2));
    assert!(!uf.equals(&1, &2));
    assert_eq!(uf.size(), 4);

    uf.union(&0, &1);
    assert!(uf.equals(&0, &1));
    assert!(!uf.equals(&0, &2));
    assert_eq!(uf.size(), 4);

    uf.union(&2, &3);
    assert!(uf.equals(&2, &3));
    assert!(!uf.equals(&0, &2));
    assert_eq!(uf.size(), 4);

    uf.union(&1, &2);
    assert!(uf.equals(&0, &1));
    assert!(uf.equals(&0, &2));
    assert!(uf.equals(&0, &3));
    assert!(uf.equals(&1, &2));
    assert!(uf.equals(&1, &3));
    assert!(uf.equals(&2, &3));
    assert_eq!(uf.size(), 4);
}

#[test]
fn test_union_find_path_compression() {
    let mut uf = UnionFindPC::<usize>::new();

    for i in 0..5 {
        uf.insert(i);
    }

    uf.union(&0, &1);
    uf.union(&1, &2);
    uf.union(&2, &3);
    uf.union(&3, &4);

    assert!(uf.equals(&0, &4));
    assert_eq!(uf.size(), 5);

    let root = uf.find(&4);
    assert!(uf.equals(&0, &root));
}

#[test]
fn test_union_find_empty() {
    let uf = UnionFindPC::<usize>::new();
    assert_eq!(uf.size(), 0);
}

#[test]
fn test_union_find_single() {
    let mut uf = UnionFindPC::<usize>::new();
    uf.insert(42);
    assert_eq!(uf.size(), 1);
    assert!(uf.equals(&42, &42));
}

#[test]
fn test_union_find_string_vertices() {
    let mut uf = UnionFindPC::<String>::new();

    uf.insert("A".to_string());
    uf.insert("B".to_string());
    uf.insert("C".to_string());

    assert_eq!(uf.size(), 3);

    uf.union(&"A".to_string(), &"B".to_string());
    assert!(uf.equals(&"A".to_string(), &"B".to_string()));
    assert!(!uf.equals(&"A".to_string(), &"C".to_string()));
    assert_eq!(uf.size(), 3);
}

#[test]
fn test_union_find_already_unioned() {
    let mut uf = UnionFindPC::<usize>::new();

    uf.insert(1);
    uf.insert(2);

    uf.union(&1, &2);
    assert_eq!(uf.size(), 2);

    // Union again (no-op).
    uf.union(&1, &2);
    assert_eq!(uf.size(), 2);
}

#[test]
fn test_find_root() {
    let mut uf = UnionFindPC::<usize>::new();
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
    let mut uf = UnionFindPC::<usize>::new();

    for i in 0..100 {
        uf.insert(i);
    }
    assert_eq!(uf.size(), 100);

    for i in (2..100).step_by(2) {
        uf.union(&0, &i);
    }
    for i in (3..100).step_by(2) {
        uf.union(&1, &i);
    }

    assert!(uf.equals(&0, &50));
    assert!(uf.equals(&20, &80));
    assert!(uf.equals(&1, &51));
    assert!(uf.equals(&21, &81));
    assert!(!uf.equals(&0, &1));
}

#[test]
fn test_union_find_reflexive() {
    let mut uf = UnionFindPC::<usize>::new();
    uf.insert(5);
    assert!(uf.equals(&5, &5));
}

#[test]
fn test_union_find_symmetric() {
    let mut uf = UnionFindPC::<usize>::new();
    uf.insert(1);
    uf.insert(2);

    uf.union(&1, &2);
    assert!(uf.equals(&1, &2));
    assert!(uf.equals(&2, &1));
}

#[test]
fn test_union_find_transitive() {
    let mut uf = UnionFindPC::<usize>::new();
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);

    uf.union(&1, &2);
    uf.union(&2, &3);

    assert!(uf.equals(&1, &3));
}

#[test]
fn test_multiple_components() {
    let mut uf = UnionFindPC::<usize>::new();

    for i in 0..9 {
        uf.insert(i);
    }

    uf.union(&0, &1); uf.union(&1, &2);
    uf.union(&3, &4); uf.union(&4, &5);
    uf.union(&6, &7); uf.union(&7, &8);

    assert!(uf.equals(&0, &2));
    assert!(uf.equals(&3, &5));
    assert!(uf.equals(&6, &8));

    assert!(!uf.equals(&0, &3));
    assert!(!uf.equals(&3, &6));
    assert!(!uf.equals(&0, &6));
}

#[test]
fn test_star_pattern() {
    let mut uf = UnionFindPC::<usize>::new();

    for i in 0..10 {
        uf.insert(i);
    }

    for i in 1..10 {
        uf.union(&0, &i);
    }

    for i in 1..10 {
        for j in 1..10 {
            assert!(uf.equals(&i, &j));
        }
    }
}

#[test]
fn test_rank_based_union_smaller_to_larger() {
    let mut uf = UnionFindPC::<usize>::new();

    uf.insert(0); uf.insert(1); uf.insert(2); uf.insert(3);
    uf.union(&0, &1);
    uf.union(&2, &3);
    uf.union(&0, &2);

    uf.insert(4);
    uf.union(&4, &0);

    assert!(uf.equals(&0, &4));
}

#[test]
fn test_default_trait() {
    let mut uf = UnionFindPC::<i32>::new();
    uf.insert(1);
    assert_eq!(uf.size(), 1);
}

// Trait-based tests.

#[test]
fn test_trait_new() {
    let uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    assert_eq!(uf.size(), 0);
}

#[test]
fn test_trait_insert() {
    let mut uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::insert(&mut uf, 1);
    <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::insert(&mut uf, 2);
    assert_eq!(uf.size(), 2);
}

#[test]
fn test_trait_find() {
    let mut uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.union(&1, &2);

    let root1 = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::find(&mut uf, &1);
    let root2 = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::find(&mut uf, &2);
    assert_eq!(root1, root2);
}

#[test]
fn test_trait_union() {
    let mut uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::union(&mut uf, &1, &2);
    assert!(uf.equals(&1, &2));
}

#[test]
fn test_trait_equals() {
    let mut uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.union(&1, &2);
    assert!(<UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::equals(
        &mut uf, &1, &2
    ));
}

#[test]
fn test_trait_size() {
    let mut uf = <UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::new();
    uf.insert(1);
    uf.insert(2);
    uf.insert(3);
    assert_eq!(<UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::size(&uf), 3);
    uf.union(&1, &2);
    assert_eq!(<UnionFindPC<i32> as UnionFindPCStEphTrait<i32>>::size(&uf), 3);
}
