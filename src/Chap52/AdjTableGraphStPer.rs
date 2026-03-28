// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Adjacency Table Graph representation (persistent, single-threaded).
//! G = (V × V set) table - maps vertices to sets of their out-neighbors.

pub mod AdjTableGraphStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::{
        spec_sum_adj_sizes, spec_sum_entry_sizes,
        lemma_sum_adj_remove, lemma_sum_entry_sizes_monotone,
    };
    #[cfg(verus_keep_ghost)]
    use crate::Chap42::TableStEph::TableStEph::spec_entries_to_map as steph_entries_to_map;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_feq_fulls,
        lemma_cloned_view_eq};
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
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphStPer<V: StT + Ord> {
        pub adj: TableStPer<V, AVLTreeSetStPer<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 7. proof fns

    /// Bridge: StPer and StEph spec_entries_to_map are identical (same open spec body).
    proof fn lemma_entries_to_map_eq<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries) == steph_entries_to_map(entries)
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_eq::<KV, VV>(entries.drop_last());
        }
    }

    /// Bridge: StPer and StEph spec_keys_no_dups are identical.
    proof fn lemma_keys_no_dups_eq<KV, VV>(entries: Seq<(KV, VV)>)
        ensures
            spec_keys_no_dups(entries)
            == crate::Chap42::TableStEph::TableStEph::spec_keys_no_dups(entries)
    {}

    /// Connect sequential entry sum to recursive map sum (StPer version).
    proof fn lemma_sum_entry_sizes_eq_stper<VV>(entries: Seq<(VV, Set<VV>)>, n: int)
        requires
            0 <= n <= entries.len(),
            spec_keys_no_dups(entries),
        ensures
            spec_sum_entry_sizes(entries, n) == spec_sum_adj_sizes(
                spec_entries_to_map(entries.subrange(0, n)))
    {
        lemma_entries_to_map_eq::<VV, Set<VV>>(entries.subrange(0, n));
        lemma_keys_no_dups_eq::<VV, Set<VV>>(entries);
        crate::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::lemma_sum_entry_sizes_eq::<VV>(entries, n);
    }

    // 8. traits

    pub trait AdjTableGraphStPerTrait<V: StT + Ord>: Sized {
        spec fn spec_adjtablegraphstper_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures out.spec_adjtablegraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_table(table: TableStPer<V, AVLTreeSetStPer<V>>) -> (out: Self)
            requires
                table.spec_tablestper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                forall|k: <V as View>::V| #[trigger] table@.dom().contains(k) ==>
                    table.spec_stored_value(k).spec_avltreesetstper_wf(),
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphstper_wf();
        /// Work Theta(|V| + |E|), Span Theta(|V| + |E|)
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphstper_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// Work Theta(|V|), Span Theta(|V|)
        fn vertices(&self) -> (verts: AVLTreeSetStPer<V>)
            requires
                self.spec_adjtablegraphstper_wf(),
                self.spec_adj().dom().len() < usize::MAX as nat,
            ensures verts@ == self.spec_adj().dom();
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphstper_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            requires self.spec_adjtablegraphstper_wf()
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphstper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures updated.spec_adjtablegraphstper_wf(), updated.spec_adj().dom().contains(v@);
        /// Work Theta((|V| + |E|) log |V|), Span Theta((|V| + |E|) log |V|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures updated.spec_adjtablegraphstper_wf(), !updated.spec_adj().dom().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires
                self.spec_adjtablegraphstper_wf(),
                self.spec_adj().dom().len() + 1 < usize::MAX as nat,
            ensures
                updated.spec_adjtablegraphstper_wf(),
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures
                updated.spec_adjtablegraphstper_wf(),
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStPerTrait<V> for AdjTableGraphStPer<V> {
        open spec fn spec_adjtablegraphstper_wf(&self) -> bool {
            // Type-level predicates needed by table and set operations.
            obeys_view_eq::<V>()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            // Table internal invariant (keys are unique).
            && spec_keys_no_dups::<V::V, Set<V::V>>(self.adj.entries@)
            // feq/clone properties needed by table operations.
            && obeys_feq_fulls::<V, AVLTreeSetStPer<V>>()
            && obeys_feq_full::<Pair<V, AVLTreeSetStPer<V>>>()
            // All stored neighbor sets are well-formed.
            && forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
                self.adj.spec_stored_value(k).spec_avltreesetstper_wf()
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
            let adj: TableStPer<V, AVLTreeSetStPer<V>> = TableStPer::empty();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStPer<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStPer<V>>>());
                assert(obeys_view_eq_trigger::<V>());
            }
            AdjTableGraphStPer { adj }
        }

        fn from_table(table: TableStPer<V, AVLTreeSetStPer<V>>) -> (out: Self) {
            let out = AdjTableGraphStPer { adj: table };
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStPer<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStPer<V>>>());
                assert(obeys_view_eq_trigger::<V>());
                // keys_no_dups: from table.spec_tablestper_wf() in requires.
                // stored-value wf: from quantifier in requires.
                // graph closure: from quantifier in requires.
                // Type predicates: from requires + broadcast triggers above.
            }
            out
        }

        fn num_vertices(&self) -> usize {
            proof { reveal(obeys_view_eq); }
            self.adj.size()
        }

        fn num_edges(&self) -> (m: usize) {
            proof {
                reveal(obeys_view_eq);
                lemma_entries_to_map_len::<V::V, Set<V::V>>(self.adj.entries@);
                lemma_sum_entry_sizes_eq_stper::<V::V>(self.adj.entries@, self.adj.entries@.len() as int);
                assert(self.adj.entries@.subrange(0, self.adj.entries@.len() as int)
                    =~= self.adj.entries@);
            }
            let len = self.adj.entries.length();
            let ghost total = spec_sum_entry_sizes(self.adj.entries@, len as int);
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_adjtablegraphstper_wf(),
                    0 <= i <= len,
                    len == self.adj.entries.spec_len(),
                    count as nat == spec_sum_entry_sizes(self.adj.entries@, i as int),
                    total == self.spec_num_edges(),
                    total == spec_sum_entry_sizes(self.adj.entries@, len as int),
                    self.spec_num_edges() <= usize::MAX as nat,
                decreases len - i,
            {
                let pair: &Pair<V, AVLTreeSetStPer<V>> = self.adj.entries.nth(i);
                proof {
                    lemma_entries_to_map_contains_key::<V::V, Set<V::V>>(
                        self.adj.entries@, i as int);
                }
                let ns = self.adj.find_ref(&pair.0).unwrap();
                proof {
                    lemma_entries_to_map_get::<V::V, Set<V::V>>(self.adj.entries@, i as int);
                    lemma_sum_entry_sizes_monotone::<V::V>(
                        self.adj.entries@, i as int + 1, len as int);
                }
                count = count + ns.size();
                i = i + 1;
            }
            count
        }

        fn vertices(&self) -> (verts: AVLTreeSetStPer<V>) {
            proof {
                lemma_entries_to_map_len::<V::V, Set<V::V>>(self.adj.entries@);
            }
            let len = self.adj.entries.length();
            let mut verts = AVLTreeSetStPer::<V>::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_adjtablegraphstper_wf(),
                    0 <= i <= len,
                    len == self.adj.entries.spec_len(),
                    len < usize::MAX,
                    verts.spec_avltreesetstper_wf(),
                    verts@.len() <= i as nat,
                    // verts contains exactly the keys from entries[0..i].
                    forall|j: int| 0 <= j < i
                        ==> #[trigger] verts@.contains(self.adj.entries@[j].0),
                    forall|v: V::V| #[trigger] verts@.contains(v)
                        ==> exists|j: int| 0 <= j < i
                            && self.adj.entries@[j].0 == v,
                decreases len - i,
            {
                let pair: &Pair<V, AVLTreeSetStPer<V>> = self.adj.entries.nth(i);
                let key: V = pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq::<V>(pair.0, key);
                }
                let ghost old_verts = verts@;
                verts = verts.insert(key);
                proof {
                    // Maintain forward invariant: entries[0..i+1] keys all in verts.
                    assert forall|j: int| 0 <= j < i + 1
                        implies #[trigger] verts@.contains(self.adj.entries@[j].0)
                    by {
                        if j < i {
                            assert(old_verts.contains(self.adj.entries@[j].0));
                        }
                    };
                    // Maintain backward invariant: every element in verts came from entries[0..i+1].
                    assert forall|v: V::V| #[trigger] verts@.contains(v)
                        implies exists|j: int| 0 <= j < i + 1
                            && self.adj.entries@[j].0 == v
                    by {
                        if old_verts.contains(v) {
                            let j = choose|j: int| 0 <= j < i
                                && self.adj.entries@[j].0 == v;
                            assert(j < i + 1);
                        } else {
                            // v == key@ == pair.0@ == entries[i].0
                            assert(self.adj.entries@[i as int].0 == v);
                        }
                    };
                }
                i = i + 1;
            }
            proof {
                // At end: verts contains exactly all entry keys = self@.dom().
                assert forall|v: V::V| #[trigger] verts@.contains(v)
                    == self.spec_adj().dom().contains(v)
                by {
                    if verts@.contains(v) {
                        let j = choose|j: int| 0 <= j < len as int
                            && self.adj.entries@[j].0 == v;
                        lemma_entries_to_map_contains_key::<V::V, Set<V::V>>(
                            self.adj.entries@, j);
                    }
                    if self.spec_adj().dom().contains(v) {
                        lemma_entries_to_map_key_in_seq::<V::V, Set<V::V>>(
                            self.adj.entries@, v);
                        let j = choose|j: int| 0 <= j < self.adj.entries@.len()
                            && (#[trigger] self.adj.entries@[j]).0 == v;
                    }
                };
                lemma_entries_to_map_finite::<V::V, Set<V::V>>(self.adj.entries@);
                assert(verts@ =~= self.spec_adj().dom());
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

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>) {
            proof { reveal(obeys_view_eq); }
            match self.adj.find(u) {
                Some(ns) => {
                    // find ensures: self.adj@.contains_key(u@) && self.adj@[u@] == ns@.
                    // Since spec_adj() = self.adj@, both postconditions follow directly.
                    ns
                }
                None => {
                    let empty = AVLTreeSetStPer::empty();
                    // find ensures: !self.adj@.contains_key(u@).
                    // empty ensures: empty@ == Set::empty().
                    // Both postconditions are trivially satisfied.
                    empty
                }
            }
        }

        fn out_degree(&self, u: &V) -> usize {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => neighbors.size(),
                None => 0,
            }
        }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty(),
                |old: &AVLTreeSetStPer<V>, _new: &AVLTreeSetStPer<V>| -> (r: AVLTreeSetStPer<V>)
                    ensures r@ == old@
                { old.clone() });
            let updated = AdjTableGraphStPer { adj: new_adj };
            proof {
                // Graph closure: domain grew by {v@}, edge sets unchanged or empty.
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    updated.spec_adj().dom().contains(u)
                    && #[trigger] updated.spec_adj().index(u).contains(w)
                    implies updated.spec_adj().dom().contains(w)
                by {
                    if u != v@ {
                        assert(old_adj.dom().contains(u));
                        assert(old_adj.index(u).contains(w));
                    } else if !old_dom.contains(v@) {
                        // v@ new: adj[v@] == Set::empty().
                    } else {
                        // v@ existed: adj[v@] == old_adj[v@] (combine ensures r@ == old@).
                        assert(old_adj.dom().contains(v@));
                        assert(old_adj.index(v@).contains(w));
                    }
                };
                // Stored-value wf: proved via lemma_spec_stored_value_view.
                assert forall|k: <V as View>::V| #[trigger] updated.adj@.dom().contains(k) implies
                    updated.adj.spec_stored_value(k).spec_avltreesetstper_wf()
                by {
                    updated.adj.lemma_spec_stored_value_view(k);
                    if k != v@ {
                        assert(old_adj.dom().contains(k));
                        assert(updated.adj@[k] == old_adj[k]);
                        self.adj.lemma_spec_stored_value_view(k);
                        let old_sv = self.adj.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetstper_wf());
                        assert(updated.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                    } else if !old_dom.contains(v@) {
                        // New key: stored value is empty, which is wf.
                    } else {
                        // Existing key: combine returned clone with same view.
                        assert(old_adj.dom().contains(v@));
                        self.adj.lemma_spec_stored_value_view(v@);
                        let old_sv = self.adj.spec_stored_value(v@);
                        assert(old_sv.spec_avltreesetstper_wf());
                    }
                };
            }
            updated
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            // Step 1: Remove v as a key from the adjacency table.
            let new_adj = self.adj.delete(v);
            let ghost adj_after_delete = new_adj@;
            // Prove stored-value wf after delete (initial loop invariant).
            proof {
                assert forall|k: <V as View>::V| #[trigger] new_adj@.dom().contains(k) implies
                    new_adj.spec_stored_value(k).spec_avltreesetstper_wf()
                by {
                    new_adj.lemma_spec_stored_value_view(k);
                    assert(old_adj.dom().contains(k));
                    self.adj.lemma_spec_stored_value_view(k);
                    let old_sv = self.adj.spec_stored_value(k);
                    assert(old_sv.spec_avltreesetstper_wf());
                    assert(new_adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                };
            }
            // Obtain domain sequence from table with v removed.
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            // Step 2: For each remaining key, remove v from its neighbor set.
            let mut result_adj = new_adj;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == seq@.len(),
                    seq@.no_duplicates(),
                    result_adj.spec_tablestper_wf(),
                    obeys_view_eq::<V>(),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                    obeys_feq_fulls::<V, AVLTreeSetStPer<V>>(),
                    obeys_feq_full::<Pair<V, AVLTreeSetStPer<V>>>(),
                    !result_adj@.dom().contains(v@),
                    // Stored-value wf invariant.
                    forall|k: <V as View>::V| #[trigger] result_adj@.dom().contains(k) ==>
                        result_adj.spec_stored_value(k).spec_avltreesetstper_wf(),
                    // Domain unchanged through loop.
                    result_adj@.dom() =~= adj_after_delete.dom(),
                    // For all keys: neighbor sets are subsets of adj_after_delete values.
                    forall|k: <V as View>::V| #[trigger] result_adj@.dom().contains(k) ==>
                        result_adj@[k].subset_of(adj_after_delete[k]),
                    // v@ removed from all processed neighbor sets.
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==>
                        (result_adj@.dom().contains(seq@[j]) ==> !result_adj@[seq@[j]].contains(v@)),
                decreases len - i,
            {
                let nth_ref = seq.nth(i);
                let u = nth_ref.clone_plus();
                proof {
                    lemma_cloned_view_eq::<V>(*nth_ref, u);
                    seq.lemma_view_index(i as int);
                    assert(u@ == seq@[i as int]);
                }
                if let Some(neighbors) = result_adj.find(&u) {
                    proof {
                        // Prove neighbors.spec_avltreesetstper_wf() from stored-value-wf.
                        // find ensures neighbors@ == result_adj@[u@].
                        // lemma gives spec_stored_value(u@)@ == result_adj@[u@].
                        // So neighbors@ == spec_stored_value(u@)@, i.e., neighbors.tree@ == sv.tree@.
                        // From invariant: sv.spec_avltreesetstper_wf(). Same tree@ ⟹ same wf.
                        result_adj.lemma_spec_stored_value_view(u@);
                        let sv = result_adj.spec_stored_value(u@);
                        assert(sv.spec_avltreesetstper_wf());
                        assert(neighbors@ == sv@);
                        assert(neighbors.tree@ =~= sv.tree@);
                    }
                    let new_neighbors = neighbors.delete(v);
                    let ghost nn_view = new_neighbors@;
                    let ghost pre_insert = result_adj@;
                    let ghost pre_insert_adj = result_adj;
                    result_adj = result_adj.insert(u, new_neighbors,
                        |_old: &AVLTreeSetStPer<V>, new: &AVLTreeSetStPer<V>| -> (r: AVLTreeSetStPer<V>)
                            ensures r@ == new@
                        { new.clone() });
                    proof {
                        // Help Z3 with the insert_edge existential ensures.
                        assert(pre_insert.dom().contains(u@));
                        // Prove stored-value wf after insert.
                        assert forall|k: <V as View>::V| #[trigger] result_adj@.dom().contains(k) implies
                            result_adj.spec_stored_value(k).spec_avltreesetstper_wf()
                        by {
                            result_adj.lemma_spec_stored_value_view(k);
                            if k == u@ {
                                // insert ensures for existing key: updated@[key@] == r@ where
                                // combine.ensures((&old_v, &value), r) gives r@ == new_neighbors@.
                                assert(result_adj.spec_stored_value(k).tree@ =~= new_neighbors.tree@);
                            } else {
                                assert(pre_insert.dom().contains(k));
                                pre_insert_adj.lemma_spec_stored_value_view(k);
                                let old_sv = pre_insert_adj.spec_stored_value(k);
                                assert(old_sv.spec_avltreesetstper_wf());
                                assert(result_adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                            }
                        };
                        // Domain unchanged.
                        assert(result_adj@.dom() =~= adj_after_delete.dom());
                        // Subset invariant.
                        assert forall|k: <V as View>::V| #[trigger] result_adj@.dom().contains(k) implies
                            result_adj@[k].subset_of(adj_after_delete[k])
                        by {
                            if k == u@ {
                                // insert ensures for existing key: r@ == new_neighbors@.
                                // new_neighbors@ ⊆ neighbors@ ⊆ pre_insert[u@] ⊆ adj_after_delete[u@].
                            } else {
                                assert(pre_insert.dom().contains(k));
                            }
                        };
                        // v-removal invariant.
                        assert(!nn_view.contains(v@));
                        // Helper: for k != u@ in post-insert domain, value unchanged from pre_insert.
                        assert(pre_insert.dom() =~= adj_after_delete.dom());
                        assert forall|k: <V as View>::V| k != u@ && #[trigger] result_adj@.dom().contains(k) implies
                            pre_insert.dom().contains(k) && result_adj@[k] == pre_insert[k]
                        by {
                            assert(adj_after_delete.dom().contains(k));
                            assert(pre_insert.dom().contains(k));
                        };
                        assert forall|j: int| #![trigger seq@[j]] 0 <= j < (i + 1) as int implies
                            (result_adj@.dom().contains(seq@[j]) ==> !result_adj@[seq@[j]].contains(v@))
                        by {
                            if j == i as int {
                                assert(result_adj@[u@] == nn_view);
                            } else if result_adj@.dom().contains(seq@[j]) {
                                // j < i. no_duplicates ⟹ seq@[j] != seq@[i] == u@.
                                assert(seq@[j] != u@);
                                // Helper: value preserved for non-u@ keys.
                                assert(result_adj@[seq@[j]] == pre_insert[seq@[j]]);
                                // Old invariant at pre-insert: !pre_insert[seq@[j]].contains(v@).
                                assert(pre_insert.dom().contains(seq@[j]));
                                assert(!pre_insert[seq@[j]].contains(v@));
                            }
                        };
                    }
                }
                i += 1;
            }
            let updated = AdjTableGraphStPer { adj: result_adj };
            proof {
                // v@ removed from ALL neighbor sets (loop processed entire domain).
                assert forall|k: <V as View>::V| #[trigger] result_adj@.dom().contains(k) implies
                    !result_adj@[k].contains(v@)
                by {
                    // k ∈ dom =~= adj_after_delete.dom() =~= seq@.to_set().
                    assert(seq@.to_set().contains(k));
                    // Seq::to_set().contains ⟹ Seq::contains ⟹ ∃j: seq@[j]==k.
                    // Loop invariant (i==len) gives !result_adj@[k].contains(v@).
                };
                // Graph closure: for any u,w with adj[u].contains(w), w is in the domain.
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    updated.spec_adj().dom().contains(u)
                    && #[trigger] updated.spec_adj().index(u).contains(w)
                    implies updated.spec_adj().dom().contains(w)
                by {
                    // !adj[u].contains(v@) (from above), but adj[u].contains(w), so w≠v@.
                    assert(!result_adj@[u].contains(v@));
                    assert(w != v@);
                    // adj[u] ⊆ adj_after_delete[u] == old_adj[u], so old_adj[u].contains(w).
                    assert(adj_after_delete[u].contains(w));
                    assert(old_adj[u].contains(w));
                    // Old graph closure gives old_dom.contains(w). w≠v@ ⟹ dom.contains(w).
                    assert(old_dom.contains(w));
                };
            }
            updated
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            // Clone v with view equality proof for the neighbor set.
            let vc = v.clone_plus();
            proof { lemma_cloned_view_eq::<V>(v, vc); }
            let neighbors = match self.adj.find_ref(&u) {
                Some(ns_ref) => {
                    proof {
                        // Capacity: graph closure ⇒ ns@ ⊆ domain ⇒ ns@.len() ≤ dom.len().
                        let dom = self.spec_adj().dom();
                        assert(ns_ref@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] ns_ref@.contains(w) implies dom.contains(w) by {
                                assert(self.spec_adj().index(u@).contains(w));
                            };
                        };
                        lemma_entries_to_map_finite::<V::V, Set<V::V>>(self.adj.entries@);
                        vstd::set_lib::lemma_len_subset(ns_ref@, dom);
                    }
                    ns_ref.clone_wf().insert(vc)
                }
                None => AVLTreeSetStPer::singleton(vc),
            };
            let ghost neighbors_view = neighbors@;
            proof {
                assert(neighbors_view.subset_of(old_dom.insert(v_view))) by {
                    assert forall|w: <V as View>::V| #[trigger] neighbors_view.contains(w)
                        implies old_dom.insert(v_view).contains(w)
                    by {
                        if w != v_view {
                            assert(old_adj.dom().contains(u_view));
                            assert(old_adj.index(u_view).contains(w));
                        }
                    };
                };
            }
            let new_adj = self.adj.insert(u, neighbors,
                |_old: &AVLTreeSetStPer<V>, new: &AVLTreeSetStPer<V>| -> (r: AVLTreeSetStPer<V>)
                    ensures r@ == new@
                { new.clone() });
            proof {
                assert(new_adj@.dom().contains(u_view));
                assert(new_adj@[u_view] == neighbors_view);
            }
            let final_adj = if new_adj.find_ref(&v).is_none() {
                let ghost pre_second = new_adj@;
                let result = new_adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone());
                proof {
                    assert(!pre_second.dom().contains(v_view));
                    assert(pre_second.dom().contains(u_view));
                    assert(u_view != v_view);
                    assert(result@[u_view] == pre_second[u_view]);
                }
                result
            } else {
                new_adj
            };
            let updated = AdjTableGraphStPer { adj: final_adj };
            proof {
                assert(updated.spec_adj().dom().contains(u_view));
                assert(updated.spec_adj().dom().contains(v_view));
                assert(neighbors_view.contains(v_view));
                assert(updated.spec_adj()[u_view].contains(v_view));
                // Graph closure: all neighbors of any vertex are in the domain.
                assert forall|x: <V as View>::V, w: <V as View>::V|
                    updated.spec_adj().dom().contains(x)
                    && #[trigger] updated.spec_adj().index(x).contains(w)
                    implies updated.spec_adj().dom().contains(w)
                by {
                    if x == u_view {
                        assert(neighbors_view.contains(w));
                        assert(old_dom.insert(v_view).contains(w));
                    } else {
                        assert(old_adj.dom().contains(x));
                        assert(old_adj.index(x).contains(w));
                    }
                };
                // Stored-value wf: proved via lemma_spec_stored_value_view.
                assert forall|k: <V as View>::V| #[trigger] updated.adj@.dom().contains(k) implies
                    updated.adj.spec_stored_value(k).spec_avltreesetstper_wf()
                by {
                    updated.adj.lemma_spec_stored_value_view(k);
                    if k == u_view {
                        assert(updated.adj@[u_view] == neighbors_view);
                        assert(updated.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                    } else if k == v_view && !old_dom.contains(v_view) {
                        // v@ was new: stored value is empty, which is wf.
                    } else {
                        assert(old_adj.dom().contains(k));
                        self.adj.lemma_spec_stored_value_view(k);
                        let old_sv = self.adj.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetstper_wf());
                        assert(updated.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                    }
                };
            }
            updated
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let ghost old_adj = self.spec_adj();
            if self.adj.find_ref(u).is_some() {
                let ns_ref = self.adj.find_ref(u).unwrap();
                let new_neighbors = ns_ref.clone_wf().delete(v);
                let ghost nn_view = new_neighbors@;
                let uc = u.clone_plus();
                proof { lemma_cloned_view_eq::<V>(*u, uc); }
                let new_adj = self.adj.insert(uc, new_neighbors,
                    |_old: &AVLTreeSetStPer<V>, new: &AVLTreeSetStPer<V>| -> (r: AVLTreeSetStPer<V>)
                        ensures r@ == new@
                    { new.clone() });
                let updated = AdjTableGraphStPer { adj: new_adj };
                proof {
                    assert(new_adj@[u_view] == nn_view);
                    assert(!updated.spec_adj()[u_view].contains(v_view));
                    // Graph closure: nn_view ⊆ old_adj[u@] ⊆ old_dom == dom.
                    assert forall|x: <V as View>::V, w: <V as View>::V|
                        updated.spec_adj().dom().contains(x)
                        && #[trigger] updated.spec_adj().index(x).contains(w)
                        implies updated.spec_adj().dom().contains(w)
                    by {
                        if x == u_view {
                            assert(old_adj.index(u_view).contains(w));
                        } else {
                            assert(old_adj.dom().contains(x));
                            assert(old_adj.index(x).contains(w));
                        }
                    };
                    // Stored-value wf: proved via lemma_spec_stored_value_view.
                    assert forall|k: <V as View>::V| #[trigger] updated.adj@.dom().contains(k) implies
                        updated.adj.spec_stored_value(k).spec_avltreesetstper_wf()
                    by {
                        updated.adj.lemma_spec_stored_value_view(k);
                        if k == u_view {
                            assert(new_adj@[u_view] == nn_view);
                            assert(updated.adj.spec_stored_value(k).tree@ =~= new_neighbors.tree@);
                        } else {
                            assert(old_adj.dom().contains(k));
                            self.adj.lemma_spec_stored_value_view(k);
                            let old_sv = self.adj.spec_stored_value(k);
                            assert(old_sv.spec_avltreesetstper_wf());
                            assert(updated.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                        }
                    };
                }
                updated
            } else {
                // u@ not in domain: return unchanged via adj clone (view-preserving).
                let cloned_adj = self.adj.clone();
                let updated = AdjTableGraphStPer { adj: cloned_adj };
                proof {
                    // Stored-value wf: clone preserves entries, so same stored values.
                    assert forall|k: <V as View>::V| #[trigger] updated.adj@.dom().contains(k) implies
                        updated.adj.spec_stored_value(k).spec_avltreesetstper_wf()
                    by {
                        updated.adj.lemma_spec_stored_value_view(k);
                        self.adj.lemma_spec_stored_value_view(k);
                        let old_sv = self.adj.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetstper_wf());
                        assert(updated.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                    };
                    // Graph closure: unchanged from old wf.
                    assert forall|x: <V as View>::V, w: <V as View>::V|
                        updated.spec_adj().dom().contains(x)
                        && #[trigger] updated.spec_adj().index(x).contains(w)
                        implies updated.spec_adj().dom().contains(w)
                    by {
                        assert(old_adj.dom().contains(x));
                        assert(old_adj.index(x).contains(w));
                    };
                }
                updated
            }
        }
    }

    } // verus!
}
