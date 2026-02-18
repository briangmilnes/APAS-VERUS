//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Example 56.3 - Negative Weight Cycles
//!
//! Demonstrates how negative weight cycles affect shortest path calculations.
//! When a graph has a negative weight cycle, distances to vertices reachable through
//! the cycle become arbitrarily small (approach negative infinity).

pub mod Example56_3 {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::path_weight_int;

    pub trait Example56_3Trait {
        /// Example demonstrating a negative weight cycle
        /// APAS: Work O(|V| + |E|), Span O(1)
        fn example_negative_cycle();

        /// Example showing that shortest paths are undefined in presence of negative cycles
        /// APAS: Work O(|E|), Span O(1)
        fn example_undefined_shortest_path();
    }

    /// Example demonstrating a negative weight cycle.
    /// Graph: 0 -> 1 -> 2 -> 1 (cycle with negative total weight).
    /// - APAS: N/A — demonstration code.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — constant-sized example graph.
    pub fn example_negative_cycle() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 1, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 2]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, -4, 0]),
        ]);

        println!("Graph with negative cycle 1->2->1 (weight = 2 + (-4) = -2):");

        let simple_path = ArraySeqStPerS::from_vec(vec![0, 1]);
        match path_weight_int(&simple_path, &weights) {
            | Some(w) => println!("  Path 0→1: {w}"),
            | None => println!("  Invalid path"),
        }

        let one_cycle = ArraySeqStPerS::from_vec(vec![0, 1, 2, 1]);
        match path_weight_int(&one_cycle, &weights) {
            | Some(w) => println!("  Path 0→1→2→1 (one cycle): {w}"),
            | None => println!("  Invalid path"),
        }

        let two_cycles = ArraySeqStPerS::from_vec(vec![0, 1, 2, 1, 2, 1]);
        match path_weight_int(&two_cycles, &weights) {
            | Some(w) => println!("  Path 0→1→2→1→2→1 (two cycles): {w}"),
            | None => println!("  Invalid path"),
        }

        println!("  Each cycle around 1→2→1 subtracts 2, approaching -∞");
    }

    /// Example showing that shortest paths are undefined in presence of negative cycles.
    /// - APAS: N/A — demonstration code.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — constant-sized example graph.
    pub fn example_undefined_shortest_path() {
        let _weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 1, i64::MAX, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 1, 1]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, -3, 0, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, i64::MAX, 0]),
        ]);

        println!("\nGraph with cycle 1->2->1 (weight = 1 + (-3) = -2):");
        println!("  Shortest path from 0 to 1 is undefined (can traverse cycle repeatedly)");
        println!("  Shortest path from 0 to 3 through 1 is also undefined");
    }
}
