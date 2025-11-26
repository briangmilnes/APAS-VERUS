//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for LabUnDirGraphMtEph - ALL trait methods

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_ai::Types::Types::*;
use apas_ai::SetLit;
use std::hash::Hash;

// Helper to check if set contains element
fn set_contains<T: StT + Ord + Hash>(s: &SetStEph<T>, elem: &T) -> bool {
    s.mem(elem)
}

#[test]
fn test_empty() {
    let g = LabUnDirGraphMtEph::<i32, String>::empty();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_edges().size(), 0);
}

#[test]
fn test_from_vertices_and_labeled_edges() {
    let vertices = SetLit![1, 2, 3];
    let labeled_edges = SetLit![
        LabEdge(1, 2, "ab".to_string()),
        LabEdge(2, 3, "bc".to_string())
    ];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(vertices, labeled_edges);
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 2);
}

#[test]
fn test_vertices() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_vertex(1);
    g.add_vertex(2);
    let v = g.vertices();
    assert_eq!(v.size(), 2);
    assert!(set_contains(v, &1));
    assert!(set_contains(v, &2));
}

#[test]
fn test_labeled_edges() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "test".to_string());
    let edges = g.labeled_edges();
    assert_eq!(edges.size(), 1);
}

#[test]
fn test_edges() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "a".to_string());
    g.add_labeled_edge(2, 3, "b".to_string());
    let edges = g.edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_add_vertex() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_vertex(5);
    g.add_vertex(10);
    assert_eq!(g.vertices().size(), 2);
    assert!(set_contains(g.vertices(), &5));
    assert!(set_contains(g.vertices(), &10));
}

#[test]
fn test_add_labeled_edge() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "edge_label".to_string());
    assert_eq!(g.labeled_edges().size(), 1);
    assert_eq!(g.vertices().size(), 2); // Should auto-add vertices
}

#[test]
fn test_get_edge_label() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "label1".to_string());
    g.add_labeled_edge(2, 3, "label2".to_string());
    
    let label = g.get_edge_label(&1, &2);
    assert!(label.is_some());
    assert_eq!(label.unwrap(), &"label1".to_string());
    
    // Undirected, so order shouldn't matter
    let label_rev = g.get_edge_label(&2, &1);
    assert!(label_rev.is_some());
    assert_eq!(label_rev.unwrap(), &"label1".to_string());
    
    // Non-existent edge
    let no_label = g.get_edge_label(&1, &3);
    assert!(no_label.is_none());
}

#[test]
fn test_has_edge() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "test".to_string());
    
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Undirected
    assert!(!g.has_edge(&1, &3));
    assert!(!g.has_edge(&2, &3));
}

#[test]
fn test_neighbors() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "a".to_string());
    g.add_labeled_edge(1, 3, "b".to_string());
    g.add_labeled_edge(2, 4, "c".to_string());
    
    let neighbors_1 = g.neighbors(&1);
    assert_eq!(neighbors_1.size(), 2);
    assert!(set_contains(&neighbors_1, &2));
    assert!(set_contains(&neighbors_1, &3));
    
    let neighbors_2 = g.neighbors(&2);
    assert_eq!(neighbors_2.size(), 2);
    assert!(set_contains(&neighbors_2, &1));
    assert!(set_contains(&neighbors_2, &4));
}

#[test]
#[should_panic(expected = "normalize_edge cannot create LabEdge without a label")]
fn test_normalize_edge() {
    // This method is broken by design - it panics
    // Including test to cover the panic path
    type TestGraph = LabUnDirGraphMtEph<i32, String>;
    let _ = TestGraph::normalize_edge(1, 2);
}

#[test]
fn test_empty_graph_operations() {
    let g = LabUnDirGraphMtEph::<i32, String>::empty();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_edges().size(), 0);
    assert_eq!(g.edges().size(), 0);
    assert!(!g.has_edge(&1, &2));
    assert_eq!(g.get_edge_label(&1, &2), None);
    assert_eq!(g.neighbors(&1).size(), 0);
}

#[test]
fn test_multiple_edges() {
    let mut g = LabUnDirGraphMtEph::<i32, String>::empty();
    g.add_labeled_edge(1, 2, "a".to_string());
    g.add_labeled_edge(2, 3, "b".to_string());
    g.add_labeled_edge(3, 4, "c".to_string());
    g.add_labeled_edge(4, 1, "d".to_string());
    
    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_edges().size(), 4);
    assert_eq!(g.edges().size(), 4);
}
