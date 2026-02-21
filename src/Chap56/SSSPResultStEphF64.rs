//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Float Weights)
//!
//! Uses `WrappedF64` from vstdplus::float for distances, giving Verus a View impl
//! over f64 values stored in ArraySeq containers.

pub mod SSSPResultStEphF64 {

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::float::float::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 9. impls

    // 4. type definitions

    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStEphF64 {
        pub distances: ArraySeqStEphS<WrappedF64>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    // 9. impls

    impl SSSPResultStEphF64 {
        pub fn new(n: usize, source: usize) -> (result: Self)
            requires source < n,
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_vec: Vec<WrappedF64> = Vec::new();
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
            SSSPResultStEphF64 { distances, predecessors, source }
        }

        pub fn get_distance(&self, v: usize) -> (dist: WrappedF64) {
            if v >= self.distances.length() {
                return unreachable_dist();
            }
            *self.distances.nth(v)
        }

        pub fn set_distance(&mut self, v: usize, dist: WrappedF64) {
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

    impl SSSPResultStEphF64 {
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
