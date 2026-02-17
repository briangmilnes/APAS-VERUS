//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AdjTableGraphStEph.

use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let g = AdjTableGraphStEph::<N>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);

    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_edge() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 2);
    g.insert_edge(2, 3);
    g.insert_edge(1, 3);

    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 3);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &3));
    assert!(g.has_edge(&1, &3));
    assert!(!g.has_edge(&3, &1));
}

#[test]
fn test_delete_vertex() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 2);
    g.insert_edge(2, 3);
    g.insert_edge(1, 3);
    g.insert_edge(3, 1);

    g.delete_vertex(&2);

    assert_eq!(g.num_vertices(), 2);
    assert!(g.has_edge(&1, &3));
    assert!(g.has_edge(&3, &1));
    assert!(!g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &3));
}

#[test]
fn test_delete_edge() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 2);
    g.insert_edge(2, 3);
    g.insert_edge(1, 3);

    g.delete_edge(&1, &2);

    assert_eq!(g.num_edges(), 2);
    assert!(!g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &3));
    assert!(g.has_edge(&1, &3));
}

#[test]
fn test_out_neighbors() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);
    g.insert_edge(1, 4);
    g.insert_edge(2, 3);

    let neighbors_1 = g.out_neighbors(&1);
    assert_eq!(neighbors_1.size(), 3);
    assert!(neighbors_1.find(&2));
    assert!(neighbors_1.find(&3));
    assert!(neighbors_1.find(&4));

    let neighbors_2 = g.out_neighbors(&2);
    assert_eq!(neighbors_2.size(), 1);
    assert!(neighbors_2.find(&3));

    let neighbors_5 = g.out_neighbors(&5);
    assert_eq!(neighbors_5.size(), 0);
}

#[test]
fn test_out_degree() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);
    g.insert_edge(1, 4);
    g.insert_edge(2, 3);

    assert_eq!(g.out_degree(&1), 3);
    assert_eq!(g.out_degree(&2), 1);
    assert_eq!(g.out_degree(&3), 0);
    assert_eq!(g.out_degree(&5), 0);
}

#[test]
fn test_vertices() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);

    let verts = g.vertices();
    assert_eq!(verts.size(), 3);
    assert!(verts.find(&1));
    assert!(verts.find(&2));
    assert!(verts.find(&3));
}

#[test]
fn test_self_loop() {
    let mut g = AdjTableGraphStEph::empty();
    g.insert_edge(1, 1);

    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 1);
    assert!(g.has_edge(&1, &1));

    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 1);
    assert!(neighbors.find(&1));
}

#[test]
fn test_clone() {
    let mut g1 = AdjTableGraphStEph::empty();
    g1.insert_edge(1, 2);
    g1.insert_edge(2, 3);

    let g2 = g1.clone();

    assert_eq!(g2.num_vertices(), 3);
    assert_eq!(g2.num_edges(), 2);
    assert!(g2.has_edge(&1, &2));
    assert!(g2.has_edge(&2, &3));
}
