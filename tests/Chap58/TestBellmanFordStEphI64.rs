// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//!
//! Tests for Bellman-Ford Algorithm (Integer Weights)

use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
use apas_verus::Chap58::BellmanFordStEphI64::BellmanFordStEphI64::*;
use apas_verus::SetLit;
use apas_verus::Types::Types::*;

#[test]
fn test_example_58_2_dijkstra_fails() {
    // Example 58.2: Graph where Dijkstra's algorithm fails with negative edges
    // s -> b (weight 2), s -> a (weight 3), a -> b (weight -2)
    // Shortest path to b should be 1 (via a), not 2 (direct)
    let vertices = SetLit![0, 1, 2]; // s=0, a=1, b=2
    let edges = SetLit![
        WeightedEdge(0, 2, 2),  // s -> b (2)
        WeightedEdge(0, 1, 3),  // s -> a (3)
        WeightedEdge(1, 2, -2)  // a -> b (-2)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 3);
    assert_eq!(result.get_distance(2), 1); // s -> a -> b = 3 + (-2) = 1

    // Check path reconstruction
    let path_to_b = result.extract_path(2).unwrap();
    assert_eq!(path_to_b.length(), 3); // s -> a -> b
    assert_eq!(*path_to_b.nth(0), 0); // s
    assert_eq!(*path_to_b.nth(1), 1); // a
    assert_eq!(*path_to_b.nth(2), 2); // b
}

#[test]
fn test_example_58_3_k_hop_distances() {
    // Example 58.3: Graph demonstrating k-hop distance computation
    // Tests that algorithm correctly updates distances through iterations
    // Modified to remove negative cycle (changed c->b edge from 5 to 7)
    let vertices = SetLit![0, 1, 2, 3]; // s=0, a=1, b=2, c=3
    let edges = SetLit![
        WeightedEdge(0, 1, 1),  // s -> a (1)
        WeightedEdge(0, 2, 5),  // s -> b (5)
        WeightedEdge(1, 2, 3),  // a -> b (3)
        WeightedEdge(1, 3, 7),  // a -> c (7)
        WeightedEdge(2, 3, -6), // b -> c (-6)
        WeightedEdge(3, 2, 7)   // c -> b (7) - increased to avoid negative cycle
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 1); // s -> a
    assert_eq!(result.get_distance(2), 4); // s -> a -> b
    assert_eq!(result.get_distance(3), -2); // s -> a -> b -> c = 1 + 3 + (-6) = -2
}

#[test]
fn test_example_58_4_algorithm_steps() {
    // Example 58.4: Multiple rounds of distance updates
    let vertices = SetLit![0, 1, 2, 3, 4]; // s=0, a=1, b=2, c=3, d=4
    let edges = SetLit![
        WeightedEdge(0, 1, 6),  // s -> a
        WeightedEdge(0, 3, 7),  // s -> c
        WeightedEdge(1, 2, 5),  // a -> b
        WeightedEdge(1, 3, 8),  // a -> c
        WeightedEdge(1, 4, -4), // a -> d
        WeightedEdge(2, 1, -2), // b -> a
        WeightedEdge(3, 2, -3), // c -> b
        WeightedEdge(3, 4, 9),  // c -> d
        WeightedEdge(4, 0, 2),  // d -> s
        WeightedEdge(4, 2, 7)   // d -> b
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 2); // s -> c -> b -> a = 7 + (-3) + (-2) = 2
    assert_eq!(result.get_distance(2), 4); // s -> c -> b = 7 + (-3) = 4
    assert_eq!(result.get_distance(3), 7); // s -> c
    assert_eq!(result.get_distance(4), -2); // s -> c -> b -> a -> d = 2 + (-4) = -2
}

#[test]
fn test_negative_cycle_detection() {
    // Graph with a negative-weight cycle
    // s -> a -> b -> c -> a (cycle with total weight -1)
    let vertices = SetLit![0, 1, 2, 3]; // s=0, a=1, b=2, c=3
    let edges = SetLit![
        WeightedEdge(0, 1, 1),  // s -> a
        WeightedEdge(1, 2, 2),  // a -> b
        WeightedEdge(2, 3, -4), // b -> c
        WeightedEdge(3, 1, 0)   // c -> a (completes negative cycle: 2 + (-4) + 0 = -2)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0);

    assert!(matches!(result, Err(BellmanFordError::NegativeCycleDetected)));
}

#[test]
fn test_currency_exchange_example_58_1() {
    // Example 58.1: Currency exchange reduction to shortest paths
    // Using logarithm trick: w(u,v) = -lg(rate(u,v))
    // For simplicity, we use integers representing -100*lg(rate)
    // Euro=0, USD=1, GBP=2, CNY=3, JPY=4

    // Example rates (scaled and negated):
    // EUR->USD: 1.2  => -lg(1.2) ≈ -0.079 => -8
    // USD->CNY: 6.5  => -lg(6.5) ≈ -0.813 => -81
    // CNY->JPY: 16.0 => -lg(16) = -1.204  => -120
    let vertices = SetLit![0, 1, 2, 3, 4]; // EUR, USD, GBP, CNY, JPY
    let edges = SetLit![
        WeightedEdge(0, 1, -8),   // EUR -> USD
        WeightedEdge(1, 0, 9),    // USD -> EUR (reverse, worse rate)
        WeightedEdge(1, 3, -81),  // USD -> CNY
        WeightedEdge(3, 4, -120), // CNY -> JPY
        WeightedEdge(0, 2, -11),  // EUR -> GBP
        WeightedEdge(2, 4, -150)  // GBP -> JPY (direct, but less efficient)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    // Best path from EUR to JPY should be through USD and CNY
    assert_eq!(result.get_distance(4), -209); // -8 + (-81) + (-120) = -209

    // Verify path is EUR -> USD -> CNY -> JPY
    let path = result.extract_path(4).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0); // EUR
    assert_eq!(*path.nth(1), 1); // USD
    assert_eq!(*path.nth(2), 3); // CNY
    assert_eq!(*path.nth(3), 4); // JPY
}

#[test]
fn test_unreachable_vertices() {
    // Graph with some unreachable vertices
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 5), // s -> a
        WeightedEdge(1, 2, 3)  // a -> b
                         // c is unreachable
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 5);
    assert_eq!(result.get_distance(2), 8);
    assert_eq!(result.get_distance(3), i64::MAX); // Unreachable
}

#[test]
fn test_single_vertex() {
    // Trivial graph with only source
    let vertices = SetLit![0];
    let edges = SetLit![];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
}

#[test]
fn test_convergence_early_termination() {
    // Simple path graph that should converge before |V| rounds
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![WeightedEdge(0, 1, 2), WeightedEdge(1, 2, 3)];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 2);
    assert_eq!(result.get_distance(2), 5);
}

#[test]
fn test_zero_weight_edges() {
    // Graph with zero-weight edges
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![WeightedEdge(0, 1, 0), WeightedEdge(1, 2, 0)];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 0);
    assert_eq!(result.get_distance(2), 0);
}

#[test]
fn test_all_negative_edges_no_cycle() {
    // All edges negative but no negative cycle
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![WeightedEdge(0, 1, -1), WeightedEdge(1, 2, -2)];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), -1);
    assert_eq!(result.get_distance(2), -3);
}

#[test]
fn test_diamond_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 1),
        WeightedEdge(0, 2, 4),
        WeightedEdge(1, 3, 2),
        WeightedEdge(2, 3, 1)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(3), 3); // via 0->1->3
}

#[test]
fn test_longer_path_cheaper() {
    // Direct: 0->2 costs 10. Indirect: 0->1->2 costs 3.
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        WeightedEdge(0, 2, 10),
        WeightedEdge(0, 1, 1),
        WeightedEdge(1, 2, 2)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(2), 3);
}


#[test]
fn test_complete_graph_4() {
    // Complete graph K4 with varying weights.
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 1),
        WeightedEdge(0, 2, 10),
        WeightedEdge(0, 3, 100),
        WeightedEdge(1, 2, 1),
        WeightedEdge(1, 3, 10),
        WeightedEdge(2, 3, 1)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 1);
    assert_eq!(result.get_distance(2), 2);  // via 0->1->2
    assert_eq!(result.get_distance(3), 3);  // via 0->1->2->3
}


#[test]
fn test_get_predecessor() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![WeightedEdge(0, 1, 3), WeightedEdge(1, 2, 4)];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_predecessor(0), None); // source has no predecessor
    assert_eq!(result.get_predecessor(1), Some(0));
    assert_eq!(result.get_predecessor(2), Some(1));
}


#[test]
fn test_is_reachable() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![WeightedEdge(0, 1, 5)];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert!(result.is_reachable(0));
    assert!(result.is_reachable(1));
    assert!(!result.is_reachable(2));
}


#[test]
fn test_longer_path_with_negatives() {
    // Longer indirect path through negative edges beats short direct path.
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 3, 50),
        WeightedEdge(0, 1, 10),
        WeightedEdge(1, 2, 20),
        WeightedEdge(2, 3, -100)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(3), -70); // 10 + 20 + (-100) = -70 < 50
}


#[test]
fn test_path_extraction_chain() {
    // Chain: 0 -> 1 -> 2 -> 3
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 2),
        WeightedEdge(1, 2, 3),
        WeightedEdge(2, 3, 4)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    let path = result.extract_path(3).unwrap();
    assert_eq!(path.length(), 4);
    assert_eq!(*path.nth(0), 0);
    assert_eq!(*path.nth(1), 1);
    assert_eq!(*path.nth(2), 2);
    assert_eq!(*path.nth(3), 3);
    assert_eq!(result.get_distance(3), 9);
}


#[test]
fn test_star_graph() {
    // Star topology: source connects to all other vertices.
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        WeightedEdge(0, 1, 10),
        WeightedEdge(0, 2, 20),
        WeightedEdge(0, 3, 30),
        WeightedEdge(0, 4, 40)
    ];

    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();

    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 10);
    assert_eq!(result.get_distance(2), 20);
    assert_eq!(result.get_distance(3), 30);
    assert_eq!(result.get_distance(4), 40);
}

#[test]
fn test_parallel_paths() {
    // Two paths from 0 to 3: 0->1->3 (cost 6) and 0->2->3 (cost 5).
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 2),
        WeightedEdge(1, 3, 4),
        WeightedEdge(0, 2, 1),
        WeightedEdge(2, 3, 4)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(3), 5); // 0->2->3
}

#[test]
fn test_bidirectional_edges() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        WeightedEdge(0, 1, 3),
        WeightedEdge(1, 0, 3),
        WeightedEdge(1, 2, 2),
        WeightedEdge(2, 1, 2)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(1), 3);
    assert_eq!(result.get_distance(2), 5);
}

#[test]
fn test_large_positive_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        WeightedEdge(0, 1, 1_000_000),
        WeightedEdge(1, 2, 1_000_000)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(2), 2_000_000);
}

#[test]
fn test_multiple_negative_edges_no_cycle() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, -5),
        WeightedEdge(1, 2, -3),
        WeightedEdge(2, 3, -1)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(3), -9);
}

#[test]
fn test_path_extraction_source() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![WeightedEdge(0, 1, 5)];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    let path = result.extract_path(0).unwrap();
    assert_eq!(path.length(), 1);
    assert_eq!(*path.nth(0), 0);
}

#[test]
fn test_two_vertices_bidirectional() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![
        WeightedEdge(0, 1, 5),
        WeightedEdge(1, 0, 10)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(1), 5);
    assert_eq!(result.get_predecessor(1), Some(0));
}

#[test]
fn test_chain_5_vertices() {
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        WeightedEdge(0, 1, 1),
        WeightedEdge(1, 2, 2),
        WeightedEdge(2, 3, 3),
        WeightedEdge(3, 4, 4)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(4), 10);
    assert!(result.is_reachable(4));
}

#[test]
fn test_negative_cycle_reachable() {
    // Negative cycle that is reachable from source.
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 1),
        WeightedEdge(1, 2, -1),
        WeightedEdge(2, 3, -1),
        WeightedEdge(3, 1, -1)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    assert!(bellman_ford(&graph, 0).is_err());
}

#[test]
fn test_self_loop_zero_weight() {
    let vertices = SetLit![0, 1];
    let edges = SetLit![
        WeightedEdge(0, 0, 0),
        WeightedEdge(0, 1, 5)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(0), 0);
    assert_eq!(result.get_distance(1), 5);
}

#[test]
fn test_multiple_shortest_paths_same_cost() {
    // Two paths of same cost from 0 to 3.
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        WeightedEdge(0, 1, 2),
        WeightedEdge(0, 2, 2),
        WeightedEdge(1, 3, 3),
        WeightedEdge(2, 3, 3)
    ];
    let graph = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
    let result = bellman_ford(&graph, 0).unwrap();
    assert_eq!(result.get_distance(3), 5); // Either path costs 5.
}
