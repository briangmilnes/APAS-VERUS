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
                result.distances@.len() == n as int,
                result.predecessors@.len() == n as int,
                result.source == source,
        {
            let mut dist_vec: Vec<i64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_vec@.len() == i as int,
                decreases n - i,
            {
                if i == source {
                    dist_vec.push(0i64);
                } else {
                    dist_vec.push(UNREACHABLE);
                }
                i = i + 1;
            }
            let mut pred_vec: Vec<usize> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    pred_vec@.len() == j as int,
                decreases n - j,
            {
                pred_vec.push(NO_PREDECESSOR);
                j = j + 1;
            }
            SSSPResultStEphI64 {
                distances: ArraySeqStEphS { seq: dist_vec },
                predecessors: ArraySeqStEphS { seq: pred_vec },
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
                v < old(self).distances@.len() ==> self.distances@ == old(self).distances@.update(v as int, dist),
                v >= old(self).distances@.len() ==> self.distances@ == old(self).distances@,
                self.predecessors@ == old(self).predecessors@,
                self.source == old(self).source,
        {
            if v < self.distances.seq.len() {
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
            ensures
                v < old(self).predecessors@.len() ==> self.predecessors@ == old(self).predecessors@.update(v as int, pred),
                v >= old(self).predecessors@.len() ==> self.predecessors@ == old(self).predecessors@,
                self.distances@ == old(self).distances@,
                self.source == old(self).source,
        {
            if v < self.predecessors.seq.len() {
                self.predecessors.seq.set(v, pred);
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
            if v >= n {
                return None;
            }
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
                if current >= n {
                    return None;
                }
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR || pred >= n {
                    return None;
                }
                path.push(pred);
                current = pred;
                steps = steps + 1;
            }
            if current != self.source {
                return None;
            }
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
