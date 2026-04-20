// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 52: Edge Set Graph representation (ephemeral, multi-threaded).
//! G = (V, A:) where V is a set of vertices and A: ⊆ V × V is a set of directed arcs.
//!
//! Uses AVLTreeSetMtEph with &mut self operations for in-place mutation.


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

pub mod EdgeSetGraphMtEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::accept::accept;

    verus!
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::group_avltreeseqsteph_len_bound,
};

    //		Section 7. proof fns/broadcast groups


    /// Bridges PartialEq's eq_spec to View equality via the cmp chain.
    proof fn lemma_eq_spec_iff_view_eq<V: StTInMtT + Ord + TotalOrder>()
        requires
            vstd::laws_cmp::obeys_cmp::<V>(),
            view_ord_consistent::<V>(),
        ensures
            forall|a: V, b: V| #[trigger] a.eq_spec(&b) <==> (a@ == b@),
    {
        reveal(vstd::laws_cmp::obeys_cmp_partial_ord);
        reveal(vstd::laws_cmp::obeys_cmp_ord);
    }

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct EdgeSetGraphMtEph<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static> {
        pub vertices: AVLTreeSetMtEph<V>,
        pub edges: AVLTreeSetMtEph<Pair<V, V>>,
    }

    //		Section 5. view impls


    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static> View for EdgeSetGraphMtEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    //		Section 8. traits


    pub trait EdgeSetGraphMtEphTrait<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static>: Sized {
        spec fn spec_edgesetgraphmteph_wf(&self) -> bool;
        spec fn spec_vertices(&self) -> Set<<V as View>::V>;
        spec fn spec_edges(&self) -> Set<(<V as View>::V, <V as View>::V)>;
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;

        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1) — creates empty sets.
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
            ensures out.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1) — wraps existing sets.
        fn from_vertices_and_edges(v: AVLTreeSetMtEph<V>, e: AVLTreeSetMtEph<Pair<V, V>>) -> (out: Self)
            requires
                v.spec_avltreesetmteph_wf(),
                e.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1); AVL set len.
        fn num_vertices(&self) -> usize
            requires self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1); AVL set len.
        fn num_edges(&self) -> usize
            requires self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1) — returns reference.
        fn vertices(&self) -> &AVLTreeSetMtEph<V>
            requires self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1) — returns reference.
        fn edges(&self) -> &AVLTreeSetMtEph<Pair<V, V>>
            requires self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg n), Span O(lg n); AVL find.
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(m)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m); sequential iterate+insert (filter closures not Send+Sync).
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtEph<V>)
            requires
                self.spec_edgesetgraphmteph_wf(),
                self.spec_edges().len() < usize::MAX as nat,
            ensures
                neighbors@ == self.spec_out_neighbors(u@),
                neighbors.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(m)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m); delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> usize
            requires
                self.spec_edgesetgraphmteph_wf(),
                self.spec_edges().len() < usize::MAX as nat;
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg n), Span O(lg n); AVL set insert.
        fn insert_vertex(&mut self, v: V)
            requires
                old(self).spec_edgesetgraphmteph_wf(),
                old(self).spec_vertices().len() + 1 < usize::MAX as nat,
            ensures self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m lg m), Span O(m lg m)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m); iterate + delete edges touching v.
        fn delete_vertex(&mut self, v: &V)
            requires
                old(self).spec_edgesetgraphmteph_wf(),
                old(self).spec_vertices().len() < usize::MAX as nat,
                old(self).spec_edges().len() < usize::MAX as nat,
            ensures self.spec_edgesetgraphmteph_wf(), !self.spec_vertices().contains(v@);
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n + lg m), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg n + lg m), Span O(lg n + lg m); vertex inserts + edge insert.
        fn insert_edge(&mut self, u: V, v: V)
            requires
                old(self).spec_edgesetgraphmteph_wf(),
                old(self).spec_vertices().len() + 2 < usize::MAX as nat,
                old(self).spec_edges().len() + 1 < usize::MAX as nat,
            ensures self.spec_edgesetgraphmteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg m), Span O(lg m)
        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg m), Span O(lg m); AVL set delete.
        fn delete_edge(&mut self, u: &V, v: &V)
            requires
                old(self).spec_edgesetgraphmteph_wf(),
                old(self).spec_edges().len() < usize::MAX as nat,
            ensures self.spec_edgesetgraphmteph_wf();
    }

    //		Section 9. impls


    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static> EdgeSetGraphMtEphTrait<V> for EdgeSetGraphMtEph<V> {
        open spec fn spec_edgesetgraphmteph_wf(&self) -> bool {
            self.vertices.spec_avltreesetmteph_wf()
            && self.edges.spec_avltreesetmteph_wf()
            && vstd::laws_cmp::obeys_cmp::<V>()
            && view_ord_consistent::<V>()
            && vstd::laws_cmp::obeys_cmp::<Pair<V, V>>()
            && view_ord_consistent::<Pair<V, V>>()
            && forall|u: <V as View>::V, v: <V as View>::V|
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

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self) {
            EdgeSetGraphMtEph {
                vertices: AVLTreeSetMtEph::empty(),
                edges: AVLTreeSetMtEph::empty(),
            }
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn from_vertices_and_edges(v: AVLTreeSetMtEph<V>, e: AVLTreeSetMtEph<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphMtEph { vertices: v, edges: e }
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> usize { self.vertices.size() }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn num_edges(&self) -> usize { self.edges.size() }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> &AVLTreeSetMtEph<V> { &self.vertices }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(1), Span O(1)
        fn edges(&self) -> &AVLTreeSetMtEph<Pair<V, V>> { &self.edges }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg m), Span O(lg m)
        fn has_edge(&self, u: &V, v: &V) -> bool { self.edges.find(&Pair(u.clone_view(), v.clone_view())) }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtEph<V>)
            ensures neighbors@ == self.spec_out_neighbors(u@)
        {
            proof { lemma_eq_spec_iff_view_eq::<V>(); }
            let seq = self.edges.to_seq();
            let len = seq.length();
            let ghost seq_view = seq@;
            let mut neighbors = AVLTreeSetMtEph::<V>::empty();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < len
                invariant
                    len as nat == seq@.len(),
                    i <= len,
                    neighbors@.len() <= i as nat,
                    seq.spec_avltreeseqsteph_wf(),
                    neighbors.spec_avltreesetmteph_wf(),
                    self.spec_edgesetgraphmteph_wf(),
                    self.spec_edges().len() < usize::MAX as nat,
                    seq_view =~= seq@,
                    seq@.to_set() =~= self.edges@,
                    forall|w: <V as View>::V| #[trigger] neighbors@.contains(w) ==>
                        self.edges@.contains((u@, w)),
                    forall|j: int| 0 <= j < i ==>
                        (#[trigger] seq@[j]).0 == u@
                        ==> neighbors@.contains(seq@[j].1),
                decreases len - i,
            {
                let elem = seq.nth(i);
                if elem.0 == *u {
                    let w = elem.1.clone_view();
                    proof {
                        assert(seq@[i as int].0 == u@);
                        assert(seq@.to_set().contains(seq@[i as int]));
                        assert(self.edges@.contains(seq@[i as int]));
                        // group_avltreeseqsteph_len_bound fires on seq.spec_avltreeseqsteph_wf()
                        // giving seq@.len() < usize::MAX as nat, and neighbors@.len() <= i < seq@.len()
                        // so neighbors@.len() + 1 <= seq@.len() < usize::MAX as nat
                        assert(neighbors@.len() + 1 <= seq@.len()) by {
                            assert(neighbors@.len() <= i as nat);
                            assert(i as nat + 1 <= seq@.len()) by {
                                assert(i < len);
                            };
                        };
                        assert(neighbors@.len() + 1 < usize::MAX as nat);
                        assert(vstd::laws_cmp::obeys_cmp::<V>());
                        assert(view_ord_consistent::<V>());
                    }
                    neighbors.insert(w);
                }
                i += 1;
            }

            proof {
                assert forall|v: <V as View>::V|
                    self.spec_out_neighbors(u@).contains(v) implies
                    #[trigger] neighbors@.contains(v) by {
                    assert(self.edges@.contains((u@, v)));
                    assert(seq@.to_set().contains((u@, v)));
                    let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == (u@, v);
                    assert(seq@[j].0 == u@);
                    assert(seq@[j].1 == v);
                }
                assert(neighbors@ =~= self.spec_out_neighbors(u@));
            }

            neighbors
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m)
        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg n), Span O(lg n)
        fn insert_vertex(&mut self, v: V) {
            self.vertices.insert(v);
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    assert(old(self).spec_edges().contains((a, b)));
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(m lg m), Span O(m lg m)
        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_vertices().contains(v@)
        {
            let ghost v_view = v@;
            let v_clone = v.clone_view();
            self.vertices.delete(&v_clone);
            let seq = self.edges.to_seq();
            let seq_len = seq.length();
            let mut i: usize = 0;

            proof { lemma_eq_spec_iff_view_eq::<V>(); }

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < seq_len
                invariant
                    i <= seq_len,
                    seq_len as nat == seq@.len(),
                    seq.spec_avltreeseqsteph_wf(),
                    self.edges.spec_avltreesetmteph_wf(),
                    self.edges@.len() <= old(self).spec_edges().len(),
                    vstd::laws_cmp::obeys_cmp::<V>(),
                    view_ord_consistent::<V>(),
                    vstd::laws_cmp::obeys_cmp::<Pair<V, V>>(),
                    view_ord_consistent::<Pair<V, V>>(),
                    self.edges@.subset_of(old(self).edges@),
                    !self.spec_vertices().contains(v_view),
                    self.vertices.spec_avltreesetmteph_wf(),
                    self.vertices@ =~= old(self).vertices@.remove(v_view),
                    old(self).spec_edgesetgraphmteph_wf(),
                    // All edges touching v from seq[0..i] have been deleted.
                    forall|j: int| 0 <= j < i && (seq@[j].0 == v_view || seq@[j].1 == v_view) ==>
                        !self.edges@.contains(#[trigger] seq@[j]),
                decreases seq_len - i
            {
                let edge_ref = seq.nth(i);
                if edge_ref.0 == *v || edge_ref.1 == *v {
                    proof {
                        assert(self.edges@.len() < usize::MAX as nat);
                    }
                    self.edges.delete(edge_ref);
                }
                i += 1;
            }
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    assert(old(self).spec_edges().contains((a, b)));
                    assert(old(self).spec_vertices().contains(a));
                    assert(old(self).spec_vertices().contains(b));
                    assert(a != v_view && b != v_view);
                    assert(self.vertices@ =~= old(self).vertices@.remove(v_view));
                    assert(self.vertices@.contains(b));
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg n + lg m), Span O(lg n + lg m)
        fn insert_edge(&mut self, u: V, v: V) {
            let u_cv = u.clone_view();
            let v_cv = v.clone_view();
            self.vertices.insert(u_cv);
            self.vertices.insert(v_cv);
            self.edges.insert(Pair(u, v));
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    if !old(self).spec_edges().contains((a, b)) {
                        // The new edge: (a, b) == (u@, v@), both inserted into vertices.
                    } else {
                        assert(old(self).spec_edges().contains((a, b)));
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Sonnet 4.6): Work O(lg m), Span O(lg m)
        fn delete_edge(&mut self, u: &V, v: &V) {
            self.edges.delete(&Pair(u.clone_view(), v.clone_view()));
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    assert(old(self).spec_edges().contains((a, b)));
                }
            }
        }
    }

    //		Section 12. derive impls in verus!


    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static> Default for EdgeSetGraphMtEph<V> {
        fn default() -> Self {
            EdgeSetGraphMtEph {
                vertices: AVLTreeSetMtEph::default(),
                edges: AVLTreeSetMtEph::default(),
            }
        }
    }

    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + 'static> Clone for EdgeSetGraphMtEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = EdgeSetGraphMtEph {
                vertices: self.vertices.clone(),
                edges: self.edges.clone(),
            };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + std::fmt::Debug + 'static> std::fmt::Debug for EdgeSetGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("EdgeSetGraphMtEph")
                .field("vertices", &self.vertices)
                .field("edges", &self.edges)
                .finish()
        }
    }

    impl<V: StTInMtT + Ord + TotalOrder + ClonePreservesView + std::fmt::Display + 'static> std::fmt::Display for EdgeSetGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EdgeSetGraphMtEph(vertices: {}, edges: {})", self.vertices.size(), self.edges.size())
        }
    }
}
