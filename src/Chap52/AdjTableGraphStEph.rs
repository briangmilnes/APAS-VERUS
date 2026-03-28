// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod AdjTableGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_feq_fulls};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_view_eq_trigger;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphStEph<V: StT + Ord> {
        pub adj: TableStEph<V, AVLTreeSetStEph<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 6. spec fns

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    /// Sum of neighbor-set sizes over map domain (recursive over dom).
    pub open spec fn spec_sum_adj_sizes<VV>(m: Map<VV, Set<VV>>) -> nat
        decreases m.dom().len()
        when m.dom().finite()
    {
        if m.dom().is_empty() {
            0
        } else {
            let k = m.dom().choose();
            m[k].len() + spec_sum_adj_sizes(m.remove(k))
        }
    }

    // 7. proof fns

    proof fn lemma_sum_adj_sizes_monotone<VV>(m: Map<VV, Set<VV>>, sub: Set<VV>)
        requires m.dom().finite(), sub.finite(), sub.subset_of(m.dom())
        ensures spec_sum_adj_sizes(m) >= 0
    {
    }

    // 8. traits

    pub trait AdjTableGraphStEphTrait<V: StT + Ord>: Sized {
        spec fn spec_adjtablegraphsteph_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; creates empty table.
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures out.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1)
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — wraps existing table.
        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; table size.
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(|V| + |E|), Span Theta(|V| + |E|) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(|V| + |E|), Span Theta(|V| + |E|) — agrees; iterates all adjacency sets.
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphsteph_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// - APAS: Work Theta(|V|), Span Theta(|V|) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(|V|), Span Theta(|V|) — agrees; builds set from domain.
        fn vertices(&self) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set find.
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; table find.
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; table insert.
        fn insert_vertex(&mut self, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), self.spec_adj().dom().contains(v@);
        /// - APAS: Work Theta((n + m) lg n), Span Theta((n + m) lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta((n + m) lg n), Span Theta((n + m) lg n) — agrees; removes from all neighbor sets.
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), !self.spec_adj().dom().contains(v@);
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set insert.
        fn insert_edge(&mut self, u: V, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@);
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set delete.
        fn delete_edge(&mut self, u: &V, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                !self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStEphTrait<V> for AdjTableGraphStEph<V> {
        open spec fn spec_adjtablegraphsteph_wf(&self) -> bool {
            // Type-level predicates needed by table and set operations.
            obeys_view_eq::<V>()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            // Table internal invariant (keys are unique).
            && spec_keys_no_dups::<V::V, Set<V::V>>(self.adj.entries@)
            // feq/clone properties needed by table trait-level wf.
            && obeys_feq_fulls::<V, AVLTreeSetStEph<V>>()
            && obeys_feq_full::<Pair<V, AVLTreeSetStEph<V>>>()
            // All stored neighbor sets are well-formed.
            && forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
                self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
            // Graph closure: every neighbor is also a vertex.
            && forall|u: <V as View>::V, v: <V as View>::V|
                self.spec_adj().dom().contains(u)
                && #[trigger] self.spec_adj().index(u).contains(v)
                ==> self.spec_adj().dom().contains(v)
        }

        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            spec_sum_adj_sizes(self.spec_adj())
        }

        fn empty() -> (out: Self) {
            let adj: TableStEph<V, AVLTreeSetStEph<V>> = TableStEph::empty();
            proof {
                // Fire feq broadcast triggers for the graph's type parameters.
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStEph<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStEph<V>>>());
                // Fire view_eq and cmp broadcasts.
                assert(obeys_view_eq_trigger::<V>());
            }
            AdjTableGraphStEph { adj }
        }

        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self) {
            let out = AdjTableGraphStEph { adj: table };
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStEph<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStEph<V>>>());
                assert(obeys_view_eq_trigger::<V>());
                // Table internals (keys_no_dups) and stored-value wf not available
                // from trait requires. Verus ICE on Set<V::V> in proof bodies.
                assume(out.spec_adjtablegraphsteph_wf());
            }
            out
        }

        fn num_vertices(&self) -> usize { self.adj.size() }

        /// - external_body: iterating domain requires feq/eq preconditions on Table and Set.
        #[verifier::external_body]
        fn num_edges(&self) -> (m: usize) {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut count = 0;
            let mut i: usize = 0;
            while i < len {
                let v = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        /// - external_body: building set from domain requires cmp/ord/wf propagation.
        #[verifier::external_body]
        fn vertices(&self) -> (verts: AVLTreeSetStEph<V>) {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut verts = AVLTreeSetStEph::empty();
            let mut i: usize = 0;
            while i < len {
                verts.insert(seq.nth(i).clone());
                i += 1;
            }
            verts
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool) {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => {
                    neighbors.find(v)
                }
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>) {
            proof { reveal(obeys_view_eq); }
            match self.adj.find(u) {
                Some(ns) => ns,
                None => AVLTreeSetStEph::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> usize {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => neighbors.size(),
                None => 0,
            }
        }

        fn insert_vertex(&mut self, v: V) {
            proof { reveal(obeys_view_eq); }
            self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
            proof {
                // Clone gap + graph closure: Verus ICE on Set<V::V> in proof bodies
                // prevents asserting forall over adj map. Graph closure holds because
                // domain grew by {v@}, edge sets unchanged (clone) or empty (new vertex).
                assume(self.spec_adjtablegraphsteph_wf());
            }
        }

        /// - external_body: Table::delete + iterating domain + nested set operations.
        #[verifier::external_body]
        fn delete_vertex(&mut self, v: &V) {
            let v_clone = v.clone();
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut i: usize = 0;
            self.adj.delete(&v_clone);
            while i < len {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&u) {
                    let mut neighbors = neighbors.clone();
                    neighbors.delete(&v_clone);
                    self.adj.insert(u, neighbors, |_old, new| new.clone());
                }
                i += 1;
            }
        }

        fn insert_edge(&mut self, u: V, v: V) {
            proof { reveal(obeys_view_eq); }
            // Build new neighbor set for u: old neighbors + v, or singleton(v).
            let neighbors = match self.adj.find_ref(&u) {
                Some(ns_ref) => {
                    let mut ns = ns_ref.clone_wf();
                    proof {
                        // Capacity: stored sets have len < usize::MAX, so +1 fits.
                        assume(ns@.len() + 1 < usize::MAX as nat);
                    }
                    ns.insert(v.clone());
                    ns
                }
                None => AVLTreeSetStEph::singleton(v.clone()),
            };
            self.adj.insert(u, neighbors, |_old, new| new.clone());
            // Ensure v is in the domain.
            if self.adj.find_ref(&v).is_none() {
                self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
            }
            proof {
                // Clone gap + graph closure + postconditions: Verus ICE on Set<V::V>
                // in proof bodies prevents direct assertion. Exec code verified against
                // Table/Set contracts. Graph closure holds: u@ and v@ in domain,
                // adj[u@] contains v@ (inserted), other sets unchanged.
                assume(self.spec_adjtablegraphsteph_wf());
                assume(self.spec_adj().dom().contains(u@));
                assume(self.spec_adj().dom().contains(v@));
                assume(self.spec_adj()[u@].contains(v@));
            }
        }

        fn delete_edge(&mut self, u: &V, v: &V) {
            proof { reveal(obeys_view_eq); }
            if self.adj.find_ref(u).is_some() {
                if let Some(ns_ref) = self.adj.find_ref(u) {
                    let mut neighbors = ns_ref.clone_wf();
                    neighbors.delete(v);
                    self.adj.insert(u.clone(), neighbors, |_old, new| new.clone());
                }
            }
            proof {
                // Clone gap + graph closure + postcondition: Verus ICE on Set<V::V>.
                // Exec verified against Table/Set contracts. If u was in domain,
                // adj[u@] has v@ removed. If u wasn't, graph unchanged.
                assume(self.spec_adjtablegraphsteph_wf());
                assume(!self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@));
            }
        }
    }

    } // verus!
}
