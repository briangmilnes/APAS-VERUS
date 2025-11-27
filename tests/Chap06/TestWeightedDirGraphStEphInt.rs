//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for WeightedDirGraphStEphInt.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
use apas_verus::Types::Types::*;
use apas_verus::{SetLit, WeightedDirGraphStEphIntLit};

#[test]
fn test_from_weighted_edges() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(2, 3, 20)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_add_weighted_edge() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![];
    let mut g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    g.add_weighted_edge(1, 2, 15);
    assert_eq!(g.labeled_arcs().size(), 1);

    g.add_weighted_edge(2, 3, 25);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(2, 3, 20)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(10));
    assert_eq!(g.get_edge_weight(&2, &3), Some(20));
    assert_eq!(g.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighted_edges() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(2, 3, 20)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges.clone());

    let result = g.weighted_edges();
    assert_eq!(result.size(), 2);
    assert!(result.mem(&Triple(1, 2, 10)));
    assert!(result.mem(&Triple(2, 3, 20)));
}

#[test]
fn test_out_neighbors_weighted() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(1, 3, 15), Triple(2, 4, 20)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let out1 = g.out_neighbors_weighted(&1);
    assert_eq!(out1.size(), 2);
    assert!(out1.mem(&Pair(2, 10)));
    assert!(out1.mem(&Pair(3, 15)));

    let out2 = g.out_neighbors_weighted(&2);
    assert_eq!(out2.size(), 1);
    assert!(out2.mem(&Pair(4, 20)));
}

#[test]
fn test_in_neighbors_weighted() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(3, 2, 15), Triple(2, 4, 20)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let in2 = g.in_neighbors_weighted(&2);
    assert_eq!(in2.size(), 2);
    assert!(in2.mem(&Pair(1, 10)));
    assert!(in2.mem(&Pair(3, 15)));

    let in4 = g.in_neighbors_weighted(&4);
    assert_eq!(in4.size(), 1);
    assert!(in4.mem(&Pair(2, 20)));
}

#[test]
fn test_total_weight() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10), Triple(2, 3, 20), Triple(1, 3, 5)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.total_weight(), 35);
}

#[test]
fn test_total_weight_empty() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.total_weight(), 0);
}

#[test]
fn test_edges_above_weight() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<Triple<N, N, i32>> =
        SetLit![Triple(1, 2, 10), Triple(2, 3, 25), Triple(3, 4, 5), Triple(1, 4, 30)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let above15 = g.edges_above_weight(15);
    assert_eq!(above15.size(), 2);
    assert!(above15.mem(&Triple(2, 3, 25)));
    assert!(above15.mem(&Triple(1, 4, 30)));
}

#[test]
fn test_edges_below_weight() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<Triple<N, N, i32>> =
        SetLit![Triple(1, 2, 10), Triple(2, 3, 25), Triple(3, 4, 5), Triple(1, 4, 30)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let below15 = g.edges_below_weight(15);
    assert_eq!(below15.size(), 2);
    assert!(below15.mem(&Triple(1, 2, 10)));
    assert!(below15.mem(&Triple(3, 4, 5)));
}

#[test]
fn test_negative_weights() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, -10), Triple(2, 3, 20), Triple(1, 3, -5)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(-10));
    assert_eq!(g.total_weight(), 5);
}

#[test]
fn test_zero_weight() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 0)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(0));
    assert_eq!(g.total_weight(), 0);
}

#[test]
fn test_isolated_vertex() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 10)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let out3 = g.out_neighbors_weighted(&3);
    assert_eq!(out3.size(), 0);

    let in3 = g.in_neighbors_weighted(&3);
    assert_eq!(in3.size(), 0);
}

#[test]
fn test_self_loop() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 1, 5), Triple(1, 2, 10)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &1), Some(5));
    let out1 = g.out_neighbors_weighted(&1);
    assert_eq!(out1.size(), 2);
}

#[test]
fn test_complete_graph_weighted() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![
        Triple(1, 2, 10),
        Triple(1, 3, 11),
        Triple(2, 1, 20),
        Triple(2, 3, 21),
        Triple(3, 1, 30),
        Triple(3, 2, 31)
    ];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.labeled_arcs().size(), 6);
    assert_eq!(g.total_weight(), 123);
}

#[test]
fn test_large_weights() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, i32::MAX), Triple(2, 1, i32::MIN)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(i32::MAX));
    assert_eq!(g.get_edge_weight(&2, &1), Some(i32::MIN));
}

#[test]
fn test_empty_graph() {
    let v: SetStEph<N> = SetLit![];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_arcs().size(), 0);
    assert_eq!(g.total_weight(), 0);
}

#[test]
fn test_no_edges_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 0);
    assert_eq!(g.total_weight(), 0);
}

#[test]
fn test_weighted_path() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(1, 2, 5), Triple(2, 3, 10), Triple(3, 4, 15)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    assert_eq!(g.total_weight(), 30);
    assert_eq!(g.out_neighbors_weighted(&1).size(), 1);
    assert_eq!(g.out_neighbors_weighted(&4).size(), 0);
}

#[test]
fn test_weighted_star() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let edges: SetStEph<Triple<N, N, i32>> = SetLit![Triple(0, 1, 10), Triple(0, 2, 20), Triple(0, 3, 30)];
    let g = WeightedDirGraphStEphInt::from_weighted_edges(v, edges);

    let out0 = g.out_neighbors_weighted(&0);
    assert_eq!(out0.size(), 3);
    assert_eq!(g.total_weight(), 60);
}

#[test]
fn test_weighteddirgraphstephintlit_macro_empty() {
    let g: WeightedDirGraphStEphInt<N> = WeightedDirGraphStEphIntLit!();
    assert_eq!(g.vertices().size(), 0);
}

#[test]
fn test_weighteddirgraphstephintlit_macro_simple() {
    let g = WeightedDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 10), Triple(2, 3, 20)]
    );
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.arcs().size(), 2);
    assert_eq!(g.total_weight(), 30);
}

#[test]
fn test_weighteddirgraphstephintlit_macro_star() {
    let g = WeightedDirGraphStEphIntLit!(
        V: [0, 1, 2, 3],
        E: [Triple(0, 1, 5), Triple(0, 2, 10), Triple(0, 3, 15)]
    );
    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.out_neighbors_weighted(&0).size(), 3);
    assert_eq!(g.total_weight(), 30);
}

#[test]
fn test_weighteddirgraphstephintlit_macro_trailing_comma() {
    let g = WeightedDirGraphStEphIntLit!(
        V: [1, 2, 3,],
        E: [Triple(1, 2, 100), Triple(2, 3, 200),]
    );
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.total_weight(), 300);
}
