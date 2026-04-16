#![cfg(feature = "all_chapters")]
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 64: TSP 2-Approximation Tests (Sequential)

use apas_verus::Types::Types::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap64::TSPApproxStEph::TSPApproxStEph::*;
use apas_verus::SetLit;
use apas_verus::vstdplus::float::float::{WrappedF64, finite_dist, zero_dist};

fn create_triangle_graph() -> (
    LabUnDirGraphStEph<usize, WrappedF64>,
    SetStEph<LabEdge<usize, WrappedF64>>,
) {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        LabEdge(0, 1, WrappedF64 { val: 1.0 }),
        LabEdge(1, 2, WrappedF64 { val: 2.0 }),
        LabEdge(2, 0, WrappedF64 { val: 3.0 })
    ];
    let graph = <LabUnDirGraphStEph<usize, WrappedF64> as LabUnDirGraphStEphTrait<usize, WrappedF64>>::from_vertices_and_labeled_edges(vertices, edges.clone());

    let spanning_tree = SetLit![LabEdge(0, 1, WrappedF64 { val: 1.0 }), LabEdge(1, 2, WrappedF64 { val: 2.0 })];

    (graph, spanning_tree)
}

#[test]
fn test_euler_tour() {
    let (graph, tree) = create_triangle_graph();
    let tour = euler_tour(&graph, &0, &tree);

    assert!(tour.len() >= 3);
    assert_eq!(tour[0], 0);
}

#[test]
fn test_shortcut_tour() {
    let tour_with_dups = std::vec![0, 1, 2, 1, 0, 0];
    let shortcut = shortcut_tour(&tour_with_dups);

    assert_eq!(shortcut, std::vec![0, 1, 2, 0]);
}

#[test]
fn test_tour_weight() {
    let (graph, _) = create_triangle_graph();
    let tour = std::vec![0, 1, 2, 0];
    let weight = tour_weight(&graph, &tour);

    assert_eq!(weight, WrappedF64 { val: 6.0 });
}

#[test]
fn test_approx_metric_tsp() {
    let (graph, tree) = create_triangle_graph();
    let (tour, weight) = approx_metric_tsp(&graph, &tree, &0);

    assert!(tour.len() >= 3);
    assert_eq!(tour[0], tour[tour.len() - 1]);
    assert!(weight > zero_dist());
}
