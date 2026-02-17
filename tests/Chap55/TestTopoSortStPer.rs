//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap55 TopoSortStPer.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap55::TopoSortStPer::TopoSortStPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_single_node() {
    let adj: Vec<ArraySeqStPerS<usize>> = vec![ArraySeqStPerS::empty()];
    let graph = ArraySeqStPerS::from_vec(adj);
    let result = topological_sort_opt(&graph);
    assert!(result.is_some());
    if let Some(order) = result {
        assert_eq!(order.length(), 1);
    }
}

#[test]
fn test_linear_dag() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let result = topological_sort_opt(&graph);
    assert!(result.is_some());
}

#[test]
fn test_dag_with_branches() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1, 2]);
    let adj1 = ArraySeqStPerS::from_vec(vec![3]);
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = topological_sort_opt(&graph);
    assert!(result.is_some());
}

#[test]
fn test_cycle_returns_none() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let result = topological_sort_opt(&graph);
    assert!(result.is_none());
}

#[test]
fn test_self_loop_returns_none() {
    let adj0 = ArraySeqStPerS::from_vec(vec![0]);
    let graph = ArraySeqStPerS::from_vec(vec![adj0]);
    let result = topological_sort_opt(&graph);
    assert!(result.is_none());
}

#[test]
fn test_disconnected_components() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::empty();
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = topological_sort_opt(&graph);
    assert!(result.is_some());
}

#[test]
fn test_empty_graph() {
    let graph = ArraySeqStPerS::<ArraySeqStPerS<usize>>::empty();
    let result = topological_sort_opt(&graph);
    assert!(result.is_some());
}

#[test]
fn test_topo_sort_simple() {
    let adj0 = ArraySeqStPerS::from_vec(vec![1]);
    let adj1 = ArraySeqStPerS::from_vec(vec![2]);
    let adj2 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2]);
    let result = topo_sort(&graph);
    assert_eq!(result.length(), 3);
}

#[test]
fn test_topo_sort_disconnected() {
    let adj0 = ArraySeqStPerS::empty();
    let adj1 = ArraySeqStPerS::empty();
    let adj2 = ArraySeqStPerS::from_vec(vec![3]);
    let adj3 = ArraySeqStPerS::empty();
    let graph = ArraySeqStPerS::from_vec(vec![adj0, adj1, adj2, adj3]);
    let result = topo_sort(&graph);
    assert_eq!(result.length(), 4);
}

#[test]
fn test_topo_sort_empty() {
    let graph = ArraySeqStPerS::<ArraySeqStPerS<usize>>::empty();
    let result = topo_sort(&graph);
    assert_eq!(result.length(), 0);
}
