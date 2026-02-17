//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqMtEphChap19SLit;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap54::BFSMtEph::*;
use apas_verus::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;

#[test]
fn test_empty_graph() {
    let graph: ArraySeqMtEphS<ArraySeqMtEphS<usize>> = ArraySeqMtEphChap19SLit![];
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.length(), 0);
}

#[test]
fn test_single_vertex() {
    let graph = ArraySeqMtEphS::from_vec(vec![ArraySeqMtEphChap19SLit![]]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphChap19SLit![1],
        ArraySeqMtEphChap19SLit![2],
        ArraySeqMtEphChap19SLit![3],
        ArraySeqMtEphChap19SLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
    assert_eq!(*distances.nth(3), 3);
}

#[test]
fn test_dag() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphChap19SLit![1, 2],
        ArraySeqMtEphChap19SLit![2, 3, 4],
        ArraySeqMtEphChap19SLit![4],
        ArraySeqMtEphChap19SLit![5, 6],
        ArraySeqMtEphChap19SLit![0, 4, 6],
        ArraySeqMtEphChap19SLit![],
        ArraySeqMtEphChap19SLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
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
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphChap19SLit![1],
        ArraySeqMtEphChap19SLit![],
        ArraySeqMtEphChap19SLit![3],
        ArraySeqMtEphChap19SLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), UNREACHABLE);
    assert_eq!(*distances.nth(3), UNREACHABLE);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphChap19SLit![1],
        ArraySeqMtEphChap19SLit![2],
        ArraySeqMtEphChap19SLit![0],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(*distances.nth(0), 0);
    assert_eq!(*distances.nth(1), 1);
    assert_eq!(*distances.nth(2), 2);
}

#[test]
fn test_invalid_source() {
    let graph = ArraySeqMtEphS::from_vec(vec![ArraySeqMtEphChap19SLit![1], ArraySeqMtEphChap19SLit![]]);
    let distances = BFSMtEph::bfs(&graph, 5);
    assert_eq!(distances.length(), 2);
    assert_eq!(*distances.nth(0), UNREACHABLE);
    assert_eq!(*distances.nth(1), UNREACHABLE);
}
