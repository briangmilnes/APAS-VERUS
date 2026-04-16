// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//!
//! Example 56.1 - Path Weight Computation
//!
//! Demonstrates computing the weight of a path in a weighted graph.
//! Shows path weight calculation for simple paths with both positive and negative weights.

pub mod Example56_1 {

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap56::PathWeightUtilsStEph::PathWeightUtilsStEph::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub struct Example56_1S;

    // 8. traits

    pub trait Example56_1Trait: Sized {
        /// Example demonstrating path weight computation with integer weights.
        fn example_path_weight_int();

        /// Example demonstrating path weight computation with different integer weights.
        fn example_path_weight_i64();

        /// Example with negative edge weights.
        fn example_negative_weights();
    }

    // 9. impls

    #[verifier::external] // accept hole: I/O demonstration functions with println.
    impl Example56_1Trait for Example56_1S {
    /// Example demonstrating path weight computation with integer weights.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) — constant-sized example graph.
    fn example_path_weight_int() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 5, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 3, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0, 1]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, i64::MAX, 0]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match PathWeightUtilsStEphS::path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {w}"),
            | None => println!("Invalid path"),
        }
    }

    /// Example demonstrating path weight computation with different integer weights.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) — constant-sized example graph.
    fn example_path_weight_i64() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 3, 5, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, 2, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0, 1]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, i64::MAX, 0]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2, 3]);
        match PathWeightUtilsStEphS::path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2→3 has weight: {w}"),
            | None => println!("Invalid path"),
        }
    }

    /// Example with negative edge weights.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) — constant-sized example graph.
    fn example_negative_weights() {
        let weights = ArraySeqStEphS::from_vec(vec![
            ArraySeqStEphS::from_vec(vec![0, 10, i64::MAX]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, 0, -5]),
            ArraySeqStEphS::from_vec(vec![i64::MAX, i64::MAX, 0]),
        ]);

        let path = ArraySeqStPerS::from_vec(vec![0, 1, 2]);
        match PathWeightUtilsStEphS::path_weight_int(&path, &weights) {
            | Some(w) => println!("Path 0→1→2 with negative weight has total: {w}"),
            | None => println!("Invalid path"),
        }
    }
    } // impl Example56_1Trait for Example56_1S

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for Example56_1S {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Example56_1S")
        }
    }

    impl std::fmt::Display for Example56_1S {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Example56_1")
        }
    }
}
