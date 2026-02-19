//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap54::BFSStEph::*;
use apas_verus::Chap54::BFSStEph::BFSStEph::BFSTreeStEphTrait;
use apas_verus::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;
const NO_PARENT: usize = usize::MAX;

#[test]
fn test_single_vertex() {
    let graph = ArraySeqStEphS::from_vec(vec![ArraySeqStEphSLit![]]);
    let distances = BFSStEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![2],
        ArraySeqStEphSLit![3],
        ArraySeqStEphSLit![],
    ]);
    let distances = BFSStEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
    assert_eq!(*distances.nth(3), 3);
}

#[test]
fn test_dag() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1, 2],
        ArraySeqStEphSLit![2, 3, 4],
        ArraySeqStEphSLit![4],
        ArraySeqStEphSLit![5, 6],
        ArraySeqStEphSLit![0, 4, 6],
        ArraySeqStEphSLit![],
        ArraySeqStEphSLit![],
    ]);
    let distances = BFSStEph::bfs(&graph, 0);
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
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![],
        ArraySeqStEphSLit![3],
        ArraySeqStEphSLit![],
    ]);
    let distances = BFSStEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), UNREACHABLE);
    assert_eq!(*distances.nth(3), UNREACHABLE);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![2],
        ArraySeqStEphSLit![0],
    ]);
    let distances = BFSStEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
}

// BFS Tree tests (Algorithm 54.6)

#[test]
fn test_tree_single_vertex() {
    let graph = ArraySeqStEphS::from_vec(vec![ArraySeqStEphSLit![]]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(tree.order.length(), 1);
    assert_eq!(*tree.order.nth(0), 0);
}

#[test]
fn test_tree_line_graph() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![2],
        ArraySeqStEphSLit![3],
        ArraySeqStEphSLit![],
    ]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(*tree.parents.nth(1), 0);
    assert_eq!(*tree.parents.nth(2), 1);
    assert_eq!(*tree.parents.nth(3), 2);
    assert_eq!(tree.order.length(), 4);
    assert_eq!(*tree.order.nth(0), 0);
    assert_eq!(*tree.order.nth(1), 1);
    assert_eq!(*tree.order.nth(2), 2);
    assert_eq!(*tree.order.nth(3), 3);
}

#[test]
fn test_tree_dag() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1, 2],
        ArraySeqStEphSLit![2, 3, 4],
        ArraySeqStEphSLit![4],
        ArraySeqStEphSLit![5, 6],
        ArraySeqStEphSLit![0, 4, 6],
        ArraySeqStEphSLit![],
        ArraySeqStEphSLit![],
    ]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(*tree.parents.nth(1), 0);
    assert_eq!(*tree.parents.nth(2), 0);
    assert_eq!(*tree.parents.nth(3), 1);
    assert_eq!(*tree.parents.nth(4), 1);
    assert_eq!(*tree.parents.nth(5), 3);
    assert_eq!(*tree.parents.nth(6), 3);
    assert_eq!(tree.order.length(), 7);
    assert_eq!(*tree.order.nth(0), 0);
}

#[test]
fn test_tree_unreachable() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![],
        ArraySeqStEphSLit![3],
        ArraySeqStEphSLit![],
    ]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    assert_eq!(*tree.parents.nth(0), 0);
    assert_eq!(*tree.parents.nth(1), 0);
    assert_eq!(*tree.parents.nth(2), NO_PARENT);
    assert_eq!(*tree.parents.nth(3), NO_PARENT);
    assert_eq!(tree.order.length(), 2);
}

#[test]
fn test_tree_top_down_iteration() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![2],
        ArraySeqStEphSLit![],
    ]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    let td = tree.top_down_order();
    let mut collected: Vec<usize> = Vec::new();
    for v in td {
        collected.push(*v);
    }
    assert_eq!(collected, vec![0, 1, 2]);
}

#[test]
fn test_tree_bottom_up_iteration() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphSLit![1],
        ArraySeqStEphSLit![2],
        ArraySeqStEphSLit![],
    ]);
    let tree = BFSStEph::bfs_tree(&graph, 0);
    let bu = tree.bottom_up_order();
    let mut collected: Vec<usize> = Vec::new();
    for v in &bu {
        collected.push(*v);
    }
    assert_eq!(collected, vec![2, 1, 0]);
}

