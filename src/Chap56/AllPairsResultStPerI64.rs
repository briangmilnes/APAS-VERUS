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
        #[verifier::external_body]
        fn new(n: usize) -> (result: Self)
            ensures result.n == n,
        {
            let distances = ArraySeqStPerS::tabulate(
                &|i| ArraySeqStPerS::tabulate(&|j| if i == j { 0 } else { UNREACHABLE }, n),
                n,
            );
            let predecessors = ArraySeqStPerS::tabulate(&|_| ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n), n);
            AllPairsResultStPerI64 { distances, predecessors, n }
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

        #[verifier::external_body]
        fn set_distance(self, u: usize, v: usize, dist: i64) -> (result: Self)
            ensures
                result.n == self.n,
                result.predecessors == self.predecessors,
        {
            if u >= self.n || v >= self.n { return self; }
            let updated_row = ArraySeqStPerS::update(self.distances.nth(u), v, dist);
            AllPairsResultStPerI64 {
                distances: ArraySeqStPerS::update(&self.distances, u, updated_row),
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

        #[verifier::external_body]
        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (result: Self)
            ensures
                result.n == self.n,
                result.distances == self.distances,
        {
            if u >= self.n || v >= self.n { return self; }
            let updated_row = ArraySeqStPerS::update(self.predecessors.nth(u), v, pred);
            AllPairsResultStPerI64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, u, updated_row),
                n: self.n,
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
