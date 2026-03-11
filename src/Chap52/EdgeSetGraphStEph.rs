// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Edge Set Graph representation (ephemeral, single-threaded).

pub mod EdgeSetGraphStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
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

    // 4. type definitions

    #[derive(Clone)]
    pub struct EdgeSetGraphStEph<V: StT + Ord> {
        vertices: AVLTreeSetStEph<V>,
        edges: AVLTreeSetStEph<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for EdgeSetGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait EdgeSetGraphStEphTrait<V: StT + Ord> {
        spec fn spec_edgesetgraphsteph_wf(&self) -> bool;
        spec fn spec_vertices(&self) -> Set<<V as View>::V>;
        spec fn spec_edges(&self) -> Set<(<V as View>::V, <V as View>::V)>;
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            ensures out.spec_edgesetgraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> N
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_edges(&self) -> N
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn vertices(&self) -> &AVLTreeSetStEph<V>
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>>
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(|E| log |V|), Span Theta(|E| log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            requires self.spec_edgesetgraphsteph_wf()
            ensures neighbors@ == self.spec_out_neighbors(u@);
        /// Work Theta(|E|), Span Theta(|E|)
        fn out_degree(&self, u: &V) -> N
            requires self.spec_edgesetgraphsteph_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&mut self, v: V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf();
        /// Work Theta(|E| log |E|), Span Theta(|E| log |E|)
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf(), !self.spec_vertices().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&mut self, u: V, v: V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn delete_edge(&mut self, u: &V, v: &V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf();
    }

    // 9. impls

    impl<V: StT + Ord> EdgeSetGraphStEphTrait<V> for EdgeSetGraphStEph<V> {
        open spec fn spec_edgesetgraphsteph_wf(&self) -> bool {
            forall|u: <V as View>::V, v: <V as View>::V|
                #[trigger] self.spec_edges().contains((u, v))
                ==> self.spec_vertices().contains(u) && self.spec_vertices().contains(v)
        }

        open spec fn spec_vertices(&self) -> Set<<V as View>::V> {
            self.vertices@
        }

        open spec fn spec_edges(&self) -> Set<(<V as View>::V, <V as View>::V)> {
            self.edges@
        }

        open spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V> {
            Set::new(|v: <V as View>::V| self.edges@.contains((u, v)))
        }

        fn empty() -> (out: Self) {
            EdgeSetGraphStEph {
                vertices: AVLTreeSetStEph::empty(),
                edges: AVLTreeSetStEph::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphStEph { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStEph<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            ensures neighbors@ == self.spec_out_neighbors(u@)
        {
            let u_clone = u.clone();
            let filtered = self.edges.filter(|edge| edge.0 == u_clone);
            let seq = filtered.to_seq();
            let mut neighbors = AVLTreeSetStEph::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    i <= seq.length(),
                    neighbors@.finite(),
                    neighbors@ == seq.subrange(0, i as int).map(|p: (V::V, V::V)| p.1).to_set(),
                decreases seq.length() - i
            {
                let Pair(_, v) = seq.nth(i).clone();
                neighbors.insert(v);
                i += 1;
            }
            neighbors
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_vertices().contains(v@)
        {
            let v_clone = v.clone();
            self.vertices.delete(&v_clone);
            let seq = self.edges.to_seq();
            let mut to_remove: Vec<Pair<V, V>> = Vec::new();
            let mut i: usize = 0;
            while i < seq.length()
                invariant i <= seq.length()
                decreases seq.length() - i
            {
                let edge = seq.nth(i).clone();
                let Pair(u, w) = edge;
                if u == v_clone || w == v_clone {
                    to_remove.push(edge);
                }
                i += 1;
            }
            let mut j: usize = 0;
            while j < to_remove.len()
                invariant j <= to_remove.len(), !self.spec_vertices().contains(v@)
                decreases to_remove.len() - j
            {
                self.edges.delete(&to_remove[j]);
                j += 1;
            }
        }

        fn insert_edge(&mut self, u: V, v: V) {
            self.vertices.insert(u.clone());
            self.vertices.insert(v.clone());
            self.edges.insert(Pair(u, v));
        }

        fn delete_edge(&mut self, u: &V, v: &V) { self.edges.delete(&Pair(u.clone(), v.clone())); }
    }

    } // verus!
}
