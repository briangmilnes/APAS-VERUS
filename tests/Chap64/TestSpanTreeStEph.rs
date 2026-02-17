#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Spanning Tree via Star Contraction Tests (Sequential)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap64::SpanTreeStEph::SpanTreeStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

fn create_connected_graph() -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(2, 3));
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(4, 5));
    let _ = edges.insert(Edge(5, 0));
    let _ = edges.insert(Edge(1, 4));
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges)
}

#[test]
fn test_spanning_tree_cycle() {
    let graph = create_cycle_graph(6);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_connected() {
    let graph = create_connected_graph();
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_empty() {
    let vertices = SetLit![];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::FromSets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 0);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_verify_spanning_tree() {
    let graph = create_cycle_graph(5);
    let tree = spanning_tree_star_contraction(&graph);

    assert!(verify_spanning_tree(&graph, &tree));

    let mut bad_tree = tree.clone();
    let _ = bad_tree.insert(Edge(100, 200));
    assert!(!verify_spanning_tree(&graph, &bad_tree));
}
