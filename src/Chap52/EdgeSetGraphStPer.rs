//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 52: Edge Set Graph representation (persistent, single-threaded).
//! G = (V, E) where V is a set of vertices and E ⊆ V × V is a set of edges.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 7. proof fns/broadcast groups
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod EdgeSetGraphStPer {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus!
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    //		Section 7. proof fns/broadcast groups


    /// Bridges PartialEq's eq_spec to View equality via the cmp chain.
    proof fn lemma_eq_spec_iff_view_eq<V: StT + Ord + TotalOrder>()
        requires
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures
            forall|a: V, b: V| #[trigger] a.eq_spec(&b) <==> (a@ == b@),
    {
        reveal(vstd::laws_cmp::obeys_cmp_partial_ord);
        reveal(vstd::laws_cmp::obeys_cmp_ord);
    }

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct EdgeSetGraphStPer<V: StT + Ord + TotalOrder + ClonePreservesView> {
        pub vertices: AVLTreeSetStPer<V>,
        pub edges: AVLTreeSetStPer<Pair<V, V>>,
    }

    //		Section 5. view impls


    impl<V: StT + Ord + TotalOrder + ClonePreservesView> View for EdgeSetGraphStPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    //		Section 8. traits


    pub trait EdgeSetGraphStPerTrait<V: StT + Ord + TotalOrder + ClonePreservesView>: Sized {
        spec fn spec_edgesetgraphstper_wf(&self) -> bool;
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
            ensures out.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> (out: Self)
            requires
                v.spec_avltreesetstper_wf(),
                e.spec_avltreesetstper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); AVL set len
        fn num_vertices(&self) -> usize
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); AVL set len
        fn num_edges(&self) -> usize
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> &AVLTreeSetStPer<V>
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>>
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); AVL find
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(|E| log |V|), Span Theta(|E| log |V|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(m), Span O(m) work; sequential filter
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            requires self.spec_edgesetgraphstper_wf()
            ensures neighbors@ == self.spec_out_neighbors(u@), neighbors.spec_avltreesetstper_wf();
        /// Work Theta(|E|), Span Theta(|E|)
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(m), Span O(m) work; sequential filter then len
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_edgesetgraphstper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires
                self.spec_edgesetgraphstper_wf(),
                self.spec_vertices().len() + 1 < usize::MAX as nat,
            ensures updated.spec_edgesetgraphstper_wf();
        /// Work Theta(|E| log |E|), Span Theta(|E| log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log m), Span O(m log m)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf(), !updated.spec_vertices().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + log m), Span O(log n + log m)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires
                self.spec_edgesetgraphstper_wf(),
                self.spec_vertices().len() + 2 < usize::MAX as nat,
                self.spec_edges().len() + 1 < usize::MAX as nat,
            ensures updated.spec_edgesetgraphstper_wf();
        /// Work Theta(log |E|), Span Theta(log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_edgesetgraphstper_wf()
            ensures updated.spec_edgesetgraphstper_wf();
    }

    //		Section 9. impls


    impl<V: StT + Ord + TotalOrder + ClonePreservesView> EdgeSetGraphStPerTrait<V> for EdgeSetGraphStPer<V> {
        open spec fn spec_edgesetgraphstper_wf(&self) -> bool {
            self.vertices.spec_avltreesetstper_wf()
            && self.edges.spec_avltreesetstper_wf()
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
            EdgeSetGraphStPer {
                vertices: AVLTreeSetStPer::empty(),
                edges: AVLTreeSetStPer::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vertices_and_edges(v: AVLTreeSetStPer<V>, e: AVLTreeSetStPer<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphStPer { vertices: v, edges: e }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> usize { self.vertices.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_edges(&self) -> usize { self.edges.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> &AVLTreeSetStPer<V> { &self.vertices }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> &AVLTreeSetStPer<Pair<V, V>> { &self.edges }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn has_edge(&self, u: &V, v: &V) -> bool { self.edges.find(&Pair(u.clone(), v.clone())) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            ensures neighbors@ == self.spec_out_neighbors(u@)
        {
            proof { lemma_eq_spec_iff_view_eq::<V>(); }
            let pred = |edge: &Pair<V, V>| -> (keep: bool)
                ensures keep == (edge@.0 == u@)
            {
                edge.0 == *u
            };
            let filtered = self.edges.filter(
                pred,
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
                    neighbors@.len() <= i as nat,
                    filtered.spec_avltreesetstper_wf(),
                    seq@.to_set() =~= filtered_view,
                    self.spec_edgesetgraphstper_wf(),
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
                let pair_ref = seq.nth(i);
                let v = pair_ref.1.clone_view();
                proof {
                    assert(seq@.to_set().contains(seq@[i as int]));
                    assert(filtered_view.contains(seq@[i as int]));
                    assert(self.spec_edges().contains(seq@[i as int]));
                    assert(seq@[i as int].0 == u@);
                    assert(self.spec_vertices().contains(v@));
                }
                if !neighbors.find(&v) {
                    proof {
                        assert forall|w: <V as View>::V|
                            #[trigger] neighbors@.insert(v@).contains(w)
                            implies self.spec_vertices().contains(w) by {
                            if w != v@ {
                                assert(neighbors@.contains(w));
                                assert(self.spec_edges().contains((u@, w)));
                            }
                        }
                        vstd::set_lib::lemma_len_subset(neighbors@.insert(v@), self.spec_vertices());
                    }
                    neighbors = neighbors.insert(v);
                }
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

        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self) {
            let new_vertices = self.vertices.insert(v);
            let new_edges = self.edges.clone();
            proof {
                // Edges unchanged; vertices grew. Edge invariant preserved.
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {}
            }
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log m), Span O(m log m)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.vertices@.contains(v@)
        {
            let v_clone = v.clone_view();
            let new_vertices = self.vertices.delete(&v_clone);
            proof { lemma_eq_spec_iff_view_eq::<V>(); }
            let pred = |edge: &Pair<V, V>| -> (keep: bool)
                ensures keep == (edge@.0 != v@ && edge@.1 != v@)
            {
                let Pair(u, w) = edge;
                *u != *v && *w != *v
            };
            let new_edges = self.edges.filter(
                pred,
                Ghost(|p: (V::V, V::V)| p.0 != v@ && p.1 != v@),
            );
            proof {
                // Prove edge containment for the result.
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {
                    assert(self.edges@.contains((a, b)));
                    assert(a != v@ && b != v@);
                    assert(self.vertices@.contains(a));
                    assert(self.vertices@.contains(b));
                    assert(new_vertices@.contains(a));
                    assert(new_vertices@.contains(b));
                }
            }
            EdgeSetGraphStPer {
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
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {
                    // New edge is (u@, v@); vertices now contain u@ and v@.
                    // Old edges: endpoints were in old vertices, which are in new vertices.
                }
            }
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log m), Span O(log m)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            let new_edges = self.edges.delete(&Pair(u.clone(), v.clone()));
            let new_vertices = self.vertices.clone();
            proof {
                // Edges only shrank; vertices unchanged. Edge invariant preserved.
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] new_edges@.contains((a, b))
                    implies new_vertices@.contains(a) && new_vertices@.contains(b) by {}
            }
            EdgeSetGraphStPer {
                vertices: new_vertices,
                edges: new_edges,
            }
        }
    }

    //		Section 12. derive impls in verus!


    impl<V: StT + Ord + TotalOrder + ClonePreservesView> Clone for EdgeSetGraphStPer<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = EdgeSetGraphStPer {
                vertices: self.vertices.clone(),
                edges: self.edges.clone(),
            };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<V: StT + Ord + TotalOrder + ClonePreservesView + PartialEq> PartialEqSpecImpl for EdgeSetGraphStPer<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<V: StT + Ord + TotalOrder + ClonePreservesView + PartialEq> PartialEq for EdgeSetGraphStPer<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@),
        {
            let equal = self.vertices == other.vertices && self.edges == other.edges;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<V: StT + Ord + TotalOrder + ClonePreservesView + Eq> Eq for EdgeSetGraphStPer<V> {}

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<V: StT + Ord + TotalOrder + ClonePreservesView + fmt::Debug> fmt::Debug for EdgeSetGraphStPer<V> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work depends on graph size — outside verus!, not verified.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("EdgeSetGraphStPer")
                .field("vertices", &self.vertices)
                .field("edges", &self.edges)
                .finish()
        }
    }

    impl<V: StT + Ord + TotalOrder + ClonePreservesView + fmt::Display> fmt::Display for EdgeSetGraphStPer<V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "EdgeSetGraphStPer(vertices: {}, edges: {})", self.vertices.size(), self.edges.size())
        }
    }
}
