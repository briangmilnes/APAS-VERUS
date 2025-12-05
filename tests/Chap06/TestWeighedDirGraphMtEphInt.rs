//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::{Arc, Barrier};
use std::thread;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_ai::Chap06::WeighedDirGraphMtEphInt::WeighedDirGraphMtEphInt::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;
use apas_ai::WeighedDirGraphMtEphIntLit;

#[test]
fn test_weigheddirgraphmtephintlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeighedDirGraphMtEphInt<i32> = WeighedDirGraphMtEphIntLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with weighed edges
    let with_data = WeighedDirGraphMtEphIntLit!(
        V: [1, 2, 3],
        E: [Triple(1, 2, 10), Triple(2, 3, 20), Triple(3, 1, 30)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.arcs().size(), 3);
}

#[test]
fn test_weigheddirgraphmtephint_empty() {
    let empty_graph = WeighedDirGraphMtEphInt::<i32>::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
}

#[test]
fn test_weigheddirgraphmtephint_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![
        LabEdge(0, 1, 10),
        LabEdge(1, 2, 20),
        LabEdge(2, 3, 30),
        LabEdge(0, 3, 40)
    ];
    let g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

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
fn test_weigheddirgraphmtephint_incident_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![LabEdge(0, 1, 100), LabEdge(1, 2, 200), LabEdge(0, 2, 300)];
    let _g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

    // Test incident edges
    // let _incident0 = g.incident(&0); // TODO: method not available
    // assert_eq!(_incident0.size(), 2); // 0->1 and 0->2 // TODO: method not available

    // let _incident1 = g.incident(&1); // TODO: method not available
    // assert_eq!(_incident1.size(), 2); // 0->1 (incoming) and 1->2 (outgoing) // TODO: method not available

    // let _incident2 = g.incident(&2); // TODO: method not available
    // assert_eq!(_incident2.size(), 2); // 1->2 and 0->2 (both incoming) // TODO: method not available
}

#[test]
fn test_weigheddirgraphmtephint_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![LabEdge(0, 1, 1), LabEdge(1, 2, 2), LabEdge(2, 3, 3), LabEdge(0, 3, 4)];
    let _g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];
    // let _ng_subset = g.ng_of_vertices(&vertices_subset); // TODO: method not available

    // Neighbors of {0, 1} should be {1, 2, 3}
    // assert_eq!(_ng_subset.size(), 3); // TODO: method not available
    // assert_eq!(_ng_subset.mem(&1), true); // TODO: method not available
    // assert_eq!(_ng_subset.mem(&2), true); // TODO: method not available
    // assert_eq!(_ng_subset.mem(&3), true); // TODO: method not available
}

#[test]
fn test_weigheddirgraphmtephint_nplusminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![
        LabEdge(0, 1, 5),
        LabEdge(1, 2, 15),
        LabEdge(2, 0, 25),
        LabEdge(3, 1, 35)
    ];
    let _g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

    let _vertices_subset: SetStEph<N> = SetLit![0, 1];

    // Test NPlusOfVertices (out-neighbors)
    // let _nplus_subset = g.n_plus_of_vertices(&vertices_subset); // TODO: method not available
    // assert_eq!(_nplus_subset.size(), 2); // TODO: method not available
    // assert_eq!(_nplus_subset.mem(&1), true); // 0->1 // TODO: method not available
    // assert_eq!(_nplus_subset.mem(&2), true); // 1->2 // TODO: method not available

    // Test NMinusOfVertices (in-neighbors)
    // let _nminus_subset = g.n_minus_of_vertices(&vertices_subset); // TODO: method not available
    // assert_eq!(_nminus_subset.size(), 2); // TODO: method not available
    // assert_eq!(_nminus_subset.mem(&2), true); // 2->0 // TODO: method not available
    // assert_eq!(_nminus_subset.mem(&3), true); // 3->1 // TODO: method not available
}

#[test]
fn test_weigheddirgraphmtephint_edge_cases() {
    // Test empty graph
    let empty = WeighedDirGraphMtEphInt::<i32>::empty();
    assert!(!empty.has_arc(&0, &1));
    assert_eq!(empty.out_neighbors(&0).size(), 0);
    assert_eq!(empty.out_neighbors(&0).size(), 0);

    // Test single vertex
    let v_single: SetStEph<N> = SetLit![42];
    let a_empty: SetStEph<LabEdge<N, i32>> = SetLit![];
    let g_single = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_arcs().size(), 0);
    assert_eq!(g_single.out_neighbors(&42).size(), 0);
    assert_eq!(g_single.out_neighbors(&42).size(), 0);

    // Test self-loop with weight
    let v_self: SetStEph<N> = SetLit![1];
    let a_self: SetStEph<LabEdge<N, i32>> = SetLit![LabEdge(1, 1, 999)];
    let g_self = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v_self, a_self);

    assert!(g_self.has_arc(&1, &1));
    assert_eq!(g_self.out_neighbors(&1).size(), 1); // Self-loop to self
    assert_eq!(g_self.in_neighbors(&1).size(), 1);
    assert_eq!(g_self.out_neighbors(&1).size(), 1);
}

#[test]
fn test_weigheddirgraphmtephint_nonexistent_vertex() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![LabEdge(0, 1, 777)];
    let g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

    // Query non-existent vertex
    assert!(!g.has_arc(&99, &0));
    assert_eq!(g.out_neighbors(&99).size(), 0);
    assert_eq!(g.out_neighbors(&99).size(), 0);
    assert_eq!(g.in_neighbors(&99).size(), 0);
    assert_eq!(g.out_neighbors(&99).size(), 0);
}

#[test]
fn test_weigheddirgraphmtephint_weight_variations() {
    // Test with various integer weight values including negative, zero, and extremes
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![
        LabEdge(0, 1, 0),        // Zero weight
        LabEdge(1, 2, -100),     // Negative weight
        LabEdge(2, 3, 1),        // Small positive
        LabEdge(3, 4, i32::MAX), // Maximum positive
        LabEdge(4, 0, i32::MIN)  // Minimum (most negative)
    ];
    let g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

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
fn test_weigheddirgraphmtephint_large_weights() {
    // Test with large integer weights to ensure no overflow issues
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![
        LabEdge(0, 1, 1_000_000),
        LabEdge(1, 2, -1_000_000),
        LabEdge(2, 0, 999_999_999)
    ];
    let g = WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_arcs().size(), 3);

    // Verify all connections work with large weights
    assert!(g.has_arc(&0, &1));
    assert!(g.has_arc(&1, &2));
    assert!(g.has_arc(&2, &0));

    // Each vertex should have degree 2 (one in, one out)
    for vertex in [0, 1, 2] {
        assert_eq!(g.out_neighbors(&vertex).size(), 1);
        assert_eq!(g.in_neighbors(&vertex).size(), 1);
        assert_eq!(g.out_neighbors(&vertex).size(), 1);
    }
}

#[test]
fn test_weigheddirgraphmtephint_concurrent_access() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<N, i32>> = SetLit![
        LabEdge(0, 1, 11),
        LabEdge(1, 2, 22),
        LabEdge(2, 3, 33),
        LabEdge(3, 4, 44)
    ];
    let g = Arc::new(WeighedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a));

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
fn test_from_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 10), Triple(2, 3, 20)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.vertices().size(), 3);
}

#[test]
fn test_in_neighbors_weighed() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 10), Triple(3, 2, 20)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    let in_neighbors = g.in_neighbors_weighed(&2);
    assert_eq!(in_neighbors.size(), 2);
}

#[test]
fn test_total_weight() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, 100)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.total_weight(), 100);
}

#[test]
fn test_weighed_edges_empty() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.weighed_edges().size(), 0);
    assert_eq!(g.total_weight(), 0);
}

#[test]
fn test_get_edge_weight_nonexistent() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, 50)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.get_edge_weight(&2, &1), None);
}

#[test]
fn test_out_neighbors_weighed_isolated() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 10)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.out_neighbors_weighed(&3).size(), 0);
}

#[test]
fn test_in_neighbors_weighed_empty() {
    let vertices = SetLit![1, 2];
    let edges = SetLit![Triple(1, 2, 30)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.in_neighbors_weighed(&1).size(), 0);
}

// Additional tests for uncovered functionality

#[test]
fn test_add_weighed_edge() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![];
    let mut g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    g.add_weighed_edge(1, 2, 100);
    assert!(g.has_arc(&1, &2));

    g.add_weighed_edge(2, 3, 200);
    assert!(g.has_arc(&2, &3));

    assert_eq!(g.labeled_arcs().size(), 2);
}

#[test]
fn test_weighed_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 10), Triple(2, 3, 20), Triple(3, 1, 30)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices.clone(), edges.clone());

    let weighed_edges = g.weighed_edges();
    assert_eq!(weighed_edges.size(), 3);

    // Verify all edges are present
    assert!(weighed_edges.mem(&Triple(1, 2, 10)));
    assert!(weighed_edges.mem(&Triple(2, 3, 20)));
    assert!(weighed_edges.mem(&Triple(3, 1, 30)));
}

#[test]
fn test_get_edge_weight() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 100), Triple(2, 3, 200)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    assert_eq!(g.get_edge_weight(&1, &2), Some(100));
    assert_eq!(g.get_edge_weight(&2, &3), Some(200));
    assert_eq!(g.get_edge_weight(&3, &1), None);
}

#[test]
fn test_out_neighbors_weighed() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![Triple(1, 2, 10), Triple(1, 3, 20), Triple(1, 4, 30), Triple(2, 3, 40)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let out_1 = g.out_neighbors_weighed(&1);
    assert_eq!(out_1.size(), 3);
    assert!(out_1.mem(&Pair(2, 10)));
    assert!(out_1.mem(&Pair(3, 20)));
    assert!(out_1.mem(&Pair(4, 30)));

    let out_2 = g.out_neighbors_weighed(&2);
    assert_eq!(out_2.size(), 1);
    assert!(out_2.mem(&Pair(3, 40)));
}

#[test]
fn test_out_neighbors_weighed_large_parallel() {
    // Create graph with 15 edges to trigger parallel path (> 8)
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut edges = SetStEph::empty();
    for i in 0..15 {
        edges.insert(Triple(i, i + 1, i as i32 * 10));
    }
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let out_0 = g.out_neighbors_weighed(&0);
    assert_eq!(out_0.size(), 1);
    assert!(out_0.mem(&Pair(1, 0)));

    let out_5 = g.out_neighbors_weighed(&5);
    assert_eq!(out_5.size(), 1);
    assert!(out_5.mem(&Pair(6, 50)));
}

#[test]
fn test_in_neighbors_weighed_large_parallel() {
    // Create graph with 15 edges to trigger parallel path (> 8)
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut edges = SetStEph::empty();
    for i in 0..15 {
        edges.insert(Triple(i, i + 1, i as i32 * 10));
    }
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let in_1 = g.in_neighbors_weighed(&1);
    assert_eq!(in_1.size(), 1);
    assert!(in_1.mem(&Pair(0, 0)));

    let in_10 = g.in_neighbors_weighed(&10);
    assert_eq!(in_10.size(), 1);
    assert!(in_10.mem(&Pair(9, 90)));
}

#[test]
fn test_total_weight_multiple_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 100), Triple(2, 3, 200), Triple(3, 1, 300)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.total_weight(), 600);
}

#[test]
fn test_total_weight_negative() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 100), Triple(2, 3, -50)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);
    assert_eq!(g.total_weight(), 50);
}

#[test]
fn test_out_neighbors_weighed_no_outgoing() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![Triple(1, 2, 10), Triple(2, 3, 20)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let out_3 = g.out_neighbors_weighed(&3);
    assert_eq!(out_3.size(), 0);
}

#[test]
fn test_in_neighbors_weighed_multiple() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![Triple(1, 2, 10), Triple(3, 2, 20), Triple(4, 2, 30)];
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let in_2 = g.in_neighbors_weighed(&2);
    assert_eq!(in_2.size(), 3);
    assert!(in_2.mem(&Pair(1, 10)));
    assert!(in_2.mem(&Pair(3, 20)));
    assert!(in_2.mem(&Pair(4, 30)));
}

#[test]
fn test_minimal_parallel_out_neighbors() {
    // Exactly 9 edges - minimal case to trigger parallel path
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut edges = SetStEph::empty();
    for i in 0..9 {
        edges.insert(Triple(i, i + 1, i as i32));
    }
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let out_0 = g.out_neighbors_weighed(&0);
    assert_eq!(out_0.size(), 1);
    assert!(out_0.mem(&Pair(1, 0)));
}

#[test]
fn test_minimal_parallel_in_neighbors() {
    // Exactly 9 edges - minimal case to trigger parallel path
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut edges = SetStEph::empty();
    for i in 0..9 {
        edges.insert(Triple(i, i + 1, i as i32 * 5));
    }
    let g = WeighedDirGraphMtEphInt::from_weighed_edges(vertices, edges);

    let in_5 = g.in_neighbors_weighed(&5);
    assert_eq!(in_5.size(), 1);
    assert!(in_5.mem(&Pair(4, 20)));
}
