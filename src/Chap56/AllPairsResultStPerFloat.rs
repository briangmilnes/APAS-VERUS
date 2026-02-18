//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Persistent (Float Weights)
//!
//! Data structure for storing the result of all-pairs shortest path algorithms
//! with floating-point edge weights. Stores distance matrix and predecessor matrix for path reconstruction.
//!
//! Uses persistent array sequences for functional-style immutability.
//! Uses `OrderedF64` (OrderedFloat<f64>) for weights to ensure Eq/Hash traits.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n²), Span O(n²) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod AllPairsResultStPerFloat {

    use ordered_float::OrderedFloat;

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    const UNREACHABLE: OrderedF64 = OrderedFloat(f64::INFINITY);
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Result structure for all-pairs shortest paths with floating-point weights (persistent).
    pub struct AllPairsResultStPerFloat {
        /// Distance matrix: distances.nth(u).nth(v) is the distance from u to v.
        pub distances: ArraySeqStPerS<ArraySeqStPerS<OrderedF64>>,
        /// Predecessor matrix: predecessors.nth(u).nth(v) is the predecessor of v on shortest path from u.
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        /// Number of vertices.
        pub n: usize,
    }

    // 5. view impls

    impl View for AllPairsResultStPerFloat {
        type V = Seq<Seq<int>>;
        open spec fn view(&self) -> Self::V {
            Seq::new(self.predecessors@.len(), |i: int|
                self.predecessors@[i]@.map(|_j: int, v: usize| v as int)
            )
        }
    }

    // 8. traits

    /// Trait for all-pairs shortest path result operations
    pub trait AllPairsResultStPerFloatTrait: Sized {
        fn new(n: usize) -> (result: Self);

        fn get_distance(&self, u: usize, v: usize) -> (dist: OrderedF64);

        fn set_distance(self, u: usize, v: usize, dist: OrderedF64) -> (result: Self);

        fn get_predecessor(&self, u: usize, v: usize) -> (result: Option<usize>);

        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (result: Self);

        fn is_reachable(&self, u: usize, v: usize) -> (result: bool);

        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl AllPairsResultStPerFloatTrait for AllPairsResultStPerFloat {
        #[verifier::external_body]
        fn new(n: usize) -> (result: Self)
            ensures
                result.n == n,
        {
            let distances = ArraySeqStPerS::tabulate(
                &|i| ArraySeqStPerS::tabulate(&|j| if i == j { OrderedFloat(0.0) } else { UNREACHABLE }, n),
                n,
            );
            let predecessors = ArraySeqStPerS::tabulate(&|_| ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n), n);
            AllPairsResultStPerFloat {
                distances,
                predecessors,
                n,
            }
        }

        #[verifier::external_body]
        fn get_distance(&self, u: usize, v: usize) -> (dist: OrderedF64)
            ensures
                u < self.n && v < self.n ==> dist == self.distances@[u as int]@[v as int],
                (u >= self.n || v >= self.n) ==> dist == UNREACHABLE,
        {
            if u >= self.n || v >= self.n {
                return UNREACHABLE;
            }
            *self.distances.nth(u).nth(v)
        }

        #[verifier::external_body]
        fn set_distance(self, u: usize, v: usize, dist: OrderedF64) -> (result: Self)
            ensures
                result.n == self.n,
                result.predecessors@ == self.predecessors@,
        {
            if u >= self.n || v >= self.n {
                return self;
            }
            let updated_row = ArraySeqStPerS::update(self.distances.nth(u), v, dist);
            AllPairsResultStPerFloat {
                distances: ArraySeqStPerS::update(&self.distances, u, updated_row),
                predecessors: self.predecessors,
                n: self.n,
            }
        }

        #[verifier::external_body]
        fn get_predecessor(&self, u: usize, v: usize) -> (result: Option<usize>)
            ensures
                (u >= self.n || v >= self.n) ==> result.is_none(),
                u < self.n && v < self.n && self.predecessors@[u as int]@[v as int] == NO_PREDECESSOR ==> result.is_none(),
                u < self.n && v < self.n && self.predecessors@[u as int]@[v as int] != NO_PREDECESSOR ==> result == Some(self.predecessors@[u as int]@[v as int]),
        {
            if u >= self.n || v >= self.n {
                return None;
            }
            let pred = *self.predecessors.nth(u).nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        #[verifier::external_body]
        fn set_predecessor(self, u: usize, v: usize, pred: usize) -> (result: Self)
            ensures
                result.n == self.n,
                result.distances@ == self.distances@,
        {
            if u >= self.n || v >= self.n {
                return self;
            }
            let updated_row = ArraySeqStPerS::update(self.predecessors.nth(u), v, pred);
            AllPairsResultStPerFloat {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, u, updated_row),
                n: self.n,
            }
        }

        #[verifier::external_body]
        fn is_reachable(&self, u: usize, v: usize) -> (result: bool) { self.get_distance(u, v).is_finite() }

        #[verifier::external_body]
        fn extract_path(&self, u: usize, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
            if u == v {
                return Some(ArraySeqStPerS::from_vec(vec![u]));
            }
            if !self.is_reachable(u, v) {
                return None;
            }

            let mut path = Vec::new();
            let mut current = v;
            path.push(current);

            while current != u {
                let pred = *self.predecessors.nth(u).nth(current);
                if pred == NO_PREDECESSOR {
                    return None;
                }
                path.push(pred);
                current = pred;
            }

            path.reverse();
            Some(ArraySeqStPerS::from_vec(path))
        }
    }

    } // verus!
}
