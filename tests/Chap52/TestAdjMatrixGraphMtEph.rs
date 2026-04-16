// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Chapter 52: Adjacency Matrix Graph (ephemeral, multi-threaded).

use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap52::AdjMatrixGraphMtEph::AdjMatrixGraphMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new_empty_graph() {
    let g = AdjMatrixGraphMtEph::new(5);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_num_vertices() {
    let g1 = AdjMatrixGraphMtEph::new(0);
    assert_eq!(g1.num_vertices(), 0);

    let g2 = AdjMatrixGraphMtEph::new(10);
    assert_eq!(g2.num_vertices(), 10);
}

#[test]
fn test_has_edge_empty_graph() {
    let g = AdjMatrixGraphMtEph::new(5);
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(4, 3));
    assert!(!g.has_edge(2, 2));
}

#[test]
fn test_has_edge_out_of_bounds() {
    let g = AdjMatrixGraphMtEph::new(5);
    assert!(!g.has_edge(5, 0));
    assert!(!g.has_edge(0, 5));
    assert!(!g.has_edge(10, 10));
}

#[test]
fn test_out_neighbors_empty_graph() {
    let g = AdjMatrixGraphMtEph::new(5);
    let neighbors = g.out_neighbors(0);
    assert_eq!(neighbors.length(), 0);
}

#[test]
fn test_out_degree_empty_graph() {
    let g = AdjMatrixGraphMtEph::new(5);
    assert_eq!(g.out_degree(0), 0);
    assert_eq!(g.out_degree(4), 0);
}

#[test]
fn test_set_edge() {
    let mut g = AdjMatrixGraphMtEph::new(5);
    assert!(!g.has_edge(0, 1));
    g.set_edge(0, 1, true);
    assert!(g.has_edge(0, 1));
    assert!(!g.has_edge(1, 0)); // directed: only 0->1
    assert_eq!(g.num_edges(), 1);

    g.set_edge(0, 1, false);
    assert!(!g.has_edge(0, 1));
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_set_edge_multiple() {
    let mut g = AdjMatrixGraphMtEph::new(4);
    g.set_edge(0, 1, true);
    g.set_edge(1, 2, true);
    g.set_edge(2, 3, true);
    assert_eq!(g.num_edges(), 3);
    assert!(g.has_edge(0, 1));
    assert!(g.has_edge(1, 2));
    assert!(g.has_edge(2, 3));
    assert!(!g.has_edge(0, 2));
}

#[test]
fn test_complement_empty_graph() {
    let g = AdjMatrixGraphMtEph::new(3);
    let gc = g.complement();

    assert_eq!(gc.num_vertices(), 3);
    assert_eq!(gc.num_edges(), 6); // 3*2 = 6 edges (no self-loops)

    assert!(gc.has_edge(0, 1));
    assert!(gc.has_edge(0, 2));
    assert!(gc.has_edge(1, 0));
    assert!(gc.has_edge(1, 2));
    assert!(gc.has_edge(2, 0));
    assert!(gc.has_edge(2, 1));

    assert!(!gc.has_edge(0, 0));
    assert!(!gc.has_edge(1, 1));
    assert!(!gc.has_edge(2, 2));
}

#[test]
fn test_complement_single_vertex() {
    let g = AdjMatrixGraphMtEph::new(1);
    let gc = g.complement();

    assert_eq!(gc.num_vertices(), 1);
    assert_eq!(gc.num_edges(), 0);
    assert!(!gc.has_edge(0, 0));
}

#[test]
fn test_complement_zero_vertices() {
    let g = AdjMatrixGraphMtEph::new(0);
    let gc = g.complement();

    assert_eq!(gc.num_vertices(), 0);
    assert_eq!(gc.num_edges(), 0);
}

#[test]
fn test_complement_idempotence() {
    let g = AdjMatrixGraphMtEph::new(3);
    let gc = g.complement();
    let gcc = gc.complement();

    assert_eq!(g.num_edges(), gcc.num_edges());
    assert_eq!(g.has_edge(0, 1), gcc.has_edge(0, 1));
    assert_eq!(g.has_edge(1, 0), gcc.has_edge(1, 0));
}

#[test]
fn test_multiple_vertices_all_operations() {
    let g = AdjMatrixGraphMtEph::new(10);

    assert_eq!(g.num_vertices(), 10);
    assert_eq!(g.num_edges(), 0);

    let gc = g.complement();
    assert_eq!(gc.num_edges(), 90); // 10*9 = 90 edges
}
