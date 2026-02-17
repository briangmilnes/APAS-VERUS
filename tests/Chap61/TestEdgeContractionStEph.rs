//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 61 Edge Contraction (Sequential)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap61::EdgeContractionStEph::EdgeContractionStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphStEph<usize> {
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

    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

fn create_star_graph(n: usize) -> UnDirGraphStEph<usize> {
    let mut vertices: SetStEph<usize> = SetLit![];
    let mut edges: SetStEph<Edge<usize>> = SetLit![];

    let _ = vertices.insert(0);
    for i in 1..=n {
        let _ = vertices.insert(i);
        let _ = edges.insert(Edge(0, i));
    }

    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_edge_contract_cycle() {
    let graph = create_cycle_graph(6);

    // Create a matching of 2 edges
    let mut matching: SetStEph<Edge<usize>> = SetLit![];
    let _ = matching.insert(Edge(0, 1));
    let _ = matching.insert(Edge(3, 4));

    let contracted = edge_contract(&graph, &matching);

    // Should have 4 vertices (2 merged pairs + 2 singletons)
    assert_eq!(contracted.sizeV(), 4);

    // Should have fewer edges than original
    assert!(contracted.sizeE() < graph.sizeE());
}

#[test]
fn test_edge_contract_star() {
    let graph = create_star_graph(4);

    // Match center with one satellite
    let mut matching: SetStEph<Edge<usize>> = SetLit![];
    let _ = matching.insert(Edge(0, 1));

    let contracted = edge_contract(&graph, &matching);

    // 4 vertices (merged 0-1 + satellites 2,3,4)
    assert_eq!(contracted.sizeV(), 4);
}

#[test]
fn test_contract_round_cycle() {
    let graph = create_cycle_graph(8);
    let contracted = contract_round(&graph);

    // Should reduce vertices
    assert!(contracted.sizeV() < graph.sizeV());
    assert!(contracted.sizeE() <= graph.sizeE());
}
