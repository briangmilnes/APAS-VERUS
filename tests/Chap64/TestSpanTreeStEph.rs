//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Spanning Tree via Star Contraction Tests (Sequential)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap64::SpanTreeStEph::SpanTreeStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphStEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

fn create_connected_graph() -> UnDirGraphStEph<usize> {
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
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
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
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
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

#[test]
fn test_spanning_tree_two_vertices() {
    let vertices = SetLit![0, 1];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 1);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_complete_graph_4() {
    let mut vertices = SetLit![];
    for i in 0..4 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..4usize {
        for j in (i + 1)..4 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    // Spanning tree of K4 should have exactly 3 edges.
    assert_eq!(tree.size(), 3);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_path_graph() {
    let n = 10;
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..(n - 1) {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    // Path graph is already a tree, so spanning tree = all edges.
    assert_eq!(tree.size(), n - 1);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_star_graph() {
    let n = 8;
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..n {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), n - 1);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_large_cycle() {
    let graph = create_cycle_graph(50);
    let tree = spanning_tree_star_contraction(&graph);

    assert_eq!(tree.size(), 49);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_disconnected_pair() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);

    // Disconnected graph — spanning tree cannot connect both vertices.
    assert_eq!(tree.size(), 0);
}

#[test]
fn test_verify_spanning_tree_empty_tree_on_connected() {
    let graph = create_connected_graph();
    let empty_tree = SetLit![];
    assert!(!verify_spanning_tree(&graph, &empty_tree));
}

#[test]
fn test_verify_spanning_tree_too_many_edges() {
    let graph = create_cycle_graph(4);
    // A tree with too many edges (all cycle edges) is not a spanning tree.
    let mut all_edges = SetLit![];
    for i in 0..4usize {
        let _ = all_edges.insert(Edge(i, (i + 1) % 4));
    }
    assert!(!verify_spanning_tree(&graph, &all_edges));
}
