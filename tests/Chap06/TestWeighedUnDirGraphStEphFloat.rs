//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap06 WeighedUnDirGraphStEphFloat.

use std::f64::consts::{E, PI};

use ordered_float::OrderedFloat;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_ai::Chap06::WeighedUnDirGraphStEphFloat::WeighedUnDirGraphStEphFloat::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use apas_ai::WeighedUnDirGraphStEphFloatLit;

#[test]
fn test_weighedundirgraphstephfloatlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeighedUnDirGraphStEphFloat<i32> = WeighedUnDirGraphStEphFloatLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with weighed edges
    let with_data = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.5)), Triple(3, 1, OrderedFloat(3.5))]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 3);
}

#[test]
fn test_create_empty_graph() {
    let graph = WeighedUnDirGraphStEphFloat::<i32>::empty();
    assert_eq!(graph.vertices().size(), 0);
    assert_eq!(graph.edges().size(), 0);
}

#[test]
fn test_add_vertices_and_edges() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);

    graph.add_weighed_edge(1, 2, OrderedFloat(PI));
    graph.add_weighed_edge(2, 3, OrderedFloat(E));
    assert_eq!(graph.edges().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(42.5));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(42.5)));
    assert_eq!(graph.get_edge_weight(&2, &1), Some(OrderedFloat(42.5))); // Undirected
    assert_eq!(graph.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighed_edges() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, OrderedFloat(5.5)), Triple(2, 3, OrderedFloat(10.5))]
    );

    let edges = graph.weighed_edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_weighed() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(5.5)), Triple(1, 3, OrderedFloat(10.5)), Triple(2, 4, OrderedFloat(15.5))]
    );

    let neighbors_1 = graph.neighbors_weighed(&1);
    assert_eq!(neighbors_1.size(), 2);

    let neighbors_2 = graph.neighbors_weighed(&2);
    assert_eq!(neighbors_2.size(), 2);

    let neighbors_4 = graph.neighbors_weighed(&4);
    assert_eq!(neighbors_4.size(), 1);
}

#[test]
fn test_total_weight() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, OrderedFloat(10.0)), Triple(2, 3, OrderedFloat(20.0)), Triple(3, 1, OrderedFloat(30.0))]
    );

    assert!((graph.total_weight().0 - 60.0).abs() < 0.001);
}

#[test]
fn test_vertex_degree() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(5.0)), Triple(1, 3, OrderedFloat(10.0)), Triple(1, 4, OrderedFloat(15.0))]
    );

    assert_eq!(graph.vertex_degree(&1), 3);
    assert_eq!(graph.vertex_degree(&2), 1);
    assert_eq!(graph.vertex_degree(&3), 1);
    assert_eq!(graph.vertex_degree(&4), 1);
}

#[test]
fn test_is_connected_single_vertex() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1], E: []);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_two_vertices() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2], E: [Triple(1, 2, OrderedFloat(10.0))]);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_disconnected() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![Triple(1, 2, OrderedFloat(5.0)), Triple(3, 4, OrderedFloat(10.0))];
    let graph = WeighedUnDirGraphStEphFloat::from_weighed_edges(vertices, edges);
    assert!(!graph.is_connected());
}

#[test]
fn test_is_connected_fully_connected() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(10.0)), Triple(3, 1, OrderedFloat(15.0))]
    );
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_empty_graph() {
    let graph: WeighedUnDirGraphStEphFloat<i32> = WeighedUnDirGraphStEphFloatLit!();
    assert!(graph.is_connected()); // Empty graph is considered connected
}

#[test]
fn test_from_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(10.5)), Triple(2, 3, OrderedFloat(20.5))];
    let graph = WeighedUnDirGraphStEphFloat::from_weighed_edges(vertices, edges);

    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(10.5)));
    assert_eq!(graph.get_edge_weight(&2, &3), Some(OrderedFloat(20.5)));
}

#[test]
fn test_zero_weight_edge() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(0.0));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(0.0)));
}

#[test]
fn test_negative_weight_edge() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(-10.5));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(-10.5)));
}

#[test]
fn test_fractional_weights() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(PI));

    let weight = graph.get_edge_weight(&1, &2).unwrap();
    assert!((weight.0 - PI).abs() < 0.00001);
}

#[test]
fn test_min_weight_edge() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(2.0)), Triple(3, 4, OrderedFloat(8.0))]
    );

    let min_edge = graph.min_weight_edge().unwrap();
    assert_eq!(min_edge.2, OrderedFloat(2.0));
}

#[test]
fn test_max_weight_edge() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(2.0)), Triple(3, 4, OrderedFloat(8.0))]
    );

    let max_edge = graph.max_weight_edge().unwrap();
    assert_eq!(max_edge.2, OrderedFloat(8.0));
}

#[test]
fn test_min_max_weight_edge_empty() {
    let graph: WeighedUnDirGraphStEphFloat<i32> = WeighedUnDirGraphStEphFloatLit!();
    assert_eq!(graph.min_weight_edge(), None);
    assert_eq!(graph.max_weight_edge(), None);
}

#[test]
fn test_undirected_edge_symmetry() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(42.5));

    // Should be accessible from both directions with same weight
    assert_eq!(graph.get_edge_weight(&1, &2), graph.get_edge_weight(&2, &1));
}

#[test]
fn test_vertices_method() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2, 3, 4], E: [Triple(1, 2, OrderedFloat(1.0)), Triple(3, 4, OrderedFloat(2.0))]);
    let verts = graph.vertices();
    assert_eq!(verts.size(), 4);
    assert!(verts.mem(&1));
    assert!(verts.mem(&4));
}

#[test]
fn test_edges_method() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2, 3], E: [Triple(1, 2, OrderedFloat(5.0)), Triple(2, 3, OrderedFloat(10.0))]);
    let edges = graph.edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_method() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2, 3, 4], E: [Triple(1, 2, OrderedFloat(1.0)), Triple(1, 3, OrderedFloat(2.0)), Triple(1, 4, OrderedFloat(3.0))]);
    let neighbors = graph.neighbors(&1);
    assert_eq!(neighbors.size(), 3);
    assert!(neighbors.mem(&2));
    assert!(neighbors.mem(&3));
    assert!(neighbors.mem(&4));
}

#[test]
fn test_has_vertex() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2, 3], E: [Triple(1, 2, OrderedFloat(1.0))]);
    assert!(graph.vertices().mem(&1));
    assert!(graph.vertices().mem(&2));
    assert!(!graph.vertices().mem(&99));
}

#[test]
fn test_has_edge() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2, 3], E: [Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0))]);
    assert!(graph.get_edge_weight(&1, &2).is_some());
    assert!(graph.get_edge_weight(&2, &1).is_some()); // Undirected
    assert!(graph.get_edge_weight(&2, &3).is_some());
    assert!(graph.get_edge_weight(&1, &3).is_none());
}

#[test]
fn test_large_graph() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    for i in 0..50 {
        graph.add_vertex(i);
    }
    for i in 0..49 {
        graph.add_weighed_edge(i, i + 1, OrderedFloat((i as f64) * 0.5));
    }
    assert_eq!(graph.vertices().size(), 50);
    assert_eq!(graph.edges().size(), 49);
    assert!(graph.is_connected());
}

#[test]
fn test_cycle_graph() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(1.0)), Triple(3, 4, OrderedFloat(1.0)), Triple(4, 1, OrderedFloat(1.0))]
    );
    assert_eq!(graph.edges().size(), 4);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 2);
    }
}

#[test]
fn test_isolated_vertices() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 0);
    assert!(!graph.is_connected());
}

#[test]
fn test_self_loop() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_weighed_edge(1, 1, OrderedFloat(5.0));
    assert_eq!(graph.edges().size(), 1);
}

#[test]
fn test_duplicate_edge_addition() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(5.0));
    graph.add_weighed_edge(1, 2, OrderedFloat(10.0));
    // Behavior depends on underlying implementation
    assert!(graph.has_edge(&1, &2));
}

#[test]
fn test_string_vertices() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: ["New York", "Boston", "Philadelphia"],
        E: [Triple("New York", "Boston", OrderedFloat(215.0)), Triple("Boston", "Philadelphia", OrderedFloat(305.0))]
    );
    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert!(graph.vertices().mem(&"New York"));
}

#[test]
fn test_weighed_neighbors_isolated() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    let neighbors = graph.neighbors_weighed(&1);
    assert_eq!(neighbors.size(), 0);
}

#[test]
fn test_total_weight_empty() {
    let graph: WeighedUnDirGraphStEphFloat<i32> = WeighedUnDirGraphStEphFloatLit!();
    assert_eq!(graph.total_weight(), OrderedFloat(0.0));
}

#[test]
fn test_vertex_degree_zero() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    assert_eq!(graph.vertex_degree(&1), 0);
}

#[test]
fn test_complete_graph_k4() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, OrderedFloat(1.0)), Triple(1, 3, OrderedFloat(1.0)), Triple(1, 4, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(1.0)), Triple(2, 4, OrderedFloat(1.0)), Triple(3, 4, OrderedFloat(1.0))]
    );
    assert_eq!(graph.edges().size(), 6);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 3);
    }
    assert!(graph.is_connected());
}

#[test]
fn test_path_graph() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4, 5],
        E: [Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0)), Triple(3, 4, OrderedFloat(3.0)), Triple(4, 5, OrderedFloat(4.0))]
    );
    assert!(graph.is_connected());
    assert_eq!(graph.vertex_degree(&1), 1);
    assert_eq!(graph.vertex_degree(&3), 2);
}

#[test]
fn test_star_graph() {
    let graph = WeighedUnDirGraphStEphFloatLit!(
        V: [0, 1, 2, 3, 4],
        E: [Triple(0, 1, OrderedFloat(1.0)), Triple(0, 2, OrderedFloat(2.0)), Triple(0, 3, OrderedFloat(3.0)), Triple(0, 4, OrderedFloat(4.0))]
    );
    assert_eq!(graph.vertex_degree(&0), 4);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 1);
    }
}

#[test]
fn test_inf_weight() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(f64::INFINITY));
    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(f64::INFINITY)));
}

#[test]
fn test_very_small_weight() {
    let mut graph = WeighedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, OrderedFloat(1e-10));
    let weight = graph.get_edge_weight(&1, &2).unwrap();
    assert!((weight.0 - 1e-10).abs() < 1e-15);
}

#[test]
fn test_display_format() {
    let graph = WeighedUnDirGraphStEphFloatLit!(V: [1, 2], E: [Triple(1, 2, OrderedFloat(PI))]);
    let display_str = format!("{graph}");
    assert!(!display_str.is_empty());
}
