// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Chapter 66: Boruvka's MST Algorithm (Parallel Ephemeral)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap66::BoruvkaMtEph::BoruvkaMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::vstdplus::float::float::*;
use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::HashMapWithViewPlusTrait;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

#[test]
fn test_boruvka_mt_triangle() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, w(3.0));
    assert!(mst_labels.mem(&1));
    assert!(mst_labels.mem(&2));
}

#[test]
fn test_boruvka_mt_square() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 1, w(4.0), 3),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(6.0));
}

#[test]
fn test_boruvka_mt_complete_4() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(1, 3, w(2.0), 1),
        LabeledEdge(1, 4, w(3.0), 2),
        LabeledEdge(2, 3, w(4.0), 3),
        LabeledEdge(2, 4, w(5.0), 4),
        LabeledEdge(3, 4, w(6.0), 5),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(6.0));
}

#[test]
fn test_boruvka_mt_star() {
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(0, 1, w(1.0), 0),
        LabeledEdge(0, 2, w(1.0), 1),
        LabeledEdge(0, 3, w(1.0), 2),
        LabeledEdge(0, 4, w(1.0), 3),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(4.0));
}

#[test]
fn test_boruvka_mt_path() {
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 5, w(4.0), 3),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
}

#[test]
fn test_boruvka_mt_single_vertex() {
    let vertices = SetLit![1];
    let edges = SetLit![];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_mt_two_vertices() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![LabeledEdge(1, 2, w(5.0), 0)];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 1);
    assert_eq!(mst_w, w(5.0));
    assert!(mst_labels.mem(&0));
}

#[test]
fn test_boruvka_mt_cycle_5() {
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 5, w(4.0), 3),
        LabeledEdge(5, 1, w(10.0), 4),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
    assert!(!mst_labels.mem(&4));
}

#[test]
fn test_boruvka_mt_larger_graph() {
    let vertices = SetLit![1, 2, 3, 4, 5, 6, 7, 8];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 5, w(4.0), 3),
        LabeledEdge(5, 6, w(5.0), 4),
        LabeledEdge(6, 7, w(6.0), 5),
        LabeledEdge(7, 8, w(7.0), 6),
        LabeledEdge(8, 1, w(8.0), 7),
        LabeledEdge(1, 5, w(9.0), 8),
        LabeledEdge(2, 6, w(10.0), 9),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 7);
    assert!(mst_w.val < 40.0);
}

#[test]
fn test_vertex_bridges_mt_triangle() {
    use std::sync::Arc;
    let edges = vec![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let edges_arc = Arc::new(edges);
    let bridges = vertex_bridges_mt(edges_arc, 0, 3);
    assert_eq!(bridges.len(), 3);
    assert_eq!(bridges.get(&1), Some(&(3, w(1.0), 2)));
    assert_eq!(bridges.get(&2), Some(&(3, w(2.0), 1)));
    assert_eq!(bridges.get(&3), Some(&(1, w(1.0), 2)));
}

#[test]
fn test_vertex_bridges_mt_empty() {
    use std::sync::Arc;
    let edges: Vec<LabeledEdge<i32>> = vec![];
    let edges_arc = Arc::new(edges);
    let bridges = vertex_bridges_mt(edges_arc, 0, 0);
    assert_eq!(bridges.len(), 0);
}

#[test]
fn test_bridge_star_partition_mt() {
    use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    let vertices = vec![1, 2, 3];
    let mut bridges = HashMapWithViewPlus::new();
    bridges.insert(1, (2, w(1.0), 0));
    bridges.insert(2, (3, w(2.0), 1));
    let (remaining, partition) = bridge_star_partition_mt(vertices, bridges, 42, 0);
    assert!(remaining.size() + partition.len() <= 3);
    assert!(remaining.size() > 0 || partition.len() > 0);
}

#[test]
fn test_boruvka_mst_mt_direct() {
    let vertices = vec![1, 2, 3];
    let edges = vec![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let mst_labels = boruvka_mst_mt(vertices, edges, SetLit![], 42, 0);
    assert_eq!(mst_labels.size(), 2);
}

#[test]
fn test_boruvka_mst_mt_empty() {
    let vertices: Vec<i32> = vec![];
    let edges: Vec<LabeledEdge<i32>> = vec![];
    let mst_labels = boruvka_mst_mt(vertices, edges, SetLit![], 42, 0);
    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_mt_k5_complete() {
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(1, 3, w(2.0), 1),
        LabeledEdge(1, 4, w(3.0), 2),
        LabeledEdge(1, 5, w(4.0), 3),
        LabeledEdge(2, 3, w(5.0), 4),
        LabeledEdge(2, 4, w(6.0), 5),
        LabeledEdge(2, 5, w(7.0), 6),
        LabeledEdge(3, 4, w(8.0), 7),
        LabeledEdge(3, 5, w(9.0), 8),
        LabeledEdge(4, 5, w(10.0), 9),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
}

#[test]
fn test_boruvka_mt_equal_weights() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(5.0), 0),
        LabeledEdge(2, 3, w(5.0), 1),
        LabeledEdge(3, 4, w(5.0), 2),
        LabeledEdge(4, 1, w(5.0), 3),
    ];
    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(15.0));
}

#[test]
fn test_boruvka_mt_deterministic_across_seeds() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 1, w(4.0), 3),
    ];
    let w1 = mst_weight(&edges, &boruvka_mst_mt_with_seed(&vertices, &edges, 1));
    let w2 = mst_weight(&edges, &boruvka_mst_mt_with_seed(&vertices, &edges, 99));
    assert_eq!(w1, w2);
}
