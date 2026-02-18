//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod AllPairsResultStEphInt {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStEphInt {
        pub distances: ArraySeqStEphS<ArraySeqStEphS<i64>>,
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub n: usize,
    }

    impl AllPairsResultStEphInt {
        pub fn get_distance(&self, u: usize, v: usize) -> (dist: i64) {
            if u >= self.distances.length() {
                return UNREACHABLE;
            }
            let row = self.distances.nth(u);
            if v >= row.length() {
                return UNREACHABLE;
            }
            *row.nth(v)
        }

        pub fn set_distance(&mut self, u: usize, v: usize, dist: i64) {
            if u < self.distances.length() {
                let row_ref = self.distances.nth(u);
                if v < row_ref.length() {
                    let mut row = row_ref.clone();
                    let _ = row.set(v, dist);
                    let _ = self.distances.set(u, row);
                }
            }
        }

        pub fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>) {
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

        pub fn set_predecessor(&mut self, u: usize, v: usize, pred: usize) {
            if u < self.predecessors.length() {
                let row_ref = self.predecessors.nth(u);
                if v < row_ref.length() {
                    let mut row = row_ref.clone();
                    let _ = row.set(v, pred);
                    let _ = self.predecessors.set(u, row);
                }
            }
        }

        pub fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    impl AllPairsResultStEphInt {
        pub fn new(n: usize) -> Self {
            let mut dist_matrix = Vec::with_capacity(n);
            for i in 0..n {
                let mut row = vec![UNREACHABLE; n];
                row[i] = 0;
                dist_matrix.push(ArraySeqStEphS::from_vec(row));
            }
            let distances = ArraySeqStEphS::from_vec(dist_matrix);
            let pred_matrix = vec![ArraySeqStEphS::new(n, NO_PREDECESSOR); n];
            let predecessors = ArraySeqStEphS::from_vec(pred_matrix);
            AllPairsResultStEphInt { distances, predecessors, n }
        }

        pub fn extract_path(&self, u: usize, v: usize) -> Option<ArraySeqStPerS<usize>> {
            if u == v { return Some(ArraySeqStPerS::from_vec(vec![u])); }
            if !self.is_reachable(u, v) { return None; }
            let mut path = Vec::new();
            let mut current = v;
            path.push(current);
            while current != u {
                let pred = *self.predecessors.nth(u).nth(current);
                if pred == NO_PREDECESSOR { return None; }
                path.push(pred);
                current = pred;
            }
            path.reverse();
            Some(ArraySeqStPerS::from_vec(path))
        }
    }
}
