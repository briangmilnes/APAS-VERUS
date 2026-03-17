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
        spec fn spec_edgesetgraphstper_wf(&self) -> bool;
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            ensures out.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> (out: Self)
            requires
                v.spec_avltreesetstper_wf(),
                e.spec_avltreesetstper_wf(),
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> N
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_edges(&self) -> N
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn vertices(&self) -> &AVLTreeSetStPer<V>
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>>
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(|E| log |V|), Span Theta(|E| log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            requires self.spec_edgesetgraphstper_wf()
            ensures neighbors@ == self.spec_out_neighbors(u@);
        /// Work Theta(|E|), Span Theta(|E|)
        fn out_degree(&self, u: &V) -> N
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf();
        /// Work Theta(|E| log |E|), Span Theta(|E| log |E|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf(), !updated.vertices@.contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf();
    }

    // 9. impls

    impl<V: StT + Ord> EdgeSetGraphStPerTrait<V> for EdgeSetGraphStPer<V> {
        open spec fn spec_edgesetgraphstper_wf(&self) -> bool {
            self.vertices.spec_avltreesetstper_wf()
            && self.edges.spec_avltreesetstper_wf()
            && forall|u: <V as View>::V, v: <V as View>::V|
                #[trigger] self.edges@.contains((u, v))
                ==> self.vertices@.contains(u) && self.vertices@.contains(v)
        }

        open spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V> {
            Set::new(|v: <V as View>::V| self.edges@.contains((u, v)))
        }

        fn empty() -> (out: Self) {
            EdgeSetGraphStPer {
                vertices: AVLTreeSetStPer::empty(),
                edges: AVLTreeSetStPer::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphStPer { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStPer<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            ensures neighbors@ == self.spec_out_neighbors(u@)
        {
            let u_clone = u.clone();
            let filtered = self.edges.filter(
                |edge| edge.0 == u_clone,
                Ghost(|v: (V::V, V::V)| v.0 == u@),
            );
            let seq = filtered.to_seq();
            let ghost filtered_view = filtered@;
            let mut neighbors = AVLTreeSetStPer::empty();
            let n = seq.length();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < n
                invariant
                    n as int == seq@.len(),
                    i <= n,
                    neighbors.spec_avltreesetstper_wf(),
                    seq@.to_set() =~= filtered_view,
                    forall|p: (<V as View>::V, <V as View>::V)|
                        #[trigger] filtered_view.contains(p)
                        ==> self.edges@.contains(p) && p.0 == u@,
                    forall|p: (<V as View>::V, <V as View>::V)|
                        self.edges@.contains(p) && p.0 == u@
                        ==> #[trigger] filtered_view.contains(p),
                    forall|v: <V as View>::V| #[trigger] neighbors@.contains(v) ==>
                        self.edges@.contains((u@, v)),
                    forall|j: int| 0 <= j < i ==>
                        #[trigger] neighbors@.contains(seq@[j].1),
                decreases n - i
            {
                let Pair(_, v) = seq.nth(i).clone();
                proof {
                    assert(seq@.to_set().contains(seq@[i as int]));
                    assert(filtered_view.contains(seq@[i as int]));
                    assert(self.edges@.contains(seq@[i as int]));
                    assert(seq@[i as int].0 == u@);
                }
                neighbors = neighbors.insert(v);
                i += 1;
            }

            proof {
                assert forall|v: <V as View>::V|
                    self.spec_out_neighbors(u@).contains(v) implies
                    #[trigger] neighbors@.contains(v) by {
                    assert(self.edges@.contains((u@, v)));
                    assert(filtered_view.contains((u@, v)));
                    assert(seq@.to_set().contains((u@, v)));
                    let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == (u@, v);
                    assert(seq@[j].1 == v);
                }
                assert(neighbors@ =~= self.spec_out_neighbors(u@));
            }

            neighbors
        }

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1, degree of vertex]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            EdgeSetGraphStPer {
                vertices: self.vertices.insert(v),
                edges: self.edges.clone(),
            }
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.vertices@.contains(v@)
        {
            let v_clone = v.clone();
            let ghost v_view = v@;
            let new_vertices = self.vertices.delete(&v_clone);
            let new_edges = self.edges.filter(
                |edge| {
                    let Pair(u, w) = edge;
                    *u != v_clone && *w != v_clone
                },
                Ghost(|p: (V::V, V::V)| p.0 != v_view && p.1 != v_view),
            );
            proof {
                // Prove edge containment for the result.
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {
                    // new_edges ⊆ self.edges, and the filter keeps edges where neither endpoint is v.
                    assert(self.edges@.contains((a, b)));
                    assert(a != v_view && b != v_view);
                    // From old wf: a and b are in self.vertices.
                    assert(self.vertices@.contains(a));
                    assert(self.vertices@.contains(b));
                    // new_vertices = self.vertices.remove(v@). Since a != v@ and b != v@:
                    assert(new_vertices@.contains(a));
                    assert(new_vertices@.contains(b));
                }
            }
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            let new_vertices = self.vertices.insert(u.clone()).insert(v.clone());
            let new_edges = self.edges.insert(Pair(u, v));
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
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
