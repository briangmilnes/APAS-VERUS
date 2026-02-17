#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 GraphSearchMtPer.

use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_verus::Chap53::GraphSearchMtPer::GraphSearchMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_reachable_single_node() {
    let graph = |_v: &i32| AVLTreeSetMtPer::empty();
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 1);
}

#[test]
fn test_reachable_simple_path() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 2);
}

#[test]
fn test_graph_search_single_source() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 2);
}

#[test]
fn test_graph_search_multi_source() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let sources = AVLTreeSetMtPer::singleton(1).insert(3);
    let result = graph_search_multi(&graph, sources, &SelectAll);
    assert_eq!(result.visited.size(), 3);
}

#[test]
fn test_select_one_strategy() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2).insert(3)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let _result = graph_search(&graph, 1, &SelectOne);
}

#[test]
fn test_empty_graph() {
    let graph = |_v: &i32| AVLTreeSetMtPer::empty();
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 1);
}

#[test]
fn test_cycle_detection() {
    let graph = |v: &i32| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::singleton(3),
        | 3 => AVLTreeSetMtPer::singleton(1),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 3);
}
