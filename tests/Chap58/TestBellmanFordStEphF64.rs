//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Tests for Bellman-Ford Algorithm (Float Weights)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
use apas_verus::Chap58::BellmanFordStEphF64::BellmanFordStEphF64::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::vstdplus::float::float::*;

fn w(v: f64) -> WrappedF64 {
    WrappedF64 { val: v }
}

fn mk_graph(n: usize, edges: SetStEph<WeightedEdge<usize, WrappedF64>>) -> WeightedDirGraphStEphF64<usize> {
    let mut vertices = SetStEph::empty();
    for v in 0..n {
        vertices.insert(v);
    }
    <WeightedDirGraphStEphF64<usize> as WeightedDirGraphStEphF64Trait<usize>>::from_weighed_edges(vertices, edges)
}

#[test]
fn test_basic_path() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(1.5)),
        WeightedEdge(1, 2, w(2.5))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 1.5);
    assert_eq!(result.get_distance(2).val, 4.0);
}

#[test]
fn test_negative_edges() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(5.0)),
        WeightedEdge(0, 2, w(2.0)),
        WeightedEdge(1, 2, w(-4.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 5.0);
    assert_eq!(result.get_distance(2).val, 1.0);
}

#[test]
fn test_negative_cycle() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(1, 2, w(2.0)),
        WeightedEdge(2, 1, w(-4.0))
    ]);
    let result = bellman_ford(&graph, 0);
    assert!(result.is_err());
}

#[test]
fn test_fractional_weights() {
    let graph = mk_graph(4, SetLit![
        WeightedEdge(0, 1, w(0.5)),
        WeightedEdge(1, 2, w(1.25)),
        WeightedEdge(2, 3, w(-0.75))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 0.5);
    assert_eq!(result.get_distance(2).val, 1.75);
    assert_eq!(result.get_distance(3).val, 1.0);
}

#[test]
fn test_unreachable() {
    let graph = mk_graph(3, SetLit![WeightedEdge(0, 1, w(1.0))]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0).val, 0.0);
    assert_eq!(result.get_distance(1).val, 1.0);
    assert!(!result.is_reachable(2));
}

#[test]
fn test_single_vertex() {
    let graph = mk_graph(1, SetLit![]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0).val, 0.0);
}

#[test]
fn test_zero_weight_edges() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(0.0)),
        WeightedEdge(1, 2, w(0.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(1).val, 0.0);
    assert_eq!(result.get_distance(2).val, 0.0);
}

#[test]
fn test_diamond_graph() {
    // 0 -> 1 (1), 0 -> 2 (4), 1 -> 3 (2), 2 -> 3 (1)
    let graph = mk_graph(4, SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(0, 2, w(4.0)),
        WeightedEdge(1, 3, w(2.0)),
        WeightedEdge(2, 3, w(1.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(3).val, 3.0); // via 0->1->3
}

#[test]
fn test_path_extraction_f64() {
    let graph = mk_graph(4, SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(1, 2, w(2.0)),
        WeightedEdge(2, 3, w(3.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    let path = result.extract_path(3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
}

#[test]
fn test_two_negative_edges_no_cycle() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(-1.0)),
        WeightedEdge(1, 2, w(-2.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(1).val, -1.0);
    assert_eq!(result.get_distance(2).val, -3.0);
}

#[test]
fn test_longer_path_cheaper() {
    // Direct: 0->2 costs 10. Indirect: 0->1->2 costs 3.
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 2, w(10.0)),
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(1, 2, w(2.0))
    ]);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(2).val, 3.0);
}
