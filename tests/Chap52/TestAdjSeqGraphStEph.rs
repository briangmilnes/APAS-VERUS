//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap52 AdjSeqGraphStEph.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap52::AdjSeqGraphStEph::AdjSeqGraphStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(5);
    assert_eq!(graph.num_vertices(), 5);
}

#[test]
fn test_set_edge() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(0, 2, true);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
}

#[test]
fn test_has_edge() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(3);
    graph.set_edge(0, 1, true);
    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
}

#[test]
fn test_out_neighbors() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(0, 2, true);
    let neighbors = graph.out_neighbors(0);
    assert_eq!(neighbors.length(), 2);
}

#[test]
fn test_out_degree() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(0, 2, true);
    assert_eq!(graph.out_degree(0), 2);
}

#[test]
fn test_remove_edge() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(3);
    graph.set_edge(0, 1, true);
    assert!(graph.has_edge(0, 1));
    graph.set_edge(0, 1, false);
    assert!(!graph.has_edge(0, 1));
}

#[test]
fn test_num_edges() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(1, 2, true);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_self_loop() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(3);
    graph.set_edge(1, 1, true);
    assert!(graph.has_edge(1, 1));
}

#[test]
fn test_empty_graph() {
    let graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(5);
    assert_eq!(graph.num_edges(), 0);
}

#[test]
fn test_from_seq() {
    // Create adj list manually: [[1, 2], [2], []]
    let adj0 = ArraySeqStEphS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2]);
    let adj2 = ArraySeqStEphS::empty();
    let adj_seq = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2]);
    
    let graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::from_seq(adj_seq);
    
    assert_eq!(graph.num_vertices(), 3);
    assert_eq!(graph.out_degree(0), 2);
    assert_eq!(graph.out_degree(1), 1);
    assert_eq!(graph.out_degree(2), 0);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(1, 2));
}

#[test]
fn test_set_neighbors() {
    let mut graph: AdjSeqGraphStEph = AdjSeqGraphStEphTrait::new(4);
    
    // Set neighbors for vertex 0
    let neighbors = ArraySeqStEphS::from_vec(vec![1, 2, 3]);
    graph.set_neighbors(0, neighbors);
    
    assert_eq!(graph.out_degree(0), 3);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(0, 3));
    
    // Replace neighbors
    let new_neighbors = ArraySeqStEphS::from_vec(vec![2]);
    graph.set_neighbors(0, new_neighbors);
    
    assert_eq!(graph.out_degree(0), 1);
    assert!(!graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(!graph.has_edge(0, 3));
}
