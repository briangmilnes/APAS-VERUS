//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Multi-threaded Ephemeral Tests

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap62::StarPartitionMtEph::StarPartitionMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_cycle_graph(n: N) -> UnDirGraphMtEph<N> {
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
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::from_sets(vertices, edges)
}

fn create_star_graph(n: N) -> UnDirGraphMtEph<N> {
    if n == 0 {
        return <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::empty();
    }
    let mut vertices = SetLit![0]; // Center vertex
    for i in 1..n {
        let _ = vertices.insert(i); // Satellite vertices
    }
    let mut edges = SetLit![];
    for i in 1..n {
        let _ = edges.insert(Edge(0, i)); // Edges from center to satellites
    }
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::from_sets(vertices, edges)
}

#[test]
fn test_parallel_star_partition_cycle() {
    let graph = create_cycle_graph(8);
    let (centers, partition_map) = parallel_star_partition(&graph, 123);

    // Verify partition map covers all vertices
    assert_eq!(partition_map.len(), 8);

    // Verify every vertex maps to a center
    for v in 0..8 {
        let center = partition_map.get(&v).unwrap();
        assert!(centers.mem(center));
    }

    // Verify centers map to themselves
    for center in centers.iter() {
        assert_eq!(partition_map.get(center), Some(center));
    }
}

#[test]
fn test_parallel_star_partition_star() {
    let graph = create_star_graph(5);
    let (centers, partition_map) = parallel_star_partition(&graph, 456);

    // All vertices should be in the partition
    assert_eq!(partition_map.len(), 5);

    // Verify centers map to themselves
    for center in centers.iter() {
        assert_eq!(partition_map.get(center), Some(center));
    }
}

#[test]
fn test_determinism() {
    let graph = create_cycle_graph(6);

    // Same seed should give same result
    let (centers1, map1) = parallel_star_partition(&graph, 789);
    let (centers2, map2) = parallel_star_partition(&graph, 789);

    assert_eq!(centers1.size(), centers2.size());
    for v in 0..6 {
        assert_eq!(map1.get(&v), map2.get(&v));
    }
}

#[test]
fn test_partition_validity() {
    let graph = create_cycle_graph(10);
    let (centers, partition_map) = parallel_star_partition(&graph, 999);

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
