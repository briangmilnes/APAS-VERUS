//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod AllPairsResultStEphI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStEphI64 {
        pub distances: ArraySeqStEphS<ArraySeqStEphS<i64>>,
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStEphI64Trait: Sized {
        spec fn spec_n(&self) -> usize;

        spec fn spec_distances_len(&self) -> nat;

        spec fn spec_distances_row_len(&self, u: int) -> nat;

        spec fn spec_distance_at(&self, u: int, v: int) -> i64;

        spec fn spec_predecessors_len(&self) -> nat;

        spec fn spec_predecessors_row_len(&self, u: int) -> nat;

        spec fn spec_predecessor_at(&self, u: int, v: int) -> usize;

        fn new(n: usize) -> (empty: Self)
            ensures empty.spec_n() == n;

        fn get_distance(&self, u: usize, v: usize) -> (dist: i64)
            ensures
                (u as int) >= self.spec_distances_len() ==> dist == UNREACHABLE,
                (u as int) < self.spec_distances_len() && (v as int) >= self.spec_distances_row_len(u as int) ==> dist == UNREACHABLE,
                (u as int) < self.spec_distances_len() && (v as int) < self.spec_distances_row_len(u as int) ==> dist == self.spec_distance_at(u as int, v as int);

        fn set_distance(&mut self, u: usize, v: usize, dist: i64)
            ensures
                self.spec_n() == old(self).spec_n();

        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>)
            ensures
                (u as int) >= self.spec_predecessors_len() ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) >= self.spec_predecessors_row_len(u as int) ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) < self.spec_predecessors_row_len(u as int) && self.spec_predecessor_at(u as int, v as int) == NO_PREDECESSOR ==> pred is None,
                (u as int) < self.spec_predecessors_len() && (v as int) < self.spec_predecessors_row_len(u as int) && self.spec_predecessor_at(u as int, v as int) != NO_PREDECESSOR ==> pred == Some(self.spec_predecessor_at(u as int, v as int));

        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
            ensures
                self.spec_n() == old(self).spec_n();

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool);

        fn extract_path(&self, u: usize, v: usize) -> (path: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStEphI64Trait for AllPairsResultStEphI64 {
        open spec fn spec_n(&self) -> usize { self.n }

        open spec fn spec_distances_len(&self) -> nat { self.distances.spec_len() }

        open spec fn spec_distances_row_len(&self, u: int) -> nat { self.distances.spec_index(u).spec_len() }

        open spec fn spec_distance_at(&self, u: int, v: int) -> i64 { self.distances.spec_index(u).spec_index(v) }

        open spec fn spec_predecessors_len(&self) -> nat { self.predecessors.spec_len() }

        open spec fn spec_predecessors_row_len(&self, u: int) -> nat { self.predecessors.spec_index(u).spec_len() }

        open spec fn spec_predecessor_at(&self, u: int, v: int) -> usize { self.predecessors.spec_index(u).spec_index(v) }

        fn new(n: usize) -> (empty: Self)
        {
            let mut dist_rows: Vec<ArraySeqStEphS<i64>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_rows@.len() == i as int,
                decreases n - i,
            {
                let mut row: Vec<i64> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant
                        j <= n,
                        row@.len() == j as int,
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
            AllPairsResultStEphI64 {
                distances: ArraySeqStEphS { seq: dist_rows },
                predecessors: ArraySeqStEphS { seq: pred_rows },
                n,
            }
        }

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

        fn set_distance(&mut self, u: usize, v: usize, dist: i64)
        {
            if u < self.distances.length() {
                let row_ref = self.distances.nth(u);
                if v < row_ref.length() {
                    let mut row = row_ref.clone();
                    let _ = row.set(v, dist);
                    let _ = self.distances.set(u, row);
                }
            }
        }

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

        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize)
        {
            if u < self.predecessors.length() {
                let row_ref = self.predecessors.nth(u);
                if v < row_ref.length() {
                    let mut row = row_ref.clone();
                    let _ = row.set(v, pred);
                    let _ = self.predecessors.set(u, row);
                }
            }
        }

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
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
