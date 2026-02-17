//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 61 Vertex Matching (Sequential)

use std::vec::Vec;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap61::VertexMatchingStEph::VertexMatchingStEph::*;
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

    let _ = vertices.insert(0); // Center
    for i in 1..=n {
        let _ = vertices.insert(i);
        let _ = edges.insert(Edge(0, i));
    }

    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_greedy_matching_cycle() {
    let graph = create_cycle_graph(6);
    let matching = greedy_matching(&graph);

    // Greedy should find at least 2 edges in a cycle of 6
    assert!(matching.size() >= 2);

    // Verify matching property: no two edges share an endpoint
    let mut matched_vertices: SetStEph<usize> = SetLit![];
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(!matched_vertices.mem(u), "Vertex {u} already matched");
        assert!(!matched_vertices.mem(v), "Vertex {v} already matched");
        let _ = matched_vertices.insert(*u);
        let _ = matched_vertices.insert(*v);
    }
}

#[test]
fn test_greedy_matching_star() {
    let graph = create_star_graph(5);
    let matching = greedy_matching(&graph);

    // In a star, greedy can only match 1 edge (center is matched first)
    assert_eq!(matching.size(), 1);

    // Verify the matched edge includes the center
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(u == &0 || v == &0, "Star matching must include center");
    }
}

#[test]
fn test_parallel_matching_st_cycle() {
    let graph = create_cycle_graph(8);
    let matching = parallel_matching_st(&graph, 42);

    // Verify matching property: no two edges share an endpoint
    let mut matched_vertices: SetStEph<usize> = SetLit![];
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(!matched_vertices.mem(u), "Vertex {u} already matched");
        assert!(!matched_vertices.mem(v), "Vertex {v} already matched");
        let _ = matched_vertices.insert(*u);
        let _ = matched_vertices.insert(*v);
    }

    // In expectation, should match ~1/8 of edges (but test is deterministic with seed)
    assert!(matching.size() <= graph.sizeE());
}

#[test]
fn test_parallel_matching_st_star() {
    let graph = create_star_graph(4);
    let matching = parallel_matching_st(&graph, 123);

    // Verify matching property
    let mut matched_vertices: SetStEph<usize> = SetLit![];
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(!matched_vertices.mem(u));
        assert!(!matched_vertices.mem(v));
        let _ = matched_vertices.insert(*u);
        let _ = matched_vertices.insert(*v);
    }

    // At most 1 edge in a star matching
    assert!(matching.size() <= 1);
}

#[test]
fn test_matching_properties() {
    let graph = create_cycle_graph(10);
    let matching = greedy_matching(&graph);

    // Property 1: All edges in matching are in the graph
    for edge in matching.iter() {
        assert!(graph.edges().mem(edge), "Matched edge not in graph");
    }

    // Property 2: No two edges share an endpoint
    let all_edges = matching.iter().collect::<Vec<&Edge<usize>>>();
    for i in 0..all_edges.len() {
        for j in (i + 1)..all_edges.len() {
            let Edge(u1, v1) = all_edges[i];
            let Edge(u2, v2) = all_edges[j];
            assert!(u1 != u2 && u1 != v2 && v1 != u2 && v1 != v2, "Edges share endpoint");
        }
    }
}
