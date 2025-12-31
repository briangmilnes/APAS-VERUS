//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for WeightedDirGraphStEphU32.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
use apas_verus::Chap06::WeightedDirGraphStEphU32::WeightedDirGraphStEphU32::*;
use apas_verus::Types::Types::*;
use apas_verus::SetLit;

#[test]
fn test_from_weighed_edges() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10), WeightedEdge(2, 3, 20)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_add_weighed_edge() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![];
    let mut g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    g.add_weighed_edge(1, 2, 15);
    assert_eq!(g.labeled_arcs().size(), 1);

    g.add_weighed_edge(2, 3, 25);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10), WeightedEdge(2, 3, 20)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(10));
    assert_eq!(g.get_edge_weight(&2, &3), Some(20));
    assert_eq!(g.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighed_edges() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10), WeightedEdge(2, 3, 20)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges.clone());

    let result = g.weighed_edges();
    assert_eq!(result.size(), 2);
    assert!(result.mem(&WeightedEdge(1, 2, 10)));
    assert!(result.mem(&WeightedEdge(2, 3, 20)));
}

#[test]
fn test_out_neighbors_weighed() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10), WeightedEdge(1, 3, 15), WeightedEdge(2, 4, 20)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    let out1 = g.out_neighbors_weighed(&1);
    assert_eq!(out1.size(), 2);
    assert!(out1.mem(&Pair(2, 10)));
    assert!(out1.mem(&Pair(3, 15)));

    let out2 = g.out_neighbors_weighed(&2);
    assert_eq!(out2.size(), 1);
    assert!(out2.mem(&Pair(4, 20)));
}

#[test]
fn test_in_neighbors_weighed() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10), WeightedEdge(3, 2, 15), WeightedEdge(2, 4, 20)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    let in2 = g.in_neighbors_weighed(&2);
    assert_eq!(in2.size(), 2);
    assert!(in2.mem(&Pair(1, 10)));
    assert!(in2.mem(&Pair(3, 15)));

    let in4 = g.in_neighbors_weighed(&4);
    assert_eq!(in4.size(), 1);
    assert!(in4.mem(&Pair(2, 20)));
}

#[test]
fn test_zero_weight() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 0)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(0));
}

#[test]
fn test_isolated_vertex() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 10)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    let out3 = g.out_neighbors_weighed(&3);
    assert_eq!(out3.size(), 0);

    let in3 = g.in_neighbors_weighed(&3);
    assert_eq!(in3.size(), 0);
}

#[test]
fn test_self_loop() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 1, 5), WeightedEdge(1, 2, 10)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &1), Some(5));
    let out1 = g.out_neighbors_weighed(&1);
    assert_eq!(out1.size(), 2);
}

#[test]
fn test_complete_graph_weighted() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![
        WeightedEdge(1, 2, 10),
        WeightedEdge(1, 3, 11),
        WeightedEdge(2, 1, 20),
        WeightedEdge(2, 3, 21),
        WeightedEdge(3, 1, 30),
        WeightedEdge(3, 2, 31)
    ];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.labeled_arcs().size(), 6);
}

#[test]
fn test_empty_graph() {
    let v: SetStEph<N> = SetLit![];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_arcs().size(), 0);
}

#[test]
fn test_no_edges_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 0);
}

#[test]
fn test_weighted_path() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(1, 2, 5), WeightedEdge(2, 3, 10), WeightedEdge(3, 4, 15)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    assert_eq!(g.out_neighbors_weighed(&1).size(), 1);
    assert_eq!(g.out_neighbors_weighed(&4).size(), 0);
}

#[test]
fn test_weighted_star() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, u32>> = SetLit![WeightedEdge(0, 1, 10), WeightedEdge(0, 2, 20), WeightedEdge(0, 3, 30)];
    let g = WeightedDirGraphStEphU32::from_weighed_edges(v, edges);

    let out0 = g.out_neighbors_weighed(&0);
    assert_eq!(out0.size(), 3);
}
