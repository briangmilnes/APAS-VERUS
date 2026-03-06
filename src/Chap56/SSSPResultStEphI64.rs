//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod SSSPResultStEphI64 {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
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

    // 8. traits

    pub trait SSSPResultStEphI64Trait: Sized {
        spec fn spec_distances(&self) -> Seq<i64>;

        spec fn spec_predecessors(&self) -> Seq<usize>;

        spec fn spec_source(&self) -> usize;

        fn new(n: usize, source: usize) -> (empty: Self)
            requires source < n,
            ensures
                empty.spec_distances().len() == n,
                empty.spec_predecessors().len() == n,
                empty.spec_source() == source;

        fn get_distance(&self, v: usize) -> (dist: i64)
            ensures
                v >= self.spec_distances().len() ==> dist == UNREACHABLE,
                v < self.spec_distances().len() ==> dist == self.spec_distances()[v as int];

        fn set_distance(&mut self, v: usize, dist: i64)
            ensures
                self.spec_distances().len() == old(self).spec_distances().len(),
                v < old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances().update(v as int, dist),
                v >= old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances(),
                self.spec_predecessors() == old(self).spec_predecessors(),
                self.spec_source() == old(self).spec_source();

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>)
            ensures
                v >= self.spec_predecessors().len() ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] == NO_PREDECESSOR ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] != NO_PREDECESSOR ==> pred == Some(self.spec_predecessors()[v as int]);

        fn set_predecessor(&mut self, v: usize, pred: usize)
            ensures
                self.spec_predecessors().len() == old(self).spec_predecessors().len(),
                v < old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors().update(v as int, pred),
                v >= old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors(),
                self.spec_distances() == old(self).spec_distances(),
                self.spec_source() == old(self).spec_source();

        fn is_reachable(&self, v: usize) -> (b: bool)
            ensures
                v >= self.spec_distances().len() ==> !b,
                v < self.spec_distances().len() ==> b == (self.spec_distances()[v as int] != UNREACHABLE);

        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            ensures
                path is Some ==> path->Some_0.spec_len() >= 1,
                path is Some ==> path->Some_0.spec_index(0) == self.spec_source(),
                path is Some ==> path->Some_0.spec_index(path->Some_0.spec_len() - 1) == v;
    }

    // 9. impls

    impl SSSPResultStEphI64Trait for SSSPResultStEphI64 {
        open spec fn spec_distances(&self) -> Seq<i64> { self.distances.seq@ }

        open spec fn spec_predecessors(&self) -> Seq<usize> { self.predecessors.seq@ }

        open spec fn spec_source(&self) -> usize { self.source }

        fn new(n: usize, source: usize) -> (empty: Self)
        {
            let mut dist_seq = ArraySeqStEphS::<i64>::new(n, UNREACHABLE);
            let ok = dist_seq.set(source, 0i64);
            assert(ok.is_ok());
            let pred_seq = ArraySeqStEphS::<usize>::new(n, NO_PREDECESSOR);
            SSSPResultStEphI64 {
                distances: dist_seq,
                predecessors: pred_seq,
                source,
            }
        }

        fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        fn set_distance(&mut self, v: usize, dist: i64)
        {
            if v < self.distances.length() {
                self.distances.seq.set(v, dist);
            }
        }

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(&mut self, v: usize, pred: usize)
        {
            if v < self.predecessors.length() {
                self.predecessors.seq.set(v, pred);
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }

        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) {
                return None;
            }
            let n = self.predecessors.length();
            let mut path = Vec::new();
            let mut current = v;
            path.push(current);
            let mut steps: usize = 0;
            while current != self.source && steps < n
                invariant
                    steps <= n,
                    n == self.predecessors.spec_len(),
                    path@.len() > 0,
                    path@[0] == v,
                    path@[path@.len() - 1] == current,
                decreases n - steps
            {
                if current >= n {
                    return None;
                }
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR { return None; }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != self.source {
                return None;
            }
            let path_len = path.len();
            let mut reversed = Vec::new();
            let mut k: usize = path_len;
            while k > 0
                invariant
                    k <= path_len,
                    path_len == path@.len(),
                    reversed@.len() == (path_len - k) as int,
                    path@[0] == v,
                    path@[path@.len() - 1] == self.source,
                    forall|j: int| #![trigger reversed@[j]]
                        0 <= j < reversed@.len() ==> reversed@[j] == path@[path_len - 1 - j],
                decreases k
            {
                k = k - 1;
                reversed.push(path[k]);
            }
            Some(ArraySeqStPerS::from_vec(reversed))
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for SSSPResultStEphI64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SSSPResultStEphI64(source={}, n={})", self.source, self.distances.length())
        }
    }

    impl std::fmt::Display for SSSPResultStEphI64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SSSPResult(source={}, n={})", self.source, self.distances.length())
        }
    }
}
