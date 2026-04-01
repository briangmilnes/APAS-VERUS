//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Float Weights)
//!
//! Uses `WrappedF64` from vstdplus::float for distances with ephemeral array sequences.

pub mod AllPairsResultStEphF64 {

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

    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStEphF64 {
        pub distances: ArraySeqStEphS<ArraySeqStEphS<WrappedF64>>,
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStEphF64Trait: Sized {
        spec fn spec_allpairsresultstephf64_wf(&self) -> bool;

        spec fn spec_n(&self) -> usize;

        /// - Alg Analysis: APAS: (no cost stated) — data structure scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — initializes n x n distance and predecessor matrices.
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_allpairsresultstephf64_wf(),
                empty.spec_n() == n;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two array index lookups.
        fn get_distance(&self, u: usize, v: usize) -> (dist: WrappedF64)
            requires self.spec_allpairsresultstephf64_wf();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_distance(&mut self, u: usize, v: usize, dist: WrappedF64)
            requires old(self).spec_allpairsresultstephf64_wf(),
            ensures
                self.spec_allpairsresultstephf64_wf(),
                self.spec_n() == old(self).spec_n();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two array index lookups.
        fn get_predecessor(&self, u: usize, v: usize) -> (predecessor: Option<usize>)
            requires self.spec_allpairsresultstephf64_wf();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
            requires old(self).spec_allpairsresultstephf64_wf(),
            ensures
                self.spec_allpairsresultstephf64_wf(),
                self.spec_n() == old(self).spec_n();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, u: usize, v: usize) -> (reachable: bool)
            requires self.spec_allpairsresultstephf64_wf();

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — predecessor chain traversal + reversal.
        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            requires self.spec_allpairsresultstephf64_wf();
    }

    // 9. impls

    impl AllPairsResultStEphF64Trait for AllPairsResultStEphF64 {
        open spec fn spec_allpairsresultstephf64_wf(&self) -> bool {
            self.distances.spec_len() == self.n as nat
            && self.predecessors.spec_len() == self.n as nat
            && forall|r: int|
                #![trigger self.distances.spec_index(r)]
                #![trigger self.predecessors.spec_index(r)]
                0 <= r < self.n ==> (
                    self.distances.spec_index(r).spec_len() == self.n as nat
                    && self.predecessors.spec_index(r).spec_len() == self.n as nat
                )
        }

        open spec fn spec_n(&self) -> usize { self.n }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — allocates n×n distance + predecessor matrices.
        fn new(n: usize) -> (empty: Self)
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_rows: Vec<ArraySeqStEphS<WrappedF64>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_rows@.len() == i as int,
                    forall|r: int| #![trigger dist_rows@[r]]
                        0 <= r < i ==> dist_rows@[r].spec_len() == n as nat,
                decreases n - i,
            {
                let mut row: Vec<WrappedF64> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant
                        j <= n,
                        row@.len() == j as int,
                    decreases n - j,
                {
                    if j == i { row.push(zero); } else { row.push(unreach); }
                    j = j + 1;
                }
                dist_rows.push(ArraySeqStEphS { seq: row });
                i = i + 1;
            }
            let mut pred_rows: Vec<ArraySeqStEphS<usize>> = Vec::new();
            let mut k: usize = 0;
            while k < n
                invariant
                    k <= n,
                    pred_rows@.len() == k as int,
                    forall|r: int| #![trigger pred_rows@[r]]
                        0 <= r < k ==> pred_rows@[r].spec_len() == n as nat,
                decreases n - k,
            {
                let mut prow: Vec<usize> = Vec::new();
                let mut m: usize = 0;
                while m < n
                    invariant
                        m <= n,
                        prow@.len() == m as int,
                    decreases n - m,
                {
                    prow.push(NO_PREDECESSOR);
                    m = m + 1;
                }
                pred_rows.push(ArraySeqStEphS { seq: prow });
                k = k + 1;
            }
            AllPairsResultStEphF64 {
                distances: ArraySeqStEphS { seq: dist_rows },
                predecessors: ArraySeqStEphS { seq: pred_rows },
                n,
            }
        }
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index read.

        fn get_distance(&self, u: usize, v: usize) -> (dist: WrappedF64) {
            if u >= self.distances.length() {
                return unreachable_dist();
            }
            let row = self.distances.nth(u);
            if v >= row.length() {
                return unreachable_dist();
            }
            *row.nth(v)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index write.
        }

        fn set_distance(&mut self, u: usize, v: usize, dist: WrappedF64)
        {
            if u < self.distances.length() {
                let row_ref = self.distances.nth(u);
                if v < row_ref.length() {
                    let ghost old_distances = self.distances;
                    let ghost n = self.n;
                    let mut row = row_ref.clone();
                    let _ = row.set(v, dist);
                    assert(row.spec_len() == n as nat);
                    let _ = self.distances.set(u, row);
                    assert forall|r: int| 0 <= r < n
                        implies #[trigger] self.distances.spec_index(r).spec_len() == n as nat
                    by {
                        if r == u as int {
                        } else {
                            assert(self.distances.spec_index(r) == old_distances.spec_index(r));
                        }
                    };
                }
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index read.
            }
        }

        fn get_predecessor(&self, u: usize, v: usize) -> (predecessor: Option<usize>) {
            if u >= self.predecessors.length() {
                return None;
            }
            let row = self.predecessors.nth(u);
            if v >= row.length() {
                return None;
            }
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index write.
            let pred = *row.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
        {
            if u < self.predecessors.length() {
                let row_ref = self.predecessors.nth(u);
                if v < row_ref.length() {
                    let ghost old_predecessors = self.predecessors;
                    let ghost n = self.n;
                    let mut row = row_ref.clone();
                    let _ = row.set(v, pred);
                    assert(row.spec_len() == n as nat);
                    let _ = self.predecessors.set(u, row);
                    assert forall|r: int| 0 <= r < n
                        implies #[trigger] self.predecessors.spec_index(r).spec_len() == n as nat
                    by {
                        if r == u as int {
                        } else {
                            assert(self.predecessors.spec_index(r) == old_predecessors.spec_index(r));
                        }
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index read.
                    };
                }
            }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — follows predecessor chain; St sequential.
        }

        fn is_reachable(&self, u: usize, v: usize) -> (reachable: bool) {
            self.get_distance(u, v).is_finite()
        }

        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>) {
            if u >= self.predecessors.length() || v >= self.predecessors.length() {
                return None;
            }
            if u == v {
                let mut single: Vec<usize> = Vec::new();
                single.push(u);
                return Some(ArraySeqStPerS::from_vec(single));
            }
            if !self.is_reachable(u, v) { return None; }
            let pred_row = self.predecessors.nth(u);
            let row_len = pred_row.length();
            if v >= row_len {
                return None;
            }
            let mut path: Vec<usize> = Vec::new();
            let mut current: usize = v;
            path.push(current);
            let mut steps: usize = 0;
            while current != u && steps < row_len
                invariant
                    steps <= row_len,
                    current < row_len,
                    row_len as int == pred_row.spec_len(),
                    path@.len() > 0,
                decreases row_len - steps,
            {
                if current >= row_len {
                    return None;
                }
                let pred = *pred_row.nth(current);
                if pred == NO_PREDECESSOR || pred >= row_len {
                    return None;
                }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != u {
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

    impl std::fmt::Debug for AllPairsResultStEphF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResultStEphF64(n={})", self.n)
        }
    }

    impl std::fmt::Display for AllPairsResultStEphF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResult(n={})", self.n)
        }
    }
}
