//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Dijkstra's Algorithm - Single Source Shortest Path (SSSP+) for non-negative float edge weights
//!
//! Implements Algorithm 57.2 from the textbook using priority queues.
//!
//! **Algorithmic Analysis:**
//! - Dijkstra: Work O(m log n), Span O(m log n) where m = |E|, n = |V|

pub mod DijkstraStEphFloat {

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::Chap56::SSSPResultStEphFloat::SSSPResultStEphFloat::SSSPResultStEphFloat;
    use crate::Types::Types::*;

    pub type T = PQEntry;

    pub trait DijkstraStEphFloatTrait {
        /// Dijkstra's single source shortest path algorithm
        /// APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
        fn dijkstra(graph: &WeightedDirGraphStEphFloat<usize>, source: usize) -> SSSPResultStEphFloat;
    }

    /// Priority queue entry: (distance, vertex)
    /// Ordered by distance (min-heap)
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct PQEntry {
        dist: OrderedF64,
        vertex: usize,
    }

    /// Module-level function to create a new PQEntry
    fn pq_entry_new(dist: OrderedF64, vertex: usize) -> PQEntry { PQEntry { dist, vertex } }

    impl Ord for PQEntry {
        fn cmp(&self, other: &Self) -> Ordering {
            // Min-heap: smaller distance has higher priority
            self.dist.cmp(&other.dist)
        }
    }

    impl PartialOrd for PQEntry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl Display for PQEntry {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.dist, self.vertex) }
    }

    /// Runs Dijkstra's algorithm on a weighted directed graph
    /// Computes single-source shortest paths for non-negative edge weights
    ///
    /// **Algorithm 57.2**: Priority-First Search using Priority Queue
    ///
    /// Work: O(m log n), Span: O(m log n)
    /// where n = number of vertices, m = number of edges
    ///
    /// # Arguments
    /// * `graph` - Weighted directed graph with non-negative float weights (usize vertices)
    /// * `source` - Source vertex (0-indexed)
    ///
    /// # Returns
    /// SSSPResultStEphFloat with distances and predecessors
    pub fn dijkstra(graph: &WeightedDirGraphStEphFloat<usize>, source: usize) -> SSSPResultStEphFloat {
        let n = graph.vertices().size();

        // Initialize result with all distances = infinity except source = 0
        let mut result = SSSPResultStEphFloat::new(n, source);

        // Track visited vertices (X in the algorithm)
        let mut visited = HashMap::<usize, OrderedF64>::new();

        // Priority queue Q: stores PQEntry(distance, vertex)
        // BinaryHeapPQ is a min-heap
        let mut pq = BinaryHeapPQ::<PQEntry>::singleton(pq_entry_new(OrderedF64::from(0.0), source));

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
                let neighbors = graph.out_neighbors_weighted(&v);
                for neighbor in neighbors.iter() {
                    let Pair(u, weight) = neighbor;
                    let u_idx = *u;

                    // Skip if already visited
                    if visited.contains_key(&u_idx) {
                        continue;
                    }

                    let new_dist = OrderedF64::from(dist.0 + weight.0);
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

    verus! {
        impl View for PQEntry {
            type V = Self;
            open spec fn view(&self) -> Self { *self }
        }
    }
}
