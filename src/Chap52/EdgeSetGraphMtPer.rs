// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtPer with Arc-based backing for PARALLEL operations.

pub mod EdgeSetGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // 3. broadcast use
    broadcast use {
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
                v.spec_avltreesetmtper_wf(),
                e.spec_avltreesetmtper_wf(),
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
            self.vertices.spec_avltreesetmtper_wf()
            && self.edges.spec_avltreesetmtper_wf()
            && forall|u: <V as View>::V, v: <V as View>::V|
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
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — filter edges, iterate filtered seq, insert second components.
        fn out_neighbors(&self, u: &V) -> AVLTreeSetMtPer<V> {
            let ghost u_view = u@;
            let u_clone = u.clone();
            let filtered = self.edges.filter(
                move |edge| {
                    let Pair(eu, _) = edge;
                    *eu == u_clone
                },
                Ghost(|v: (V::V, V::V)| v.0 == u_view),
            );
            let seq = filtered.to_seq();
            let ghost filtered_view = filtered@;
            let len = seq.length();
            let mut neighbors = AVLTreeSetMtPer::<V>::empty();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < len
                invariant
                    seq.spec_avltreeseqmtper_wf(),
                    len as int == seq@.len(),
                    i <= len,
                    neighbors.spec_avltreesetmtper_wf(),
                    seq@.to_set() =~= filtered_view,
                    forall|p: (<V as View>::V, <V as View>::V)|
                        #[trigger] filtered_view.contains(p)
                        ==> self.edges@.contains(p) && p.0 == u_view,
                    forall|p: (<V as View>::V, <V as View>::V)|
                        self.edges@.contains(p) && p.0 == u_view
                        ==> #[trigger] filtered_view.contains(p),
                    forall|v: <V as View>::V| #[trigger] neighbors@.contains(v) ==>
                        self.edges@.contains((u_view, v)),
                    forall|j: int| 0 <= j < i ==>
                        #[trigger] neighbors@.contains(seq@[j].1),
                decreases len - i,
            {
                let elem = seq.nth(i);
                let Pair(_, v) = elem;
                proof {
                    assert(seq@.to_set().contains(seq@[i as int]));
                    assert(filtered_view.contains(seq@[i as int]));
                    assert(self.edges@.contains(seq@[i as int]));
                    assert(seq@[i as int].0 == u_view);
                }
                neighbors = neighbors.insert(v.clone());
                i += 1;
            }

            proof {
                assert forall|v: <V as View>::V|
                    Set::new(|v: <V as View>::V| self.edges@.contains((u_view, v))).contains(v) implies
                    #[trigger] neighbors@.contains(v) by {
                    assert(self.edges@.contains((u_view, v)));
                    assert(filtered_view.contains((u_view, v)));
                    assert(seq@.to_set().contains((u_view, v)));
                    let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == (u_view, v);
                    assert(seq@[j].1 == v);
                }
                assert(neighbors@ =~= Set::new(|v: <V as View>::V| self.edges@.contains((u@, v))));
            }

            neighbors
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
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {
                    assert(self.edges@.contains((a, b)));
                    assert(a != v_view && b != v_view);
                    assert(self.vertices@.contains(a));
                    assert(self.vertices@.contains(b));
                    assert(new_vertices@.contains(a));
                    assert(new_vertices@.contains(b));
                }
            }
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
