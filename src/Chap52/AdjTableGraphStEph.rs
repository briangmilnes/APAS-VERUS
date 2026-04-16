// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod AdjTableGraphStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap42::TableStEph::TableStEph::*;
    pub use crate::Chap52::AdjTableGraphSpecsAndLemmas::AdjTableGraphSpecsAndLemmas::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_feq_fulls,
        lemma_cloned_view_eq};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_view_eq_trigger;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphStEph<V: StT + Ord + TotalOrder> {
        pub adj: TableStEph<V, AVLTreeSetStEph<V>>,
    }

    //		Section 5. view impls


    impl<V: StT + Ord + TotalOrder> View for AdjTableGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    //		Section 8. traits


    pub trait AdjTableGraphStEphTrait<V: StT + Ord + TotalOrder>: Sized {
        spec fn spec_adjtablegraphsteph_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(1), Span Theta(1) — agrees; creates empty table.
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures out.spec_adjtablegraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(1), Span Theta(1) — wraps existing table.
        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self)
            requires
                table.spec_tablesteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                forall|k: <V as View>::V| #[trigger] table@.dom().contains(k) ==>
                    table.spec_stored_value(k).spec_avltreesetsteph_wf(),
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); table size
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — ACCEPTED DIFFERENCE: APAS assumes cached; impl sums degrees sequentially
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphsteph_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(|V|), Span O(|V|) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(|V|), Span Theta(|V|) — agrees; builds set from domain.
        fn vertices(&self) -> (verts: AVLTreeSetStEph<V>)
            requires
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().len() < usize::MAX as nat,
            ensures verts@ == self.spec_adj().dom();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find + set find
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphsteph_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n + d_g(v)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find returns neighbor set
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            requires self.spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find + set len
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(lg n), Span Theta(lg n) — agrees; table insert.
        fn insert_vertex(&mut self, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), self.spec_adj().dom().contains(v@);
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O((n + m) lg n), Span O((n + m) lg n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta((n + m) lg n), Span Theta((n + m) lg n) — agrees; removes from all neighbor sets.
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), !self.spec_adj().dom().contains(v@);
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n + lg m), Span O(lg n + lg m) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n + lg m), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set insert.
        fn insert_edge(&mut self, u: V, v: V)
            requires
                old(self).spec_adjtablegraphsteph_wf(),
                old(self).spec_adj().dom().len() + 1 < usize::MAX as nat,
            ensures
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@);
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n + lg m), Span O(lg n + lg m) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n + lg m), Span O(lg n + lg m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set delete.
        fn delete_edge(&mut self, u: &V, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                !self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@);
    }

    //		Section 9. impls


    impl<V: StT + Ord + TotalOrder> AdjTableGraphStEphTrait<V> for AdjTableGraphStEph<V> {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self) {
            let adj: TableStEph<V, AVLTreeSetStEph<V>> = TableStEph::empty();
            // Veracity: NEEDED proof block
            proof {
                // Fire feq broadcast triggers for the graph's type parameters.
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<V>());
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<AVLTreeSetStEph<V>>());
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStEph<V>>>());
                // Fire view_eq and cmp broadcasts.
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<V>());
            }
            AdjTableGraphStEph { adj }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self) {
            // Veracity: NEEDED proof block
            let out = AdjTableGraphStEph { adj: table };
            proof {
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<V>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<AVLTreeSetStEph<V>>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStEph<V>>>());
                // Veracity: NEEDED assert
                assert(obeys_view_eq_trigger::<V>());
                // keys_no_dups: from table.spec_tablesteph_wf() in requires.
                // stored-value wf: from quantifier in requires.
                // graph closure: from quantifier in requires.
                // Type predicates: from requires + broadcast triggers above.
            }
            out
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> usize { self.adj.size() }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn num_edges(&self) -> (m: usize) {
            proof {
                reveal(obeys_view_eq);
                lemma_entries_to_map_len::<V::V, Set<V::V>>(self.adj.entries@);
                // Establish total equality: seq sum == map sum == spec_num_edges.
                lemma_sum_entry_sizes_eq::<V::V>(self.adj.entries@, self.adj.entries@.len() as int);
                // Veracity: NEEDED assert
                assert(self.adj.entries@.subrange(0, self.adj.entries@.len() as int)
                    =~= self.adj.entries@);
            }
            let len = self.adj.entries.length();
            let ghost total = spec_sum_entry_sizes(self.adj.entries@, len as int);
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_adjtablegraphsteph_wf(),
                    0 <= i <= len,
                    len == self.adj.entries.spec_len(),
                    count as nat == spec_sum_entry_sizes(self.adj.entries@, i as int),
                    total == self.spec_num_edges(),
                    total == spec_sum_entry_sizes(self.adj.entries@, len as int),
                    self.spec_num_edges() <= usize::MAX as nat,
                // Veracity: NEEDED proof block
                decreases len - i,
            {
                let pair: &Pair<V, AVLTreeSetStEph<V>> = self.adj.entries.nth(i);
                proof {
                    // Veracity: NEEDED proof block
                    lemma_entries_to_map_contains_key::<V::V, Set<V::V>>(
                        self.adj.entries@, i as int);
                }
                let ns = self.adj.find_ref(&pair.0).unwrap();
                proof {
                    lemma_entries_to_map_get::<V::V, Set<V::V>>(self.adj.entries@, i as int);
                    // Capacity: partial sum + current <= total <= usize::MAX.
                    lemma_sum_entry_sizes_monotone::<V::V>(
                        self.adj.entries@, i as int + 1, len as int);
                }
                count = count + ns.size();
                i = i + 1;
            }
            count
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn vertices(&self) -> (verts: AVLTreeSetStEph<V>)
            ensures verts@ == self.spec_adj().dom()
        {
            proof {
                lemma_entries_to_map_len::<V::V, Set<V::V>>(self.adj.entries@);
            }
            let len = self.adj.entries.length();
            let mut verts = AVLTreeSetStEph::<V>::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_adjtablegraphsteph_wf(),
                    0 <= i <= len,
                    len == self.adj.entries.spec_len(),
                    len < usize::MAX,
                    verts.spec_avltreesetsteph_wf(),
                    verts@.len() <= i as nat,
                    forall|j: int| 0 <= j < i ==>
                        #[trigger] verts@.contains(self.adj.entries@[j].0),
                    // Veracity: NEEDED proof block
                    forall|k: <V as View>::V| #[trigger] verts@.contains(k) ==>
                        exists|j: int| 0 <= j < i && (#[trigger] self.adj.entries@[j]).0 == k,
                decreases len - i,
            {
                // Veracity: NEEDED proof block
                let pair: &Pair<V, AVLTreeSetStEph<V>> = self.adj.entries.nth(i);
                let key: V = pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq::<V>(pair.0, key);
                }
                let ghost old_verts = verts@;
                verts.insert(key);
                proof {
                    // Veracity: NEEDED assert
                    assert forall|k: <V as View>::V| #[trigger] verts@.contains(k)
                        implies exists|j: int| 0 <= j < i + 1 && (#[trigger] self.adj.entries@[j]).0 == k
                    by {
                        if old_verts.contains(k) {
                            let j = choose|j: int| 0 <= j < i && (#[trigger] self.adj.entries@[j]).0 == k;
// Veracity: UNNEEDED assert                             assert(self.adj.entries@[j].0 == k);
                        } else {
                            // Veracity: NEEDED assert (speed hint)
                            // Veracity: NEEDED proof block
                            assert(k == key@);
                            // Veracity: NEEDED assert
                            assert(self.adj.entries@[i as int].0 == key@);
                        }
                    };
                }
                i = i + 1;
            }
            proof {
                // dom → verts: every key in the map domain is in verts.
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] self.spec_adj().dom().contains(k)
                    implies verts@.contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<V::V, Set<V::V>>(self.adj.entries@, k);
                    let j = choose|j: int| 0 <= j < self.adj.entries@.len() && (#[trigger] self.adj.entries@[j]).0 == k;
// Veracity: UNNEEDED assert                     assert(verts@.contains(self.adj.entries@[j].0));
                };
                // verts → dom: every key in verts came from an entry, which is in the map.
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] verts@.contains(k)
                    implies self.spec_adj().dom().contains(k)
                by {
                    let j = choose|j: int| 0 <= j < len && (#[trigger] self.adj.entries@[j]).0 == k;
                    lemma_entries_to_map_contains_key::<V::V, Set<V::V>>(self.adj.entries@, j);
                };
// Veracity: NEEDED proof block (speed hint)
// Veracity: UNNEEDED assert                 assert(verts@ =~= self.spec_adj().dom());
            }
            verts
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => {
                    neighbors.find(v)
                }
                // Veracity: NEEDED proof block
                None => false,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                // Veracity: NEEDED proof block (speed hint)
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty()
        {
            proof { reveal(obeys_view_eq); }
            match self.adj.find(u) {
                Some(ns) => ns,
                None => AVLTreeSetStEph::empty(),
            }
        }
// Veracity: NEEDED proof block (speed hint)

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn out_degree(&self, u: &V) -> usize {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => neighbors.size(),
                // Veracity: NEEDED proof block
                None => 0,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&mut self, v: V) {
            proof { reveal(obeys_view_eq); }
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            self.adj.insert(v, AVLTreeSetStEph::empty(),
                |old: &AVLTreeSetStEph<V>, _new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                    ensures r@ == old@
                { old.clone() });
            proof {
                // Type-level predicates: from old wf.
                // keys_no_dups: from Table::insert ensures (spec_tablesteph_wf()).
                // feq: from old wf.
                // Graph closure: domain grew by {v@}, edge sets unchanged or empty.
                // Veracity: NEEDED assert
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(u)
                    && #[trigger] self.spec_adj().index(u).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    if u != v@ {
                        // adj[u] == old_adj[u] (view preserved), old closure gives old_dom.contains(w).
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_adj.dom().contains(u));
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_adj.index(u).contains(w));
                    } else if !old_dom.contains(v@) {
                        // v@ was new: adj[v@] == Set::empty(), no w exists.
                    } else {
                        // v@ existed: adj[v@] == old.clone()@ == old_adj[v@] (from combine ensures).
// Veracity: UNNEEDED assert                         assert(old_adj.dom().contains(v@));
// Veracity: UNNEEDED assert                         assert(old_adj.index(v@).contains(w));
                    }
                };
                // Stored-value wf: use lemma_spec_stored_value_view to connect
                // spec_stored_value(k)@ to self@[k], then view preservation proves wf.
                let ghost old_self_adj = old(self).adj;
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    let sv = self.adj.spec_stored_value(k);
                    // Connect sv@ to self.adj@[k].
                    self.adj.lemma_spec_stored_value_view(k);
                    if k != v@ {
                        // View preserved: self.adj@[k] == old_adj[k].
// Veracity: UNNEEDED assert                         assert(old_adj.dom().contains(k));
// Veracity: UNNEEDED assert                         assert(self.adj@[k] == old_adj[k]);
                        // Connect old stored value view to old_adj[k].
                        old_self_adj.lemma_spec_stored_value_view(k);
                        let old_sv = old_self_adj.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetsteph_wf());
                        // sv@ == self.adj@[k] == old_adj[k] == old_sv@.
                        // Since AVLTreeSetStEph@ = self.tree@:
                        // sv.tree@ == sv@ == old_sv@ == old_sv.tree@.
                        // old_sv.tree@.finite() && len < MAX from old wf.
// Veracity: UNNEEDED assert                         assert(sv.tree@ =~= old_sv.tree@);
                    } else if !old_dom.contains(v@) {
                        // New key: spec_stored_value(v@) == empty, which is wf.
                    // Veracity: NEEDED proof block
                    } else {
                        // Existing key: combine returned old.clone() with same view.
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_adj.dom().contains(v@));
                        old_self_adj.lemma_spec_stored_value_view(v@);
                        let old_sv = old_self_adj.spec_stored_value(v@);
                        assert(old_sv.spec_avltreesetsteph_wf());
                    }
                };
            }
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * (log n + d)), Span O(n * (log n + d))
        fn delete_vertex(&mut self, v: &V) {
            proof { reveal(obeys_view_eq); }
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            // Obtain domain sequence before mutation.
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            // Step 1: Remove v as a key from the adjacency table.
            self.adj.delete(v);
            let ghost adj_after_delete = self.adj@;
            // Prove stored-value wf after delete (initial loop invariant).
            proof {
                let ghost old_self_adj = old(self).adj;
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    self.adj.lemma_spec_stored_value_view(k);
                    // Veracity: NEEDED assert (speed hint)
                    assert(old_adj.dom().contains(k));
                    old_self_adj.lemma_spec_stored_value_view(k);
                    let old_sv = old_self_adj.spec_stored_value(k);
                    assert(old_sv.spec_avltreesetsteph_wf());
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                };
            }
            // Step 2: For each remaining key, remove v from its neighbor set.
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == seq@.len(),
                    seq@.no_duplicates(),
                    self.adj.spec_tablesteph_wf(),
                    obeys_view_eq::<V>(),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                    obeys_feq_fulls::<V, AVLTreeSetStEph<V>>(),
                    obeys_feq_full::<Pair<V, AVLTreeSetStEph<V>>>(),
                    // Veracity: NEEDED proof block
                    !self.adj@.dom().contains(v@),
                    // Stored-value wf invariant.
                    forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
                        self.adj.spec_stored_value(k).spec_avltreesetsteph_wf(),
                    // Domain unchanged through loop.
                    self.adj@.dom() =~= adj_after_delete.dom(),
                    // For all keys: neighbor sets are subsets of adj_after_delete values.
                    forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
                        self.adj@[k].subset_of(adj_after_delete[k]),
                    // v@ removed from all processed neighbor sets.
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==>
                        (self.adj@.dom().contains(seq@[j]) ==> !self.adj@[seq@[j]].contains(v@)),
                decreases len - i,
            {
                let nth_ref = seq.nth(i);
                let u = nth_ref.clone_plus();
                // Veracity: NEEDED proof block
                proof {
                    lemma_cloned_view_eq::<V>(*nth_ref, u);
                    seq.lemma_view_index(i as int);
                    // Veracity: NEEDED assert (speed hint)
                    assert(u@ == seq@[i as int]);
                }
                if let Some(ns_ref) = self.adj.find_ref(&u) {
                    let ghost pre_insert = self.adj@;
                    let ghost pre_insert_adj = self.adj;
                    // ns_ref == spec_stored_value(u@), which is wf by invariant.
                    let mut neighbors = ns_ref.clone_wf();
                    neighbors.delete(v);
                    let ghost neighbors_view = neighbors@;
                    self.adj.insert(u, neighbors,
                        |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                            ensures r@ == new@
                        { new.clone() });
                    proof {
                        // Prove stored-value wf after insert.
                        // Veracity: NEEDED assert
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                        by {
                            self.adj.lemma_spec_stored_value_view(k);
                            if k == u@ {
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.adj@[u@] == neighbors_view);
// Veracity: UNNEEDED assert                                 assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                            } else {
// Veracity: UNNEEDED assert                                 assert(pre_insert.dom().contains(k));
                                pre_insert_adj.lemma_spec_stored_value_view(k);
                                let old_sv = pre_insert_adj.spec_stored_value(k);
                                assert(old_sv.spec_avltreesetsteph_wf());
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                            }
                        };
                        // Domain unchanged.
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.adj@.dom() =~= adj_after_delete.dom());
                        // Subset invariant.
                        // Veracity: NEEDED assert
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj@[k].subset_of(adj_after_delete[k])
                        by {
                            if k == u@ {
// Veracity: UNNEEDED assert                                 assert(neighbors_view.subset_of(pre_insert[u@]));
                                // Veracity: NEEDED assert (speed hint)
                                assert(pre_insert[u@].subset_of(adj_after_delete[u@]));
                            } else {
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.adj@[k] == pre_insert[k]);
                            }
                        };
                        // v-removal invariant.
// Veracity: UNNEEDED assert                         assert(!neighbors_view.contains(v@));
                        // Helper: for k != u@ in post-insert domain, value unchanged from pre_insert.
// Veracity: UNNEEDED assert                         assert(pre_insert.dom() =~= adj_after_delete.dom());
                        // Veracity: NEEDED assert
                        assert forall|k: <V as View>::V| k != u@ && #[trigger] self.adj@.dom().contains(k) implies
                            pre_insert.dom().contains(k) && self.adj@[k] == pre_insert[k]
                        by {
// Veracity: UNNEEDED assert                             assert(adj_after_delete.dom().contains(k));
                            // Veracity: NEEDED assert (speed hint)
                            assert(pre_insert.dom().contains(k));
                        };
                        // Veracity: NEEDED assert
                        assert forall|j: int| #![trigger seq@[j]] 0 <= j < (i + 1) as int && self.adj@.dom().contains(seq@[j]) implies
                            // Veracity: NEEDED proof block
                            !self.adj@[seq@[j]].contains(v@)
                        by {
                            if j == i as int {
// Veracity: UNNEEDED assert                                 assert(self.adj@[u@] == neighbors_view);
                            } else if self.adj@.dom().contains(seq@[j]) {
                                // j < i. no_duplicates ⟹ seq@[j] != seq@[i] == u@.
// Veracity: UNNEEDED assert                                 assert(seq@[j] != u@);
                                // Helper: value preserved for non-u@ keys.
// Veracity: UNNEEDED assert                                 assert(self.adj@[seq@[j]] == pre_insert[seq@[j]]);
                                // Old invariant at pre-insert: !pre_insert[seq@[j]].contains(v@).
// Veracity: UNNEEDED assert                                 assert(pre_insert.dom().contains(seq@[j]));
// Veracity: UNNEEDED assert                                 assert(!pre_insert[seq@[j]].contains(v@));
                            }
                        };
                    }
                }
                i += 1;
            }
            proof {
                // v@ removed from ALL neighbor sets (loop processed entire domain).
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    !self.adj@[k].contains(v@)
                by {
                    // k ∈ dom ⊆ old_dom = seq@.to_set(). So seq@ contains k.
// Veracity: UNNEEDED assert                     assert(old_dom.contains(k));
// Veracity: UNNEEDED assert                     assert(seq@.to_set().contains(k));
                    // Seq::to_set().contains ⟹ Seq::contains ⟹ ∃j: seq@[j]==k.
                    // Loop invariant (i==len) gives !adj[k].contains(v@).
                };
                // Graph closure: for any u,w with adj[u].contains(w), w is in the domain.
                // Veracity: NEEDED assert
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(u)
                    // Veracity: NEEDED proof block
                    && #[trigger] self.spec_adj().index(u).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    // !adj[u].contains(v@) (from above), but adj[u].contains(w), so w≠v@.
// Veracity: UNNEEDED assert                     assert(!self.adj@[u].contains(v@));
                    // Veracity: NEEDED assert (speed hint)
                    // Veracity: NEEDED proof block (speed hint)
                    assert(w != v@);
                    // adj[u] ⊆ adj_after_delete[u] == old_adj[u], so old_adj[u].contains(w).
                    // Veracity: NEEDED assert (speed hint)
                    assert(adj_after_delete[u].contains(w));
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                     assert(old_adj[u].contains(w));
                    // Old graph closure gives old_dom.contains(w). w≠v@ ⟹ dom.contains(w).
// Veracity: UNNEEDED assert                     assert(old_dom.contains(w));
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
        fn insert_edge(&mut self, u: V, v: V) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            // Clone v with view equality proof for the neighbor set.
            let vc = v.clone_plus();
            proof { lemma_cloned_view_eq::<V>(v, vc); }
            // Veracity: NEEDED proof block
            // Build new neighbor set for u: old neighbors + vc, or singleton(vc).
            let neighbors = match self.adj.find_ref(&u) {
                Some(ns_ref) => {
                    let mut ns = ns_ref.clone_wf();
                    proof {
                        // Capacity: graph closure ⇒ ns@ ⊆ domain ⇒ ns@.len() ≤ dom.len().
                        let dom = self.spec_adj().dom();
// Veracity: UNNEEDED assert                         assert(ns@.subset_of(dom)) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert forall|w: <V as View>::V| #[trigger] ns@.contains(w) implies dom.contains(w) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 assert(self.spec_adj().index(u@).contains(w));
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             };
// Veracity: UNNEEDED assert                         };
                        lemma_entries_to_map_finite::<V::V, Set<V::V>>(self.adj.entries@);
                        vstd::set_lib::lemma_len_subset(ns@, dom);
                    }
                    ns.insert(vc);
                    ns
                }
                None => AVLTreeSetStEph::singleton(vc),
            };
            // Veracity: NEEDED proof block
            let ghost neighbors_view = neighbors@;
            // neighbors_view ⊆ old_dom ∪ {v@}: elements are from old_adj[u@] (⊆ old_dom) plus v@.
            proof {
// Veracity: UNNEEDED assert                 assert(neighbors_view.subset_of(old_dom.insert(v_view))) by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                     assert forall|w: <V as View>::V| #[trigger] neighbors_view.contains(w)
// Veracity: UNNEEDED assert                         implies old_dom.insert(v_view).contains(w)
// Veracity: UNNEEDED assert                     by {
// Veracity: UNNEEDED assert                         if w != v_view {
// Veracity: UNNEEDED assert                             // w was in old_adj[u@] (from graph closure).
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                             assert(old_adj.dom().contains(u_view));
// Veracity: UNNEEDED assert                             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                             assert(old_adj.index(u_view).contains(w));
// Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                 };
            }
            self.adj.insert(u, neighbors,
                // Veracity: NEEDED proof block
                |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                    ensures r@ == new@
                { new.clone() });
            // First insert ensures: dom contains u@, and adj[u@] == neighbors_view.
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(self.adj@.dom().contains(u_view));
                // Veracity: NEEDED assert (speed hint)
                assert(self.adj@[u_view] == neighbors_view);
            }
            let ghost adj_after_first = self.adj@;
            // Ensure v is in the domain.
            if self.adj.find_ref(&v).is_none() {
                let ghost pre_second = self.adj@;
                self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
                proof {
                    // Veracity: NEEDED assert (speed hint)
                    assert(!pre_second.dom().contains(v_view));
// Veracity: UNNEEDED assert                     assert(pre_second.dom().contains(u_view));
                    // Veracity: NEEDED assert (speed hint)
                    assert(u_view != v_view);
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.adj@[u_view] == pre_second[u_view]);
                }
            }
            proof {
// Veracity: UNNEEDED assert                 assert(self.spec_adj().dom().contains(u_view));
// Veracity: UNNEEDED assert                 assert(self.spec_adj().dom().contains(v_view));
// Veracity: UNNEEDED assert                 assert(neighbors_view.contains(v_view));
                // Veracity: NEEDED assert (speed hint)
                assert(self.spec_adj()[u_view].contains(v_view));
                // Graph closure: all neighbors of any vertex are in the domain.
                // Veracity: NEEDED assert
                assert forall|x: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(x)
                    && #[trigger] self.spec_adj().index(x).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    if x == u_view {
                        // adj[u@] == neighbors_view ⊆ old_dom ∪ {v@} ⊆ final_dom.
// Veracity: UNNEEDED assert                         assert(neighbors_view.contains(w));
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_dom.insert(v_view).contains(w));
                    } else {
                        // adj[x] == old_adj[x] (view preserved through both inserts).
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_adj.dom().contains(x));
// Veracity: UNNEEDED assert                         assert(old_adj.index(x).contains(w));
                    }
                };
                // Stored-value wf: proved via lemma_spec_stored_value_view.
                let ghost old_self_adj_ie = old(self).adj;
                // Veracity: NEEDED assert
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    // Veracity: NEEDED proof block
                    self.adj.lemma_spec_stored_value_view(k);
                    if k == u_view {
                        // spec_stored_value(u@)@ == self.adj@[u@] == neighbors_view == neighbors@.
                        // neighbors was built from clone_wf + insert (or singleton), both wf.
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.adj@[u_view] == neighbors_view);
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                    } else if k == v_view && !old_dom.contains(v_view) {
                        // v@ was new: spec_stored_value(v@)@ == empty@.
                    } else {
                        // Veracity: NEEDED proof block (speed hint)
                        // View preserved: self.adj@[k] == old_adj[k].
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_adj.dom().contains(k));
                        old_self_adj_ie.lemma_spec_stored_value_view(k);
                        // Veracity: NEEDED proof block
                        let old_sv = old_self_adj_ie.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetsteph_wf());
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                    }
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
        fn delete_edge(&mut self, u: &V, v: &V) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            if self.adj.find_ref(u).is_some() {
                if let Some(ns_ref) = self.adj.find_ref(u) {
                    let mut neighbors = ns_ref.clone_wf();
                    neighbors.delete(v);
                    let ghost neighbors_view = neighbors@;
// Veracity: UNNEEDED assert                     proof { assert(!neighbors_view.contains(v_view)); }
                    let uc = u.clone_plus();
                    proof { lemma_cloned_view_eq::<V>(*u, uc); }
                    self.adj.insert(uc, neighbors,
                        |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                            ensures r@ == new@
                        { new.clone() });
                    proof {
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.adj@[u_view] == neighbors_view);
                        // Veracity: NEEDED assert (speed hint)
                        assert(!self.spec_adj()[u_view].contains(v_view));
                        // Graph closure: neighbors_view ⊆ old_adj[u@] ⊆ old_dom == dom.
                        // Veracity: NEEDED assert
                        assert forall|x: <V as View>::V, w: <V as View>::V|
                            self.spec_adj().dom().contains(x)
                            && #[trigger] self.spec_adj().index(x).contains(w)
                            implies self.spec_adj().dom().contains(w)
                        by {
                            if x == u_view {
                                // adj[u@] = old_adj[u@].remove(v@) ⊆ old_adj[u@].
                                // Veracity: NEEDED assert (speed hint)
                                assert(old_adj.index(u_view).contains(w));
                            } else {
                                // Veracity: NEEDED assert (speed hint)
                                assert(old_adj.dom().contains(x));
// Veracity: UNNEEDED assert                                 assert(old_adj.index(x).contains(w));
                            }
                        };
                        // Stored-value wf: proved via lemma_spec_stored_value_view.
                        let ghost old_self_adj_de = old(self).adj;
                        // Veracity: NEEDED assert
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                        by {
                            self.adj.lemma_spec_stored_value_view(k);
                            if k == u_view {
                                // spec_stored_value(u@)@ == self.adj@[u@] == neighbors_view.
                                // neighbors was clone_wf + delete, both wf-preserving.
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.adj@[u_view] == neighbors_view);
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                            } else {
                                // View preserved: self.adj@[k] == old_adj[k].
                                // Veracity: NEEDED assert (speed hint)
                                assert(old_adj.dom().contains(k));
                                old_self_adj_de.lemma_spec_stored_value_view(k);
                                let old_sv = old_self_adj_de.spec_stored_value(k);
                                assert(old_sv.spec_avltreesetsteph_wf());
// Veracity: UNNEEDED assert                                 assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                            }
                        };
                    }
                }
            }
            // No-mutation branch: postcondition from old wf (u@ not in domain).
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<V: StT + Ord + TotalOrder> std::fmt::Debug for AdjTableGraphStEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjTableGraphStEph(vertices: {})", self.adj.size())
        }
    }

    impl<V: StT + Ord + TotalOrder> std::fmt::Display for AdjTableGraphStEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjTableGraphStEph(vertices: {})", self.adj.size())
        }
    }
}
