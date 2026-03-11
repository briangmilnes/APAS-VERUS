//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Persistent (Float Weights)
//!
//! Uses `WrappedF64` from vstdplus::float for distances with persistent array sequences.

pub mod AllPairsResultStPerF64 {

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

    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStPerF64 {
        pub distances: ArraySeqStPerS<ArraySeqStPerS<WrappedF64>>,
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStPerF64Trait: Sized {
        spec fn spec_allpairsresultstperf64_wf(&self) -> bool;

        spec fn spec_n(&self) -> usize;

        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_allpairsresultstperf64_wf(),
                empty.spec_n() == n;

        fn get_distance(&self, u: usize, v: usize) -> (dist: WrappedF64)
            requires self.spec_allpairsresultstperf64_wf();

        fn set_distance(self, u: usize, v: usize, dist: WrappedF64) -> (updated: Self)
            requires self.spec_allpairsresultstperf64_wf(),
            ensures
                updated.spec_allpairsresultstperf64_wf(),
                updated.spec_n() == self.spec_n();

        fn get_predecessor(&self, u: usize, v: usize) -> (predecessor: Option<usize>)
            requires self.spec_allpairsresultstperf64_wf();

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (updated: Self)
            requires self.spec_allpairsresultstperf64_wf(),
            ensures
                updated.spec_allpairsresultstperf64_wf(),
                updated.spec_n() == self.spec_n();

        fn is_reachable(&self, u: usize, v: usize) -> (reachable: bool)
            requires self.spec_allpairsresultstperf64_wf();

        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            requires self.spec_allpairsresultstperf64_wf();
    }

    // 9. impls

    impl AllPairsResultStPerF64Trait for AllPairsResultStPerF64 {
        open spec fn spec_allpairsresultstperf64_wf(&self) -> bool {
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

        fn new(n: usize) -> (empty: Self)
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_rows: Vec<ArraySeqStPerS<WrappedF64>> = Vec::new();
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
                    invariant j <= n, row@.len() == j as int,
                    decreases n - j,
                {
                    if j == i { row.push(zero); } else { row.push(unreach); }
                    j = j + 1;
                }
                dist_rows.push(ArraySeqStPerS { seq: row });
                i = i + 1;
            }
            let mut pred_rows: Vec<ArraySeqStPerS<usize>> = Vec::new();
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
                    invariant m <= n, prow@.len() == m as int,
                    decreases n - m,
                {
                    prow.push(NO_PREDECESSOR);
                    m = m + 1;
                }
                pred_rows.push(ArraySeqStPerS { seq: prow });
                k = k + 1;
            }
            AllPairsResultStPerF64 {
                distances: ArraySeqStPerS { seq: dist_rows },
                predecessors: ArraySeqStPerS { seq: pred_rows },
                n,
            }
        }

        fn get_distance(&self, u: usize, v: usize) -> (dist: WrappedF64) {
            if u >= self.distances.length() {
                return unreachable_dist();
            }
            let row = self.distances.nth(u);
            if v >= row.length() {
                return unreachable_dist();
            }
            *row.nth(v)
        }

        fn set_distance(self, u: usize, v: usize, dist: WrappedF64) -> (updated: Self)
        {
            if u >= self.distances.seq.len() || v >= self.n { return self; }
            let ghost old_distances = self.distances;
            let ghost n = self.n;
            assert(self.distances.spec_index(u as int).spec_len() == n as nat);
            let mut row_vec = self.distances.seq[u].seq.clone();
            if v < row_vec.len() {
                row_vec.set(v, dist);
            }
            let updated_row = ArraySeqStPerS { seq: row_vec };
            assert(updated_row.spec_len() == n as nat);
            let mut dist_vec = self.distances.seq;
            dist_vec.set(u, updated_row);
            assert forall|r: int| 0 <= r < n
                implies #[trigger] dist_vec@[r].spec_len() == n as nat
            by {
                if r == u as int {
                } else {
                    assert(dist_vec@[r] == old_distances.spec_index(r));
                }
            };
            AllPairsResultStPerF64 {
                distances: ArraySeqStPerS { seq: dist_vec },
                predecessors: self.predecessors,
                n: self.n,
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
            let pred = *row.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (updated: Self)
        {
            if u >= self.predecessors.seq.len() || v >= self.n { return self; }
            let ghost old_predecessors = self.predecessors;
            let ghost n = self.n;
            assert(self.predecessors.spec_index(u as int).spec_len() == n as nat);
            let mut row_vec = self.predecessors.seq[u].seq.clone();
            if v < row_vec.len() {
                row_vec.set(v, pred);
            }
            let updated_row = ArraySeqStPerS { seq: row_vec };
            assert(updated_row.spec_len() == n as nat);
            let mut pred_vec = self.predecessors.seq;
            pred_vec.set(u, updated_row);
            assert forall|r: int| 0 <= r < n
                implies #[trigger] pred_vec@[r].spec_len() == n as nat
            by {
                if r == u as int {
                } else {
                    assert(pred_vec@[r] == old_predecessors.spec_index(r));
                }
            };
            AllPairsResultStPerF64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS { seq: pred_vec },
                n: self.n,
            }
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
            if v >= row_len { return None; }
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
                if current >= row_len { return None; }
                let pred = *pred_row.nth(current);
                if pred == NO_PREDECESSOR || pred >= row_len { return None; }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != u { return None; }
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

    impl std::fmt::Debug for AllPairsResultStPerF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResultStPerF64(n={})", self.n)
        }
    }

    impl std::fmt::Display for AllPairsResultStPerF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResult(n={})", self.n)
        }
    }
}
