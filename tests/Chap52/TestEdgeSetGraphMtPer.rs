#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for EdgeSetGraphMtPer

use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_verus::Chap52::EdgeSetGraphMtPer::EdgeSetGraphMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_from_vertices_and_edges() {
    let mut v = AVLTreeSetMtPer::empty();
    v = v.insert(1);
    v = v.insert(2);
    v = v.insert(3);

    let mut e = AVLTreeSetMtPer::empty();
    e = e.insert(Pair(1, 2));
    e = e.insert(Pair(2, 3));

    let g = EdgeSetGraphMtPer::from_vertices_and_edges(v, e);
    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 2);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &3));
    assert!(!g.has_edge(&1, &3));
}

#[test]
fn test_vertices_accessor() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);

    let vertices = g.vertices();
    assert_eq!(vertices.size(), 2);
    assert!(vertices.find(&1));
    assert!(vertices.find(&2));
    assert!(!vertices.find(&3));
}

#[test]
fn test_edges_accessor() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_edge(1, 2);
    let g = g.insert_edge(2, 3);

    let edges = g.edges();
    assert_eq!(edges.size(), 2);
    assert!(edges.find(&Pair(1, 2)));
    assert!(edges.find(&Pair(2, 3)));
    assert!(!edges.find(&Pair(1, 3)));
}

#[test]
fn test_default() {
    let g = EdgeSetGraphMtPer::<i32>::default();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);

    let g = g.insert_vertex(2);
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_insert_edge() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);
    let g = g.insert_edge(1, 2);

    assert_eq!(g.num_edges(), 1);
    assert!(g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &1));
}

#[test]
fn test_delete_vertex() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);
    let g = g.insert_edge(1, 2);

    let g = g.delete_vertex(&1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_delete_edge() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);
    let g = g.insert_edge(1, 2);

    let g = g.delete_edge(&1, &2);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(&1, &2));
}

#[test]
fn test_out_neighbors() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);
    let g = g.insert_vertex(3);
    let g = g.insert_edge(1, 2);
    let g = g.insert_edge(1, 3);

    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 2);
    assert!(neighbors.find(&2));
    assert!(neighbors.find(&3));
}

#[test]
fn test_out_degree() {
    let g = EdgeSetGraphMtPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g = g.insert_vertex(2);
    let g = g.insert_vertex(3);
    let g = g.insert_edge(1, 2);
    let g = g.insert_edge(1, 3);

    assert_eq!(g.out_degree(&1), 2);
    assert_eq!(g.out_degree(&2), 0);
}
