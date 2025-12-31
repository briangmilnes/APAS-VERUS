//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::f64::consts::{E, PI};

use ordered_float::OrderedFloat;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
use apas_ai::Chap06::WeighedDirGraphStEphFloat::WeighedDirGraphStEphFloat::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use apas_ai::WeighedDirGraphStEphFloatLit;

#[test]
fn test_weigheddirgraphstephfloatlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeighedDirGraphStEphFloat<i32> = WeighedDirGraphStEphFloatLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with weighed edges
    let with_data = WeighedDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        A: [Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.5)), Triple(3, 1, OrderedFloat(3.5))]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.arcs().size(), 3);
}

#[test]
fn test_weigheddirgraphstephfloat_empty() {
    let empty_graph = WeighedDirGraphStEphFloat::<i32>::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
    assert_eq!(empty_graph.arcs().size(), 0);
}

#[test]
fn test_weigheddirgraphstephfloat_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedF64>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.5)),
        LabEdge(1, 2, OrderedFloat(2.7)),
        LabEdge(2, 3, OrderedFloat(0.8))
    ];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_arcs().size(), 3);

    // Test arc relationships
    assert!(g.has_arc(&0, &1));
    assert!(!g.has_arc(&1, &0)); // Directed graph
    assert!(g.has_arc(&1, &2));
    assert!(!g.has_arc(&2, &1));

    // Test out-neighbors
    let out0 = g.out_neighbors(&0);
    assert_eq!(out0.size(), 1);
    assert!(out0.mem(&1));

    let out1 = g.out_neighbors(&1);
    assert_eq!(out1.size(), 1);
    assert!(out1.mem(&2));

    // Test in-neighbors
    let in1 = g.in_neighbors(&1);
    assert_eq!(in1.size(), 1);
    assert!(in1.mem(&0));

    let in3 = g.in_neighbors(&3);
    assert_eq!(in3.size(), 1);
    assert!(in3.mem(&2));

    // Test arc weights
    assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(1.5)));
    assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(2.7)));
    assert_eq!(g.get_arc_label(&2, &3), Some(&OrderedFloat(0.8)));
    assert_eq!(g.get_arc_label(&0, &2), None); // No direct arc
}

#[test]
fn test_weigheddirgraphstephfloat_mutable_operations() {
    let mut g = WeighedDirGraphStEphFloat::<i32>::empty();

    // Add vertices
    g.add_vertex(0);
    g.add_vertex(1);
    g.add_vertex(2);

    assert_eq!(g.vertices().size(), 3);
    assert!(g.vertices().mem(&0));
    assert!(g.vertices().mem(&1));
    assert!(g.vertices().mem(&2));

    // Add weighed arcs
    g.add_labeled_arc(0, 1, OrderedFloat(PI));
    g.add_labeled_arc(1, 2, OrderedFloat(E));

    assert_eq!(g.labeled_arcs().size(), 2);
    assert!(g.has_arc(&0, &1));
    assert!(g.has_arc(&1, &2));
    assert!(!g.has_arc(&0, &2));

    // Test weights
    assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(PI)));
    assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(E)));
}

#[test]
fn test_weigheddirgraphstephfloat_weight_variations() {
    // Test with various weight values including negative, zero, and very small/large
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, OrderedF64>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
        LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
        LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
        LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
        LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
    ];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 5);
    assert_eq!(g.labeled_arcs().size(), 5);

    // All edges should still be recognized regardless of weight
    assert!(g.has_arc(&0, &1));
    assert!(g.has_arc(&1, &2));
    assert!(g.has_arc(&2, &3));
    assert!(g.has_arc(&3, &4));
    assert!(g.has_arc(&4, &0));

    // Test specific weights
    assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(0.0)));
    assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(-1.5)));
    assert_eq!(g.get_arc_label(&2, &3), Some(&OrderedFloat(1e-10)));
    assert_eq!(g.get_arc_label(&3, &4), Some(&OrderedFloat(1e10)));
    assert_eq!(g.get_arc_label(&4, &0), Some(&OrderedFloat(f64::INFINITY)));
}

#[test]
fn test_weigheddirgraphstephfloat_nan_handling() {
    // Test with NaN weights (OrderedFloat handles NaN consistently)
    let v: SetStEph<N> = SetLit![0, 1];
    let a: SetStEph<LabEdge<N, OrderedF64>> = SetLit![LabEdge(0, 1, OrderedFloat(f64::NAN))];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 2);
    assert_eq!(g.labeled_arcs().size(), 1);
    assert!(g.has_arc(&0, &1));

    // NaN should be handled consistently by OrderedFloat
    let weight = g.get_arc_label(&0, &1);
    assert!(weight.is_some());
    assert!(weight.unwrap().is_nan());
}

#[test]
fn test_weigheddirgraphstephfloat_edge_cases() {
    // Test empty graph
    let empty = WeighedDirGraphStEphFloat::<i32>::empty();
    assert!(!empty.has_arc(&0, &1));
    assert_eq!(empty.out_neighbors(&0).size(), 0);
    assert_eq!(empty.in_neighbors(&0).size(), 0);
    assert_eq!(empty.get_arc_label(&0, &1), None);

    // Test single vertex
    let v_single: SetStEph<N> = SetLit![42];
    let a_empty: SetStEph<LabEdge<N, OrderedF64>> = SetLit![];
    let g_single = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_arcs().size(), 0);
    assert_eq!(g_single.out_neighbors(&42).size(), 0);
    assert_eq!(g_single.in_neighbors(&42).size(), 0);

    // Test self-loop with weight
    let v_self: SetStEph<N> = SetLit![1];
    let a_self: SetStEph<LabEdge<N, OrderedF64>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
    let g_self = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v_self, a_self);

    assert!(g_self.has_arc(&1, &1));
    assert_eq!(g_self.out_neighbors(&1).size(), 1);
    assert_eq!(g_self.in_neighbors(&1).size(), 1);
    assert_eq!(g_self.get_arc_label(&1, &1), Some(&OrderedFloat(99.9)));
}

#[test]
fn test_weigheddirgraphstephfloat_nonexistent_vertex() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedF64>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    // Query non-existent vertex
    assert!(!g.has_arc(&99, &0));
    assert_eq!(g.out_neighbors(&99).size(), 0);
    assert_eq!(g.in_neighbors(&99).size(), 0);
    assert_eq!(g.get_arc_label(&99, &0), None);
}

#[test]
fn test_weigheddirgraphstephfloat_arcs_conversion() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedF64>> =
        SetLit![LabEdge(0, 1, OrderedFloat(1.1)), LabEdge(1, 2, OrderedFloat(2.2))];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    // Test arcs() method that converts weighed arcs to unlabeled edges
    let arcs = g.arcs();
    assert_eq!(arcs.size(), 2);
    assert!(arcs.mem(&Edge(0, 1)));
    assert!(arcs.mem(&Edge(1, 2)));
    assert!(!arcs.mem(&Edge(0, 2)));
}

#[test]
fn test_weigheddirgraphstephfloat_complex_topology() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedF64>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(2, 3, OrderedFloat(3.0)),
        LabEdge(0, 3, OrderedFloat(4.0)),
        LabEdge(1, 3, OrderedFloat(5.0))
    ];
    let g = WeighedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);

    // Test multiple paths and weights
    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_arcs().size(), 5);

    // Test out-neighbors with multiple edges
    let out0 = g.out_neighbors(&0);
    assert_eq!(out0.size(), 2);
    assert!(out0.mem(&1));
    assert!(out0.mem(&3));

    let out1 = g.out_neighbors(&1);
    assert_eq!(out1.size(), 2);
    assert!(out1.mem(&2));
    assert!(out1.mem(&3));

    // Test in-neighbors with multiple edges
    let in3 = g.in_neighbors(&3);
    assert_eq!(in3.size(), 3);
    assert!(in3.mem(&0));
    assert!(in3.mem(&1));
    assert!(in3.mem(&2));

    // Test different path weights to same destination
    assert_eq!(g.get_arc_label(&0, &3), Some(&OrderedFloat(4.0))); // Direct path
    assert_eq!(g.get_arc_label(&1, &3), Some(&OrderedFloat(5.0))); // Via vertex 1
}

#[test]
fn test_from_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.5))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_add_weighed_edge() {
    let mut g = WeighedDirGraphStEphFloat::<i32>::empty();
    g.add_vertex(1);
    g.add_vertex(2);
    g.add_weighed_edge(1, 2, OrderedFloat(3.0));
    assert!(g.has_arc(&1, &2));
    assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(3.0)));
}

#[test]
fn test_get_edge_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(4.5))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(4.5)));
    assert_eq!(g.get_edge_weight(&2, &3), None);
}

#[test]
fn test_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let weighed_edges = g.weighed_edges();
    assert_eq!(weighed_edges.size(), 2);
}

#[test]
fn test_out_neighbors_weighed() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(1, 3, OrderedFloat(2.5))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let out_neighbors = g.out_neighbors_weighed(&1);
    assert_eq!(out_neighbors.size(), 2);
}

#[test]
fn test_in_neighbors_weighed() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(3, 2, OrderedFloat(2.5))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let in_neighbors = g.in_neighbors_weighed(&2);
    assert_eq!(in_neighbors.size(), 2);
}

#[test]
fn test_total_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    assert_eq!(g.total_weight(), OrderedFloat(3.0));
}

#[test]
fn test_edges_above_weight() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.0)),
        Triple(2, 3, OrderedFloat(5.0)),
        Triple(3, 4, OrderedFloat(10.0))
    ];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let above_3 = g.edges_above_weight(OrderedFloat(3.0));
    assert_eq!(above_3.size(), 2);
}

#[test]
fn test_edges_below_weight() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.0)),
        Triple(2, 3, OrderedFloat(5.0)),
        Triple(3, 4, OrderedFloat(10.0))
    ];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let below_6 = g.edges_below_weight(OrderedFloat(6.0));
    assert_eq!(below_6.size(), 2);
}

#[test]
fn test_min_weight_edge() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let min_edge = g.min_weight_edge();
    assert!(min_edge.is_some());
    assert_eq!(min_edge.unwrap().2, OrderedFloat(2.0));
}

#[test]
fn test_max_weight_edge() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    let max_edge = g.max_weight_edge();
    assert!(max_edge.is_some());
    assert_eq!(max_edge.unwrap().2, OrderedFloat(5.0));
}

#[test]
fn test_scale_weights() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(2.0)), Triple(2, 3, OrderedFloat(4.0))];
    let mut g = WeighedDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    g.scale_weights(OrderedFloat(2.0));
    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(4.0)));
    assert_eq!(g.get_edge_weight(&2, &3), Some(OrderedFloat(8.0)));
}
