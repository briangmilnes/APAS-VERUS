//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap55 CycleDetectStEph.

use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap55::CycleDetectStEph::CycleDetectStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_no_cycle_empty_graph() {
    let graph = ArraySeqStEphS::<ArraySeqStEphS<usize>>::empty();
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_no_cycle_single_node() {
    let adj: Vec<ArraySeqStEphS<usize>> = vec![ArraySeqStEphS::empty()];
    let graph = ArraySeqStEphS::from_vec(adj);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_no_cycle_linear() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2]);
    let adj2 = ArraySeqStEphS::empty();
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2]);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_simple_cycle() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1]);
    let adj1 = ArraySeqStEphS::from_vec(vec![2]);
    let adj2 = ArraySeqStEphS::from_vec(vec![0]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2]);
    let result = has_cycle(&graph);
    assert!(result);
}

#[test]
fn test_self_loop() {
    let adj0 = ArraySeqStEphS::from_vec(vec![0]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0]);
    let result = has_cycle(&graph);
    assert!(result);
}

#[test]
fn test_dag_no_cycle() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStEphS::from_vec(vec![3]);
    let adj2 = ArraySeqStEphS::from_vec(vec![3]);
    let adj3 = ArraySeqStEphS::empty();
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_cycle_in_dag_structure() {
    let adj0 = ArraySeqStEphS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStEphS::from_vec(vec![3]);
    let adj2 = ArraySeqStEphS::from_vec(vec![3]);
    let adj3 = ArraySeqStEphS::from_vec(vec![0]);
    let graph = ArraySeqStEphS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = has_cycle(&graph);
    assert!(result);
}
