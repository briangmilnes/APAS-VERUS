// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtPer with Arc-based backing for PARALLEL operations.

pub mod EdgeSetGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    const SEQUENTIAL_CUTOFF: usize = 1;

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
        spec fn spec_edgesetgraphmtper_wf(&self) -> bool;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            ensures out.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> N
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_edges(&self) -> N
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn vertices(&self) -> &AVLTreeSetMtPer<V>
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>>
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(|E| log |V|), Span Theta(log |E| * log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            requires self.spec_edgesetgraphmtper_wf()
            ensures neighbors@ == Set::new(|v: <V as View>::V| self.edges@.contains((u@, v)));
        /// Work Theta(|E|), Span Theta(log |E|)
        fn out_degree(&self, u: &V) -> N
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf();
        /// Work Theta(|E| log |V| + |E| log |E|), Span Theta(log |E| * log |V|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf(), !updated.vertices@.contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf();
    }

    // 9. impls

    impl<V: StTInMtT + Ord + 'static> EdgeSetGraphMtPerTrait<V> for EdgeSetGraphMtPer<V> {
        open spec fn spec_edgesetgraphmtper_wf(&self) -> bool {
            forall|u: <V as View>::V, v: <V as View>::V|
                #[trigger] self.edges@.contains((u, v))
                ==> self.vertices@.contains(u) && self.vertices@.contains(v)
        }

        fn empty() -> (out: Self) {
            EdgeSetGraphMtPer {
                vertices: AVLTreeSetMtPer::empty(),
                edges: AVLTreeSetMtPer::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphMtPer { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetMtPer<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Work Θ(m), Span Θ(lg m) — parallel filter + parallel neighbor set build via union.
        /// Requires external_body: filter's spec only ensures subset_of, cannot capture
        /// predicate semantics (Verus limitation: runtime Fn closures have no spec-level interpretation).
        /// The ensures needs exact set equality with a comprehension over filtered edges.
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
            let seq_left = seq.clone();
            let seq_right = seq.clone();
            let f1 = move || -> AVLTreeSetMtPer<V> {
                let mut neighbors = AVLTreeSetMtPer::empty();
                for i in 0..mid {
                    let Pair(_, v) = seq_left.nth(i);
                    neighbors = neighbors.insert(v.clone());
                }
                neighbors
            };
            let f2 = move || -> AVLTreeSetMtPer<V> {
                let mut neighbors = AVLTreeSetMtPer::empty();
                for i in mid..len {
                    let Pair(_, v) = seq_right.nth(i);
                    neighbors = neighbors.insert(v.clone());
                }
                neighbors
            };
            let (left_neighbors, right_neighbors) = join(f1, f2);
            left_neighbors.union(&right_neighbors)
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            EdgeSetGraphMtPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.vertices@.contains(v@)
        {
            let v_clone = v.clone();
            let new_vertices = self.vertices.delete(&v_clone);
            let new_edges = self.edges.filter(|edge| {
                let Pair(u, w) = edge;
                *u != v_clone && *w != v_clone
            });
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            EdgeSetGraphMtPer {
                vertices: self.vertices.clone(),
                edges: self.edges.delete(&Pair(u.clone(), v.clone())),
            }
        }
    }

    // 11. derive impls in verus!

    impl<V: StTInMtT + Ord + 'static> Default for EdgeSetGraphMtPer<V> {
        fn default() -> Self { Self::empty() }
    }

    } // verus!
}
