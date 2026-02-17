//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap55 SCCStEph.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap55::SCCStEph::SCCStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_single_node() {
    let adj: Vec<ArraySeqStEphS<usize>> = vec![ArraySeqStEphS::empty()];
    let graph = ArraySeqStEphS::from_vec(adj);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_two_nodes_no_edges() {
    let adj0 = ArraySeqStEphS::empty();
    let adj1 = ArraySeqStEphS::empty();
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 2);
}

#[test]
fn test_simple_cycle() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2]);
    let adj2 = ArraySeqStEphS::from_vec(vec![0]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_two_separate_sccs() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![0]);
    let adj2 = ArraySeqStEphS::from_vec(vec![3]);
    let adj3 = ArraySeqStEphS::from_vec(vec![2]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 2);
}

#[test]
fn test_linear_dag() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2]);
    let adj2 = ArraySeqStEphS::empty();
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 3);
}

#[test]
fn test_self_loop() {
    let adj0 = ArraySeqStEphS::from_vec(vec![0]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0]);
    let sccs = scc(&graph);
    assert_eq!(sccs.length(), 1);
}

#[test]
fn test_complex_graph() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2, 4]);
    let adj2 = ArraySeqStEphS::from_vec(vec![3, 0]);
    let adj3 = ArraySeqStEphS::from_vec(vec![2]);
    let adj4 = ArraySeqStEphS::from_vec(vec![5]);
    let adj5 = ArraySeqStEphS::from_vec(vec![4]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2, adj3, adj4, adj5]);
    let sccs = scc(&graph);
    assert!(sccs.length() >= 1);
}
