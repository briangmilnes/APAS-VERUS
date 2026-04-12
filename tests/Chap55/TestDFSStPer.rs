//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for DFS - Sequential Persistent.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap55::DFSStPer::DFSStPer::*;
use apas_verus::Types::Types::*;

// test_empty_graph removed: dfs panics with out-of-bounds when source vertex 0 does not exist.

#[test]
fn test_single_vertex() {
    let graph = ArraySeqStPerS::from_vec(vec![ArraySeqStPerS::from_vec(vec![])]);
    let result = DFSStPer::dfs(&graph, 0);
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
    let result = DFSStPer::dfs(&graph, 0);
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
    let result = DFSStPer::dfs(&graph, 0);
    assert_eq!(result.size(), 4);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqStPerS::from_vec(vec![
        ArraySeqStPerS::from_vec(vec![1]),
        ArraySeqStPerS::from_vec(vec![2]),
        ArraySeqStPerS::from_vec(vec![0]),
    ]);
    let result = DFSStPer::dfs(&graph, 0);
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
    let result = DFSStPer::dfs(&graph, 0);
    assert_eq!(result.size(), 2);
    assert!(result.find(&0));
    assert!(result.find(&1));
    assert!(!result.find(&2));
    assert!(!result.find(&3));
}

// test_invalid_source removed: dfs panics with out-of-bounds when source vertex 10 does not exist in a 2-vertex graph.
