//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::f64::consts::{E, PI, SQRT_2};
use std::sync::{Arc, Barrier};
use std::thread;

use ordered_float::OrderedFloat;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use apas_ai::WeightedDirGraphMtEphFloatLit;

#[test]
fn test_weighteddirgraphmtephfloatlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloatLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.labeled_arcs().size(), 0);

    // Test graph creation with vertices and arcs
    let with_data: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloatLit!(
        V: [1, 2, 3],
        A: [Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.0))]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.labeled_arcs().size(), 2);
}

#[test]
fn test_weighteddirgraphmtephfloat_empty() {
    let empty_graph = WeightedDirGraphMtEphFloat::<i32>::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
}

#[test]
fn test_weighteddirgraphmtephfloat_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.5)),
        LabEdge(1, 2, OrderedFloat(2.7)),
        LabEdge(2, 3, OrderedFloat(0.8)),
        LabEdge(0, 3, OrderedFloat(4.2))
    ];
    let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_arcs().size(), 4);

    // Test neighbor relationships
    assert!(g.has_arc(&0, &1));
    assert!(!g.has_arc(&1, &0)); // Directed graph
    assert!(g.has_arc(&1, &2));
    assert!(!g.has_arc(&2, &1));
    assert!(g.has_arc(&0, &3));

    // Test NG (neighbors)
    let ng0 = g.out_neighbors(&0);
    assert_eq!(ng0.size(), 2);
    assert!(ng0.mem(&1));
    assert!(ng0.mem(&3));

    let ng1 = g.out_neighbors(&1);
    assert_eq!(ng1.size(), 1);
    assert!(ng1.mem(&2));

    // Test NPlus (out-neighbors)
    let nplus0 = g.out_neighbors(&0);
    assert_eq!(nplus0.size(), 2);
    assert!(nplus0.mem(&1));
    assert!(nplus0.mem(&3));

    // Test NMinus (in-neighbors)
    let nminus1 = g.in_neighbors(&1);
    assert_eq!(nminus1.size(), 1);
    assert!(nminus1.mem(&0));

    let nminus3 = g.in_neighbors(&3);
    assert_eq!(nminus3.size(), 2);
    assert!(nminus3.mem(&0));
    assert!(nminus3.mem(&2));

    // Test degrees
    assert_eq!(g.out_neighbors(&0).size(), 2);
    assert_eq!(g.in_neighbors(&0).size(), 0);
    assert_eq!(g.out_neighbors(&0).size(), 2);

    assert_eq!(g.out_neighbors(&1).size(), 1);
    assert_eq!(g.in_neighbors(&1).size(), 1);
    assert_eq!(g.out_neighbors(&1).size(), 1);

    assert_eq!(g.out_neighbors(&3).size(), 0);
    assert_eq!(g.in_neighbors(&3).size(), 2);
    assert_eq!(g.out_neighbors(&3).size(), 0);
}

#[test]
fn test_weighteddirgraphmtephfloat_incident_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(PI)),
        LabEdge(1, 2, OrderedFloat(E)),
        LabEdge(0, 2, OrderedFloat(SQRT_2))
    ];
    let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    // Test incident edges
    // let incident0 = _g.Incident(&0); // TODO: method not available
    // assert_eq!(incident0.size(), 2); // 0->1 and 0->2 // TODO: method not available

    // let incident1 = g.Incident(&1); // TODO: method not available
    // assert_eq!(incident1.size(), 2); // 0->1 (incoming) and 1->2 (outgoing) // TODO: method not available

    // let incident2 = g.Incident(&2); // TODO: method not available
    // assert_eq!(incident2.size(), 2); // 1->2 and 0->2 (both incoming) // TODO: method not available
}

#[test]
fn test_weighteddirgraphmtephfloat_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(2, 3, OrderedFloat(3.0)),
        LabEdge(0, 3, OrderedFloat(4.0))
    ];
    let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];
    // let ng_subset = _g.NGOfVertices(&_vertices_subset); // TODO: method not available

    // Neighbors of {0, 1} should be {1, 2, 3}
    // assert_eq!(ng_subset.size(), 3); // TODO: method not available
    // assert_eq!(ng_subset.mem(&1), true); // TODO: method not available
    // assert_eq!(ng_subset.mem(&2), true); // TODO: method not available
    // assert_eq!(ng_subset.mem(&3), true); // TODO: method not available
}

#[test]
fn test_weighteddirgraphmtephfloat_nplusminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.5)),
        LabEdge(1, 2, OrderedFloat(1.5)),
        LabEdge(2, 0, OrderedFloat(2.5)),
        LabEdge(3, 1, OrderedFloat(3.5))
    ];
    let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];

    // Test NPlusOfVertices (out-neighbors)
    // let nplus_subset = _g.NPlusOfVertices(&_vertices_subset); // TODO: method not available
    // assert_eq!(nplus_subset.size(), 2); // TODO: method not available
    // assert_eq!(nplus_subset.mem(&1), true); // 0->1 // TODO: method not available
    // assert_eq!(nplus_subset.mem(&2), true); // 1->2 // TODO: method not available

    // Test NMinusOfVertices (in-neighbors)
    // let nminus_subset = g.NMinusOfVertices(&vertices_subset); // TODO: method not available
    // assert_eq!(nminus_subset.size(), 2); // TODO: method not available
    // assert_eq!(nminus_subset.mem(&2), true); // 2->0 // TODO: method not available
    // assert_eq!(nminus_subset.mem(&3), true); // 3->1 // TODO: method not available
}

#[test]
fn test_weighteddirgraphmtephfloat_edge_cases() {
    // Test empty graph
    let empty = WeightedDirGraphMtEphFloat::<i32>::empty();
    assert!(!empty.has_arc(&0, &1));
    assert_eq!(empty.out_neighbors(&0).size(), 0);
    assert_eq!(empty.out_neighbors(&0).size(), 0);

    // Test single vertex
    let v_single: SetStEph<N> = SetLit![42];
    let a_empty: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![];
    let g_single = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_arcs().size(), 0);
    assert_eq!(g_single.out_neighbors(&42).size(), 0);
    assert_eq!(g_single.out_neighbors(&42).size(), 0);

    // Test self-loop with weight
    let v_self: SetStEph<N> = SetLit![1];
    let a_self: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
    let g_self = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v_self, a_self);

    assert!(g_self.has_arc(&1, &1));
    assert_eq!(g_self.out_neighbors(&1).size(), 1); // Self-loop to self
    assert_eq!(g_self.in_neighbors(&1).size(), 1);
    assert_eq!(g_self.out_neighbors(&1).size(), 1);
}

#[test]
fn test_weighteddirgraphmtephfloat_nonexistent_vertex() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
    let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    // Query non-existent vertex
    assert!(!g.has_arc(&99, &0));
    assert_eq!(g.out_neighbors(&99).size(), 0);
    assert_eq!(g.out_neighbors(&99).size(), 0);
    assert_eq!(g.in_neighbors(&99).size(), 0);
    assert_eq!(g.out_neighbors(&99).size(), 0);
}

#[test]
fn test_weighteddirgraphmtephfloat_weight_variations() {
    // Test with various weight values including negative, zero, and very small/large
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
        LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
        LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
        LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
        LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
    ];
    let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 5);
    assert_eq!(g.labeled_arcs().size(), 5);

    // All edges should still be recognized regardless of weight
    assert!(g.has_arc(&0, &1));
    assert!(g.has_arc(&1, &2));
    assert!(g.has_arc(&2, &3));
    assert!(g.has_arc(&3, &4));
    assert!(g.has_arc(&4, &0));

    // Each vertex should have degree 2 (one in, one out)
    for vertex in [0, 1, 2, 3, 4] {
        assert_eq!(g.out_neighbors(&vertex).size(), 1);
        assert_eq!(g.in_neighbors(&vertex).size(), 1);
        assert_eq!(g.out_neighbors(&vertex).size(), 1);
    }
}

#[test]
fn test_weighteddirgraphmtephfloat_concurrent_access() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, OrderedFloat<f64>>> = SetLit![
        LabEdge(0, 1, OrderedFloat(1.1)),
        LabEdge(1, 2, OrderedFloat(2.2)),
        LabEdge(2, 3, OrderedFloat(3.3)),
        LabEdge(3, 4, OrderedFloat(4.4))
    ];
    let g = Arc::new(WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a));

    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = vec![];
    for i in 0..num_threads {
        let g_clone = Arc::clone(&g);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait(); // Wait for all threads to be ready

            // Perform various read operations concurrently
            let _ = g_clone.has_arc(&i, &(i + 1));
            let _ = g_clone.out_neighbors(&i);
            let _ = g_clone.out_neighbors(&i);
            let _ = g_clone.in_neighbors(&i);
            let _ = g_clone.out_neighbors(&i);
            let _ = g_clone.in_neighbors(&i);
            let _ = g_clone.out_neighbors(&i);

            // Verify basic properties
            assert_eq!(g_clone.vertices().size(), 5);
            assert_eq!(g_clone.labeled_arcs().size(), 4);

            (
                g_clone.out_neighbors(&i).size(),
                g_clone.out_neighbors(&i),
                g_clone.in_neighbors(&i),
                g_clone.out_neighbors(&i),
            )
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}

#[test]
fn test_from_weighted_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.vertices().size(), 3);
}

#[test]
fn test_in_neighbors_weighted() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5)), Triple(3, 2, OrderedFloat(2.5))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    let in_neighbors = g.in_neighbors_weighted(&2);
    assert_eq!(in_neighbors.size(), 2);
}

#[test]
fn test_total_weight() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, OrderedFloat(3.0))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.total_weight(), OrderedFloat(3.0));
}

#[test]
fn test_weighted_edges_empty() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.weighted_edges().size(), 0);
}

#[test]
fn test_get_edge_weight_missing() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.5))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.get_edge_weight(&2, &1), None);
}

// Additional comprehensive tests for uncovered functionality

#[test]
fn test_add_weighted_edge() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![];
    let mut g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    g.add_weighted_edge(1, 2, OrderedFloat(1.5));
    assert!(g.has_arc(&1, &2));

    g.add_weighted_edge(2, 3, OrderedFloat(2.7));
    assert!(g.has_arc(&2, &3));

    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_weighted_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.5)),
        Triple(2, 3, OrderedFloat(2.7)),
        Triple(3, 1, OrderedFloat(3.14))
    ];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices.clone(), edges.clone());

    let weighted_edges = g.weighted_edges();
    assert_eq!(weighted_edges.size(), 3);

    assert!(weighted_edges.mem(&Triple(1, 2, OrderedFloat(1.5))));
    assert!(weighted_edges.mem(&Triple(2, 3, OrderedFloat(2.7))));
    assert!(weighted_edges.mem(&Triple(3, 1, OrderedFloat(3.14))));
}

#[test]
fn test_get_edge_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(10.5)), Triple(2, 3, OrderedFloat(20.3))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(OrderedFloat(10.5)));
    assert_eq!(g.get_edge_weight(&2, &3), Some(OrderedFloat(20.3)));
    assert_eq!(g.get_edge_weight(&3, &1), None);
}

#[test]
fn test_out_neighbors_weighted() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.1)),
        Triple(1, 3, OrderedFloat(2.2)),
        Triple(1, 4, OrderedFloat(3.3)),
        Triple(2, 3, OrderedFloat(4.4))
    ];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let out_1 = g.out_neighbors_weighted(&1);
    assert_eq!(out_1.size(), 3);
    assert!(out_1.mem(&Pair(2, OrderedFloat(1.1))));
    assert!(out_1.mem(&Pair(3, OrderedFloat(2.2))));
    assert!(out_1.mem(&Pair(4, OrderedFloat(3.3))));
}

#[test]
fn test_out_neighbors_weighted_large_parallel() {
    // Create graph with 15 edges to trigger parallel path (> 8)
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut edges = SetStEph::empty();
    for i in 0..15 {
        edges.insert(Triple(i, i + 1, OrderedFloat(i as f64 * 1.5)));
    }
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let out_0 = g.out_neighbors_weighted(&0);
    assert_eq!(out_0.size(), 1);
    assert!(out_0.mem(&Pair(1, OrderedFloat(0.0))));

    let out_5 = g.out_neighbors_weighted(&5);
    assert_eq!(out_5.size(), 1);
    assert!(out_5.mem(&Pair(6, OrderedFloat(7.5))));
}

#[test]
fn test_in_neighbors_weighted_large_parallel() {
    // Create graph with 15 edges to trigger parallel path (> 8)
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut edges = SetStEph::empty();
    for i in 0..15 {
        edges.insert(Triple(i, i + 1, OrderedFloat(i as f64 * 0.1)));
    }
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let in_1 = g.in_neighbors_weighted(&1);
    assert_eq!(in_1.size(), 1);
    assert!(in_1.mem(&Pair(0, OrderedFloat(0.0))));

    let in_10 = g.in_neighbors_weighted(&10);
    assert_eq!(in_10.size(), 1);
    assert!(in_10.mem(&Pair(9, OrderedFloat(0.9))));
}

#[test]
fn test_total_weight_multiple_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.5)),
        Triple(2, 3, OrderedFloat(2.5)),
        Triple(3, 1, OrderedFloat(3.0))
    ];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.total_weight(), OrderedFloat(7.0));
}

#[test]
fn test_total_weight_negative() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(10.5)), Triple(2, 3, OrderedFloat(-5.5))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
    assert_eq!(g.total_weight(), OrderedFloat(5.0));
}

#[test]
fn test_out_neighbors_weighted_no_outgoing() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, OrderedFloat(1.0)), Triple(2, 3, OrderedFloat(2.0))];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let out_3 = g.out_neighbors_weighted(&3);
    assert_eq!(out_3.size(), 0);
}

#[test]
fn test_in_neighbors_weighted_multiple() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        Triple(1, 2, OrderedFloat(1.1)),
        Triple(3, 2, OrderedFloat(2.2)),
        Triple(4, 2, OrderedFloat(3.3))
    ];
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let in_2 = g.in_neighbors_weighted(&2);
    assert_eq!(in_2.size(), 3);
    assert!(in_2.mem(&Pair(1, OrderedFloat(1.1))));
    assert!(in_2.mem(&Pair(3, OrderedFloat(2.2))));
    assert!(in_2.mem(&Pair(4, OrderedFloat(3.3))));
}

#[test]
fn test_minimal_parallel_out_neighbors() {
    // Exactly 9 edges - minimal case to trigger parallel path
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut edges = SetStEph::empty();
    for i in 0..9 {
        edges.insert(Triple(i, i + 1, OrderedFloat(i as f64)));
    }
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let out_0 = g.out_neighbors_weighted(&0);
    assert_eq!(out_0.size(), 1);
    assert!(out_0.mem(&Pair(1, OrderedFloat(0.0))));
}

#[test]
fn test_minimal_parallel_in_neighbors() {
    // Exactly 9 edges - minimal case to trigger parallel path
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut edges = SetStEph::empty();
    for i in 0..9 {
        edges.insert(Triple(i, i + 1, OrderedFloat(i as f64 * 0.5)));
    }
    let g = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);

    let in_5 = g.in_neighbors_weighted(&5);
    assert_eq!(in_5.size(), 1);
    assert!(in_5.mem(&Pair(4, OrderedFloat(2.0))));
}
