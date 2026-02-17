//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 PQMinStPer.

use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap53::PQMinStPer::PQMinStPer::*;
use apas_verus::Types::Types::*;

// Simple priority function: vertex value itself as priority
fn vertex_priority() -> ClosurePriority<N, N, impl Fn(&N) -> N> { ClosurePriority::new(|v: &N| *v) }

// Distance-based priority (for simulating shortest path)
fn distance_priority(distances: AVLTreeSetStPer<Pair<N, N>>) -> impl PriorityFn<N, N> {
    ClosurePriority::new(move |v: &N| {
        let seq = distances.to_seq();
        for i in 0..seq.length() {
            let pair = seq.nth(i);
            if pair.0 == *v {
                return pair.1;
            }
        }
        999999 // Large value for unreachable
    })
}

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
fn test_pq_min_empty_graph() {
    let graph = |_: &N| AVLTreeSetStPer::empty();
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_pq_min_single_edge() {
    let graph = |v: &N| {
        if *v == 1 {
            AVLTreeSetStPer::singleton(2)
        } else {
            AVLTreeSetStPer::empty()
        }
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_pq_min_dag() {
    let graph = test_graph_1();
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_priority_order() {
    // Graph: 1 -> {2, 3}, 2 -> {4}, 3 -> {5}
    // With vertex value as priority, lower values visited first
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(3)),
        | 2 => AVLTreeSetStPer::singleton(4),
        | 3 => AVLTreeSetStPer::singleton(5),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    // All reachable vertices should be visited
    assert_eq!(result.visited.size(), 5);

    // Check priorities were recorded
    assert_eq!(result.priorities.size(), 5);
}

#[test]
fn test_pq_min_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetStPer::singleton(2).union(&AVLTreeSetStPer::singleton(5));
    let prio_fn = vertex_priority();
    let result = pq_min_multi(&graph, sources, &prio_fn);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_pq_min_linear_chain() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(4),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_cycle() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetStPer::singleton(2),
        | 2 => AVLTreeSetStPer::singleton(3),
        | 3 => AVLTreeSetStPer::singleton(1),
        | _ => AVLTreeSetStPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_priority_fn_direct() {
    use apas_verus::Chap53::PQMinStPer::PQMinStPer::*;
    
    let prio_fn = vertex_priority();
    
    // Test priority function directly
    assert_eq!(prio_fn.priority(&1), 1);
    assert_eq!(prio_fn.priority(&5), 5);
    assert_eq!(prio_fn.priority(&10), 10);
}
