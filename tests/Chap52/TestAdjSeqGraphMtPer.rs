#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap52::AdjSeqGraphMtPer::AdjSeqGraphMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new_empty_graph() {
    let g = AdjSeqGraphMtPer::new(5);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_num_vertices() {
    let g1 = AdjSeqGraphMtPer::new(0);
    assert_eq!(g1.num_vertices(), 0);

    let g2 = AdjSeqGraphMtPer::new(10);
    assert_eq!(g2.num_vertices(), 10);
}

#[test]
fn test_has_edge_empty_graph() {
    let g = AdjSeqGraphMtPer::new(5);
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(4, 3));
    assert!(!g.has_edge(2, 2));
}

#[test]
fn test_has_edge_out_of_bounds() {
    let g = AdjSeqGraphMtPer::new(5);
    assert!(!g.has_edge(5, 0)); // u out of bounds
    assert!(!g.has_edge(0, 5)); // v could be out of bounds but won't crash
    assert!(!g.has_edge(10, 10)); // both out of bounds
}

#[test]
fn test_out_neighbors_empty_graph() {
    let g = AdjSeqGraphMtPer::new(5);
    let neighbors = g.out_neighbors(0);
    assert_eq!(neighbors.length(), 0);
}

#[test]
fn test_out_degree_empty_graph() {
    let g = AdjSeqGraphMtPer::new(5);
    assert_eq!(g.out_degree(0), 0);
    assert_eq!(g.out_degree(4), 0);
}

#[test]
fn test_clone() {
    let g1 = AdjSeqGraphMtPer::new(5);
    let g2 = g1.clone();

    assert_eq!(g1.num_vertices(), g2.num_vertices());
    assert_eq!(g1.num_edges(), g2.num_edges());
}

#[test]
fn test_map_vertices_empty_graph() {
    let g = AdjSeqGraphMtPer::new(3);
    let g2 = g.map_vertices(|v| v * 2);

    assert_eq!(g2.num_vertices(), 3);
    assert_eq!(g2.num_edges(), 0);
}

#[test]
fn test_map_vertices_identity() {
    let g = AdjSeqGraphMtPer::new(5);
    let g2 = g.map_vertices(|v| v);

    assert_eq!(g.num_vertices(), g2.num_vertices());
    assert_eq!(g.num_edges(), g2.num_edges());
}

#[test]
fn test_map_vertices_transformation() {
    let g = AdjSeqGraphMtPer::new(4);
    let g2 = g.map_vertices(|v| v + 10);

    assert_eq!(g2.num_vertices(), 4);
    // All neighbor IDs should be incremented by 10
}

#[test]
fn test_zero_vertices() {
    let g = AdjSeqGraphMtPer::new(0);

    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_single_vertex() {
    let g = AdjSeqGraphMtPer::new(1);

    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(0, 0));
}

#[test]
fn test_multiple_vertices_all_operations() {
    let g = AdjSeqGraphMtPer::new(10);

    assert_eq!(g.num_vertices(), 10);
    assert_eq!(g.num_edges(), 0);

    for i in 0..10 {
        assert_eq!(g.out_degree(i), 0);
        assert_eq!(g.out_neighbors(i).length(), 0);
        assert!(!g.has_edge(i, (i + 1) % 10));
    }
}

#[test]
fn test_map_vertices_larger_graph() {
    let g = AdjSeqGraphMtPer::new(20);
    let g2 = g.map_vertices(|v| v * 3);

    assert_eq!(g2.num_vertices(), 20);
    assert_eq!(g2.num_edges(), 0);
}

#[test]
fn test_map_vertices_with_modulo() {
    let g = AdjSeqGraphMtPer::new(5);
    let g2 = g.map_vertices(|v| v % 3);

    assert_eq!(g2.num_vertices(), 5);
}

#[test]
fn test_out_neighbors_boundary() {
    let g = AdjSeqGraphMtPer::new(3);

    // Test all vertices
    for i in 0..3 {
        let neighbors = g.out_neighbors(i);
        assert_eq!(neighbors.length(), 0);
    }
}

#[test]
fn test_has_edge_linear_search() {
    // This tests the linear search in has_edge
    let g = AdjSeqGraphMtPer::new(5);

    // Even though graph is empty, the function should iterate through neighbor list
    for u in 0..5 {
        for v in 0..5 {
            assert!(!g.has_edge(u, v));
        }
    }
}
