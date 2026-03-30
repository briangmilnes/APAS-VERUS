//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 66: Boruvka's MST Algorithm (Sequential Ephemeral)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap66::BoruvkaStEph::BoruvkaStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::vstdplus::float::float::*;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

#[test]
fn test_boruvka_triangle() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, w(3.0));
    assert!(mst_labels.mem(&1));
    assert!(mst_labels.mem(&2));
}

#[test]
fn test_boruvka_square() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 1, w(4.0), 3),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(6.0));
}

#[test]
fn test_boruvka_complete_4() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(1, 3, w(2.0), 1),
        LabeledEdge(1, 4, w(3.0), 2),
        LabeledEdge(2, 3, w(4.0), 3),
        LabeledEdge(2, 4, w(5.0), 4),
        LabeledEdge(3, 4, w(6.0), 5),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(6.0)); // 1+2+3
}

#[test]
fn test_boruvka_star() {
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(0, 1, w(1.0), 0),
        LabeledEdge(0, 2, w(1.0), 1),
        LabeledEdge(0, 3, w(1.0), 2),
        LabeledEdge(0, 4, w(1.0), 3),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(4.0));
}

#[test]
fn test_boruvka_path() {
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 5, w(4.0), 3),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
}

#[test]
fn test_boruvka_single_vertex() {
    let vertices = SetLit![1];
    let edges = SetLit![];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_two_vertices() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![LabeledEdge(1, 2, w(5.0), 0)];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 1);
    assert_eq!(mst_w, w(5.0));
    assert!(mst_labels.mem(&0));
}

#[test]
fn test_boruvka_cycle_5() {
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 5, w(4.0), 3),
        LabeledEdge(5, 1, w(10.0), 4),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
    assert!(!mst_labels.mem(&4));
}

#[test]
fn test_vertex_bridges_triangle() {
    let edges = SetLit![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let bridges = BoruvkaStEph::vertex_bridges(&edges);
    assert_eq!(bridges.inner.len(), 3);
    assert_eq!(bridges.inner.get(&1), Some(&(3, w(1.0), 2)));
    assert_eq!(bridges.inner.get(&2), Some(&(3, w(2.0), 1)));
    assert_eq!(bridges.inner.get(&3), Some(&(1, w(1.0), 2)));
}

#[test]
fn test_vertex_bridges_star() {
    let edges = SetLit![
        LabeledEdge(0, 1, w(1.0), 0),
        LabeledEdge(0, 2, w(2.0), 1),
        LabeledEdge(0, 3, w(3.0), 2),
    ];
    let bridges = BoruvkaStEph::vertex_bridges(&edges);
    assert_eq!(bridges.inner.len(), 4);
    assert_eq!(bridges.inner.get(&0), Some(&(1, w(1.0), 0)));
    assert_eq!(bridges.inner.get(&1), Some(&(0, w(1.0), 0)));
    assert_eq!(bridges.inner.get(&2), Some(&(0, w(2.0), 1)));
    assert_eq!(bridges.inner.get(&3), Some(&(0, w(3.0), 2)));
}

#[test]
fn test_vertex_bridges_empty() {
    let edges: SetStEph<LabeledEdge<i32>> = SetLit![];
    let bridges = BoruvkaStEph::vertex_bridges(&edges);
    assert_eq!(bridges.inner.len(), 0);
}

#[test]
fn test_bridge_star_partition() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let bridges = BoruvkaStEph::vertex_bridges(&edges);
    let (remaining, partition) = BoruvkaStEph::bridge_star_partition(&vertices, &bridges, 42);
    assert!(remaining.size() + partition.inner.len() <= 3);
    assert!(remaining.size() > 0 || partition.inner.len() > 0);
}

#[test]
fn test_bridge_star_partition_single_vertex() {
    use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::HashMapWithViewPlus;
    let vertices = SetLit![1];
    let bridges: HashMapWithViewPlus<i32, (i32, WrappedF64, usize)> =
        HashMapWithViewPlus { inner: std::collections::HashMap::new() };
    let (remaining, partition) = BoruvkaStEph::bridge_star_partition(&vertices, &bridges, 42);
    assert_eq!(remaining.size(), 1);
    assert_eq!(partition.inner.len(), 0);
}

#[test]
fn test_boruvka_mst_direct() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, w(3.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 1, w(1.0), 2),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst(&vertices, &edges, SetLit![], 42);
    assert_eq!(mst_labels.size(), 2);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_w, w(3.0));
}

#[test]
fn test_boruvka_mst_direct_empty() {
    let vertices = SetLit![1];
    let edges: SetStEph<LabeledEdge<i32>> = SetLit![];
    let mst_labels = BoruvkaStEph::boruvka_mst(&vertices, &edges, SetLit![], 42);
    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_k5_complete() {
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
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, w(10.0));
}

#[test]
fn test_boruvka_equal_weights() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(5.0), 0),
        LabeledEdge(2, 3, w(5.0), 1),
        LabeledEdge(3, 4, w(5.0), 2),
        LabeledEdge(4, 1, w(5.0), 3),
        LabeledEdge(1, 3, w(5.0), 4),
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, w(15.0));
}

#[test]
fn test_boruvka_larger_8v() {
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
    ];
    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_labels.size(), 7);
    assert!(mst_w.val < 35.0);
}

#[test]
fn test_boruvka_deterministic_across_seeds() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(2.0), 1),
        LabeledEdge(3, 4, w(3.0), 2),
        LabeledEdge(4, 1, w(4.0), 3),
    ];
    let w1 = BoruvkaStEph::mst_weight(&edges, &BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 1));
    let w2 = BoruvkaStEph::mst_weight(&edges, &BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 99));
    assert_eq!(w1, w2);
}


#[test]
fn test_boruvka_larger_graph() {
    // 8 vertices with ring + cross edges.
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

    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 7); // n-1 edges
    assert!(mst_w <= w(28.0)); // 1+2+3+4+5+6+7 = 28
}


#[test]
fn test_boruvka_mst_weight_empty() {
    let edges: SetStEph<LabeledEdge<i32>> = SetLit![];
    let mst_labels: SetStEph<usize> = SetLit![];
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);
    assert_eq!(mst_w, w(0.0));
}


#[test]
fn test_boruvka_parallel_edges_different_weights() {
    // Two alternative paths between same components.
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, w(1.0), 0),
        LabeledEdge(2, 3, w(10.0), 1),
        LabeledEdge(1, 3, w(2.0), 2),
    ];

    let mst_labels = BoruvkaStEph::boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = BoruvkaStEph::mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, w(3.0)); // 1.0 + 2.0 = 3.0 (cheapest spanning)
}


#[test]
fn test_vertex_bridges_single_edge() {
    let edges = SetLit![LabeledEdge(1, 2, w(7.0), 0)];
    let bridges = BoruvkaStEph::vertex_bridges(&edges);
    assert_eq!(bridges.inner.len(), 2);
    assert_eq!(bridges.inner.get(&1), Some(&(2, w(7.0), 0)));
    assert_eq!(bridges.inner.get(&2), Some(&(1, w(7.0), 0)));
}
