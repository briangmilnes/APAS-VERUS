// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Johnson's APSP algorithm — multi-threaded (F64 weights).

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
use apas_verus::Chap56::AllPairsResultStEphF64::AllPairsResultStEphF64::*;
use apas_verus::Chap59::JohnsonMtEphF64::JohnsonMtEphF64::*;
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
fn test_mt_simple_graph() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(5.5)),
        WeightedEdge(1, 2, w(3.2)),
        WeightedEdge(0, 2, w(10.0))
    ]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 0).val, 0.0);
    assert_eq!(result.get_distance(0, 1).val, 5.5);
    assert!((result.get_distance(0, 2).val - 8.7).abs() < 1e-10);
    assert_eq!(result.get_distance(1, 2).val, 3.2);
    assert!(!result.is_reachable(1, 0));
}

#[test]
fn test_mt_negative_weights() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(1.5)),
        WeightedEdge(1, 2, w(-0.8)),
        WeightedEdge(0, 2, w(1.0))
    ]);
    let result = johnson_apsp(&graph);
    assert!((result.get_distance(0, 2).val - 0.7).abs() < 1e-10);
}

#[test]
fn test_mt_single_vertex() {
    let graph = mk_graph(1, SetLit![]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 0).val, 0.0);
}

#[test]
fn test_mt_disconnected() {
    let graph = mk_graph(4, SetLit![
        WeightedEdge(0, 1, w(2.5)),
        WeightedEdge(2, 3, w(1.8))
    ]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 1).val, 2.5);
    assert_eq!(result.get_distance(2, 3).val, 1.8);
    assert!(!result.is_reachable(0, 2));
}

#[test]
fn test_mt_diamond() {
    let graph = mk_graph(4, SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(0, 2, w(4.0)),
        WeightedEdge(1, 3, w(2.0)),
        WeightedEdge(2, 3, w(1.0))
    ]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 3).val, 3.0);
}

#[test]
fn test_mt_cycle() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(1.0)),
        WeightedEdge(1, 2, w(1.0)),
        WeightedEdge(2, 0, w(1.0))
    ]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 2).val, 2.0);
    assert_eq!(result.get_distance(1, 0).val, 2.0);
}

#[test]
fn test_mt_zero_weights() {
    let graph = mk_graph(3, SetLit![
        WeightedEdge(0, 1, w(0.0)),
        WeightedEdge(1, 2, w(0.0))
    ]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 2).val, 0.0);
}

#[test]
fn test_mt_two_vertices() {
    let graph = mk_graph(2, SetLit![WeightedEdge(0, 1, w(7.0))]);
    let result = johnson_apsp(&graph);
    assert_eq!(result.get_distance(0, 1).val, 7.0);
    assert!(!result.is_reachable(1, 0));
}
