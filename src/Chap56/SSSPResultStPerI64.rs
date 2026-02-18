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

    // 8. traits

    pub trait SSSPResultStPerI64Trait: Sized {
        fn new(n: usize, source: usize) -> (result: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: i64);

        fn set_distance(self, v: usize, dist: i64) -> (result: Self);

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(self, v: usize, pred: usize) -> (result: Self);

        fn is_reachable(&self, v: usize) -> (b: bool);

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStPerI64Trait for SSSPResultStPerI64 {
        fn new(n: usize, source: usize) -> (result: Self)
            ensures
                result.distances@.len() == n as int,
                result.predecessors@.len() == n as int,
                result.source == source,
        {
            let distances = ArraySeqStPerS::tabulate(
                &(|i: usize| -> (r: i64)
                    requires i < n
                    ensures r == (if i == source { 0i64 } else { UNREACHABLE })
                {
                    if i == source { 0i64 } else { UNREACHABLE }
                }),
                n,
            );
            let predecessors = ArraySeqStPerS::tabulate(
                &(|_i: usize| -> (r: usize)
                    ensures r == NO_PREDECESSOR
                {
                    NO_PREDECESSOR
                }),
                n,
            );
            SSSPResultStPerI64 { distances, predecessors, source }
        }

        fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        #[verifier::external_body]
        fn set_distance(self, v: usize, dist: i64) -> (result: Self)
            ensures
                v < self.distances@.len() ==> result.distances@ == self.distances@.update(v as int, dist),
                v >= self.distances@.len() ==> result.distances@ == self.distances@,
                result.predecessors@ == self.predecessors@,
                result.source == self.source,
        {
            if v >= self.distances.length() { return self; }
            SSSPResultStPerI64 {
                distances: ArraySeqStPerS::update(&self.distances, v, dist),
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        #[verifier::external_body]
        fn set_predecessor(self, v: usize, pred: usize) -> (result: Self)
            ensures
                v < self.predecessors@.len() ==> result.predecessors@ == self.predecessors@.update(v as int, pred),
                v >= self.predecessors@.len() ==> result.predecessors@ == self.predecessors@,
                result.distances@ == self.distances@,
                result.source == self.source,
        {
            if v >= self.predecessors.length() { return self; }
            SSSPResultStPerI64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, v, pred),
                source: self.source,
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }

        #[verifier::external_body]
        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
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

    } // verus!
}
