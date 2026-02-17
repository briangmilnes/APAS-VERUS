//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).

use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap52::AdjMatrixGraphMtPer::AdjMatrixGraphMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new_empty_graph() {
    let g = AdjMatrixGraphMtPer::new(5);
    assert_eq!(g.num_vertices(), 5);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_num_vertices() {
    let g1 = AdjMatrixGraphMtPer::new(0);
    assert_eq!(g1.num_vertices(), 0);

    let g2 = AdjMatrixGraphMtPer::new(10);
    assert_eq!(g2.num_vertices(), 10);
}

#[test]
fn test_has_edge_empty_graph() {
    let g = AdjMatrixGraphMtPer::new(5);
    assert!(!g.has_edge(0, 1));
    assert!(!g.has_edge(4, 3));
    assert!(!g.has_edge(2, 2));
}

#[test]
fn test_has_edge_out_of_bounds() {
    let g = AdjMatrixGraphMtPer::new(5);
    assert!(!g.has_edge(5, 0)); // u out of bounds
    assert!(!g.has_edge(0, 5)); // v out of bounds
    assert!(!g.has_edge(10, 10)); // both out of bounds
}

#[test]
fn test_out_neighbors_empty_graph() {
    let g = AdjMatrixGraphMtPer::new(5);
    let neighbors = g.out_neighbors(0);
    assert_eq!(neighbors.length(), 0);
}

#[test]
fn test_out_neighbors_out_of_bounds() {
    let g = AdjMatrixGraphMtPer::new(5);
    let neighbors = g.out_neighbors(10);
    assert_eq!(neighbors.length(), 0);
}

#[test]
fn test_out_degree_empty_graph() {
    let g = AdjMatrixGraphMtPer::new(5);
    assert_eq!(g.out_degree(0), 0);
    assert_eq!(g.out_degree(4), 0);
}

#[test]
fn test_out_degree_out_of_bounds() {
    let g = AdjMatrixGraphMtPer::new(5);
    assert_eq!(g.out_degree(10), 0);
}

#[test]
fn test_complement_empty_graph() {
    let g = AdjMatrixGraphMtPer::new(3);
    let gc = g.complement();

    // Complement of empty graph should have all edges except self-loops
    assert_eq!(gc.num_vertices(), 3);
    assert_eq!(gc.num_edges(), 6); // 3*2 = 6 edges (no self-loops)

    // Check edges
    assert!(gc.has_edge(0, 1));
    assert!(gc.has_edge(0, 2));
    assert!(gc.has_edge(1, 0));
    assert!(gc.has_edge(1, 2));
    assert!(gc.has_edge(2, 0));
    assert!(gc.has_edge(2, 1));

    // No self-loops
    assert!(!gc.has_edge(0, 0));
    assert!(!gc.has_edge(1, 1));
    assert!(!gc.has_edge(2, 2));
}

#[test]
fn test_complement_single_vertex() {
    let g = AdjMatrixGraphMtPer::new(1);
    let gc = g.complement();

    assert_eq!(gc.num_vertices(), 1);
    assert_eq!(gc.num_edges(), 0);
    assert!(!gc.has_edge(0, 0));
}

#[test]
fn test_complement_zero_vertices() {
    let g = AdjMatrixGraphMtPer::new(0);
    let gc = g.complement();

    assert_eq!(gc.num_vertices(), 0);
    assert_eq!(gc.num_edges(), 0);
}

#[test]
fn test_clone() {
    let g1 = AdjMatrixGraphMtPer::new(5);
    let g2 = g1.clone();

    assert_eq!(g1.num_vertices(), g2.num_vertices());
    assert_eq!(g1.num_edges(), g2.num_edges());
}

#[test]
fn test_num_edges_sequential() {
    let g = AdjMatrixGraphMtPer::new(4);
    assert_eq!(g.num_edges(), 0);

    // Complement has all edges except self-loops
    let gc = g.complement();
    assert_eq!(gc.num_edges(), 12); // 4*3 = 12 edges
}

#[test]
fn test_out_neighbors_after_complement() {
    let g = AdjMatrixGraphMtPer::new(3);
    let gc = g.complement();

    let neighbors0 = gc.out_neighbors(0);
    assert_eq!(neighbors0.length(), 2);
    assert_eq!(*neighbors0.nth(0), 1);
    assert_eq!(*neighbors0.nth(1), 2);

    let neighbors1 = gc.out_neighbors(1);
    assert_eq!(neighbors1.length(), 2);
    assert_eq!(*neighbors1.nth(0), 0);
    assert_eq!(*neighbors1.nth(1), 2);
}

#[test]
fn test_out_degree_after_complement() {
    let g = AdjMatrixGraphMtPer::new(4);
    let gc = g.complement();

    assert_eq!(gc.out_degree(0), 3);
    assert_eq!(gc.out_degree(1), 3);
    assert_eq!(gc.out_degree(2), 3);
    assert_eq!(gc.out_degree(3), 3);
}

#[test]
fn test_complement_idempotence() {
    let g = AdjMatrixGraphMtPer::new(3);
    let gc = g.complement();
    let gcc = gc.complement();

    // Complement of complement should restore original
    assert_eq!(g.num_edges(), gcc.num_edges());
    assert_eq!(g.has_edge(0, 1), gcc.has_edge(0, 1));
    assert_eq!(g.has_edge(1, 0), gcc.has_edge(1, 0));
}

#[test]
fn test_multiple_vertices_all_operations() {
    let g = AdjMatrixGraphMtPer::new(10);

    assert_eq!(g.num_vertices(), 10);
    assert_eq!(g.num_edges(), 0);

    for i in 0..10 {
        assert_eq!(g.out_degree(i), 0);
        assert_eq!(g.out_neighbors(i).length(), 0);
    }

    let gc = g.complement();
    assert_eq!(gc.num_edges(), 90); // 10*9 = 90 edges
}
