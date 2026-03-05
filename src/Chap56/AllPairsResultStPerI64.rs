//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Persistent (Integer Weights)

pub mod AllPairsResultStPerI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    // Table of Contents
    // 6. spec fns
    // 8. traits
    // 9. impls

    pub struct AllPairsResultStPerI64 {
        pub distances: ArraySeqStPerS<ArraySeqStPerS<i64>>,
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        pub n: usize,
    }

    // 6. spec fns

    pub open spec fn spec_allpairsresultstperi64_wf(s: &AllPairsResultStPerI64) -> bool {
        s.distances.spec_len() == s.n as nat
        && s.predecessors.spec_len() == s.n as nat
        && forall|r: int| #![trigger s.distances.spec_index(r)]
            0 <= r < s.n ==> s.distances.spec_index(r).spec_len() == s.n as nat
        && forall|r: int| #![trigger s.predecessors.spec_index(r)]
            0 <= r < s.n ==> s.predecessors.spec_index(r).spec_len() == s.n as nat
    }

    // 8. traits

    pub trait AllPairsResultStPerI64Trait: Sized {
        fn new(n: usize) -> (empty: Self);

        fn get_distance(&self, u: usize, v: usize) -> (dist: i64);

        fn set_distance(self, u: usize, v: usize, dist: i64) -> (updated: Self);

        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (updated: Self);

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool);

        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStPerI64Trait for AllPairsResultStPerI64 {
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.n == n,
                empty.distances.spec_len() == n as nat,
                empty.predecessors.spec_len() == n as nat,
                forall|r: int| #![trigger empty.distances.spec_index(r)]
                    0 <= r < n ==> empty.distances.spec_index(r).spec_len() == n as nat,
                forall|r: int| #![trigger empty.predecessors.spec_index(r)]
                    0 <= r < n ==> empty.predecessors.spec_index(r).spec_len() == n as nat,
                forall|r: int, c: int| #![trigger empty.distances.spec_index(r).spec_index(c)]
                    0 <= r < n && 0 <= c < n ==>
                    empty.distances.spec_index(r).spec_index(c) == (if c == r { 0i64 } else { UNREACHABLE }),
                forall|r: int, c: int| #![trigger empty.predecessors.spec_index(r).spec_index(c)]
                    0 <= r < n && 0 <= c < n ==>
                    empty.predecessors.spec_index(r).spec_index(c) == NO_PREDECESSOR,
                spec_allpairsresultstperi64_wf(&empty),
        {
            let mut dist_rows: Vec<ArraySeqStPerS<i64>> = Vec::new();
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
                pred_rows.push(ArraySeqStPerS { seq: prow });
                k = k + 1;
            }
            AllPairsResultStPerI64 {
                distances: ArraySeqStPerS { seq: dist_rows },
                predecessors: ArraySeqStPerS { seq: pred_rows },
                n,
            }
        }

        fn get_distance(&self, u: usize, v: usize) -> (dist: i64)
            ensures
                u >= self.distances.spec_len() ==> dist == UNREACHABLE,
                u < self.distances.spec_len() && v >= self.distances.spec_index(u as int).spec_len() ==> dist == UNREACHABLE,
                u < self.distances.spec_len() && v < self.distances.spec_index(u as int).spec_len() ==> dist == self.distances.spec_index(u as int).spec_index(v as int),
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

        fn set_distance(self, u: usize, v: usize, dist: i64) -> (updated: Self)
            ensures
                updated.n == self.n,
                updated.predecessors == self.predecessors,
                updated.distances.spec_len() == self.distances.spec_len(),
                forall|r: int| #![trigger updated.distances.spec_index(r)]
                    0 <= r < self.distances.spec_len()
                    ==> updated.distances.spec_index(r).spec_len() == self.distances.spec_index(r).spec_len(),
                u < self.distances.spec_len() && v < self.n && v < self.distances.spec_index(u as int).spec_len()
                    ==> updated.distances.spec_index(u as int).spec_index(v as int) == dist,
                forall|r: int, c: int| #![trigger updated.distances.spec_index(r).spec_index(c)]
                    0 <= r < self.distances.spec_len()
                    && 0 <= c < self.distances.spec_index(r).spec_len()
                    && (r != u as int || c != v as int)
                    ==> updated.distances.spec_index(r).spec_index(c) == self.distances.spec_index(r).spec_index(c),
        {
            if u >= self.distances.seq.len() || v >= self.n { return self; }
            let mut row_vec = self.distances.seq[u].seq.clone();
            if v < row_vec.len() {
                row_vec.set(v, dist);
            }
            let updated_row = ArraySeqStPerS { seq: row_vec };
            let mut dist_vec = self.distances.seq;
            dist_vec.set(u, updated_row);
            AllPairsResultStPerI64 {
                distances: ArraySeqStPerS { seq: dist_vec },
                predecessors: self.predecessors,
                n: self.n,
            }
        }

        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>)
            ensures
                u >= self.predecessors.spec_len() ==> pred is None,
                u < self.predecessors.spec_len() && v >= self.predecessors.spec_index(u as int).spec_len() ==> pred is None,
                u < self.predecessors.spec_len() && v < self.predecessors.spec_index(u as int).spec_len() && self.predecessors.spec_index(u as int).spec_index(v as int) == NO_PREDECESSOR ==> pred is None,
                u < self.predecessors.spec_len() && v < self.predecessors.spec_index(u as int).spec_len() && self.predecessors.spec_index(u as int).spec_index(v as int) != NO_PREDECESSOR ==> pred == Some(self.predecessors.spec_index(u as int).spec_index(v as int)),
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

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (updated: Self)
            ensures
                updated.n == self.n,
                updated.distances == self.distances,
                updated.predecessors.spec_len() == self.predecessors.spec_len(),
                forall|r: int| #![trigger updated.predecessors.spec_index(r)]
                    0 <= r < self.predecessors.spec_len()
                    ==> updated.predecessors.spec_index(r).spec_len() == self.predecessors.spec_index(r).spec_len(),
                u < self.predecessors.spec_len() && v < self.n && v < self.predecessors.spec_index(u as int).spec_len()
                    ==> updated.predecessors.spec_index(u as int).spec_index(v as int) == pred,
                forall|r: int, c: int| #![trigger updated.predecessors.spec_index(r).spec_index(c)]
                    0 <= r < self.predecessors.spec_len()
                    && 0 <= c < self.predecessors.spec_index(r).spec_len()
                    && (r != u as int || c != v as int)
                    ==> updated.predecessors.spec_index(r).spec_index(c) == self.predecessors.spec_index(r).spec_index(c),
        {
            if u >= self.predecessors.seq.len() || v >= self.n { return self; }
            let mut row_vec = self.predecessors.seq[u].seq.clone();
            if v < row_vec.len() {
                row_vec.set(v, pred);
            }
            let updated_row = ArraySeqStPerS { seq: row_vec };
            let mut pred_vec = self.predecessors.seq;
            pred_vec.set(u, updated_row);
            AllPairsResultStPerI64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS { seq: pred_vec },
                n: self.n,
            }
        }

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool)
            ensures
                u >= self.distances.spec_len() ==> !b,
                u < self.distances.spec_len() && v >= self.distances.spec_index(u as int).spec_len() ==> !b,
                u < self.distances.spec_len() && v < self.distances.spec_index(u as int).spec_len() ==>
                    b == (self.distances.spec_index(u as int).spec_index(v as int) != UNREACHABLE),
        {
            self.get_distance(u, v) != UNREACHABLE
        }

        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            ensures
                u >= self.predecessors.spec_len() ==> path is None,
                v >= self.predecessors.spec_len() ==> path is None,
                u != v && u < self.distances.spec_len()
                    && v < self.distances.spec_index(u as int).spec_len()
                    && self.distances.spec_index(u as int).spec_index(v as int) == UNREACHABLE
                    ==> path is None,
                path is Some ==> path->Some_0.spec_len() >= 1,
                path is Some ==> path->Some_0.spec_index(0) == u,
                path is Some ==> path->Some_0.spec_index(path->Some_0.spec_len() - 1) == v,
                path is Some && u != v ==>
                    forall|j: int| #![trigger path->Some_0.spec_index(j)]
                        0 <= j < path->Some_0.spec_len()
                        ==> path->Some_0.spec_index(j) < self.predecessors.spec_index(u as int).spec_len(),
        {
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
                    path@[0] == v,
                    path@[path@.len() - 1] == current,
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < row_len,
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
}
