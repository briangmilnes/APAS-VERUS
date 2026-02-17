#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Tests for DijkstraStEphInt

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap57::DijkstraStEphInt::DijkstraStEphInt::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_57_1() {
    // Example 57.1 from textbook: Graph where BFS fails but Dijkstra succeeds
    // s -> a (weight 1), s -> b (weight 2), a -> b (weight 1)
    // Shortest path to b is s -> a -> b (weight 2), not s -> b (weight 2)
    // Actually both have same weight, but example shows BFS might visit b first

    let mut vertices = SetStEph::empty();
    for v in 0..3 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, 1)); // s=0 -> a=1 weight 1
    edges.insert(Triple(0, 2, 3)); // s=0 -> b=2 weight 3
    edges.insert(Triple(1, 2, 1)); // a=1 -> b=2 weight 1

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), 0); // source
    assert_eq!(result.get_distance(1), 1); // a
    assert_eq!(result.get_distance(2), 2); // b via a
}

#[test]
fn test_example_57_3() {
    // Example 57.3 from textbook
    // Graph: s=0, a=1, b=2, c=3, d=4, e=5
    // s -> a (1), s -> b (5)
    // a -> b (2), a -> c (12)
    // b -> c (2), b -> d (3)
    // c -> d (1)
    // d (no outgoing)
    // e (isolated vertex)

    let mut vertices = SetStEph::empty();
    for v in 0..6 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, 1)); // s -> a
    edges.insert(Triple(0, 2, 5)); // s -> b
    edges.insert(Triple(1, 2, 2)); // a -> b
    edges.insert(Triple(1, 3, 12)); // a -> c
    edges.insert(Triple(2, 3, 2)); // b -> c
    edges.insert(Triple(2, 4, 3)); // b -> d
    edges.insert(Triple(3, 4, 1)); // c -> d

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 1);
    assert_eq!(result.get_distance(2), 3); // via a
    assert_eq!(result.get_distance(3), 5); // via a->b->c
    assert_eq!(result.get_distance(4), 6); // via a->b->c->d or a->b->d
    assert_eq!(result.get_distance(5), i64::MAX); // unreachable
}

#[test]
fn test_single_vertex() {
    let mut vertices = SetStEph::empty();
    vertices.insert(0);

    let edges = SetStEph::empty();
    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), 0);
}

#[test]
fn test_unreachable_vertices() {
    let mut vertices = SetStEph::empty();
    for v in 0..3 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, 1)); // s -> a
    // vertex 2 is unreachable

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 1);
    assert_eq!(result.get_distance(2), i64::MAX);
    assert!(!result.is_reachable(2));
}

#[test]
fn test_path_extraction() {
    let mut vertices = SetStEph::empty();
    for v in 0..4 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, 1));
    edges.insert(Triple(1, 2, 2));
    edges.insert(Triple(2, 3, 3));

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    let path = result.extract_path(3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_multiple_paths_same_weight() {
    // Multiple paths with same total weight
    let mut vertices = SetStEph::empty();
    for v in 0..4 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(Triple(0, 1, 5));
    edges.insert(Triple(0, 2, 3));
    edges.insert(Triple(2, 1, 2));
    edges.insert(Triple(1, 3, 1));

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(1), 5); // Both paths have weight 5
    assert_eq!(result.get_distance(3), 6);
}

#[test]
fn test_larger_graph() {
    // Larger test graph
    let mut vertices = SetStEph::empty();
    for v in 0..10 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    for i in 0..9 {
        edges.insert(Triple(i, i + 1, 1));
    }
    // Add some shortcuts
    edges.insert(Triple(0, 5, 3));
    edges.insert(Triple(2, 7, 4));

    let graph = WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(5), 3); // via shortcut 0->5
    assert_eq!(result.get_distance(9), 7); // via shortcut 0->5, then 5->6->7->8->9
}
