//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for WeightedDirGraphStEphIsize.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
use apas_verus::Chap06::WeightedDirGraphStEphIsize::WeightedDirGraphStEphIsize::*;
use apas_verus::Types::Types::*;
use apas_verus::SetLit;

#[test]
fn test_from_weighed_edges() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, isize>> =
        SetLit![WeightedEdge(1, 2, 10), WeightedEdge(2, 3, 20)];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_add_weighed_edge() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, isize>> = SetLit![];
    let mut g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);

    g.add_weighed_edge(1, 2, 15);
    assert_eq!(g.labeled_arcs().size(), 1);

    g.add_weighed_edge(2, 3, 25);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_empty_graph() {
    let v: SetStEph<N> = SetLit![];
    let edges: SetStEph<WeightedEdge<N, isize>> = SetLit![];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_arcs().size(), 0);
}

#[test]
fn test_self_loop() {
    let v: SetStEph<N> = SetLit![1];
    let edges: SetStEph<WeightedEdge<N, isize>> = SetLit![WeightedEdge(1, 1, 5)];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    assert_eq!(g.vertices().size(), 1);
    assert_eq!(g.labeled_arcs().size(), 1);
}

#[test]
fn test_max_weight() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<WeightedEdge<N, isize>> = SetLit![WeightedEdge(1, 2, 1000000)];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    assert_eq!(g.labeled_arcs().size(), 1);
}

#[test]
fn test_clone() {
    let v: SetStEph<N> = SetLit![1, 2];
    let edges: SetStEph<WeightedEdge<N, isize>> = SetLit![WeightedEdge(1, 2, 42)];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    let g2 = g.clone();
    assert_eq!(g.vertices().size(), g2.vertices().size());
    assert_eq!(g.labeled_arcs().size(), g2.labeled_arcs().size());
}

#[test]
fn test_multiple_edges_same_source() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let edges: SetStEph<WeightedEdge<N, isize>> =
        SetLit![WeightedEdge(1, 2, 10), WeightedEdge(1, 3, 20)];
    let g = WeightedDirGraphStEphIsize::from_weighed_edges(v, edges);
    assert_eq!(g.labeled_arcs().size(), 2);
}
