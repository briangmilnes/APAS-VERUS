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

    pub struct AllPairsResultStPerI64 {
        pub distances: ArraySeqStPerS<ArraySeqStPerS<i64>>,
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStPerI64Trait: Sized {
        fn new(n: usize) -> (result: Self);

        fn get_distance(&self, u: usize, v: usize) -> (dist: i64);

        fn set_distance(self, u: usize, v: usize, dist: i64) -> (result: Self);

        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (result: Self);

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool);

        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStPerI64Trait for AllPairsResultStPerI64 {
        fn new(n: usize) -> (result: Self)
            ensures result.n == n,
        {
            let mut dist_rows: Vec<ArraySeqStPerS<i64>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant i <= n, dist_rows@.len() == i as int,
                decreases n - i,
            {
                let mut row: Vec<i64> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant j <= n, row@.len() == j as int,
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
                invariant k <= n, pred_rows@.len() == k as int,
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

        fn set_distance(self, u: usize, v: usize, dist: i64) -> (result: Self)
            ensures
                result.n == self.n,
                result.predecessors == self.predecessors,
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

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (result: Self)
            ensures
                result.n == self.n,
                result.distances == self.distances,
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

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
        }

        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
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
}
