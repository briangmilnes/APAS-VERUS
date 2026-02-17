//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap52 AdjTableGraphStPer.

use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap43::OrderedTableStPer::OrderedTableStPer::*;
use apas_verus::Chap52::AdjTableGraphStPer::AdjTableGraphStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let g = AdjTableGraphStPer::<i32>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_from_table() {
    let mut table = OrderedTableStPer::empty();
    table = table.insert(1, AVLTreeSetStPer::singleton(2));
    let g = AdjTableGraphStPer::from_table(table);
    assert_eq!(g.num_vertices(), 1);
}

#[test]
fn test_num_vertices() {
    let g = AdjTableGraphStPer::<i32>::empty();
    assert_eq!(g.num_vertices(), 0);
    let g = g.insert_vertex(1);
    assert_eq!(g.num_vertices(), 1);
    let g = g.insert_vertex(2);
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_num_edges() {
    let g = AdjTableGraphStPer::<i32>::empty();
    assert_eq!(g.num_edges(), 0);
    let g = g.insert_edge(1, 2);
    assert_eq!(g.num_edges(), 1);
    let g = g.insert_edge(2, 3);
    assert_eq!(g.num_edges(), 2);
}

#[test]
fn test_vertices() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let vertices = g.vertices();
    assert_eq!(vertices.size(), 0);

    let g = g.insert_vertex(1).insert_vertex(2).insert_vertex(3);
    let vertices = g.vertices();
    assert_eq!(vertices.size(), 3);
    assert!(vertices.find(&1));
    assert!(vertices.find(&2));
    assert!(vertices.find(&3));
}

#[test]
fn test_has_edge() {
    let g = AdjTableGraphStPer::<i32>::empty();
    assert!(!g.has_edge(&1, &2));

    let g = g.insert_edge(1, 2);
    assert!(g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &1));
}

#[test]
fn test_out_neighbors() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 0);

    let g = g.insert_edge(1, 2).insert_edge(1, 3);
    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 2);
    assert!(neighbors.find(&2));
    assert!(neighbors.find(&3));
}

#[test]
fn test_out_degree() {
    let g = AdjTableGraphStPer::<i32>::empty();
    assert_eq!(g.out_degree(&1), 0);

    let g = g.insert_edge(1, 2).insert_edge(1, 3).insert_edge(1, 4);
    assert_eq!(g.out_degree(&1), 3);
    assert_eq!(g.out_degree(&2), 0);
}

#[test]
fn test_insert_vertex() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_vertex(1);
    assert_eq!(g.num_vertices(), 1);
    assert!(g.vertices().find(&1));

    let g = g.insert_vertex(2);
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_delete_vertex() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_edge(1, 2).insert_edge(2, 3).insert_edge(3, 1);

    let g = g.delete_vertex(&2);
    assert_eq!(g.num_vertices(), 2);
    assert!(!g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &3));
}

#[test]
fn test_insert_edge() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_edge(1, 2);
    assert!(g.has_edge(&1, &2));
    assert_eq!(g.num_edges(), 1);

    let g = g.insert_edge(1, 3);
    assert!(g.has_edge(&1, &3));
    assert_eq!(g.num_edges(), 2);
}

#[test]
fn test_delete_edge() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_edge(1, 2);
    assert!(g.has_edge(&1, &2));

    let g = g.delete_edge(&1, &2);
    assert!(!g.has_edge(&1, &2));
}

#[test]
fn test_insert_edge_creates_vertices() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_edge(1, 2);
    assert_eq!(g.num_vertices(), 2);
    assert!(g.vertices().find(&1));
    assert!(g.vertices().find(&2));
}

#[test]
fn test_delete_edge_nonexistent() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g.insert_vertex(1);
    let g2 = g.delete_edge(&1, &2);
    assert_eq!(g2.num_vertices(), g.num_vertices());
}

#[test]
fn test_complex_graph() {
    let g = AdjTableGraphStPer::<i32>::empty();
    let g = g
        .insert_edge(1, 2)
        .insert_edge(1, 3)
        .insert_edge(2, 3)
        .insert_edge(3, 4);

    assert_eq!(g.num_vertices(), 4);
    assert_eq!(g.num_edges(), 4);
    assert_eq!(g.out_degree(&1), 2);
    assert_eq!(g.out_degree(&2), 1);
    assert_eq!(g.out_degree(&3), 1);
    assert_eq!(g.out_degree(&4), 0);
}
