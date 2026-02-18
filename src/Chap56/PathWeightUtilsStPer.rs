//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Shortest Path Utility Functions - Sequential Persistent
//!
//! Provides utility functions for computing path weights and validating
//! the sub-paths property for shortest paths with both integer and float weights.
//!
//! Uses persistent data structures for path manipulation.
//!
//! **Algorithmic Analysis:**
//! - `path_weight`: Work O(k), Span O(k) where k is path length
//! - `validate_subpath_property`: Work O(k²), Span O(k²) for k-vertex path

pub mod PathWeightUtilsStPer {

    use ordered_float::OrderedFloat;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T = ArraySeqStPerS<usize>;

    pub trait PathWeightUtilsStPerTrait {
        /// Claude Work: O(k), Span: O(k) where k is path length
        /// Computes the total weight of a path given edge weights (integer).
        fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>) -> Option<i64>;

        /// Claude Work: O(k), Span: O(k) where k is path length
        /// Computes the total weight of a path with floating-point weights.
        fn path_weight_float(
            path: &ArraySeqStPerS<usize>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<OrderedF64>>,
        ) -> Option<OrderedF64>;

        /// Claude Work: O(k²), Span: O(k²) for k-vertex path
        /// Validates the sub-paths property for integer weights.
        fn validate_subpath_property_int(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStPerS<i64>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>,
        ) -> bool;

        /// Claude Work: O(k²), Span: O(k²) for k-vertex path
        /// Validates the sub-paths property for floating-point weights.
        fn validate_subpath_property_float(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStPerS<OrderedF64>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<OrderedF64>>,
        ) -> bool;
    }

    /// Computes the total weight of a path given edge weights (integer version).
    /// Path is a sequence of vertices [v0, v1, ..., vk].
    /// Weights is an adjacency matrix where weights.nth(i).nth(j) is the weight of edge (i,j).
    /// Returns the sum of weights along the path, or None if path is invalid.
    /// - APAS: Work Θ(k), Span Θ(k) — implied by Def 56.1 (sum of k-1 edge weights).
    /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — agrees with APAS.
    pub fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>) -> Option<i64> {
        let k = path.length();
        if k < 2 {
            return Some(0);
        }

        let mut total = 0i64;
        for i in 0..k - 1 {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return None;
            }
            let edge_weight = *weights.nth(u).nth(v);
            total = total.saturating_add(edge_weight);
        }
        Some(total)
    }

    /// Computes the total weight of a path with floating-point weights.
    /// - APAS: Work Θ(k), Span Θ(k) — implied by Def 56.1.
    /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — agrees with APAS.
    pub fn path_weight_float(
        path: &ArraySeqStPerS<usize>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<OrderedF64>>,
    ) -> Option<OrderedF64> {
        let k = path.length();
        if k < 2 {
            return Some(OrderedFloat(0.0));
        }

        let mut total = OrderedFloat(0.0);
        for i in 0..k - 1 {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return None;
            }
            let edge_weight = *weights.nth(u).nth(v);
            total += edge_weight;
        }
        Some(total)
    }

    /// Validates the sub-paths property: every sub-path of a shortest path is itself a shortest path.
    /// Given a path and distances for all vertices from source, check if all sub-paths are optimal.
    /// This is a validation utility, not used in actual shortest path algorithms.
    /// - APAS: (no cost stated) — Def 56.4 states the property but not a validation algorithm.
    /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — checks k-1 consecutive edges; module header overstates as O(k²).
    pub fn validate_subpath_property_int(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStPerS<i64>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>,
    ) -> bool {
        let k = path.length();
        if k < 2 {
            return true;
        }

        for i in 0..k - 1 {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= distances.length() || v >= distances.length() {
                return false;
            }
            let dist_u = *distances.nth(u);
            let dist_v = *distances.nth(v);
            let edge_weight = *weights.nth(u).nth(v);

            if dist_u != i64::MAX && dist_v != dist_u.saturating_add(edge_weight) {
                return false;
            }
        }
        true
    }

    /// Validates the sub-paths property for floating-point weights.
    /// - APAS: (no cost stated) — Def 56.4 states the property but not a validation algorithm.
    /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — checks k-1 consecutive edges; module header overstates as O(k²).
    pub fn validate_subpath_property_float(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStPerS<OrderedF64>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<OrderedF64>>,
    ) -> bool {
        let k = path.length();
        if k < 2 {
            return true;
        }

        let epsilon = 1e-9;
        for i in 0..k - 1 {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= distances.length() || v >= distances.length() {
                return false;
            }
            let dist_u = *distances.nth(u);
            let dist_v = *distances.nth(v);
            let edge_weight = *weights.nth(u).nth(v);

            if dist_u.is_finite() && ((dist_v - (dist_u + edge_weight)).abs() > epsilon) {
                return false;
            }
        }
        true
    }
}
