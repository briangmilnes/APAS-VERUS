//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Persistent (Float Weights)
//!
//! Uses `WrappedF64` from vstdplus::float for distances with persistent array sequences.

pub mod SSSPResultStPerF64 {

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::float::float::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub const NO_PREDECESSOR: usize = usize::MAX;

    pub struct SSSPResultStPerF64 {
        pub distances: ArraySeqStPerS<WrappedF64>,
        pub predecessors: ArraySeqStPerS<usize>,
        pub source: usize,
    }

    // 5. view impls

    impl View for SSSPResultStPerF64 {
        type V = Seq<int>;
        open spec fn view(&self) -> Self::V {
            self.predecessors@.map(|_i: int, v: usize| v as int)
        }
    }

    // 8. traits

    pub trait SSSPResultStPerF64Trait: Sized {
        fn new(n: usize, source: usize) -> (result: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: WrappedF64);

        fn set_distance(self, v: usize, dist: WrappedF64) -> (result: Self);

        fn get_predecessor(&self, v: usize) -> (result: Option<usize>);

        fn set_predecessor(self, v: usize, pred: usize) -> (result: Self);

        fn is_reachable(&self, v: usize) -> (result: bool);

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStPerF64Trait for SSSPResultStPerF64 {
        fn new(n: usize, source: usize) -> (result: Self)
            ensures
                result.distances@.len() == n,
                result.predecessors@.len() == n,
                result.source == source,
        {
            let unreach = unreachable_dist();
            let zero = zero_dist();
            let mut dist_vec: Vec<WrappedF64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_vec@.len() == i as int,
                    n <= usize::MAX,
                decreases n - i,
            {
                if i == source {
                    dist_vec.push(zero);
                } else {
                    dist_vec.push(unreach);
                }
                i = i + 1;
            }
            let distances = ArraySeqStPerS::from_vec(dist_vec);
            let predecessors = ArraySeqStPerS::<usize>::new(n, NO_PREDECESSOR);
            SSSPResultStPerF64 { distances, predecessors, source }
        }

        fn get_distance(&self, v: usize) -> (dist: WrappedF64) {
            if v >= self.distances.length() {
                return unreachable_dist();
            }
            *self.distances.nth(v)
        }

        fn set_distance(self, v: usize, dist: WrappedF64) -> (result: Self)
            ensures
                result.predecessors@ == self.predecessors@,
                result.source == self.source,
        {
            if v >= self.distances.seq.len() { return self; }
            let mut dist_vec = self.distances.seq;
            dist_vec.set(v, dist);
            SSSPResultStPerF64 {
                distances: ArraySeqStPerS { seq: dist_vec },
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        fn get_predecessor(&self, v: usize) -> (result: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(self, v: usize, pred: usize) -> (result: Self)
            ensures
                result.distances@ == self.distances@,
                result.source == self.source,
        {
            if v >= self.predecessors.seq.len() { return self; }
            let mut pred_vec = self.predecessors.seq;
            pred_vec.set(v, pred);
            SSSPResultStPerF64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS { seq: pred_vec },
                source: self.source,
            }
        }

        fn is_reachable(&self, v: usize) -> (result: bool) {
            self.get_distance(v).is_finite()
        }

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) { return None; }
            let n = self.predecessors.length();
            if v >= n { return None; }
            let mut path: Vec<usize> = Vec::new();
            let mut current: usize = v;
            path.push(current);
            let mut steps: usize = 0;
            while current != self.source && steps < n
                invariant
                    steps <= n,
                    current < n,
                    n as int == self.predecessors.spec_len(),
                    path@.len() > 0,
                decreases n - steps,
            {
                if current >= n { return None; }
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR || pred >= n { return None; }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != self.source { return None; }
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
