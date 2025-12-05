//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use apas_verus::UnDirGraphStEphLit;

#[test]
fn test_undirgraphstephlit_macro_functionality() {
    // Test empty graph creation
    let empty: UnDirGraphStEph<i32> = UnDirGraphStEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.edges().size(), 0);

    // Test graph creation with vertices and edges
    let with_data: UnDirGraphStEph<i32> = UnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2), (2, 3)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 2);
}

#[test]
fn test_undigraph_vertices_and_edges() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let e = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        let _ = s.insert(Edge(3, 3));
        s
    }; // allow self-loop representation
    let g = UnDirGraphStEph::from_sets(v.clone(), e.clone());
    assert_eq!(g.sizeV(), v.size());
    assert_eq!(g.sizeE(), e.size());
    assert_eq!(g.vertices(), &v);
    assert_eq!(g.edges(), &e);
}

#[test]
fn test_sizea() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);
    assert_eq!(g.sizeE(), 2);
}

#[test]
fn test_arcs() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e.clone());
    assert_eq!(g.edges(), &e);
}

#[test]
fn test_nplus() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);
    let nplus = g.ng(&1);
    assert!(nplus.mem(&2));
}

#[test]
fn test_nminus() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);
    let nminus = g.ng(&2);
    assert!(nminus.mem(&1));
}

#[test]
fn test_indegree() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);
    assert_eq!(g.degree(&2), 2);
}

#[test]
fn test_outdegree() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);
    assert_eq!(g.degree(&2), 2);
}

#[test]
fn test_empty() {
    let g = UnDirGraphStEph::<i32>::empty();
    assert_eq!(g.sizeV(), 0);
    assert_eq!(g.sizeE(), 0);
}

#[test]
fn test_neighbor() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert!(g.neighbor(&1, &2));
    assert!(g.neighbor(&2, &1)); // Undirected
    assert!(!g.neighbor(&1, &3));
}

#[test]
fn test_ng() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let ng2 = g.ng(&2);
    assert_eq!(ng2.size(), 2);
    assert!(ng2.mem(&1));
    assert!(ng2.mem(&3));
}

#[test]
fn test_ngofvertices() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let subset: SetStEph<N> = SetLit![1, 2];
    let ng = g.ng_of_vertices(&subset);
    assert!(ng.mem(&1));
    assert!(ng.mem(&2));
    assert!(ng.mem(&3));
}

#[test]
fn test_nplusofvertices() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let subset: SetStEph<N> = SetLit![1];
    let nplus = g.ng_of_vertices(&subset);
    assert!(nplus.mem(&2)); // Neighbor of 1
}

#[test]
fn test_nminusofvertices() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let subset: SetStEph<N> = SetLit![2];
    let nminus = g.ng_of_vertices(&subset);
    assert!(nminus.mem(&1)); // Neighbor of 2
    assert!(nminus.mem(&3)); // Neighbor of 2
}

#[test]
fn test_incident() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert!(g.incident(&Edge(1, 2), &1));
    assert!(g.incident(&Edge(1, 2), &2));
    assert!(!g.incident(&Edge(1, 2), &3));
}

#[test]
fn test_degree() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g.degree(&1), 1);
    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&3), 1);
}

#[test]
fn test_isolated_vertex() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g.degree(&3), 0);
    assert_eq!(g.ng(&3).size(), 0);
}

#[test]
fn test_self_loop() {
    let v: SetStEph<N> = SetLit![1];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 1)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert!(g.neighbor(&1, &1));
    assert_eq!(g.degree(&1), 1);
}

#[test]
fn test_complete_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(1, 3), Edge(2, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g.degree(&1), 2);
    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&3), 2);
}

#[test]
fn test_star_graph() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let e: SetStEph<Edge<N>> = SetLit![Edge(0, 1), Edge(0, 2), Edge(0, 3)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g.degree(&0), 3);
    assert_eq!(g.degree(&1), 1);
    assert_eq!(g.degree(&2), 1);
    assert_eq!(g.degree(&3), 1);
}

#[test]
fn test_path_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3), Edge(3, 4)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g.degree(&1), 1);
    assert_eq!(g.degree(&2), 2);
    assert_eq!(g.degree(&3), 2);
    assert_eq!(g.degree(&4), 1);
}

#[test]
fn test_cycle_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(2, 3), Edge(3, 4), Edge(4, 1)];
    let g = UnDirGraphStEph::from_sets(v, e);

    for i in 1..=4 {
        assert_eq!(g.degree(&i), 2);
    }
}

#[test]
fn test_disconnected_graph() {
    let v: SetStEph<N> = SetLit![1, 2, 3, 4];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2), Edge(3, 4)];
    let g = UnDirGraphStEph::from_sets(v, e);

    assert!(!g.neighbor(&1, &3));
    assert!(!g.neighbor(&2, &4));
}

#[test]
fn test_equality() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g1 = UnDirGraphStEph::from_sets(v.clone(), e.clone());
    let g2 = UnDirGraphStEph::from_sets(v, e);

    assert_eq!(g1, g2);
}

#[test]
fn test_display() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let s = format!("{g}");
    assert!(s.contains("V="));
}

#[test]
fn test_debug() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g = UnDirGraphStEph::from_sets(v, e);

    let s = format!("{g:?}");
    assert!(!s.is_empty());
}

#[test]
fn test_clone() {
    let v: SetStEph<N> = SetLit![1, 2];
    let e: SetStEph<Edge<N>> = SetLit![Edge(1, 2)];
    let g1 = UnDirGraphStEph::from_sets(v, e);
    let g2 = g1.clone();

    assert_eq!(g1, g2);
}
