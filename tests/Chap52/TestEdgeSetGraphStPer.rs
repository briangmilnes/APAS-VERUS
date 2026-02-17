//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for EdgeSetGraphStPer

use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap52::EdgeSetGraphStPer::EdgeSetGraphStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let g = EdgeSetGraphStPer::<N>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_vertex(1);
    let g2 = g1.insert_vertex(2);
    assert_eq!(g2.num_vertices(), 2);
    assert_eq!(g2.num_edges(), 0);
}

#[test]
fn test_insert_edge() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_edge(1, 2);
    assert_eq!(g1.num_vertices(), 2);
    assert_eq!(g1.num_edges(), 1);
    assert!(g1.has_edge(&1, &2));
    assert!(!g1.has_edge(&2, &1));
}

#[test]
fn test_delete_edge() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_edge(1, 2).insert_edge(1, 3);
    assert_eq!(g1.num_edges(), 2);
    let g2 = g1.delete_edge(&1, &2);
    assert_eq!(g2.num_edges(), 1);
    assert!(!g2.has_edge(&1, &2));
    assert!(g2.has_edge(&1, &3));
}

#[test]
fn test_out_neighbors() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_edge(1, 2).insert_edge(1, 3).insert_edge(2, 3);
    let neighbors = g1.out_neighbors(&1);
    assert_eq!(neighbors.size(), 2);
    assert!(neighbors.find(&2));
    assert!(neighbors.find(&3));
}

#[test]
fn test_out_degree() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_edge(1, 2).insert_edge(1, 3);
    assert_eq!(g1.out_degree(&1), 2);
    assert_eq!(g1.out_degree(&2), 0);
}

#[test]
fn test_delete_vertex() {
    let g = EdgeSetGraphStPer::<N>::empty();
    let g1 = g.insert_edge(1, 2).insert_edge(2, 3).insert_edge(3, 1);
    assert_eq!(g1.num_vertices(), 3);
    assert_eq!(g1.num_edges(), 3);
    let g2 = g1.delete_vertex(&2);
    assert_eq!(g2.num_vertices(), 2);
    assert_eq!(g2.num_edges(), 1); // Only 3->1 remains
    assert!(!g2.has_edge(&1, &2));
    assert!(!g2.has_edge(&2, &3));
    assert!(g2.has_edge(&3, &1));
}
