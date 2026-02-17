//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Tests

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
use apas_verus::Chap63::ConnectivityMtEph::ConnectivityMtEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_connected_graph() -> UnDirGraphMtEph<N> {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..6 {
        let _ = edges.insert(Edge(i, (i + 1) % 6));
    }
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::from_sets(vertices, edges)
}

fn create_multi_component_graph() -> UnDirGraphMtEph<N> {
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
    <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::from_sets(vertices, edges)
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
