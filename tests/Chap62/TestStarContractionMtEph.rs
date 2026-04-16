// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 62: Star Contraction - Multi-threaded Ephemeral Tests

use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap62::StarContractionMtEph::StarContractionMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let u = i;
        let v = (i + 1) % n;
        let _ = edges.insert(Edge(u, v));
    }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_contract_to_vertices_mt_cycle() {
    let graph = create_cycle_graph(8);
    let result = contract_to_vertices_mt(&graph, 123);

    // After contracting, we should have fewer or equal vertices
    assert!(result.size() <= graph.sizeV());
    assert!(result.size() > 0);
}

// test_contract_with_base_expand_mt removed: star_contract_mt requires Ghost<spec_fn> 5th arg, not callable from RTT.

#[test]
fn test_determinism_mt() {
    let graph = create_cycle_graph(6);

    // Same seed should give same result
    let result1 = contract_to_vertices_mt(&graph, 789);
    let result2 = contract_to_vertices_mt(&graph, 789);

    assert_eq!(result1.size(), result2.size());
}

#[test]
fn test_empty_graph_contraction_mt() {
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::empty();
    let result = contract_to_vertices_mt(&graph, 999);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_single_edge_mt() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices_mt(&graph, 42);
    assert!(result.size() >= 1 && result.size() <= 2);
}

#[test]
fn test_large_cycle_mt() {
    let graph = create_cycle_graph(50);
    let result = contract_to_vertices_mt(&graph, 42);
    assert!(result.size() > 0);
    assert!(result.size() <= 50);
}

#[test]
fn test_complete_graph_mt() {
    let mut vertices = SetLit![];
    for i in 0..5usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..5usize {
        for j in (i + 1)..5 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices_mt(&graph, 123);
    assert!(result.size() > 0);
}

#[test]
fn test_single_vertex_mt() {
    let vertices = SetLit![0];
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices_mt(&graph, 42);
    assert_eq!(result.size(), 1);
}

#[test]
fn test_triangle_mt() {
    let vertices = SetLit![0, 1, 2];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices_mt(&graph, 42);
    assert!(result.size() >= 1 && result.size() <= 3);
}

#[test]
fn test_path_graph_mt() {
    let mut vertices = SetLit![];
    for i in 0..10usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..9usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices_mt(&graph, 42);
    assert!(result.size() > 0 && result.size() <= 10);
}
