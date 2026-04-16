// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Tests for Chap53 GraphSearchStEph.

use vstd::prelude::Ghost;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap53::GraphSearchStEph::GraphSearchStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_reachable_single_node() {
    let graph = |_v: &i32| AVLTreeSetStEph::empty();
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
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
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
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
    let result = graph_search(&graph, 1, &SelectAll, Ghost::assume_new());
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
    let result = graph_search_multi(&graph, sources, &SelectAll, Ghost::assume_new());
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
    let _result = graph_search(&graph, 1, &SelectOne, Ghost::assume_new());
}

#[test]
fn test_empty_graph() {
    let graph = |_v: &i32| AVLTreeSetStEph::empty();
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
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
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
    assert_eq!(reachable_set.size(), 3);
}

#[test]
fn test_disconnected_graph() {
    let graph = |v: &i32| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | _ => AVLTreeSetStEph::empty(),
    };
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
    assert_eq!(reachable_set.size(), 2);
    assert!(reachable_set.find(&1));
    assert!(reachable_set.find(&2));
}

#[test]
fn test_star_graph() {
    let graph = |v: &i32| match *v {
        | 0 => {
            let mut s = AVLTreeSetStEph::singleton(1);
            s.insert(2);
            s.insert(3);
            s.insert(4);
            s
        },
        | _ => AVLTreeSetStEph::empty(),
    };
    let reachable_set = reachable(&graph, 0, Ghost::assume_new());
    assert_eq!(reachable_set.size(), 5);
}

#[test]
fn test_self_loop() {
    let graph = |v: &i32| match *v {
        | 1 => AVLTreeSetStEph::singleton(1),
        | _ => AVLTreeSetStEph::empty(),
    };
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
    assert_eq!(reachable_set.size(), 1);
}


#[test]
fn test_diamond_graph() {
    let graph = |v: &i32| match *v {
        | 1 => {
            let mut s = AVLTreeSetStEph::singleton(2);
            s.insert(3);
            s
        }
        | 2 => AVLTreeSetStEph::singleton(4),
        | 3 => AVLTreeSetStEph::singleton(4),
        | _ => AVLTreeSetStEph::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll, Ghost::assume_new());
    assert_eq!(result.visited.size(), 4);
}


#[test]
fn test_linear_chain() {
    let graph = |v: &i32| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | 2 => AVLTreeSetStEph::singleton(3),
        | 3 => AVLTreeSetStEph::singleton(4),
        | 4 => AVLTreeSetStEph::singleton(5),
        | _ => AVLTreeSetStEph::empty(),
    };
    let reachable_set = reachable(&graph, 1, Ghost::assume_new());
    assert_eq!(reachable_set.size(), 5);
}
