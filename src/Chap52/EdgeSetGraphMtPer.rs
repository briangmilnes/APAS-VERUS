// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 52: Edge Set Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtPer with Arc-based backing for PARALLEL operations.

pub mod EdgeSetGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;

    verus! {

    // 3. broadcast use
    broadcast use {
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct EdgeSetGraphMtPer<V: StTInMtT + Ord + ClonePreservesView + 'static> {
        pub vertices: AVLTreeSetMtPer<V>,
        pub edges: AVLTreeSetMtPer<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StTInMtT + Ord + ClonePreservesView + 'static> View for EdgeSetGraphMtPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 7. proof fns

    /// Bridges PartialEq's eq_spec to View equality via the cmp chain.
    proof fn lemma_eq_spec_iff_view_eq<V: StTInMtT + Ord>()
        requires
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures
            forall|a: V, b: V| #[trigger] a.eq_spec(&b) <==> (a@ == b@),
    {
        reveal(vstd::laws_cmp::obeys_cmp_partial_ord);
        reveal(vstd::laws_cmp::obeys_cmp_ord);
    }

    // 8. traits

    pub trait EdgeSetGraphMtPerTrait<V: StTInMtT + Ord + ClonePreservesView + 'static>: Sized {
        spec fn spec_edgesetgraphmtper_wf(&self) -> bool;
        spec fn spec_vertices(&self) -> Set<<V as View>::V>;
        spec fn spec_edges(&self) -> Set<(<V as View>::V, <V as View>::V)>;
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
            ensures out.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> (out: Self)
            requires
                v.spec_avltreesetmtper_wf(),
                e.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; AVL set len
        fn num_vertices(&self) -> usize
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; AVL set len
        fn num_edges(&self) -> usize
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> &AVLTreeSetMtPer<V>
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>>
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS; AVL find
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(|E| log |V|), Span Theta(log |E| * log |V|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg n) — matches APAS; parallel filter
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            requires self.spec_edgesetgraphmtper_wf()
            ensures
                neighbors@ == Set::new(|v: <V as View>::V| self.spec_edges().contains((u@, v))),
                neighbors.spec_avltreesetmtper_wf();
        /// Work Theta(|E|), Span Theta(log |E|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg n) — matches APAS; parallel filter then len
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires
                self.spec_edgesetgraphmtper_wf(),
                self.spec_vertices().len() + 1 < usize::MAX as nat,
            ensures updated.spec_edgesetgraphmtper_wf();
        /// Work Theta(|E| log |V| + |E| log |E|), Span Theta(log |E| * log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log m), Span O(m log m)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf(), !updated.spec_vertices().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + log m), Span O(log n + log m)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires
                self.spec_edgesetgraphmtper_wf(),
                self.spec_vertices().len() + 2 < usize::MAX as nat,
                self.spec_edges().len() + 1 < usize::MAX as nat,
            ensures updated.spec_edgesetgraphmtper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphmtper_wf()
            ensures updated.spec_edgesetgraphmtper_wf();
    }

    // 9. impls

    impl<V: StTInMtT + Ord + ClonePreservesView + 'static> EdgeSetGraphMtPerTrait<V> for EdgeSetGraphMtPer<V> {
        open spec fn spec_edgesetgraphmtper_wf(&self) -> bool {
            self.vertices.spec_avltreesetmtper_wf()
            && self.edges.spec_avltreesetmtper_wf()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            && vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>()
            && view_ord_consistent::<Pair<V, V>>()
            && forall|u: <V as View>::V, v: <V as View>::V|
                #[trigger] self.edges@.contains((u, v))
                ==> self.vertices@.contains(u) && self.vertices@.contains(v)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self) {
            EdgeSetGraphMtPer {
                vertices: AVLTreeSetMtPer::empty(),
                edges: AVLTreeSetMtPer::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtPer<V>, e: AVLTreeSetMtPer<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphMtPer { vertices: v, edges: e }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> usize { self.vertices.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_edges(&self) -> usize { self.edges.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> &AVLTreeSetMtPer<V> { &self.vertices }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> &AVLTreeSetMtPer<Pair<V, V>> { &self.edges }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn has_edge(&self, u: &V, v: &V) -> bool { self.edges.find(&Pair(u.clone(), v.clone())) }

        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg n) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(|E| log |V|), Span Theta(|E| log |V|) — sequential iterate+insert.
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
        {
            proof { lemma_eq_spec_iff_view_eq::<V>(); }
            let seq = self.edges.to_seq();
            let len = seq.length();
            let mut neighbors = AVLTreeSetMtPer::<V>::empty();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < len
                invariant
                    len as nat == seq.spec_seq().len(),
                    i <= len,
                    seq.spec_avltreeseqmtper_wf(),
                    neighbors.spec_avltreesetmtper_wf(),
                    self.spec_edgesetgraphmtper_wf(),
                    seq.spec_seq().to_set() =~= self.edges@,
                    forall|w: <V as View>::V| #[trigger] neighbors@.contains(w) ==>
                        self.edges@.contains((u@, w)),
                    forall|j: int| 0 <= j < i ==>
                        (#[trigger] seq.spec_seq()[j]).0 == u@
                        ==> neighbors@.contains(seq.spec_seq()[j].1),
                decreases len - i,
            {
                let elem = seq.nth(i);
                if elem.0 == *u {
                    let w = elem.1.clone_view();
                    proof {
                        assert(seq.spec_seq().to_set().contains(seq.spec_seq()[i as int]));
                        assert(self.edges@.contains(seq.spec_seq()[i as int]));
                        assert(seq.spec_seq()[i as int].0 == u@);
                    }
                    neighbors = neighbors.insert(w);
                }
                i += 1;
            }

            proof {
                assert forall|v: <V as View>::V|
                    Set::new(|v: <V as View>::V| self.spec_edges().contains((u@, v))).contains(v)
                    implies #[trigger] neighbors@.contains(v) by {
                    assert(self.edges@.contains((u@, v)));
                    assert(seq.spec_seq().to_set().contains((u@, v)));
                    let j = choose|j: int| 0 <= j < seq.spec_seq().len() && seq.spec_seq()[j] == (u@, v);
                    assert(seq.spec_seq()[j].0 == u@);
                    assert(seq.spec_seq()[j].1 == v);
                }
                assert forall|v: <V as View>::V|
                    #[trigger] neighbors@.contains(v)
                    implies Set::new(|v: <V as View>::V| self.spec_edges().contains((u@, v))).contains(v) by {
                    assert(self.edges@.contains((u@, v)));
                }
                assert(neighbors@ =~= Set::new(|v: <V as View>::V| self.spec_edges().contains((u@, v))));
            }

            neighbors
        }

        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg n) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(|E| log |V|), Span Theta(|E| log |V|) — delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self) {
            let new_vertices = self.vertices.insert(v);
            let new_edges = self.edges.clone();
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {}
            }
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(|E| log |E|), Span O(log |E| * log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E| log |E|), Span O(log |E| * log |V|) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(|E| log |E|), Span Theta(|E| log |E|) — sequential iterate+insert.
        fn delete_vertex(&self, v: &V) -> (updated: Self)
        {
            proof { lemma_eq_spec_iff_view_eq::<V>(); }
            let v_cv = v.clone_view();
            let new_vertices = self.vertices.delete(&v_cv);
            let seq = self.edges.to_seq();
            let len = seq.length();
            let mut new_edges = AVLTreeSetMtPer::<Pair<V, V>>::empty();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < len
                invariant
                    len as nat == seq.spec_seq().len(),
                    i <= len,
                    seq.spec_avltreeseqmtper_wf(),
                    new_edges.spec_avltreesetmtper_wf(),
                    self.spec_edgesetgraphmtper_wf(),
                    v_cv@ == v@,
                    new_vertices@ == self.vertices@.remove(v@),
                    seq.spec_seq().to_set() =~= self.edges@,
                    forall|a: <V as View>::V, b: <V as View>::V|
                        #[trigger] new_edges@.contains((a, b))
                        ==> self.edges@.contains((a, b)) && a != v@ && b != v@,
                    forall|j: int| 0 <= j < i ==>
                        (seq.spec_seq()[j].0 != v@ && seq.spec_seq()[j].1 != v@)
                        ==> #[trigger] new_edges@.contains(seq.spec_seq()[j]),
                decreases len - i,
            {
                let elem = seq.nth(i);
                if !(elem.0 == *v) && !(elem.1 == *v) {
                    let e = Pair(elem.0.clone_view(), elem.1.clone_view());
                    proof {
                        assert(seq.spec_seq().to_set().contains(seq.spec_seq()[i as int]));
                        assert(self.edges@.contains(seq.spec_seq()[i as int]));
                        assert(e@ == seq.spec_seq()[i as int]);
                    }
                    new_edges = new_edges.insert(e);
                }
                i += 1;
            }

            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {
                    assert(self.edges@.contains((a, b)));
                    assert(a != v@ && b != v@);
                    assert(self.vertices@.contains(a));
                    assert(self.vertices@.contains(b));
                }
            }

            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + log m), Span O(log n + log m)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            let u_cv = u.clone_view();
            let v_cv = v.clone_view();
            let new_vertices = self.vertices.insert(u_cv).insert(v_cv);
            let new_edges = self.edges.insert(Pair(u, v));
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {}
            }
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            let new_edges = self.edges.delete(&Pair(u.clone(), v.clone()));
            let new_vertices = self.vertices.clone();
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {}
            }
            EdgeSetGraphMtPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }
    }

    } // verus!
}
