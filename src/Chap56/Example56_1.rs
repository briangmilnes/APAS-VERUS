//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Example 56.1 - Path Weight Computation
//!
//! Demonstrates computing the weight of a path in a weighted graph.
//! Shows path weight calculation for simple paths with both positive and negative weights.

pub mod Example56_1 {

    use ordered_float::OrderedFloat;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::*;

    pub trait Example56_1Trait {
        /// Claude Work: O(1), Span: O(1)
        /// Example demonstrating path weight computation with integer weights.
        fn example_path_weight_int();

        /// Claude Work: O(1), Span: O(1)
        /// Example demonstrating path weight computation with floating-point weights.
        fn example_path_weight_float();

        /// Claude Work: O(1), Span: O(1)
        /// Example with negative edge weights.
        fn example_negative_weights();
    }

    /// Example demonstrating path weight computation with integer weights.
    /// - APAS: N/A — demonstration code.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — constant-sized example graph.
    pub fn example_path_weight_int() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 5, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 3, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0, 1]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, i64::MAX, 0]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {w}"),
            | None => println!("Invalid path"),
        }
    }

    /// Example demonstrating path weight computation with floating-point weights.
    /// - APAS: N/A — demonstration code.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — constant-sized example graph.
    pub fn example_path_weight_float() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(0.0),
                OrderedFloat(2.5),
                OrderedFloat(5.0),
                OrderedFloat(f64::INFINITY),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
                OrderedFloat(1.5),
                OrderedFloat(f64::INFINITY),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
                OrderedFloat(0.5),
            ]),
            ArraySeqStEphS::from_vec(vec![
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(f64::INFINITY),
                OrderedFloat(0.0),
            ]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match path_weight_float(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {:.1}", w.0),
            | None => println!("Invalid path"),
        }
    }

    /// Example with negative edge weights.
    /// - APAS: N/A — demonstration code.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — constant-sized example graph.
    pub fn example_negative_weights() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, -5]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
        match path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2 with negative weight has total: {w}"),
            | None => println!("Invalid path"),
        }
    }
}
