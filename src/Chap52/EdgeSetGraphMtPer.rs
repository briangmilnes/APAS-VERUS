// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtPer with Arc-based backing for PARALLEL operations.

pub mod EdgeSetGraphMtPer {

    use std::thread;

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    const SEQUENTIAL_CUTOFF: N = 64;

    verus! {

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    pub struct EdgeSetGraphMtPer<V: StTInMtT + Ord + 'static> {
        vertices: AVLTreeSetMtPer<V>,
        edges: AVLTreeSetMtPer<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StTInMtT + Ord + 'static> View for EdgeSetGraphMtPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait EdgeSetGraphMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_edges(&self)                                                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                                                -> &AVLTreeSetMtPer<V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                                                   -> &AVLTreeSetMtPer<Pair<V, V>>;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                                                  -> B;
        /// claude-4-sonet: Work Θ(|E| log |V|), Span Θ(log |E| × log |V|), Parallelism Θ(|E|/log |E|)
        fn out_neighbors(&self, u: &V)                                                    -> AVLTreeSetMtPer<V>;
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|), Parallelism Θ(|E|/log |E|)
        fn out_degree(&self, u: &V)                                                       -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)                                                     -> Self;
        /// claude-4-sonet: Work Θ(|E| log |V| + |E| log |E|), Span Θ(log |E| × log |V|), Parallelism Θ(|E|/log |E|)
        fn delete_vertex(&self, v: &V)                                                    -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)                                                 -> Self;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V)                                               -> Self;
    }

    // 9. impls

    impl<V: StTInMtT + Ord + 'static> EdgeSetGraphMtPerTrait<V> for EdgeSetGraphMtPer<V> {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — creates two empty AVL sets.
        #[verifier::external_body]
        fn empty() -> Self {
            EdgeSetGraphMtPer {
                vertices: AVLTreeSetMtPer::empty(),
                edges: AVLTreeSetMtPer::empty(),
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing sets.
        #[verifier::external_body]
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> Self {
            EdgeSetGraphMtPer { vertices: v, edges: e }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to AVL tree size.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.vertices.size() }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to AVL tree size.
        #[verifier::external_body]
        fn num_edges(&self) -> N { self.edges.size() }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — returns reference.
        #[verifier::external_body]
        fn vertices(&self) -> &AVLTreeSetMtPer<V> { &self.vertices }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — returns reference.
        #[verifier::external_body]
        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>> { &self.edges }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(lg m), Span Θ(lg m) — agrees with APAS.
        #[verifier::external_body]
        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(lg m) — parallel filter + parallel neighbor set build via union.
        #[verifier::external_body]
        fn out_neighbors(&self, u: &V) -> AVLTreeSetMtPer<V> {
            let u_clone = u.clone();
            let filtered = self.edges.filter(move |edge| edge.0 == u_clone);
            let seq = filtered.to_seq();
            let len = seq.length();
            if len <= SEQUENTIAL_CUTOFF {
                let mut neighbors = AVLTreeSetMtPer::empty();
                for i in 0..len {
                    let Pair(_, v) = seq.nth(i);
                    neighbors = neighbors.insert(v.clone());
                }
                return neighbors;
            }
            let mid = len / 2;
            let seq_clone = seq.clone();
            let left_handle = thread::spawn(move || {
                let mut neighbors = AVLTreeSetMtPer::empty();
                for i in 0..mid {
                    let Pair(_, v) = seq_clone.nth(i);
                    neighbors = neighbors.insert(v.clone());
                }
                neighbors
            });
            let mut right_neighbors = AVLTreeSetMtPer::empty();
            for i in mid..len {
                let Pair(_, v) = seq.nth(i);
                right_neighbors = right_neighbors.insert(v.clone());
            }
            let left_neighbors = left_handle.join().unwrap();
            left_neighbors.union(&right_neighbors)
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors.
        #[verifier::external_body]
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_vertex(&self, v: V) -> Self {
            EdgeSetGraphMtPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1, isolated vertex]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(lg m) — parallel filter over edges; APAS assumes isolated.
        #[verifier::external_body]
        fn delete_vertex(&self, v: &V) -> Self {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let v_clone2 = v_clone.clone();
            let new_edges = self.edges.filter(move |edge| {
                let Pair(u, w) = edge;
                u != &v_clone2 && w != &v_clone2
            });
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_edge(&self, u: V, v: V) -> Self {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            EdgeSetGraphMtPer {
                vertices: self.vertices.clone(),
                edges: self.edges.delete(&Pair(u.clone(), v.clone())),
            }
        }
    }

    // 11. derive impls in verus!

    impl<V: StTInMtT + Ord + 'static> Default for EdgeSetGraphMtPer<V> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to empty(); external_body, not verified.
        #[verifier::external_body]
        fn default() -> Self { Self::empty() }
    }

    } // verus!
}
