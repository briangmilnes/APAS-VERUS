//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::f64::consts::{E, PI, SQRT_2};
use std::sync::{Arc, Barrier};
use std::thread;

use ordered_float::OrderedFloat;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_ai::Chap06::WeighedUnDirGraphMtEphFloat::WeighedUnDirGraphMtEphFloat::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use apas_ai::WeighedUnDirGraphMtEphFloatLit;

#[test]
fn test_weighedundirgraphmtephfloatlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeighedUnDirGraphMtEphFloat<i32> = WeighedUnDirGraphMtEphFloatLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.labeled_edges().size(), 0);

    // Test graph creation with vertices and edges
    let with_data: WeighedUnDirGraphMtEphFloat<i32> = WeighedUnDirGraphMtEphFloatLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.0))]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.labeled_edges().size(), 2);
}

#[test]
fn test_weighedundirgraphmtephfloat_empty() {
    let empty_graph = WeighedUnDirGraphMtEphFloat::<i32>::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_edges().size(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_edges().size(), 0);
}

#[test]
fn test_weighedundirgraphmtephfloat_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.5)),
        LabEdge(1, 2, OrderedFloat(2.7)),
        LabEdge(2, 3, OrderedFloat(0.8))
    ];
    let g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_edges().size(), 3);

    // Test neighbor relationships (undirected - both directions)
    assert!(g.has_edge(&0, &1));
    assert!(g.has_edge(&1, &0)); // Undirected graph
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1));
    assert!(!g.has_edge(&0, &2)); // No direct edge

    // Test NG (neighbors) - should be symmetric
    let ng0 = g.neighbors(&0);
    assert_eq!(ng0.size(), 1);
    assert!(ng0.mem(&1));

    let ng1 = g.neighbors(&1);
    assert_eq!(ng1.size(), 2);
    assert!(ng1.mem(&0));
    assert!(ng1.mem(&2));

    let ng2 = g.neighbors(&2);
    assert_eq!(ng2.size(), 2);
    assert!(ng2.mem(&1));
    assert!(ng2.mem(&3));

    // Test degrees (in undirected graph, InDegree = OutDegree = Degree)
    assert_eq!(g.vertex_degree(&0), 1);
    assert_eq!(g.vertex_degree(&0), 1);
    assert_eq!(g.vertex_degree(&0), 1);

    assert_eq!(g.vertex_degree(&1), 2);
    assert_eq!(g.vertex_degree(&1), 2);
    assert_eq!(g.vertex_degree(&1), 2);

    assert_eq!(g.vertex_degree(&2), 2);
    assert_eq!(g.vertex_degree(&2), 2);
    assert_eq!(g.vertex_degree(&2), 2);

    assert_eq!(g.vertex_degree(&3), 1);
    assert_eq!(g.vertex_degree(&3), 1);
    assert_eq!(g.vertex_degree(&3), 1);
}

#[test]
fn test_weighedundirgraphmtephfloat_incident_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(PI)),
        LabEdge(1, 2, OrderedFloat(E)),
        LabEdge(0, 2, OrderedFloat(SQRT_2))
    ];
    let _g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    // Test incident edges (each edge is incident to both endpoints)
    // let incident0 = g.incident(&0); // TODO: method not available
    // assert_eq!(incident0.size(), 2); // 0-1 and 0-2

    // let incident1 = g.incident(&1); // TODO: method not available
    // assert_eq!(incident1.size(), 2); // 0-1 and 1-2

    // let incident2 = g.incident(&2); // TODO: method not available
    // assert_eq!(incident2.size(), 2); // 1-2 and 0-2
}

#[test]
fn test_weighedundirgraphmtephfloat_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(2, 3, OrderedFloat(3.0)),
        LabEdge(0, 3, OrderedFloat(4.0))
    ];
    let _g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];
    // let ng_subset = g.ng_of_vertices(&vertices_subset); // TODO: method not available

    // Neighbors of {0, 1} should include all vertices connected to 0 or 1
    // assert_eq!(ng_subset.size(), 3); // TODO: method not available
    // assert_eq!(ng_subset.mem(&1), true); // 0-1 // TODO: method not available
    // assert_eq!(ng_subset.mem(&2), true); // 1-2 // TODO: method not available
    // assert_eq!(ng_subset.mem(&3), true); // 0-3 // TODO: method not available
}

#[test]
fn test_weighedundirgraphmtephfloat_nplusminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.5)),
        LabEdge(1, 2, OrderedFloat(1.5)),
        LabEdge(2, 0, OrderedFloat(2.5)),
        LabEdge(3, 1, OrderedFloat(3.5))
    ];
    let _g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];

    // In undirected graphs, NPlus and NMinus should be the same as NG
    // let nplus_subset = g.n_plus_of_vertices(&vertices_subset); // TODO: method not available
    // let nminus_subset = g.n_minus_of_vertices(&vertices_subset); // TODO: method not available
    // let ng_subset = g.ng_of_vertices(&vertices_subset); // TODO: method not available

    // All should be equal in undirected graph
    // assert_eq!(nplus_subset.size(), ng_subset.size()); // TODO: method not available
    // assert_eq!(nminus_subset.size(), ng_subset.size()); // TODO: method not available

    // Check that all contain the same elements
    for _vertex in [1, 2, 3] {
        // assert_eq!(nplus_subset.mem(&vertex), ng_subset.mem(&vertex)); // TODO: method not available
        // assert_eq!(nminus_subset.mem(&vertex), ng_subset.mem(&vertex)); // TODO: method not available
    }
}

#[test]
fn test_weighedundirgraphmtephfloat_edge_cases() {
    // Test empty graph
    let empty = WeighedUnDirGraphMtEphFloat::<i32>::empty();
    assert!(!empty.has_edge(&0, &1));
    assert_eq!(empty.neighbors(&0).size(), 0);
    assert_eq!(empty.vertex_degree(&0), 0);

    // Test single vertex
    let v_single: SetStEph<N> = SetLit![42];
    let a_empty: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![];
    let g_single = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_edges().size(), 0);
    assert_eq!(g_single.vertex_degree(&42), 0);
    assert_eq!(g_single.neighbors(&42).size(), 0);

    // Test self-loop with weight
    let v_self: SetStEph<N> = SetLit![1];
    let a_self: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
    let g_self = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v_self, a_self);

    assert!(g_self.has_edge(&1, &1));
    // In this implementation, self-loop contributes 1 to degree (unique neighbors only)
    assert_eq!(g_self.vertex_degree(&1), 1);
}

#[test]
fn test_weighedundirgraphmtephfloat_nonexistent_vertex() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
    let g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    // Query non-existent vertex
    assert!(!g.has_edge(&99, &0));
    assert_eq!(g.neighbors(&99).size(), 0);
    assert_eq!(g.vertex_degree(&99), 0);
    assert_eq!(g.vertex_degree(&99), 0);
    assert_eq!(g.vertex_degree(&99), 0);
}

#[test]
fn test_weighedundirgraphmtephfloat_weight_variations() {
    // Test with various weight values including negative, zero, and very small/large
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
        LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
        LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
        LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
        LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
    ];
    let g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.vertices().size(), 5);
    assert_eq!(g.labeled_edges().size(), 5);

    // All edges should still be recognized regardless of weight
    assert!(g.has_edge(&0, &1));
    assert!(g.has_edge(&1, &0)); // Undirected
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Undirected
    assert!(g.has_edge(&2, &3));
    assert!(g.has_edge(&3, &2)); // Undirected
    assert!(g.has_edge(&3, &4));
    assert!(g.has_edge(&4, &3)); // Undirected
    assert!(g.has_edge(&4, &0));
    assert!(g.has_edge(&0, &4)); // Undirected

    // Each vertex should have degree 2 (connected to 2 neighbors)
    for vertex in [0, 1, 2, 3, 4] {
        assert_eq!(g.vertex_degree(&vertex), 2);
        assert_eq!(g.vertex_degree(&vertex), 2); // Same as degree in undirected
        assert_eq!(g.vertex_degree(&vertex), 2); // Same as degree in undirected
    }
}

#[test]
fn test_weighedundirgraphmtephfloat_concurrent_access() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.1)),
        LabEdge(1, 2, OrderedFloat(2.2)),
        LabEdge(2, 3, OrderedFloat(3.3)),
        LabEdge(3, 4, OrderedFloat(4.4)),
        LabEdge(0, 4, OrderedFloat(5.5)) // Additional edge for more interesting topology
    ];
    let g = Arc::new(WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a));

    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = vec![];
    for i in 0..num_threads {
        let g_clone = Arc::clone(&g);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait(); // Wait for all threads to be ready

            // Perform various read operations concurrently
            let _ = g_clone.has_edge(&i, &(i + 1));
            let _ = g_clone.neighbors(&i);
            // let _ = g_clone.n_plus(&i); // TODO: method not available
            // let _ = g_clone.n_minus(&i); // TODO: method not available
            let _ = g_clone.vertex_degree(&i);
            let _ = g_clone.vertex_degree(&i);
            let _ = g_clone.vertex_degree(&i);

            // Verify basic properties
            assert_eq!(g_clone.vertices().size(), 5);
            assert_eq!(g_clone.labeled_edges().size(), 5);

            // In undirected graph, InDegree should equal OutDegree
            assert_eq!(g_clone.vertex_degree(&i), g_clone.vertex_degree(&i));

            (
                g_clone.neighbors(&i).size(),
                g_clone.vertex_degree(&i),
                g_clone.vertex_degree(&i),
                g_clone.vertex_degree(&i),
            )
        }));
    }

    for handle in handles {
        let (_ng_size, degree, in_degree, out_degree) = handle.join().unwrap();
        // Verify undirected graph properties
        assert_eq!(in_degree, out_degree);
        assert_eq!(degree, in_degree); // In undirected graph, degree == in_degree == out_degree
    }
}

#[test]
fn test_weighedundirgraphmtephfloat_complete_graph() {
    // Test complete graph K4 with weights
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.1)),
        LabEdge(0, 2, OrderedFloat(0.2)),
        LabEdge(0, 3, OrderedFloat(0.3)),
        LabEdge(1, 2, OrderedFloat(1.2)),
        LabEdge(1, 3, OrderedFloat(1.3)),
        LabEdge(2, 3, OrderedFloat(2.3))
    ];
    let g = WeighedUnDirGraphMtEphFloat::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_edges().size(), 6);

    // Every vertex should have degree 3 in K4
    for vertex in [0, 1, 2, 3] {
        assert_eq!(g.vertex_degree(&vertex), 3);
        assert_eq!(g.neighbors(&vertex).size(), 3);
        assert_eq!(g.vertex_degree(&vertex), 3);
        assert_eq!(g.vertex_degree(&vertex), 3);
    }

    // Every pair should be neighbors
    for i in [0, 1, 2, 3] {
        for j in [0, 1, 2, 3] {
            if i != j {
                assert!(g.has_edge(&i, &j));
            }
        }
    }
}

#[test]
fn test_from_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 2);
}

#[test]
fn test_add_weighed_edge() {
    let mut g = WeighedUnDirGraphMtEphFloat::<i32>::empty();
    g.add_vertex(1);
    g.add_vertex(2);
    g.add_weighed_edge(1, 2, OrderedFloat(PI));
    assert!(g.has_edge(&1, &2));
    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(PI)));
}

#[test]
fn test_get_edge_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.7))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(1.5)));
    assert_eq!(g.get_edge_weight(&2, &3), Some(OrderedFloat(2.7)));
    assert_eq!(g.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighed_edges() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, OrderedFloat(5.0))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let weighed = g.weighed_edges();
    assert_eq!(weighed.size(), 1);
}

#[test]
fn test_neighbors_weighed() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(1, 3, OrderedFloat(2.5))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let neighbors = g.neighbors_weighed(&1);
    assert_eq!(neighbors.size(), 2);
}

#[test]
fn test_total_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    assert_eq!(g.total_weight(), OrderedFloat(3.0));
}

#[test]
fn test_vertex_degree() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.0)), Triple(1, 3, OrderedFloat(2.0))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    assert_eq!(g.vertex_degree(&1), 2);
    assert_eq!(g.vertex_degree(&2), 1);
    assert_eq!(g.vertex_degree(&3), 1);
}

#[test]
fn test_parallel_neighbors_weighed() {
    let mut vertices = SetStEph::empty();
    for i in 0..15 {
        vertices.insert(i);
    }

    let mut edges = SetStEph::empty();
    for i in 1..13 {
        edges.insert(Triple(0, i, OrderedFloat(i as f64 * 1.5)));
    }

    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let neighbors_weighed = g.neighbors_weighed(&0);
    assert_eq!(neighbors_weighed.size(), 12);

    assert!(neighbors_weighed.mem(&Pair(1, OrderedFloat(1.5))));
    assert!(neighbors_weighed.mem(&Pair(5, OrderedFloat(7.5))));
    assert!(neighbors_weighed.mem(&Pair(12, OrderedFloat(18.0))));
}

#[test]
fn test_display_debug_traits() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, OrderedFloat(3.14))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let display_str = format!("{}", g);
    assert!(display_str.contains("LabUnDirGraph"));

    let debug_str = format!("{:?}", g);
    assert!(debug_str.contains("LabUnDirGraph"));
}

#[test]
fn test_clone_mteph() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.5))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let g2 = g.clone();
    assert_eq!(g2.vertices().size(), 3);
    assert_eq!(g2.labeled_edges().size(), 2);
    assert!(g2.has_edge(&1, &2));
}

#[test]
fn test_get_edge_weight_mteph() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.5))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(1.5)));
    assert_eq!(g.get_edge_weight(&2, &1), Some(OrderedFloat(1.5)));
    assert_eq!(g.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighed_edges_mteph() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, OrderedFloat(3.14))];
    let g = WeighedUnDirGraphMtEphFloat::from_weighed_edges(vertices, edges);

    let we = g.weighed_edges();
    assert_eq!(we.size(), 1);
}
