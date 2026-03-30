//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Tests

use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap63::ConnectivityMtEph::ConnectivityMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_connected_graph() -> UnDirGraphMtEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..6 {
        let _ = edges.insert(Edge(i, (i + 1) % 6));
    }
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

fn create_multi_component_graph() -> UnDirGraphMtEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..8 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(5, 6));
    let _ = edges.insert(Edge(6, 7));
    <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges)
}

#[test]
fn test_count_components_mt_single() {
    let graph = create_connected_graph();
    let count = count_components_mt(&graph, 123);
    assert_eq!(count, 1);
}

#[test]
fn test_count_components_mt_multiple() {
    let graph = create_multi_component_graph();
    let count = count_components_mt(&graph, 456);
    assert_eq!(count, 3);
}

#[test]
fn test_connected_components_mt_single() {
    let graph = create_connected_graph();
    let (reps, comp_map) = connected_components_mt(&graph, 123);

    assert_eq!(reps.size(), 1);

    let first_comp = comp_map.get(&0).unwrap();
    for i in 1..6 {
        assert_eq!(comp_map.get(&i).unwrap(), first_comp);
    }
}

#[test]
fn test_connected_components_mt_multiple() {
    let graph = create_multi_component_graph();
    let (reps, comp_map) = connected_components_mt(&graph, 789);

    assert_eq!(reps.size(), 3);

    // Vertices in same component map to same representative
    let comp0 = comp_map.get(&0).unwrap();
    assert_eq!(comp_map.get(&1).unwrap(), comp0);
    assert_eq!(comp_map.get(&2).unwrap(), comp0);

    let comp3 = comp_map.get(&3).unwrap();
    assert_eq!(comp_map.get(&4).unwrap(), comp3);

    let comp5 = comp_map.get(&5).unwrap();
    assert_eq!(comp_map.get(&6).unwrap(), comp5);
    assert_eq!(comp_map.get(&7).unwrap(), comp5);

    assert_ne!(comp0, comp3);
    assert_ne!(comp0, comp5);
    assert_ne!(comp3, comp5);
}

#[test]
fn test_count_components_hof_mt() {
    let graph = create_multi_component_graph();
    let count_hof = count_components_hof(&graph, 999);
    let count_direct = count_components_mt(&graph, 999);
    assert_eq!(count_hof, count_direct);
    assert_eq!(count_hof, 3);
}

#[test]
fn test_connected_components_hof_mt() {
    let graph = create_multi_component_graph();
    let (reps_hof, comp_map_hof) = connected_components_hof(&graph, 1111);
    let (reps_direct, _comp_map_direct) = connected_components_mt(&graph, 1111);

    // Should produce same number of components
    assert_eq!(reps_hof.size(), reps_direct.size());
    assert_eq!(reps_hof.size(), 3);

    // Verify same component structure
    // Vertices in same component map to same representative
    let comp0 = comp_map_hof.get(&0).unwrap();
    assert_eq!(comp_map_hof.get(&1).unwrap(), comp0);
    assert_eq!(comp_map_hof.get(&2).unwrap(), comp0);

    let comp3 = comp_map_hof.get(&3).unwrap();
    assert_eq!(comp_map_hof.get(&4).unwrap(), comp3);

    let comp5 = comp_map_hof.get(&5).unwrap();
    assert_eq!(comp_map_hof.get(&6).unwrap(), comp5);
    assert_eq!(comp_map_hof.get(&7).unwrap(), comp5);

    assert_ne!(comp0, comp3);
    assert_ne!(comp0, comp5);
    assert_ne!(comp3, comp5);
}

#[test]
fn test_count_components_mt_empty() {
    let vertices: SetStEph<usize> = SetLit![];
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 0);
}

#[test]
fn test_count_components_mt_isolated() {
    let mut vertices = SetLit![];
    for i in 0..4 {
        let _ = vertices.insert(i);
    }
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 4);
}

#[test]
fn test_count_components_mt_single_vertex() {
    let vertices = SetLit![42];
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 1);
}

#[test]
fn test_connected_components_mt_complete() {
    let mut vertices = SetLit![];
    for i in 0..5usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..5usize {
        for j in (i + 1)..5 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components_mt(&graph, 42);
    assert_eq!(reps.size(), 1);
    let rep = comp_map.get(&0).unwrap();
    for i in 1..5 {
        assert_eq!(comp_map.get(&i).unwrap(), rep);
    }
}

#[test]
fn test_count_components_mt_path() {
    let mut vertices = SetLit![];
    for i in 0..10 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..9usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 1);
}

#[test]
fn test_count_components_mt_star() {
    let mut vertices = SetLit![0];
    for i in 1..8usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..8usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 1);
}

#[test]
fn test_count_components_mt_two_triangles() {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(4, 5));
    let _ = edges.insert(Edge(3, 5));
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_mt(&graph, 42), 2);
}

#[test]
fn test_connected_components_mt_two_disconnected() {
    let vertices = SetLit![0, 1];
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components_mt(&graph, 42);
    assert_eq!(reps.size(), 2);
    assert_ne!(comp_map.get(&0), comp_map.get(&1));
}

#[test]
fn test_count_components_hof_mt_star() {
    let mut vertices = SetLit![0];
    for i in 1..6usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..6usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_hof(&graph, 42), 1);
}

#[test]
fn test_connected_components_hof_mt_isolated() {
    let mut vertices = SetLit![];
    for i in 0..4 {
        let _ = vertices.insert(i);
    }
    let edges: SetStEph<Edge<usize>> = SetLit![];
    let graph = <UnDirGraphMtEph<usize> as UnDirGraphMtEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, _) = connected_components_hof(&graph, 42);
    assert_eq!(reps.size(), 4);
}
