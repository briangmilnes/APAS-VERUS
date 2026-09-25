// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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

// Graph sweep (r224): small graphs of 2-8 vertices. StEph takes no seed, so each graph runs once.

/// Graph families for the sweep: (name, vertex count, edge list).
fn sweep_graphs() -> Vec<(String, usize, Vec<(usize, usize)>)> {
    let mut graphs = Vec::new();
    for n in 2..=8usize {
        graphs.push((format!("path{n}"), n, (0..n - 1).map(|i| (i, i + 1)).collect()));
        graphs.push((format!("star{n}"), n, (1..n).map(|i| (0, i)).collect()));
        if n >= 3 {
            graphs.push((format!("cycle{n}"), n, (0..n).map(|i| (i, (i + 1) % n)).collect()));
        }
        let mut complete = Vec::new();
        for u in 0..n {
            for w in u + 1..n {
                complete.push((u, w));
            }
        }
        graphs.push((format!("complete{n}"), n, complete));
        // Two paths and, for odd n, one isolated vertex.
        let half = n / 2;
        let mut split = Vec::new();
        for i in 0..half.saturating_sub(1) {
            split.push((i, i + 1));
        }
        for i in half..(2 * half).saturating_sub(1) {
            split.push((i, i + 1));
        }
        graphs.push((format!("disconnected{n}"), n, split));
        graphs.push((format!("empty{n}"), n, Vec::new()));
        // A path with a self-loop at every vertex.
        let mut looped: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        for i in 0..n {
            looped.push((i, i));
        }
        graphs.push((format!("looped_path{n}"), n, looped));
    }
    graphs
}

fn uf_find(parent: &mut Vec<usize>, x: usize) -> usize {
    let mut r = x;
    while parent[r] != r {
        r = parent[r];
    }
    let mut y = x;
    while parent[y] != r {
        let next = parent[y];
        parent[y] = r;
        y = next;
    }
    r
}

/// Components of the graph, by union-find over its edges.
fn component_labels(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut parent: Vec<usize> = (0..n).collect();
    for &(u, w) in edges {
        let (ru, rw) = (uf_find(&mut parent, u), uf_find(&mut parent, w));
        if ru != rw {
            parent[ru] = rw;
        }
    }
    (0..n).map(|v| uf_find(&mut parent, v)).collect()
}

/// Checks that `tree` is a spanning forest of the graph: every tree edge is a
/// graph edge, no tree edge closes a cycle, and the tree connects exactly the
/// graph's components. Hence |tree| = |V| - (number of components).
fn check_spanning_forest(name: &str, seed: u64, n: usize, edges: &[(usize, usize)], tree: &[(usize, usize)]) {
    let graph_labels = component_labels(n, edges);
    let mut roots: Vec<usize> = graph_labels.clone();
    roots.sort();
    roots.dedup();
    let components = roots.len();
    assert_eq!(tree.len(), n - components, "{name} seed {seed}: |tree| = {} but |V| - components = {}", tree.len(), n - components);
    let mut parent: Vec<usize> = (0..n).collect();
    for &(u, w) in tree {
        assert!(edges.contains(&(u, w)) || edges.contains(&(w, u)), "{name} seed {seed}: tree edge ({u}, {w}) is not a graph edge");
        let (ru, rw) = (uf_find(&mut parent, u), uf_find(&mut parent, w));
        assert_ne!(ru, rw, "{name} seed {seed}: tree edge ({u}, {w}) closes a cycle");
        parent[ru] = rw;
    }
    for u in 0..n {
        for w in 0..n {
            let same_graph = graph_labels[u] == graph_labels[w];
            let same_tree = uf_find(&mut parent, u) == uf_find(&mut parent, w);
            assert_eq!(same_graph, same_tree, "{name} seed {seed}: vertices {u}, {w} connectivity differs");
        }
    }
}

fn build_st_graph(n: usize, edges: &[(usize, usize)]) -> UnDirGraphStEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edge_set = SetLit![];
    for &(u, w) in edges {
        let _ = edge_set.insert(Edge(u, w));
    }
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edge_set)
}

#[test]
fn test_spanning_tree_graph_sweep() {
    for (name, n, edges) in sweep_graphs() {
        let graph = build_st_graph(n, &edges);
        let labels = component_labels(n, &edges);
        let connected = labels.iter().all(|&l| l == labels[0]);
        let tree = spanning_tree_star_contraction(&graph);
        let tree_vec: Vec<(usize, usize)> = tree.iter().map(|e| (e.0, e.1)).collect();
        check_spanning_forest(&name, 0, n, &edges, &tree_vec);
        if connected {
            assert!(verify_spanning_tree(&graph, &tree), "{name}: verify_spanning_tree rejected");
        }
    }
}
