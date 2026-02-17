#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for DijkstraStEphFloat.

use ordered_float::OrderedFloat;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
use apas_verus::Chap57::DijkstraStEphFloat::DijkstraStEphFloat::*;
use apas_verus::Types::Types::*;

#[test]
fn test_simple_path() {
    let mut vertices = SetStEph::empty();
    for v in 0..3 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, OrderedFloat(1.0)));
    edges.insert(Triple(0, 2, OrderedFloat(3.0)));
    edges.insert(Triple(1, 2, OrderedFloat(1.0)));

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), OrderedFloat(0.0));
    assert_eq!(result.get_distance(1), OrderedFloat(1.0));
    assert_eq!(result.get_distance(2), OrderedFloat(2.0));
}

#[test]
fn test_complex_graph() {
    let mut vertices = SetStEph::empty();
    for v in 0..6 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, OrderedFloat(1.0)));
    edges.insert(Triple(0, 2, OrderedFloat(5.0)));
    edges.insert(Triple(1, 2, OrderedFloat(2.0)));
    edges.insert(Triple(1, 3, OrderedFloat(12.0)));
    edges.insert(Triple(2, 3, OrderedFloat(2.0)));
    edges.insert(Triple(2, 4, OrderedFloat(3.0)));
    edges.insert(Triple(3, 4, OrderedFloat(1.0)));

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), OrderedFloat(0.0));
    assert_eq!(result.get_distance(1), OrderedFloat(1.0));
    assert_eq!(result.get_distance(2), OrderedFloat(3.0));
    assert_eq!(result.get_distance(3), OrderedFloat(5.0));
    assert_eq!(result.get_distance(4), OrderedFloat(6.0));
    assert_eq!(result.get_distance(5), OrderedFloat(f64::INFINITY));
}

#[test]
fn test_single_vertex() {
    let mut vertices = SetStEph::empty();
    vertices.insert(0);

    let edges = SetStEph::empty();

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), OrderedFloat(0.0));
}

#[test]
fn test_disconnected_graph() {
    let mut vertices = SetStEph::empty();
    for v in 0..4 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, OrderedFloat(1.0)));
    edges.insert(Triple(2, 3, OrderedFloat(1.0)));

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), OrderedFloat(0.0));
    assert_eq!(result.get_distance(1), OrderedFloat(1.0));
    assert_eq!(result.get_distance(2), OrderedFloat(f64::INFINITY));
    assert_eq!(result.get_distance(3), OrderedFloat(f64::INFINITY));
}
