//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Persistent (Integer Weights)
//!
//! Data structure for storing the result of all-pairs shortest path algorithms
//! with integer edge weights. Stores distance matrix and predecessor matrix for path reconstruction.
//!
//! Uses persistent array sequences for functional-style immutability.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n²), Span O(n²) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod AllPairsResultStPerInt {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const UNREACHABLE: i64 = i64::MAX;
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Trait for all-pairs shortest path result operations
    pub trait AllPairsResultStPerIntTrait {
        /// Create new all-pairs result
        /// APAS: Work Θ(n²), Span Θ(n²)
        fn new(n: N)                   -> Self;

        /// Get distance between vertices
        /// APAS: Work Θ(1), Span Θ(1)
        fn distance(&self, u: N, v: N) -> Option<i32>;

        /// Check if path exists
        /// APAS: Work Θ(1), Span Θ(1)
        fn has_path(&self, u: N, v: N) -> B;
    }

    /// Result structure for all-pairs shortest paths with integer weights (persistent).
    pub struct AllPairsResultStPerInt {
        /// Distance matrix: distances.nth(u).nth(v) is the distance from u to v.
        pub distances: ArraySeqStPerS<ArraySeqStPerS<i64>>,
        /// Predecessor matrix: predecessors.nth(u).nth(v) is the predecessor of v on shortest path from u.
        pub predecessors: ArraySeqStPerS<ArraySeqStPerS<usize>>,
        /// Number of vertices.
        pub n: usize,
    }

    impl AllPairsResultStPerInt {
        /// Creates a new all-pairs result structure initialized for n vertices.
        /// All distances are set to UNREACHABLE except diagonal (0), all predecessors to NO_PREDECESSOR.
        /// - APAS: Work Θ(n²), Span Θ(n²)
        /// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — agrees with APAS.
        pub fn new(n: usize) -> Self {
            let distances = ArraySeqStPerS::tabulate(
                &|i| ArraySeqStPerS::tabulate(&|j| if i == j { 0 } else { UNREACHABLE }, n),
                n,
            );
            let predecessors = ArraySeqStPerS::tabulate(&|_| ArraySeqStPerS::tabulate(&|_| NO_PREDECESSOR, n), n);
            AllPairsResultStPerInt {
                distances,
                predecessors,
                n,
            }
        }

        /// Returns the distance from vertex u to vertex v.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn get_distance(&self, u: usize, v: usize) -> i64 {
            if u >= self.n || v >= self.n {
                return UNREACHABLE;
            }
            *self.distances.nth(u).nth(v)
        }

        /// Sets the distance from vertex u to vertex v, returning a new structure.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — persistent row update plus outer array update.
        pub fn set_distance(self, u: usize, v: usize, dist: i64) -> Self {
            if u >= self.n || v >= self.n {
                return self;
            }
            let updated_row = ArraySeqStPerS::update(self.distances.nth(u), v, dist);
            AllPairsResultStPerInt {
                distances: ArraySeqStPerS::update(&self.distances, u, updated_row),
                predecessors: self.predecessors,
                n: self.n,
            }
        }

        /// Returns the predecessor of vertex v in the shortest path from u.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — nested array lookup.
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
        pub fn set_predecessor(self, u: usize, v: usize, pred: usize) -> Self {
            if u >= self.n || v >= self.n {
                return self;
            }
            let updated_row = ArraySeqStPerS::update(self.predecessors.nth(u), v, pred);
            AllPairsResultStPerInt {
                distances: self.distances,
                predecessors: ArraySeqStPerS::update(&self.predecessors, u, updated_row),
                n: self.n,
            }
        }

        /// Checks if vertex v is reachable from vertex u.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        pub fn is_reachable(&self, u: usize, v: usize) -> bool { self.get_distance(u, v) != UNREACHABLE }

        /// Extracts the shortest path from u to v by following predecessors.
        /// Returns None if v is unreachable from u, otherwise returns the path as a sequence.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(k), Span Θ(k) — follows k predecessor links.
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
}
