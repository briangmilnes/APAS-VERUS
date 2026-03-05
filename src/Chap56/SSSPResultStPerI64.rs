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

    // Table of Contents
    // 6. spec fns
    // 8. traits
    // 9. impls

    pub struct SSSPResultStPerI64 {
        pub distances: ArraySeqStPerS<i64>,
        pub predecessors: ArraySeqStPerS<usize>,
        pub source: usize,
    }

    // 6. spec fns

    pub open spec fn spec_ssspresultstperi64_wf(s: &SSSPResultStPerI64) -> bool {
        s.distances@.len() == s.predecessors@.len()
        && s.source < s.distances@.len()
    }

    // 8. traits

    pub trait SSSPResultStPerI64Trait: Sized {
        fn new(n: usize, source: usize) -> (empty: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: i64);

        fn set_distance(self, v: usize, dist: i64) -> (updated: Self);

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>);

        fn set_predecessor(self, v: usize, pred: usize) -> (updated: Self);

        fn is_reachable(&self, v: usize) -> (b: bool);

        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStPerI64Trait for SSSPResultStPerI64 {
        fn new(n: usize, source: usize) -> (empty: Self)
            ensures
                empty.distances@.len() == n as int,
                empty.predecessors@.len() == n as int,
                empty.source == source,
                forall|i: int| #![trigger empty.distances@[i]] 0 <= i < n ==>
                    empty.distances@[i] == (if i == source as int { 0i64 } else { UNREACHABLE }),
                forall|i: int| #![trigger empty.predecessors@[i]] 0 <= i < n ==>
                    empty.predecessors@[i] == NO_PREDECESSOR,
                spec_ssspresultstperi64_wf(&empty),
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

        fn get_distance(&self, v: usize) -> (dist: i64)
            ensures
                v >= self.distances@.len() ==> dist == UNREACHABLE,
                v < self.distances@.len() ==> dist == self.distances@[v as int],
        {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        fn set_distance(self, v: usize, dist: i64) -> (updated: Self)
            ensures
                v < self.distances@.len() ==> updated.distances@ == self.distances@.update(v as int, dist),
                v >= self.distances@.len() ==> updated.distances@ == self.distances@,
                updated.predecessors@ == self.predecessors@,
                updated.source == self.source,
                spec_ssspresultstperi64_wf(&self) ==> spec_ssspresultstperi64_wf(&updated),
        {
            if v >= self.distances.seq.len() { return self; }
            let mut dist_vec = self.distances.seq;
            dist_vec.set(v, dist);
            SSSPResultStPerI64 {
                distances: ArraySeqStPerS { seq: dist_vec },
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>)
            ensures
                v >= self.predecessors@.len() ==> pred is None,
                v < self.predecessors@.len() && self.predecessors@[v as int] == NO_PREDECESSOR ==> pred is None,
                v < self.predecessors@.len() && self.predecessors@[v as int] != NO_PREDECESSOR ==> pred == Some(self.predecessors@[v as int]),
        {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        fn set_predecessor(self, v: usize, pred: usize) -> (updated: Self)
            ensures
                v < self.predecessors@.len() ==> updated.predecessors@ == self.predecessors@.update(v as int, pred),
                v >= self.predecessors@.len() ==> updated.predecessors@ == self.predecessors@,
                updated.distances@ == self.distances@,
                updated.source == self.source,
                spec_ssspresultstperi64_wf(&self) ==> spec_ssspresultstperi64_wf(&updated),
        {
            if v >= self.predecessors.seq.len() { return self; }
            let mut pred_vec = self.predecessors.seq;
            pred_vec.set(v, pred);
            SSSPResultStPerI64 {
                distances: self.distances,
                predecessors: ArraySeqStPerS { seq: pred_vec },
                source: self.source,
            }
        }

        fn is_reachable(&self, v: usize) -> (b: bool)
            ensures
                v >= self.distances@.len() ==> !b,
                v < self.distances@.len() ==> b == (self.distances@[v as int] != UNREACHABLE),
        {
            self.get_distance(v) != UNREACHABLE
        }

        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            ensures
                v >= self.distances@.len() ==> path is None,
                v < self.distances@.len() && self.distances@[v as int] == UNREACHABLE ==> path is None,
                v >= self.predecessors@.len() ==> path is None,
                path is Some ==> path->Some_0.spec_len() >= 1,
                path is Some ==> path->Some_0.spec_index(0) == self.source,
                path is Some ==> path->Some_0.spec_index(path->Some_0.spec_len() - 1) == v,
                path is Some ==> forall|j: int| #![trigger path->Some_0.spec_index(j)]
                    0 <= j < path->Some_0.spec_len()
                    ==> path->Some_0.spec_index(j) < self.predecessors.spec_len(),
        {
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
                    path@[0] == v,
                    path@[path@.len() - 1] == current,
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < n,
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
                    path@[0] == v,
                    path@[path@.len() - 1] == self.source,
                    forall|j: int| #![trigger reversed@[j]]
                        0 <= j < reversed@.len() ==> reversed@[j] == path@[path_len - 1 - j],
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < n,
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
