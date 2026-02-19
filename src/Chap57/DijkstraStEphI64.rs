//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Dijkstra's Algorithm - Single Source Shortest Path (SSSP+) for non-negative integer edge weights
//!
//! Implements Algorithm 57.2 from the textbook using priority queues.
//!
//! **Algorithmic Analysis:**
//! - Dijkstra: Work O(m log n), Span O(m log n) where m = |E|, n = |V|

pub mod DijkstraStEphI64 {

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (DijkstraStEphI64)
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 13. derive impls outside verus!

    // 4. type definitions

    pub type T = PQEntry;

    /// Priority queue entry: (distance, vertex)
    /// Ordered by distance (min-heap)
    #[derive(Clone, Eq, PartialEq)]
    pub struct PQEntry {
        pub dist: i64,
        pub vertex: usize,
    }

    // 5. view impls

    impl View for PQEntry {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    // 8. traits

    pub trait DijkstraStEphI64Trait {
        /// Dijkstra's single source shortest path algorithm
        /// - APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
        /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — agrees with APAS.
        fn dijkstra(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> SSSPResultStEphI64;
    }

    // 9. impls

    fn pq_entry_new(dist: i64, vertex: usize) -> (r: PQEntry)
        ensures r.dist == dist, r.vertex == vertex,
    {
        PQEntry { dist, vertex }
    }

    impl Ord for PQEntry {
        fn cmp(&self, other: &Self) -> (r: Ordering) {
            if self.dist < other.dist {
                Ordering::Less
            } else if self.dist == other.dist {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }

    impl PartialOrd for PQEntry {
        fn partial_cmp(&self, other: &Self) -> (r: Option<Ordering>) {
            Some(self.cmp(other))
        }
    }

    /// Runs Dijkstra's algorithm on a weighted directed graph.
    /// Computes single-source shortest paths for non-negative edge weights.
    ///
    /// **Algorithm 57.2**: Priority-First Search using Priority Queue
    ///
    /// - APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
    /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — agrees with APAS. Sequential
    ///   implementation with BinaryHeapPQ insert/deleteMin at O(log m) each, m edge relaxations.
    #[verifier::external_body]
    pub fn dijkstra(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> SSSPResultStEphI64 {
        let n = graph.vertices().size();

        // Initialize result with all distances = infinity except source = 0
        let mut result = SSSPResultStEphI64::new(n, source);

        // Track visited vertices (X in the algorithm)
        let mut visited = HashMap::<usize, i64>::new();

        // Priority queue Q: stores PQEntry(distance, vertex)
        // BinaryHeapPQ is a min-heap
        let mut pq = BinaryHeapPQ::<PQEntry>::singleton(pq_entry_new(0, source));

        // Main loop: deleteMin until queue is empty
        while !pq.is_empty() {
            // deleteMin from priority queue
            let (new_pq, min_elem) = pq.delete_min();
            pq = new_pq;

            if let Some(entry) = min_elem {
                let dist = entry.dist;
                let v = entry.vertex;

                // Skip if already visited (handles duplicate entries)
                if visited.contains_key(&v) {
                    continue;
                }

                // Mark v as visited with distance dist
                visited.insert(v, dist);
                result.set_distance(v, dist);

                // Relax all out-neighbors: add PQEntry(d + w, u) to PQ
                let neighbors = graph.out_neighbors_weighed(&v);
                for neighbor in neighbors.iter() {
                    let Pair(u, weight) = neighbor;
                    let u_idx = *u;

                    // Skip if already visited
                    if visited.contains_key(&u_idx) {
                        continue;
                    }

                    let new_dist = dist + (*weight as i64);
                    pq = pq.insert(pq_entry_new(new_dist, u_idx));

                    // Update predecessor if this is a better path
                    // (First time we reach u with minimum distance through PQ ordering)
                    if result.get_distance(u_idx) > new_dist {
                        result.set_predecessor(u_idx, v);
                    }
                }
            }
        }

        result
    }

    } // verus!

    // 13. derive impls outside verus!

    impl Debug for PQEntry {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("PQEntry")
                .field("dist", &self.dist)
                .field("vertex", &self.vertex)
                .finish()
        }
    }

    impl Display for PQEntry {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.dist, self.vertex) }
    }
}
