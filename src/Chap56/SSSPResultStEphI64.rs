//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod SSSPResultStEphI64 {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 4. type definitions


    pub const UNREACHABLE: i64 = i64::MAX;
    pub const NO_PREDECESSOR: usize = usize::MAX;


    pub struct SSSPResultStEphI64 {
        pub distances: ArraySeqStEphS<i64>,
        pub predecessors: ArraySeqStEphS<usize>,
        pub source: usize,
    }

    //		Section 8. traits


    pub trait SSSPResultStEphI64Trait: Sized {
        spec fn spec_ssspresultstephi64_wf(s: &SSSPResultStEphI64) -> bool;

        spec fn spec_distances(&self) -> Seq<i64>;

        spec fn spec_predecessors(&self) -> Seq<usize>;

        spec fn spec_source(&self) -> usize;

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — initializes distance and predecessor arrays.
        fn new(n: usize, source: usize) -> (empty: Self)
            requires source < n,
            ensures
                empty.spec_distances().len() == n,
                empty.spec_predecessors().len() == n,
                empty.spec_source() == source,
                forall|i: int| #![trigger empty.spec_distances()[i]] 0 <= i < n ==>
                    empty.spec_distances()[i] == (if i == source as int { 0i64 } else { UNREACHABLE }),
                forall|i: int| #![trigger empty.spec_predecessors()[i]] 0 <= i < n ==>
                    empty.spec_predecessors()[i] == NO_PREDECESSOR;

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index lookup.
        fn get_distance(&self, v: usize) -> (dist: i64)
            ensures
                v >= self.spec_distances().len() ==> dist == UNREACHABLE,
                v < self.spec_distances().len() ==> dist == self.spec_distances()[v as int];

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index update.
        fn set_distance(&mut self, v: usize, dist: i64)
            ensures
                self.spec_distances().len() == old(self).spec_distances().len(),
                v < old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances().update(v as int, dist),
                v >= old(self).spec_distances().len() ==> self.spec_distances() =~= old(self).spec_distances(),
                self.spec_predecessors() == old(self).spec_predecessors(),
                self.spec_source() == old(self).spec_source();

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index lookup.
        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>)
            ensures
                v >= self.spec_predecessors().len() ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] == NO_PREDECESSOR ==> pred is None,
                v < self.spec_predecessors().len() && self.spec_predecessors()[v as int] != NO_PREDECESSOR ==> pred == Some(self.spec_predecessors()[v as int]);

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index update.
        fn set_predecessor(&mut self, v: usize, pred: usize)
            ensures
                self.spec_predecessors().len() == old(self).spec_predecessors().len(),
                v < old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors().update(v as int, pred),
                v >= old(self).spec_predecessors().len() ==> self.spec_predecessors() =~= old(self).spec_predecessors(),
                self.spec_distances() == old(self).spec_distances(),
                self.spec_source() == old(self).spec_source();

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, v: usize) -> (b: bool)
            ensures
                v >= self.spec_distances().len() ==> !b,
                v < self.spec_distances().len() ==> b == (self.spec_distances()[v as int] != UNREACHABLE);

        /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — predecessor chain traversal + reversal.
        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>)
            ensures
                v >= self.spec_distances().len() ==> path is None,
                v < self.spec_distances().len() && self.spec_distances()[v as int] == UNREACHABLE ==> path is None,
                v >= self.spec_predecessors().len() ==> path is None,
                path is Some ==> path->Some_0.spec_len() >= 1,
                path is Some ==> path->Some_0.spec_index(0) == self.spec_source(),
                path is Some ==> path->Some_0.spec_index(path->Some_0.spec_len() - 1) == v,
                path is Some ==>
                    forall|j: int| #![trigger path->Some_0.spec_index(j)]
                        0 <= j < path->Some_0.spec_len()
                        ==> (path->Some_0.spec_index(j) as int) < self.spec_predecessors().len();
    }

    //		Section 9. impls


    impl SSSPResultStEphI64Trait for SSSPResultStEphI64 {
        open spec fn spec_ssspresultstephi64_wf(s: &SSSPResultStEphI64) -> bool {
            s.distances.seq@.len() == s.predecessors.seq@.len()
            && s.source < s.distances.seq@.len()
        }

        open spec fn spec_distances(&self) -> Seq<i64> { self.distances.seq@ }

        open spec fn spec_predecessors(&self) -> Seq<usize> { self.predecessors.seq@ }

        open spec fn spec_source(&self) -> usize { self.source }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — allocates and fills distance + predecessor arrays.
        fn new(n: usize, source: usize) -> (empty: Self)
        {
            let mut dist_vec: Vec<i64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    dist_vec@.len() == i as int,
                    forall|k: int| #![trigger dist_vec@[k]] 0 <= k < i ==>
                        dist_vec@[k] == (if k == source as int { 0i64 } else { UNREACHABLE }),
                decreases n - i,
            {
                if i == source { dist_vec.push(0i64); } else { dist_vec.push(UNREACHABLE); }
                i = i + 1;
            }
            let mut pred_vec: Vec<usize> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    pred_vec@.len() == j as int,
                    forall|k: int| #![trigger pred_vec@[k]] 0 <= k < j ==>
                        pred_vec@[k] == NO_PREDECESSOR,
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index read.
        fn get_distance(&self, v: usize) -> (dist: i64) {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index write.
        fn set_distance(&mut self, v: usize, dist: i64)
        {
            if v < self.distances.length() {
                self.distances.seq.set(v, dist);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index read.
        fn get_predecessor(&self, v: usize) -> (pred: Option<usize>) {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index write.
        fn set_predecessor(&mut self, v: usize, pred: usize)
        {
            if v < self.predecessors.length() {
                self.predecessors.seq.set(v, pred);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — comparison with sentinel.
        fn is_reachable(&self, v: usize) -> (b: bool) {
            self.get_distance(v) != UNREACHABLE
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|V|), Span O(|V|) — follows predecessor chain then reverses; St sequential.
        fn extract_path(&self, v: usize) -> (path: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) {
                return None;
            }
            let n = self.predecessors.length();
            if v >= n { return None; }
            let mut path = Vec::new();
            let mut current = v;
            path.push(current);
            let mut steps: usize = 0;
            while current != self.source && steps < n
                invariant
                    steps <= n,
                    current < n,
                    n == self.predecessors.spec_len(),
                    path@.len() > 0,
                    path@[0] == v,
                    path@[path@.len() - 1] == current,
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < n,
                decreases n - steps
            {
                if current >= n {
                    return None;
                }
                let pred = *self.predecessors.nth(current);
                if pred == NO_PREDECESSOR || pred >= n { return None; }
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
                    forall|j: int| #![trigger path@[j]]
                        0 <= j < path@.len() ==> path@[j] < n,
                    n == self.predecessors.spec_len(),
                decreases k
            {
                k = k - 1;
                reversed.push(path[k]);
            }
            Some(ArraySeqStPerS::from_vec(reversed))
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


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
