//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//!
//! Shortest Path Utility Functions - Sequential Ephemeral (Integer Weights)
//!
//! Provides utility functions for computing path weights and validating
//! the sub-paths property for shortest paths with integer edge weights.
//!
//! Uses ephemeral data structures for path manipulation.
//!
//! **Algorithmic Analysis:**
//! - `path_weight`: Work O(k), Span O(k) where k is path length
//! - `validate_subpath_property`: Work O(k²), Span O(k²) for k-vertex path

pub mod PathWeightUtilsStEph {

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::float::float::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T = ArraySeqStPerS<usize>;

    pub struct PathWeightUtilsStEphS;

    // 8. traits

    pub trait PathWeightUtilsStEphTrait: Sized {
        spec fn spec_path_weight_int(
            path: &ArraySeqStPerS<usize>,
            weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
            i: int,
            total: int,
        ) -> Option<i64>
            recommends 0 <= i, i64::MIN as int <= total <= i64::MAX as int;

        /// - Alg Analysis: APAS (Ch56 Def 56.1): computes path weight (sum of edge weights along a path).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k), Span O(k) — definition, k = path length
        fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>) -> (weight: Option<i64>)
            ensures weight == Self::spec_path_weight_int(path, weights, 0, 0);

        /// - Alg Analysis: APAS (Ch56 Def 56.1): computes path weight (sum of edge weights along a path).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k), Span O(k) — definition, k = path length
        fn path_weight_float(
            path: &ArraySeqStPerS<usize>,
            weights: &ArraySeqStEphS<ArraySeqStEphS<WrappedF64>>,
        ) -> (weight: Option<WrappedF64>);

        spec fn spec_validate_subpath_int(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStEphS<i64>,
            weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
            i: int,
        ) -> bool;

        /// - Alg Analysis: APAS (Ch56 Def 56.4): validates sub-paths property along a path.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k), Span O(k) — definition, k = path length
        fn validate_subpath_property_int(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStEphS<i64>,
            weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
        ) -> (valid: bool)
            ensures valid == Self::spec_validate_subpath_int(path, distances, weights, 0);

        /// - Alg Analysis: APAS (Ch56 Def 56.4): validates sub-paths property along a path.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k), Span O(k) — definition, k = path length
        fn validate_subpath_property_float(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStEphS<WrappedF64>,
            weights: &ArraySeqStEphS<ArraySeqStEphS<WrappedF64>>,
        ) -> (valid: bool);
    }

    // 9. impls

    impl PathWeightUtilsStEphTrait for PathWeightUtilsStEphS {
    open spec fn spec_path_weight_int(
        path: &ArraySeqStPerS<usize>,
        weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
        i: int,
        total: int,
    ) -> Option<i64>
        decreases path.spec_len() - i,
    {
        if i < 0 || path.spec_len() < 2 || i >= path.spec_len() - 1 {
            if i64::MIN as int <= total <= i64::MAX as int { Some(total as i64) } else { None }
        } else {
            let u = path.spec_index(i);
            let v = path.spec_index(i + 1);
            if u >= weights.spec_len() || v >= weights.spec_index(u as int).spec_len() {
                None
            } else {
                let ew = weights.spec_index(u as int).spec_index(v as int) as int;
                let new_total = total + ew;
                if !(i64::MIN as int <= new_total <= i64::MAX as int) {
                    None
                } else {
                    Self::spec_path_weight_int(path, weights, i + 1, new_total)
                }
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|path|), Span O(|path|) — sums edge weights along path; St sequential.
    fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>) -> (weight: Option<i64>) {
        let k = path.length();
        if k < 2 {
            return Some(0);
        }
        let mut total: i64 = 0;
        let mut i: usize = 0;
        let end = k - 1;
        while i < end
            invariant
                i <= end,
                end == k - 1,
                k == path.spec_len(),
                end < k,
                k >= 2,
                Self::spec_path_weight_int(path, weights, 0, 0) == Self::spec_path_weight_int(path, weights, i as int, total as int),
            decreases end - i,
        {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return None;
            }
            let edge_weight = *weights.nth(u).nth(v);
            match i64::checked_add(total, edge_weight) {
                Some(sum) => { total = sum; }
                None => { return None; }
            }
            i = i + 1;
        }
        Some(total)
    }
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|path|), Span O(|path|) — sums edge weights along path; St sequential.

    fn path_weight_float(
        path: &ArraySeqStPerS<usize>,
        weights: &ArraySeqStEphS<ArraySeqStEphS<WrappedF64>>,
    ) -> (weight: Option<WrappedF64>) {
        let k = path.length();
        if k < 2 {
            return Some(zero_dist());
        }
        let mut total = zero_dist();
        let mut i: usize = 0;
        let end = k - 1;
        while i < end
            invariant
                i <= end,
                end == k - 1,
                k == path.spec_len(),
                end < k,
            decreases end - i,
        {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return None;
            }
            let edge_weight = *weights.nth(u).nth(v);
            total = total.dist_add(&edge_weight);
            i = i + 1;
        }
        Some(total)
    }

    open spec fn spec_validate_subpath_int(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStEphS<i64>,
        weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
        i: int,
    ) -> bool
        decreases path.spec_len() - i,
    {
        if i < 0 || path.spec_len() < 2 || i >= path.spec_len() - 1 {
            true
        } else {
            let u = path.spec_index(i);
            let v = path.spec_index(i + 1);
            if u >= distances.spec_len() || v >= distances.spec_len() {
                false
            } else {
                let dist_u = distances.spec_index(u as int);
                let dist_v = distances.spec_index(v as int);
                if u >= weights.spec_len() || v >= weights.spec_index(u as int).spec_len() {
                    false
                } else {
                    let ew = weights.spec_index(u as int).spec_index(v as int);
                    if dist_u != i64::MAX {
                        let sum = dist_u as int + ew as int;
                        if !(i64::MIN as int <= sum <= i64::MAX as int) {
                            false
                        } else if dist_v != sum as i64 {
                            false
                        } else {
                            Self::spec_validate_subpath_int(path, distances, weights, i + 1)
                        }
                    } else {
                        Self::spec_validate_subpath_int(path, distances, weights, i + 1)
                    }
                }
            }
        }
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2), Span O(|V|^2) — checks triangle inequality for all vertex triples; St sequential.
    }

    fn validate_subpath_property_int(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStEphS<i64>,
        weights: &ArraySeqStEphS<ArraySeqStEphS<i64>>,
    ) -> (valid: bool) {
        let k = path.length();
        if k < 2 {
            return true;
        }
        let mut i: usize = 0;
        let end = k - 1;
        while i < end
            invariant
                i <= end,
                end == k - 1,
                k == path.spec_len(),
                end < k,
                k >= 2,
                Self::spec_validate_subpath_int(path, distances, weights, 0) == Self::spec_validate_subpath_int(path, distances, weights, i as int),
            decreases end - i,
        {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= distances.length() || v >= distances.length() {
                return false;
            }
            let dist_u = *distances.nth(u);
            let dist_v = *distances.nth(v);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return false;
            }
            let edge_weight = *weights.nth(u).nth(v);
            if dist_u != i64::MAX {
                match i64::checked_add(dist_u, edge_weight) {
                    Some(expected) => { if dist_v != expected { return false; } }
                    None => { return false; }
                }
            }
            i = i + 1;
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2), Span O(|V|^2) — checks triangle inequality for all vertex triples; St sequential.
        true
    }

    fn validate_subpath_property_float(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStEphS<WrappedF64>,
        weights: &ArraySeqStEphS<ArraySeqStEphS<WrappedF64>>,
    ) -> (valid: bool) {
        let k = path.length();
        if k < 2 {
            return true;
        }
        let mut i: usize = 0;
        let end = k - 1;
        while i < end
            invariant
                i <= end,
                end == k - 1,
                k == path.spec_len(),
                end < k,
            decreases end - i,
        {
            let u = *path.nth(i);
            let v = *path.nth(i + 1);
            if u >= distances.length() || v >= distances.length() {
                return false;
            }
            let dist_u = *distances.nth(u);
            let dist_v = *distances.nth(v);
            if u >= weights.length() || v >= weights.nth(u).length() {
                return false;
            }
            let edge_weight = *weights.nth(u).nth(v);
            if dist_u.is_finite() {
                let expected = dist_u.dist_add(&edge_weight);
                let ok = if dist_v.is_finite() && expected.is_finite() {
                    dist_v.approx_eq(&expected)
                } else {
                    dist_v.eq(&expected)
                };
                if !ok {
                    return false;
                }
            }
            i = i + 1;
        }
        true
    }
    } // impl PathWeightUtilsStEphTrait for PathWeightUtilsStEphS

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for PathWeightUtilsStEphS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PathWeightUtilsStEphS")
        }
    }

    impl std::fmt::Display for PathWeightUtilsStEphS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PathWeightUtilsStEph")
        }
    }
}
