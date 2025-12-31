//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap06 WeighedUnDirGraphStEphInt.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap06::WeighedUnDirGraphStEphInt::WeighedUnDirGraphStEphInt::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::WeighedUnDirGraphStEphIntLit;

#[test]
fn test_weighedundirgraphstephintlit_macro_functionality() {
    let empty: WeighedUnDirGraphStEphInt<i32> = WeighedUnDirGraphStEphIntLit!();
    assert_eq!(empty.vertices().size(), 0);

    let with_data = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 15), Triple(2, 3, 25), Triple(3, 1, 35)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 3);
}

#[test]
fn test_create_empty_graph() {
    let graph = WeighedUnDirGraphStEphInt::<i32>::empty();
    assert_eq!(graph.vertices().size(), 0);
    assert_eq!(graph.edges().size(), 0);
}

#[test]
fn test_add_vertices_and_edges() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);

    graph.add_weighed_edge(1, 2, 314);
    graph.add_weighed_edge(2, 3, 271);
    assert_eq!(graph.edges().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, 425);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(425));
    assert_eq!(graph.get_edge_weight(&2, &1), Some(425)); // Undirected
    assert_eq!(graph.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighed_edges() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 55), Triple(2, 3, 105)]
    );

    let edges = graph.weighed_edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_weighed() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 55), Triple(1, 3, 105), Triple(2, 4, 155)]
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
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 10), Triple(2, 3, 20), Triple(3, 1, 30)]
    );

    assert_eq!(graph.total_weight(), 60);
}

#[test]
fn test_vertex_degree() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 5), Triple(1, 3, 10), Triple(1, 4, 15)]
    );

    assert_eq!(graph.vertex_degree(&1), 3);
    assert_eq!(graph.vertex_degree(&2), 1);
    assert_eq!(graph.vertex_degree(&3), 1);
    assert_eq!(graph.vertex_degree(&4), 1);
}

#[test]
fn test_is_connected_single_vertex() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1], E: []);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_two_vertices() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2], E: [Triple(1, 2, 10)]);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_disconnected() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![Triple(1, 2, 5), Triple(3, 4, 10)];
    let graph = WeighedUnDirGraphStEphInt::from_weighed_edges(vertices, edges);
    assert!(!graph.is_connected());
}

#[test]
fn test_is_connected_fully_connected() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 5), Triple(2, 3, 10), Triple(3, 1, 15)]
    );
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_empty_graph() {
    let graph: WeighedUnDirGraphStEphInt<i32> = WeighedUnDirGraphStEphIntLit!();
    assert!(graph.is_connected());
}

#[test]
fn test_from_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 105), Triple(2, 3, 205)];
    let graph = WeighedUnDirGraphStEphInt::from_weighed_edges(vertices, edges);

    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(105));
    assert_eq!(graph.get_edge_weight(&2, &3), Some(205));
}

#[test]
fn test_zero_weight_edge() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, 0);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(0));
}

#[test]
fn test_negative_weight_edge() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, -105);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(-105));
}

#[test]
fn test_min_weight_edge() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 5), Triple(2, 3, 2), Triple(3, 4, 8)]
    );

    let edges = graph.weighed_edges();
    let min_edge = edges.iter().min_by_key(|e| e.2).unwrap();
    assert_eq!(min_edge.2, 2);
}

#[test]
fn test_max_weight_edge() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 5), Triple(2, 3, 2), Triple(3, 4, 8)]
    );

    let edges = graph.weighed_edges();
    let max_edge = edges.iter().max_by_key(|e| e.2).unwrap();
    assert_eq!(max_edge.2, 8);
}

#[test]
fn test_min_max_weight_edge_empty() {
    let graph: WeighedUnDirGraphStEphInt<i32> = WeighedUnDirGraphStEphIntLit!();
    let edges = graph.weighed_edges();
    assert_eq!(edges.iter().min_by_key(|e| e.2), None);
    assert_eq!(edges.iter().max_by_key(|e| e.2), None);
}

#[test]
fn test_undirected_edge_symmetry() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, 425);

    assert_eq!(graph.get_edge_weight(&1, &2), graph.get_edge_weight(&2, &1));
}

#[test]
fn test_vertices_method() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2, 3, 4], E: [Triple(1, 2, 1), Triple(3, 4, 2)]);
    let verts = graph.vertices();
    assert_eq!(verts.size(), 4);
    assert!(verts.mem(&1));
    assert!(verts.mem(&4));
}

#[test]
fn test_edges_method() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2, 3], E: [Triple(1, 2, 5), Triple(2, 3, 10)]);
    let edges = graph.edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_method() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2, 3, 4], E: [Triple(1, 2, 1), Triple(1, 3, 2), Triple(1, 4, 3)]);
    let neighbors = graph.neighbors(&1);
    assert_eq!(neighbors.size(), 3);
    assert!(neighbors.mem(&2));
    assert!(neighbors.mem(&3));
    assert!(neighbors.mem(&4));
}

#[test]
fn test_has_vertex() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2, 3], E: [Triple(1, 2, 1)]);
    assert!(graph.vertices().mem(&1));
    assert!(graph.vertices().mem(&2));
    assert!(!graph.vertices().mem(&99));
}

#[test]
fn test_has_edge() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2, 3], E: [Triple(1, 2, 1), Triple(2, 3, 2)]);
    assert!(graph.get_edge_weight(&1, &2).is_some());
    assert!(graph.get_edge_weight(&2, &1).is_some()); // Undirected
    assert!(graph.get_edge_weight(&2, &3).is_some());
    assert!(graph.get_edge_weight(&1, &3).is_none());
}

#[test]
fn test_large_graph() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    for i in 0..50 {
        graph.add_vertex(i);
    }
    for i in 0..49 {
        graph.add_weighed_edge(i, i + 1, i * 5);
    }
    assert_eq!(graph.vertices().size(), 50);
    assert_eq!(graph.edges().size(), 49);
    assert!(graph.is_connected());
}

#[test]
fn test_cycle_graph() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 1), Triple(2, 3, 1), Triple(3, 4, 1), Triple(4, 1, 1)]
    );
    assert_eq!(graph.edges().size(), 4);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 2);
    }
}

#[test]
fn test_isolated_vertices() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 0);
    assert!(!graph.is_connected());
}

#[test]
fn test_self_loop() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_weighed_edge(1, 1, 5);
    assert_eq!(graph.edges().size(), 1);
}

#[test]
fn test_duplicate_edge_addition() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, 5);
    graph.add_weighed_edge(1, 2, 10);
    assert!(graph.get_edge_weight(&1, &2).is_some());
}

#[test]
fn test_string_vertices() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: ["NYC", "BOS", "PHL"],
        E: [Triple("NYC", "BOS", 215), Triple("BOS", "PHL", 305)]
    );
    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert!(graph.vertices().mem(&"NYC"));
}

#[test]
fn test_weighed_neighbors_isolated() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    let neighbors = graph.neighbors_weighed(&1);
    assert_eq!(neighbors.size(), 0);
}

#[test]
fn test_total_weight_empty() {
    let graph: WeighedUnDirGraphStEphInt<i32> = WeighedUnDirGraphStEphIntLit!();
    assert_eq!(graph.total_weight(), 0);
}

#[test]
fn test_vertex_degree_zero() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    assert_eq!(graph.vertex_degree(&1), 0);
}

#[test]
fn test_complete_graph_k4() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [Triple(1, 2, 1), Triple(1, 3, 1), Triple(1, 4, 1), Triple(2, 3, 1), Triple(2, 4, 1), Triple(3, 4, 1)]
    );
    assert_eq!(graph.edges().size(), 6);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 3);
    }
    assert!(graph.is_connected());
}

#[test]
fn test_path_graph() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4, 5],
        E: [Triple(1, 2, 1), Triple(2, 3, 2), Triple(3, 4, 3), Triple(4, 5, 4)]
    );
    assert!(graph.is_connected());
    assert_eq!(graph.vertex_degree(&1), 1);
    assert_eq!(graph.vertex_degree(&3), 2);
}

#[test]
fn test_star_graph() {
    let graph = WeighedUnDirGraphStEphIntLit!(
        V: [0, 1, 2, 3, 4],
        E: [Triple(0, 1, 1), Triple(0, 2, 2), Triple(0, 3, 3), Triple(0, 4, 4)]
    );
    assert_eq!(graph.vertex_degree(&0), 4);
    for i in 1..=4 {
        assert_eq!(graph.vertex_degree(&i), 1);
    }
}

#[test]
fn test_large_weight() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, i32::MAX);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(i32::MAX));
}

#[test]
fn test_min_weight() {
    let mut graph = WeighedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighed_edge(1, 2, i32::MIN);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(i32::MIN));
}

#[test]
fn test_display_format() {
    let graph = WeighedUnDirGraphStEphIntLit!(V: [1, 2], E: [Triple(1, 2, 314)]);
    let display_str = format!("{graph}");
    assert!(!display_str.is_empty());
}
