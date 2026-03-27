//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 PQMinStPer.

use vstd::prelude::Ghost;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap53::PQMinStPer::PQMinStPer::*;
use apas_verus::Types::Types::*;

fn test_graph_1() -> impl Fn(&usize) -> AVLTreeSetStPer<usize> {
    |v: &usize| match *v {
        | 1 => AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(3)),
        | 2 => AVLTreeSetStPer::singleton(4),
        | 3 => AVLTreeSetStPer::singleton(4).union(&AVLTreeSetStPer::singleton(5)),
        | 4 => AVLTreeSetStPer::empty(),
        | 5 => AVLTreeSetStPer::empty(),
        | _ => AVLTreeSetStPer::empty(),
    }
}

#[test]
fn test_pq_min_empty_graph() {
    let graph = |_: &usize| AVLTreeSetStPer::empty();
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_pq_min_single_edge() {
    let graph = |v: &usize| {
        if *v == 1 {
            AVLTreeSetStPer::singleton(2)
        } else {
            AVLTreeSetStPer::empty()
        }
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_pq_min_dag() {
    let graph = test_graph_1();
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_priority_order() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(3)),
        | 2 => AVLTreeSetStPer::singleton(4),
        | 3 => AVLTreeSetStPer::singleton(5),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    assert_eq!(result.priorities.size(), 5);
}

#[test]
fn test_pq_min_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(5));
    let prio_fn = |v: &usize| *v;
    let result = pq_min_multi(&graph, sources, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_pq_min_linear_chain() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(4),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_cycle() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(1),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new(), Ghost::assume_new());
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}
