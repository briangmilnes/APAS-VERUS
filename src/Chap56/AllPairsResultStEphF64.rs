//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Float Weights)
//!
//! Uses `F64Dist` from vstdplus::float for distances with ephemeral array sequences.

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
        pub distances: ArraySeqStEphS<ArraySeqStEphS<F64Dist>>,
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub n: usize,
    }

    // 8. traits

    pub trait AllPairsResultStEphF64Trait: Sized {
        fn new(n: usize) -> (result: Self);

        fn get_distance(&self, u: usize, v: usize) -> (dist: F64Dist);

        fn set_distance(&mut self, u: usize, v: usize, dist: F64Dist);

        fn get_predecessor(&self, u: usize, v: usize) -> (result: Option<usize>);

        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize);

        fn is_reachable(&self, u: usize, v: usize) -> (result: bool);

        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStEphF64Trait for AllPairsResultStEphF64 {
        fn new(n: usize) -> (result: Self)
            ensures result.n == n,
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_rows: Vec<ArraySeqStEphS<F64Dist>> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_rows@.len() == i as int,
                decreases n - i,
            {
                let mut row: Vec<F64Dist> = Vec::new();
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

        fn get_distance(&self, u: usize, v: usize) -> (dist: F64Dist) {
            if u >= self.distances.length() {
                return unreachable_dist();
            }
            let row = self.distances.nth(u);
            if v >= row.length() {
                return unreachable_dist();
            }
            *row.nth(v)
        }

        fn set_distance(&mut self, u: usize, v: usize, dist: F64Dist)
            ensures
                self.n == old(self).n,
                self.predecessors == old(self).predecessors,
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

        fn get_predecessor(&self, u: usize, v: usize) -> (result: Option<usize>) {
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
            ensures
                self.n == old(self).n,
                self.distances == old(self).distances,
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

        fn is_reachable(&self, u: usize, v: usize) -> (result: bool) {
            self.get_distance(u, v).is_finite()
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
}
