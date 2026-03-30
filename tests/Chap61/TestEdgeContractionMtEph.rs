//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 61 Edge Contraction (Multi-threaded)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap61::EdgeContractionMtEph::EdgeContractionMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];

    for i in 0..n {
        let _ = vertices.insert(i);
    }

    for i in 0..n {
        let next = (i + 1) % n;
        let edge = if i < next { Edge(i, next) } else { Edge(next, i) };
        let _ = edges.insert(edge);
    }

    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

fn create_star_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];

    let _ = vertices.insert(0);
    for i in 1..=n {
        let _ = vertices.insert(i);
        let _ = edges.insert(Edge(0, i));
    }

    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_edge_contract_mt_cycle() {
    let graph = create_cycle_graph(8);

    let mut matching: SetStEph<Edge<usize>> = SetLit![];
    let _ = matching.insert(Edge(0, 1));
    let _ = matching.insert(Edge(3, 4));
    let _ = matching.insert(Edge(6, 7));

    let contracted = edge_contract_mt(&graph, &matching);

    assert_eq!(contracted.sizeV(), 5); // 3 merged + 2 singletons
    assert!(contracted.sizeE() < graph.sizeE());
}

#[test]
fn test_edge_contract_mt_star() {
    let graph = create_star_graph(5);

    let mut matching: SetStEph<Edge<usize>> = SetLit![];
    let _ = matching.insert(Edge(0, 1));

    let contracted = edge_contract_mt(&graph, &matching);

    assert_eq!(contracted.sizeV(), 5); // merged 0-1 + 4 singletons
}

#[test]
fn test_contract_round_mt_correctness() {
    let graph = create_cycle_graph(10);
    let contracted = contract_round_mt(&graph, 999);

    assert!(contracted.sizeV() <= graph.sizeV());
    assert!(contracted.sizeE() <= graph.sizeE());
}

#[test]
fn test_edge_contract_mt_empty_matching() {
    let graph = create_cycle_graph(5);
    let empty_matching: SetStEph<Edge<usize>> = SetLit![];
    let contracted = edge_contract_mt(&graph, &empty_matching);
    assert_eq!(contracted.sizeV(), graph.sizeV());
}

#[test]
fn test_edge_contract_mt_single_edge() {
    let mut vertices: SetStEph<usize> = SetLit![];
    let _ = vertices.insert(0);
    let _ = vertices.insert(1);
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);

    let matching = SetLit![Edge(0, 1)];
    let contracted = edge_contract_mt(&graph, &matching);
    assert_eq!(contracted.sizeV(), 1);
    assert_eq!(contracted.sizeE(), 0);
}

#[test]
fn test_contract_round_mt_reduces_vertices() {
    let graph = create_star_graph(8);
    let contracted = contract_round_mt(&graph, 42);
    assert!(contracted.sizeV() < graph.sizeV());
}

#[test]
fn test_contract_round_mt_complete_graph() {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..6usize {
        let _ = vertices.insert(i);
    }
    let mut edges: SetStEph<Edge<usize>> = SetLit![];
    for i in 0..6usize {
        for j in (i + 1)..6 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let contracted = contract_round_mt(&graph, 123);
    assert!(contracted.sizeV() < 6);
}
