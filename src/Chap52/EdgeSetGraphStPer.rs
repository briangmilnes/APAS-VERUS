// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Edge Set Graph representation (persistent, single-threaded).
//! G = (V, E) where V is a set of vertices and E ⊆ V × V is a set of edges.

pub mod EdgeSetGraphStPer {

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 4. type definitions

    #[derive(Clone, PartialEq, Eq)]
    pub struct EdgeSetGraphStPer<V: StT + Ord> {
        vertices: AVLTreeSetStPer<V>,
        edges: AVLTreeSetStPer<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for EdgeSetGraphStPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait EdgeSetGraphStPerTrait<V: StT + Ord> {
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                                            -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_edges(&self)                                                               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self)                                                                -> &AVLTreeSetStPer<V>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self)                                                                   -> &AVLTreeSetStPer<Pair<V, V>>;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                                                  -> B;
        /// claude-4-sonet: Work Θ(|E| log |V|), Span Θ(|E| log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V) -> (result: AVLTreeSetStPer<V>)
            ensures result@ == self.spec_out_neighbors(u@);
        /// claude-4-sonet: Work Θ(|E|), Span Θ(|E|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                                       -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)                                                     -> Self;
        /// claude-4-sonet: Work Θ(|E| log |E|), Span Θ(|E| log |E|), Parallelism Θ(1)
        fn delete_vertex(&self, v: &V) -> (result: Self)
            ensures !result.vertices@.contains(v@);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)                                                 -> Self;
        /// claude-4-sonet: Work Θ(log |E|), Span Θ(log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V)                                               -> Self;
    }

    // 9. impls

    impl<V: StT + Ord> EdgeSetGraphStPerTrait<V> for EdgeSetGraphStPer<V> {
        open spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V> {
            Set::new(|v: <V as View>::V| self.edges@.contains((u, v)))
        }

        fn empty() -> Self {
            EdgeSetGraphStPer {
                vertices: AVLTreeSetStPer::empty(),
                edges: AVLTreeSetStPer::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> Self {
            EdgeSetGraphStPer { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStPer<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> (result: AVLTreeSetStPer<V>)
            ensures result@ == self.spec_out_neighbors(u@)
        {
            let u_clone = u.clone();
            let filtered = self.edges.filter(|edge| edge.0 == u_clone);
            let seq = filtered.to_seq();
            let mut neighbors = AVLTreeSetStPer::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    i <= seq.length(),
                    neighbors@.finite(),
                    neighbors@ == seq.subrange(0, i as int).map(|p: (V::V, V::V)| p.1).to_set(),
                decreases seq.length() - i
            {
                let Pair(_, v) = seq.nth(i).clone();
                neighbors = neighbors.insert(v);
                i += 1;
            }
            neighbors
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1, degree of vertex]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> Self {
            EdgeSetGraphStPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        fn delete_vertex(&self, v: &V) -> (result: Self)
            ensures !result.vertices@.contains(v@)
        {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let new_edges = self.edges.filter(|edge| {
                let Pair(u, w) = edge;
                *u != v_clone && *w != v_clone
            });
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn insert_edge(&self, u: V, v: V) -> Self {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn delete_edge(&self, u: &V, v: &V) -> Self {
            let new_edges = self.edges.delete(&Pair(u.clone(), v.clone()));
            EdgeSetGraphStPer {
                vertices: self.vertices.clone(),
                edges: new_edges,
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<V: StT + Ord + fmt::Debug> fmt::Debug for EdgeSetGraphStPer<V> {
        /// - APAS: N/A — Rust Debug trait, not in textbook.
        /// - Claude-Opus-4.6: Work depends on graph size — outside verus!, not verified.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("EdgeSetGraphStPer")
                .field("vertices", &self.vertices)
                .field("edges", &self.edges)
                .finish()
        }
    }
}
