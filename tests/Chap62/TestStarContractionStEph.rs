//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Contraction - Sequential Ephemeral Tests

use std::collections::HashMap;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap62::StarContractionStEph::StarContractionStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: N) -> UnDirGraphStEph<N> {
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
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges)
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
    let base = |vertices: &SetStEph<N>| vertices.size();

    // Expand function that just returns the recursive result
    let expand = |_v: &SetStEph<N>, _e: &SetStEph<Edge<N>>, _centers: &SetStEph<N>, _part: &HashMap<N, N>, r: N| r;

    let result = star_contract(&graph, &base, &expand);

    // Should eventually contract to some number of isolated vertices
    assert!(result > 0);
}

#[test]
fn test_empty_graph_contraction() {
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::empty();
    let result = contract_to_vertices(&graph);

    assert_eq!(result.size(), 0);
}

#[test]
fn test_single_edge_contraction() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges);

    let result = contract_to_vertices(&graph);

    // Should contract to at least one vertex
    assert!(result.size() >= 1);
    assert!(result.size() <= 2);
}
