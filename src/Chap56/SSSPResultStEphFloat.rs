//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Float Weights)
//!
//! Uses a `F64Dist` newtype around f64 with a View impl so that Verus can reason about
//! the distance array. At runtime, callers convert to/from `OrderedF64` at the boundary.

pub mod SSSPResultStEphFloat {

    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 6. spec fns
    // 9. impls

    // 4. type definitions

    pub const NO_PREDECESSOR: usize = usize::MAX;

    // Newtype wrapper for f64 distances, giving Verus a View impl.
    // OrderedFloat<f64> is not available during verification (cfg-gated out),
    // and f64 has no View impl in vstd.
    #[derive(Clone, Copy)]
    pub struct F64Dist {
        pub val: f64,
    }

    impl View for F64Dist {
        type V = f64;
        open spec fn view(&self) -> f64 { self.val }
    }

    impl F64Dist {
        pub open spec fn spec_is_finite(&self) -> bool {
            self.val.is_finite_spec()
        }

        #[verifier::external_body]
        pub fn is_finite(&self) -> (b: bool)
            ensures b == self.spec_is_finite()
        {
            self.val.is_finite()
        }

        #[verifier::external_body]
        pub fn eq(&self, other: &Self) -> (b: bool)
            ensures b == (self@ == other@)
        {
            self.val == other.val
        }
    }

    pub uninterp spec fn UNREACHABLE_SPEC() -> f64;

    #[verifier::external_body]
    pub broadcast proof fn axiom_unreachable_not_finite()
        ensures #[trigger] UNREACHABLE_SPEC().is_finite_spec() == false
    {}

    #[verifier::external_body]
    pub fn unreachable_dist() -> (d: F64Dist)
        ensures d@ == UNREACHABLE_SPEC(),
                !d.spec_is_finite(),
    {
        F64Dist { val: f64::INFINITY }
    }

    #[verifier::external_body]
    pub fn zero_dist() -> (d: F64Dist)
        ensures d.spec_is_finite(),
    {
        F64Dist { val: 0.0 }
    }

    // 6. spec fns

    /// Result structure for single-source shortest paths with floating-point weights.
    pub struct SSSPResultStEphFloat {
        pub distances: ArraySeqStEphS<F64Dist>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    // 9. impls

    impl SSSPResultStEphFloat {
        pub fn new(n: usize, source: usize) -> (result: Self)
            requires source < n,
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_vec: Vec<F64Dist> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_vec@.len() == i as int,
                    n <= usize::MAX,
                decreases n - i,
            {
                if i == source {
                    dist_vec.push(zero);
                } else {
                    dist_vec.push(unreach);
                }
                i = i + 1;
            }
            let distances = ArraySeqStEphS::from_vec(dist_vec);
            let predecessors = ArraySeqStEphS::<usize>::new(n, NO_PREDECESSOR);
            SSSPResultStEphFloat { distances, predecessors, source }
        }

        pub fn get_distance(&self, v: usize) -> (dist: F64Dist) {
            if v >= self.distances.length() {
                return unreachable_dist();
            }
            *self.distances.nth(v)
        }

        pub fn set_distance(&mut self, v: usize, dist: F64Dist) {
            if v < self.distances.length() {
                let _ = self.distances.set(v, dist);
            }
        }

        pub fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        pub fn set_predecessor(&mut self, v: usize, pred: usize) {
            if v < self.predecessors.length() {
                let _ = self.predecessors.set(v, pred);
            }
        }

        pub fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v).is_finite()
        }
    }

    } // verus!

    impl std::fmt::Debug for F64Dist {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "F64Dist({})", self.val)
        }
    }

    impl PartialEq for F64Dist {
        fn eq(&self, other: &Self) -> bool { self.val == other.val }
    }

    #[cfg(not(verus_keep_ghost))]
    impl SSSPResultStEphFloat {
        pub fn extract_path(&self, v: usize) -> Option<ArraySeqStPerS<usize>> {
            if !self.is_reachable(v) {
                return None;
            }
            let mut path = Vec::new();
            let mut current = v;
            path.push(current);
            while current != self.source {
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR { return None; }
                path.push(pred);
                current = pred;
            }
            path.reverse();
            Some(ArraySeqStPerS::from_vec(path))
        }
    }
}
