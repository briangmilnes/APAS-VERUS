//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::{Arc, Barrier};
use std::thread;

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_verus::{LabDirGraphMtEphLit, SetLit};
use apas_verus::Types::Types::*;

#[test]
fn test_labdirgraphmtephlit_macro_functionality() {
    // Test empty graph creation
    let empty: LabDirGraphMtEph<i32, &str> = LabDirGraphMtEphLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with labeled edges
    let with_data = LabDirGraphMtEphLit!(
        V: [1, 2, 3],
        A: [(1, 2, "a"), (2, 3, "b"), (3, 1, "c")]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.arcs().size(), 3);
}

#[test]
fn test_labdirgraphmteph_empty() {
    let empty_graph = LabDirGraphMtEph::<i32, String>::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_arcs().size(), 0);
    assert_eq!(empty_graph.arcs().size(), 0);
}

#[test]
fn test_labdirgraphmteph_basic_operations() {
    let v: SetStEph<usize> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![
        LabEdge(0, 1, "edge01".to_string()),
        LabEdge(1, 2, "edge12".to_string()),
        LabEdge(2, 3, "edge23".to_string())
    ];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_arcs().size(), 3);

    // Test arc relationships
    assert!(g.has_arc(&0, &1));
    assert!(!g.has_arc(&1, &0)); // Directed graph
    assert!(g.has_arc(&1, &2));
    assert!(!g.has_arc(&2, &1));

    // Test n_plus (out-neighbors)
    let out0 = g.n_plus(&0);
    assert_eq!(out0.size(), 1);
    assert!(out0.mem(&1));

    let out1 = g.n_plus(&1);
    assert_eq!(out1.size(), 1);
    assert!(out1.mem(&2));

    // Test n_minus (in-neighbors)
    let in1 = g.n_minus(&1);
    assert_eq!(in1.size(), 1);
    assert!(in1.mem(&0));

    let in3 = g.n_minus(&3);
    assert_eq!(in3.size(), 1);
    assert!(in3.mem(&2));

    // Test arc labels
    assert_eq!(g.get_arc_label(&0, &1), Some(&"edge01".to_string()));
    assert_eq!(g.get_arc_label(&1, &2), Some(&"edge12".to_string()));
    assert_eq!(g.get_arc_label(&2, &3), Some(&"edge23".to_string()));
    assert_eq!(g.get_arc_label(&0, &2), None); // No direct arc
}

#[test]
fn test_labdirgraphmteph_mutable_operations() {
    let mut g = LabDirGraphMtEph::<i32, String>::empty();

    // Add vertices
    g.add_vertex(0);
    g.add_vertex(1);
    g.add_vertex(2);

    assert_eq!(g.vertices().size(), 3);
    assert!(g.vertices().mem(&0));
    assert!(g.vertices().mem(&1));
    assert!(g.vertices().mem(&2));

    // Add labeled arcs
    g.add_labeled_arc(0, 1, "first".to_string());
    g.add_labeled_arc(1, 2, "second".to_string());

    assert_eq!(g.labeled_arcs().size(), 2);
    assert!(g.has_arc(&0, &1));
    assert!(g.has_arc(&1, &2));
    assert!(!g.has_arc(&0, &2));

    // Test labels
    assert_eq!(g.get_arc_label(&0, &1), Some(&"first".to_string()));
    assert_eq!(g.get_arc_label(&1, &2), Some(&"second".to_string()));
}

#[test]
fn test_labdirgraphmteph_neighbors() {
    let v: SetStEph<usize> = SetLit![0, 1, 2, 3];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![
        LabEdge(0, 1, "a".to_string()),
        LabEdge(1, 2, "b".to_string()),
        LabEdge(2, 3, "c".to_string()),
        LabEdge(0, 3, "d".to_string())
    ];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    // Test n_plus (out-neighbors)
    let out0 = g.n_plus(&0);
    assert_eq!(out0.size(), 2);
    assert!(out0.mem(&1));
    assert!(out0.mem(&3));

    let out1 = g.n_plus(&1);
    assert_eq!(out1.size(), 1);
    assert!(out1.mem(&2));

    let out3 = g.n_plus(&3);
    assert_eq!(out3.size(), 0);

    // Test n_minus (in-neighbors)
    let in0 = g.n_minus(&0);
    assert_eq!(in0.size(), 0);

    let in1 = g.n_minus(&1);
    assert_eq!(in1.size(), 1);
    assert!(in1.mem(&0));

    let in3 = g.n_minus(&3);
    assert_eq!(in3.size(), 2);
    assert!(in3.mem(&0));
    assert!(in3.mem(&2));
}

#[test]
fn test_labdirgraphmteph_edge_cases() {
    // Test empty graph
    let empty = LabDirGraphMtEph::<i32, String>::empty();
    assert!(!empty.has_arc(&0, &1));
    assert_eq!(empty.n_plus(&0).size(), 0);
    assert_eq!(empty.n_minus(&0).size(), 0);
    assert_eq!(empty.get_arc_label(&0, &1), None);

    // Test single vertex
    let v_single: SetStEph<usize> = SetLit![42];
    let a_empty: SetStEph<LabEdge<usize, String>> = SetLit![];
    let g_single = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_arcs().size(), 0);
    assert_eq!(g_single.n_plus(&42).size(), 0);
    assert_eq!(g_single.n_minus(&42).size(), 0);

    // Test self-loop
    let v_self: SetStEph<usize> = SetLit![1];
    let a_self: SetStEph<LabEdge<usize, String>> = SetLit![LabEdge(1, 1, "self".to_string())];
    let g_self = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v_self, a_self);

    assert!(g_self.has_arc(&1, &1));
    assert_eq!(g_self.n_plus(&1).size(), 1);
    assert_eq!(g_self.n_minus(&1).size(), 1);
    assert_eq!(g_self.get_arc_label(&1, &1), Some(&"self".to_string()));
}

#[test]
fn test_labdirgraphmteph_nonexistent_vertex() {
    let v: SetStEph<usize> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![LabEdge(0, 1, "test".to_string())];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    // Query non-existent vertex
    assert!(!g.has_arc(&99, &0));
    assert_eq!(g.n_plus(&99).size(), 0);
    assert_eq!(g.n_minus(&99).size(), 0);
    assert_eq!(g.get_arc_label(&99, &0), None);
}

#[test]
fn test_labdirgraphmteph_concurrent_access() {
    let v: SetStEph<usize> = SetLit![0, 1, 2, 3, 4];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![
        LabEdge(0, 1, "a".to_string()),
        LabEdge(1, 2, "b".to_string()),
        LabEdge(2, 3, "c".to_string()),
        LabEdge(3, 4, "d".to_string())
    ];
    let g = Arc::new(LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a));

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
            let _ = g_clone.n_plus(&i);
            let _ = g_clone.n_minus(&i);
            let _ = g_clone.get_arc_label(&i, &(i + 1));

            // Verify basic properties
            assert_eq!(g_clone.vertices().size(), 5);
            assert_eq!(g_clone.labeled_arcs().size(), 4);

            (g_clone.n_plus(&i).size(), g_clone.n_minus(&i).size())
        }));
    }

    for handle in handles {
        let _ = handle.join().unwrap();
    }
}

#[test]
fn test_labdirgraphmteph_arcs_conversion() {
    let v: SetStEph<usize> = SetLit![0, 1, 2];
    let a: SetStEph<LabEdge<usize, String>> =
        SetLit![LabEdge(0, 1, "first".to_string()), LabEdge(1, 2, "second".to_string())];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    // Test arcs() method that converts labeled arcs to unlabeled edges
    let arcs = g.arcs();
    assert_eq!(arcs.size(), 2);
    assert!(arcs.mem(&Edge(0, 1)));
    assert!(arcs.mem(&Edge(1, 2)));
    assert!(!arcs.mem(&Edge(0, 2)));
}

#[test]
fn test_labdirgraphmteph_parallel_n_plus() {
    // Create graph with >8 arcs to trigger parallel code path
    let mut vertices = SetStEph::empty();
    for i in 0..20 {
        vertices.insert(i);
    }

    let mut arcs = SetStEph::empty();
    // Create 15 arcs from vertex 0 to vertices 1-15
    for i in 1..16 {
        arcs.insert(LabEdge(0, i, format!("arc{}", i)));
    }

    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(vertices, arcs);

    let out = g.n_plus(&0);
    assert_eq!(out.size(), 15);
    for i in 1..16 {
        assert!(out.mem(&i));
    }
}

#[test]
fn test_labdirgraphmteph_parallel_n_minus() {
    // Create graph with >8 arcs to trigger parallel code path
    let mut vertices = SetStEph::empty();
    for i in 0..20 {
        vertices.insert(i);
    }

    let mut arcs = SetStEph::empty();
    // Create 15 arcs from vertices 1-15 to vertex 0
    for i in 1..16 {
        arcs.insert(LabEdge(i, 0, format!("arc{}", i)));
    }

    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(vertices, arcs);

    let in_n = g.n_minus(&0);
    assert_eq!(in_n.size(), 15);
    for i in 1..16 {
        assert!(in_n.mem(&i));
    }
}

#[test]
fn test_labdirgraphmteph_parallel_mixed() {
    // Create large graph with multiple sources/sinks
    let mut vertices = SetStEph::empty();
    for i in 0..30 {
        vertices.insert(i);
    }

    let mut arcs = SetStEph::empty();
    // 10 arcs out from vertex 0
    for i in 1..11 {
        arcs.insert(LabEdge(0, i, format!("out{}", i)));
    }
    // 10 arcs in to vertex 29
    for i in 19..29 {
        arcs.insert(LabEdge(i, 29, format!("in{}", i)));
    }

    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(vertices, arcs);

    let out = g.n_plus(&0);
    assert_eq!(out.size(), 10);

    let in_n = g.n_minus(&29);
    assert_eq!(in_n.size(), 10);
}

#[test]
fn test_labdirgraphmteph_display_trait() {
    let v: SetStEph<usize> = SetLit![1, 2, 3];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![LabEdge(1, 2, "test".to_string())];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    let display_str = format!("{}", g);
    assert!(display_str.contains("LabDirGraph"));
}

#[test]
fn test_labdirgraphmteph_debug_trait() {
    let v: SetStEph<usize> = SetLit![1, 2];
    let a: SetStEph<LabEdge<usize, String>> = SetLit![LabEdge(1, 2, "test".to_string())];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    let debug_str = format!("{:?}", g);
    assert!(debug_str.contains("LabDirGraph"));
    assert!(debug_str.contains("vertices"));
    assert!(debug_str.contains("labeled_arcs"));
}

#[test]
fn test_labdirgraphmteph_clone() {
    let v: SetStEph<usize> = SetLit![1, 2, 3];
    let a: SetStEph<LabEdge<usize, String>> =
        SetLit![LabEdge(1, 2, "test".to_string()), LabEdge(2, 3, "test2".to_string())];
    let g = LabDirGraphMtEph::from_vertices_and_labeled_arcs(v, a);

    let g2 = g.clone();
    assert_eq!(g2.vertices().size(), 3);
    assert_eq!(g2.labeled_arcs().size(), 2);
    assert!(g2.has_arc(&1, &2));
    assert!(g2.has_arc(&2, &3));
}
