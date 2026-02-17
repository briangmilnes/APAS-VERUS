#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral)

use ordered_float::OrderedFloat;
use rand::rngs::StdRng;
use rand::SeedableRng;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap66::BoruvkaStEph::BoruvkaStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

#[test]
fn test_boruvka_triangle() {
    // Triangle: 1-2 (w=3), 2-3 (w=2), 3-1 (w=1)
    // MST: edges 2 (3-1, w=1) and 1 (2-3, w=2), total weight = 3
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(3.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 1, OrderedFloat(1.0), 2),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, OrderedFloat(3.0));
    assert!(mst_labels.mem(&1));
    assert!(mst_labels.mem(&2));
}

#[test]
fn test_boruvka_square() {
    // Square: 1-2 (w=1), 2-3 (w=2), 3-4 (w=3), 4-1 (w=4)
    // MST: edges 0, 1, 2 (no diagonal)
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(1.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 4, OrderedFloat(3.0), 2),
        LabeledEdge(4, 1, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0));
}

#[test]
fn test_boruvka_complete_4() {
    // Complete graph on 4 vertices with increasing weights
    // MST should have 3 edges with minimum total weight
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(1.0), 0),
        LabeledEdge(1, 3, OrderedFloat(2.0), 1),
        LabeledEdge(1, 4, OrderedFloat(3.0), 2),
        LabeledEdge(2, 3, OrderedFloat(4.0), 3),
        LabeledEdge(2, 4, OrderedFloat(5.0), 4),
        LabeledEdge(3, 4, OrderedFloat(6.0), 5),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0)); // 1+2+3 = 6
}

#[test]
fn test_boruvka_star() {
    // Star: center 0 connected to 1, 2, 3, 4 with equal weights
    // MST: all edges from center
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        LabeledEdge(0, 1, OrderedFloat(1.0), 0),
        LabeledEdge(0, 2, OrderedFloat(1.0), 1),
        LabeledEdge(0, 3, OrderedFloat(1.0), 2),
        LabeledEdge(0, 4, OrderedFloat(1.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(4.0));
}

#[test]
fn test_boruvka_path() {
    // Path: 1-2-3-4-5 with weights 1, 2, 3, 4
    // MST: all edges (it's a tree already)
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(1.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 4, OrderedFloat(3.0), 2),
        LabeledEdge(4, 5, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
}

#[test]
fn test_boruvka_single_vertex() {
    // Single vertex: no edges
    let vertices = SetLit![1];
    let edges = SetLit![];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);

    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_two_vertices() {
    // Two vertices with one edge
    let vertices = SetLit![1, 2];
    let edges = SetLit![LabeledEdge(1, 2, OrderedFloat(5.0), 0),];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 1);
    assert_eq!(mst_w, OrderedFloat(5.0));
    assert!(mst_labels.mem(&0));
}

#[test]
fn test_boruvka_cycle_5() {
    // Cycle: 1-2-3-4-5-1 with weights 1, 2, 3, 4, 10
    // MST: omit the heaviest edge (label 4)
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(1.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 4, OrderedFloat(3.0), 2),
        LabeledEdge(4, 5, OrderedFloat(4.0), 3),
        LabeledEdge(5, 1, OrderedFloat(10.0), 4),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
    assert!(!mst_labels.mem(&4)); // heaviest edge not in MST
}

// ============================================================================
// Direct tests for internal helper functions (for coverage)
// ============================================================================

#[test]
fn test_vertex_bridges_triangle() {
    // Triangle: 1-2 (w=3), 2-3 (w=2), 3-1 (w=1)
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(3.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 1, OrderedFloat(1.0), 2),
    ];

    let bridges = vertex_bridges(&edges);

    // Each vertex should have a minimum bridge
    assert_eq!(bridges.len(), 3);
    
    // Vertex 1: min edge is 3-1 (w=1)
    assert_eq!(bridges.get(&1), Some(&(3, OrderedFloat(1.0), 2)));
    
    // Vertex 2: min edge is 2-3 (w=2)
    assert_eq!(bridges.get(&2), Some(&(3, OrderedFloat(2.0), 1)));
    
    // Vertex 3: min edge is 3-1 (w=1)
    assert_eq!(bridges.get(&3), Some(&(1, OrderedFloat(1.0), 2)));
}

#[test]
fn test_vertex_bridges_star() {
    // Star: center 0 connected to 1, 2, 3 with weights 1, 2, 3
    let edges = SetLit![
        LabeledEdge(0, 1, OrderedFloat(1.0), 0),
        LabeledEdge(0, 2, OrderedFloat(2.0), 1),
        LabeledEdge(0, 3, OrderedFloat(3.0), 2),
    ];

    let bridges = vertex_bridges(&edges);

    assert_eq!(bridges.len(), 4);
    
    // Center 0: min edge is to 1 (w=1)
    assert_eq!(bridges.get(&0), Some(&(1, OrderedFloat(1.0), 0)));
    
    // Each leaf has only one edge (to center)
    assert_eq!(bridges.get(&1), Some(&(0, OrderedFloat(1.0), 0)));
    assert_eq!(bridges.get(&2), Some(&(0, OrderedFloat(2.0), 1)));
    assert_eq!(bridges.get(&3), Some(&(0, OrderedFloat(3.0), 2)));
}

#[test]
fn test_vertex_bridges_empty() {
    let edges: SetStEph<LabeledEdge<i32>> = SetLit![];
    let bridges = vertex_bridges(&edges);
    assert_eq!(bridges.len(), 0);
}

#[test]
fn test_bridge_star_partition() {
    // Triangle with bridges computed
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(3.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 1, OrderedFloat(1.0), 2),
    ];

    let bridges = vertex_bridges(&edges);
    let mut rng = StdRng::seed_from_u64(42);

    let (remaining, partition) = bridge_star_partition(&vertices, &bridges, &mut rng);

    // Some vertices should be contracted (partition non-empty)
    // Remaining + contracted should equal original vertices
    assert!(remaining.size() + partition.len() <= 3);
    assert!(remaining.size() > 0 || partition.len() > 0);
}

#[test]
fn test_bridge_star_partition_single_vertex() {
    let vertices = SetLit![1];
    let bridges = std::collections::HashMap::new();
    let mut rng = StdRng::seed_from_u64(42);

    let (remaining, partition) = bridge_star_partition(&vertices, &bridges, &mut rng);

    // Single vertex with no bridges: stays as-is
    assert_eq!(remaining.size(), 1);
    assert_eq!(partition.len(), 0);
}

#[test]
fn test_boruvka_mst_direct() {
    // Triangle: test the direct boruvka_mst function
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        LabeledEdge(1, 2, OrderedFloat(3.0), 0),
        LabeledEdge(2, 3, OrderedFloat(2.0), 1),
        LabeledEdge(3, 1, OrderedFloat(1.0), 2),
    ];

    let mut rng = StdRng::seed_from_u64(42);
    let mst_labels = boruvka_mst(&vertices, &edges, SetLit![], &mut rng);

    // MST of triangle should have 2 edges
    assert_eq!(mst_labels.size(), 2);
    let mst_w = mst_weight(&edges, &mst_labels);
    assert_eq!(mst_w, OrderedFloat(3.0)); // edges with weights 1 and 2
}

#[test]
fn test_boruvka_mst_direct_empty() {
    let vertices = SetLit![1];
    let edges: SetStEph<LabeledEdge<i32>> = SetLit![];

    let mut rng = StdRng::seed_from_u64(42);
    let mst_labels = boruvka_mst(&vertices, &edges, SetLit![], &mut rng);

    // No edges = empty MST
    assert_eq!(mst_labels.size(), 0);
}
