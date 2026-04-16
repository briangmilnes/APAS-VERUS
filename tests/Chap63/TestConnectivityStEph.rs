// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 63: Graph Connectivity - Sequential Ephemeral Tests

use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
use apas_verus::Chap63::ConnectivityStEph::ConnectivityStEph::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

fn create_connected_graph() -> UnDirGraphStEph<usize> {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Create a cycle: 0-1-2-3-4-5-0
    for i in 0..6 {
        let _ = edges.insert(Edge(i, (i + 1) % 6));
    }
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
}

fn create_multi_component_graph() -> UnDirGraphStEph<usize> {
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
    <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges)
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
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
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

#[test]
fn test_count_components_isolated_vertices() {
    let mut vertices = SetLit![];
    for i in 0..5 {
        let _ = vertices.insert(i);
    }
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let count = count_components(&graph);
    assert_eq!(count, 5);
}

#[test]
fn test_count_components_single_vertex() {
    let vertices = SetLit![42];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
}

#[test]
fn test_count_components_two_vertices_connected() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![Edge(0, 1)];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
}

#[test]
fn test_count_components_two_vertices_disconnected() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 2);
}

#[test]
fn test_connected_components_isolated_vertices() {
    let mut vertices = SetLit![];
    for i in 0..4 {
        let _ = vertices.insert(i);
    }
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components(&graph);

    assert_eq!(reps.size(), 4);
    // Each vertex should be its own component representative.
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(comp_map.get(&i), comp_map.get(&j));
        }
    }
}

#[test]
fn test_connected_components_complete_graph() {
    let mut vertices = SetLit![];
    for i in 0..5 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..5usize {
        for j in (i + 1)..5 {
            let _ = edges.insert(Edge(i, j));
        }
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components(&graph);

    assert_eq!(reps.size(), 1);
    let rep = comp_map.get(&0).unwrap();
    for i in 1..5 {
        assert_eq!(comp_map.get(&i).unwrap(), rep);
    }
}

#[test]
fn test_count_components_hof_single() {
    let graph = create_connected_graph();
    assert_eq!(count_components_hof(&graph), 1);
}

#[test]
fn test_count_components_hof_empty() {
    let vertices = SetLit![];
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components_hof(&graph), 0);
}

#[test]
fn test_connected_components_hof_single() {
    let graph = create_connected_graph();
    let (reps, _comp_map) = connected_components_hof(&graph);
    assert_eq!(reps.size(), 1);
}

#[test]
fn test_count_components_path_graph() {
    let mut vertices = SetLit![];
    for i in 0..10 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..9usize {
        let _ = edges.insert(Edge(i, i + 1));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
    assert_eq!(count_components_hof(&graph), 1);
}

#[test]
fn test_count_components_star_graph() {
    let mut vertices = SetLit![0];
    for i in 1..8usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..8usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
}

#[test]
fn test_count_components_two_triangles() {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    // Triangle 1: 0-1-2.
    let _ = edges.insert(Edge(0, 1));
    let _ = edges.insert(Edge(1, 2));
    let _ = edges.insert(Edge(0, 2));
    // Triangle 2: 3-4-5.
    let _ = edges.insert(Edge(3, 4));
    let _ = edges.insert(Edge(4, 5));
    let _ = edges.insert(Edge(3, 5));
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 2);
}

#[test]
fn test_connected_components_star_graph() {
    let mut vertices = SetLit![0];
    for i in 1..5usize {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 1..5usize {
        let _ = edges.insert(Edge(0, i));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components(&graph);
    assert_eq!(reps.size(), 1);
    let rep = comp_map.get(&0).unwrap();
    for i in 1..5 {
        assert_eq!(comp_map.get(&i).unwrap(), rep);
    }
}

#[test]
fn test_count_components_grid_connected() {
    // 3x3 grid, fully connected.
    let mut vertices = SetLit![];
    for i in 0..9 {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for r in 0..3usize {
        for c in 0..3usize {
            let v = r * 3 + c;
            if c + 1 < 3 { let _ = edges.insert(Edge(v, v + 1)); }
            if r + 1 < 3 { let _ = edges.insert(Edge(v, v + 3)); }
        }
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
}

#[test]
fn test_count_components_many_singletons() {
    let mut vertices = SetLit![];
    for i in 0..20 {
        let _ = vertices.insert(i);
    }
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 20);
    assert_eq!(count_components_hof(&graph), 20);
}

#[test]
fn test_connected_components_hof_isolated() {
    let mut vertices = SetLit![];
    for i in 0..6 {
        let _ = vertices.insert(i);
    }
    let edges = SetLit![];
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    let (reps, comp_map) = connected_components_hof(&graph);
    assert_eq!(reps.size(), 6);
    for i in 0..6 {
        for j in (i + 1)..6 {
            assert_ne!(comp_map.get(&i), comp_map.get(&j));
        }
    }
}

#[test]
fn test_count_components_large_cycle() {
    let n = 50;
    let mut vertices = SetLit![];
    for i in 0..n {
        let _ = vertices.insert(i);
    }
    let mut edges = SetLit![];
    for i in 0..n {
        let _ = edges.insert(Edge(i, (i + 1) % n));
    }
    let graph = <UnDirGraphStEph<usize> as UnDirGraphStEphTrait<usize>>::from_sets(vertices, edges);
    assert_eq!(count_components(&graph), 1);
}
