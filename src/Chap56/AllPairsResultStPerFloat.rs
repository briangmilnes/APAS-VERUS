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
        /// Create new all-pairs result
        /// APAS: Work Θ(n²), Span Θ(n²)
        fn new(n: N)                   -> Self;

        /// Get distance between vertices
        /// APAS: Work Θ(1), Span Θ(1)
        fn distance(&self, u: N, v: N) -> Option<OrderedF64>;

        /// Check if path exists
        /// APAS: Work Θ(1), Span Θ(1)
        fn has_path(&self, u: N, v: N) -> B;
    }

    // 9. impls

    impl AllPairsResultStPerFloat {
        /// Creates a new all-pairs result structure initialized for n vertices.
        /// All distances are set to UNREACHABLE except diagonal (0.0), all predecessors to NO_PREDECESSOR.
        /// - APAS: Work Θ(n²), Span Θ(n²)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — agrees with APAS.
        #[verifier::external_body]
        pub fn new(n: usize) -> Self {
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

        /// Returns the distance from vertex u to vertex v.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn get_distance(&self, u: usize, v: usize) -> OrderedF64 {
            if u >= self.n || v >= self.n {
                return UNREACHABLE;
            }
            *self.distances.nth(u).nth(v)
        }

        /// Sets the distance from vertex u to vertex v, returning a new structure.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — persistent row update plus outer array update.
        #[verifier::external_body]
        pub fn set_distance(self, u: usize, v: usize, dist: OrderedF64) -> Self {
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

        /// Returns the predecessor of vertex v in the shortest path from u.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — nested array lookup.
        #[verifier::external_body]
        pub fn get_predecessor(&self, u: usize, v: usize) -> Option<usize> {
            if u >= self.n || v >= self.n {
                return None;
            }
            let pred = *self.predecessors.nth(u).nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// Sets the predecessor of vertex v in the shortest path from u, returning a new structure.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — persistent row update plus outer array update.
        #[verifier::external_body]
        pub fn set_predecessor(self, u: usize, v: usize, pred: usize) -> Self {
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

        /// Checks if vertex v is reachable from vertex u.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn is_reachable(&self, u: usize, v: usize) -> bool { self.get_distance(u, v).is_finite() }

        /// Extracts the shortest path from u to v by following predecessors.
        /// Returns None if v is unreachable from u, otherwise returns the path as a sequence.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — follows k predecessor links.
        #[verifier::external_body]
        pub fn extract_path(&self, u: usize, v: usize) -> Option<ArraySeqStPerS<usize>> {
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
