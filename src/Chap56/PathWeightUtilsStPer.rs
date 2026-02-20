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

    use vstd::prelude::*;

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

    // 8. traits

    pub trait PathWeightUtilsStPerTrait {
        fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>) -> Option<i64>;

        fn path_weight_float(
            path: &ArraySeqStPerS<usize>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<F64Dist>>,
        ) -> Option<F64Dist>;

        fn validate_subpath_property_int(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStPerS<i64>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>,
        ) -> bool;

        fn validate_subpath_property_float(
            path: &ArraySeqStPerS<usize>,
            distances: &ArraySeqStPerS<F64Dist>,
            weights: &ArraySeqStPerS<ArraySeqStPerS<F64Dist>>,
        ) -> bool;
    }

    // 9. impls

    pub fn path_weight_int(path: &ArraySeqStPerS<usize>, weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>) -> Option<i64> {
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

    pub fn path_weight_float(
        path: &ArraySeqStPerS<usize>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<F64Dist>>,
    ) -> Option<F64Dist> {
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

    pub fn validate_subpath_property_int(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStPerS<i64>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<i64>>,
    ) -> bool {
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
            if dist_u != i64::MAX {
                match i64::checked_add(dist_u, edge_weight) {
                    Some(expected) => { if dist_v != expected { return false; } }
                    None => { return false; }
                }
            }
            i = i + 1;
        }
        true
    }

    #[verifier::external_body]
    pub fn validate_subpath_property_float(
        path: &ArraySeqStPerS<usize>,
        distances: &ArraySeqStPerS<F64Dist>,
        weights: &ArraySeqStPerS<ArraySeqStPerS<F64Dist>>,
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
            if dist_u.is_finite() && ((dist_v.val - (dist_u.val + edge_weight.val)).abs() > epsilon) {
                return false;
            }
        }
        true
    }

    } // verus!
}
