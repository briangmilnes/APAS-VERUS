//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for EdgeSetGraphStEph

use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap52::EdgeSetGraphStEph::EdgeSetGraphStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let g = EdgeSetGraphStEph::<i32>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);

    g.insert_vertex(2);
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_insert_edge() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    assert_eq!(g.num_edges(), 1);
    assert!(g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &1));
}

#[test]
fn test_delete_vertex() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    g.delete_vertex(&1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_delete_edge() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    g.delete_edge(&1, &2);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(&1, &2));
}

#[test]
fn test_out_neighbors() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);

    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 2);
    assert!(neighbors.find(&2));
    assert!(neighbors.find(&3));
}

#[test]
fn test_out_degree() {
    let mut g = EdgeSetGraphStEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);

    assert_eq!(g.out_degree(&1), 2);
    assert_eq!(g.out_degree(&2), 0);
}
