//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 52: Adjacency Sequence Graph (ephemeral, multi-threaded).

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap52::AdjSeqGraphMtEph::AdjSeqGraphMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new_empty_graph() {
    let g = AdjSeqGraphMtEph::new(5);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_num_vertices() {
    let g1 = AdjSeqGraphMtEph::new(0);
    assert_eq!(g1.num_vertices(), 0);

    let g2 = AdjSeqGraphMtEph::new(10);
    assert_eq!(g2.num_vertices(), 10);
}

#[test]
fn test_has_edge_empty_graph() {
    let g = AdjSeqGraphMtEph::new(5);
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(4, 3));
    assert!(!g.has_edge(2, 2));
}

#[test]
fn test_has_edge_in_bounds_no_edges() {
    let g = AdjSeqGraphMtEph::new(5);
    assert!(!g.has_edge(0, 4));
    assert!(!g.has_edge(4, 0));
    assert!(!g.has_edge(2, 3));
}

#[test]
fn test_out_neighbors_empty_graph() {
    let g = AdjSeqGraphMtEph::new(5);
    let neighbors = g.out_neighbors(0);
    assert_eq!(neighbors.length(), 0);
}

#[test]
fn test_out_degree_empty_graph() {
    let g = AdjSeqGraphMtEph::new(5);
    assert_eq!(g.out_degree(0), 0);
    assert_eq!(g.out_degree(4), 0);
}

#[test]
fn test_zero_vertices() {
    let g = AdjSeqGraphMtEph::new(0);

    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_single_vertex() {
    let g = AdjSeqGraphMtEph::new(1);

    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(0, 0));
}

#[test]
fn test_set_edge() {
    let mut g = AdjSeqGraphMtEph::new(5);
    assert!(!g.has_edge(0, 1));
    g.set_edge(0, 1, true);
    assert!(g.has_edge(0, 1));
    assert!(!g.has_edge(1, 0));
    assert_eq!(g.num_edges(), 1);
    assert_eq!(g.out_degree(0), 1);

    g.set_edge(0, 1, false);
    assert!(!g.has_edge(0, 1));
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_multiple_vertices_all_operations() {
    let g = AdjSeqGraphMtEph::new(10);

    assert_eq!(g.num_vertices(), 10);
    assert_eq!(g.num_edges(), 0);

    for i in 0..10 {
        assert_eq!(g.out_degree(i), 0);
        assert_eq!(g.out_neighbors(i).length(), 0);
        assert!(!g.has_edge(i, (i + 1) % 10));
    }
}

#[test]
fn test_out_neighbors_boundary() {
    let g = AdjSeqGraphMtEph::new(3);

    for i in 0..3 {
        let neighbors = g.out_neighbors(i);
        assert_eq!(neighbors.length(), 0);
    }
}

#[test]
fn test_has_edge_linear_search() {
    let g = AdjSeqGraphMtEph::new(5);

    for u in 0..5 {
        for v in 0..5 {
            assert!(!g.has_edge(u, v));
        }
    }
}
