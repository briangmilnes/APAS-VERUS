#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal's MST Algorithm Tests (Sequential)

use ordered_float::OrderedFloat;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap65::KruskalStEph::KruskalStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn build_triangle_graph() -> LabUnDirGraphStEph<N, OrderedFloat<f64>> {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(0, 2, OrderedFloat(3.0))
    ];
    <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges)
}

fn build_square_graph() -> LabUnDirGraphStEph<N, OrderedFloat<f64>> {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(2, 3, OrderedFloat(3.0)),
        LabEdge(0, 3, OrderedFloat(4.0)),
        LabEdge(0, 2, OrderedFloat(5.0))
    ];
    <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges)
}

#[test]
fn test_kruskal_triangle() {
    let graph = build_triangle_graph();
    let mst = kruskal_mst(&graph);

    assert_eq!(mst.size(), 2);
    let weight = mst_weight(&mst);
    assert_eq!(weight, OrderedFloat(3.0));

    assert!(mst.mem(&LabEdge(0, 1, OrderedFloat(1.0))));
    assert!(mst.mem(&LabEdge(1, 2, OrderedFloat(2.0))));
}

#[test]
fn test_kruskal_square() {
    let graph = build_square_graph();
    let mst = kruskal_mst(&graph);

    assert_eq!(mst.size(), 3);
    let weight = mst_weight(&mst);
    assert_eq!(weight, OrderedFloat(6.0));

    assert!(mst.mem(&LabEdge(0, 1, OrderedFloat(1.0))));
    assert!(mst.mem(&LabEdge(1, 2, OrderedFloat(2.0))));
    assert!(mst.mem(&LabEdge(2, 3, OrderedFloat(3.0))));
}

#[test]
fn test_kruskal_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = kruskal_mst(&graph);
    assert_eq!(mst.size(), 0);
    assert_eq!(mst_weight(&mst), OrderedFloat(0.0));
    assert!(verify_mst_size(1, &mst));
}

#[test]
fn test_kruskal_star_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(5.0)),
        LabEdge(0, 2, OrderedFloat(3.0)),
        LabEdge(0, 3, OrderedFloat(7.0))
    ];
    let graph = <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = kruskal_mst(&graph);
    assert_eq!(mst.size(), 3);
    assert_eq!(mst_weight(&mst), OrderedFloat(15.0));
    assert!(verify_mst_size(4, &mst));
}

#[test]
fn test_kruskal_complete_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(0, 2, OrderedFloat(2.0)),
        LabEdge(0, 3, OrderedFloat(3.0)),
        LabEdge(1, 2, OrderedFloat(4.0)),
        LabEdge(1, 3, OrderedFloat(5.0)),
        LabEdge(2, 3, OrderedFloat(6.0))
    ];
    let graph = <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = kruskal_mst(&graph);
    assert_eq!(mst.size(), 3);
    assert_eq!(mst_weight(&mst), OrderedFloat(6.0));
    assert!(verify_mst_size(4, &mst));
}

#[test]
fn test_kruskal_verify_size() {
    let graph = build_triangle_graph();
    let mst = kruskal_mst(&graph);
    assert!(verify_mst_size(3, &mst));
}

#[test]
fn test_kruskal_all_equal_weights() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(1.0)),
        LabEdge(2, 3, OrderedFloat(1.0)),
        LabEdge(0, 3, OrderedFloat(1.0))
    ];
    let graph = <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = kruskal_mst(&graph);
    assert_eq!(mst.size(), 3);
    assert_eq!(mst_weight(&mst), OrderedFloat(3.0));
}
