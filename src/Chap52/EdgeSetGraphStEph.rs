// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Edge Set Graph representation (ephemeral, single-threaded).

pub mod EdgeSetGraphStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

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
    // 7. proof fns
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 7. proof fns

    /// Bridges PartialEq's eq_spec to View equality via the cmp chain.
    proof fn lemma_eq_spec_iff_view_eq<V: StT + Ord>()
        requires
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures
            forall|a: V, b: V| #[trigger] a.eq_spec(&b) <==> (a@ == b@),
    {
        reveal(vstd::laws_cmp::obeys_cmp_partial_ord);
        reveal(vstd::laws_cmp::obeys_cmp_ord);
    }

    // 4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct EdgeSetGraphStEph<V: StT + Ord + ClonePreservesView> {
        pub vertices: AVLTreeSetStEph<V>,
        pub edges: AVLTreeSetStEph<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StT + Ord + ClonePreservesView> View for EdgeSetGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait EdgeSetGraphStEphTrait<V: StT + Ord + ClonePreservesView>: Sized {
        spec fn spec_edgesetgraphsteph_wf(&self) -> bool;
        spec fn spec_vertices(&self) -> Set<<V as View>::V>;
        spec fn spec_edges(&self) -> Set<(<V as View>::V, <V as View>::V)>;
        spec fn spec_out_neighbors(&self, u: <V as View>::V) -> Set<<V as View>::V>;

        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; creates empty sets.
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
            ensures out.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1)
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — wraps existing sets.
        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> (out: Self)
            requires
                v.spec_avltreesetsteph_wf(),
                e.spec_avltreesetsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                view_ord_consistent::<Pair<V, V>>(),
                forall|u: <V as View>::V, w: <V as View>::V|
                    #[trigger] e@.contains((u, w))
                    ==> v@.contains(u) && v@.contains(w),
            ensures out.spec_edgesetgraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn num_vertices(&self) -> usize
            requires self.spec_edgesetgraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn num_edges(&self) -> usize
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; returns reference.
        fn vertices(&self) -> &AVLTreeSetStEph<V>
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; returns reference.
        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>>
            requires self.spec_edgesetgraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_edgesetgraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            requires self.spec_edgesetgraphsteph_wf()
            ensures neighbors@ == self.spec_out_neighbors(u@), neighbors.spec_avltreesetsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.1): Work O(m), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; AVL set insert.
        fn insert_vertex(&mut self, v: V)
            requires
                old(self).spec_edgesetgraphsteph_wf(),
                old(self).spec_vertices().len() + 1 < usize::MAX as nat,
            ensures self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(m lg m), Span Theta(m lg m) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(m lg m), Span Theta(m lg m) — agrees; filter and rebuild edge set.
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf(), !self.spec_vertices().contains(v@);
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; vertex insert + edge insert.
        fn insert_edge(&mut self, u: V, v: V)
            requires
                old(self).spec_edgesetgraphsteph_wf(),
                old(self).spec_vertices().len() + 2 < usize::MAX as nat,
                old(self).spec_edges().len() + 1 < usize::MAX as nat,
            ensures self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(lg m), Span Theta(lg m) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(lg m), Span Theta(lg m) — agrees; AVL set delete.
        fn delete_edge(&mut self, u: &V, v: &V)
            requires old(self).spec_edgesetgraphsteph_wf()
            ensures self.spec_edgesetgraphsteph_wf();
    }

    // 9. impls

    impl<V: StT + Ord + ClonePreservesView> EdgeSetGraphStEphTrait<V> for EdgeSetGraphStEph<V> {
        open spec fn spec_edgesetgraphsteph_wf(&self) -> bool {
            self.vertices.spec_avltreesetsteph_wf()
            && self.edges.spec_avltreesetsteph_wf()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            && vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>()
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

        fn empty() -> (out: Self) {
            EdgeSetGraphStEph {
                vertices: AVLTreeSetStEph::empty(),
                edges: AVLTreeSetStEph::empty(),
            }
        }

        fn from_vertices_and_edges(v: AVLTreeSetStEph<V>, e: AVLTreeSetStEph<Pair<V, V>>) -> (out: Self) {
            EdgeSetGraphStEph { vertices: v, edges: e }
        }

        fn num_vertices(&self) -> usize { self.vertices.size() }

        fn num_edges(&self) -> usize { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStEph<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> bool { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
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
            let mut neighbors = AVLTreeSetStEph::empty();
            let n = seq.length();
            let mut i: usize = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < n
                invariant
                    n as int == seq@.len(),
                    i <= n,
                    neighbors.spec_avltreesetsteph_wf(),
                    neighbors@.len() <= i as nat,
                    filtered.spec_avltreesetsteph_wf(),
                    seq@.to_set() =~= filtered_view,
                    self.spec_edgesetgraphsteph_wf(),
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
                        // v@ not in neighbors, so insert increases len by 1.
                        // neighbors@.insert(v@) ⊆ vertices.
                        assert forall|w: <V as View>::V|
                            #[trigger] neighbors@.insert(v@).contains(w)
                            implies self.spec_vertices().contains(w) by {
                            if w != v@ {
                                assert(neighbors@.contains(w));
                                assert(self.spec_edges().contains((u@, w)));
                            }
                        }
                        vstd::set_lib::lemma_len_subset(neighbors@.insert(v@), self.spec_vertices());
                        // neighbors@.insert(v@).len() == neighbors@.len() + 1 <= vertices.len() < usize::MAX.
                    }
                    neighbors.insert(v);
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

        /// - APAS: Work Θ(m), Span Θ(lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Θ(m), Span Θ(m) — delegates to out_neighbors which is sequential.
        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V) {
            self.vertices.insert(v);
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    // Trigger old wf: edges unchanged, so (a,b) was in old edges.
                    assert(old(self).spec_edges().contains((a, b)));
                    // Old wf gives: a and b in old vertices. New vertices is a superset.
                }
            }
        }

        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_vertices().contains(v@)
        {
            let ghost v_view = v@;
            let v_clone = v.clone_view();
            self.vertices.delete(&v_clone);
            // Iterate edges, delete those touching v.
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
                    self.edges.spec_avltreesetsteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                    view_ord_consistent::<Pair<V, V>>(),
                    self.edges@.subset_of(old(self).edges@),
                    !self.spec_vertices().contains(v_view),
                    self.vertices.spec_avltreesetsteph_wf(),
                    self.vertices@ =~= old(self).vertices@.remove(v_view),
                    old(self).spec_edgesetgraphsteph_wf(),
                    // All edges touching v from seq[0..i] have been deleted.
                    forall|j: int| 0 <= j < i && (seq@[j].0 == v_view || seq@[j].1 == v_view) ==>
                        !self.edges@.contains(#[trigger] seq@[j]),
                decreases seq_len - i
            {
                let edge_ref = seq.nth(i);
                if edge_ref.0 == *v || edge_ref.1 == *v {
                    self.edges.delete(edge_ref);
                }
                i += 1;
            }
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    // (a,b) survived deletion, so it was in old edges.
                    assert(old(self).spec_edges().contains((a, b)));
                    // Old wf gives: a and b are in old vertices.
                    assert(old(self).spec_vertices().contains(a));
                    assert(old(self).spec_vertices().contains(b));
                    // (a,b) survived, so a != v@ and b != v@.
                    assert(a != v_view && b != v_view);
                    // self.vertices@ == old(self).vertices@.remove(v@)
                    // Set.remove(x) preserves all elements except x.
                    assert(self.vertices@ =~= old(self).vertices@.remove(v_view));
                    assert(self.vertices@.contains(a));
                    assert(self.vertices@.contains(b));
                }
            }
        }

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
                        // Must be the new edge: (a,b) == (u@, v@). Both are in vertices.
                    } else {
                        // Old edge: trigger old wf.
                        assert(old(self).spec_edges().contains((a, b)));
                    }
                }
            }
        }

        fn delete_edge(&mut self, u: &V, v: &V) {
            self.edges.delete(&Pair(u.clone(), v.clone()));
            proof {
                assert forall|a: <V as View>::V, b: <V as View>::V|
                    #[trigger] self.spec_edges().contains((a, b))
                    implies self.spec_vertices().contains(a) && self.spec_vertices().contains(b) by {
                    // Remaining edge was in old edges; trigger old wf.
                    assert(old(self).spec_edges().contains((a, b)));
                }
            }
        }
    }

    // 12. derive impls in verus!

    impl<V: StT + Ord + ClonePreservesView> Clone for EdgeSetGraphStEph<V> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@,
        {
            let result = EdgeSetGraphStEph {
                vertices: self.vertices.clone(),
                edges: self.edges.clone(),
            };
            proof { assume(result@ == self@); }
            result
        }
    }

    } // verus!
}
