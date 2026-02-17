//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Sequential Ephemeral Tests

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap63::ConnectivityStEph::ConnectivityStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_connected_graph() -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Create a cycle: 0-1-2-3-4-5-0
    for i in 0..6 {
        let _ = edges.insert(Edge(i, (i + 1) % 6));
    }
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges)
}

fn create_multi_component_graph() -> UnDirGraphStEph<N> {
    let mut vertices = SetLit![];
    for i in 0..8 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Component 1: 0-1-2
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    // Component 2: 3-4
    let _ = edges.insert(Edge(3, 4));
    // Component 3: 5-6-7
    let _ = edges.insert(Edge(5, 6));
    let _ = edges.insert(Edge(6, 7));
    <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges)
}

#[test]
fn test_count_components_single() {
    let graph = create_connected_graph();
    let count = count_components(&graph);
    assert_eq!(count, 1);
}

#[test]
fn test_count_components_multiple() {
    let graph = create_multi_component_graph();
    let count = count_components(&graph);
    assert_eq!(count, 3);
}

#[test]
fn test_count_components_empty() {
    let vertices = SetLit![];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<N> as UnDirGraphStEphTrait<N>>::from_sets(vertices, edges);
    let count = count_components(&graph);
    assert_eq!(count, 0);
}

#[test]
fn test_connected_components_single() {
    let graph = create_connected_graph();
    let (reps, comp_map) = connected_components(&graph);

    // Should have exactly 1 representative
    assert_eq!(reps.size(), 1);

    // All vertices should map to the same component
    let first_comp = comp_map.get(&0).unwrap();
    for i in 1..6 {
        assert_eq!(comp_map.get(&i).unwrap(), first_comp);
    }
}

#[test]
fn test_connected_components_multiple() {
    let graph = create_multi_component_graph();
    let (reps, comp_map) = connected_components(&graph);

    // Should have exactly 3 representatives
    assert_eq!(reps.size(), 3);

    // Vertices in same component should map to same representative
    let comp0 = comp_map.get(&0).unwrap();
    assert_eq!(comp_map.get(&1).unwrap(), comp0);
    assert_eq!(comp_map.get(&2).unwrap(), comp0);

    let comp3 = comp_map.get(&3).unwrap();
    assert_eq!(comp_map.get(&4).unwrap(), comp3);

    let comp5 = comp_map.get(&5).unwrap();
    assert_eq!(comp_map.get(&6).unwrap(), comp5);
    assert_eq!(comp_map.get(&7).unwrap(), comp5);

    // Components should be different
    assert_ne!(comp0, comp3);
    assert_ne!(comp0, comp5);
    assert_ne!(comp3, comp5);
}

#[test]
fn test_count_components_hof() {
    let graph = create_multi_component_graph();
    let count = count_components_hof(&graph);
    assert_eq!(count, 3);

    // Should match direct implementation
    let count_direct = count_components(&graph);
    assert_eq!(count, count_direct);
}

#[test]
fn test_connected_components_hof() {
    let graph = create_multi_component_graph();
    let (reps_hof, comp_map_hof) = connected_components_hof(&graph);
    let (reps_direct, comp_map_direct) = connected_components(&graph);

    // Should have same number of components
    assert_eq!(reps_hof.size(), reps_direct.size());
    assert_eq!(reps_hof.size(), 3);

    // Should produce equivalent component maps
    // (representatives may differ, but vertices in same component should match)
    for i in 0..8 {
        for j in (i + 1)..8 {
            let same_comp_hof = comp_map_hof.get(&i) == comp_map_hof.get(&j);
            let same_comp_direct = comp_map_direct.get(&i) == comp_map_direct.get(&j);
            assert_eq!(same_comp_hof, same_comp_direct);
        }
    }
}
