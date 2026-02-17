//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap52 AdjMatrixGraphStPer.

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap52::AdjMatrixGraphStPer::AdjMatrixGraphStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(5);
    assert_eq!(graph.num_vertices(), 5);
}

#[test]
fn test_has_edge() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(3);
    let graph2 = graph.set_edge(0, 1, true);
    assert!(graph2.has_edge(0, 1));
    assert!(!graph.has_edge(0, 1));
}

#[test]
fn test_out_neighbors() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(4);
    let graph2 = graph.set_edge(0, 1, true).set_edge(0, 2, true);
    let neighbors = graph2.out_neighbors(0);
    assert_eq!(neighbors.length(), 2);
}

#[test]
fn test_out_degree() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(4);
    let graph2 = graph.set_edge(0, 1, true).set_edge(0, 2, true);
    assert_eq!(graph2.out_degree(0), 2);
}

#[test]
fn test_set_edge() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(3);
    let graph2 = graph.set_edge(0, 1, true);
    assert!(graph2.has_edge(0, 1));
    let graph3 = graph2.set_edge(0, 1, false);
    assert!(!graph3.has_edge(0, 1));
}

#[test]
fn test_complement() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(3);
    let graph2 = graph.set_edge(0, 1, true);
    let comp = graph2.complement();
    assert!(!comp.has_edge(0, 1));
}

#[test]
fn test_num_edges() {
    let graph: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(4);
    let graph2 = graph.set_edge(0, 1, true).set_edge(1, 2, true);
    assert_eq!(graph2.num_edges(), 2);
}

#[test]
fn test_persistent_semantics() {
    let graph1: AdjMatrixGraphStPer = AdjMatrixGraphStPerTrait::new(3);
    let graph2 = graph1.set_edge(0, 1, true);
    assert!(!graph1.has_edge(0, 1));
    assert!(graph2.has_edge(0, 1));
}

#[test]
fn test_from_matrix() {
    use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;

    // Create a 3x3 matrix manually
    let row0 = ArraySeqStPerS::from_vec(vec![false, true, false]);
    let row1 = ArraySeqStPerS::from_vec(vec![false, false, true]);
    let row2 = ArraySeqStPerS::from_vec(vec![false, false, false]);
    let matrix = ArraySeqStPerS::from_vec(vec![row0, row1, row2]);
    
    let g = AdjMatrixGraphStPer::from_matrix(matrix);
    
    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 2); // edges 0->1 and 1->2
    assert!(g.has_edge(0, 1));
    assert!(g.has_edge(1, 2));
    assert!(!g.has_edge(0, 2));
}
