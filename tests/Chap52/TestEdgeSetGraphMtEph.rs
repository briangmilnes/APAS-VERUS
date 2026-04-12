//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for EdgeSetGraphMtEph

use std::sync::{Arc, Mutex};
use std::time::Duration;

use apas_verus::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
use apas_verus::Chap52::EdgeSetGraphMtEph::EdgeSetGraphMtEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let g = EdgeSetGraphMtEph::<i32>::empty();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_from_vertices_and_edges() {
    let mut v = AVLTreeSetMtEph::empty();
    v.insert(1i32);
    v.insert(2i32);
    v.insert(3i32);

    let mut e = AVLTreeSetMtEph::empty();
    e.insert(Pair(1i32, 2i32));
    e.insert(Pair(2i32, 3i32));

    let g = EdgeSetGraphMtEph::from_vertices_and_edges(v, e);
    assert_eq!(g.num_vertices(), 3);
    assert_eq!(g.num_edges(), 2);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &3));
    assert!(!g.has_edge(&1, &3));
}

#[test]
fn test_vertices_accessor() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);

    let vertices = g.vertices();
    assert_eq!(vertices.size(), 2);
    assert!(vertices.find(&1));
    assert!(vertices.find(&2));
    assert!(!vertices.find(&3));
}

#[test]
fn test_edges_accessor() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_edge(1, 2);
    g.insert_edge(2, 3);

    let edges = g.edges();
    assert_eq!(edges.size(), 2);
    assert!(edges.find(&Pair(1, 2)));
    assert!(edges.find(&Pair(2, 3)));
    assert!(!edges.find(&Pair(1, 3)));
}

#[test]
fn test_default() {
    let g = EdgeSetGraphMtEph::<i32>::default();
    assert_eq!(g.num_vertices(), 0);
    assert_eq!(g.num_edges(), 0);
}

#[test]
fn test_insert_vertex() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);

    g.insert_vertex(2);
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_insert_edge() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    assert_eq!(g.num_edges(), 1);
    assert!(g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &1));
}

#[test]
fn test_insert_edge_adds_vertices() {
    // insert_edge auto-inserts both endpoints.
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_edge(10, 20);

    assert!(g.vertices().find(&10));
    assert!(g.vertices().find(&20));
    assert_eq!(g.num_edges(), 1);
}

#[test]
fn test_delete_vertex_removes_incident_edges() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    g.delete_vertex(&1);
    assert_eq!(g.num_vertices(), 1);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(&1, &2));
}

#[test]
fn test_delete_edge() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_edge(1, 2);

    g.delete_edge(&1, &2);
    assert_eq!(g.num_edges(), 0);
    assert!(!g.has_edge(&1, &2));
    // Vertices remain after edge deletion.
    assert_eq!(g.num_vertices(), 2);
}

#[test]
fn test_out_neighbors() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);

    let neighbors = g.out_neighbors(&1);
    assert_eq!(neighbors.size(), 2);
    assert!(neighbors.find(&2));
    assert!(neighbors.find(&3));
    assert!(!neighbors.find(&1));
}

#[test]
fn test_out_degree() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.insert_vertex(2);
    g.insert_vertex(3);
    g.insert_edge(1, 2);
    g.insert_edge(1, 3);

    assert_eq!(g.out_degree(&1), 2);
    assert_eq!(g.out_degree(&2), 0);
}

#[test]
fn test_idempotent_insert() {
    // Inserting the same vertex or edge twice should not increase counts.
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(5);
    g.insert_vertex(5);
    assert_eq!(g.num_vertices(), 1);

    g.insert_edge(5, 5);
    g.insert_edge(5, 5);
    assert_eq!(g.num_edges(), 1);
}

#[test]
fn test_delete_nonexistent() {
    // Deleting an absent vertex or edge should not panic.
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    g.insert_vertex(1);
    g.delete_vertex(&99);
    assert_eq!(g.num_vertices(), 1);

    g.insert_edge(1, 2);
    g.delete_edge(&9, &9);
    assert_eq!(g.num_edges(), 1);
}

/// Stress test: build a graph concurrently using independent clones, then merge.
#[test]
fn test_concurrent_build_stress() {
    let result = std::thread::Builder::new()
        .name("concurrent_build".to_string())
        .spawn(|| {
            let n = 20usize;
            // Build n independent graphs in parallel, each adding one edge.
            let handles: Vec<_> = (0..n)
                .map(|i| {
                    std::thread::spawn(move || {
                        let mut g = EdgeSetGraphMtEph::<i32>::empty();
                        g.insert_edge(i as i32, (i + 1) as i32);
                        (i as i32, (i + 1) as i32, g.num_edges())
                    })
                })
                .collect();

            let mut all_ok = true;
            for h in handles {
                let (_, _, edge_count) = h.join().expect("thread panicked");
                if edge_count != 1 {
                    all_ok = false;
                }
            }
            all_ok
        })
        .expect("spawn failed");

    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        if result.is_finished() {
            break;
        }
        if std::time::Instant::now() > deadline {
            panic!("test_concurrent_build_stress timed out after 5s");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(result.join().expect("thread panicked"));
}

/// Stress test: multiple threads read a shared graph (Arc + Mutex for mutation).
#[test]
fn test_shared_graph_concurrent_reads() {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    for i in 0..10i32 {
        g.insert_edge(i, i + 1);
    }
    let shared = Arc::new(Mutex::new(g));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let shared = Arc::clone(&shared);
            std::thread::spawn(move || {
                let g = shared.lock().unwrap();
                let count = g.num_edges();
                count
            })
        })
        .collect();

    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    for h in handles {
        loop {
            if h.is_finished() {
                break;
            }
            if std::time::Instant::now() > deadline {
                panic!("test_shared_graph_concurrent_reads timed out");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        let count = h.join().expect("thread panicked");
        assert_eq!(count, 10);
    }
}
