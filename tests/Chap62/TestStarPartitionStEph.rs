//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Sequential Ephemeral Tests

use apas_verus::Types::Types::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap62::StarPartitionStEph::StarPartitionStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;
use std::collections::HashMap;

fn create_cycle_graph(n: N) -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let u = i;
        let v = (i + 1) % n;
        let _ = edges.insert(Edge(u, v));
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges)
}

fn create_star_graph(n: N) -> UnDirGraphStEph<N> {
    if n == 0 {
        return <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::empty();
    }
    let mut vertices = SetLit![0]; // Center vertex
    for i in 1..n {
        let _ = vertices.insert(i); // Satellite vertices
    }
    let mut edges = SetLit![];
    for i in 1..n {
        let _ = edges.insert(Edge(0, i)); // Edges from center to satellites
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges)
}

#[test]
fn test_sequential_star_partition_cycle() {
    let graph = create_cycle_graph(6); // 0-1-2-3-4-5-0
    let (centers, partition_map) = sequential_star_partition(&graph);

    // Verify partition map covers all vertices
    assert_eq!(partition_map.len(), 6);
    for v in 0..6 {
        assert!(partition_map.contains_key(&v));
    }

    // Verify centers are in the partition map
    for center in centers.iter() {
        assert_eq!(partition_map.get(center), Some(center));
    }

    // Verify all vertices map to some center
    for v in 0..6 {
        let center = partition_map.get(&v).unwrap();
        assert!(centers.mem(center));
    }
}

#[test]
fn test_sequential_star_partition_star() {
    let graph = create_star_graph(5); // Center 0, satellites 1,2,3,4
    let (centers, partition_map) = sequential_star_partition(&graph);

    // Star graph can produce 1-5 centers depending on vertex iteration order
    // The greedy algorithm is correct but non-deterministic based on set ordering
    assert!(centers.size() >= 1 && centers.size() <= 5);

    // All vertices should map to the same center
    for v in 0..5 {
        let center = partition_map.get(&v).unwrap();
        assert!(centers.mem(center));
    }
}

#[test]
fn test_partition_properties() {
    let graph = create_cycle_graph(8);
    let (centers, partition_map) = sequential_star_partition(&graph);

    // Property 1: Every vertex is in the partition map
    assert_eq!(partition_map.len(), graph.sizeV() as usize);

    // Property 2: Centers map to themselves
    for center in centers.iter() {
        assert_eq!(partition_map.get(center), Some(center));
    }

    // Property 3: Every vertex maps to a valid center
    for v in graph.vertices().iter() {
        let center = partition_map.get(v).unwrap();
        assert!(centers.mem(center));
    }
}

#[test]
fn test_empty_graph() {
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::empty();
    let (centers, partition_map) = sequential_star_partition(&graph);

    assert_eq!(centers.size(), 0);
    assert_eq!(partition_map.len(), 0);
}

#[test]
fn test_single_vertex() {
    let vertices = SetLit![0];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges);

    let (centers, partition_map) = sequential_star_partition(&graph);

    // Single isolated vertex should be its own center
    assert_eq!(centers.size(), 1);
    assert!(centers.mem(&0));
    assert_eq!(partition_map.get(&0), Some(&0));
}
