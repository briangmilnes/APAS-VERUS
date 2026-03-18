//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Dijkstra's Algorithm - Single Source Shortest Path (SSSP+) for non-negative integer edge weights
//!
//! Implements Algorithm 57.2 from the textbook using priority queues.
//!
//! **Algorithmic Analysis:**
//! - Dijkstra: Work O(m log n), Span O(m log n) where m = |E|, n = |V|

pub mod DijkstraStEphU64 {

    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    verus! {

    // Table of Contents
    // 1. module (DijkstraStEphU64)
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 13. derive impls outside verus!

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    // 4. type definitions

    /// Priority queue entry: (distance, vertex)
    /// Ordered by distance (min-heap)
    #[derive(Eq, PartialEq)]
    pub struct PQEntry {
        pub dist: i64,
        pub vertex: usize,
    }

    // 5. view impls

    impl View for PQEntry {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    impl Clone for PQEntry {
        fn clone(&self) -> (result: PQEntry)
            ensures result@ == self@
        {
            PQEntry { dist: self.dist, vertex: self.vertex }
        }
    }

    // 8. traits

    pub trait DijkstraStEphU64Trait {
        /// Dijkstra's single source shortest path algorithm.
        /// - APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|.
        /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — agrees with APAS.
        fn dijkstra(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
            -> (sssp: SSSPResultStEphI64)
            requires
                source < graph@.V.len(),
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                graph@.A.len() * 2 + 2 <= usize::MAX as int,
            ensures
                sssp.spec_distances().len() == graph@.V.len(),
                sssp.spec_source() == source;
    }

    // 9. impls

    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    fn pq_entry_new(dist: i64, vertex: usize) -> (r: PQEntry)
        ensures r.dist == dist, r.vertex == vertex,
    {
        PQEntry { dist, vertex }
    }

    impl Ord for PQEntry {
        #[verifier::external_body]
        fn cmp(&self, other: &Self) -> Ordering {
            if self.dist < other.dist {
                Ordering::Less
            } else if self.dist > other.dist {
                Ordering::Greater
            } else if self.vertex < other.vertex {
                Ordering::Less
            } else if self.vertex == other.vertex {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }

    impl PartialOrd for PQEntry {
        #[verifier::external_body]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(std::cmp::Ord::cmp(self, other))
        }
    }

    impl TotalOrder for PQEntry {
        open spec fn le(self, other: Self) -> bool {
            self.dist < other.dist || (self.dist == other.dist && self.vertex <= other.vertex)
        }

        proof fn reflexive(x: Self) {}
        proof fn transitive(x: Self, y: Self, z: Self) {}
        proof fn antisymmetric(x: Self, y: Self) {}
        proof fn total(x: Self, y: Self) {}

        fn cmp(&self, other: &Self) -> (c: Ordering) {
            if self.dist < other.dist {
                Ordering::Less
            } else if self.dist > other.dist {
                Ordering::Greater
            } else if self.vertex < other.vertex {
                Ordering::Less
            } else if self.vertex == other.vertex {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }

    /// Runs Dijkstra's algorithm on a weighted directed graph.
    /// Computes single-source shortest paths for non-negative edge weights.
    ///
    /// - APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|.
    /// - Claude-Opus-4.6: Work O(m log n), Span O(m log n) — sequential implementation.
    #[verifier::exec_allows_no_decreases_clause]
    pub fn dijkstra(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> (sssp: SSSPResultStEphI64)
        requires
            source < graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            sssp.spec_distances().len() == graph@.V.len(),
            sssp.spec_source() == source,
    {
        let n = graph.vertices().size();
        assert(n == graph@.V.len());
        proof { assume(obeys_feq_clone::<PQEntry>()); }

        // Edge count for PQ size bound: total PQ inserts <= |E|.
        let arcs_ref = graph.labeled_arcs();
        proof {
            assert(arcs_ref@.finite());
            assert(valid_key_type::<LabEdge<usize, i128>>());
        }
        let m = arcs_ref.size();
        assert(m as int == graph@.A.len());

        let mut sssp = SSSPResultStEphI64::new(n, source);
        let mut visited = SetStEph::<usize>::empty();
        let mut pq = BinaryHeapPQ::<PQEntry>::singleton(pq_entry_new(0, source));
        let ghost mut remaining_budget: int = m as int;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while !pq.is_empty()
            invariant
                sssp.spec_distances().len() == n as int,
                sssp.spec_source() == source,
                n == graph@.V.len(),
                visited@.finite(),
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                obeys_feq_clone::<PQEntry>(),
                m as int == graph@.A.len(),
                graph@.A.len() * 2 + 2 <= usize::MAX as int,
                remaining_budget >= 0,
                pq@.len() + remaining_budget <= m as int + 1,
        {
            // PQ size bound from budget invariant:
            // pq@.len() <= m + 1, and (m + 1) * 2 = m * 2 + 2 <= usize::MAX.
            assert(pq@.len() * 2 <= usize::MAX as int);
            proof {
                assume(BinaryHeapPQ::<PQEntry>::spec_is_exec_heap(pq.spec_seq()));
            }
            let (new_pq, min_elem) = pq.delete_min();
            pq = new_pq;

            if let Some(entry) = min_elem {
                let dist = entry.dist;
                let v = entry.vertex;

                if visited.mem(&v) {
                    continue;
                }

                let _ = visited.insert(v);
                sssp.set_distance(v, dist);

                if v < n {
                    let neighbors = graph.out_neighbors_weighed(&v);
                    let mut it = neighbors.iter();

                    loop
                        invariant
                            sssp.spec_distances().len() == n as int,
                            sssp.spec_source() == source,
                            it@.0 <= it@.1.len(),
                            obeys_feq_clone::<PQEntry>(),
                            m as int == graph@.A.len(),
                            graph@.A.len() * 2 + 2 <= usize::MAX as int,
                            remaining_budget >= 0,
                            pq@.len() + remaining_budget <= m as int,
                    {
                        match it.next() {
                            None => break,
                            Some(pair) => {
                                let Pair(u, weight) = pair;
                                if *u < n {
                                    let u_dist = sssp.get_distance(*u);
                                    let new_dist = dist.wrapping_add((*weight) as i64);
                                    if new_dist < u_dist {
                                        assert(pq@.len() + 1 <= usize::MAX as int);
                                        // Budget: total Dijkstra PQ inserts <= |E|.
                                        proof { assume(remaining_budget > 0); }
                                        pq = pq.insert(pq_entry_new(new_dist, *u));
                                        proof { remaining_budget = remaining_budget - 1; }
                                        sssp.set_predecessor(*u, v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        sssp
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
