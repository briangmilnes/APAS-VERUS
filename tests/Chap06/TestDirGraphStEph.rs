//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::DirGraphStEph::DirGraphStEph::*;
use apas_verus::{DirGraphStEphLit, SetLit};
use apas_verus::Types::Types::*;

#[test]
fn test_dirgraphstephlit_macro_functionality() {
    // Test empty graph creation
    let empty: DirGraphStEph<i32> = DirGraphStEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.arcs().size(), 0);

    // Test graph creation with vertices and arcs
    let with_data: DirGraphStEph<i32> = DirGraphStEphLit!(
        V: [1, 2, 3],
        A: [(1, 2), (2, 3)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.arcs().size(), 2);
}

#[test]
fn test_digraph_vertices_and_arcs() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        let _ = s.insert(Edge(3, 3));
        s
    }; // includes self-loop (3,3)
    let g = DirGraphStEph::FromSets(v.clone(), a.clone());
    assert_eq!(g.sizeV(), v.size());
    assert_eq!(g.sizeA(), a.size());
    assert_eq!(g.vertices(), &v);
    assert_eq!(g.arcs(), &a);
}

#[test]
fn test_dirgraph_empty() {
    let empty_graph = DirGraphStEph::<i32>::empty();
    assert_eq!(empty_graph.sizeV(), 0);
    assert_eq!(empty_graph.sizeA(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.arcs().size(), 0);
}

#[test]
fn test_dirgraph_neighbor() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(0, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    // Test Neighbor method - checks if edge exists between two vertices
    assert!(g.Neighbor(&0, &1)); // edge 0->1 exists
    assert!(g.Neighbor(&0, &2)); // edge 0->2 exists
    assert!(g.Neighbor(&1, &2)); // edge 1->2 exists
    assert!(!g.Neighbor(&1, &0)); // edge 1->0 does not exist
    assert!(!g.Neighbor(&2, &0)); // edge 2->0 does not exist
    assert!(!g.Neighbor(&2, &1)); // edge 2->1 does not exist
}

#[test]
fn test_dirgraph_ng() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let ng_0 = g.NG(&0);
    assert_eq!(ng_0.size(), 1);
    assert!(ng_0.mem(&1));

    let ng_2 = g.NG(&2);
    assert_eq!(ng_2.size(), 1); // vertex 2 has incoming neighbor 1
    assert!(ng_2.mem(&1));
}

#[test]
fn test_dirgraph_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let vertices_subset = SetLit![0, 1];
    let ng_subset = g.NGOfVertices(&vertices_subset);
    assert_eq!(ng_subset.size(), 3); // NG(0)={1} ∪ NG(1)={0,2} = {0,1,2}
    assert!(ng_subset.mem(&0));
    assert!(ng_subset.mem(&1));
    assert!(ng_subset.mem(&2));
}

#[test]
fn test_dirgraph_nplus() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let nplus_0 = g.NPlus(&0);
    assert_eq!(nplus_0.size(), 1);
    assert!(nplus_0.mem(&1));

    let nplus_2 = g.NPlus(&2);
    assert_eq!(nplus_2.size(), 0);
}

#[test]
fn test_dirgraph_nminus() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let nminus_1 = g.NMinus(&1);
    assert_eq!(nminus_1.size(), 1);
    assert!(nminus_1.mem(&0));

    let nminus_0 = g.NMinus(&0);
    assert_eq!(nminus_0.size(), 0);
}

#[test]
fn test_dirgraph_nplusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let vertices_subset = SetLit![0, 1];
    let nplus_subset = g.NPlusOfVertices(&vertices_subset);
    assert_eq!(nplus_subset.size(), 2);
    assert!(nplus_subset.mem(&1));
    assert!(nplus_subset.mem(&2));
}

#[test]
fn test_dirgraph_nminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let vertices_subset = SetLit![1, 2];
    let nminus_subset = g.NMinusOfVertices(&vertices_subset);
    assert_eq!(nminus_subset.size(), 2);
    assert!(nminus_subset.mem(&0));
    assert!(nminus_subset.mem(&1));
}

#[test]
fn test_dirgraph_incident() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    // Test Incident method - checks if edge is incident to vertex
    assert!(g.Incident(&Edge(0, 1), &0)); // edge (0,1) is incident to vertex 0
    assert!(g.Incident(&Edge(0, 1), &1)); // edge (0,1) is incident to vertex 1
    assert!(!g.Incident(&Edge(0, 1), &2)); // edge (0,1) is not incident to vertex 2
    assert!(g.Incident(&Edge(1, 2), &1)); // edge (1,2) is incident to vertex 1
    assert!(g.Incident(&Edge(1, 2), &2)); // edge (1,2) is incident to vertex 2
}

#[test]
fn test_dirgraph_degree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    assert_eq!(g.Degree(&0), 2); // one in + one out = 2
    assert_eq!(g.Degree(&1), 2); // one in + one out = 2
    assert_eq!(g.Degree(&2), 2); // one in + one out = 2
}

#[test]
fn test_dirgraph_indegree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    assert_eq!(g.InDegree(&0), 1); // edge from 2
    assert_eq!(g.InDegree(&1), 1); // edge from 0
    assert_eq!(g.InDegree(&2), 1); // edge from 1
}

#[test]
fn test_dirgraph_outdegree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    assert_eq!(g.OutDegree(&0), 1); // edge to 1
    assert_eq!(g.OutDegree(&1), 1); // edge to 2
    assert_eq!(g.OutDegree(&2), 1); // edge to 0
}

#[test]
fn test_dirgraph_empty_graph_edge_cases() {
    let empty_graph = DirGraphStEph::<i32>::empty();

    // All operations on empty graph should return empty sets, false, or 0
    assert!(!empty_graph.Neighbor(&42, &99));

    let ng = empty_graph.NG(&42);
    assert_eq!(ng.size(), 0);

    let nplus = empty_graph.NPlus(&42);
    assert_eq!(nplus.size(), 0);

    let nminus = empty_graph.NMinus(&42);
    assert_eq!(nminus.size(), 0);

    assert!(empty_graph.Incident(&Edge(42, 99), &42)); // Incident always returns True for any edge-vertex pair

    assert_eq!(empty_graph.Degree(&42), 0);
    assert_eq!(empty_graph.InDegree(&42), 0);
    assert_eq!(empty_graph.OutDegree(&42), 0);
}

#[test]
fn test_dirgraph_single_vertex_edge_cases() {
    let v: SetStEph<N> = SetLit![42];
    let a = SetStEph::<Edge<N>>::empty();
    let g = DirGraphStEph::FromSets(v, a);

    // Single vertex with no edges
    assert_eq!(g.sizeV(), 1);
    assert_eq!(g.sizeA(), 0);

    assert!(!g.Neighbor(&42, &42)); // no self-loop

    assert_eq!(g.Degree(&42), 0);
    assert_eq!(g.InDegree(&42), 0);
    assert_eq!(g.OutDegree(&42), 0);
}

#[test]
fn test_dirgraph_selfloop_edge_cases() {
    let v: SetStEph<N> = SetLit![0];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    // Self-loop should be handled correctly
    assert!(g.Neighbor(&0, &0)); // self-loop exists

    let ng_0 = g.NG(&0);
    assert_eq!(ng_0.size(), 1);
    assert!(ng_0.mem(&0));

    assert_eq!(g.Degree(&0), 2); // self-loop: in-degree 1 + out-degree 1 = 2
    assert_eq!(g.InDegree(&0), 1);
    assert_eq!(g.OutDegree(&0), 1);
}

#[test]
fn test_dirgraph_nonexistent_vertex_edge_cases() {
    let v: SetStEph<N> = SetLit![0, 1];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    // Queries for non-existent vertex should return false/empty/0
    assert!(!g.Neighbor(&99, &0));
    assert!(!g.Neighbor(&0, &99));

    let ng_99 = g.NG(&99);
    assert_eq!(ng_99.size(), 0);

    assert_eq!(g.Degree(&99), 0);
    assert_eq!(g.InDegree(&99), 0);
    assert_eq!(g.OutDegree(&99), 0);
}

#[test]
fn test_dirgraph_extreme_vertex_references_graceful() {
    // Test with extreme vertex values to verify no panics occur
    // APAS style: bad arguments produce empty sequences/sets, not panics

    let v: SetStEph<i32> = SetLit![0, 1, i32::MAX, i32::MIN];
    let a = {
        let mut s = SetStEph::<Edge<i32>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(i32::MAX, i32::MIN));
        let _ = s.insert(Edge(i32::MIN, 0));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    // Test operations with extreme values - should not panic
    assert!(g.Neighbor(&i32::MAX, &i32::MIN));
    assert!(g.Neighbor(&i32::MIN, &0));
    assert!(!g.Neighbor(&i32::MAX, &0));

    // Test degree operations with extreme values
    assert!(g.Degree(&i32::MAX) >= 1);
    assert!(g.Degree(&i32::MIN) >= 1);
    assert!(g.InDegree(&i32::MIN) >= 1);
    assert!(g.OutDegree(&i32::MAX) >= 1);

    // Test with non-existent extreme values - should return graceful defaults
    assert!(!g.Neighbor(&(i32::MAX - 1), &0));
    assert_eq!(g.Degree(&(i32::MIN + 1)), 0);

    let ng_extreme = g.NG(&(i32::MAX - 1));
    assert_eq!(ng_extreme.size(), 0);
}

#[test]
fn test_dirgraph_large_graph_stress() {
    // Test with large graph to verify no panics occur
    let vertices = (0..1000).collect::<Vec<i32>>();
    let v = SetStEph::<i32>::FromVec(vertices);

    // Create edges: each vertex connects to next vertex (0->1, 1->2, ..., 998->999)
    let mut a = SetStEph::<Edge<i32>>::empty();
    for i in 0..999 {
        let _ = a.insert(Edge(i, i + 1));
    }
    // Add some random edges for more complexity
    for i in (0..1000).step_by(100) {
        let _ = a.insert(Edge(i, (i + 500) % 1000));
    }

    let g = DirGraphStEph::FromSets(v, a);

    assert_eq!(g.sizeV(), 1000);
    assert!(g.sizeA() >= 999); // At least the chain edges

    // Test operations on large graph - should not panic
    assert!(g.Neighbor(&0, &1));
    assert!(!g.Neighbor(&999, &0));

    // Test degree operations
    assert!(g.OutDegree(&0) >= 1);
    assert!(g.InDegree(&999) >= 1);
    assert_eq!(g.InDegree(&0), 1); // Only receives from vertex 900 (if exists)

    // Test with vertices in the middle of the chain
    assert!(g.Neighbor(&500, &501));
    assert!(g.Degree(&500) >= 2); // At least in-degree 1 and out-degree 1

    // Test NGOfVertices with subset
    let subset: SetStEph<i32> = SetLit![0, 1, 2, 3, 4];
    let ng_subset = g.NGOfVertices(&subset);
    assert!(ng_subset.size() >= 4); // At least vertices 1,2,3,4 are neighbors

    // Test with non-existent vertices - should return graceful defaults
    assert!(!g.Neighbor(&2000, &0));
    assert_eq!(g.Degree(&2000), 0);
}

#[test]
fn test_dirgraph_clone() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);
    let g2 = g.clone();

    assert_eq!(g.sizeV(), g2.sizeV());
    assert_eq!(g.sizeA(), g2.sizeA());
    assert!(g2.Neighbor(&1, &2));
    assert!(g2.Neighbor(&2, &3));
}

#[test]
fn test_dirgraph_debug_display() {
    let v: SetStEph<N> = SetLit![1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphStEph::FromSets(v, a);

    let debug_str = format!("{:?}", g);
    assert!(!debug_str.is_empty());

    let display_str = format!("{}", g);
    assert!(!display_str.is_empty());
}

#[test]
fn test_dirgraph_equality() {
    let v1: SetStEph<N> = SetLit![1, 2];
    let a1 = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g1 = DirGraphStEph::FromSets(v1.clone(), a1.clone());

    let g2 = DirGraphStEph::FromSets(v1, a1);
    assert_eq!(g1, g2);

    let v3: SetStEph<N> = SetLit![1, 2, 3];
    let g3 = DirGraphStEph::FromSets(v3, SetStEph::empty());
    assert_ne!(g1, g3);
}
