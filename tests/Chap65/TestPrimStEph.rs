#![cfg(feature = "all_chapters")]
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 65: Prim's MST Algorithm Tests (Sequential)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap65::PrimStEph::PrimStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn build_triangle_graph() -> LabUnDirGraphStEph<usize, u64> {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        LabEdge(0, 1, 1u64),
        LabEdge(1, 2, 2u64),
        LabEdge(0, 2, 3u64)
    ];
    <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges)
}

fn build_square_graph() -> LabUnDirGraphStEph<usize, u64> {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, 1u64),
        LabEdge(1, 2, 2u64),
        LabEdge(2, 3, 3u64),
        LabEdge(0, 3, 4u64),
        LabEdge(0, 2, 5u64)
    ];
    <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges)
}

#[test]
fn test_prim_triangle() {
    let graph = build_triangle_graph();
    let mst = prim_mst(&graph, &0);

    assert_eq!(mst.size(), 2);
    let weight = mst_weight(&mst);
    assert_eq!(weight, 3u64);

    assert!(mst.mem(&LabEdge(0, 1, 1u64)));
    assert!(mst.mem(&LabEdge(1, 2, 2u64)));
}

#[test]
fn test_prim_square() {
    let graph = build_square_graph();
    let mst = prim_mst(&graph, &0);

    assert_eq!(mst.size(), 3);
    let weight = mst_weight(&mst);
    assert_eq!(weight, 6u64);

    assert!(mst.mem(&LabEdge(0, 1, 1u64)));
    assert!(mst.mem(&LabEdge(1, 2, 2u64)));
    assert!(mst.mem(&LabEdge(2, 3, 3u64)));
}

#[test]
fn test_prim_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = prim_mst(&graph, &0);
    assert_eq!(mst.size(), 0);
    assert_eq!(mst_weight(&mst), 0u64);
}

#[test]
fn test_prim_different_start() {
    let graph = build_triangle_graph();

    let mst1 = prim_mst(&graph, &0);
    let mst2 = prim_mst(&graph, &1);
    let mst3 = prim_mst(&graph, &2);

    assert_eq!(mst_weight(&mst1), mst_weight(&mst2));
    assert_eq!(mst_weight(&mst2), mst_weight(&mst3));
}

#[test]
fn test_prim_star_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, 5u64),
        LabEdge(0, 2, 3u64),
        LabEdge(0, 3, 7u64)
    ];
    let graph = <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = prim_mst(&graph, &0);
    assert_eq!(mst.size(), 3);
    assert_eq!(mst_weight(&mst), 15u64);
}

#[test]
fn test_prim_complete_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        LabEdge(0, 1, 1u64),
        LabEdge(0, 2, 2u64),
        LabEdge(0, 3, 3u64),
        LabEdge(1, 2, 4u64),
        LabEdge(1, 3, 5u64),
        LabEdge(2, 3, 6u64)
    ];
    let graph = <LabUnDirGraphStEph<usize, u64> as LabUnDirGraphStEphTrait<usize, u64>>::from_vertices_and_labeled_edges(vertices, edges);

    let mst = prim_mst(&graph, &0);
    assert_eq!(mst.size(), 3);
    assert_eq!(mst_weight(&mst), 6u64);
}
