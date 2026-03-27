//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 PQMinStEph.

use vstd::prelude::Ghost;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap53::PQMinStEph::PQMinStEph::*;
use apas_verus::Types::Types::*;

fn test_graph_1() -> impl Fn(&usize) -> AVLTreeSetStEph<usize> {
    |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2).union(&AVLTreeSetStEph::singleton(3)),
        | 2 => AVLTreeSetStEph::singleton(4),
        | 3 => AVLTreeSetStEph::singleton(4).union(&AVLTreeSetStEph::singleton(5)),
        | 4 => AVLTreeSetStEph::empty(),
        | 5 => AVLTreeSetStEph::empty(),
        | _ => AVLTreeSetStEph::empty(),
    }
}

#[test]
fn test_pq_min_empty_graph() {
    let graph = |_: &usize| AVLTreeSetStEph::empty();
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_pq_min_single_edge() {
    let graph = |v: &usize| {
        if *v == 1 {
            AVLTreeSetStEph::singleton(2)
        } else {
            AVLTreeSetStEph::empty()
        }
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_pq_min_dag() {
    let graph = test_graph_1();
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_priority_order() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2).union(&AVLTreeSetStEph::singleton(3)),
        | 2 => AVLTreeSetStEph::singleton(4),
        | 3 => AVLTreeSetStEph::singleton(5),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    assert_eq!(result.priorities.size(), 5);
}

#[test]
fn test_pq_min_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetStEph::singleton(2).union(&AVLTreeSetStEph::singleton(5));
    let prio_fn = |v: &usize| *v;
    let result = pq_min_multi(&graph, sources, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_pq_min_disconnected_graph() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | 2 => AVLTreeSetStEph::empty(),
        | 3 => AVLTreeSetStEph::singleton(4),
        | 4 => AVLTreeSetStEph::empty(),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(!result.visited.find(&3));
    assert!(!result.visited.find(&4));
}

#[test]
fn test_pq_min_cycle() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | 2 => AVLTreeSetStEph::singleton(3),
        | 3 => AVLTreeSetStEph::singleton(1),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_pq_min_custom_priority() {
    let distances = AVLTreeSetStEph::singleton(Pair(1, 0))
        .union(&AVLTreeSetStEph::singleton(Pair(2, 5)))
        .union(&AVLTreeSetStEph::singleton(Pair(3, 10)));

    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2).union(&AVLTreeSetStEph::singleton(3)),
        | _ => AVLTreeSetStEph::empty(),
    };

    let prio_fn = move |v: &usize| -> usize {
        let seq = distances.to_seq();
        for i in 0..seq.length() {
            let pair = seq.nth(i);
            if pair.0 == *v {
                return pair.1;
            }
        }
        999999
    };
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_pq_min_linear_chain() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2),
        | 2 => AVLTreeSetStEph::singleton(3),
        | 3 => AVLTreeSetStEph::singleton(4),
        | 4 => AVLTreeSetStEph::singleton(5),
        | 5 => AVLTreeSetStEph::empty(),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_complete_graph() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2)
            .union(&AVLTreeSetStEph::singleton(3))
            .union(&AVLTreeSetStEph::singleton(4)),
        | 2 => AVLTreeSetStEph::singleton(1)
            .union(&AVLTreeSetStEph::singleton(3))
            .union(&AVLTreeSetStEph::singleton(4)),
        | 3 => AVLTreeSetStEph::singleton(1)
            .union(&AVLTreeSetStEph::singleton(2))
            .union(&AVLTreeSetStEph::singleton(4)),
        | 4 => AVLTreeSetStEph::singleton(1)
            .union(&AVLTreeSetStEph::singleton(2))
            .union(&AVLTreeSetStEph::singleton(3)),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_star_graph() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2)
            .union(&AVLTreeSetStEph::singleton(3))
            .union(&AVLTreeSetStEph::singleton(4))
            .union(&AVLTreeSetStEph::singleton(5)),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_binary_tree() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(2).union(&AVLTreeSetStEph::singleton(3)),
        | 2 => AVLTreeSetStEph::singleton(4).union(&AVLTreeSetStEph::singleton(5)),
        | 3 => AVLTreeSetStEph::singleton(6).union(&AVLTreeSetStEph::singleton(7)),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 7);
    for i in 1..=7 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_self_loop() {
    let graph = |v: &usize| match *v {
        | 1 => AVLTreeSetStEph::singleton(1).union(&AVLTreeSetStEph::singleton(2)),
        | _ => AVLTreeSetStEph::empty(),
    };
    let prio_fn = |v: &usize| *v;
    let result = pq_min(&graph, 1, &prio_fn, Ghost::assume_new());
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}
