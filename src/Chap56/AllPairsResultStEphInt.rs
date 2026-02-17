//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! All-Pairs Shortest Path Result Structure - Sequential Ephemeral (Integer Weights)
//!
//! Data structure for storing the result of all-pairs shortest path algorithms
//! with integer edge weights. Stores distance matrix and predecessor matrix for path reconstruction.
//!
//! Uses ephemeral array sequences for efficient in-place updates.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(n²), Span O(n²) for n vertices
//! - `get_distance`: Work O(1), Span O(1)
//! - `extract_path`: Work O(k), Span O(k) where k is path length

pub mod AllPairsResultStEphInt {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const UNREACHABLE: i64 = i64::MAX;
    const NO_PREDECESSOR: usize = usize::MAX;

    /// Trait for all-pairs shortest path result operations
    pub trait AllPairsResultStEphIntTrait {
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

    /// Result structure for all-pairs shortest paths with integer weights.
    pub struct AllPairsResultStEphInt {
        /// Distance matrix: distances.nth(u).nth(v) is the distance from u to v.
        pub distances: ArraySeqStEphS<ArraySeqStEphS<i64>>,
        /// Predecessor matrix: predecessors.nth(u).nth(v) is the predecessor of v on shortest path from u.
        pub predecessors: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        /// Number of vertices.
        pub n: usize,
    }

    impl AllPairsResultStEphInt {
        /// Creates a new all-pairs result structure initialized for n vertices.
        /// All distances are set to UNREACHABLE except diagonal (0), all predecessors to NO_PREDECESSOR.
        pub fn new(n: usize) -> Self {
            let mut dist_matrix = Vec::with_capacity(n);
            for i in 0..n {
                let mut row = vec![UNREACHABLE; n];
                row[i] = 0;
                dist_matrix.push(ArraySeqStEphS::from_vec(row));
            }
            let distances = ArraySeqStEphS::from_vec(dist_matrix);

            let pred_matrix = vec![ArraySeqStEphS::new(n, NO_PREDECESSOR); n];
            let predecessors = ArraySeqStEphS::from_vec(pred_matrix);
            AllPairsResultStEphInt {
                distances,
                predecessors,
                n,
            }
        }

        /// Returns the distance from vertex u to vertex v.
        pub fn get_distance(&self, u: usize, v: usize) -> i64 {
            if u >= self.n || v >= self.n {
                return UNREACHABLE;
            }
            *self.distances.nth(u).nth(v)
        }

        /// Sets the distance from vertex u to vertex v.
        pub fn set_distance(&mut self, u: usize, v: usize, dist: i64) {
            if u < self.n && v < self.n {
                let mut row = self.distances.nth(u).clone();
                let _ = row.set(v, dist);
                let _ = self.distances.set(u, row);
            }
        }

        /// Returns the predecessor of vertex v in the shortest path from u.
        pub fn get_predecessor(&self, u: usize, v: usize) -> Option<usize> {
            if u >= self.n || v >= self.n {
                return None;
            }
            let pred = *self.predecessors.nth(u).nth(v);
            if pred == NO_PREDECESSOR { None } else { Some(pred) }
        }

        /// Sets the predecessor of vertex v in the shortest path from u.
        pub fn set_predecessor(&mut self, u: usize, v: usize, pred: usize) {
            if u < self.n && v < self.n {
                let mut row = self.predecessors.nth(u).clone();
                let _ = row.set(v, pred);
                let _ = self.predecessors.set(u, row);
            }
        }

        /// Checks if vertex v is reachable from vertex u.
        pub fn is_reachable(&self, u: usize, v: usize) -> bool { self.get_distance(u, v) != UNREACHABLE }

        /// Extracts the shortest path from u to v by following predecessors.
        /// Returns None if v is unreachable from u, otherwise returns the path as a sequence.
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
