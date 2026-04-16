// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 62: Star Contraction - Sequential Ephemeral Tests

use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

use vstd::prelude::Ghost;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap62::StarContractionStEph::StarContractionStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphStEph<usize> {
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
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_contract_to_vertices_cycle() {
    let graph = create_cycle_graph(6);
    let result = contract_to_vertices(&graph);

    // After contracting, we should have fewer vertices than original
    assert!(result.size() <= graph.sizeV());
    assert!(result.size() > 0);
}

#[test]
fn test_contract_with_base_expand() {
    let graph = create_cycle_graph(4);

    // Simple base function that counts vertices
    let base = |vertices: &SetStEph<usize>| vertices.size();

    // Expand function that just returns the recursive result
    let expand = |_v: &SetStEph<usize>, _e: &SetStEph<Edge<usize>>, _centers: &SetStEph<usize>, _part: &HashMapWithViewPlus<usize, usize>, r: usize| r;

    let result = star_contract(&graph, &base, &expand, Ghost::assume_new());

    // Should eventually contract to some number of isolated vertices
    assert!(result > 0);
}

#[test]
fn test_empty_graph_contraction() {
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::empty();
    let result = contract_to_vertices(&graph);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_single_edge_contraction() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);

    let result = contract_to_vertices(&graph);

    // Should contract to at least one vertex
    assert!(result.size() >= 1);
    assert!(result.size() <= 2);
}

#[test]
fn test_path_contraction() {
    let mut vertices = SetLit![];
    for i in 0..8usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..7usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices(&graph);

    assert!(result.size() > 0);
    assert!(result.size() <= 8);
}

#[test]
fn test_star_graph_contraction() {
    let mut vertices = SetLit![0];
    for i in 1..6usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..6usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices(&graph);

    assert!(result.size() > 0);
    assert!(result.size() <= 6);
}

#[test]
fn test_complete_graph_contraction() {
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
    let result = contract_to_vertices(&graph);

    assert!(result.size() > 0);
}

#[test]
fn test_single_vertex_contraction() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices(&graph);
    assert_eq!(result.size(), 1);
}

#[test]
fn test_triangle_contraction() {
    let vertices = SetLit![0, 1, 2];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices(&graph);
    assert!(result.size() >= 1 && result.size() <= 3);
}

#[test]
fn test_large_cycle_contraction() {
    let graph = create_cycle_graph(30);
    let result = contract_to_vertices(&graph);
    assert!(result.size() > 0);
    assert!(result.size() <= 30);
}

#[test]
fn test_disconnected_contraction() {
    // Two disconnected pairs.
    let vertices = SetLit![0, 1, 2, 3];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(2, 3));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let result = contract_to_vertices(&graph);
    assert!(result.size() >= 2); // At least one per component.
    assert!(result.size() <= 4);
}
