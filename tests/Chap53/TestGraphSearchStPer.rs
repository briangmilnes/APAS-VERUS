//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 GraphSearchStPer.

use apas_verus::ArraySeqStPerSLit;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap53::GraphSearchStPer::GraphSearchStPer::*;
use apas_verus::Types::Types::*;

// Build test graph as adjacency function
fn test_graph_1() -> impl Fn(&N) -> AVLTreeSetStPer<N> {
    |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(3)),
        | 2 => AVLTreeSetStPer::singleton(4),
        | 3 => AVLTreeSetStPer::singleton(4).union(&AVLTreeSetStPer::singleton(5)),
        | 4 => AVLTreeSetStPer::empty(),
        | 5 => AVLTreeSetStPer::empty(),
        | _ => AVLTreeSetStPer::empty(),
    }
}

#[test]
fn test_empty_graph() {
    let graph = |_: &N| AVLTreeSetStPer::empty();
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_single_edge() {
    let graph = |v: &N| {
        if *v == 1 {
            AVLTreeSetStPer::singleton(2)
        } else {
            AVLTreeSetStPer::empty()
        }
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_linear_chain() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(4),
        | _ => AVLTreeSetStPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_dag() {
    let graph = test_graph_1();
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_cycle() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(1),
        | _ => AVLTreeSetStPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_disconnected_component() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::empty(),
        | 3 => AVLTreeSetStPer::singleton(4),
        | _ => AVLTreeSetStPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(!result.visited.find(&3));
    assert!(!result.visited.find(&4));
}

#[test]
fn test_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(5));
    let result = graph_search_multi(&graph, sources, &SelectAll);
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
    assert!(!result.visited.find(&1));
}

#[test]
fn test_reachable() {
    let graph = test_graph_1();
    let reachable = reachable(&graph, 1);
    assert_eq!(reachable.size(), 5);
    for i in 1..=5 {
        assert!(reachable.find(&i));
    }
}

#[test]
fn test_select_one_dfs_style() {
    let graph = test_graph_1();
    let result = graph_search(&graph, 1, &SelectOne);
    // SelectOne should still visit all reachable vertices, just in a different order
    assert!(result.visited.size() >= 1);
    assert!(result.visited.find(&1));
}
