//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap55 CycleDetectStPer.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap55::CycleDetectStPer::CycleDetectStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_no_cycle_empty_graph() {
    let graph = ArraySeqStPerS::<ArraySeqStPerS<usize>>::empty();
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_no_cycle_single_node() {
    let adj: Vec<ArraySeqStPerS<usize>> = vec![ArraySeqStPerS::empty()];
    let graph = ArraySeqStPerS::from_vec(adj);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_no_cycle_linear() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_simple_cycle() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let result = has_cycle(&graph);
    assert!(result);
}

#[test]
fn test_self_loop() {
    let adj0 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0]);
    let result = has_cycle(&graph);
    assert!(result);
}

#[test]
fn test_dag_no_cycle() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStPerS::from_vec(vec![3]);
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = has_cycle(&graph);
    assert!(!result);
}

#[test]
fn test_cycle_in_dag_structure() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStPerS::from_vec(vec![3]);
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = has_cycle(&graph);
    assert!(result);
}
