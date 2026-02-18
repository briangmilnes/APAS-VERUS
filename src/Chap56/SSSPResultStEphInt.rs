//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)
//!
//! Data structure for storing the result of single-source shortest path algorithms
//! with integer edge weights. Stores distance and predecessor arrays for path reconstruction.
//!
//! Uses ephemeral array sequences for efficient in-place updates.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n), Span O(n) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod SSSPResultStEphInt {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const UNREACHABLE: i64 = i64::MAX;
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Trait for single-source shortest path result operations
    pub trait SSSPResultStEphIntTrait {
        /// Create new SSSP result
        /// APAS: Work Θ(n), Span Θ(n)
        fn new(n: N, source: N)      -> Self;

        /// Get distance to vertex
        /// APAS: Work Θ(1), Span Θ(1)
        fn distance(&self, v: N)     -> Option<i32>;

        /// Check if vertex is reachable
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_reachable(&self, v: N) -> B;
    }

    /// Result structure for single-source shortest paths with integer weights.
    pub struct SSSPResultStEphInt {
        /// Distance from source to each vertex (i64::MAX for unreachable).
        pub distances: ArraySeqStEphS<i64>,
        /// Predecessor of each vertex in shortest path tree (usize::MAX for source/unreachable).
        pub predecessors: ArraySeqStEphS<usize>,
        /// Source vertex.
        pub source: usize,
    }

    impl SSSPResultStEphInt {
        /// Creates a new SSSP result structure initialized for n vertices from given source.
        /// All distances are set to UNREACHABLE, all predecessors to NO_PREDECESSOR.
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — agrees with APAS.
        pub fn new(n: usize, source: usize) -> Self {
            let mut dist_vec = vec![UNREACHABLE; n];
            dist_vec[source] = 0;
            let distances = ArraySeqStEphS::from_vec(dist_vec);
            let predecessors = ArraySeqStEphS::new(n, NO_PREDECESSOR);
            SSSPResultStEphInt {
                distances,
                predecessors,
                source,
            }
        }

        /// Returns the distance from source to vertex v.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn get_distance(&self, v: usize) -> i64 {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        /// Sets the distance from source to vertex v.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — in-place array update.
        pub fn set_distance(&mut self, v: usize, dist: i64) {
            if v < self.distances.length() {
                let _ = self.distances.set(v, dist);
            }
        }

        /// Returns the predecessor of vertex v in the shortest path from source.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array lookup.
        pub fn get_predecessor(&self, v: usize) -> Option<usize> {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// Sets the predecessor of vertex v in the shortest path from source.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — in-place array update.
        pub fn set_predecessor(&mut self, v: usize, pred: usize) {
            if v < self.predecessors.length() {
                let _ = self.predecessors.set(v, pred);
            }
        }

        /// Checks if vertex v is reachable from source.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn is_reachable(&self, v: usize) -> bool { self.get_distance(v) != UNREACHABLE }

        /// Extracts the shortest path from source to vertex v by following predecessors.
        /// Returns None if v is unreachable, otherwise returns the path as a sequence.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — follows k predecessor links.
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
