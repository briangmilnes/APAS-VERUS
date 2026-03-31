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
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
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
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

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

    /// Sequential sum of entry value-set sizes: entries[0].1.len() + ... + entries[n-1].1.len().
    pub open spec fn spec_sum_entry_sizes<VV>(entries: Seq<(VV, Set<VV>)>, n: int) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { entries[n - 1].1.len() + spec_sum_entry_sizes(entries, n - 1) }
    }

    // 7. proof fns

    /// Extract any key from the recursive sum: decompose at k regardless of choose() order.
    pub proof fn lemma_sum_adj_remove<VV>(m: Map<VV, Set<VV>>, k: VV)
        requires m.dom().finite(), m.dom().contains(k)
        ensures spec_sum_adj_sizes(m) == m[k].len() + spec_sum_adj_sizes(m.remove(k))
        decreases m.dom().len()
    {
        let chosen = m.dom().choose();
        if chosen == k {
            // Definition picks k directly.
        } else {
            // Definition picks chosen != k.
            // IH on m.remove(chosen) extracting k:
            lemma_sum_adj_remove(m.remove(chosen), k);
            // IH on m.remove(k) extracting chosen:
            lemma_sum_adj_remove(m.remove(k), chosen);
            // Commutativity of remove.
            assert(m.remove(chosen).remove(k) =~= m.remove(k).remove(chosen));
        }
    }

    /// Connect sequential entry sum to recursive map sum.
    pub proof fn lemma_sum_entry_sizes_eq<VV>(entries: Seq<(VV, Set<VV>)>, n: int)
        requires
            0 <= n <= entries.len(),
            spec_keys_no_dups(entries),
        ensures
            spec_sum_entry_sizes(entries, n) == spec_sum_adj_sizes(
                spec_entries_to_map(entries.subrange(0, n)))
        decreases n
    {
        lemma_entries_to_map_finite::<VV, Set<VV>>(entries.subrange(0, n));
        if n == 0 {
            assert(entries.subrange(0, 0) =~= Seq::<(VV, Set<VV>)>::empty());
        } else {
            // Establish no-dup for the subrange.
            let sub_n = entries.subrange(0, n);
            let sub_prev = entries.subrange(0, n - 1);
            assert(spec_keys_no_dups::<VV, Set<VV>>(sub_n)) by {
                assert forall|i: int, j: int|
                    0 <= i < j < sub_n.len()
                    implies (#[trigger] sub_n[i]).0 != (#[trigger] sub_n[j]).0
                by {
                    assert(entries[i].0 != entries[j].0);
                };
            };
            assert(spec_keys_no_dups::<VV, Set<VV>>(sub_prev)) by {
                assert forall|i: int, j: int|
                    0 <= i < j < sub_prev.len()
                    implies (#[trigger] sub_prev[i]).0 != (#[trigger] sub_prev[j]).0
                by {
                    assert(entries[i].0 != entries[j].0);
                };
            };

            // IH for n-1.
            lemma_sum_entry_sizes_eq(entries, n - 1);
            // Now: spec_sum_entry_sizes(entries, n-1) == spec_sum_adj_sizes(spec_entries_to_map(sub_prev))

            let prefix_map = spec_entries_to_map(sub_prev);
            let full_map = spec_entries_to_map(sub_n);
            let key = entries[n - 1].0;
            let val = entries[n - 1].1;

            // sub_n == sub_prev.push((key, val))
            assert(sub_n =~= sub_prev.push((key, val)));
            // So full_map == prefix_map.insert(key, val)

            // key not in prefix_map (no dup keys).
            assert forall|idx: int| 0 <= idx < sub_prev.len()
                implies (#[trigger] sub_prev[idx]).0 != key
            by {
                // entries[idx].0 != entries[n-1].0 because idx < n-1 < n and no dups.
            };
            lemma_entries_to_map_no_key::<VV, Set<VV>>(sub_prev, key);

            // full_map == prefix_map.insert(key, val) by spec_entries_to_map definition.
            assert(sub_n.drop_last() =~= sub_prev);
            assert(sub_n.last() == (key, val));

            // Extract key from full_map sum.
            lemma_entries_to_map_finite::<VV, Set<VV>>(sub_n);
            lemma_entries_to_map_contains_key::<VV, Set<VV>>(sub_n, n - 1);
            lemma_sum_adj_remove(full_map, key);

            // full_map[key] == val.
            lemma_entries_to_map_get::<VV, Set<VV>>(sub_n, n - 1);

            // full_map.remove(key) =~= prefix_map: insert then remove with fresh key.
            lemma_entries_to_map_finite::<VV, Set<VV>>(sub_prev);
            assert(full_map =~= prefix_map.insert(key, val));
            assert(!prefix_map.dom().contains(key));
            assert(prefix_map.insert(key, val).remove(key) =~= prefix_map);
        }
    }

    /// Partial sums are monotonically non-decreasing.
    pub proof fn lemma_sum_entry_sizes_monotone<VV>(entries: Seq<(VV, Set<VV>)>, i: int, j: int)
        requires 0 <= i <= j <= entries.len()
        ensures spec_sum_entry_sizes(entries, i) <= spec_sum_entry_sizes(entries, j)
        decreases j - i
    {
        if i < j {
            lemma_sum_entry_sizes_monotone(entries, i, j - 1);
        }
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; table size
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — DIFFERS: APAS assumes cached; impl sums degrees sequentially
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphsteph_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// - APAS: Work Theta(|V|), Span Theta(|V|) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(|V|), Span Theta(|V|) — agrees; builds set from domain.
        fn vertices(&self) -> (verts: AVLTreeSetStEph<V>)
            requires
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().len() < usize::MAX as nat,
            ensures verts@ == self.spec_adj().dom();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS; table find + set find
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphsteph_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n + d_g(v)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS; table find returns neighbor set
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            requires self.spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS; table find + set len
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
            requires
                old(self).spec_adjtablegraphsteph_wf(),
                old(self).spec_adj().dom().len() + 1 < usize::MAX as nat,
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
                // keys_no_dups: from table.spec_tablesteph_wf() in requires.
                // stored-value wf: from quantifier in requires.
                // graph closure: from quantifier in requires.
                // Type predicates: from requires + broadcast triggers above.
            }
            out
        }

        fn num_vertices(&self) -> usize { self.adj.size() }

        fn num_edges(&self) -> (m: usize) {
            proof {
                reveal(obeys_view_eq);
                lemma_entries_to_map_len::<V::V, Set<V::V>>(self.adj.entries@);
                // Establish total equality: seq sum == map sum == spec_num_edges.
                lemma_sum_entry_sizes_eq::<V::V>(self.adj.entries@, self.adj.entries@.len() as int);
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
                decreases len - i,
            {
                let pair: &Pair<V, AVLTreeSetStEph<V>> = self.adj.entries.nth(i);
                proof {
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
                    forall|k: <V as View>::V| #[trigger] verts@.contains(k) ==>
                        exists|j: int| 0 <= j < i && (#[trigger] self.adj.entries@[j]).0 == k,
                decreases len - i,
            {
                let pair: &Pair<V, AVLTreeSetStEph<V>> = self.adj.entries.nth(i);
                let key: V = pair.0.clone_plus();
                proof {
                    lemma_cloned_view_eq::<V>(pair.0, key);
                }
                let ghost old_verts = verts@;
                verts.insert(key);
                proof {
                    assert forall|k: <V as View>::V| #[trigger] verts@.contains(k)
                        implies exists|j: int| 0 <= j < i + 1 && (#[trigger] self.adj.entries@[j]).0 == k
                    by {
                        if old_verts.contains(k) {
                            let j = choose|j: int| 0 <= j < i && (#[trigger] self.adj.entries@[j]).0 == k;
                            assert(self.adj.entries@[j].0 == k);
                        } else {
                            assert(k == key@);
                            assert(self.adj.entries@[i as int].0 == key@);
                        }
                    };
                }
                i = i + 1;
            }
            proof {
                // dom → verts: every key in the map domain is in verts.
                assert forall|k: <V as View>::V| #[trigger] self.spec_adj().dom().contains(k)
                    implies verts@.contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<V::V, Set<V::V>>(self.adj.entries@, k);
                    let j = choose|j: int| 0 <= j < self.adj.entries@.len() && (#[trigger] self.adj.entries@[j]).0 == k;
                    assert(verts@.contains(self.adj.entries@[j].0));
                };
                // verts → dom: every key in verts came from an entry, which is in the map.
                assert forall|k: <V as View>::V| #[trigger] verts@.contains(k)
                    implies self.spec_adj().dom().contains(k)
                by {
                    let j = choose|j: int| 0 <= j < len && (#[trigger] self.adj.entries@[j]).0 == k;
                    lemma_entries_to_map_contains_key::<V::V, Set<V::V>>(self.adj.entries@, j);
                };
                assert(verts@ =~= self.spec_adj().dom());
            }
            verts
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => {
                    neighbors.find(v)
                }
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty()
        {
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
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(u)
                    && #[trigger] self.spec_adj().index(u).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    if u != v@ {
                        // adj[u] == old_adj[u] (view preserved), old closure gives old_dom.contains(w).
                        assert(old_adj.dom().contains(u));
                        assert(old_adj.index(u).contains(w));
                    } else if !old_dom.contains(v@) {
                        // v@ was new: adj[v@] == Set::empty(), no w exists.
                    } else {
                        // v@ existed: adj[v@] == old.clone()@ == old_adj[v@] (from combine ensures).
                        assert(old_adj.dom().contains(v@));
                        assert(old_adj.index(v@).contains(w));
                    }
                };
                // Stored-value wf: use lemma_spec_stored_value_view to connect
                // spec_stored_value(k)@ to self@[k], then view preservation proves wf.
                let ghost old_self_adj = old(self).adj;
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    let sv = self.adj.spec_stored_value(k);
                    // Connect sv@ to self.adj@[k].
                    self.adj.lemma_spec_stored_value_view(k);
                    if k != v@ {
                        // View preserved: self.adj@[k] == old_adj[k].
                        assert(old_adj.dom().contains(k));
                        assert(self.adj@[k] == old_adj[k]);
                        // Connect old stored value view to old_adj[k].
                        old_self_adj.lemma_spec_stored_value_view(k);
                        let old_sv = old_self_adj.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetsteph_wf());
                        // sv@ == self.adj@[k] == old_adj[k] == old_sv@.
                        // Since AVLTreeSetStEph@ = self.tree@:
                        // sv.tree@ == sv@ == old_sv@ == old_sv.tree@.
                        // old_sv.tree@.finite() && len < MAX from old wf.
                        assert(sv.tree@ =~= old_sv.tree@);
                    } else if !old_dom.contains(v@) {
                        // New key: spec_stored_value(v@) == empty, which is wf.
                    } else {
                        // Existing key: combine returned old.clone() with same view.
                        assert(old_adj.dom().contains(v@));
                        old_self_adj.lemma_spec_stored_value_view(v@);
                        let old_sv = old_self_adj.spec_stored_value(v@);
                        assert(old_sv.spec_avltreesetsteph_wf());
                    }
                };
            }
        }

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
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    self.adj.lemma_spec_stored_value_view(k);
                    assert(old_adj.dom().contains(k));
                    old_self_adj.lemma_spec_stored_value_view(k);
                    let old_sv = old_self_adj.spec_stored_value(k);
                    assert(old_sv.spec_avltreesetsteph_wf());
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
                proof {
                    lemma_cloned_view_eq::<V>(*nth_ref, u);
                    seq.lemma_view_index(i as int);
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
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                        by {
                            self.adj.lemma_spec_stored_value_view(k);
                            if k == u@ {
                                assert(self.adj@[u@] == neighbors_view);
                                assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                            } else {
                                assert(pre_insert.dom().contains(k));
                                pre_insert_adj.lemma_spec_stored_value_view(k);
                                let old_sv = pre_insert_adj.spec_stored_value(k);
                                assert(old_sv.spec_avltreesetsteph_wf());
                                assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                            }
                        };
                        // Domain unchanged.
                        assert(self.adj@.dom() =~= adj_after_delete.dom());
                        // Subset invariant.
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj@[k].subset_of(adj_after_delete[k])
                        by {
                            if k == u@ {
                                assert(neighbors_view.subset_of(pre_insert[u@]));
                                assert(pre_insert[u@].subset_of(adj_after_delete[u@]));
                            } else {
                                assert(self.adj@[k] == pre_insert[k]);
                            }
                        };
                        // v-removal invariant.
                        assert(!neighbors_view.contains(v@));
                        // Helper: for k != u@ in post-insert domain, value unchanged from pre_insert.
                        assert(pre_insert.dom() =~= adj_after_delete.dom());
                        assert forall|k: <V as View>::V| k != u@ && #[trigger] self.adj@.dom().contains(k) implies
                            pre_insert.dom().contains(k) && self.adj@[k] == pre_insert[k]
                        by {
                            assert(adj_after_delete.dom().contains(k));
                            assert(pre_insert.dom().contains(k));
                        };
                        assert forall|j: int| #![trigger seq@[j]] 0 <= j < (i + 1) as int && self.adj@.dom().contains(seq@[j]) implies
                            !self.adj@[seq@[j]].contains(v@)
                        by {
                            if j == i as int {
                                assert(self.adj@[u@] == neighbors_view);
                            } else if self.adj@.dom().contains(seq@[j]) {
                                // j < i. no_duplicates ⟹ seq@[j] != seq@[i] == u@.
                                assert(seq@[j] != u@);
                                // Helper: value preserved for non-u@ keys.
                                assert(self.adj@[seq@[j]] == pre_insert[seq@[j]]);
                                // Old invariant at pre-insert: !pre_insert[seq@[j]].contains(v@).
                                assert(pre_insert.dom().contains(seq@[j]));
                                assert(!pre_insert[seq@[j]].contains(v@));
                            }
                        };
                    }
                }
                i += 1;
            }
            proof {
                // v@ removed from ALL neighbor sets (loop processed entire domain).
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    !self.adj@[k].contains(v@)
                by {
                    // k ∈ dom ⊆ old_dom = seq@.to_set(). So seq@ contains k.
                    assert(old_dom.contains(k));
                    assert(seq@.to_set().contains(k));
                    // Seq::to_set().contains ⟹ Seq::contains ⟹ ∃j: seq@[j]==k.
                    // Loop invariant (i==len) gives !adj[k].contains(v@).
                };
                // Graph closure: for any u,w with adj[u].contains(w), w is in the domain.
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(u)
                    && #[trigger] self.spec_adj().index(u).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    // !adj[u].contains(v@) (from above), but adj[u].contains(w), so w≠v@.
                    assert(!self.adj@[u].contains(v@));
                    assert(w != v@);
                    // adj[u] ⊆ adj_after_delete[u] == old_adj[u], so old_adj[u].contains(w).
                    assert(adj_after_delete[u].contains(w));
                    assert(old_adj[u].contains(w));
                    // Old graph closure gives old_dom.contains(w). w≠v@ ⟹ dom.contains(w).
                    assert(old_dom.contains(w));
                };
            }
        }

        fn insert_edge(&mut self, u: V, v: V) {
            proof { reveal(obeys_view_eq); }
            let ghost u_view: <V as View>::V = u@;
            let ghost v_view: <V as View>::V = v@;
            let ghost old_adj = self.spec_adj();
            let ghost old_dom = old_adj.dom();
            // Clone v with view equality proof for the neighbor set.
            let vc = v.clone_plus();
            proof { lemma_cloned_view_eq::<V>(v, vc); }
            // Build new neighbor set for u: old neighbors + vc, or singleton(vc).
            let neighbors = match self.adj.find_ref(&u) {
                Some(ns_ref) => {
                    let mut ns = ns_ref.clone_wf();
                    proof {
                        // Capacity: graph closure ⇒ ns@ ⊆ domain ⇒ ns@.len() ≤ dom.len().
                        let dom = self.spec_adj().dom();
                        assert(ns@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] ns@.contains(w) implies dom.contains(w) by {
                                assert(self.spec_adj().index(u@).contains(w));
                            };
                        };
                        lemma_entries_to_map_finite::<V::V, Set<V::V>>(self.adj.entries@);
                        vstd::set_lib::lemma_len_subset(ns@, dom);
                    }
                    ns.insert(vc);
                    ns
                }
                None => AVLTreeSetStEph::singleton(vc),
            };
            let ghost neighbors_view = neighbors@;
            // neighbors_view ⊆ old_dom ∪ {v@}: elements are from old_adj[u@] (⊆ old_dom) plus v@.
            proof {
                assert(neighbors_view.subset_of(old_dom.insert(v_view))) by {
                    assert forall|w: <V as View>::V| #[trigger] neighbors_view.contains(w)
                        implies old_dom.insert(v_view).contains(w)
                    by {
                        if w != v_view {
                            // w was in old_adj[u@] (from graph closure).
                            assert(old_adj.dom().contains(u_view));
                            assert(old_adj.index(u_view).contains(w));
                        }
                    };
                };
            }
            self.adj.insert(u, neighbors,
                |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                    ensures r@ == new@
                { new.clone() });
            // First insert ensures: dom contains u@, and adj[u@] == neighbors_view.
            proof {
                assert(self.adj@.dom().contains(u_view));
                assert(self.adj@[u_view] == neighbors_view);
            }
            let ghost adj_after_first = self.adj@;
            // Ensure v is in the domain.
            if self.adj.find_ref(&v).is_none() {
                let ghost pre_second = self.adj@;
                self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
                proof {
                    assert(!pre_second.dom().contains(v_view));
                    assert(pre_second.dom().contains(u_view));
                    assert(u_view != v_view);
                    assert(self.adj@[u_view] == pre_second[u_view]);
                }
            }
            proof {
                assert(self.spec_adj().dom().contains(u_view));
                assert(self.spec_adj().dom().contains(v_view));
                assert(neighbors_view.contains(v_view));
                assert(self.spec_adj()[u_view].contains(v_view));
                // Graph closure: all neighbors of any vertex are in the domain.
                assert forall|x: <V as View>::V, w: <V as View>::V|
                    self.spec_adj().dom().contains(x)
                    && #[trigger] self.spec_adj().index(x).contains(w)
                    implies self.spec_adj().dom().contains(w)
                by {
                    if x == u_view {
                        // adj[u@] == neighbors_view ⊆ old_dom ∪ {v@} ⊆ final_dom.
                        assert(neighbors_view.contains(w));
                        assert(old_dom.insert(v_view).contains(w));
                    } else {
                        // adj[x] == old_adj[x] (view preserved through both inserts).
                        assert(old_adj.dom().contains(x));
                        assert(old_adj.index(x).contains(w));
                    }
                };
                // Stored-value wf: proved via lemma_spec_stored_value_view.
                let ghost old_self_adj_ie = old(self).adj;
                assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                    self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                by {
                    self.adj.lemma_spec_stored_value_view(k);
                    if k == u_view {
                        // spec_stored_value(u@)@ == self.adj@[u@] == neighbors_view == neighbors@.
                        // neighbors was built from clone_wf + insert (or singleton), both wf.
                        assert(self.adj@[u_view] == neighbors_view);
                        assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                    } else if k == v_view && !old_dom.contains(v_view) {
                        // v@ was new: spec_stored_value(v@)@ == empty@.
                    } else {
                        // View preserved: self.adj@[k] == old_adj[k].
                        assert(old_adj.dom().contains(k));
                        old_self_adj_ie.lemma_spec_stored_value_view(k);
                        let old_sv = old_self_adj_ie.spec_stored_value(k);
                        assert(old_sv.spec_avltreesetsteph_wf());
                        assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                    }
                };
            }
        }

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
                    proof { assert(!neighbors_view.contains(v_view)); }
                    let uc = u.clone_plus();
                    proof { lemma_cloned_view_eq::<V>(*u, uc); }
                    self.adj.insert(uc, neighbors,
                        |_old: &AVLTreeSetStEph<V>, new: &AVLTreeSetStEph<V>| -> (r: AVLTreeSetStEph<V>)
                            ensures r@ == new@
                        { new.clone() });
                    proof {
                        assert(self.adj@[u_view] == neighbors_view);
                        assert(!self.spec_adj()[u_view].contains(v_view));
                        // Graph closure: neighbors_view ⊆ old_adj[u@] ⊆ old_dom == dom.
                        assert forall|x: <V as View>::V, w: <V as View>::V|
                            self.spec_adj().dom().contains(x)
                            && #[trigger] self.spec_adj().index(x).contains(w)
                            implies self.spec_adj().dom().contains(w)
                        by {
                            if x == u_view {
                                // adj[u@] = old_adj[u@].remove(v@) ⊆ old_adj[u@].
                                assert(old_adj.index(u_view).contains(w));
                            } else {
                                assert(old_adj.dom().contains(x));
                                assert(old_adj.index(x).contains(w));
                            }
                        };
                        // Stored-value wf: proved via lemma_spec_stored_value_view.
                        let ghost old_self_adj_de = old(self).adj;
                        assert forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) implies
                            self.adj.spec_stored_value(k).spec_avltreesetsteph_wf()
                        by {
                            self.adj.lemma_spec_stored_value_view(k);
                            if k == u_view {
                                // spec_stored_value(u@)@ == self.adj@[u@] == neighbors_view.
                                // neighbors was clone_wf + delete, both wf-preserving.
                                assert(self.adj@[u_view] == neighbors_view);
                                assert(self.adj.spec_stored_value(k).tree@ =~= neighbors.tree@);
                            } else {
                                // View preserved: self.adj@[k] == old_adj[k].
                                assert(old_adj.dom().contains(k));
                                old_self_adj_de.lemma_spec_stored_value_view(k);
                                let old_sv = old_self_adj_de.spec_stored_value(k);
                                assert(old_sv.spec_avltreesetsteph_wf());
                                assert(self.adj.spec_stored_value(k).tree@ =~= old_sv.tree@);
                            }
                        };
                    }
                }
            }
            // No-mutation branch: postcondition from old wf (u@ not in domain).
        }
    }

    } // verus!
}
