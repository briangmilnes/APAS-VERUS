//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Tests for DijkstraStEphF64.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::SSSPResultStEphF64::SSSPResultStEphF64::*;
use apas_verus::Chap57::DijkstraStEphF64::DijkstraStEphF64::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::vstdplus::float::float::*;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

#[test]
fn test_example_57_1() {
    // Example 57.1 from textbook: s -> a (weight 1), s -> b (weight 3), a -> b (weight 1)
    let mut vertices = SetStEph::empty();
    for v in 0..3 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(WeightedEdge(0, 1, w(1.0))); // s=0 -> a=1 weight 1
    edges.insert(WeightedEdge(0, 2, w(3.0))); // s=0 -> b=2 weight 3
    edges.insert(WeightedEdge(1, 2, w(1.0))); // a=1 -> b=2 weight 1

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 1.0);
    assert_eq!(result.get_distance(2).val, 2.0);
}

#[test]
fn test_example_57_3() {
    // Example 57.3 from textbook
    let mut vertices = SetStEph::empty();
    for v in 0..6 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(WeightedEdge(0, 1, w(1.0)));
    edges.insert(WeightedEdge(0, 2, w(5.0)));
    edges.insert(WeightedEdge(1, 2, w(2.0)));
    edges.insert(WeightedEdge(1, 3, w(12.0)));
    edges.insert(WeightedEdge(2, 3, w(2.0)));
    edges.insert(WeightedEdge(2, 4, w(3.0)));
    edges.insert(WeightedEdge(3, 4, w(1.0)));

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 1.0);
    assert_eq!(result.get_distance(2).val, 3.0);
    assert_eq!(result.get_distance(3).val, 5.0);
    assert_eq!(result.get_distance(4).val, 6.0);
    assert!(!result.is_reachable(5)); // vertex 5 unreachable
}

#[test]
fn test_single_vertex() {
    let mut vertices = SetStEph::empty();
    vertices.insert(0);

    let edges = SetStEph::empty();
    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0).val, 0.0);
}

#[test]
fn test_unreachable_vertices() {
    let mut vertices = SetStEph::empty();
    for v in 0..3 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(WeightedEdge(0, 1, w(1.0)));

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 1.0);
    assert!(!result.is_reachable(2));
}

#[test]
fn test_path_extraction() {
    let mut vertices = SetStEph::empty();
    for v in 0..4 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(WeightedEdge(0, 1, w(1.0)));
    edges.insert(WeightedEdge(1, 2, w(2.0)));
    edges.insert(WeightedEdge(2, 3, w(3.0)));

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
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
    let mut vertices = SetStEph::empty();
    for v in 0..4 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    edges.insert(WeightedEdge(0, 1, w(5.0)));
    edges.insert(WeightedEdge(0, 2, w(3.0)));
    edges.insert(WeightedEdge(2, 1, w(2.0)));
    edges.insert(WeightedEdge(1, 3, w(1.0)));

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(1).val, 5.0);
    assert_eq!(result.get_distance(3).val, 6.0);
}

#[test]
fn test_larger_graph() {
    let mut vertices = SetStEph::empty();
    for v in 0..10 {
        vertices.insert(v);
    }

    let mut edges = SetStEph::empty();
    for i in 0..9usize {
        edges.insert(WeightedEdge(i, i + 1, w(1.0)));
    }
    edges.insert(WeightedEdge(0, 5, w(3.0)));
    edges.insert(WeightedEdge(2, 7, w(4.0)));

    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);

    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(5).val, 3.0);
    assert_eq!(result.get_distance(9).val, 7.0);
}

#[test]
fn test_diamond_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(0, 2, w(4.0)),
        WeightedEdge(1, 3, w(2.0)),
        WeightedEdge(2, 3, w(1.0))
    ];
    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);
    assert_eq!(result.get_distance(3).val, 3.0);
}

#[test]
fn test_fractional_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        WeightedEdge(0, 1, w(0.5)),
        WeightedEdge(1, 2, w(1.25))
    ];
    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);
    assert_eq!(result.get_distance(1).val, 0.5);
    assert_eq!(result.get_distance(2).val, 1.75);
}

#[test]
fn test_zero_weight_edges() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        WeightedEdge(0, 1, w(0.0)),
        WeightedEdge(1, 2, w(0.0))
    ];
    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);
    assert_eq!(result.get_distance(1).val, 0.0);
    assert_eq!(result.get_distance(2).val, 0.0);
}

#[test]
fn test_all_unreachable() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![];
    let graph = <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges);
    let result = dijkstra(&graph, 0);
    assert!(!result.is_reachable(1));
    assert!(!result.is_reachable(2));
}
