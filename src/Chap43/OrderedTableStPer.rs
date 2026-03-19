//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table backed by AVLTreeSetStPer<Pair<K, V>>.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

   // Veracity: added broadcast group
   broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
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
    // 10. iterators
    // 12. derive impls in verus!
    // 13. macros

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT + Ord> {
        pub base_set: AVLTreeSetStPer<Pair<K, V>>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStPer<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_entries_to_map(self.base_set.elements@) }
    }

    // 6. spec fns

    proof fn lemma_keys_no_dups_implies_no_duplicates<KV, VV>(entries: Seq<(KV, VV)>)
        requires spec_keys_no_dups(entries),
        ensures entries.no_duplicates(),
    {
        assert forall|i: int, j: int| 0 <= i < j < entries.len()
            implies entries[i] != entries[j]
        by {
            assert(entries[i].0 != entries[j].0);
        };
    }

    // 7. proof fns

    /// When two sequences have the same to_set, both have no duplicate pairs, and one has
    /// unique keys, then the other also has unique keys.
    proof fn lemma_keys_no_dups_preserved_by_set_eq<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
    )
        requires
            spec_keys_no_dups(s1),
            s1.no_duplicates(),
            s2.no_duplicates(),
            s1.to_set() =~= s2.to_set(),
        ensures
            spec_keys_no_dups(s2),
    {
        assert forall|i: int, j: int| 0 <= i < j < s2.len()
            implies (#[trigger] s2[i]).0 != (#[trigger] s2[j]).0
        by {
            if s2[i].0 == s2[j].0 {
                // s2[i] is in s2.to_set() = s1.to_set()
                assert(s2.to_set().contains(s2[i]));
                assert(s1.to_set().contains(s2[i]));
                assert(s1.contains(s2[i]));
                let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[i];
                // s2[j] is in s2.to_set() = s1.to_set()
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().contains(s2[j]));
                assert(s1.contains(s2[j]));
                let q = choose|q: int| 0 <= q < s1.len() && s1[q] == s2[j];
                // s1[p] = s2[i] != s2[j] = s1[q], so p != q
                assert(s1[p] != s1[q]);
                if p < q {
                    assert(s1[p].0 != s1[q].0);
                } else {
                    assert(s1[q].0 != s1[p].0);
                }
            }
        };
    }

    /// When s2.to_set() is s1.to_set() with one pair removed, and s1 has unique keys,
    /// then s2 also has unique keys.
    proof fn lemma_keys_no_dups_after_set_remove<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
        removed: (KV, VV),
    )
        requires
            spec_keys_no_dups(s1),
            s1.no_duplicates(),
            s2.no_duplicates(),
            s2.to_set() =~= s1.to_set().remove(removed),
        ensures
            spec_keys_no_dups(s2),
    {
        assert forall|i: int, j: int| 0 <= i < j < s2.len()
            implies (#[trigger] s2[i]).0 != (#[trigger] s2[j]).0
        by {
            if s2[i].0 == s2[j].0 {
                assert(s2.to_set().contains(s2[i]));
                assert(s1.to_set().remove(removed).contains(s2[i]));
                assert(s1.to_set().contains(s2[i]));
                assert(s1.contains(s2[i]));
                let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[i];
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().remove(removed).contains(s2[j]));
                assert(s1.to_set().contains(s2[j]));
                assert(s1.contains(s2[j]));
                let q = choose|q: int| 0 <= q < s1.len() && s1[q] == s2[j];
                assert(s1[p] != s1[q]);
                if p < q {
                    assert(s1[p].0 != s1[q].0);
                } else {
                    assert(s1[q].0 != s1[p].0);
                }
            }
        };
    }

    /// When s2 is s1 with pair (k,v) removed from the set representation,
    /// the resulting map is the original map with key k removed.
    proof fn lemma_entries_to_map_after_remove_pair<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
        pair: (KV, VV),
    )
        requires
            spec_keys_no_dups(s1),
            spec_keys_no_dups(s2),
            s1.no_duplicates(),
            s2.no_duplicates(),
            s1.to_set().contains(pair),
            s2.to_set() =~= s1.to_set().remove(pair),
        ensures
            spec_entries_to_map(s2) =~= spec_entries_to_map(s1).remove(pair.0),
    {
        let map1 = spec_entries_to_map(s1);
        let map2 = spec_entries_to_map(s2);
        let k = pair.0;
        // Show k not in dom(map2): if it were, the unique entry with key k in s1 is pair,
        // but pair is removed from s2.to_set(), contradiction.
        assert(!map2.contains_key(k)) by {
            if map2.contains_key(k) {
                lemma_entries_to_map_key_in_seq::<KV, VV>(s2, k);
                let j = choose|j: int| 0 <= j < s2.len() && (#[trigger] s2[j]).0 == k;
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().remove(pair).contains(s2[j]));
                assert(s2[j] != pair);
                assert(s1.to_set().contains(s2[j]));
                assert(s1.contains(s2[j]));
                let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[j];
                assert(s1.to_set().contains(pair));
                assert(s1.contains(pair));
                let q = choose|q: int| 0 <= q < s1.len() && s1[q] == pair;
                assert(s1[p].0 == k);
                assert(s1[q].0 == k);
                if p < q { assert(s1[p].0 != s1[q].0); }
                else if q < p { assert(s1[q].0 != s1[p].0); }
                assert(p == q);
                assert(false);
            }
        };
        // Domain equality: map2.dom() == map1.remove(k).dom()
        assert forall|k2: KV| map2.dom().contains(k2) <==>
            #[trigger] map1.remove(k).dom().contains(k2) by {
            if map2.dom().contains(k2) {
                assert(k2 != k);
                lemma_entries_to_map_key_in_seq::<KV, VV>(s2, k2);
                let j = choose|j: int| 0 <= j < s2.len() && (#[trigger] s2[j]).0 == k2;
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().contains(s2[j]));
                assert(s1.contains(s2[j]));
                let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[j];
                lemma_entries_to_map_contains_key::<KV, VV>(s1, p);
            }
            if map1.remove(k).dom().contains(k2) {
                lemma_entries_to_map_key_in_seq::<KV, VV>(s1, k2);
                let p = choose|p: int| 0 <= p < s1.len() && (#[trigger] s1[p]).0 == k2;
                assert(s1[p] != pair);
                assert(s1.to_set().contains(s1[p]));
                assert(s1.to_set().remove(pair).contains(s1[p]));
                assert(s2.to_set().contains(s1[p]));
                assert(s2.contains(s1[p]));
                let j = choose|j: int| 0 <= j < s2.len() && s2[j] == s1[p];
                lemma_entries_to_map_contains_key::<KV, VV>(s2, j);
            }
        };
        // Value equality for keys in map2
        assert forall|k2: KV| map2.dom().contains(k2) implies
            map2[k2] == #[trigger] map1.remove(k)[k2] by {
            lemma_entries_to_map_key_in_seq::<KV, VV>(s2, k2);
            let j = choose|j: int| 0 <= j < s2.len() && (#[trigger] s2[j]).0 == k2;
            assert(s2.to_set().contains(s2[j]));
            assert(s1.to_set().contains(s2[j]));
            assert(s1.contains(s2[j]));
            let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[j];
            lemma_entries_to_map_get::<KV, VV>(s1, p);
            lemma_entries_to_map_get::<KV, VV>(s2, j);
        };
    }

    /// After inserting a pair whose key is absent from s1, the result has unique keys.
    proof fn lemma_keys_no_dups_after_set_insert<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
        pair: (KV, VV),
    )
        requires
            spec_keys_no_dups(s1),
            s1.no_duplicates(),
            s2.no_duplicates(),
            !spec_entries_to_map(s1).contains_key(pair.0),
            s2.to_set() =~= s1.to_set().insert(pair),
        ensures
            spec_keys_no_dups(s2),
    {
        assert forall|i: int, j: int| 0 <= i < j < s2.len()
            implies (#[trigger] s2[i]).0 != (#[trigger] s2[j]).0
        by {
            if s2[i].0 == s2[j].0 {
                assert(s2.to_set().contains(s2[i]));
                assert(s1.to_set().insert(pair).contains(s2[i]));
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().insert(pair).contains(s2[j]));
                if s2[i] == pair && s2[j] == pair {
                    assert(false);
                } else if s2[i] == pair {
                    assert(s1.to_set().contains(s2[j]));
                    assert(s1.contains(s2[j]));
                    let q = choose|q: int| 0 <= q < s1.len() && s1[q] == s2[j];
                    lemma_entries_to_map_contains_key::<KV, VV>(s1, q);
                    assert(false);
                } else if s2[j] == pair {
                    assert(s1.to_set().contains(s2[i]));
                    assert(s1.contains(s2[i]));
                    let q = choose|q: int| 0 <= q < s1.len() && s1[q] == s2[i];
                    lemma_entries_to_map_contains_key::<KV, VV>(s1, q);
                    assert(false);
                } else {
                    assert(s1.to_set().contains(s2[i]));
                    assert(s1.contains(s2[i]));
                    let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[i];
                    assert(s1.to_set().contains(s2[j]));
                    assert(s1.contains(s2[j]));
                    let q = choose|q: int| 0 <= q < s1.len() && s1[q] == s2[j];
                    assert(s1[p] != s1[q]);
                    if p < q { assert(s1[p].0 != s1[q].0); }
                    else { assert(s1[q].0 != s1[p].0); }
                }
            }
        };
    }

    /// After set-inserting a pair, the map domain gains that key.
    proof fn lemma_entries_to_map_dom_after_insert<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
        pair: (KV, VV),
    )
        requires
            spec_keys_no_dups(s1),
            spec_keys_no_dups(s2),
            s1.no_duplicates(),
            s2.no_duplicates(),
            s2.to_set() =~= s1.to_set().insert(pair),
        ensures
            spec_entries_to_map(s2).dom() =~= spec_entries_to_map(s1).dom().insert(pair.0),
    {
        let map1 = spec_entries_to_map(s1);
        let map2 = spec_entries_to_map(s2);
        // pair is in s2, so pair.0 is in map2
        assert(s2.to_set().contains(pair));
        assert(s2.contains(pair));
        let j0 = choose|j: int| 0 <= j < s2.len() && s2[j] == pair;
        lemma_entries_to_map_contains_key::<KV, VV>(s2, j0);
        // Domain biconditional
        assert forall|k2: KV| map2.dom().contains(k2) <==>
            #[trigger] map1.dom().insert(pair.0).contains(k2)
        by {
            if map2.dom().contains(k2) {
                lemma_entries_to_map_key_in_seq::<KV, VV>(s2, k2);
                let j = choose|j: int| 0 <= j < s2.len() && (#[trigger] s2[j]).0 == k2;
                assert(s2.to_set().contains(s2[j]));
                assert(s1.to_set().insert(pair).contains(s2[j]));
                if s2[j] != pair {
                    assert(s1.to_set().contains(s2[j]));
                    assert(s1.contains(s2[j]));
                    let p = choose|p: int| 0 <= p < s1.len() && s1[p] == s2[j];
                    lemma_entries_to_map_contains_key::<KV, VV>(s1, p);
                }
            }
            if map1.dom().insert(pair.0).contains(k2) {
                if k2 != pair.0 {
                    assert(map1.dom().contains(k2));
                    lemma_entries_to_map_key_in_seq::<KV, VV>(s1, k2);
                    let p = choose|p: int| 0 <= p < s1.len() && (#[trigger] s1[p]).0 == k2;
                    assert(s1.to_set().contains(s1[p]));
                    assert(s1.to_set().insert(pair).contains(s1[p]));
                    assert(s2.to_set().contains(s1[p]));
                    assert(s2.contains(s1[p]));
                    let j = choose|j: int| 0 <= j < s2.len() && s2[j] == s1[p];
                    lemma_entries_to_map_contains_key::<KV, VV>(s2, j);
                }
            }
        };
    }

    /// When two sequences have the same to_set and both have unique keys,
    /// spec_entries_to_map produces the same map for both.
    proof fn lemma_entries_to_map_set_determines_map<KV, VV>(
        s1: Seq<(KV, VV)>,
        s2: Seq<(KV, VV)>,
    )
        requires
            spec_keys_no_dups(s1),
            spec_keys_no_dups(s2),
            s1.to_set() =~= s2.to_set(),
        ensures
            spec_entries_to_map(s1) =~= spec_entries_to_map(s2),
    {
        let map1 = spec_entries_to_map(s1);
        let map2 = spec_entries_to_map(s2);
        assert forall|k: KV| map1.contains_key(k) implies map2.contains_key(k) by {
            lemma_entries_to_map_key_in_seq(s1, k);
            let i = choose|i: int| 0 <= i < s1.len() && (#[trigger] s1[i]).0 == k;
            assert(s1.to_set().contains(s1[i]));
            assert(s2.to_set().contains(s1[i]));
            assert(s2.contains(s1[i]));
            let j = choose|j: int| 0 <= j < s2.len() && s2[j] == s1[i];
            lemma_entries_to_map_contains_key::<KV, VV>(s2, j);
        };
        assert forall|k: KV| map2.contains_key(k) implies map1.contains_key(k) by {
            lemma_entries_to_map_key_in_seq(s2, k);
            let i = choose|i: int| 0 <= i < s2.len() && (#[trigger] s2[i]).0 == k;
            assert(s2.to_set().contains(s2[i]));
            assert(s1.to_set().contains(s2[i]));
            assert(s1.contains(s2[i]));
            let j = choose|j: int| 0 <= j < s1.len() && s1[j] == s2[i];
            lemma_entries_to_map_contains_key::<KV, VV>(s1, j);
        };
        assert forall|k: KV| map1.contains_key(k) implies map1[k] == map2[k] by {
            lemma_entries_to_map_key_in_seq(s1, k);
            let i = choose|i: int| 0 <= i < s1.len() && (#[trigger] s1[i]).0 == k;
            lemma_entries_to_map_get::<KV, VV>(s1, i);
            assert(s1.to_set().contains(s1[i]));
            assert(s2.to_set().contains(s1[i]));
            assert(s2.contains(s1[i]));
            let j = choose|j: int| 0 <= j < s2.len() && s2[j] == s1[i];
            lemma_entries_to_map_get::<KV, VV>(s2, j);
        };
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablestper_wf(&self) -> bool;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- delegates to TableStPer.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- constructs empty TableStPer
        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- wraps TableStPer.singleton
        fn singleton(k: K, v: V) -> (table: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table@.dom().finite(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.find (linear scan)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.insert (linear dup check)
        fn insert(&self, k: K, v: V) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.delete (linear scan)
        fn delete(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures table@ == self@.remove(k@), table@.dom().finite(), table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.domain
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures keys@ =~= self@.dom(), self@.dom().finite();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n^2), Span Θ(n^2) -- delegates to TableStPer.tabulate (sequential insert loop)
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>(),
            ensures
                table@.dom() =~= keys@,
                table.spec_orderedtablestper_wf(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && table@[k] == result@),
                table@.dom().finite();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.map (linear iteration)
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), forall|v: &V| f.requires((v,)), obeys_feq_full::<K>(),
            ensures
                table@.dom() == self@.dom(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && table@[k] == result@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- delegates to TableStPer.filter (linear iteration)
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_full::<Pair<K, V>>(),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] table@.dom().contains(k),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.intersection (linear scan)
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<K>(),
            ensures
                table@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.union (linear merge)
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) && !other@.contains_key(k)
                    ==> table@[k] == self@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !self@.contains_key(k)
                    ==> table@[k] == other@[k],
                forall|k: K::V| #[trigger] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to TableStPer.difference (linear scan)
        fn difference(&self, other: &Self) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- linear scan over self for each key
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(n log n), Span Θ(n log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects entries and sorts by key
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), sorted_entries.spec_avltreeseqstper_wf(), sorted_entries@.len() == self@.dom().len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns first element
        fn first_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then returns last element
        fn last_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for predecessor
        fn previous_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then scans for successor
        fn next_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by key
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@;
        /// - APAS: Work Θ(m log(n/m + 1)), Span Θ(log n log m)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union (linear merge)
        fn join_key(left: &Self, right: &Self) -> (table: Self)
            requires
                left.spec_orderedtablestper_wf(),
                right.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                table@.dom() =~= left@.dom().union(right@.dom()),
                table@.dom().finite(),
                table.spec_orderedtablestper_wf();
        /// - APAS: Work Θ(log n + m), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then filters by range
        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().finite(),
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key];
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then counts elements < k
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then indexes
        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - APAS: Work Θ(log n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects then partitions by rank
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.1@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_orderedtablestper_wf(&self) -> bool {
            self.base_set.spec_avltreesetstper_wf()
            && spec_keys_no_dups(self.base_set.elements@)
        }

        fn size(&self) -> (count: usize)
        {
            let count = self.base_set.size();
            proof {
                self.base_set.elements@.unique_seq_to_set();
                lemma_entries_to_map_len::<K::V, V::V>(self.base_set.elements@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
            }
            count
        }

        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf()
        {
            let base = AVLTreeSetStPer::empty();
            proof {
                base.elements@.unique_seq_to_set();
            }
            OrderedTableStPer { base_set: base }
        }

        fn singleton(k: K, v: V) -> (table: Self)
        {
            let pair = Pair(k, v);
            let base = AVLTreeSetStPer::singleton(pair);
            // base@ == Set::empty().insert(Pair(k,v)@)
            // base.spec_avltreesetstper_wf()
            // base.elements@.to_set() == {(k@, v@)}
            // Need to show spec_entries_to_map(base.elements@) == Map::empty().insert(k@, v@)
            proof {
                // Build a known singleton sequence for comparison
                let singleton_seq = seq![(k@, v@)];
                // singleton_seq.to_set() == {(k@, v@)} == base@
                assert(singleton_seq.to_set() =~= base.elements@.to_set()) by {
                    assert(singleton_seq[0] == (k@, v@));
                    assert(singleton_seq.to_set().contains((k@, v@)));
                    assert(base.elements@.to_set().contains((k@, v@)));
                    assert forall|x: (K::V, V::V)| singleton_seq.to_set().contains(x)
                        implies base.elements@.to_set().contains(x)
                    by {
                        assert(x == (k@, v@));
                    };
                    assert forall|x: (K::V, V::V)| base.elements@.to_set().contains(x)
                        implies singleton_seq.to_set().contains(x)
                    by {
                        assert(base@.contains(x));
                        assert(x == (k@, v@));
                        assert(singleton_seq.contains(x));
                    };
                };
                // singleton_seq has unique keys (trivially, length 1)
                assert(spec_keys_no_dups(singleton_seq));
                // base.elements@ has no duplicate pairs (from wf)
                // base.elements@ has same to_set as singleton_seq
                lemma_keys_no_dups_implies_no_duplicates::<K::V, V::V>(singleton_seq);
                lemma_keys_no_dups_preserved_by_set_eq::<K::V, V::V>(
                    singleton_seq, base.elements@,
                );
                lemma_entries_to_map_set_determines_map::<K::V, V::V>(
                    singleton_seq, base.elements@,
                );
                // Unfold spec_entries_to_map for length-1 sequence
                assert(singleton_seq.len() == 1);
                assert(singleton_seq.last() == (k@, v@));
                let drop = singleton_seq.drop_last();
                assert(drop.len() == 0);
                assert(spec_entries_to_map(drop) =~= Map::<K::V, V::V>::empty());
                assert(spec_entries_to_map(singleton_seq) =~= Map::<K::V, V::V>::empty().insert(k@, v@));
                lemma_entries_to_map_finite::<K::V, V::V>(base.elements@);
            }
            OrderedTableStPer { base_set: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>)
        {
            let len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    forall|j: int| 0 <= j < i as int
                        ==> (#[trigger] self.base_set.elements@[j]).0 != k@,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0.eq(k) {
                    let v = pair.1.clone_plus();
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(
                            self.base_set.elements@, i as int,
                        );
                    }
                    return Some(v);
                }
                i += 1;
            }
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.base_set.elements@, k@);
            }
            None
        }

        fn insert(&self, k: K, v: V) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.spec_avltreesetstper_wf(),
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    forall|j: int| 0 <= j < i as int
                        ==> (#[trigger] self.base_set.elements@[j]).0 != k@,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0.eq(&k) {
                    // Found existing key — delete old pair, insert new pair.
                    let pair_clone = pair.clone_plus();
                    let deleted = self.base_set.delete(&pair_clone);
                    proof {
                        assert(self.base_set.elements@[i as int] == pair_clone@);
                        lemma_keys_no_dups_after_set_remove::<K::V, V::V>(
                            self.base_set.elements@,
                            deleted.elements@,
                            pair_clone@,
                        );
                        lemma_entries_to_map_after_remove_pair::<K::V, V::V>(
                            self.base_set.elements@,
                            deleted.elements@,
                            pair_clone@,
                        );
                        assert(pair_clone@.0 == k@);
                    }
                    let new_pair = Pair(k, v);
                    let inserted = deleted.insert(new_pair);
                    proof {
                        lemma_keys_no_dups_after_set_insert::<K::V, V::V>(
                            deleted.elements@,
                            inserted.elements@,
                            (k@, v@),
                        );
                        lemma_entries_to_map_dom_after_insert::<K::V, V::V>(
                            deleted.elements@,
                            inserted.elements@,
                            (k@, v@),
                        );
                        lemma_entries_to_map_finite::<K::V, V::V>(inserted.elements@);
                    }
                    return OrderedTableStPer { base_set: inserted };
                }
                i += 1;
            }
            // Key not found — just insert new pair.
            let new_pair = Pair(k, v);
            let inserted = self.base_set.insert(new_pair);
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.base_set.elements@, k@);
                lemma_keys_no_dups_after_set_insert::<K::V, V::V>(
                    self.base_set.elements@,
                    inserted.elements@,
                    (k@, v@),
                );
                lemma_entries_to_map_dom_after_insert::<K::V, V::V>(
                    self.base_set.elements@,
                    inserted.elements@,
                    (k@, v@),
                );
                lemma_entries_to_map_finite::<K::V, V::V>(inserted.elements@);
            }
            OrderedTableStPer { base_set: inserted }
        }

        fn delete(&self, k: &K) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    self.base_set.spec_avltreesetstper_wf(),
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_clone::<Pair<K, V>>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    forall|j: int| 0 <= j < i as int
                        ==> (#[trigger] self.base_set.elements@[j]).0 != k@,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0.eq(k) {
                    let pair_clone = pair.clone_plus();
                    let deleted = self.base_set.delete(&pair_clone);
                    proof {
                        assert(self.base_set.elements@[i as int] == pair_clone@);
                        lemma_keys_no_dups_after_set_remove::<K::V, V::V>(
                            self.base_set.elements@,
                            deleted.elements@,
                            pair_clone@,
                        );
                        lemma_entries_to_map_after_remove_pair::<K::V, V::V>(
                            self.base_set.elements@,
                            deleted.elements@,
                            pair_clone@,
                        );
                        assert(pair_clone@.0 == k@);
                        lemma_entries_to_map_finite::<K::V, V::V>(deleted.elements@);
                    }
                    return OrderedTableStPer { base_set: deleted };
                }
                i += 1;
            }
            // Key not found — return copy with same entries.
            let copy_elements = self.base_set.elements.clone();
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.base_set.elements@, k@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                assert(self@.remove(k@) =~= self@) by {
                    assert forall|k2: K::V| self@.dom().contains(k2)
                        implies k2 != k@ by {};
                };
            }
            OrderedTableStPer {
                base_set: AVLTreeSetStPer { elements: copy_elements },
            }
        }

        #[verifier::external_body]
        fn domain(&self) -> (keys: ArraySetStEph<K>)
        {
            let len = self.base_set.elements.length();
            let mut keys = ArraySetStEph::empty();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                keys.insert(pair.0.clone());
                i += 1;
            }
            keys
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let key_seq = keys.to_seq();
            let len = key_seq.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut key_args: Seq<K> = Seq::empty();
            while i < len
                invariant
                    key_seq@.no_duplicates(),
                    key_seq@.to_set() =~= keys@,
                    obeys_feq_full::<K>(),
                    len == key_seq.spec_len(),
                    0 <= i <= len,
                    forall|k: &K| f.requires((k,)),
                    result_vec@.len() == i as int,
                    key_args.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] result_vec@[j])@.0 == key_seq@[j],
                    forall|j: int| #![trigger key_args[j]]
                        0 <= j < i as int ==>
                        key_args[j]@ == key_seq@[j]
                        && f.ensures((&key_args[j],), result_vec@[j].1),
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                decreases len - i,
            {
                let k = key_seq.nth(i);
                let v = f(k);
                let key_clone = k.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<K>());
                    key_seq.lemma_view_index(i as int);
                    assert(key_clone@ == key_seq@[i as int]);
                    assert forall|a: int| 0 <= a < result_vec@.len()
                        implies (#[trigger] result_vec@[a])@.0 != key_seq@[i as int]
                    by {
                        assert(result_vec@[a]@.0 == key_seq@[a]);
                    };
                }
                result_vec.push(Pair(key_clone, v));
                proof {
                    key_args = key_args.push(*k);
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                // Prove table@.dom() =~= keys@.
                assert(table@.dom() =~= keys@) by {
                    assert forall|kv: K::V| #[trigger] table@.dom().contains(kv) <==> keys@.contains(kv) by {
                        if table@.dom().contains(kv) {
                            lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                            let j = choose|j: int| 0 <= j < seq@.len()
                                && (#[trigger] seq@[j]).0 == kv;
                            assert(seq@[j] == result_vec@[j]@);
                            assert(result_vec@[j]@.0 == key_seq@[j]);
                            assert(key_seq@[j] == kv);
                        }
                        if keys@.contains(kv) {
                            assert(key_seq@.to_set().contains(kv));
                            let j = choose|j: int| 0 <= j < key_seq@.len()
                                && (#[trigger] key_seq@[j]) == kv;
                            assert(seq@[j] == result_vec@[j]@);
                            assert(result_vec@[j]@.0 == key_seq@[j]);
                            assert(seq@[j].0 == kv);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                        }
                    };
                };
                // Prove value postcondition.
                assert forall|kv: K::V| #[trigger] table@.contains_key(kv) implies
                    (exists|key_arg: K, result: V|
                        key_arg@ == kv && f.ensures((&key_arg,), result)
                        && table@[kv] == result@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == kv;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    assert(key_args[j]@ == kv);
                    assert(f.ensures((&key_args[j],), result_vec@[j].1));
                    assert(table@[kv] == seq@[j].1);
                    assert(seq@[j].1 == result_vec@[j]@.1);
                };
            }
            table
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut orig_vals: Seq<V> = Seq::empty();
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_feq_full::<K>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    forall|v: &V| f.requires((v,)),
                    result_vec@.len() == i as int,
                    orig_vals.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] result_vec@[j])@.0 == self.base_set.elements@[j].0,
                    forall|j: int| #![trigger orig_vals[j]]
                        0 <= j < i as int ==>
                        orig_vals[j]@ == self.base_set.elements@[j].1
                        && f.ensures((&orig_vals[j],), result_vec@[j].1),
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                let new_v = f(&pair.1);
                let key_clone = pair.0.clone_plus();
                proof {
                    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
                    assert(key_clone@ == self.base_set.elements@[i as int].0);
                }
                result_vec.push(Pair(key_clone, new_v));
                proof {
                    orig_vals = orig_vals.push(pair.1);
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(result_vec@[a]@.0 == self.base_set.elements@[a].0);
                    assert(seq@[b] == result_vec@[b]@);
                    assert(result_vec@[b]@.0 == self.base_set.elements@[b].0);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                assert(table@.dom() =~= self@.dom()) by {
                    assert forall|k: K::V| table@.dom().contains(k) <==> self@.dom().contains(k) by {
                        if table@.dom().contains(k) {
                            lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                            let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                            assert(seq@[j] == result_vec@[j]@);
                            assert(result_vec@[j]@.0 == self.base_set.elements@[j].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_set.elements@, j);
                        }
                        if self@.dom().contains(k) {
                            lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_set.elements@, k);
                            let j = choose|j: int| 0 <= j < len && (#[trigger] self.base_set.elements@[j]).0 == k;
                            assert(seq@[j] == result_vec@[j]@);
                            assert(result_vec@[j]@.0 == self.base_set.elements@[j].0);
                            assert(seq@[j].0 == k);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                        }
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k) implies
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && table@[k] == result@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, j);
                    // Witnesses: orig_vals[j] for old_val, result_vec@[j].1 for result
                    assert(orig_vals[j]@ == self@[k]);
                    assert(f.ensures((&orig_vals[j],), result_vec@[j].1));
                    assert(seq@[j] == result_vec@[j]@);
                };
            }
            table
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_feq_full::<Pair<K, V>>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                    result_src.len() == result_vec@.len(),
                    result_idx.len() == i as int,
                    // Forward: each result entry's source index and properties.
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && result_vec@[j]@ == self.base_set.elements@[result_src[j]]
                        && spec_pred(self.base_set.elements@[result_src[j]].0, self.base_set.elements@[result_src[j]].1),
                    // Backward: passing entries map to result indices via result_idx.
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && spec_pred(self.base_set.elements@[m].0, self.base_set.elements@[m].1)
                        ==> 0 <= result_idx[m] < result_vec@.len()
                        && result_vec@[result_idx[m]]@ == self.base_set.elements@[m],
                    // Result entries have unique keys.
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                if f(&pair.0, &pair.1) {
                    let pair_clone = pair.clone_plus();
                    proof {
                        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
                        assert(pair_clone@ == self.base_set.elements@[i as int]);
                        assert(spec_pred(self.base_set.elements@[i as int].0, self.base_set.elements@[i as int].1));
                        // Key uniqueness: new entry's key differs from all existing entries' keys.
                        assert forall|a: int| 0 <= a < result_vec@.len()
                            implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(result_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_result_idx = result_vec@.len() as int;
                    result_vec.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                        result_idx = result_idx.push(new_result_idx);
                    }
                } else {
                    proof {
                        assert(f.ensures((&pair.0, &pair.1), false));
                        assert(!spec_pred(self.base_set.elements@[i as int].0, self.base_set.elements@[i as int].1));
                        result_idx = result_idx.push(0int);
                    }
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                    assert(result_vec@[a]@.0 != result_vec@[b]@.0);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Domain subset: table keys are all in self
                assert forall|k: K::V| table@.dom().contains(k) implies self@.dom().contains(k) by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_set.elements@, m);
                };
                // Value preservation
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                };
                // Backward completeness: passing entries are in table
                assert forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    implies #[trigger] table@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.base_set.elements@, k);
                    let m = choose|m: int| 0 <= m < len
                        && (#[trigger] self.base_set.elements@[m]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                    assert(spec_pred(self.base_set.elements@[m].0, self.base_set.elements@[m].1));
                    let j = result_idx[m];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    assert(seq@[j] == result_vec@[j]@);
                    assert(seq@[j].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                };
            }
            table
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            // Ghost sequences to track closure results for the value postcondition.
            let ghost mut result_v1: Seq<V> = Seq::empty();
            let ghost mut result_v2: Seq<V> = Seq::empty();
            let ghost mut result_r: Seq<V> = Seq::empty();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
            }
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    other.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    spec_keys_no_dups(other.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    result_src.len() == result_vec@.len(),
                    result_idx.len() == i as int,
                    result_v1.len() == result_vec@.len(),
                    result_v2.len() == result_vec@.len(),
                    result_r.len() == result_vec@.len(),
                    // Forward: result entries came from self and are in other's domain.
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && result_vec@[j]@.0 == self.base_set.elements@[result_src[j]].0
                        && other@.contains_key(self.base_set.elements@[result_src[j]].0),
                    // Value tracking: result entry j was produced by f(v1, v2).
                    forall|j: int| 0 <= j < result_vec@.len() ==> {
                        let src = #[trigger] result_src[j];
                        &&& result_v1[j]@ == self@[self.base_set.elements@[src].0]
                        &&& result_v2[j]@ == other@[self.base_set.elements@[src].0]
                        &&& f.ensures((&result_v1[j], &result_v2[j]), result_r[j])
                        &&& result_vec@[j]@.1 == result_r[j]@
                    },
                    // No dups in result keys.
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                    // Backward: self entries with key in other's domain are in result.
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && other@.contains_key(self.base_set.elements@[m].0)
                        ==> 0 <= result_idx[m] < result_vec@.len()
                        && result_vec@[result_idx[m]]@.0 == self.base_set.elements@[m].0,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        let key_clone = pair.0.clone_plus();
                        proof {
                            assert(obeys_feq_full_trigger::<K>());
                            assert(key_clone@ == pair.0@);
                            // other.find gives us other@[pair.0@] == other_v@.
                            // Prove no dup with existing result entries.
                            assert forall|a: int| 0 <= a < result_vec@.len()
                                implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                            by {
                                let src_a = result_src[a];
                                assert(result_vec@[a]@.0 == self.base_set.elements@[src_a].0);
                                assert(src_a != i as int);
                            };
                            // Track the closure arguments.
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                self.base_set.elements@, i as int,
                            );
                            lemma_entries_to_map_get::<K::V, V::V>(
                                self.base_set.elements@, i as int,
                            );
                        }
                        let ghost new_result_idx = result_vec@.len() as int;
                        result_vec.push(Pair(key_clone, combined));
                        proof {
                            result_src = result_src.push(i as int);
                            result_idx = result_idx.push(new_result_idx);
                            result_v1 = result_v1.push(pair.1);
                            result_v2 = result_v2.push(other_v);
                            result_r = result_r.push(combined);
                        }
                    },
                    None => {
                        proof {
                            result_idx = result_idx.push(0int);
                        }
                    },
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Prove table@.dom() =~= self@.dom().intersect(other@.dom()).
                assert forall|kv: K::V| #[trigger] table@.dom().contains(kv)
                    <==> self@.dom().contains(kv) && other@.dom().contains(kv)
                by {
                    if table@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == kv;
                        assert(seq@[j] == result_vec@[j]@);
                        let m = result_src[j];
                        assert(result_vec@[j]@.0 == self.base_set.elements@[m].0);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                        assert(other@.dom().contains(kv));
                    }
                    if self@.dom().contains(kv) && other@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_set.elements@, kv,
                        );
                        let m = choose|m: int| 0 <= m < len
                            && (#[trigger] self.base_set.elements@[m]).0 == kv;
                        let ri = result_idx[m];
                        assert(result_vec@[ri]@.0 == self.base_set.elements@[m].0);
                        assert(seq@[ri] == result_vec@[ri]@);
                        assert(seq@[ri].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, ri);
                    }
                };
                assert(table@.dom() =~= self@.dom().intersect(other@.dom()));
                // Prove value preservation.
                assert forall|kv: K::V| #[trigger] table@.contains_key(kv)
                    implies (exists|v1: V, v2: V, r: V|
                        v1@ == self@[kv] && v2@ == other@[kv]
                        && f.ensures((&v1, &v2), r)
                        && table@[kv] == r@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == kv;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    let src = result_src[j];
                    assert(self.base_set.elements@[src].0 == kv);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, src);
                    // Witness: v1 = result_v1[j], v2 = result_v2[j], r = result_r[j].
                    let v1 = result_v1[j];
                    let v2 = result_v2[j];
                    let r = result_r[j];
                    assert(v1@ == self@[kv]);
                    assert(v2@ == other@[kv]);
                    assert(f.ensures((&v1, &v2), r));
                    assert(table@[kv] == r@);
                };
            }
            table
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
        {
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            // Phase 1: all entries from self (merged with other where keys overlap).
            let self_len = self.base_set.elements.length();
            let mut i: usize = 0;
            let ghost mut phase1_src: Seq<int> = Seq::empty();
            // Track which self entries are in other (for the combined-value postcondition).
            let ghost mut self_in_other: Seq<bool> = Seq::empty();
            // Ghost for closure witness.
            let ghost mut combined_v1: Seq<V> = Seq::empty();
            let ghost mut combined_v2: Seq<V> = Seq::empty();
            let ghost mut combined_r: Seq<V> = Seq::empty();
            while i < self_len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    other.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    spec_keys_no_dups(other.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self_len == self.base_set.elements@.len(),
                    0 <= i <= self_len,
                    phase1_src.len() == result_vec@.len(),
                    self_in_other.len() == result_vec@.len(),
                    combined_v1.len() == result_vec@.len(),
                    combined_v2.len() == result_vec@.len(),
                    combined_r.len() == result_vec@.len(),
                    // result_vec has exactly i entries (one per self element).
                    result_vec@.len() == i as int,
                    // Forward: each result entry corresponds to a self element.
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        (#[trigger] phase1_src[j]) == j
                        && result_vec@[j]@.0 == self.base_set.elements@[j].0,
                    // self_in_other tracks whether the key was in other.
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        (#[trigger] self_in_other[j]) == other@.contains_key(self.base_set.elements@[j].0),
                    // Value: when NOT in other, value == self's value.
                    forall|j: int| 0 <= j < result_vec@.len() && !(#[trigger] self_in_other[j]) ==>
                        result_vec@[j]@.1 == self.base_set.elements@[j].1,
                    // Value: when in other, f was applied.
                    forall|j: int| 0 <= j < result_vec@.len() && (#[trigger] self_in_other[j]) ==> {
                        &&& combined_v1[j]@ == self@[self.base_set.elements@[j].0]
                        &&& combined_v2[j]@ == other@[self.base_set.elements@[j].0]
                        &&& f.ensures((&combined_v1[j], &combined_v2[j]), combined_r[j])
                        &&& result_vec@[j]@.1 == combined_r[j]@
                    },
                    // No dups in keys.
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                decreases self_len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        let key_clone = pair.0.clone_plus();
                        proof {
                            assert(obeys_feq_full_trigger::<K>());
                            assert(key_clone@ == pair.0@);
                            // Prove no dup: new key differs from all existing entries.
                            assert forall|a: int| 0 <= a < result_vec@.len()
                                implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                            by {
                                assert(phase1_src[a] == a);  // trigger the invariant
                                assert(a != i as int);
                            };
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                self.base_set.elements@, i as int,
                            );
                            lemma_entries_to_map_get::<K::V, V::V>(
                                self.base_set.elements@, i as int,
                            );
                        }
                        result_vec.push(Pair(key_clone, combined));
                        proof {
                            phase1_src = phase1_src.push(i as int);
                            self_in_other = self_in_other.push(true);
                            combined_v1 = combined_v1.push(pair.1);
                            combined_v2 = combined_v2.push(other_v);
                            combined_r = combined_r.push(combined);
                        }
                    },
                    None => {
                        let pair_clone = pair.clone_plus();
                        proof {
                            assert(obeys_feq_full_trigger::<Pair<K, V>>());
                            assert(pair_clone@ == self.base_set.elements@[i as int]);
                            // Prove no dup.
                            assert forall|a: int| 0 <= a < result_vec@.len()
                                implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                            by {
                                assert(phase1_src[a] == a);  // trigger the invariant
                                assert(a != i as int);
                            };
                        }
                        result_vec.push(pair_clone);
                        proof {
                            phase1_src = phase1_src.push(i as int);
                            self_in_other = self_in_other.push(false);
                            combined_v1 = combined_v1.push(pair.1);
                            combined_v2 = combined_v2.push(pair.1);
                            combined_r = combined_r.push(pair.1);
                        }
                    },
                }
                i += 1;
            }
            // Phase 2: entries from other not in self.
            let ghost phase1_len = result_vec@.len();
            let other_len = other.base_set.elements.length();
            let ghost mut phase2_src: Seq<int> = Seq::empty();
            let ghost mut phase2_idx: Seq<int> = Seq::empty();
            i = 0;
            // Bridge: re-state phase 1 results with result_vec trigger for phase 2.
            proof {
                assert forall|j: int| 0 <= j < phase1_len
                    implies (#[trigger] result_vec@[j])@.0 == self.base_set.elements@[j].0
                by {
                    assert(phase1_src[j] == j);
                };
            }
            while i < other_len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    other.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    spec_keys_no_dups(other.base_set.elements@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self_len == self.base_set.elements@.len(),
                    other_len == other.base_set.elements@.len(),
                    0 <= i <= other_len,
                    phase1_len == self_len as int,
                    result_vec@.len() >= phase1_len,
                    phase2_src.len() == result_vec@.len() - phase1_len,
                    phase2_idx.len() == i as int,
                    // Phase 1 entries preserved.
                    forall|j: int| 0 <= j < phase1_len ==>
                        (#[trigger] result_vec@[j])@.0 == self.base_set.elements@[j].0,
                    forall|j: int| 0 <= j < phase1_len ==>
                        (#[trigger] self_in_other[j]) == other@.contains_key(self.base_set.elements@[j].0),
                    forall|j: int| 0 <= j < phase1_len && !(#[trigger] self_in_other[j]) ==>
                        result_vec@[j]@.1 == self.base_set.elements@[j].1,
                    forall|j: int| 0 <= j < phase1_len && (#[trigger] self_in_other[j]) ==> {
                        &&& combined_v1[j]@ == self@[self.base_set.elements@[j].0]
                        &&& combined_v2[j]@ == other@[self.base_set.elements@[j].0]
                        &&& f.ensures((&combined_v1[j], &combined_v2[j]), combined_r[j])
                        &&& result_vec@[j]@.1 == combined_r[j]@
                    },
                    // Phase 2 forward: entries from other, not in self.
                    forall|j: int| 0 <= j < phase2_src.len() ==>
                        0 <= (#[trigger] phase2_src[j]) < i
                        && result_vec@[phase1_len + j]@ == other.base_set.elements@[phase2_src[j]]
                        && !self@.contains_key(other.base_set.elements@[phase2_src[j]].0),
                    // Phase 2 backward: other entries not in self are in result.
                    forall|m: int| #![trigger phase2_idx[m]]
                        0 <= m < i
                        && !self@.contains_key(other.base_set.elements@[m].0)
                        ==> 0 <= phase2_idx[m] < phase2_src.len()
                        && result_vec@[phase1_len + phase2_idx[m]]@ == other.base_set.elements@[m],
                    // No dups in all keys.
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                decreases other_len - i,
            {
                let pair = other.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                let self_find = self.find(&pair.0);
                if self_find.is_none() {
                    let pair_clone = pair.clone_plus();
                    proof {
                        assert(pair_clone@ == other.base_set.elements@[i as int]);
                        // Prove no dup with existing entries.
                        assert forall|a: int| 0 <= a < result_vec@.len()
                            implies (#[trigger] result_vec@[a])@.0 != other.base_set.elements@[i as int].0
                        by {
                            if a < phase1_len {
                                assert(result_vec@[a]@.0 == self.base_set.elements@[a].0);
                                lemma_entries_to_map_contains_key::<K::V, V::V>(
                                    self.base_set.elements@, a,
                                );
                            } else {
                                let p2idx = a - phase1_len;
                                let src = phase2_src[p2idx];
                                assert(result_vec@[a]@ == other.base_set.elements@[src]);
                                assert(src != i as int);
                            }
                        };
                    }
                    let ghost new_p2_idx = phase2_src.len() as int;
                    result_vec.push(pair_clone);
                    proof {
                        phase2_src = phase2_src.push(i as int);
                        phase2_idx = phase2_idx.push(new_p2_idx);
                    }
                } else {
                    proof {
                        phase2_idx = phase2_idx.push(0int);
                    }
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                lemma_entries_to_map_finite::<K::V, V::V>(other.base_set.elements@);
                // Prove table@.dom() =~= self@.dom().union(other@.dom()).
                assert forall|kv: K::V| #[trigger] table@.dom().contains(kv)
                    <==> self@.dom().contains(kv) || other@.dom().contains(kv)
                by {
                    if table@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == kv;
                        assert(seq@[j] == result_vec@[j]@);
                        if j < phase1_len {
                            assert(result_vec@[j]@.0 == self.base_set.elements@[j].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                self.base_set.elements@, j,
                            );
                        } else {
                            let p2idx = j - phase1_len;
                            let src = phase2_src[p2idx];
                            assert(result_vec@[j]@ == other.base_set.elements@[src]);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                other.base_set.elements@, src,
                            );
                        }
                    }
                    if self@.dom().contains(kv) {
                        // Key from self is at some index m in self.base_set.elements.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_set.elements@, kv,
                        );
                        let m = choose|m: int| 0 <= m < self_len
                            && (#[trigger] self.base_set.elements@[m]).0 == kv;
                        // Phase 1 put it at result_vec[m].
                        assert(result_vec@[m]@.0 == kv);
                        assert(seq@[m] == result_vec@[m]@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, m);
                    }
                    if other@.dom().contains(kv) && !self@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            other.base_set.elements@, kv,
                        );
                        let m = choose|m: int| 0 <= m < other_len
                            && (#[trigger] other.base_set.elements@[m]).0 == kv;
                        // Use backward tracking to find this entry in phase 2.
                        let p2i = phase2_idx[m];
                        assert(result_vec@[phase1_len + p2i]@ == other.base_set.elements@[m]);
                        assert(seq@[phase1_len + p2i] == result_vec@[phase1_len + p2i]@);
                        assert(seq@[phase1_len + p2i].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, phase1_len + p2i);
                    }
                };
                assert(table@.dom() =~= self@.dom().union(other@.dom()));
                // Prove value postconditions.
                // Case 1: key in self but not in other.
                assert forall|kv: K::V| #[trigger] self@.contains_key(kv) && !other@.contains_key(kv)
                    implies table@[kv] == self@[kv]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                        self.base_set.elements@, kv,
                    );
                    let m = choose|m: int| 0 <= m < self_len
                        && (#[trigger] self.base_set.elements@[m]).0 == kv;
                    assert(!self_in_other[m]);
                    assert(result_vec@[m]@.1 == self.base_set.elements@[m].1);
                    assert(seq@[m] == result_vec@[m]@);
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, m);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                };
                // Case 2: key in other but not in self.
                assert forall|kv: K::V| #[trigger] other@.contains_key(kv) && !self@.contains_key(kv)
                    implies table@[kv] == other@[kv]
                by {
                    // Use backward tracking to find the phase 2 entry directly.
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                        other.base_set.elements@, kv,
                    );
                    let m = choose|m: int| 0 <= m < other_len
                        && (#[trigger] other.base_set.elements@[m]).0 == kv;
                    let p2i = phase2_idx[m];
                    let j = phase1_len + p2i;
                    assert(result_vec@[j]@ == other.base_set.elements@[m]);
                    assert(seq@[j] == result_vec@[j]@);
                    assert(seq@[j].0 == kv);
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(other.base_set.elements@, m);
                };
                // Case 3: key in both self and other.
                assert forall|kv: K::V| #[trigger] self@.contains_key(kv) && other@.contains_key(kv)
                    implies (exists|v1: V, v2: V, r: V|
                        v1@ == self@[kv] && v2@ == other@[kv]
                        && f.ensures((&v1, &v2), r)
                        && table@[kv] == r@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                        self.base_set.elements@, kv,
                    );
                    let m = choose|m: int| 0 <= m < self_len
                        && (#[trigger] self.base_set.elements@[m]).0 == kv;
                    assert(self_in_other[m]);
                    assert(result_vec@[m]@.0 == kv);
                    assert(seq@[m] == result_vec@[m]@);
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, m);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                    let v1 = combined_v1[m];
                    let v2 = combined_v2[m];
                    let r = combined_r[m];
                    assert(v1@ == self@[kv]);
                    assert(v2@ == other@[kv]);
                    assert(f.ensures((&v1, &v2), r));
                    assert(table@[kv] == r@);
                };
            }
            table
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                let other_find = other.find(&pair.0);
                if other_find.is_none() {
                    result_vec.push(pair.clone());
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            OrderedTableStPer { base_set: AVLTreeSetStPer::from_seq(seq) }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            proof { vstd::seq_lib::seq_to_set_is_finite(keys.elements@); }
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    keys@.finite(),
                    result_src.len() == result_vec@.len(),
                    result_idx.len() == i as int,
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && result_vec@[j]@ == self.base_set.elements@[result_src[j]]
                        && keys@.contains(self.base_set.elements@[result_src[j]].0),
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && keys@.contains(self.base_set.elements@[m].0)
                        ==> 0 <= result_idx[m] < result_vec@.len()
                        && result_vec@[result_idx[m]]@ == self.base_set.elements@[m],
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                if keys.find(&pair.0) {
                    let pair_clone = pair.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<Pair<K, V>>());
                        assert(pair_clone@ == self.base_set.elements@[i as int]);
                        assert forall|a: int| 0 <= a < result_vec@.len()
                            implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(result_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_result_idx = result_vec@.len() as int;
                    result_vec.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                        result_idx = result_idx.push(new_result_idx);
                    }
                } else {
                    proof {
                        result_idx = result_idx.push(0int);
                    }
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Prove table@.dom() =~= self@.dom().intersect(keys@).
                assert forall|kv: K::V| #[trigger] table@.dom().contains(kv)
                    <==> self@.dom().contains(kv) && keys@.contains(kv)
                by {
                    if table@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == kv;
                        assert(seq@[j] == result_vec@[j]@);
                        let m = result_src[j];
                        assert(result_vec@[j]@ == self.base_set.elements@[m]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                        assert(keys@.contains(kv));
                    }
                    if self@.dom().contains(kv) && keys@.contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_set.elements@, kv,
                        );
                        let m = choose|m: int| 0 <= m < len
                            && (#[trigger] self.base_set.elements@[m]).0 == kv;
                        let ri = result_idx[m];
                        assert(result_vec@[ri]@ == self.base_set.elements@[m]);
                        assert(seq@[ri] == result_vec@[ri]@);
                        assert(seq@[ri].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, ri);
                    }
                };
                assert(table@.dom() =~= self@.dom().intersect(keys@));
                // Prove value preservation.
                assert forall|kv: K::V| #[trigger] table@.contains_key(kv)
                    implies table@[kv] == self@[kv]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == kv;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                };
            }
            table
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            let ghost mut result_idx: Seq<int> = Seq::empty();
            proof { vstd::seq_lib::seq_to_set_is_finite(keys.elements@); }
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    keys@.finite(),
                    result_src.len() == result_vec@.len(),
                    result_idx.len() == i as int,
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && result_vec@[j]@ == self.base_set.elements@[result_src[j]]
                        && !keys@.contains(self.base_set.elements@[result_src[j]].0),
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                    forall|m: int| #![trigger result_idx[m]]
                        0 <= m < i
                        && !keys@.contains(self.base_set.elements@[m].0)
                        ==> 0 <= result_idx[m] < result_vec@.len()
                        && result_vec@[result_idx[m]]@ == self.base_set.elements@[m],
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                if !keys.find(&pair.0) {
                    let pair_clone = pair.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<Pair<K, V>>());
                        assert(pair_clone@ == self.base_set.elements@[i as int]);
                        assert forall|a: int| 0 <= a < result_vec@.len()
                            implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(result_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_result_idx = result_vec@.len() as int;
                    result_vec.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                        result_idx = result_idx.push(new_result_idx);
                    }
                } else {
                    proof {
                        result_idx = result_idx.push(0int);
                    }
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Prove table@.dom() =~= self@.dom().difference(keys@).
                assert forall|kv: K::V| #[trigger] table@.dom().contains(kv)
                    <==> self@.dom().contains(kv) && !keys@.contains(kv)
                by {
                    if table@.dom().contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == kv;
                        assert(seq@[j] == result_vec@[j]@);
                        let m = result_src[j];
                        assert(result_vec@[j]@ == self.base_set.elements@[m]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                        assert(!keys@.contains(kv));
                    }
                    if self@.dom().contains(kv) && !keys@.contains(kv) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_set.elements@, kv,
                        );
                        let m = choose|m: int| 0 <= m < len
                            && (#[trigger] self.base_set.elements@[m]).0 == kv;
                        let ri = result_idx[m];
                        assert(result_vec@[ri]@ == self.base_set.elements@[m]);
                        assert(seq@[ri] == result_vec@[ri]@);
                        assert(seq@[ri].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, ri);
                    }
                };
                assert(table@.dom() =~= self@.dom().difference(keys@));
                // Prove value preservation.
                assert forall|kv: K::V| #[trigger] table@.contains_key(kv)
                    implies table@[kv] == self@[kv]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, kv);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == kv;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                };
            }
            table
        }

        #[verifier::external_body]
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
        {
            self.base_set.to_seq()
        }

        #[verifier::external_body]
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let first_pair = self.base_set.elements.nth(0);
                let mut min_key = first_pair.0.clone();
                let mut i: usize = 1;
                while i < len {
                    let elem_pair = self.base_set.elements.nth(i);
                    if elem_pair.0 < min_key {
                        min_key = elem_pair.0.clone();
                    }
                    i += 1;
                }
                Some(min_key)
            }
        }


        #[verifier::external_body]
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            if len == 0 {
                None
            } else {
                let first_pair = self.base_set.elements.nth(0);
                let mut max_key = first_pair.0.clone();
                let mut i: usize = 1;
                while i < len {
                    let elem_pair = self.base_set.elements.nth(i);
                    if elem_pair.0 > max_key {
                        max_key = elem_pair.0.clone();
                    }
                    i += 1;
                }
                Some(max_key)
            }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 < *k {
                    match &best {
                        None => { best = Some(pair.0.clone()); },
                        Some(b) => {
                            if pair.0 > *b {
                                best = Some(pair.0.clone());
                            }
                        },
                    }
                }
                i += 1;
            }
            best
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 > *k {
                    match &best {
                        None => { best = Some(pair.0.clone()); },
                        Some(b) => {
                            if pair.0 < *b {
                                best = Some(pair.0.clone());
                            }
                        },
                    }
                }
                i += 1;
            }
            best
        }

        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
        {
            let len = self.base_set.elements.length();
            let mut left_vec: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            let ghost mut left_src: Seq<int> = Seq::empty();
            let ghost mut left_idx: Seq<int> = Seq::empty();
            let ghost mut found: bool = false;
            let ghost mut found_idx: int = 0;
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    obeys_view_eq::<K>(),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    left_src.len() == left_vec@.len(),
                    left_idx.len() == i as int,
                    // Forward: left entries map to source entries with key != k@
                    forall|j: int| 0 <= j < left_vec@.len() ==>
                        0 <= (#[trigger] left_src[j]) < i
                        && left_vec@[j]@ == self.base_set.elements@[left_src[j]]
                        && self.base_set.elements@[left_src[j]].0 != k@,
                    // No dups in left keys
                    forall|a: int, b: int| 0 <= a < b < left_vec@.len() ==>
                        (#[trigger] left_vec@[a])@.0 != (#[trigger] left_vec@[b])@.0,
                    // Backward: source entries with key != k@ are in left
                    forall|m: int| #![trigger left_idx[m]]
                        0 <= m < i
                        && self.base_set.elements@[m].0 != k@
                        ==> 0 <= left_idx[m] < left_vec@.len()
                        && left_vec@[left_idx[m]]@ == self.base_set.elements@[m],
                    // Found value tracking
                    found_value is Some <==> found,
                    found ==> 0 <= found_idx < i as int,
                    found ==> self.base_set.elements@[found_idx].0 == k@,
                    found ==> found_value->Some_0@ == self.base_set.elements@[found_idx].1,
                    !found ==> forall|m: int| 0 <= m < i as int ==>
                        (#[trigger] self.base_set.elements@[m]).0 != k@,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0.eq(k) {
                    let v_clone = pair.1.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        found = true;
                        found_idx = i as int;
                        left_idx = left_idx.push(0int);
                    }
                    found_value = Some(v_clone);
                } else {
                    let pair_clone = pair.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<Pair<K, V>>());
                        assert(pair_clone@ == self.base_set.elements@[i as int]);
                        assert forall|a: int| 0 <= a < left_vec@.len()
                            implies (#[trigger] left_vec@[a])@.0 != self.base_set.elements@[i as int].0
                        by {
                            let src_a = left_src[a];
                            assert(left_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    let ghost new_left_idx = left_vec@.len() as int;
                    left_vec.push(pair_clone);
                    proof {
                        left_src = left_src.push(i as int);
                        left_idx = left_idx.push(new_left_idx);
                    }
                }
                i += 1;
            }
            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            proof {
                assert(left_seq@ =~= left_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < left_seq@.len()
                    implies (#[trigger] left_seq@[a]).0 != (#[trigger] left_seq@[b]).0
                by {
                    assert(left_seq@[a] == left_vec@[a]@);
                    assert(left_seq@[b] == left_vec@[b]@);
                };
            }
            let left_table = from_sorted_entries(left_seq);
            let right_table = Self::empty();
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Prove found_value postconditions.
                if found {
                    lemma_entries_to_map_contains_key::<K::V, V::V>(
                        self.base_set.elements@, found_idx,
                    );
                    lemma_entries_to_map_get::<K::V, V::V>(
                        self.base_set.elements@, found_idx,
                    );
                } else {
                    lemma_entries_to_map_no_key::<K::V, V::V>(
                        self.base_set.elements@, k@,
                    );
                }
                // Prove !left_table@.dom().contains(k@).
                assert(!left_table@.dom().contains(k@)) by {
                    if left_table@.dom().contains(k@) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, k@);
                        let j = choose|j: int| 0 <= j < left_seq@.len()
                            && (#[trigger] left_seq@[j]).0 == k@;
                        assert(left_seq@[j] == left_vec@[j]@);
                        let src_j = left_src[j];
                        assert(left_vec@[j]@ == self.base_set.elements@[src_j]);
                        assert(self.base_set.elements@[src_j].0 != k@);
                    }
                };
                // Prove left_table@.dom().subset_of(self@.dom()).
                assert(left_table@.dom().subset_of(self@.dom())) by {
                    assert forall|key: K::V| left_table@.dom().contains(key)
                        implies self@.dom().contains(key)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, key);
                        let j = choose|j: int| 0 <= j < left_seq@.len()
                            && (#[trigger] left_seq@[j]).0 == key;
                        assert(left_seq@[j] == left_vec@[j]@);
                        let m = left_src[j];
                        assert(left_vec@[j]@ == self.base_set.elements@[m]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                    };
                };
                // Prove completeness.
                assert forall|key: K::V| #[trigger] self@.dom().contains(key)
                    implies left_table@.dom().contains(key)
                        || right_table@.dom().contains(key) || key == k@
                by {
                    if key != k@ {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            self.base_set.elements@, key,
                        );
                        let m = choose|m: int| 0 <= m < len
                            && (#[trigger] self.base_set.elements@[m]).0 == key;
                        let li = left_idx[m];
                        assert(left_vec@[li]@ == self.base_set.elements@[m]);
                        assert(left_seq@[li] == left_vec@[li]@);
                        assert(left_seq@[li].0 == key);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(left_seq@, li);
                    }
                };
            }
            (left_table, found_value, right_table)
        }

        fn join_key(left: &Self, right: &Self) -> (table: Self)
        {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            let len = self.base_set.elements.length();
            let mut result_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            let ghost mut result_src: Seq<int> = Seq::empty();
            while i < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    len == self.base_set.elements@.len(),
                    0 <= i <= len,
                    result_src.len() == result_vec@.len(),
                    forall|j: int| 0 <= j < result_vec@.len() ==>
                        0 <= (#[trigger] result_src[j]) < i
                        && result_vec@[j]@ == self.base_set.elements@[result_src[j]],
                    forall|a: int, b: int| 0 <= a < b < result_vec@.len() ==>
                        (#[trigger] result_vec@[a])@.0 != (#[trigger] result_vec@[b])@.0,
                decreases len - i,
            {
                let pair = self.base_set.elements.nth(i);
                if pair.0 >= *k1 && pair.0 <= *k2 {
                    let pair_clone = pair.clone_plus();
                    proof {
                        assert(obeys_feq_full_trigger::<Pair<K, V>>());
                        assert(pair_clone@ == self.base_set.elements@[i as int]);
                        assert forall|a: int| 0 <= a < result_vec@.len()
                            implies (#[trigger] result_vec@[a])@.0 != self.base_set.elements@[i as int].0
                        by {
                            let src_a = result_src[a];
                            assert(result_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != i as int);
                        };
                    }
                    result_vec.push(pair_clone);
                    proof {
                        result_src = result_src.push(i as int);
                    }
                }
                i += 1;
            }
            let seq = AVLTreeSeqStPerS::from_vec(result_vec);
            proof {
                assert(seq@ =~= result_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < seq@.len()
                    implies (#[trigger] seq@[a]).0 != (#[trigger] seq@[b]).0
                by {
                    assert(seq@[a] == result_vec@[a]@);
                    assert(seq@[b] == result_vec@[b]@);
                    assert(result_vec@[a]@.0 != result_vec@[b]@.0);
                };
            }
            let table = from_sorted_entries(seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                assert forall|k: K::V| table@.dom().contains(k) implies self@.dom().contains(k) by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.base_set.elements@, m);
                };
                assert forall|k: K::V| #[trigger] table@.dom().contains(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len() && (#[trigger] seq@[j]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(seq@[j] == result_vec@[j]@);
                    let m = result_src[j];
                    assert(result_vec@[j]@ == self.base_set.elements@[m]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.base_set.elements@, m);
                };
            }
            table
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            let len = self.base_set.elements.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len {
                let pair = self.base_set.elements.nth(i);
                if pair.0 < *k {
                    count += 1;
                }
                i += 1;
            }
            count
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
        {
            let entries = self.collect();
            let len = entries.length();
            if i >= len {
                return None;
            }
            // Collect all keys, sort, return i-th.
            let mut keys: Vec<K> = Vec::new();
            let mut j: usize = 0;
            while j < len {
                let pair = entries.nth(j);
                keys.push(pair.0.clone());
                j += 1;
            }
            keys.sort();
            Some(keys[i].clone())
        }

        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
        {
            let len = self.base_set.elements.length();
            let split_at = if i >= len { len } else { i };
            let mut left_vec: Vec<Pair<K, V>> = Vec::new();
            let mut right_vec: Vec<Pair<K, V>> = Vec::new();
            let mut j: usize = 0;
            let ghost mut left_src: Seq<int> = Seq::empty();
            let ghost mut right_src: Seq<int> = Seq::empty();
            let ghost mut dest: Seq<int> = Seq::empty(); // 0 = left, 1 = right
            proof { assert(obeys_feq_full_trigger::<Pair<K, V>>()); }
            while j < len
                invariant
                    self.base_set.elements.spec_avltreeseqstper_wf(),
                    spec_keys_no_dups(self.base_set.elements@),
                    len == self.base_set.elements@.len(),
                    0 <= j <= len,
                    0 <= split_at <= len,
                    obeys_feq_full::<Pair<K, V>>(),
                    left_src.len() == left_vec@.len(),
                    right_src.len() == right_vec@.len(),
                    dest.len() == j as int,
                    // Left entries come from indices < split_at.
                    forall|a: int| 0 <= a < left_vec@.len() ==>
                        0 <= (#[trigger] left_src[a]) < j
                        && left_src[a] < split_at
                        && left_vec@[a]@ == self.base_set.elements@[left_src[a]],
                    // Right entries come from indices >= split_at.
                    forall|a: int| 0 <= a < right_vec@.len() ==>
                        0 <= (#[trigger] right_src[a]) < j
                        && right_src[a] >= split_at
                        && right_vec@[a]@ == self.base_set.elements@[right_src[a]],
                    // Backward: every processed element is in left or right.
                    forall|m: int| #![trigger dest[m]] 0 <= m < j ==>
                        if m < split_at {
                            0 <= dest[m] < left_vec@.len()
                            && left_vec@[dest[m]]@ == self.base_set.elements@[m]
                        } else {
                            0 <= dest[m] < right_vec@.len()
                            && right_vec@[dest[m]]@ == self.base_set.elements@[m]
                        },
                    // No dups in left.
                    forall|a: int, b: int| 0 <= a < b < left_vec@.len() ==>
                        (#[trigger] left_vec@[a])@.0 != (#[trigger] left_vec@[b])@.0,
                    // No dups in right.
                    forall|a: int, b: int| 0 <= a < b < right_vec@.len() ==>
                        (#[trigger] right_vec@[a])@.0 != (#[trigger] right_vec@[b])@.0,
                decreases len - j,
            {
                let pair = self.base_set.elements.nth(j);
                let pair_clone = pair.clone_plus();
                proof { assert(pair_clone@ == self.base_set.elements@[j as int]); }
                if j < split_at {
                    proof {
                        assert forall|a: int| 0 <= a < left_vec@.len()
                            implies (#[trigger] left_vec@[a])@.0 != self.base_set.elements@[j as int].0
                        by {
                            let src_a = left_src[a];
                            assert(left_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != j as int);
                        };
                    }
                    let ghost new_dest = left_vec@.len() as int;
                    left_vec.push(pair_clone);
                    proof {
                        left_src = left_src.push(j as int);
                        dest = dest.push(new_dest);
                    }
                } else {
                    proof {
                        assert forall|a: int| 0 <= a < right_vec@.len()
                            implies (#[trigger] right_vec@[a])@.0 != self.base_set.elements@[j as int].0
                        by {
                            let src_a = right_src[a];
                            assert(right_vec@[a]@ == self.base_set.elements@[src_a]);
                            assert(src_a != j as int);
                        };
                    }
                    let ghost new_dest = right_vec@.len() as int;
                    right_vec.push(pair_clone);
                    proof {
                        right_src = right_src.push(j as int);
                        dest = dest.push(new_dest);
                    }
                }
                j += 1;
            }
            let left_seq = AVLTreeSeqStPerS::from_vec(left_vec);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_vec);
            proof {
                assert(left_seq@ =~= left_vec@.map_values(|p: Pair<K, V>| p@));
                assert(right_seq@ =~= right_vec@.map_values(|p: Pair<K, V>| p@));
                assert forall|a: int, b: int| 0 <= a < b < left_seq@.len()
                    implies (#[trigger] left_seq@[a]).0 != (#[trigger] left_seq@[b]).0
                by {
                    assert(left_seq@[a] == left_vec@[a]@);
                    assert(left_seq@[b] == left_vec@[b]@);
                };
                assert forall|a: int, b: int| 0 <= a < b < right_seq@.len()
                    implies (#[trigger] right_seq@[a]).0 != (#[trigger] right_seq@[b]).0
                by {
                    assert(right_seq@[a] == right_vec@[a]@);
                    assert(right_seq@[b] == right_vec@[b]@);
                };
            }
            let left_table = from_sorted_entries(left_seq);
            let right_table = from_sorted_entries(right_seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_set.elements@);
                // Prove subset.
                assert(left_table@.dom().subset_of(self@.dom())) by {
                    assert forall|kv: K::V| left_table@.dom().contains(kv)
                        implies self@.dom().contains(kv)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, kv);
                        let a = choose|a: int| 0 <= a < left_seq@.len()
                            && (#[trigger] left_seq@[a]).0 == kv;
                        assert(left_seq@[a] == left_vec@[a]@);
                        let m = left_src[a];
                        assert(left_vec@[a]@ == self.base_set.elements@[m]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                    };
                };
                assert(right_table@.dom().subset_of(self@.dom())) by {
                    assert forall|kv: K::V| right_table@.dom().contains(kv)
                        implies self@.dom().contains(kv)
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(right_seq@, kv);
                        let a = choose|a: int| 0 <= a < right_seq@.len()
                            && (#[trigger] right_seq@[a]).0 == kv;
                        assert(right_seq@[a] == right_vec@[a]@);
                        let m = right_src[a];
                        assert(right_vec@[a]@ == self.base_set.elements@[m]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.base_set.elements@, m,
                        );
                    };
                };
                // Prove disjoint.
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|kv: K::V| left_table@.dom().contains(kv)
                        implies !right_table@.dom().contains(kv)
                    by {
                        if right_table@.dom().contains(kv) {
                            lemma_entries_to_map_key_in_seq::<K::V, V::V>(left_seq@, kv);
                            lemma_entries_to_map_key_in_seq::<K::V, V::V>(right_seq@, kv);
                            let la = choose|a: int| 0 <= a < left_seq@.len()
                                && (#[trigger] left_seq@[a]).0 == kv;
                            let ra = choose|a: int| 0 <= a < right_seq@.len()
                                && (#[trigger] right_seq@[a]).0 == kv;
                            assert(left_seq@[la] == left_vec@[la]@);
                            assert(right_seq@[ra] == right_vec@[ra]@);
                            let lm = left_src[la];
                            let rm = right_src[ra];
                            assert(left_vec@[la]@ == self.base_set.elements@[lm]);
                            assert(right_vec@[ra]@ == self.base_set.elements@[rm]);
                            // lm < split_at, rm >= split_at, so lm != rm.
                            assert(lm != rm);
                            // But both have key kv, contradicting spec_keys_no_dups.
                        }
                    };
                };
                // Prove completeness.
                assert forall|kv: K::V| #[trigger] self@.dom().contains(kv)
                    implies left_table@.dom().contains(kv) || right_table@.dom().contains(kv)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                        self.base_set.elements@, kv,
                    );
                    let m = choose|m: int| 0 <= m < len
                        && (#[trigger] self.base_set.elements@[m]).0 == kv;
                    let d = dest[m];
                    if m < split_at as int {
                        assert(left_vec@[d]@ == self.base_set.elements@[m]);
                        assert(left_seq@[d] == left_vec@[d]@);
                        assert(left_seq@[d].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(left_seq@, d);
                    } else {
                        assert(right_vec@[d]@ == self.base_set.elements@[m]);
                        assert(right_seq@[d] == right_vec@[d]@);
                        assert(right_seq@[d].0 == kv);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(right_seq@, d);
                    }
                };
            }
            (left_table, right_table)
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (table: OrderedTableStPer<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            spec_keys_no_dups(entries@),
        ensures
            table@.dom().finite(),
            table@ =~= spec_entries_to_map(entries@),
            table.spec_orderedtablestper_wf(),
    {
        let base = AVLTreeSetStPer::from_seq(entries);
        // base@ =~= entries@.to_set(), base.spec_avltreesetstper_wf()
        // base.elements@.to_set() =~= entries@.to_set()
        proof {
            lemma_keys_no_dups_implies_no_duplicates::<K::V, V::V>(entries@);
            lemma_keys_no_dups_preserved_by_set_eq::<K::V, V::V>(
                entries@, base.elements@,
            );
            lemma_entries_to_map_set_determines_map::<K::V, V::V>(
                entries@, base.elements@,
            );
            lemma_entries_to_map_finite::<K::V, V::V>(base.elements@);
        }
        OrderedTableStPer { base_set: base }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            requires self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedTableStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<'a, K: StT + Ord, V: StT + Ord> {
        pub seq: &'a AVLTreeSeqStPerS<Pair<K, V>>,
        pub pos: usize,
        pub len: usize,
    }

    impl<'a, K: StT + Ord, V: StT + Ord> View for OrderedTableStPerIter<'a, K, V> {
        type V = (int, Seq<(K::V, V::V)>);
        open spec fn view(&self) -> (int, Seq<(K::V, V::V)>) { (self.pos as int, self.seq@) }
    }

    pub open spec fn iter_invariant<'a, K: StT + Ord, V: StT + Ord>(it: &OrderedTableStPerIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::Iterator for OrderedTableStPerIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<&'a Pair<K, V>>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element@ == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos < self.len {
                let elem = self.seq.nth(self.pos);
                self.pos += 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerGhostIterator<'a, K: StT + Ord, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<(K::V, V::V)>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K: StT + Ord, V: StT + Ord> View for OrderedTableStPerGhostIterator<'a, K, V> {
        type V = Seq<(K::V, V::V)>;

        open spec fn view(&self) -> Seq<(K::V, V::V)> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<'a, K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<'a, K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStPerIter<'a, K, V>;
        type Item = (K::V, V::V);
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStPerIter<'a, K, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<(K::V, V::V)> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<'a, K, V>) -> OrderedTableStPerGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.base_set.elements@,
                iter_invariant(&it),
        {
            let len = self.base_set.elements.length();
            OrderedTableStPerIter { seq: &self.base_set.elements, pos: 0, len }
        }
    }

    // 12. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT + Ord> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT + Ord> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord, V: StT + Ord> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_set == other.base_set;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT + Ord> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            let copy = OrderedTableStPer {
                base_set: self.base_set.clone(),
            };
            proof { assume(copy@ == self@); }
            copy
        }
    }

    } // verus!

    // 13. macros

    /// Macro for creating ordered tables from sorted key-value pairs
    #[macro_export]
    macro_rules! OrderedTableStPerLit {
        () => {
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::OrderedTableStPer::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::from_sorted_entries(seq)
        }};
    }

    // 14. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }
}
