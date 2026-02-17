//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap55 SCCStPer.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap55::SCCStPer::SCCStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_single_node() {
    let adj: Vec<ArraySeqStPerS<usize>> = vec![ArraySeqStPerS::empty()];
    let graph = ArraySeqStPerS::from_vec(adj);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_two_nodes_no_edges() {
    let adj0 = ArraySeqStPerS::empty();
    let adj1 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 2);
}

#[test]
fn test_simple_cycle() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_two_separate_sccs() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![0]);
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::from_vec(vec![2]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 2);
}

#[test]
fn test_linear_dag() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 3);
}

#[test]
fn test_self_loop() {
    let adj0 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_complex_graph() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2, 4]);
    let adj2 = ArraySeqStPerS::from_vec(vec![3, 0]);
    let adj3 = ArraySeqStPerS::from_vec(vec![2]);
    let adj4 = ArraySeqStPerS::from_vec(vec![5]);
    let adj5 = ArraySeqStPerS::from_vec(vec![4]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3, adj4, adj5]);
    let sccs = scc(&graph);
    assert!(sccs.length() >= 1);
}
