//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod AllPairsResultStEphInt {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
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

    // 8. traits

    pub trait AllPairsResultStEphIntTrait: Sized {
        fn new(n: usize) -> (result: Self);

        fn get_distance(&self, u: usize, v: usize) -> (dist: i64);

        fn set_distance(&mut self, u: usize, v: usize, dist: i64);

        fn get_predecessor(&self, u: usize, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(&mut self, u: usize, v: usize, pred: usize);

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool);

        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStEphIntTrait for AllPairsResultStEphInt {
        #[verifier::external_body]
        fn new(n: usize) -> (result: Self)
            ensures result.n == n,
        {
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

        fn set_distance(&mut self, u: usize, v: usize, dist: i64)
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

        fn is_reachable(&self, u: usize, v: usize) -> (b: bool) {
            self.get_distance(u, v) != UNREACHABLE
        }

        #[verifier::external_body]
        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
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

    } // verus!
}
