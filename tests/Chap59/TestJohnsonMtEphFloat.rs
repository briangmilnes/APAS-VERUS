#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
use apas_verus::Chap59::JohnsonMtEphFloat::JohnsonMtEphFloat::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

#[test]
fn test_simple_graph() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(5.5)),
        Triple(1, 2, OrderedF64::from(3.2)),
        Triple(0, 2, OrderedF64::from(10.0))
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(5.5));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(8.7));

    assert_eq!(result.get_distance(1, 1), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(1, 2), OrderedF64::from(3.2));
    assert_eq!(result.get_distance(1, 0), OrderedF64::from(f64::INFINITY));
}

#[test]
fn test_negative_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(1.5)),
        Triple(1, 2, OrderedF64::from(-0.8)),
        Triple(0, 2, OrderedF64::from(1.0))
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.7));
}

#[test]
fn test_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetStEph::empty();

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
}

#[test]
fn test_fractional_weights() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(0.5)),
        Triple(0, 2, OrderedF64::from(1.5)),
        Triple(1, 2, OrderedF64::from(-0.25)),
        Triple(1, 3, OrderedF64::from(1.0)),
        Triple(2, 3, OrderedF64::from(0.5))
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(0.5));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.25));
    assert_eq!(result.get_distance(0, 3), OrderedF64::from(0.75));
}

#[test]
fn test_disconnected_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![Triple(0, 1, OrderedF64::from(2.5)), Triple(2, 3, OrderedF64::from(1.8))];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 1), OrderedF64::from(2.5));
    assert_eq!(result.get_distance(2, 3), OrderedF64::from(1.8));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(f64::INFINITY));
    assert_eq!(result.get_distance(1, 3), OrderedF64::from(f64::INFINITY));
}

#[test]
fn test_two_vertex_cycle() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Triple(0, 1, OrderedF64::from(1.0)), Triple(1, 0, OrderedF64::from(2.0))];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 1), OrderedF64::from(1.0));
    assert_eq!(result.get_distance(1, 0), OrderedF64::from(2.0));
}

#[test]
fn test_triangle() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(1.0)),
        Triple(1, 2, OrderedF64::from(1.0)),
        Triple(2, 0, OrderedF64::from(1.0))
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), OrderedF64::from(2.0));
    assert_eq!(result.get_distance(1, 0), OrderedF64::from(2.0));
}

#[test]
fn test_zero_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![Triple(0, 1, OrderedF64::from(0.0)), Triple(1, 2, OrderedF64::from(0.0))];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.0));
}

#[test]
fn test_large_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(1000.5)),
        Triple(1, 2, OrderedF64::from(2000.3))
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), OrderedF64::from(3000.8));
}

#[test]
fn test_self_loop() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Triple(0, 0, OrderedF64::from(1.0)), Triple(0, 1, OrderedF64::from(2.0))];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(2.0));
}

#[test]
fn test_negative_cycle() {
    // Create graph with negative cycle: 0 -> 1 -> 2 -> 0 with total weight < 0
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        Triple(0, 1, OrderedF64::from(1.0)),
        Triple(1, 2, OrderedF64::from(-2.0)),
        Triple(2, 0, OrderedF64::from(-1.0))  // Total cycle: 1 + (-2) + (-1) = -2 < 0
    ];

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    // When negative cycle exists, all distances should be infinity
    assert_eq!(result.get_distance(0, 0), OrderedF64::from(f64::INFINITY));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(f64::INFINITY));
    assert_eq!(result.get_distance(1, 2), OrderedF64::from(f64::INFINITY));
}

#[test]
fn test_empty_graph() {
    // Graph with no vertices triggers empty range base case
    let vertices = SetStEph::empty();
    let edges = SetStEph::empty();

    let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.n, 0);
}
