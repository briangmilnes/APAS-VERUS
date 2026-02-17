//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap52 AdjSeqGraphStPer.

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap52::AdjSeqGraphStPer::AdjSeqGraphStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(5);
    assert_eq!(graph.num_vertices(), 5);
}

#[test]
fn test_insert_edge() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(4);
    let graph2 = graph.insert_edge(0, 1).insert_edge(0, 2);
    assert!(graph2.has_edge(0, 1));
    assert!(graph2.has_edge(0, 2));
}

#[test]
fn test_has_edge() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(3);
    let graph2 = graph.insert_edge(0, 1);
    assert!(graph2.has_edge(0, 1));
    assert!(!graph2.has_edge(1, 0));
}

#[test]
fn test_out_neighbors() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(4);
    let graph2 = graph.insert_edge(0, 1).insert_edge(0, 2);
    let neighbors = graph2.out_neighbors(0);
    assert_eq!(neighbors.length(), 2);
}

#[test]
fn test_out_degree() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(4);
    let graph2 = graph.insert_edge(0, 1).insert_edge(0, 2);
    assert_eq!(graph2.out_degree(0), 2);
}

#[test]
fn test_delete_edge() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(3);
    let graph2 = graph.insert_edge(0, 1);
    assert!(graph2.has_edge(0, 1));
    let graph3 = graph2.delete_edge(0, 1);
    assert!(!graph3.has_edge(0, 1));
}

#[test]
fn test_num_edges() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(4);
    let graph2 = graph.insert_edge(0, 1).insert_edge(1, 2);
    assert_eq!(graph2.num_edges(), 2);
}

#[test]
fn test_persistent_semantics() {
    let graph1: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(3);
    let graph2 = graph1.insert_edge(0, 1);
    assert!(!graph1.has_edge(0, 1));
    assert!(graph2.has_edge(0, 1));
}

#[test]
fn test_empty_graph() {
    let graph: AdjSeqGraphStPer = AdjSeqGraphStPerTrait::new(5);
    assert_eq!(graph.num_edges(), 0);
}
