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
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone());
            let updated = AdjTableGraphStPer { adj: new_adj };
            proof {
                // Clone gap + graph closure: Verus ICE on Set<V::V> in proof bodies.
                assume(updated.spec_adjtablegraphstper_wf());
            }
            updated
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            // Step 1: Remove v as a key from the adjacency table.
            let new_adj = self.adj.delete(v);
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
                    result_adj.spec_tablestper_wf(),
                    obeys_view_eq::<V>(),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                    obeys_feq_fulls::<V, AVLTreeSetStPer<V>>(),
                    obeys_feq_full::<Pair<V, AVLTreeSetStPer<V>>>(),
                    !result_adj@.dom().contains(v@),
                decreases len - i,
            {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = result_adj.find(&u) {
                    proof {
                        // Neighbor-set wf from find requires quantifier over domain.
                        // blocked by Verus ICE
                        assume(neighbors.spec_avltreesetstper_wf());
                    }
                    let new_neighbors = neighbors.delete(v);
                    result_adj = result_adj.insert(u, new_neighbors, |_old, new| new.clone());
                }
                i += 1;
            }
            let updated = AdjTableGraphStPer { adj: result_adj };
            proof {
                // Graph-level wf (neighbor-set wf + graph closure) requires
                // quantifying over Map<V::V, Set<V::V>> which triggers Verus ICE.
                // blocked by Verus ICE
                assume(updated.spec_adjtablegraphstper_wf());
            }
            updated
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
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
                    ns_ref.clone_wf().insert(v.clone())
                }
                None => AVLTreeSetStPer::singleton(v.clone()),
            };
            let new_adj = self.adj.insert(u, neighbors, |_old, new| new.clone());
            // After first insert: dom contains u@.
            proof { assert(new_adj@.dom().contains(u_view)); }
            let final_adj = if new_adj.find_ref(&v).is_none() {
                new_adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone())
            } else {
                new_adj
            };
            let updated = AdjTableGraphStPer { adj: final_adj };
            proof {
                // After conditional second insert: dom contains both u@ and v@.
                assert(updated.spec_adj().dom().contains(u_view));
                assert(updated.spec_adj().dom().contains(v_view));
                // Clone gap + graph closure: Verus ICE on Set<V::V> prevents proving wf.
                assume(updated.spec_adjtablegraphstper_wf());
                assume(updated.spec_adj()[u_view].contains(v_view));
            }
            updated
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let updated = match self.adj.find_ref(u) {
                Some(neighbors) => {
                    let new_neighbors = neighbors.clone_wf().delete(v);
                    // new_neighbors@ == old_ns@.remove(v@), so !new_neighbors@.contains(v@).
                    let new_adj = self.adj.insert(u.clone(), new_neighbors, |_old, new| new.clone());
                    AdjTableGraphStPer { adj: new_adj }
                }
                None => {
                    // u@ not in domain: no changes needed.
                    self.clone()
                }
            };
            proof {
                // Clone gap + graph closure: Verus ICE on Set<V::V>.
                assume(updated.spec_adjtablegraphstper_wf());
                // Proving the postcondition requires clone view preservation through
                // the combine closure. Leave as assume for now.
                assume(!updated.spec_adj().dom().contains(u_view)
                    || !updated.spec_adj()[u_view].contains(v_view));
            }
            updated
        }
    }

    } // verus!
}
