//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Persistent (Float Weights)
//!
//! Data structure for storing the result of single-source shortest path algorithms
//! with floating-point edge weights. Stores distance and predecessor arrays for path reconstruction.
//!
//! Uses persistent array sequences for functional-style immutability.
//! Uses `OrderedF64` (OrderedFloat<f64>) for weights to ensure Eq/Hash traits.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n), Span O(n) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod SSSPResultStPerFloat {

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

    /// Result structure for single-source shortest paths with floating-point weights (persistent).
    pub struct SSSPResultStPerFloat {
        /// Distance from source to each vertex (OrderedFloat(f64::INFINITY) for unreachable).
        pub distances: ArraySeqStPerS<OrderedF64>,
        /// Predecessor of each vertex in shortest path tree (usize::MAX for source/unreachable).
        pub predecessors: ArraySeqStPerS<usize>,
        /// Source vertex.
        pub source: usize,
    }

    // 5. view impls

    impl View for SSSPResultStPerFloat {
        type V = Seq<int>;
        open spec fn view(&self) -> Self::V {
            self.predecessors@.map(|_i: int, v: usize| v as int)
        }
    }

    // 8. traits

    /// Trait for single-source shortest path result operations
    pub trait SSSPResultStPerFloatTrait: Sized {
        fn new(n: usize, source: usize) -> (result: Self)
            requires source < n;

        fn get_distance(&self, v: usize) -> (dist: OrderedF64);

        fn set_distance(self, v: usize, dist: OrderedF64) -> (result: Self);

        fn get_predecessor(&self, v: usize) -> (result: Option<usize>);

        fn set_predecessor(self, v: usize, pred: usize) -> (result: Self);

        fn is_reachable(&self, v: usize) -> (result: bool);

        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>);
    }

    // 9. impls

    impl SSSPResultStPerFloatTrait for SSSPResultStPerFloat {
        #[verifier::external_body]
        fn new(n: usize, source: usize) -> (result: Self)
            ensures
                result.distances@.len() == n,
                result.predecessors@.len() == n,
                result.source == source,
        {
            let distances = ArraySeqStPerS::tabulate(&|i| if i == source { OrderedFloat(0.0) } else { UNREACHABLE }, n);
            let predecessors = ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n);
            SSSPResultStPerFloat {
                distances,
                predecessors,
                source,
            }
        }

        #[verifier::external_body]
        fn get_distance(&self, v: usize) -> (dist: OrderedF64)
            ensures
                v < self.distances@.len() ==> dist == self.distances@[v as int],
                v >= self.distances@.len() ==> dist == UNREACHABLE,
        {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        #[verifier::external_body]
        fn set_distance(self, v: usize, dist: OrderedF64) -> (result: Self)
            ensures
                v < self.distances@.len() ==> result.distances@ == self.distances@.update(v as int, dist),
                v >= self.distances@.len() ==> result.distances@ == self.distances@,
                result.predecessors@ == self.predecessors@,
                result.source == self.source,
        {
            if v >= self.distances.length() {
                return self;
            }
            SSSPResultStPerFloat {
                distances: ArraySeqStPerS::update(&self.distances, v, dist),
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        #[verifier::external_body]
        fn get_predecessor(&self, v: usize) -> (result: Option<usize>)
            ensures
                v >= self.predecessors@.len() ==> result.is_none(),
                v < self.predecessors@.len() && self.predecessors@[v as int] == NO_PREDECESSOR ==> result.is_none(),
                v < self.predecessors@.len() && self.predecessors@[v as int] != NO_PREDECESSOR ==> result == Some(self.predecessors@[v as int]),
        {
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
            if v >= self.predecessors.length() {
                return self;
            }
            SSSPResultStPerFloat {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, v, pred),
                source: self.source,
            }
        }

        #[verifier::external_body]
        fn is_reachable(&self, v: usize) -> (result: bool) { self.get_distance(v).is_finite() }

        #[verifier::external_body]
        fn extract_path(&self, v: usize) -> (result: Option<ArraySeqStPerS<usize>>) {
            if !self.is_reachable(v) {
                return None;
            }

            let mut path = Vec::new();
            let mut current = v;
            path.push(current);

            while current != self.source {
                let pred = *self.predecessors.nth(current);
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
