//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArrayMtPerSLit;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap54::BFSMtPer::BFSMtPer::BFSTreeMtPerTrait;
use apas_verus::Chap54::BFSMtPer::*;
use apas_verus::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;
const NO_PARENT: usize = usize::MAX;

#[test]
fn test_single_vertex() {
    let graph = ArraySeqMtPerS::from_vec(vec![ArrayMtPerSLit![]]);
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![2],
        ArrayMtPerSLit![3],
        ArrayMtPerSLit![],
    ]);
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
    assert_eq!(*distances.nth(3), 3);
}

#[test]
fn test_dag() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1, 2],
        ArrayMtPerSLit![2, 3, 4],
        ArrayMtPerSLit![4],
        ArrayMtPerSLit![5, 6],
        ArrayMtPerSLit![0, 4, 6],
        ArrayMtPerSLit![],
        ArrayMtPerSLit![],
    ]);
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 1);
    assert_eq!(*distances.nth(3), 2);
    assert_eq!(*distances.nth(4), 2);
    assert_eq!(*distances.nth(5), 3);
    assert_eq!(*distances.nth(6), 3);
}

#[test]
fn test_unreachable() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![],
        ArrayMtPerSLit![3],
        ArrayMtPerSLit![],
    ]);
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), UNREACHABLE);
    assert_eq!(*distances.nth(3), UNREACHABLE);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqMtPerS::from_vec(vec![ArrayMtPerSLit![1], ArrayMtPerSLit![2], ArrayMtPerSLit![0]]);
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
}

// BFS Tree tests (Algorithm 54.6)

#[test]
fn test_tree_single_vertex() {
    let graph = ArraySeqMtPerS::from_vec(vec![ArrayMtPerSLit![]]);
    let tree = BFSMtPer::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(tree.order.length(), 1);
    assert_eq!(*tree.order.nth(0), 0);
}

#[test]
fn test_tree_line_graph() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![2],
        ArrayMtPerSLit![3],
        ArrayMtPerSLit![],
    ]);
    let tree = BFSMtPer::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(*tree.parents.nth(1), 0);
    assert_eq!(*tree.parents.nth(2), 1);
    assert_eq!(*tree.parents.nth(3), 2);
    assert_eq!(tree.order.length(), 4);
    assert_eq!(*tree.order.nth(0), 0);
}

#[test]
fn test_tree_unreachable() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![],
        ArrayMtPerSLit![3],
        ArrayMtPerSLit![],
    ]);
    let tree = BFSMtPer::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(*tree.parents.nth(1), 0);
    assert_eq!(*tree.parents.nth(2), NO_PARENT);
    assert_eq!(*tree.parents.nth(3), NO_PARENT);
    assert_eq!(tree.order.length(), 2);
}

#[test]
fn test_tree_top_down_iteration() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![2],
        ArrayMtPerSLit![],
    ]);
    let tree = BFSMtPer::bfs_tree(&graph, 0);
    let td = tree.top_down_order();
    let mut collected: Vec<usize> = Vec::new();
    for v in td {
        collected.push(*v);
    }
    assert_eq!(collected, vec![0, 1, 2]);
}

#[test]
fn test_tree_bottom_up_iteration() {
    let graph = ArraySeqMtPerS::from_vec(vec![
        ArrayMtPerSLit![1],
        ArrayMtPerSLit![2],
        ArrayMtPerSLit![],
    ]);
    let tree = BFSMtPer::bfs_tree(&graph, 0);
    let bu = tree.bottom_up_order();
    let mut collected: Vec<usize> = Vec::new();
    for v in &bu {
        collected.push(*v);
    }
    assert_eq!(collected, vec![2, 1, 0]);
}

