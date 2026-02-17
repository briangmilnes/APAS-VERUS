//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for DFS - Sequential Ephemeral.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap55::DFSStEph::DFSStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty_graph() {
    let graph = ArraySeqStEphS::<ArraySeqStEphS<usize>>::from_vec(vec![]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_single_vertex() {
    let graph = ArraySeqStEphS::from_vec(vec![ArraySeqStEphS::from_vec(vec![])]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 1);
    assert!(result.find(&0));
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![1]),
        ArraySeqStEphS::from_vec(vec![2]),
        ArraySeqStEphS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 3);
}

#[test]
fn test_dag() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![1, 2]),
        ArraySeqStEphS::from_vec(vec![3]),
        ArraySeqStEphS::from_vec(vec![3]),
        ArraySeqStEphS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 4);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![1]),
        ArraySeqStEphS::from_vec(vec![2]),
        ArraySeqStEphS::from_vec(vec![0]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 3);
}

#[test]
fn test_disconnected() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![1]),
        ArraySeqStEphS::from_vec(vec![]),
        ArraySeqStEphS::from_vec(vec![3]),
        ArraySeqStEphS::from_vec(vec![]),
    ]);
    let result = dfs(&graph, 0);
    assert_eq!(result.size(), 2);
}
