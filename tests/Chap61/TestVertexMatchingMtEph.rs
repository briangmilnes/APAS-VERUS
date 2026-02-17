//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 61 Vertex Matching (Multi-threaded)

use std::vec::Vec;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap61::VertexMatchingMtEph::VertexMatchingMtEph::*;
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
fn test_parallel_matching_mt_cycle() {
    let graph = create_cycle_graph(10);
    let matching = parallel_matching_mt(&graph, 789);

    // Verify matching property
    let mut matched_vertices: SetStEph<usize> = SetLit![];
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(!matched_vertices.mem(u));
        assert!(!matched_vertices.mem(v));
        let _ = matched_vertices.insert(*u);
        let _ = matched_vertices.insert(*v);
    }
}

#[test]
fn test_parallel_matching_mt_star() {
    let graph = create_star_graph(6);
    let matching = parallel_matching_mt(&graph, 456);

    // Verify matching property
    let mut matched_vertices: SetStEph<usize> = SetLit![];
    for edge in matching.iter() {
        let Edge(u, v) = edge;
        assert!(!matched_vertices.mem(u));
        assert!(!matched_vertices.mem(v));
        let _ = matched_vertices.insert(*u);
        let _ = matched_vertices.insert(*v);
    }

    assert!(matching.size() <= 1);
}

#[test]
fn test_parallel_matching_mt_correctness() {
    let graph = create_cycle_graph(12);
    let matching = parallel_matching_mt(&graph, 101);

    // All edges in matching must be in graph
    for edge in matching.iter() {
        assert!(graph.edges().mem(edge));
    }

    // No two edges share an endpoint
    let all_edges = matching.iter().collect::<Vec<&Edge<usize>>>();
    for i in 0..all_edges.len() {
        for j in (i + 1)..all_edges.len() {
            let Edge(u1, v1) = all_edges[i];
            let Edge(u2, v2) = all_edges[j];
            assert!(u1 != u2 && u1 != v2 && v1 != u2 && v1 != v2);
        }
    }
}
