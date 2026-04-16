#![cfg(feature = "all_chapters")]
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
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges)
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 789);

    assert_eq!(tree.size(), 0);
}

#[test]
fn test_spanning_tree_mt_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
    let tree = spanning_tree_star_contraction_mt(&graph, 42);
    assert_eq!(tree.size(), 0);
    assert!(verify_spanning_tree(&graph, &tree));
}

#[test]
fn test_spanning_tree_mt_two_vertices() {
    let vertices = SetLit![0, 1];
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::FromSets(vertices, edges);
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
