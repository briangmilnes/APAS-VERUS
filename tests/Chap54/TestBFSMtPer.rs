//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArrayMtPerSLit;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap54::BFSMtPer::*;
use apas_verus::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;

#[test]
fn test_empty_graph() {
    let graph: ArraySeqMtPerS<ArraySeqMtPerS<usize>> = ArrayMtPerSLit![];
    let distances = BFSMtPer::bfs(&graph, 0);
    assert_eq!(distances.length(), 0);
}

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

#[test]
fn test_invalid_source() {
    let graph = ArraySeqMtPerS::from_vec(vec![ArrayMtPerSLit![1], ArrayMtPerSLit![]]);
    let distances = BFSMtPer::bfs(&graph, 5);
    assert_eq!(distances.length(), 2);
    assert_eq!(*distances.nth(0), UNREACHABLE);
    assert_eq!(*distances.nth(1), UNREACHABLE);
}
