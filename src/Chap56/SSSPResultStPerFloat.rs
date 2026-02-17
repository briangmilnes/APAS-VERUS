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

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const UNREACHABLE: OrderedF64 = OrderedFloat(f64::INFINITY);
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Trait for single-source shortest path result operations
    pub trait SSSPResultStPerFloatTrait {
        /// Create new SSSP result
        /// APAS: Work Θ(n), Span Θ(n)
        fn new(n: N, source: N)      -> Self;

        /// Get distance to vertex
        /// APAS: Work Θ(1), Span Θ(1)
        fn distance(&self, v: N)     -> Option<OrderedF64>;

        /// Check if vertex is reachable
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_reachable(&self, v: N) -> B;
    }

    /// Result structure for single-source shortest paths with floating-point weights (persistent).
    pub struct SSSPResultStPerFloat {
        /// Distance from source to each vertex (OrderedFloat(f64::INFINITY) for unreachable).
        pub distances: ArraySeqStPerS<OrderedF64>,
        /// Predecessor of each vertex in shortest path tree (usize::MAX for source/unreachable).
        pub predecessors: ArraySeqStPerS<usize>,
        /// Source vertex.
        pub source: usize,
    }

    impl SSSPResultStPerFloat {
        /// Creates a new SSSP result structure initialized for n vertices from given source.
        /// All distances are set to UNREACHABLE, all predecessors to NO_PREDECESSOR.
        pub fn new(n: usize, source: usize) -> Self {
            let distances = ArraySeqStPerS::tabulate(&|i| if i == source { OrderedFloat(0.0) } else { UNREACHABLE }, n);
            let predecessors = ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n);
            SSSPResultStPerFloat {
                distances,
                predecessors,
                source,
            }
        }

        /// Returns the distance from source to vertex v.
        pub fn get_distance(&self, v: usize) -> OrderedF64 {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        /// Sets the distance from source to vertex v, returning a new structure.
        pub fn set_distance(self, v: usize, dist: OrderedF64) -> Self {
            if v >= self.distances.length() {
                return self;
            }
            SSSPResultStPerFloat {
                distances: ArraySeqStPerS::update(&self.distances, v, dist),
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        /// Returns the predecessor of vertex v in the shortest path from source.
        pub fn get_predecessor(&self, v: usize) -> Option<usize> {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// Sets the predecessor of vertex v, returning a new structure.
        pub fn set_predecessor(self, v: usize, pred: usize) -> Self {
            if v >= self.predecessors.length() {
                return self;
            }
            SSSPResultStPerFloat {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, v, pred),
                source: self.source,
            }
        }

        /// Checks if vertex v is reachable from source.
        pub fn is_reachable(&self, v: usize) -> bool { self.get_distance(v).is_finite() }

        /// Extracts the shortest path from source to vertex v by following predecessors.
        /// Returns None if v is unreachable, otherwise returns the path as a sequence.
        pub fn extract_path(&self, v: usize) -> Option<ArraySeqStPerS<usize>> {
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
}
