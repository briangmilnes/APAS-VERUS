// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 64: Spanning Tree via Star Contraction Tests (Parallel)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap62::StarPartitionMtEph::StarPartitionMtEph::*;
use apas_verus::Chap64::SpanTreeMtEph::SpanTreeMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: usize) -> UnDirGraphMtEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_spanning_tree_mt_cycle() {
    let graph = create_cycle_graph(6);
    let tree = spanning_tree_star_contraction_mt(&graph, 123);

    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_larger() {
    let graph = create_cycle_graph(10);
    let tree = spanning_tree_star_contraction_mt(&graph, 456);

    assert_eq!(tree.size(), 9);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_empty() {
    let vertices = SetLit![];
    let edges = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 789);

    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_mt_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 0);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_two_vertices() {
    let vertices = SetLit![0, 1];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 1);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_complete_4() {
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 3);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_path() {
    let mut vertices = SetLit![];
    for i in 0..8 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..7usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 7);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_star() {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..6usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_large_cycle() {
    let graph = create_cycle_graph(30);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 29);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_disconnected_pair() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_mt_wheel() {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..6usize {
        let _ = edges.insert(Edge(0, i));
    }
    for i in 1..5usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let _ = edges.insert(Edge(5, 1));
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 5);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_different_seeds() {
    let graph = create_cycle_graph(10);
    for seed in [1, 42, 100, 999, 12345] {
        let tree = spanning_tree_star_contraction_mt(&graph, seed);
        assert_eq!(tree.size(), 9, "Wrong tree size with seed {seed}");
        assert!(verify_spanning_tree(&graph, &tree), "Invalid tree with seed {seed}");
    }
}

// Seed sweep (r224): small graphs of 2-8 vertices, seeds 0..200.

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

fn build_mt_graph(n: usize, edges: &[(usize, usize)]) -> UnDirGraphMtEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edge_set = SetLit![];
    for &(u, w) in edges {
        let _ = edge_set.insert(Edge(u, w));
    }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edge_set)
}

#[test]
fn test_spanning_tree_mt_seed_sweep() {
    for (name, n, edges) in sweep_graphs() {
        let graph = build_mt_graph(n, &edges);
        let labels = component_labels(n, &edges);
        let connected = labels.iter().all(|&l| l == labels[0]);
        for seed in 0..200u64 {
            let tree = spanning_tree_star_contraction_mt(&graph, seed);
            let tree_vec: Vec<(usize, usize)> = tree.iter().map(|e| (e.0, e.1)).collect();
            check_spanning_forest(&name, seed, n, &edges, &tree_vec);
            if connected {
                assert!(verify_spanning_tree(&graph, &tree), "{name} seed {seed}: verify_spanning_tree rejected");
            }
        }
    }
}
