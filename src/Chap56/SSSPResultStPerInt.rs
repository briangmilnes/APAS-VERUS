//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Single-Source Shortest Path Result Structure - Sequential Persistent (Integer Weights)
//!
//! Data structure for storing the result of single-source shortest path algorithms
//! with integer edge weights. Stores distance and predecessor arrays for path reconstruction.
//!
//! Uses persistent array sequences for functional-style immutability.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n), Span O(n) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod SSSPResultStPerInt {

    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    const UNREACHABLE: i64 = i64::MAX;
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Result structure for single-source shortest paths with integer weights (persistent).
    pub struct SSSPResultStPerInt {
        /// Distance from source to each vertex (i64::MAX for unreachable).
        pub distances: ArraySeqStPerS<i64>,
        /// Predecessor of each vertex in shortest path tree (usize::MAX for source/unreachable).
        pub predecessors: ArraySeqStPerS<usize>,
        /// Source vertex.
        pub source: usize,
    }

    // 5. view impls

    impl View for SSSPResultStPerInt {
        type V = Seq<int>;
        open spec fn view(&self) -> Self::V {
            self.distances@.map(|_i: int, v: i64| v as int)
        }
    }

    // 8. traits

    /// Trait for single-source shortest path result operations
    pub trait SSSPResultStPerIntTrait: Sized {
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

    // 9. impls

    impl SSSPResultStPerInt {
        /// Creates a new SSSP result structure initialized for n vertices from given source.
        /// All distances are set to UNREACHABLE, all predecessors to NO_PREDECESSOR.
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — agrees with APAS.
        #[verifier::external_body]
        pub fn new(n: usize, source: usize) -> Self {
            let distances = ArraySeqStPerS::tabulate(&|i| if i == source { 0 } else { UNREACHABLE }, n);
            let predecessors = ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n);
            SSSPResultStPerInt {
                distances,
                predecessors,
                source,
            }
        }

        /// Returns the distance from source to vertex v.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn get_distance(&self, v: usize) -> i64 {
            if v >= self.distances.length() {
                return UNREACHABLE;
            }
            *self.distances.nth(v)
        }

        /// Sets the distance from source to vertex v, returning a new structure.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — persistent array update copies path to root.
        #[verifier::external_body]
        pub fn set_distance(self, v: usize, dist: i64) -> Self {
            if v >= self.distances.length() {
                return self;
            }
            SSSPResultStPerInt {
                distances: ArraySeqStPerS::update(&self.distances, v, dist),
                predecessors: self.predecessors,
                source: self.source,
            }
        }

        /// Returns the predecessor of vertex v in the shortest path from source.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array lookup.
        #[verifier::external_body]
        pub fn get_predecessor(&self, v: usize) -> Option<usize> {
            if v >= self.predecessors.length() {
                return None;
            }
            let pred = *self.predecessors.nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// Sets the predecessor of vertex v, returning a new structure.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — persistent array update copies path to root.
        #[verifier::external_body]
        pub fn set_predecessor(self, v: usize, pred: usize) -> Self {
            if v >= self.predecessors.length() {
                return self;
            }
            SSSPResultStPerInt {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, v, pred),
                source: self.source,
            }
        }

        /// Checks if vertex v is reachable from source.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn is_reachable(&self, v: usize) -> bool { self.get_distance(v) != UNREACHABLE }

        /// Extracts the shortest path from source to vertex v by following predecessors.
        /// Returns None if v is unreachable, otherwise returns the path as a sequence.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — follows k predecessor links.
        #[verifier::external_body]
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

    } // verus!
}
