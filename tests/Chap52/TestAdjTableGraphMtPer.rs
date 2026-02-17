#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AdjTableGraphMtPer with TRUE parallel operations.

use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_verus::Chap52::AdjTableGraphMtPer::AdjTableGraphMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let g = AdjTableGraphMtPer::<N>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g.insert_vertex(1);
    let g2 = g1.insert_vertex(2);
    let g3 = g2.insert_vertex(3);

    assert_eq!(g3.num_vertices(), 3);
    assert_eq!(g3.num_edges(), 0);
}

#[test]
fn test_insert_edge() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g.insert_edge(1, 2);
    let g2 = g1.insert_edge(2, 3);
    let g3 = g2.insert_edge(1, 3);

    assert_eq!(g3.num_vertices(), 3);
    assert_eq!(g3.num_edges(), 3);
    assert!(g3.has_edge(&1, &2));
    assert!(g3.has_edge(&2, &3));
    assert!(g3.has_edge(&1, &3));
    assert!(!g3.has_edge(&3, &1));
}

#[test]
fn test_delete_vertex() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g
        .insert_edge(1, 2)
        .insert_edge(2, 3)
        .insert_edge(1, 3)
        .insert_edge(3, 1);
    let g2 = g1.delete_vertex(&2);

    assert_eq!(g2.num_vertices(), 2);
    assert!(g2.has_edge(&1, &3));
    assert!(g2.has_edge(&3, &1));
    assert!(!g2.has_edge(&1, &2));
    assert!(!g2.has_edge(&2, &3));
}

#[test]
fn test_delete_edge() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g.insert_edge(1, 2).insert_edge(2, 3).insert_edge(1, 3);
    let g2 = g1.delete_edge(&1, &2);

    assert_eq!(g2.num_edges(), 2);
    assert!(!g2.has_edge(&1, &2));
    assert!(g2.has_edge(&2, &3));
    assert!(g2.has_edge(&1, &3));
}

#[test]
fn test_out_neighbors() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g
        .insert_edge(1, 2)
        .insert_edge(1, 3)
        .insert_edge(1, 4)
        .insert_edge(2, 3);

    let neighbors_1 = g1.out_neighbors(&1);
    assert_eq!(neighbors_1.size(), 3);
    assert!(neighbors_1.find(&2));
    assert!(neighbors_1.find(&3));
    assert!(neighbors_1.find(&4));

    let neighbors_2 = g1.out_neighbors(&2);
    assert_eq!(neighbors_2.size(), 1);
    assert!(neighbors_2.find(&3));

    let neighbors_5 = g1.out_neighbors(&5);
    assert_eq!(neighbors_5.size(), 0);
}

#[test]
fn test_out_degree() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g
        .insert_edge(1, 2)
        .insert_edge(1, 3)
        .insert_edge(1, 4)
        .insert_edge(2, 3);

    assert_eq!(g1.out_degree(&1), 3);
    assert_eq!(g1.out_degree(&2), 1);
    assert_eq!(g1.out_degree(&3), 0);
    assert_eq!(g1.out_degree(&5), 0);
}

#[test]
fn test_self_loop() {
    let g = AdjTableGraphMtPer::empty();
    let g1 = g.insert_edge(1, 1);

    assert_eq!(g1.num_vertices(), 1);
    assert_eq!(g1.num_edges(), 1);
    assert!(g1.has_edge(&1, &1));

    let neighbors = g1.out_neighbors(&1);
    assert_eq!(neighbors.size(), 1);
    assert!(neighbors.find(&1));
}

#[test]
fn test_clone() {
    let g1 = AdjTableGraphMtPer::empty().insert_edge(1, 2).insert_edge(2, 3);
    let g2 = g1.clone();

    assert_eq!(g2.num_vertices(), 3);
    assert_eq!(g2.num_edges(), 2);
    assert!(g2.has_edge(&1, &2));
    assert!(g2.has_edge(&2, &3));
}

#[test]
fn test_persistence() {
    let g1 = AdjTableGraphMtPer::empty().insert_edge(1, 2).insert_edge(2, 3);
    let g2 = g1.insert_edge(1, 3);

    // g1 unchanged
    assert_eq!(g1.num_edges(), 2);
    assert!(!g1.has_edge(&1, &3));

    // g2 has new edge
    assert_eq!(g2.num_edges(), 3);
    assert!(g2.has_edge(&1, &3));
}
