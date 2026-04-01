//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod AllPairsResultStEphI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStEphI64 {
        pub distances: ArraySeqStEphS<ArraySeqStEphS<i64>>,
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStEphI64Trait: Sized {
        spec fn spec_allpairsresultstephi64_wf(s: &AllPairsResultStEphI64) -> bool;

        spec fn spec_n(&self) -> usize;

        spec fn spec_distances_len(&self) -> nat;

        spec fn spec_distances_row_len(&self, u: int) -> nat;

        spec fn spec_distance_at(&self, u: int, v: int) -> i64;

        spec fn spec_predecessors_len(&self) -> nat;

        spec fn spec_predecessors_row_len(&self, u: int) -> nat;

        spec fn spec_predecessor_at(&self, u: int, v: int) -> usize;

        /// - Alg Analysis: APAS: (no cost stated) — data structure scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — initializes n x n distance and predecessor matrices.
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_n() == n,
                empty.spec_distances_len() == n as nat,
                empty.spec_predecessors_len() == n as nat,
                forall|r: int| #![trigger empty.spec_distances_row_len(r)]
                    0 <= r < n ==> empty.spec_distances_row_len(r) == n as nat,
                forall|r: int| #![trigger empty.spec_predecessors_row_len(r)]
                    0 <= r < n ==> empty.spec_predecessors_row_len(r) == n as nat,
                forall|r: int, c: int| #![trigger empty.spec_distance_at(r, c)]
                    0 <= r < n && 0 <= c < n ==>
                    empty.spec_distance_at(r, c) == (if c == r { 0i64 } else { UNREACHABLE }),
                forall|r: int, c: int| #![trigger empty.spec_predecessor_at(r, c)]
                    0 <= r < n && 0 <= c < n ==>
                    empty.spec_predecessor_at(r, c) == NO_PREDECESSOR;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two array index lookups.
        fn get_distance(&self, u: usize, v: usize) -> (dist: i64)
            ensures
                (u as int) >= self.spec_distances_len() ==> dist == UNREACHABLE,
                (u as int) < self.spec_distances_len() && (v as int) >= self.spec_distances_row_len(u as int) ==> dist == UNREACHABLE,
                (u as int) < self.spec_distances_len() && (v as int) < self.spec_distances_row_len(u as int) ==> dist == self.spec_distance_at(u as int, v as int);

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_distance(&mut self, u: usize, v: usize, dist: i64)
            ensures
                self.spec_n() == old(self).spec_n(),
                self.spec_distances_len() == old(self).spec_distances_len(),
                self.spec_predecessors_len() == old(self).spec_predecessors_len(),
                forall|r: int| #![trigger self.spec_distances_row_len(r)]
                    0 <= r < old(self).spec_distances_len()
                    ==> self.spec_distances_row_len(r) == old(self).spec_distances_row_len(r),
                forall|r: int| #![trigger self.spec_predecessors_row_len(r)]
                    0 <= r < old(self).spec_predecessors_len()
                    ==> self.spec_predecessors_row_len(r) == old(self).spec_predecessors_row_len(r),
                (u as int) < old(self).spec_distances_len()
                    && (v as int) < old(self).spec_distances_row_len(u as int)
                    ==> self.spec_distance_at(u as int, v as int) == dist,
                forall|r: int, c: int| #![trigger self.spec_distance_at(r, c)]
                    0 <= r < old(self).spec_distances_len()
                    && 0 <= c < old(self).spec_distances_row_len(r)
                    && (r != u as int || c != v as int)
                    ==> self.spec_distance_at(r, c) == old(self).spec_distance_at(r, c),
                forall|r: int, c: int| #![trigger self.spec_predecessor_at(r, c)]
                    0 <= r < old(self).spec_predecessors_len()
                    && 0 <= c < old(self).spec_predecessors_row_len(r)
                    ==> self.spec_predecessor_at(r, c) == old(self).spec_predecessor_at(r, c);

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — two array index lookups.
        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>)
            ensures
                (u as int) >= self.spec_predecessors_len() ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) >= self.spec_predecessors_row_len(u as int) ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) < self.spec_predecessors_row_len(u as int) && self.spec_predecessor_at(u as int, v as int) == NO_PREDECESSOR ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) < self.spec_predecessors_row_len(u as int) && self.spec_predecessor_at(u as int, v as int) != NO_PREDECESSOR ==> pred == Some(self.spec_predecessor_at(u as int, v as int));

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
            ensures
                self.spec_n() == old(self).spec_n(),
                self.spec_predecessors_len() == old(self).spec_predecessors_len(),
                self.spec_distances_len() == old(self).spec_distances_len(),
                forall|r: int| #![trigger self.spec_predecessors_row_len(r)]
                    0 <= r < old(self).spec_predecessors_len()
                    ==> self.spec_predecessors_row_len(r) == old(self).spec_predecessors_row_len(r),
                forall|r: int| #![trigger self.spec_distances_row_len(r)]
                    0 <= r < old(self).spec_distances_len()
                    ==> self.spec_distances_row_len(r) == old(self).spec_distances_row_len(r),
                (u as int) < old(self).spec_predecessors_len()
                    && (v as int) < old(self).spec_predecessors_row_len(u as int)
                    ==> self.spec_predecessor_at(u as int, v as int) == pred,
                forall|r: int, c: int| #![trigger self.spec_predecessor_at(r, c)]
                    0 <= r < old(self).spec_predecessors_len()
                    && 0 <= c < old(self).spec_predecessors_row_len(r)
                    && (r != u as int || c != v as int)
                    ==> self.spec_predecessor_at(r, c) == old(self).spec_predecessor_at(r, c),
                forall|r: int, c: int| #![trigger self.spec_distance_at(r, c)]
                    0 <= r < old(self).spec_distances_len()
                    && 0 <= c < old(self).spec_distances_row_len(r)
                    ==> self.spec_distance_at(r, c) == old(self).spec_distance_at(r, c);

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, u: usize, v: usize) -> (b: bool)
            ensures
                (u as int) >= self.spec_distances_len() ==> !b,
                (u as int) < self.spec_distances_len() && (v as int) >= self.spec_distances_row_len(u as int) ==> !b,
                (u as int) < self.spec_distances_len() && (v as int) < self.spec_distances_row_len(u as int) ==>
                    b == (self.spec_distance_at(u as int, v as int) != UNREACHABLE);

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Claude-Opus-4.6: Work O(n), Span O(n) — predecessor chain traversal + reversal.
        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            ensures
                (u as int) >= self.spec_predecessors_len() ==> path is None,
                (v as int) >= self.spec_predecessors_len() ==> path is None,
                u != v && (u as int) < self.spec_distances_len()
                    && (v as int) < self.spec_distances_row_len(u as int)
                    && self.spec_distance_at(u as int, v as int) == UNREACHABLE
                    ==> path is None,
                path is Some ==> path->Some_0.spec_len() >= 1,
                path is Some ==> path->Some_0.spec_index(0) == u,
                path is Some ==> path->Some_0.spec_index(path->Some_0.spec_len() - 1) == v,
                path is Some && u != v ==>
                    forall|j: int| #![trigger path->Some_0.spec_index(j)]
                        0 <= j < path->Some_0.spec_len()
                        ==> (path->Some_0.spec_index(j) as int) < self.spec_predecessors_row_len(u as int);
    }

    // 9. impls

    impl AllPairsResultStEphI64Trait for AllPairsResultStEphI64 {
        open spec fn spec_allpairsresultstephi64_wf(s: &AllPairsResultStEphI64) -> bool {
            s.distances.spec_len() == s.n as nat
            && s.predecessors.spec_len() == s.n as nat
            && forall|r: int| #![trigger s.distances.spec_index(r)]
                0 <= r < s.n ==> s.distances.spec_index(r).spec_len() == s.n as nat
            && forall|r: int| #![trigger s.predecessors.spec_index(r)]
                0 <= r < s.n ==> s.predecessors.spec_index(r).spec_len() == s.n as nat
        }

        open spec fn spec_n(&self) -> usize { self.n }

        open spec fn spec_distances_len(&self) -> nat { self.distances.spec_len() }

        open spec fn spec_distances_row_len(&self, u: int) -> nat { self.distances.spec_index(u).spec_len() }

        open spec fn spec_distance_at(&self, u: int, v: int) -> i64 { self.distances.spec_index(u).spec_index(v) }

        open spec fn spec_predecessors_len(&self) -> nat { self.predecessors.spec_len() }

        open spec fn spec_predecessors_row_len(&self, u: int) -> nat { self.predecessors.spec_index(u).spec_len() }

        open spec fn spec_predecessor_at(&self, u: int, v: int) -> usize { self.predecessors.spec_index(u).spec_index(v) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — allocates n×n distance + predecessor matrices.
        fn new(n: usize) -> (empty: Self)
            ensures
                Self::spec_allpairsresultstephi64_wf(&empty),
        {
            let mut dist_rows: Vec<ArraySeqStEphS<i64>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_rows@.len() == i as int,
                    forall|r: int| #![trigger dist_rows@[r]]
                        0 <= r < i ==> dist_rows@[r].spec_len() == n as nat,
                    forall|r: int, c: int| #![trigger dist_rows@[r].spec_index(c)]
                        0 <= r < i && 0 <= c < n ==>
                        dist_rows@[r].spec_index(c) == (if c == r { 0i64 } else { UNREACHABLE }),
                decreases n - i,
            {
                let mut row: Vec<i64> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant
                        j <= n,
                        row@.len() == j as int,
                        forall|c: int| #![trigger row@[c]]
                            0 <= c < j ==> row@[c] == (if c == i as int { 0i64 } else { UNREACHABLE }),
                    decreases n - j,
                {
                    if j == i { row.push(0i64); } else { row.push(UNREACHABLE); }
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
                    forall|r: int, c: int| #![trigger pred_rows@[r].spec_index(c)]
                        0 <= r < k && 0 <= c < n ==>
                        pred_rows@[r].spec_index(c) == NO_PREDECESSOR,
                decreases n - k,
            {
                let mut prow: Vec<usize> = Vec::new();
                let mut m: usize = 0;
                while m < n
                    invariant
                        m <= n,
                        prow@.len() == m as int,
                        forall|c: int| #![trigger prow@[c]]
                            0 <= c < m ==> prow@[c] == NO_PREDECESSOR,
                    decreases n - m,
                {
                    prow.push(NO_PREDECESSOR);
                    m = m + 1;
                }
                pred_rows.push(ArraySeqStEphS { seq: prow });
                k = k + 1;
            }
            AllPairsResultStEphI64 {
                distances: ArraySeqStEphS { seq: dist_rows },
                predecessors: ArraySeqStEphS { seq: pred_rows },
                n,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index read.
        fn get_distance(&self, u: usize, v: usize) -> (dist: i64)
        {
            if u >= self.distances.length() {
                return UNREACHABLE;
            }
            let row = self.distances.nth(u);
            if v >= row.length() {
                return UNREACHABLE;
            }
            *row.nth(v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_distance(&mut self, u: usize, v: usize, dist: i64)
        {
            if u < self.distances.seq.len() {
                let mut row = self.distances.seq[u].clone();
                if v < row.seq.len() {
                    row.seq.set(v, dist);
                    self.distances.seq.set(u, row);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — 2D array index read.
        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>)
        {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clones row, updates cell, replaces row.
        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
        {
            if u < self.predecessors.seq.len() {
                let mut row = self.predecessors.seq[u].clone();
                if v < row.seq.len() {
                    row.seq.set(v, pred);
                    self.predecessors.seq.set(u, row);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) — follows predecessor chain then reverses; St sequential.
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
                    path@[0] == v,
                    path@[path@.len() - 1] == current,
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < row_len,
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
                    path@[0] == v,
                    path@[path@.len() - 1] == u,
                    forall|j: int| #![trigger reversed@[j]]
                        0 <= j < reversed@.len() ==> reversed@[j] == path@[path_len - 1 - j],
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < row_len,
                    row_len as int == pred_row.spec_len(),
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

    impl std::fmt::Debug for AllPairsResultStEphI64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResultStEphI64(n={})", self.n)
        }
    }

    impl std::fmt::Display for AllPairsResultStEphI64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AllPairsResult(n={})", self.n)
        }
    }
}
