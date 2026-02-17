#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
use apas_verus::Chap59::JohnsonStEphInt::JohnsonStEphInt::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

#[test]
fn test_example_59_1() {
    // Example 59.1 from textbook
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        Triple(0, 1, 3),  // s -> a: 3
        Triple(0, 2, 8),  // s -> b: 8
        Triple(1, 2, -2), // a -> b: -2
        Triple(1, 3, 1),  // a -> c: 1
        Triple(2, 0, 4),  // b -> s: 4
        Triple(2, 3, 7),  // b -> c: 7
        Triple(3, 1, 2)   // c -> a: 2
    ];

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    // Verify distances from Example 59.1
    assert_eq!(result.get_distance(0, 0), 0);
    assert_eq!(result.get_distance(0, 1), 3);
    assert_eq!(result.get_distance(0, 2), 1); // s -> a -> b = 3 + (-2) = 1
    assert_eq!(result.get_distance(0, 3), 4); // s -> a -> c = 3 + 1 = 4

    assert_eq!(result.get_distance(1, 0), 2); // a -> b -> s = -2 + 4 = 2
    assert_eq!(result.get_distance(1, 1), 0);
    assert_eq!(result.get_distance(1, 2), -2); // a -> b
    assert_eq!(result.get_distance(1, 3), 1); // a -> c

    assert_eq!(result.get_distance(2, 0), 4); // b -> s
    assert_eq!(result.get_distance(2, 1), 7); // b -> s -> a = 4 + 3 = 7
    assert_eq!(result.get_distance(2, 2), 0);
    assert_eq!(result.get_distance(2, 3), 7); // b -> c
}

#[test]
fn test_simple_graph() {
    // Simple 3-vertex graph
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![Triple(0, 1, 5), Triple(1, 2, 3), Triple(0, 2, 10)];

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), 0);
    assert_eq!(result.get_distance(0, 1), 5);
    assert_eq!(result.get_distance(0, 2), 8); // via 1: 5 + 3 = 8 < 10

    assert_eq!(result.get_distance(1, 1), 0);
    assert_eq!(result.get_distance(1, 2), 3);
    assert_eq!(result.get_distance(1, 0), i64::MAX); // unreachable

    assert_eq!(result.get_distance(2, 2), 0);
    assert_eq!(result.get_distance(2, 0), i64::MAX); // unreachable
    assert_eq!(result.get_distance(2, 1), i64::MAX); // unreachable
}

#[test]
fn test_negative_weights() {
    // Graph with negative weights but no negative cycles
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![Triple(0, 1, 1), Triple(1, 2, -5), Triple(0, 2, 3)];

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), -4); // via 1: 1 + (-5) = -4 < 3
}

#[test]
fn test_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetStEph::empty();

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), 0);
}

#[test]
fn test_disconnected_graph() {
    // Two disconnected components
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![Triple(0, 1, 5), Triple(2, 3, 3)];

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    // Within components
    assert_eq!(result.get_distance(0, 1), 5);
    assert_eq!(result.get_distance(2, 3), 3);

    // Between components
    assert_eq!(result.get_distance(0, 2), i64::MAX);
    assert_eq!(result.get_distance(0, 3), i64::MAX);
    assert_eq!(result.get_distance(1, 2), i64::MAX);
}

#[test]
fn test_negative_cycle() {
    // Create graph with negative cycle: 0 -> 1 -> 2 -> 0 with total weight < 0
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, 1),
        Triple(1, 2, -2),
        Triple(2, 0, -1)  // Total cycle: 1 + (-2) + (-1) = -2 < 0
    ];

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    // When negative cycle exists, all distances should be i64::MAX
    assert_eq!(result.get_distance(0, 0), i64::MAX);
    assert_eq!(result.get_distance(0, 1), i64::MAX);
    assert_eq!(result.get_distance(1, 2), i64::MAX);
}
