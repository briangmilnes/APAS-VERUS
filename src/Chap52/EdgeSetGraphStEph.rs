// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Edge Set Graph representation (ephemeral, single-threaded).

pub mod EdgeSetGraphStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

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
    #[verifier::reject_recursive_types(V)]
    pub struct EdgeSetGraphStEph<V: StT + Ord> {
        pub vertices: AVLTreeSetStEph<V>,
        pub edges: AVLTreeSetStEph<Pair<V, V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for EdgeSetGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait EdgeSetGraphStEphTrait<V: StT + Ord>: Sized {
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
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; vertex set size.
        fn num_vertices(&self) -> N
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; edge set size.
        fn num_edges(&self) -> N
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; returns reference.
        fn vertices(&self) -> &AVLTreeSetStEph<V>
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; returns reference.
        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>>
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(lg m), Span Theta(lg m) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(lg m), Span Theta(lg m) — agrees; AVL set find.
        fn has_edge(&self, u: &V, v: &V) -> B
            requires self.spec_edgesetgraphsteph_wf();
        /// - APAS: Work Theta(m lg n), Span Theta(m lg n) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(m lg n), Span Theta(m lg n) — agrees; filter edges + build set.
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            requires self.spec_edgesetgraphsteph_wf()
            ensures neighbors@ == self.spec_out_neighbors(u@), neighbors.spec_avltreesetsteph_wf();
        /// - APAS: Work Theta(m), Span Theta(m) [Cost Spec 52.1]
        /// - Claude-Opus-4.6: Work Theta(m lg n), Span Theta(m lg n) — delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> N
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

    impl<V: StT + Ord> EdgeSetGraphStEphTrait<V> for EdgeSetGraphStEph<V> {
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

        fn num_vertices(&self) -> N { self.vertices.size() }

        fn num_edges(&self) -> N { self.edges.size() }

        fn vertices(&self) -> &AVLTreeSetStEph<V> { &self.vertices }

        fn edges(&self) -> &AVLTreeSetStEph<Pair<V, V>> { &self.edges }

        fn has_edge(&self, u: &V, v: &V) -> B { self.edges.find(&Pair(u.clone(), v.clone())) }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            ensures neighbors@ == self.spec_out_neighbors(u@)
        {
            let u_clone = u.clone();
            let filtered = self.edges.filter(
                |edge| edge.0 == u_clone,
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
                let pair_ref = seq.nth(i);
                let v = pair_ref.1.clone();
                proof {
                    assert(seq@.to_set().contains(seq@[i as int]));
                    assert(filtered_view.contains(seq@[i as int]));
                    assert(self.edges@.contains(seq@[i as int]));
                    assert(seq@[i as int].0 == u@);
                }
                neighbors.insert(v);
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
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_vertices().contains(v@)
        {
            let v_clone = v.clone();
            self.vertices.delete(&v_clone);
            let seq = self.edges.to_seq();
            let seq_len = seq.length();
            let mut to_remove: Vec<Pair<V, V>> = Vec::new();
            let mut i: usize = 0;
            while i < seq_len
                invariant
                    i <= seq_len,
                    seq_len as nat == seq@.len(),
                    seq.spec_avltreeseqsteph_wf(),
                decreases seq_len - i
            {
                let edge_ref = seq.nth(i);
                let u = edge_ref.0.clone();
                let w = edge_ref.1.clone();
                if u == v_clone || w == v_clone {
                    to_remove.push(Pair(u, w));
                }
                i += 1;
            }
            let mut j: usize = 0;
            while j < to_remove.len()
                invariant
                    j <= to_remove.len(),
                    !self.spec_vertices().contains(v@),
                    self.edges.spec_avltreesetsteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<V, V>>(),
                    view_ord_consistent::<Pair<V, V>>(),
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
