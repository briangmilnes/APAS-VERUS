//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Persistent (Integer Weights)

pub mod SSSPResultStPerI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStPerI64 {
        pub distances: ArraySeqStPerS<i64>,
        pub predecessors: ArraySeqStPerS<usize>,
        pub source: usize,
    }

    impl SSSPResultStPerI64 {
        pub fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        pub fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        pub fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    impl SSSPResultStPerI64 {
        pub fn new(n: usize, source: usize) -> Self {
            let distances = ArraySeqStPerS::tabulate(&|i| if i == source { 0 } else { UNREACHABLE }, n);
            let predecessors = ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n);
            SSSPResultStPerI64 { distances, predecessors, source }
        }

        pub fn set_distance(self, v: usize, dist: i64) -> Self {
            if v >= self.distances.length() { return self; }
            SSSPResultStPerI64 {
                distances: ArraySeqStPerS::update(&self.distances, v, dist),
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        pub fn set_predecessor(self, v: usize, pred: usize) -> Self {
            if v >= self.predecessors.length() { return self; }
            SSSPResultStPerI64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, v, pred),
                source: self.source,
            }
        }

        pub fn extract_path(&self, v: usize) -> Option<ArraySeqStPerS<usize>> {
            if !self.is_reachable(v) { return None; }
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
