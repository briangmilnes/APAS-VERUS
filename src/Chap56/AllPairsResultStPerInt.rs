//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Persistent (Integer Weights)

pub mod AllPairsResultStPerInt {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct AllPairsResultStPerInt {
        pub distances: ArraySeqStPerS<ArraySeqStPerS<i64>>,
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        pub n: usize,
    }

    impl AllPairsResultStPerInt {
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

        pub fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    impl AllPairsResultStPerInt {
        pub fn new(n: usize) -> Self {
            let distances = ArraySeqStPerS::tabulate(
                &|i| ArraySeqStPerS::tabulate(&|j| if i == j { 0 } else { UNREACHABLE }, n),
                n,
            );
            let predecessors = ArraySeqStPerS::tabulate(&|_| ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n), n);
            AllPairsResultStPerInt { distances, predecessors, n }
        }

        pub fn set_distance(self, u: usize, v: usize, dist: i64) -> Self {
            if u >= self.n || v >= self.n { return self; }
            let updated_row = ArraySeqStPerS::update(self.distances.nth(u), v, dist);
            AllPairsResultStPerInt {
                distances: ArraySeqStPerS::update(&self.distances, u, updated_row),
                predecessors: self.predecessors,
                n: self.n,
            }
        }

        pub fn set_predecessor(self, u: usize, v: usize, pred: usize) -> Self {
            if u >= self.n || v >= self.n { return self; }
            let updated_row = ArraySeqStPerS::update(self.predecessors.nth(u), v, pred);
            AllPairsResultStPerInt {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, u, updated_row),
                n: self.n,
            }
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
