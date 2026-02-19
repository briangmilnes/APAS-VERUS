//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)

pub mod SSSPResultStEphI64 {

    use vstd::prelude::*;
    use vstd::assert_seqs_equal;
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
        fn new(n: usize, source: usize) -> (result: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: i64);

        fn set_distance(&mut self, v: usize, dist: i64);

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(&mut self, v: usize, pred: usize);

        fn is_reachable(&self, v: usize) -> (b: bool);

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStEphI64Trait for SSSPResultStEphI64 {
        fn new(n: usize, source: usize) -> (result: Self)
            ensures
                result.distances.spec_len() == n,
                result.predecessors.spec_len() == n,
                result.source == source,
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
            ensures
                self.distances.spec_len() == old(self).distances.spec_len(),
                v < old(self).distances.spec_len() ==> self.distances.spec_index(v as int) == dist,
                v < old(self).distances.spec_len() ==> forall|i: int|
                    #![trigger self.distances.spec_index(i)]
                    0 <= i < old(self).distances.spec_len() && i != v as int
                    ==> self.distances.spec_index(i) == old(self).distances.spec_index(i),
                self.predecessors == old(self).predecessors,
                self.source == old(self).source,
        {
            if v < self.distances.length() {
                let ok = self.distances.set(v, dist);
                assert(ok.is_ok());
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
            ensures
                self.predecessors.spec_len() == old(self).predecessors.spec_len(),
                v < old(self).predecessors.spec_len() ==> self.predecessors.spec_index(v as int) == pred,
                v < old(self).predecessors.spec_len() ==> forall|i: int|
                    #![trigger self.predecessors.spec_index(i)]
                    0 <= i < old(self).predecessors.spec_len() && i != v as int
                    ==> self.predecessors.spec_index(i) == old(self).predecessors.spec_index(i),
                self.distances == old(self).distances,
                self.source == old(self).source,
        {
            if v < self.predecessors.length() {
                let ok = self.predecessors.set(v, pred);
                assert(ok.is_ok());
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
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
                decreases k
            {
                k = k - 1;
                reversed.push(path[k]);
            }
            Some(ArraySeqStPerS::from_vec(reversed))
        }
    }

    } // verus!
}
