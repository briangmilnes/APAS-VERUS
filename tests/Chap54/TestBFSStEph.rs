//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap54::BFSStEph::*;
use apas_verus::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;

#[test]
fn test_empty_graph() {
    let graph: ArraySeqStEphS<ArraySeqStEphS<usize>> = ArraySeqStEphSLit![];
    let distances = BFSStEph::bfs(&graph, 0);
    assert_eq!(distances.length(), 0);
}

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

#[test]
fn test_invalid_source() {
    let graph = ArraySeqStEphS::from_vec(vec![ArraySeqStEphSLit![1], ArraySeqStEphSLit![]]);
    let distances = BFSStEph::bfs(&graph, 5);
    assert_eq!(distances.length(), 2);
    assert_eq!(*distances.nth(0), UNREACHABLE);
    assert_eq!(*distances.nth(1), UNREACHABLE);
}
