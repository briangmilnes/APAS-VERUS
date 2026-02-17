//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for DFS - Sequential Persistent.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap55::DFSStPer::DFSStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let graph = ArraySeqStPerS::<ArraySeqStPerS<usize>>::from_vec(vec![]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_single_vertex() {
    let graph = ArraySeqStPerS::from_vec(vec![ArraySeqStPerS::from_vec(vec![])]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 1);
    assert!(result.find(&0));
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1]),
        ArraySeqStPerS::from_vec(vec![2]),
        ArraySeqStPerS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 3);
    assert!(result.find(&0));
    assert!(result.find(&1));
    assert!(result.find(&2));
}

#[test]
fn test_dag() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1, 2]),
        ArraySeqStPerS::from_vec(vec![3]),
        ArraySeqStPerS::from_vec(vec![3]),
        ArraySeqStPerS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 4);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1]),
        ArraySeqStPerS::from_vec(vec![2]),
        ArraySeqStPerS::from_vec(vec![0]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 3);
}

#[test]
fn test_disconnected() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1]),
        ArraySeqStPerS::from_vec(vec![]),
        ArraySeqStPerS::from_vec(vec![3]),
        ArraySeqStPerS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 2);
    assert!(result.find(&0));
    assert!(result.find(&1));
    assert!(!result.find(&2));
    assert!(!result.find(&3));
}

#[test]
fn test_invalid_source() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1]),
        ArraySeqStPerS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 10);
    assert_eq!(result.size(), 0);
}
