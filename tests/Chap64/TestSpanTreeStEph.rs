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

#[test]
fn test_spanning_tree_double_cycle() {
    // Two cycles sharing a vertex.
    let mut vertices = SetLit![];
    for i in 0..10 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Cycle 1: 0-1-2-3-4-0
    for i in 0..5 {
        let _ = edges.insert(Edge(i, (i + 1) % 5));
    }
    // Cycle 2: 0-5-6-7-8-9-0
    let _ = edges.insert(Edge(0, 5));
    for i in 5..9 {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let _ = edges.insert(Edge(9, 0));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 9);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_grid_2x3() {
    // 2x3 grid: vertices 0..5, row-major.
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Horizontal: (0,1),(1,2),(3,4),(4,5)
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(4, 5));
    // Vertical: (0,3),(1,4),(2,5)
    let _ = edges.insert(Edge(0, 3));
    let _ = edges.insert(Edge(1, 4));
    let _ = edges.insert(Edge(2, 5));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_complete_5() {
    let mut vertices = SetLit![];
    for i in 0..5 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..5usize {
        for j in (i + 1)..5 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 4);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_three_isolated() {
    // Three isolated vertices — no edges possible.
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_binary_tree_topology() {
    // Binary tree: 0 root, children 1,2; 1's children 3,4; 2's children 5,6.
    let mut vertices = SetLit![];
    for i in 0..7 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(0, 2));
    let _ = edges.insert(Edge(1, 3));
    let _ = edges.insert(Edge(1, 4));
    let _ = edges.insert(Edge(2, 5));
    let _ = edges.insert(Edge(2, 6));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    // Already a tree, so spanning tree = all edges.
    assert_eq!(tree.size(), 6);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_wheel_graph() {
    // Wheel: center vertex 0 connected to cycle 1-2-3-4-5-1.
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..6usize {
        let _ = edges.insert(Edge(0, i)); // Hub to spokes.
    }
    for i in 1..5usize {
        let _ = edges.insert(Edge(i, i + 1)); // Cycle.
    }
    let _ = edges.insert(Edge(5, 1));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_large_path() {
    let n = 30;
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
    assert_eq!(tree.size(), n - 1);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_petersen_like() {
    // Outer cycle 0-4, inner star 5-9, connections between.
    let mut vertices = SetLit![];
    for i in 0..10 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Outer cycle.
    for i in 0..5usize {
        let _ = edges.insert(Edge(i, (i + 1) % 5));
    }
    // Inner star (pentagrams).
    let _ = edges.insert(Edge(5, 7));
    let _ = edges.insert(Edge(7, 9));
    let _ = edges.insert(Edge(9, 6));
    let _ = edges.insert(Edge(6, 8));
    let _ = edges.insert(Edge(8, 5));
    // Spokes.
    for i in 0..5usize {
        let _ = edges.insert(Edge(i, i + 5));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 9);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_verify_spanning_tree_wrong_edge() {
    let graph = create_cycle_graph(5);
    let mut tree = SetLit![];
    // Make a tree-sized set but with a non-graph edge.
    let _ = tree.insert(Edge(0, 1));
    let _ = tree.insert(Edge(1, 2));
    let _ = tree.insert(Edge(2, 3));
    let _ = tree.insert(Edge(3, 99)); // Vertex not in graph.
    assert!(!verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_triangle() {
    let vertices = SetLit![0, 1, 2];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 2);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_barbell() {
    // Two triangles connected by a bridge edge.
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Triangle 1: 0-1-2.
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    // Bridge.
    let _ = edges.insert(Edge(2, 3));
    // Triangle 2: 3-4-5.
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(4, 5));
    let _ = edges.insert(Edge(3, 5));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction(&graph);
    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}
