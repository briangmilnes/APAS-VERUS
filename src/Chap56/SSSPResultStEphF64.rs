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
    // 3. broadcast use
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 3. broadcast use

    broadcast use crate::vstdplus::float::float::axiom_f64_unreachable_not_finite;

    // 4. type definitions

    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStEphF64 {
        pub distances: ArraySeqStEphS<WrappedF64>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    // 8. traits

    pub trait SSSPResultStEphF64Trait: Sized {
        spec fn spec_ssspresultstephf64_wf(&self) -> bool;

        spec fn spec_distances(&self) -> Seq<WrappedF64>;

        spec fn spec_predecessors(&self) -> Seq<usize>;

        spec fn spec_source(&self) -> usize;

        /// - Alg Analysis: APAS: (no cost stated) — data structure scaffolding.
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — initializes distance and predecessor arrays.
        fn new(n: usize, source: usize) -> (empty: Self)
            requires source < n,
            ensures
                empty.spec_ssspresultstephf64_wf(),
                empty.spec_distances().len() == n,
                empty.spec_predecessors().len() == n,
                empty.spec_source() == source;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — array index lookup.
        fn get_distance(&self, v: usize) -> (dist: WrappedF64)
            requires self.spec_ssspresultstephf64_wf(),
            ensures
                v >= self.spec_distances().len() ==> dist@ == UNREACHABLE_SPEC(),
                v < self.spec_distances().len() ==> dist == self.spec_distances()[v as int];

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — array index update.
        fn set_distance(&mut self, v: usize, dist: WrappedF64)
            requires old(self).spec_ssspresultstephf64_wf(),
            ensures
                self.spec_ssspresultstephf64_wf(),
                self.spec_distances().len() == old(self).spec_distances().len(),
                v < old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances().update(v as int, dist),
                v >= old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances(),
                self.spec_predecessors() == old(self).spec_predecessors(),
                self.spec_source() == old(self).spec_source();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — array index lookup.
        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>)
            requires self.spec_ssspresultstephf64_wf(),
            ensures
                v >= self.spec_predecessors().len() ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] == NO_PREDECESSOR ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] != NO_PREDECESSOR ==> pred == Some(self.spec_predecessors()[v as int]);

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — array index update.
        fn set_predecessor(&mut self, v: usize, pred: usize)
            requires old(self).spec_ssspresultstephf64_wf(),
            ensures
                self.spec_ssspresultstephf64_wf(),
                self.spec_predecessors().len() == old(self).spec_predecessors().len(),
                v < old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors().update(v as int, pred),
                v >= old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors(),
                self.spec_distances() == old(self).spec_distances(),
                self.spec_source() == old(self).spec_source();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, v: usize) -> (b: bool)
            requires self.spec_ssspresultstephf64_wf(),
            ensures
                v >= self.spec_distances().len() ==> !b,
                v < self.spec_distances().len() ==> b == self.spec_distances()[v as int].spec_is_finite();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — predecessor chain traversal + reversal.
        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            requires self.spec_ssspresultstephf64_wf();
    }

    // 9. impls

    impl SSSPResultStEphF64Trait for SSSPResultStEphF64 {
        open spec fn spec_ssspresultstephf64_wf(&self) -> bool {
            self.distances.seq@.len() == self.predecessors.seq@.len()
            && self.source < self.distances.seq@.len()
        }

        open spec fn spec_distances(&self) -> Seq<WrappedF64> { self.distances.seq@ }

        open spec fn spec_predecessors(&self) -> Seq<usize> { self.predecessors.seq@ }

        open spec fn spec_source(&self) -> usize { self.source }

        fn new(n: usize, source: usize) -> (empty: Self)
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

        fn get_distance(&self, v: usize) -> (dist: WrappedF64) {
            if v >= self.distances.length() {
                return unreachable_dist();
            }
            *self.distances.nth(v)
        }

        fn set_distance(&mut self, v: usize, dist: WrappedF64) {
            if v < self.distances.length() {
                self.distances.seq.set(v, dist);
            }
        }

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(&mut self, v: usize, pred: usize) {
            if v < self.predecessors.length() {
                self.predecessors.seq.set(v, pred);
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool) {
            let dist = self.get_distance(v);
            dist.is_finite()
        }

        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) {
                return None;
            }
            let n = self.predecessors.length();
            if v >= n { return None; }
            let mut path: Vec<usize> = Vec::new();
            let mut current: usize = v;
            path.push(current);
            let mut steps: usize = 0;
            while current != self.source && steps < n
                invariant
                    steps <= n,
                    n == self.predecessors.spec_len(),
                    path@.len() > 0,
                decreases n - steps,
            {
                if current >= n {
                    return None;
                }
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR { return None; }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != self.source {
                return None;
            }
            let path_len = path.len();
            let mut reversed: Vec<usize> = Vec::new();
            let mut k: usize = path_len;
            while k > 0
                invariant
                    k <= path_len,
                    path_len == path@.len(),
                    reversed@.len() == (path_len - k) as int,
                decreases k,
            {
                k = k - 1;
                reversed.push(path[k]);
            }
            Some(ArraySeqStPerS::from_vec(reversed))
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for SSSPResultStEphF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SSSPResultStEphF64(source={}, n={})", self.source, self.distances.length())
        }
    }

    impl std::fmt::Display for SSSPResultStEphF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SSSPResult(source={}, n={})", self.source, self.distances.length())
        }
    }
}
