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
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::seq_set::lemma_take_one_more_extends_the_seq_set_with_view;

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
    pub fn dijkstra(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> SSSPResultStEphI64
        requires
            source < graph.vertices().size(),
            wf_lab_graph_view(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
        ensures
            result.distances.spec_len() == graph.vertices().size(),
            result.source == source,
    {
        let n = graph.vertices().size();

        let mut result = SSSPResultStEphI64::new(n, source);
        let mut visited = SetStEph::<usize>::empty();
        let mut pq = BinaryHeapPQ::<PQEntry>::singleton(pq_entry_new(0, source));
        let ghost verts = graph.vertices()@;

        while !pq.is_empty()
            invariant
                result.distances.spec_len() == n,
                result.source == source,
                visited@.finite(),
                visited@.subset_of(verts),
                result.distances.spec_index(source as int) == 0,
        {
            let (new_pq, min_elem) = pq.delete_min();
            pq = new_pq;

            if let Some(entry) = min_elem {
                let dist = entry.dist;
                let v = entry.vertex;

                if visited.mem(&v) {
                    continue;
                }

                let _ = visited.insert(v);
                result.set_distance(v, dist);

                let neighbors = graph.out_neighbors_weighed(&v);
                let mut it = neighbors.iter();
                let ghost neighbors_seq = it@.1;

                for neighbor in iter: it
                    invariant
                        valid_key_type::<Pair<usize, i128>>(),
                        result.distances.spec_len() == n,
                        result.source == source,
                        visited@.finite(),
                        visited@.subset_of(verts),
                        it.elements == neighbors_seq,
                        neighbors_seq.map(|_i: int, p: Pair<usize, i128>| p@).to_set() == neighbors@,
                {
                    proof { lemma_take_one_more_extends_the_seq_set_with_view(neighbors_seq, it.pos); }
                    let Pair(u, weight) = neighbor;
                    let u_idx = u.clone_plus();

                    if visited.mem(&u_idx) {
                        continue;
                    }

                    let new_dist = dist + (*weight as i64);
                    pq = pq.insert(pq_entry_new(new_dist, u_idx));

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
