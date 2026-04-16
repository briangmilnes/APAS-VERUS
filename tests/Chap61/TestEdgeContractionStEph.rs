// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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

#[test]
fn test_edge_contract_empty_matching() {
    let graph = create_cycle_graph(5);
    let empty_matching: SetStEph<Edge<usize>> = SetLit![];
    let contracted = edge_contract(&graph, &empty_matching);

    // No contraction — same graph.
    assert_eq!(contracted.sizeV(), graph.sizeV());
    assert_eq!(contracted.sizeE(), graph.sizeE());
}

#[test]
fn test_edge_contract_single_edge_graph() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);

    let matching = SetLit![Edge(0, 1)];
    let contracted = edge_contract(&graph, &matching);

    // Two vertices merged into one, no edges remain.
    assert_eq!(contracted.sizeV(), 1);
    assert_eq!(contracted.sizeE(), 0);
}

#[test]
fn test_edge_contract_path_graph() {
    // Path: 0-1-2-3-4
    let mut vertices = SetLit![];
    for i in 0..5usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..4usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);

    // Match non-adjacent edges: (0,1) and (2,3)
    let matching = SetLit![Edge(0, 1), Edge(2, 3)];
    let contracted = edge_contract(&graph, &matching);

    // 3 super-vertices: {0,1}, {2,3}, {4}
    assert_eq!(contracted.sizeV(), 3);
}

#[test]
fn test_contract_round_star() {
    let graph = create_star_graph(6);
    let contracted = contract_round(&graph);

    assert!(contracted.sizeV() < graph.sizeV());
}

#[test]
fn test_contract_round_small_complete() {
    // K4: 4 vertices, 6 edges
    let mut vertices = SetLit![];
    for i in 0..4usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..4usize {
        for j in (i + 1)..4 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let contracted = contract_round(&graph);

    assert!(contracted.sizeV() < 4);
}

#[test]
fn test_edge_contract_triangle() {
    let vertices = SetLit![0, 1, 2];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let matching = SetLit![Edge(0, 1)];
    let contracted = edge_contract(&graph, &matching);
    assert_eq!(contracted.sizeV(), 2); // {0,1} and {2}.
}

#[test]
fn test_contract_round_large_cycle() {
    let graph = create_cycle_graph(20);
    let contracted = contract_round(&graph);
    assert!(contracted.sizeV() < 20);
    assert!(contracted.sizeV() > 0);
}

#[test]
fn test_edge_contract_disconnected() {
    // Two disconnected edges.
    let mut vertices = SetLit![];
    for i in 0..4usize {
        let _ = vertices.insert(i);
    }
    let edges = SetLit![Edge(0, 1), Edge(2, 3)];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let matching = SetLit![Edge(0, 1), Edge(2, 3)];
    let contracted = edge_contract(&graph, &matching);
    assert_eq!(contracted.sizeV(), 2);
    assert_eq!(contracted.sizeE(), 0);
}

#[test]
fn test_edge_contract_preserves_unmatched() {
    // 3 vertices, match only one edge.
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![Edge(0, 1), Edge(1, 2)];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let matching = SetLit![Edge(0, 1)];
    let contracted = edge_contract(&graph, &matching);
    assert_eq!(contracted.sizeV(), 2); // {0,1} merged, {2} singleton.
}
