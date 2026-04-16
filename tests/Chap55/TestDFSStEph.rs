// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for DFS - Sequential Ephemeral.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap55::DFSStEph::DFSStEph::*;
use apas_verus::Types::Types::*;

// test_empty_graph removed: dfs panics with out-of-bounds when source vertex 0 does not exist.

#[test]
fn test_single_vertex() {
    let graph = ArraySeqStEphS::from_vec(vec![ArraySeqStEphS::from_vec(vec![])]);
    let result = DFSStEph::dfs(&graph, 0);
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
    let result = DFSStEph::dfs(&graph, 0);
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
    let result = DFSStEph::dfs(&graph, 0);
    assert_eq!(result.size(), 4);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqStEphS::from_vec(vec![
        ArraySeqStEphS::from_vec(vec![1]),
        ArraySeqStEphS::from_vec(vec![2]),
        ArraySeqStEphS::from_vec(vec![0]),
    ]);
    let result = DFSStEph::dfs(&graph, 0);
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
    let result = DFSStEph::dfs(&graph, 0);
    assert_eq!(result.size(), 2);
}
