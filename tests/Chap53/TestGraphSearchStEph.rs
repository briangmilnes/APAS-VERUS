//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 GraphSearchStEph.

use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap53::GraphSearchStEph::GraphSearchStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_reachable_single_node() {
    let graph = |_v: &i32| AVLTreeSetStEph::empty();
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 1);
}

#[test]
fn test_reachable_simple_path() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetStEph::singleton(2)
        } else {
            AVLTreeSetStEph::empty()
        }
    };
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 2);
}

#[test]
fn test_graph_search_single_source() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetStEph::singleton(2)
        } else {
            AVLTreeSetStEph::empty()
        }
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 2);
}

#[test]
fn test_graph_search_multi_source() {
    let graph = |v: &i32| {
        if *v == 1 {
            AVLTreeSetStEph::singleton(2)
        } else {
            AVLTreeSetStEph::empty()
        }
    };
    let mut sources = AVLTreeSetStEph::singleton(1);
    sources.insert(3);
    let result = graph_search_multi(&graph, sources, &SelectAll);
    assert_eq!(result.visited.size(), 3);
}

#[test]
fn test_select_one_strategy() {
    let graph = |v: &i32| {
        if *v == 1 {
            let mut s = AVLTreeSetStEph::singleton(2);
            s.insert(3);
            s
        } else {
            AVLTreeSetStEph::empty()
        }
    };
    let _result = graph_search(&graph, 1, &SelectOne);
}

#[test]
fn test_empty_graph() {
    let graph = |_v: &i32| AVLTreeSetStEph::empty();
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 1);
}

#[test]
fn test_cycle_detection() {
    let graph = |v: &i32| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | 2 => AVLTreeSetStEph::singleton(3),
        | 3 => AVLTreeSetStEph::singleton(1),
        | _ => AVLTreeSetStEph::empty(),
    };
    let reachable_set = reachable(&graph, 1);
    assert_eq!(reachable_set.size(), 3);
}
