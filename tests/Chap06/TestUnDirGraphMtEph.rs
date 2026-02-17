//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::{Arc, Barrier};
use std::thread;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::UnDirGraphMtEphLit;

#[test]
fn test_undirgraphmtephlit_macro_functionality() {
    // Test empty graph creation
    let empty: UnDirGraphMtEph<i32> = UnDirGraphMtEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.edges().size(), 0);

    // Test graph creation with vertices and edges
    let with_data: UnDirGraphMtEph<i32> = UnDirGraphMtEphLit!(
        V: [1, 2, 3],
        E: [(1, 2), (2, 3)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 2);
}

#[test]
fn test_undirgraphmteph_empty() {
    let empty_graph = UnDirGraphMtEph::<i32>::empty();
    assert_eq!(empty_graph.sizeV(), 0);
    assert_eq!(empty_graph.sizeE(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.edges().size(), 0);
}

#[test]
fn test_undirgraphmteph_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    assert_eq!(g.sizeV(), 4);
    assert_eq!(g.sizeE(), 3);

    // Test neighbor relationships (undirected - both directions)
    assert!(g.neighbor(&0, &1));
    assert!(g.neighbor(&1, &0)); // Undirected graph
    assert!(g.neighbor(&1, &2));
    assert!(g.neighbor(&2, &1));
    assert!(!g.neighbor(&0, &2)); // No direct edge

    // Test NG (neighbors) - should be symmetric
    let ng0 = g.ng(&0);
    assert_eq!(ng0.size(), 1);
    assert!(ng0.mem(&1));

    let ng1 = g.ng(&1);
    assert_eq!(ng1.size(), 2);
    assert!(ng1.mem(&0));
    assert!(ng1.mem(&2));

    let ng2 = g.ng(&2);
    assert_eq!(ng2.size(), 2);
    assert!(ng2.mem(&1));
    assert!(ng2.mem(&3));

    // Test degrees (in undirected graph, InDegree = OutDegree = Degree)
    assert_eq!(g.degree(&0), 1);
    assert_eq!(g.degree(&0), 1);
    assert_eq!(g.degree(&0), 1);

    assert_eq!(g.degree(&1), 2);
    assert_eq!(g.degree(&1), 2);
    assert_eq!(g.degree(&1), 2);

    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&2), 2);

    assert_eq!(g.degree(&3), 1);
    assert_eq!(g.degree(&3), 1);
    assert_eq!(g.degree(&3), 1);
}

#[test]
fn test_undirgraphmteph_incident_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(0, 2)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    // Test incident edges
    assert!(g.incident(&Edge(0, 1), &0));
    assert!(g.incident(&Edge(0, 1), &1));
    assert!(!g.incident(&Edge(0, 1), &2));

    assert!(g.incident(&Edge(1, 2), &1));
    assert!(g.incident(&Edge(1, 2), &2));
    assert!(!g.incident(&Edge(1, 2), &0));

    // let incident0 = g.incident(&Edge(0, 1), &0); // TODO: fix incident edge tests
    // assert_eq!(incident0, true); // 0-1 incident to 0

    // let incident1 = g.incident(&Edge(0, 1), &1); // TODO: fix incident edge tests
    // assert_eq!(incident1, true); // 0-1 incident to 1

    // let incident2 = g.incident(&Edge(0, 2), &2); // TODO: fix incident edge tests
    // assert_eq!(incident2, false); // 0-2 not in graph
}

#[test]
fn test_ng_parallel_with_many_edges() {
    // Force parallel path by having more than 8 edges
    let vertices = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut edges = SetLit![];

    // Create many edges from vertex 0
    for i in 1..=10 {
        edges.insert(Edge(0, i));
    }

    let g = UnDirGraphMtEph::from_sets(vertices, edges);
    let ng = g.ng(&0);

    assert_eq!(ng.size(), 10); // 0 has 10 neighbors
}

#[test]
fn test_ng_of_vertices_parallel() {
    // Create graph with enough complexity to trigger parallel paths
    let mut vertices = SetLit![];
    for i in 0..15 {
        vertices.insert(i);
    }

    let mut edges = SetLit![];
    for i in 0..10 {
        edges.insert(Edge(i, i + 1));
    }

    let g = UnDirGraphMtEph::from_sets(vertices.clone(), edges);

    let u_set = SetLit![0, 5, 10];
    let ng = g.ng_of_vertices(&u_set);

    // Each vertex in u_set has neighbors
    assert!(ng.size() > 0);
}

#[test]
fn test_ng_single_edge() {
    let v: SetStEph<N> = SetLit![0, 1];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    let ng = g.ng(&0);
    assert_eq!(ng.size(), 1);
    assert!(ng.mem(&1));
}

#[test]
fn test_ng_of_vertices_empty_set() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    let empty_set: SetStEph<N> = SetLit![];
    let ng = g.ng_of_vertices(&empty_set);

    assert_eq!(ng.size(), 0);
}

#[test]
fn test_ng_of_vertices_large_set() {
    // Force parallel path with > 8 vertices
    let mut vertices = SetLit![];
    for i in 0..20 {
        vertices.insert(i);
    }

    let mut edges = SetLit![];
    for i in 0..15 {
        edges.insert(Edge(i, i + 1));
    }

    let g = UnDirGraphMtEph::from_sets(vertices, edges);

    // Query all vertices (> 8, triggers parallel)
    let mut u_set = SetLit![];
    for i in 0..20 {
        u_set.insert(i);
    }

    let ng = g.ng_of_vertices(&u_set);
    assert!(ng.size() > 0);
}

#[test]
fn test_clone_graph() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    let g2 = g.clone();
    assert_eq!(g.sizeV(), g2.sizeV());
    assert_eq!(g.sizeE(), g2.sizeE());
}

#[test]
fn test_undirgraphmteph_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 3), Edge(0, 3)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    let vertices_subset: SetStEph<N> = SetLit![0, 1];
    let ng_subset = g.ng_of_vertices(&vertices_subset);

    // Neighbors of {0, 1} includes all vertices connected to 0 or 1 (including 0 and 1 themselves)
    assert_eq!(ng_subset.size(), 4); // Should be {0, 1, 2, 3}
    assert!(ng_subset.mem(&0)); // 0 is connected to 1 and 3
    assert!(ng_subset.mem(&1)); // 1 is connected to 0 and 2
    assert!(ng_subset.mem(&2)); // 1-2
    assert!(ng_subset.mem(&3)); // 0-3
}

#[test]
fn test_undirgraphmteph_nplusminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 0), Edge(3, 1)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    let vertices_subset: SetStEph<N> = SetLit![0, 1];

    // In undirected graphs, NPlus and NMinus should be the same as NG
    let ng_subset = g.ng_of_vertices(&vertices_subset);

    // Neighbors of {0, 1} includes all vertices connected to 0 or 1 (including 0 and 1 themselves)
    assert_eq!(ng_subset.size(), 4); // Should include all vertices 0, 1, 2, 3
    assert!(ng_subset.mem(&0)); // 0 is connected to 1 and 2
    assert!(ng_subset.mem(&1)); // 1 is connected to 0, 2, and 3
    assert!(ng_subset.mem(&2)); // Connected to both 0 and 1
    assert!(ng_subset.mem(&3)); // Connected to 1

    // In undirected graphs, all neighbors are both in and out neighbors
}

#[test]
fn test_undirgraphmteph_edge_cases() {
    // Test empty graph
    let empty = UnDirGraphMtEph::<i32>::empty();
    assert!(!empty.neighbor(&0, &1));
    assert_eq!(empty.ng(&0).size(), 0);
    assert_eq!(empty.degree(&0), 0);

    // Test single vertex
    let v_single: SetStEph<N> = SetLit![42];
    let a_empty: SetStEph<Edge<N>> = SetLit![];
    let g_single = UnDirGraphMtEph::from_sets(v_single, a_empty);

    assert_eq!(g_single.sizeV(), 1);
    assert_eq!(g_single.sizeE(), 0);
    assert_eq!(g_single.degree(&42), 0);
    assert_eq!(g_single.ng(&42).size(), 0);

    // Test self-loop
    let v_self: SetStEph<N> = SetLit![1];
    let a_self: SetStEph<Edge<N>> = SetLit![Edge(1, 1)];
    let g_self = UnDirGraphMtEph::from_sets(v_self, a_self);

    assert!(g_self.neighbor(&1, &1));
    // In undirected graph, self-loop contributes 1 to degree (based on neighbors implementation)
    assert_eq!(g_self.degree(&1), 1);
    assert_eq!(g_self.degree(&1), 1);
    assert_eq!(g_self.degree(&1), 1);
}

#[test]
fn test_undirgraphmteph_nonexistent_vertex() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    // Query non-existent vertex
    assert!(!g.neighbor(&99, &0));
    assert_eq!(g.ng(&99).size(), 0);
    assert_eq!(g.degree(&99), 0);
    assert_eq!(g.degree(&99), 0);
    assert_eq!(g.degree(&99), 0);
}

#[test]
fn test_undirgraphmteph_concurrent_access() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<Edge<N>> = SetLit![
        Edge(0, 1),
        Edge(1, 2),
        Edge(2, 3),
        Edge(3, 4),
        Edge(0, 4) // Additional edge for more interesting topology
    ];
    let g = Arc::new(UnDirGraphMtEph::from_sets(v, a));

    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = vec![];
    for i in 0..num_threads {
        let g_clone = Arc::clone(&g);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait(); // Wait for all threads to be ready

            // Perform various read operations concurrently
            let _ = g_clone.neighbor(&i, &(i + 1));
            let _ = g_clone.ng(&i);
            let _ = g_clone.ng(&i);
            let _ = g_clone.ng(&i); // In undirected graphs, in/out neighbors are the same
            let _ = g_clone.degree(&i);
            let _ = g_clone.degree(&i);
            let _ = g_clone.degree(&i);

            // Verify basic properties
            assert_eq!(g_clone.sizeV(), 5);
            assert_eq!(g_clone.sizeE(), 5);

            // In undirected graph, InDegree should equal OutDegree
            assert_eq!(g_clone.degree(&i), g_clone.degree(&i)); // In undirected graphs, degree is the same

            (
                g_clone.ng(&i).size(),
                g_clone.degree(&i),
                g_clone.degree(&i),
                g_clone.degree(&i),
            )
        }));
    }

    for handle in handles {
        let (_ng_size, degree, in_degree, out_degree) = handle.join().unwrap();
        // Verify undirected graph properties
        assert_eq!(in_degree, out_degree);
        assert_eq!(degree, in_degree); // In undirected graphs, degree = in_degree = out_degree
    }
}

#[test]
fn test_undirgraphmteph_complete_graph() {
    // Test complete graph K4
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(0, 2), Edge(0, 3), Edge(1, 2), Edge(1, 3), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, a);

    assert_eq!(g.sizeV(), 4);
    assert_eq!(g.sizeE(), 6);

    // Every vertex should have degree 3 in K4
    for vertex in [0, 1, 2, 3] {
        assert_eq!(g.degree(&vertex), 3);
        assert_eq!(g.ng(&vertex).size(), 3);
    }

    // Every pair should be neighbors
    for i in [0, 1, 2, 3] {
        for j in [0, 1, 2, 3] {
            if i != j {
                assert!(g.neighbor(&i, &j));
            }
        }
    }
}

#[test]
fn test_sizea() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);
    assert_eq!(g.sizeE(), 1);
}

#[test]
fn test_nplus() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);
    assert!(g.ng(&1).mem(&2));
}

#[test]
fn test_nminus() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);
    assert!(g.ng(&2).mem(&1));
}

#[test]
fn test_indegree() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e);
    assert_eq!(g.degree(&2), 2);
}

#[test]
fn test_outdegree() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e);
    assert_eq!(g.degree(&2), 2);
}

#[test]
fn test_arcs() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e.clone());
    assert_eq!(g.edges(), &e);
    assert_eq!(g.edges().size(), 2);
}

#[test]
fn test_nplusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    let vertices_subset: SetStEph<N> = SetLit![0, 1];
    let nplus = g.ng_of_vertices(&vertices_subset);

    // Should include all neighbors of 0 and 1
    assert!(nplus.mem(&0));
    assert!(nplus.mem(&1));
    assert!(nplus.mem(&2));
}

#[test]
fn test_nminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    let vertices_subset: SetStEph<N> = SetLit![1, 2];
    let nminus = g.ng_of_vertices(&vertices_subset);

    // In undirected graph, same as NGOfVertices
    assert!(nminus.mem(&0));
    assert!(nminus.mem(&1));
    assert!(nminus.mem(&2));
    assert!(nminus.mem(&3));
}

#[test]
fn test_incident() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // Edge(1, 2) is incident to both 1 and 2
    assert!(g.incident(&Edge(1, 2), &1));
    assert!(g.incident(&Edge(1, 2), &2));
    assert!(!g.incident(&Edge(1, 2), &3));

    // Edge(2, 3) is incident to both 2 and 3
    assert!(g.incident(&Edge(2, 3), &2));
    assert!(g.incident(&Edge(2, 3), &3));
    assert!(!g.incident(&Edge(2, 3), &1));
}

#[test]
fn test_empty_graph_all_operations() {
    let g = UnDirGraphMtEph::<i32>::empty();

    assert_eq!(g.sizeV(), 0);
    assert_eq!(g.sizeE(), 0);
    assert_eq!(g.sizeE(), 0);
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.edges().size(), 0);
    assert_eq!(g.edges().size(), 0);

    assert!(!g.neighbor(&1, &2));
    assert_eq!(g.ng(&1).size(), 0);
    assert_eq!(g.degree(&1), 0);
    assert_eq!(g.degree(&1), 0);
    assert_eq!(g.degree(&1), 0);

    let empty_set: SetStEph<i32> = SetLit![];
    assert_eq!(g.ng_of_vertices(&empty_set).size(), 0);
    assert_eq!(g.ng_of_vertices(&empty_set).size(), 0);
    assert_eq!(g.ng_of_vertices(&empty_set).size(), 0);
}

#[test]
fn test_isolated_vertices() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4, 5];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // Vertices 3, 4, 5 are isolated
    assert_eq!(g.degree(&3), 0);
    assert_eq!(g.degree(&4), 0);
    assert_eq!(g.degree(&5), 0);
    assert_eq!(g.ng(&3).size(), 0);
    assert_eq!(g.ng(&4).size(), 0);
    assert_eq!(g.ng(&5).size(), 0);

    // Vertices 1, 2 are connected
    assert_eq!(g.degree(&1), 1);
    assert_eq!(g.degree(&2), 1);
    assert!(g.ng(&1).mem(&2));
    assert!(g.ng(&2).mem(&1));
}

#[test]
fn test_disconnected_components() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(3, 4)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // Component 1: {1, 2}
    assert!(g.neighbor(&1, &2));
    assert!(!g.neighbor(&1, &3));
    assert!(!g.neighbor(&1, &4));

    // Component 2: {3, 4}
    assert!(g.neighbor(&3, &4));
    assert!(!g.neighbor(&3, &1));
    assert!(!g.neighbor(&3, &2));

    // NGOfVertices across components
    let component1: SetStEph<N> = SetLit![1, 2];
    let ng1 = g.ng_of_vertices(&component1);
    assert!(ng1.mem(&1));
    assert!(ng1.mem(&2));
    assert!(!ng1.mem(&3)); // Different component
    assert!(!ng1.mem(&4)); // Different component
}

#[test]
fn test_star_graph() {
    // Star graph: one central vertex connected to all others
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(0, 2), Edge(0, 3), Edge(0, 4)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // Center has degree 4
    assert_eq!(g.degree(&0), 4);

    // All others have degree 1
    for i in 1..=4 {
        assert_eq!(g.degree(&i), 1);
        assert!(g.ng(&i).mem(&0));
    }

    // No edges between non-center vertices
    assert!(!g.neighbor(&1, &2));
    assert!(!g.neighbor(&2, &3));
}

#[test]
fn test_cycle_graph() {
    // Cycle graph: 0-1-2-3-0
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(1, 2), Edge(2, 3), Edge(3, 0)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // All vertices have degree 2 in a cycle
    for i in 0..=3 {
        assert_eq!(g.degree(&i), 2);
    }

    // Check cycle connectivity
    assert!(g.neighbor(&0, &1));
    assert!(g.neighbor(&1, &2));
    assert!(g.neighbor(&2, &3));
    assert!(g.neighbor(&3, &0));

    // No chords in simple cycle
    assert!(!g.neighbor(&0, &2));
    assert!(!g.neighbor(&1, &3));
}

#[test]
fn test_path_graph() {
    // Path graph: 1-2-3-4-5
    let v: SetStEph<N> = SetLit![1, 2, 3, 4, 5];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3), Edge(3, 4), Edge(4, 5)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    // End vertices have degree 1
    assert_eq!(g.degree(&1), 1);
    assert_eq!(g.degree(&5), 1);

    // Middle vertices have degree 2
    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&3), 2);
    assert_eq!(g.degree(&4), 2);
}

#[test]
fn test_large_parallel_graph() {
    // Create a larger graph to trigger parallel operations
    let mut v = SetStEph::<N>::empty();
    let mut e = SetStEph::<Edge<N>>::empty();

    // Create a grid-like graph
    for i in 0..20 {
        v.insert(i);
    }

    for i in 0..19 {
        e.insert(Edge(i, i + 1));
    }

    let g = UnDirGraphMtEph::from_sets(v, e);

    assert_eq!(g.sizeV(), 20);
    assert_eq!(g.sizeE(), 19);

    // Test parallel NG operation
    assert_eq!(g.degree(&10), 2); // Middle vertex
    assert_eq!(g.degree(&0), 1); // End vertex
    assert_eq!(g.degree(&19), 1); // End vertex

    // Test parallel NGOfVertices
    let subset: SetStEph<N> = SetLit![5, 10, 15];
    let ng = g.ng_of_vertices(&subset);
    assert!(ng.size() >= 6); // At least neighbors of the 3 vertices
}

#[test]
fn test_equality() {
    let v1: SetStEph<N> = SetLit![1, 2, 3];
    let e1: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g1 = UnDirGraphMtEph::from_sets(v1.clone(), e1.clone());
    let g2 = UnDirGraphMtEph::from_sets(v1, e1);

    assert_eq!(g1, g2);

    let v3: SetStEph<N> = SetLit![1, 2, 3];
    let e3: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g3 = UnDirGraphMtEph::from_sets(v3, e3);

    assert_ne!(g1, g3);
}

#[test]
fn test_display_format() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    let display_str = format!("{g}");
    assert!(display_str.contains("V="));
    assert!(display_str.contains("E="));
}

#[test]
fn test_debug_format() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    let debug_str = format!("{g:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn test_edge_order_symmetry() {
    // In undirected graph, Edge(1,2) and Edge(2,1) should be treated as equivalent
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e1: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphMtEph::from_sets(v, e1);

    // Both orderings should return true
    assert!(g.neighbor(&1, &2));
    assert!(g.neighbor(&2, &1));
    assert!(g.neighbor(&2, &3));
    assert!(g.neighbor(&3, &2));
}

#[test]
fn test_ngofvertices_empty_subset() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    let empty_set: SetStEph<N> = SetLit![];
    let result = g.ng_of_vertices(&empty_set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_multiple_self_loops() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 1), Edge(2, 2)];
    let g = UnDirGraphMtEph::from_sets(v, e);

    assert!(g.neighbor(&1, &1));
    assert!(g.neighbor(&2, &2));
    assert!(!g.neighbor(&1, &2));
}

#[test]
fn test_clone() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g1 = UnDirGraphMtEph::from_sets(v, e);
    let g2 = g1.clone();

    assert_eq!(g1, g2);
    assert_eq!(g1.sizeV(), g2.sizeV());
    assert_eq!(g1.sizeE(), g2.sizeE());
}
