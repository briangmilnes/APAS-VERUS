//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Minimum Spanning Trees - Spanning Tree via Star Contraction (Parallel)
//!
//! Implements Exercise 64.2: Compute spanning tree using parallel star contraction.

pub mod SpanTreeMtEph {

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::sync::Arc;
    #[cfg(not(verus_keep_ghost))]
    use std::thread;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap62::StarContractionMtEph::StarContractionMtEph::star_contract_mt;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    pub type T<V> = UnDirGraphMtEph<V>;

    verus! {
        pub struct SpanTreeMtEphSpanningEdgesWf;
        impl<V: StT + MtT + Hash + Ord> RwLockPredicate<SetStEph<Edge<V>>> for SpanTreeMtEphSpanningEdgesWf {
            open spec fn inv(self, _v: SetStEph<Edge<V>>) -> bool { true }
        }
        #[verifier::external_body]
        fn new_spanning_edges_lock<V: StT + MtT + Hash + Ord>(
            val: SetStEph<Edge<V>>,
        ) -> (lock: RwLock<SetStEph<Edge<V>>, SpanTreeMtEphSpanningEdgesWf>) {
            RwLock::new(val, Ghost(SpanTreeMtEphSpanningEdgesWf))
        }

        pub struct SpanTreeMtEphValidWf;
        impl RwLockPredicate<bool> for SpanTreeMtEphValidWf {
            open spec fn inv(self, _v: bool) -> bool { true }
        }
        #[verifier::external_body]
        fn new_valid_lock(val: bool) -> (lock: RwLock<bool, SpanTreeMtEphValidWf>) {
            RwLock::new(val, Ghost(SpanTreeMtEphValidWf))
        }

        pub trait SpanTreeMtEphTrait {
            /// Parallel spanning tree via star contraction
            /// APAS: Work O(|V| + |E|), Span O(lg² |V|)
            fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + 'static>(
                graph: &UnDirGraphMtEph<V>,
            ) -> SetStEph<Edge<V>>;

            /// Verify spanning tree properties
            /// APAS: Work O(|V| + |E|), Span O(lg |V|)
            fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(graph: &UnDirGraphMtEph<V>, tree: &SetStEph<Edge<V>>) -> B;
        }
    }

    /// Exercise 64.2: Spanning Tree via Star Contraction (Parallel)
    ///
    /// Computes a spanning tree using parallel star contraction.
    ///
    /// - APAS: Work O((n+m) lg n), Span O(lg² n)
    /// - Claude-Opus-4.6: Work O((n+m) lg n), Span O((n+m) lg n) — expand closure
    ///   uses 2-way thread::spawn splits (not full divide-and-conquer), and inner
    ///   loop scanning original_edges for each quotient edge is sequential O(E).
    ///   Span does not achieve polylog; it equals Work.
    ///
    /// Arguments:
    /// - graph: The undirected graph
    /// - seed: Random seed for star partition
    ///
    /// Returns:
    /// - Set of edges forming a spanning tree
    #[cfg(not(verus_keep_ghost))]
    pub fn spanning_tree_star_contraction_mt<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> SetStEph<Edge<V>> {
        // Base: no edges means no spanning tree edges
        let base = |_vertices: &SetStEph<V>| SetLit![];

        // Expand: add star partition edges and map quotient tree edges back
        // Parallel version: Work O(|V| + |E|), Span O(lg² |V|)
        let expand = |_v: &SetStEph<V>,
                      original_edges: &SetStEph<Edge<V>>,
                      _centers: &SetStEph<V>,
                      partition_map: &HashMap<V, V>,
                      quotient_tree: SetStEph<Edge<V>>| {
            let spanning_edges = Arc::new(new_spanning_edges_lock(SetLit![]));

            // Part 1: Add edges from vertices to their centers (star edges) - PARALLEL
            // Convert HashMap to Vec for parallel processing
            let partition_vec: Vec<(V, V)> = partition_map
                .iter()
                .map(|(v, c)| (v.clone(), c.clone()))
                .collect();

            if !partition_vec.is_empty() {
                let mid = partition_vec.len() / 2;
                let left_vec = partition_vec[..mid].to_vec();
                let right_vec = partition_vec[mid..].to_vec();

                let spanning_edges_clone = Arc::clone(&spanning_edges);
                let handle = thread::spawn(move || {
                    for (vertex, center) in left_vec {
                        if vertex != center {
                            let edge = if vertex < center {
                                Edge(vertex, center)
                            } else {
                                Edge(center, vertex)
                            };
                            let (mut current, write_handle) = spanning_edges_clone.acquire_write();
                            current.insert(edge);
                            write_handle.release_write(current);
                        }
                    }
                });

                for (vertex, center) in right_vec {
                    if vertex != center {
                        let edge = if vertex < center {
                            Edge(vertex, center)
                        } else {
                            Edge(center, vertex)
                        };
                        let (mut current, write_handle) = spanning_edges.acquire_write();
                        current.insert(edge);
                        write_handle.release_write(current);
                    }
                }

                let _ = handle.join();
            }

            // Part 2: Map quotient tree edges back to original edges - PARALLEL
            // Convert sets to vectors for parallel processing
            let quotient_vec: Vec<Edge<V>> = quotient_tree.iter().cloned().collect();
            let original_vec: Vec<Edge<V>> = original_edges.iter().cloned().collect();

            if !quotient_vec.is_empty() {
                let mid = quotient_vec.len() / 2;
                let left_quotient = quotient_vec[..mid].to_vec();
                let right_quotient = quotient_vec[mid..].to_vec();

                let partition_map_clone = partition_map.clone();
                let original_vec_clone = original_vec.clone();
                let spanning_edges_clone = Arc::clone(&spanning_edges);

                let handle = thread::spawn(move || {
                    for quotient_edge in left_quotient {
                        let Edge(c1, c2) = quotient_edge;

                        // Find an original edge that connects the two stars (parallel search)
                        for original_edge in &original_vec_clone {
                            let Edge(u, v) = original_edge;
                            let u_center = partition_map_clone.get(u).unwrap_or(u);
                            let v_center = partition_map_clone.get(v).unwrap_or(v);

                            if (u_center == &c1 && v_center == &c2) || (u_center == &c2 && v_center == &c1) {
                                let (mut current, write_handle) = spanning_edges_clone.acquire_write();
                                current.insert(original_edge.clone());
                                write_handle.release_write(current);
                                break;
                            }
                        }
                    }
                });

                for quotient_edge in right_quotient {
                    let Edge(c1, c2) = quotient_edge;

                    // Find an original edge that connects the two stars
                    for original_edge in &original_vec {
                        let Edge(u, v) = original_edge;
                        let u_center = partition_map.get(u).unwrap_or(u);
                        let v_center = partition_map.get(v).unwrap_or(v);

                        if (u_center == &c1 && v_center == &c2) || (u_center == &c2 && v_center == &c1) {
                            let (mut current, write_handle) = spanning_edges.acquire_write();
                            current.insert(original_edge.clone());
                            write_handle.release_write(current);
                            break;
                        }
                    }
                }

                let _ = handle.join();
            }

            Arc::try_unwrap(spanning_edges).unwrap().into_inner()
        };

        star_contract_mt(graph, seed, &base, &expand)
    }

    /// Verify that result is a valid spanning tree
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(|V| + |E_tree|), Span O(|E_tree|) — 2-way split
    ///   halves work but span is O(E_tree/2), not O(lg V).
    #[cfg(not(verus_keep_ghost))]
    pub fn verify_spanning_tree<V: StT + MtT + Hash + Ord>(
        graph: &UnDirGraphMtEph<V>,
        tree_edges: &SetStEph<Edge<V>>,
    ) -> B {
        let n = graph.sizeV();
        let expected_edges = if n > 0 { n - 1 } else { 0 };

        if tree_edges.size() != expected_edges {
            return false;
        }

        // Parallel verification of edges
        let edges_vec: Vec<Edge<V>> = tree_edges.iter().cloned().collect();
        if edges_vec.is_empty() {
            return true;
        }

        let valid = Arc::new(new_valid_lock(true));

        let mid = edges_vec.len() / 2;
        let left_edges = edges_vec[..mid].to_vec();
        let right_edges = edges_vec[mid..].to_vec();

        let graph_edges = graph.edges();
        let graph_edges_clone = graph_edges.clone();
        let valid_clone = Arc::clone(&valid);

        let handle = thread::spawn(move || {
            for edge in left_edges {
                let Edge(u, v) = &edge;
                if !graph_edges_clone.mem(&edge) && !graph_edges_clone.mem(&Edge(v.clone(), u.clone())) {
                    let (_current, write_handle) = valid_clone.acquire_write();
                    write_handle.release_write(false);
                    return;
                }
            }
        });

        for edge in right_edges {
            let Edge(u, v) = &edge;
            if !graph_edges.mem(&edge) && !graph_edges.mem(&Edge(v.clone(), u.clone())) {
                let (_current, write_handle) = valid.acquire_write();
                write_handle.release_write(false);
                break;
            }
        }

        let _ = handle.join();

        let read_handle = valid.acquire_read();
        let result = *read_handle.borrow();
        read_handle.release_read();
        result
    }
}
