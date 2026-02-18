//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod SSSPResultStEphI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStEphI64 {
        pub distances: ArraySeqStEphS<i64>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    impl SSSPResultStEphI64 {
        pub fn new(n: usize, source: usize) -> (result: Self)
            requires source < n,
        {
            let mut dist_seq = ArraySeqStEphS::<i64>::new(n, UNREACHABLE);
            let _ = dist_seq.set(source, 0i64);
            let pred_seq = ArraySeqStEphS::<usize>::new(n, NO_PREDECESSOR);
            SSSPResultStEphI64 {
                distances: dist_seq,
                predecessors: pred_seq,
                source,
            }
        }

        pub fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        pub fn set_distance(&mut self, v: usize, dist: i64) {
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
            self.get_distance(v) != UNREACHABLE
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    impl SSSPResultStEphI64 {
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
