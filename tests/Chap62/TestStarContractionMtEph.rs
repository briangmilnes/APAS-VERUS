//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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

#[test]
fn test_contract_with_base_expand_mt() {
    let graph = create_cycle_graph(6);

    // Simple base function that counts vertices
    let base = |vertices: &SetStEph<usize>| vertices.size();

    // Expand function that just returns the recursive result
    let expand = |_v: &SetStEph<usize>, _e: &SetStEph<Edge<usize>>, _centers: &SetStEph<usize>, _part: &HashMapWithViewPlus<usize, usize>, r: usize| r;

    let result = star_contract_mt(&graph, 456, &base, &expand);

    // Should eventually contract to some number of isolated vertices
    assert!(result > 0);
}

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
