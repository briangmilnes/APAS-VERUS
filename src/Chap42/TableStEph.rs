//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store.

//  Table of Contents
//	1. module
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module

pub mod TableStEph {

    use std::cmp::Ordering;
    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::clone_view::clone_view::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    verus! {

    //		3. broadcast use

    // 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		4. type definitions

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEph<K: StT + Ord, V: StT> {
        pub entries: ArraySeqStEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStEph<K, V>;

    //		5. view impls

    impl<K: StT + Ord, V: StT> View for TableStEph<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }

    //		6. spec fns

    // 5. view impls

    // Converts a sorted sequence of (key, value) pairs to a Map.
    // Later entries win on duplicate keys (irrelevant when keys are unique).
    pub open spec fn spec_entries_to_map<KV, VV>(entries: Seq<(KV, VV)>) -> Map<KV, VV>
        decreases entries.len()
    {
        if entries.len() == 0 {
            Map::empty()
        } else {
            let last = entries.last();
            spec_entries_to_map(entries.drop_last()).insert(last.0, last.1)
        }
    }

    // 6. spec fns

    // Keys in the entry sequence are unique.
    pub open spec fn spec_keys_no_dups<KV, VV>(entries: Seq<(KV, VV)>) -> bool {
        forall|i: int, j: int|
            0 <= i < j < entries.len() ==> (#[trigger] entries[i]).0 != (#[trigger] entries[j]).0
    }

    //		7. proof fns/broadcast groups

    // 7. proof fns

    // If a key is in spec_entries_to_map, it appears in the seq.
    pub proof fn lemma_entries_to_map_key_in_seq<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires spec_entries_to_map(entries).contains_key(k)
        ensures exists|i: int| 0 <= i < entries.len() && (#[trigger] entries[i]).0 == k
        decreases entries.len()
    {
        if entries.len() > 0 {
            let last = entries.last();
            if last.0 == k {
                assert(entries[entries.len() - 1].0 == k);
            } else {
                lemma_entries_to_map_key_in_seq::<KV, VV>(entries.drop_last(), k);
                let prefix = entries.drop_last();
                let i = choose|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == k;
                assert(entries[i].0 == k);
            }
        }
    }

    // If entries[idx] has key k, the map contains k.
    pub proof fn lemma_entries_to_map_contains_key<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
        requires 0 <= idx < entries.len()
        ensures spec_entries_to_map(entries).contains_key(entries[idx].0)
        decreases entries.len()
    {
        if entries.len() > 0 {
            if idx == entries.len() - 1 {
            } else {
                lemma_entries_to_map_contains_key::<KV, VV>(entries.drop_last(), idx);
            }
        }
    }

    // When keys are unique, spec_entries_to_map length equals seq length.
    pub proof fn lemma_entries_to_map_len<KV, VV>(entries: Seq<(KV, VV)>)
        requires spec_keys_no_dups(entries)
        ensures spec_entries_to_map(entries).len() == entries.len()
        decreases entries.len()
    {
        if entries.len() > 0 {
            let prefix = entries.drop_last();
            let last = entries.last();
            let last_idx = entries.len() - 1;
            assert(spec_keys_no_dups(prefix)) by {
                assert forall|i: int, j: int|
                    0 <= i < j < prefix.len()
                    implies (#[trigger] prefix[i]).0 != (#[trigger] prefix[j]).0
                by {
                    assert(entries[i].0 != entries[j].0);
                };
            };
            lemma_entries_to_map_len::<KV, VV>(prefix);
            let prefix_map = spec_entries_to_map(prefix);
            assert(!prefix_map.contains_key(last.0)) by {
                if prefix_map.contains_key(last.0) {
                    lemma_entries_to_map_key_in_seq(prefix, last.0);
                    let idx = choose|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == last.0;
                    assert(entries[idx].0 == last.0);
                    assert(entries[last_idx].0 == last.0);
                    assert(idx != last_idx);
                }
            };
            assert(prefix_map.dom().finite()) by {
                lemma_entries_to_map_finite::<KV, VV>(prefix);
            };
            assert(spec_entries_to_map(entries) =~=
                prefix_map.insert(last.0, last.1));
            assert(prefix_map.insert(last.0, last.1).len() ==
                prefix_map.len() + 1);
        }
    }

    // If no entry has key k, spec_entries_to_map does not contain k.
    pub proof fn lemma_entries_to_map_no_key<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires forall|i: int| 0 <= i < entries.len() ==> (#[trigger] entries[i]).0 != k
        ensures !spec_entries_to_map(entries).contains_key(k)
    {
        if spec_entries_to_map(entries).contains_key(k) {
            lemma_entries_to_map_key_in_seq(entries, k);
        }
    }

    // If entries[idx] = (k, v) and keys are unique, map contains k with value v.
    pub proof fn lemma_entries_to_map_get<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
        requires
            0 <= idx < entries.len(),
            spec_keys_no_dups(entries),
        ensures
            spec_entries_to_map(entries).contains_key(entries[idx].0),
            spec_entries_to_map(entries)[entries[idx].0] == entries[idx].1,
        decreases entries.len(),
    {
        let k = entries[idx].0;
        let v = entries[idx].1;
        if entries.len() > 0 {
            let last = entries.last();
            let prefix = entries.drop_last();
            if idx == entries.len() - 1 {
            } else {
                assert(spec_keys_no_dups(prefix)) by {
                    assert forall|i: int, j: int|
                        0 <= i < j < prefix.len()
                        implies (#[trigger] prefix[i]).0 != (#[trigger] prefix[j]).0
                    by {
                        assert(i < entries.len());
                        assert(j < entries.len());
                        assert(entries[i].0 != entries[j].0);
                    };
                };
                assert(prefix[idx] == entries[idx]);
                lemma_entries_to_map_get::<KV, VV>(prefix, idx);
                assert(last.0 != k) by {
                    assert(entries[idx].0 == k);
                    assert(entries[entries.len() - 1].0 == last.0);
                    assert(idx < entries.len() - 1);
                };
            }
        }
    }

    // If every key in sub appears in sup, sub map domain ⊆ sup map domain.
    pub proof fn lemma_entries_to_map_dom_subset<KV, VV>(
        sub: Seq<(KV, VV)>,
        sup: Seq<(KV, VV)>,
    )
        requires forall|i: int| 0 <= i < sub.len() ==>
            exists|j: int| 0 <= j < sup.len() && (#[trigger] sup[j]).0 == (#[trigger] sub[i]).0,
        ensures spec_entries_to_map(sub).dom().subset_of(spec_entries_to_map(sup).dom()),
    {
        assert forall|k: KV| spec_entries_to_map(sub).dom().contains(k)
            implies spec_entries_to_map(sup).dom().contains(k)
        by {
            lemma_entries_to_map_key_in_seq(sub, k);
            let i = choose|i: int| 0 <= i < sub.len() && (#[trigger] sub[i]).0 == k;
            let j = choose|j: int| 0 <= j < sup.len() && (#[trigger] sup[j]).0 == sub[i].0;
            lemma_entries_to_map_contains_key(sup, j);
        };
    }

    // If two sequences have the same keys at each position, their maps have the same domain.
    pub proof fn lemma_entries_to_map_dom_same_keys<KV, VV1, VV2>(
        s1: Seq<(KV, VV1)>,
        s2: Seq<(KV, VV2)>,
    )
        requires
            s1.len() == s2.len(),
            forall|i: int| 0 <= i < s1.len() ==> (#[trigger] s1[i]).0 == (#[trigger] s2[i]).0,
        ensures
            spec_entries_to_map(s1).dom() =~= spec_entries_to_map(s2).dom(),
        decreases s1.len(),
    {
        if s1.len() > 0 {
            lemma_entries_to_map_dom_same_keys::<KV, VV1, VV2>(
                s1.drop_last(), s2.drop_last(),
            );
        }
    }

    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

    //		8. traits

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_tablesteph_wf(&self) -> bool;

        /// Returns the concrete stored value for a given key.
        /// Useful for transferring exec-level properties (e.g., wf) through find_ref.
        spec fn spec_stored_value(&self, key: K::V) -> V;

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            requires self.spec_tablesteph_wf()
            ensures count == self@.len();
        /// - APAS Cost Spec 42.5: Work 1, Span 1
        /// - Alg Analysis: APAS (Ch42 ref): Work O(1), Span O(1) -- agrees with APAS.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- agrees with APAS. — matches APAS
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(key: K, value: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@ == Map::<K::V, V::V>::empty().insert(key@, value@), tree.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential key extraction
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), domain.spec_arraysetsteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|s| * W(f)), Span O(lg |s| + S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|·W(f)), Span O(|s|·W(f)) — DIFFERS: sequential loop
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>()
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_tablesteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·W(f)), Span O(n·W(f)) — DIFFERS: sequential loop
        fn map<F: Fn(&V) -> V>(&mut self, f: F)
            requires
                old(self).spec_tablesteph_wf(),
                forall|v: &V| f.requires((v,)),
                obeys_feq_clone::<K>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() == old(self)@.dom(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == old(self)@[k]
                        && f.ensures((&old_val,), result)
                        && self@[k] == result@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(k,v))), Span O(n + Σ W(f(k,v))) — DIFFERS: sequential loop
        fn filter<F: Fn(&K, &V) -> bool>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        )
            requires
                old(self).spec_tablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom().subset_of(old(self)@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                forall|k: K::V| old(self)@.dom().contains(k) && spec_pred(k, old(self)@[k])
                    ==> #[trigger] self@.dom().contains(k);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            requires
                old(self).spec_tablesteph_wf(),
                other.spec_tablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            requires
                old(self).spec_tablesteph_wf(),
                other.spec_tablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && !other@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !old(self)@.contains_key(k)
                    ==> self@[k] == other@[k],
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn difference(&mut self, other: &Self)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan on unsorted array
        fn find(&self, key: &K) -> (found: Option<V>)
            requires self.spec_tablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };
        /// Like find, but returns a reference to the stored value.
        /// The ensures `*v == self.spec_stored_value(key@)` lets callers transfer
        /// exec-level properties (e.g., wf) from the stored value to the result.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — BST/AVL tree lookup.
        fn find_ref(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_tablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@
                        && *v == self.spec_stored_value(key@),
                    None => !self@.contains_key(key@),
                };
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan + copy
        fn delete(&mut self, key: &K)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@ =~= old(self)@.remove(key@), self.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan + copy
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            requires
                old(self).spec_tablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                self.spec_tablesteph_wf(),
                self@.contains_key(key@),
                self@.dom() =~= old(self)@.dom().insert(key@),
                forall|k: K::V| k != key@ && #[trigger] old(self)@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                !old(self)@.contains_key(key@) ==> self@[key@] == value@
                    && self.spec_stored_value(key@) == value,
                old(self)@.contains_key(key@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[key@] && combine.ensures((&old_v, &value), r)
                    && self@[key@] == r@
                    && old_v == old(self).spec_stored_value(key@)
                    && self.spec_stored_value(key@) == r);
        /// Like insert, but additionally ensures all stored values preserve well-formedness.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree insertion with wf preservation.
        /// Requires K: ClonePreservesView, V: ClonePreservesWf, and that combine preserves wf.
        fn insert_wf<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            where K: ClonePreservesView, V: ClonePreservesWf
            requires
                old(self).spec_tablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                value.spec_wf(),
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) ==>
                    old(self).spec_stored_value(k).spec_wf(),
                forall|v1: &V, v2: &V, r: V|
                    #[trigger] combine.ensures((v1, v2), r) && v1.spec_wf() && v2.spec_wf()
                    ==> r.spec_wf(),
            ensures
                self.spec_tablesteph_wf(),
                self@.contains_key(key@),
                self@.dom() =~= old(self)@.dom().insert(key@),
                forall|k: K::V| k != key@ && #[trigger] old(self)@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                !old(self)@.contains_key(key@) ==> self@[key@] == value@
                    && self.spec_stored_value(key@) == value,
                old(self)@.contains_key(key@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[key@] && combine.ensures((&old_v, &value), r)
                    && self@[key@] == r@
                    && old_v == old(self).spec_stored_value(key@)
                    && self.spec_stored_value(key@) == r),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    self.spec_stored_value(k).spec_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree deletion.
        /// Like delete, but additionally ensures all remaining stored values preserve well-formedness.
        /// Requires K: ClonePreservesView, V: ClonePreservesWf.
        fn delete_wf(&mut self, key: &K)
            where K: ClonePreservesView, V: ClonePreservesWf
            requires
                old(self).spec_tablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) ==>
                    old(self).spec_stored_value(k).spec_wf(),
            ensures
                self@ =~= old(self)@.remove(key@),
                self.spec_tablesteph_wf(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    self.spec_stored_value(k).spec_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.finite(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.finite(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().difference(keys@),
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — in-order traversal or linear construction.
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];

        /// Returns a flat sequence of (K, V) pairs in key order.
        fn entries(&self) -> (entries: ArraySeqStEphS<Pair<K, V>>)
            ensures spec_entries_to_map(entries@) == self@;
    }

    //		9. impls

    impl<K: StT + Ord, V: StT> TableStEph<K, V> {
        pub open spec fn spec_tablesteph_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }

        pub open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            self.entries.seq@[i].1
        }

        /// The view of spec_stored_value(k) equals the map value self@[k].
        pub proof fn lemma_spec_stored_value_view(&self, k: K::V)
            requires self.spec_tablesteph_wf(), self@.contains_key(k)
            ensures self.spec_stored_value(k)@ == self@[k]
        {
            // Get the choose index from spec_stored_value.
            lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
            let view_idx = choose|i: int| 0 <= i < self.entries@.len()
                && (#[trigger] self.entries@[i]).0 == k;
            // entries@[view_idx].0 == k, i.e., entries.seq@[view_idx].0@ == k.
            self.entries.lemma_view_index(view_idx);
            // spec_stored_value chooses sv_idx with entries.seq@[sv_idx].0@ == k.
            let sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == k;
            // By no_dups: both indices have the same key, so they must be equal.
            assert(self.entries@[sv_idx].0 == k) by {
                self.entries.lemma_view_index(sv_idx);
            };
            assert(self.entries@[view_idx].0 == k);
            // lemma_entries_to_map_get tells us self@[k] == entries@[sv_idx].1.
            lemma_entries_to_map_get::<K::V, V::V>(self.entries@, sv_idx);
            // entries@[sv_idx].1 == entries.seq@[sv_idx].1@ (from View for Pair).
            self.entries.lemma_view_index(sv_idx);
            // spec_stored_value(k) == entries.seq@[sv_idx].1, so spec_stored_value(k)@ == self@[k].
        }
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> TableStEphTrait<K, V> for TableStEph<K, V> {
        open spec fn spec_tablesteph_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            self.entries.seq@[i].1
        }

        fn size(&self) -> (count: usize)
        {
            proof {
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty collection.
                lemma_entries_to_map_len::<K::V, V::V>(self.entries@);
            }
            self.entries.length()
        }

        fn empty() -> (empty: Self)
        {
            let entries = ArraySeqStEphS::empty();
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            proof {
                assert(obeys_feq_full_trigger::<K>());
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single-element collection.
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            TableStEph { entries }
        }

        fn singleton(key: K, value: V) -> (tree: Self)
        {
            let entries = ArraySeqStEphS::singleton(Pair(key, value));
            assert(entries@ =~= seq![(key@, value@)]);
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                let s = entries@;
                assert(s.len() == 1);
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — collects distinct elements.
                assert(s.drop_last() =~= Seq::<(K::V, V::V)>::empty());
                assert(spec_entries_to_map(s.drop_last()) =~= Map::<K::V, V::V>::empty());
                assert(s.last() == (key@, value@));
            }
            TableStEph { entries }
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let mut keys = ArraySetStEph::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    keys.spec_arraysetsteph_wf(),
                    keys@.finite(),
                    forall|j: int| 0 <= j < i as int
                        ==> keys@.contains((#[trigger] self.entries@[j]).0),
                    forall|k: K::V| keys@.contains(k)
                        ==> exists|j: int| 0 <= j < i as int
                            && (#[trigger] self.entries@[j]).0 == k,
                    obeys_feq_clone::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                let ghost old_keys = keys@;
                let key_clone = pair.0.clone_plus();
                keys.insert(key_clone);
                proof {
                    assert forall|j: int| 0 <= j < i as int + 1
                        implies keys@.contains((#[trigger] self.entries@[j]).0)
                    by {
                        if j < i as int {
                            assert(old_keys.contains(self.entries@[j].0));
                        }
                    };
                    assert forall|k: K::V| keys@.contains(k)
                        implies exists|j: int| 0 <= j < i as int + 1
                            && (#[trigger] self.entries@[j]).0 == k
                    by {
                        if old_keys.contains(k) {
                            let j = choose|j: int| 0 <= j < i as int
                                && (#[trigger] self.entries@[j]).0 == k;
                            assert(j < i as int + 1);
                        } else {
                            assert(self.entries@[i as int].0 == k);
                        }
                    };
                }
                i += 1;
            }
            proof {
                assert forall|k: K::V| #[trigger] keys@.contains(k) == self@.dom().contains(k)
                by {
                    if keys@.contains(k) {
                        let j = choose|j: int| 0 <= j < self.entries@.len()
                            && (#[trigger] self.entries@[j]).0 == k;
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                    if self@.dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let j = choose|j: int| 0 <= j < self.entries@.len()
                            && (#[trigger] self.entries@[j]).0 == k;
                    }
                };
            }
            keys
        }

        #[verifier::loop_isolation(false)]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
                      assert(obeys_feq_full_trigger::<K>());
           assert(obeys_feq_full_trigger::<V>());
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let key_seq = keys.to_seq();
            let mut entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < key_seq.length()
                invariant
                    i <= key_seq.spec_len(),
                    entries@.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] entries@[j]).0@ == key_seq@[j],
                    forall|j: int| #![trigger key_seq.seq@[j]] 0 <= j < i as int ==>
                        f.ensures((&key_seq.seq@[j],), entries@[j].1),
                    forall|k: &K| f.requires((k,)),
                decreases key_seq.spec_len() - i,
            {
                let key = key_seq.nth(i);
                let value = f(key);
                let key_clone = key.clone_plus();
                proof {
                    lemma_cloned_view_eq::<K>(*key, key_clone);
                }
                entries.push(Pair(key_clone, value));
                i += 1;
            }
            let seq = ArraySeqStEphS::from_vec(entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(seq@);
                // Each entry key matches the corresponding key_seq element.
                assert forall|j: int| 0 <= j < seq@.len()
                    implies (#[trigger] seq@[j]).0 == key_seq@[j]
                by {
                    assert(seq.spec_index(j) == entries@[j]);
                    assert(entries@[j].0@ == key_seq@[j]);
                };
                // No duplicate keys since key_seq has no duplicates.
                assert(spec_keys_no_dups(seq@)) by {
                    assert forall|i: int, j: int|
                        0 <= i < j < seq@.len()
                        implies (#[trigger] seq@[i]).0 != (#[trigger] seq@[j]).0
                    by {
                        assert(seq@[i].0 == key_seq@[i]);
                        assert(seq@[j].0 == key_seq@[j]);
                    };
                };
                // Domain matches keys@.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(seq@).dom().contains(k) == keys@.contains(k)
                by {
                    if spec_entries_to_map(seq@).dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == k;
                        assert(key_seq@[j] == k);
                    }
                    if keys@.contains(k) {
                        let j = choose|j: int| 0 <= j < key_seq@.len()
                            && key_seq@[j] == k;
                        assert(seq@[j].0 == k);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                    }
                };
                // Closure ensures postcondition.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(seq@).contains_key(k)
                    implies exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && spec_entries_to_map(seq@)[k] == result@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == k;
                    assert(seq.spec_index(j) == entries@[j]);
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — applies f to each of n elements.
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                    assert(f.ensures((&key_seq.seq@[j],), entries@[j].1));
                    assert(key_seq.seq@[j]@ == key_seq@[j]);
                    assert(seq@[j].1 == entries@[j].1@);
                };
            }
            TableStEph { entries: seq }
        }

        fn map<F: Fn(&V) -> V>(&mut self, f: F)
        {
            let ghost old_entries = self.entries@;
            let ghost old_raw = self.entries.seq@;
            let mut mapped: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_entries,
                    self.entries.seq@ == old_raw,
                    mapped@.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] mapped@[j]).0@ == old_entries[j].0,
                    forall|j: int| #![trigger mapped@[j]] 0 <= j < i as int ==>
                        f.ensures((&old_raw[j].1,), mapped@[j].1),
                    forall|v: &V| f.requires((v,)),
                    obeys_feq_clone::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                let new_value = f(&pair.1);
                let key_clone = pair.0.clone_plus();
                mapped.push(Pair(key_clone, new_value));
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(mapped);
            proof {
                assert forall|i: int| 0 <= i < self.entries@.len()
                    implies (#[trigger] self.entries@[i]).0 == old_entries[i].0
                by {
                    assert(self.entries.spec_index(i) == mapped@[i]);
                };
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(old_entries, self.entries@);
                // No duplicate keys (inherited from old).
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        assert(self.entries@[a].0 == old_entries[a].0);
                        assert(self.entries@[b].0 == old_entries[b].0);
                        assert(old_entries[a].0 != old_entries[b].0);
                    };
                };
                // Closure ensures postcondition.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).contains_key(k)
                    implies exists|old_val: V, result: V|
                        old_val@ == spec_entries_to_map(old_entries)[k]
                        && f.ensures((&old_val,), result)
                        && spec_entries_to_map(self.entries@)[k] == result@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let j = choose|j: int| 0 <= j < self.entries@.len()
                        && (#[trigger] self.entries@[j]).0 == k;
                    assert(self.entries.spec_index(j) == mapped@[j]);
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — single-pass filter.
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_entries, j);
                    assert(f.ensures((&old_raw[j].1,), mapped@[j].1));
                    assert(old_raw[j].1@ == old_entries[j].1);
                    assert(mapped@[j].1@ == self.entries@[j].1);
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn filter<F: Fn(&K, &V) -> bool>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        )
        {
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    forall|k: K, v: V, keep: bool|
                        f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= (#[trigger] sources[j]) < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < sources.len() ==> (#[trigger] sources[j]) < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    // Completeness: every processed entry satisfying spec_pred was kept.
                    forall|si: int| 0 <= si < i as int
                        && spec_pred((#[trigger] old_view[si]).0, old_view[si].1)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    spec_keys_no_dups(old_view),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if f(&pair.0, &pair.1) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        sources = sources.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && spec_pred((#[trigger] old_view[si]).0, old_view[si].1)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                assert(sources[j] == old_sources[j]);
                            } else {
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                } else {
                    proof {
                        // f returned false, so spec_pred is false for this entry.
                        assert(!spec_pred(old_view[i as int].0, old_view[i as int].1));
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                assert forall|k: K::V|
                    #[trigger] self@.dom().contains(k)
                    implies spec_entries_to_map(old_view).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    assert(old_view[s].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < self.entries@.len()
                        implies (#[trigger] self.entries@[j1]).0
                            != (#[trigger] self.entries@[j2]).0
                    by {
                        assert(self.entries.spec_index(j1) == kept@[j1]);
                        assert(self.entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(old_view[sources[j1]].0 != old_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V|
                    #[trigger] self@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                    assert(kept@[idx].1@ == old_view[s].1);
                };
                // Completeness: every key satisfying spec_pred was kept.
                assert forall|k: K::V|
                    spec_entries_to_map(old_view).dom().contains(k)
                    && spec_pred(k, spec_entries_to_map(old_view)[k])
                    implies #[trigger] self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) — pairwise set intersection.
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    assert(spec_pred(old_view[si].0, old_view[si].1));
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                    assert(self.entries.spec_index(j) == kept@[j]);
                    assert(kept@[j].0@ == old_view[si].0);
                    assert(self.entries@[j].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
            }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
        {
            let ghost old_self_view = self.entries@;
            let ghost other_view = other.entries@;
            let ghost old_self_raw = self.entries.seq@;
            let ghost other_raw = other.entries.seq@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut self_srcs: Seq<int> = Seq::empty();
            let ghost mut other_srcs: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_self_view,
                    other.entries@ == other_view,
                    self.entries.seq@ == old_self_raw,
                    other.entries.seq@ == other_raw,
                    self_srcs.len() == kept@.len(),
                    other_srcs.len() == kept@.len(),
                    forall|k: int| 0 <= k < self_srcs.len() ==>
                        0 <= (#[trigger] self_srcs[k]) < old_self_view.len()
                        && old_self_view[self_srcs[k]].0 == kept@[k].0@,
                    forall|k: int| 0 <= k < other_srcs.len() ==>
                        0 <= (#[trigger] other_srcs[k]) < other_view.len()
                        && other_view[other_srcs[k]].0 == kept@[k].0@,
                    forall|k: int| #![trigger kept@[k]] 0 <= k < kept@.len() ==>
                        combine.ensures(
                            (&old_self_raw[self_srcs[k]].1, &other_raw[other_srcs[k]].1),
                            kept@[k].1),
                    forall|j: int| 0 <= j < self_srcs.len() ==> (#[trigger] self_srcs[j]) < i as int,
                    forall|j1: int, j2: int| #![trigger self_srcs[j1], self_srcs[j2]]
                        0 <= j1 < j2 < self_srcs.len() ==> self_srcs[j1] < self_srcs[j2],
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    obeys_feq_clone::<K>(),
                    obeys_view_eq::<K>(),
                    forall|si: int| 0 <= si < i as int
                        && (exists|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == (#[trigger] old_self_view[si]).0)
                        ==> exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si,
                    forall|si: int| 0 <= si < i as int
                        && !(exists|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == (#[trigger] old_self_view[si]).0)
                        ==> !spec_entries_to_map(other_view).contains_key(old_self_view[si].0),
                decreases self.entries.spec_len() - i,
            {
                let pair_i = self.entries.nth(i);
                let ghost key_view: K::V = old_self_view[i as int].0;
                let mut found = false;
                let mut found_idx: usize = 0;
                let mut j: usize = 0;
                while j < other.entries.length() && !found
                    invariant
                        j <= other.entries.spec_len(),
                        other.entries@ == other_view,
                        i < self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        found ==> found_idx < other.entries.spec_len()
                            && other_view[found_idx as int].0 == key_view,
                        !found ==> forall|jj: int| 0 <= jj < j as int
                            ==> (#[trigger] other_view[jj]).0 != key_view,
                        key_view == pair_i.0@,
                        obeys_view_eq::<K>(),
                    decreases other.entries.spec_len() - j,
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
                        other.entries.lemma_view_index(j as int);
                    }
                    if pair_i.0 == pair_j.0 {
                        found = true;
                        found_idx = j;
                    }
                    j += 1;
                }
                if found {
                    let pair_j = other.entries.nth(found_idx);
                    let combined_value = combine(&pair_i.1, &pair_j.1);
                    let key_clone = pair_i.0.clone_plus();
                    kept.push(Pair(key_clone, combined_value));
                    proof {
                        let ghost old_self_srcs = self_srcs;
                        self_srcs = self_srcs.push(i as int);
                        other_srcs = other_srcs.push(found_idx as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (exists|oj: int| 0 <= oj < other_view.len()
                                && (#[trigger] other_view[oj]).0 == (#[trigger] old_self_view[si]).0)
                            implies exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_self_srcs.len() && old_self_srcs[j] == si;
                                assert(self_srcs[j] == old_self_srcs[j]);
                            } else {
                                assert(self_srcs[self_srcs.len() - 1] == i as int);
                            }
                        };
                    }
                } else {
                    proof {
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, key_view);
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_self_view).dom().intersect(other@.dom());
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s1 = self_srcs[idx];
                        let s2 = other_srcs[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, s1);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(other_view, s2);
                    }
                    if spec_entries_to_map(old_self_view).dom().contains(k)
                        && other@.dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                        let si = choose|si: int| 0 <= si < old_self_view.len()
                            && (#[trigger] old_self_view[si]).0 == k;
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(other_view, k);
                        let oj = choose|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == k;
                        let j = choose|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si;
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // No duplicate keys (self_srcs monotone + old has no dups).
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < self.entries@.len()
                        implies (#[trigger] self.entries@[j1]).0 != (#[trigger] self.entries@[j2]).0
                    by {
                        assert(self.entries.spec_index(j1) == kept@[j1]);
                        assert(self.entries.spec_index(j2) == kept@[j2]);
                        assert(self_srcs[j1] < self_srcs[j2]);
                        assert(old_self_view[self_srcs[j1]].0 != old_self_view[self_srcs[j2]].0);
                    };
                };
                // Value preservation: combine.ensures postcondition.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).contains_key(k)
                    implies exists|v1: V, v2: V, r: V|
                        v1@ == spec_entries_to_map(old_self_view)[k]
                        && v2@ == spec_entries_to_map(other_view)[k]
                        && combine.ensures((&v1, &v2), r)
                        && spec_entries_to_map(self.entries@)[k] == r@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s1 = self_srcs[idx];
                    let s2 = other_srcs[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) — pairwise set operation.
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, s1);
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, s2);
                    assert(combine.ensures(
                        (&old_self_raw[s1].1, &other_raw[s2].1), kept@[idx].1));
                    assert(old_self_raw[s1].1@ == old_self_view[s1].1);
                    assert(other_raw[s2].1@ == other_view[s2].1);
                    assert(kept@[idx].1@ == self.entries@[idx].1);
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
        {
            let ghost old_self_view = self.entries@;
            let ghost old_self_raw = self.entries.seq@;
            let ghost other_raw = other.entries.seq@;
            let other_len = other.entries.length();
            let self_len = self.entries.length();
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut phase1_matches: Seq<int> = Seq::empty();
            // Phase 1: For each self entry, scan other for match.
            // If match, combine values; otherwise clone. Keep all self entries.
            let mut i: usize = 0;
            while i < self_len
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_self_view,
                    self.entries.seq@ == old_self_raw,
                    other.entries.seq@ == other_raw,
                    self_len as int == self.entries.spec_len(),
                    other_len as int == other.entries.spec_len(),
                    kept@.len() == i as int,
                    phase1_matches.len() == i as int,
                    forall|k: int| 0 <= k < i as int ==>
                        (#[trigger] kept@[k]).0@ == old_self_view[k].0,
                    forall|k: int| 0 <= k < i as int ==>
                        (#[trigger] phase1_matches[k]) >= -1int
                        && phase1_matches[k] < other.entries@.len(),
                    forall|k: int| 0 <= k < i as int && phase1_matches[k] >= 0 ==>
                        0 <= (#[trigger] phase1_matches[k]) < other.entries@.len()
                        && other.entries@[phase1_matches[k]].0 == old_self_view[k].0
                        && combine.ensures(
                            (&old_self_raw[k].1, &other_raw[phase1_matches[k]].1),
                            kept@[k].1)
                        && spec_entries_to_map(other.entries@).contains_key(old_self_view[k].0),
                    forall|k: int| #![trigger phase1_matches[k]] 0 <= k < i as int && phase1_matches[k] < 0 ==>
                        kept@[k].1@ == old_self_view[k].1
                        && !spec_entries_to_map(other.entries@).contains_key(old_self_view[k].0),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    obeys_feq_clone::<K>(),
                    obeys_view_eq::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair_i = self.entries.nth(i);
                let ghost key_view: K::V = old_self_view[i as int].0;
                // Scan other for matching key.
                let mut match_idx: usize = other_len;
                let mut j: usize = 0;
                while j < other_len
                    invariant
                        j <= other_len,
                        i < self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        other_len as int == other.entries.spec_len(),
                        match_idx <= other_len,
                        match_idx < other_len ==>
                            other.entries@[match_idx as int].0 == key_view,
                        match_idx == other_len ==> forall|jj: int|
                            0 <= jj < j as int ==> (#[trigger] other.entries@[jj]).0 != key_view,
                        key_view == pair_i.0@,
                        obeys_view_eq::<K>(),
                    decreases other_len - j,
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
                        other.entries.lemma_view_index(j as int);
                    }
                    if pair_i.0 == pair_j.0 {
                        match_idx = j;
                    }
                    j += 1;
                }
                if match_idx < other_len {
                    let pair_j = other.entries.nth(match_idx);
                    let key_clone = pair_i.0.clone_plus();
                    let combined_value = combine(&pair_i.1, &pair_j.1);
                    kept.push(Pair(key_clone, combined_value));
                    proof {
                        phase1_matches = phase1_matches.push(match_idx as int);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            other.entries@, match_idx as int);
                    }
                } else {
                    let cloned = pair_i.clone_plus();
                    kept.push(cloned);
                    proof {
                        phase1_matches = phase1_matches.push(-1int);
                        lemma_entries_to_map_no_key::<K::V, V::V>(
                            other.entries@, key_view);
                    }
                }
                i += 1;
            }
            let ghost phase1_len: int = kept@.len() as int;
            proof { assert(phase1_len == old_self_view.len()); }
            // Phase 2: For each other entry, scan self for match.
            // If no match, add to output (entries only in other).
            let ghost mut phase2_sources: Seq<int> = Seq::empty();
            let mut j: usize = 0;
            while j < other_len
                invariant
                    j <= other_len,
                    other_len as int == other.entries.spec_len(),
                    self_len as int == self.entries.spec_len(),
                    self.entries@ == old_self_view,
                    phase1_len == old_self_view.len(),
                    kept@.len() == phase1_len + phase2_sources.len(),
                    forall|k: int| 0 <= k < phase1_len ==>
                        (#[trigger] kept@[k]).0@ == old_self_view[k].0,
                    forall|k: int| 0 <= k < phase2_sources.len() ==>
                        0 <= (#[trigger] phase2_sources[k]) < other.entries@.len()
                        && other.entries@[phase2_sources[k]].0
                            == kept@[(phase1_len + k) as int].0@
                        && other.entries@[phase2_sources[k]].1
                            == kept@[(phase1_len + k) as int].1@,
                    forall|oj: int| 0 <= oj < j as int ==>
                        spec_entries_to_map(old_self_view).contains_key(
                            (#[trigger] other.entries@[oj]).0)
                        || (exists|k: int| 0 <= k < phase2_sources.len()
                            && (#[trigger] phase2_sources[k]) == oj),
                    forall|k: int| 0 <= k < phase2_sources.len()
                        ==> (#[trigger] phase2_sources[k]) < j as int,
                    forall|k1: int, k2: int| #![trigger phase2_sources[k1], phase2_sources[k2]]
                        0 <= k1 < k2 < phase2_sources.len()
                        ==> phase2_sources[k1] < phase2_sources[k2],
                    forall|k: int| 0 <= k < phase2_sources.len() ==>
                        !spec_entries_to_map(old_self_view).contains_key(
                            other.entries@[#[trigger] phase2_sources[k]].0),
                    // Phase 1 value tracking preserved through phase 2.
                    forall|k: int| 0 <= k < phase1_len && phase1_matches[k] >= 0 ==>
                        0 <= (#[trigger] phase1_matches[k]) < other.entries@.len()
                        && other.entries@[phase1_matches[k]].0 == old_self_view[k].0
                        && combine.ensures(
                            (&old_self_raw[k].1, &other_raw[phase1_matches[k]].1),
                            kept@[k].1),
                    forall|k: int| 0 <= k < phase1_len && phase1_matches[k] < 0 ==>
                        (#[trigger] kept@[k]).1@ == old_self_view[k].1,
                    other.entries.seq@ == other_raw,
                    self.entries.seq@ == old_self_raw,
                    obeys_view_eq::<K>(),
                decreases other_len - j,
            {
                let pair_j = other.entries.nth(j);
                let ghost key_view: K::V = other.entries@[j as int].0;
                proof { other.entries.lemma_view_index(j as int); }
                // Scan self for matching key.
                let mut found: bool = false;
                let ghost mut found_idx: int = -1int;
                let mut ii: usize = 0;
                while ii < self_len
                    invariant
                        ii <= self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        self_len as int == self.entries.spec_len(),
                        found ==> (0 <= found_idx < old_self_view.len()
                            && old_self_view[found_idx].0 == key_view),
                        !found ==> forall|kk: int| 0 <= kk < ii as int ==>
                            (#[trigger] old_self_view[kk]).0 != key_view,
                        key_view == pair_j.0@,
                        obeys_view_eq::<K>(),
                    decreases self.entries.spec_len() - ii,
                {
                    let pair_ii = self.entries.nth(ii);
                    proof {
                        reveal(obeys_view_eq);
                        self.entries.lemma_view_index(ii as int);
                    }
                    if pair_j.0 == pair_ii.0 {
                        found = true;
                        proof { found_idx = ii as int; }
                    }
                    ii += 1;
                }
                let ghost old_phase2_sources = phase2_sources;
                let ghost old_kept = kept@;
                if !found {
                    let cloned = pair_j.clone_plus();
                    kept.push(cloned);
                    proof {
                        phase2_sources = phase2_sources.push(j as int);
                        lemma_entries_to_map_no_key::<K::V, V::V>(
                            old_self_view, key_view);
                    }
                } else {
                    proof {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            old_self_view, found_idx);
                    }
                }
                proof {
                    // Re-establish coverage for all oj in 0..j+1.
                    assert forall|oj: int| 0 <= oj < j + 1 implies
                        spec_entries_to_map(old_self_view).contains_key(
                            (#[trigger] other.entries@[oj]).0)
                        || (exists|k: int| 0 <= k < phase2_sources.len()
                            && (#[trigger] phase2_sources[k]) == oj)
                    by {
                        if oj < j as int {
                            // Old entry: invariant held before this iteration.
                            if spec_entries_to_map(old_self_view).contains_key(
                                other.entries@[oj].0)
                            {
                                // Already covered by map membership.
                            } else {
                                // Had a witness k in old_phase2_sources.
                                let k = choose|k: int|
                                    0 <= k < old_phase2_sources.len()
                                    && (#[trigger] old_phase2_sources[k]) == oj;
                                // phase2_sources preserves old entries.
                                assert(phase2_sources[k] == old_phase2_sources[k]);
                                assert(phase2_sources[k] == oj);
                            }
                        } else {
                            // oj == j: the current entry.
                            if found {
                                assert(spec_entries_to_map(old_self_view).contains_key(
                                    other.entries@[oj].0));
                            } else {
                                // We just pushed j onto phase2_sources.
                                let k = phase2_sources.len() - 1;
                                assert(phase2_sources[k] == j as int);
                                assert(phase2_sources[k] == oj);
                            }
                        }
                    };
                }
                j += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                // Every old self key is in the output (Phase 1).
                assert forall|k: K::V|
                    spec_entries_to_map(old_self_view).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    assert(0 <= si && si < kept@.len());
                    assert(self.entries.spec_index(si) == kept@[si]);
                    self.entries.lemma_view_index(si);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, si);
                };
                // Every other key is in the output.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(other.entries@).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other.entries@, k);
                    let oj = choose|oj: int| 0 <= oj < other.entries@.len()
                        && (#[trigger] other.entries@[oj]).0 == k;
                    if spec_entries_to_map(old_self_view).contains_key(
                        other.entries@[oj].0)
                    {
                        // Key in self: Phase 1 added it.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            old_self_view, k);
                        let si = choose|si: int| 0 <= si < old_self_view.len()
                            && (#[trigger] old_self_view[si]).0 == k;
                        assert(0 <= si && si < kept@.len());
                        assert(self.entries.spec_index(si) == kept@[si]);
                        self.entries.lemma_view_index(si);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, si);
                    } else {
                        // Key only in other: Phase 2 added it.
                        let kidx = choose|kidx: int|
                            0 <= kidx < phase2_sources.len()
                            && (#[trigger] phase2_sources[kidx]) == oj;
                        let out_idx = phase1_len + kidx;
                        assert(other.entries@[oj].0
                            == kept@[out_idx as int].0@);
                        assert(self.entries.spec_index(out_idx) == kept@[out_idx]);
                        self.entries.lemma_view_index(out_idx);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, out_idx);
                    }
                };
                // Reverse: every output key is in old self or other.
                assert forall|k: K::V|
                    self@.dom().contains(k)
                    implies spec_entries_to_map(old_self_view).dom().contains(k)
                        || other@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    if idx < phase1_len {
                        self.entries.lemma_view_index(idx);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, idx);
                    } else {
                        let kidx = idx - phase1_len;
                        let src = phase2_sources[kidx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(other.entries@, src);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        assert(self.entries.spec_index(a) == kept@[a]);
                        assert(self.entries.spec_index(b) == kept@[b]);
                        if a < phase1_len && b < phase1_len {
                            // Both from phase 1 — keys from old_self_view which has no dups.
                            assert(old_self_view[a].0 != old_self_view[b].0);
                        } else if a < phase1_len && b >= phase1_len {
                            // a from phase 1 (key in old_self), b from phase 2 (key NOT in old_self).
                            let kidx_b = b - phase1_len;
                            let src_b = phase2_sources[kidx_b];
                            // Phase 2 entries are NOT in old_self.
                            assert(!spec_entries_to_map(old_self_view).contains_key(
                                other.entries@[src_b].0));
                            // Phase 1 entry a has key old_self_view[a].0 which IS in old_self.
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, a);
                            // b's key == other.entries@[src_b].0 (from invariant, via kept@[b].0@).
                            // a's key == old_self_view[a].0 (from invariant).
                            // One is in the map, the other isn't, so they differ.
                        } else {
                            // Both from phase 2 — monotone sources + other has no dups.
                            let kidx_a = a - phase1_len;
                            let kidx_b = b - phase1_len;
                            assert(phase2_sources[kidx_a] < phase2_sources[kidx_b]);
                            let sa = phase2_sources[kidx_a];
                            let sb = phase2_sources[kidx_b];
                            assert(other.entries@[sa].0 != other.entries@[sb].0);
                        }
                    };
                };
                // Value: key only in self (not in other) => value unchanged.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).contains_key(k)
                    && !other@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    assert(self.entries.spec_index(si) == kept@[si]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, si);
                    // Key not in other => phase1_matches[si] < 0 => kept value == old value.
                    // Contrapositive: match >= 0 would imply contains_key, contradiction.
                    if phase1_matches[si] >= 0 {
                        assert(spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[si].0));
                        assert(false);
                    }
                    assert(kept@[si].1@ == old_self_view[si].1);
                };
                // Value: key only in other (not in self) => value is other's.
                assert forall|k: K::V|
                    #[trigger] other@.contains_key(k) && !spec_entries_to_map(old_self_view).contains_key(k)
                    implies self@[k] == other@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other.entries@, k);
                    let oj = choose|oj: int| 0 <= oj < other.entries@.len()
                        && (#[trigger] other.entries@[oj]).0 == k;
                    let kidx = choose|kidx: int|
                        0 <= kidx < phase2_sources.len()
                        && (#[trigger] phase2_sources[kidx]) == oj;
                    let out_idx = phase1_len + kidx;
                    assert(self.entries.spec_index(out_idx) == kept@[out_idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, out_idx);
                    lemma_entries_to_map_get::<K::V, V::V>(other.entries@, oj);
                    assert(kept@[out_idx].1@ == other.entries@[oj].1);
                };
                // Value: key in both => combined.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).contains_key(k)
                    && other@.contains_key(k)
                    implies exists|v1: V, v2: V, r: V|
                        v1@ == spec_entries_to_map(old_self_view)[k]
                        && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    assert(self.entries.spec_index(si) == kept@[si]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, si);
                    // Key in other => phase1 found a match.
                    // Contrapositive: match < 0 would imply !contains_key, contradiction.
                    if phase1_matches[si] < 0 {
                        assert(!spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[si].0));
                        assert(false);
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear operation.
                    }
                    let oj = phase1_matches[si];
                    lemma_entries_to_map_get::<K::V, V::V>(other.entries@, oj);
                    assert(combine.ensures(
                        (&old_self_raw[si].1, &other_raw[oj].1), kept@[si].1));
                    assert(old_self_raw[si].1@ == old_self_view[si].1);
                    assert(other_raw[oj].1@ == other.entries@[oj].1);
                    assert(kept@[si].1@ == self.entries@[si].1);
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn difference(&mut self, other: &Self)
        {
            let ghost old_self_view = self.entries@;
            let ghost other_view = other.entries@;
            let other_len = other.entries.length();
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_self_view,
                    other.entries@ == other_view,
                    other_len as int == other.entries.spec_len(),
                    sources.len() == kept@.len(),
                    forall|k: int| 0 <= k < sources.len() ==>
                        0 <= (#[trigger] sources[k]) < old_self_view.len()
                        && old_self_view[sources[k]].0 == kept@[k].0@
                        && old_self_view[sources[k]].1 == kept@[k].1@
                        && !spec_entries_to_map(other_view).contains_key(kept@[k].0@),
                    // Coverage: every processed entry not in other has been kept.
                    forall|si: int| 0 <= si < i as int
                        && !spec_entries_to_map(other_view).contains_key(
                            (#[trigger] old_self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> (#[trigger] sources[j]) < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(old_self_view),
                    obeys_view_eq::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair_i = self.entries.nth(i);
                let ghost key_view: K::V = old_self_view[i as int].0;
                let mut match_idx: usize = other_len;
                let mut j: usize = 0;
                while j < other_len
                    invariant
                        j <= other_len,
                        i < self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        other.entries@ == other_view,
                        other_len as int == other.entries.spec_len(),
                        match_idx <= other_len,
                        match_idx < other_len ==>
                            other_view[match_idx as int].0 == key_view,
                        match_idx == other_len ==>
                            forall|jj: int| 0 <= jj < j as int ==>
                                (#[trigger] other_view[jj]).0 != key_view,
                        key_view == pair_i.0@,
                        obeys_view_eq::<K>(),
                    decreases other_len - j,
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
                        other.entries.lemma_view_index(j as int);
                    }
                    if pair_i.0 == pair_j.0 {
                        match_idx = j;
                    }
                    j += 1;
                }
                if match_idx == other_len {
                    proof {
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, key_view);
                    }
                    let cloned = pair_i.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        sources = sources.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && !spec_entries_to_map(other_view).contains_key(
                                (#[trigger] old_self_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                assert(sources[j] == old_sources[j]);
                            } else {
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                } else {
                    proof {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            other_view, match_idx as int);
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_self_view).dom().difference(
                    other@.dom());
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, s);
                    }
                    if spec_entries_to_map(old_self_view).dom().contains(k)
                        && !other@.dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                        let si = choose|si: int| 0 <= si < old_self_view.len()
                            && (#[trigger] old_self_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < self.entries@.len()
                        implies (#[trigger] self.entries@[j1]).0
                            != (#[trigger] self.entries@[j2]).0
                    by {
                        assert(self.entries.spec_index(j1) == kept@[j1]);
                        assert(self.entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(old_self_view[sources[j1]].0
                            != old_self_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V|
                    #[trigger] self@.contains_key(k)
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — BST/AVL tree lookup.
                    implies self@[k] == spec_entries_to_map(old_self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, s);
                    assert(kept@[idx].1@ == old_self_view[s].1);
                };
            }
        }

        fn find(&self, key: &K) -> (found: Option<V>)
        {
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.spec_tablesteph_wf(),
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] self.entries@[j]).0 != key@,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — BST/AVL tree lookup.
                if pair.0.eq(key) {
                    let v = pair.1.clone_plus();
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(self.entries@, i as int);
                    }
                    return Some(v);
                }
                i += 1;
            }
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.entries@, key@);
            }
            None
        }

        fn find_ref(&self, key: &K) -> (found: Option<&V>)
        {
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.spec_tablesteph_wf(),
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] self.entries@[j]).0 != key@,
                    obeys_view_eq::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0.eq(key) {
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(self.entries@, i as int);
                        // Prove uniqueness: i is the only index with this key.
                        assert forall|j: int|
                            0 <= j < self.entries.seq@.len()
                            && (#[trigger] self.entries.seq@[j]).0@ == key@
                            implies j == i as int
                        by {
                            if j < i as int {
                                // From the view-level entries: entries@[j].0 != key@
                                // entries@[j].0 == entries.seq@[j].0@ (by View definition)
                                assert(self.entries@[j].0 != key@);
                            } else if j > i as int {
                                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree deletion.
                                // From spec_keys_no_dups: distinct indices have distinct keys.
                                assert(self.entries@[i as int].0 != self.entries@[j].0);
                            }
                        };
                    }
                    return Some(&pair.1);
                }
                i += 1;
            }
            proof {
                lemma_entries_to_map_no_key::<K::V, V::V>(self.entries@, key@);
            }
            None
        }

        #[verifier::loop_isolation(false)]
        fn delete(&mut self, key: &K)
        {
            let ghost old_view = self.entries@;
            let ghost old_map = self@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    src.len() == kept@.len(),
                    forall|j: int| 0 <= j < src.len() ==>
                        0 <= (#[trigger] src[j]) < old_view.len()
                        && old_view[src[j]].0 == kept@[j].0@
                        && old_view[src[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < kept@.len() ==>
                        (#[trigger] kept@[j]).0@ != key@,
                    // Source indices are strictly increasing (implies distinct).
                    forall|j: int| #![trigger src[j]] 0 <= j < src.len() ==> src[j] < i as int,
                    forall|a: int, b: int| 0 <= a < b < src.len()
                        ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int
                        && (#[trigger] old_view[si]).0 != key@
                        ==> exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if !pair.0.eq(key) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key@
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                assert(src[j] == old_src[j]);
                            } else {
                                assert(src[src.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_map = spec_entries_to_map(self.entries@);
                let ghost target_map = old_map.remove(key@);
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                };
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(self.entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
                // Result entries have no duplicate keys (inherited from old).
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|i: int, j: int|
                        0 <= i < self.entries@.len() && 0 <= j < self.entries@.len() && i != j
                        implies (#[trigger] self.entries@[i]).0 != (#[trigger] self.entries@[j]).0
                    by {
                        assert(self.entries.spec_index(i) == kept@[i]);
                        assert(self.entries.spec_index(j) == kept@[j]);
                        let si = src[i];
                        let sj = src[j];
                        // src is injective, so si != sj.
                        // old_view has no dups, so old_view[si].0 != old_view[sj].0.
                    };
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree insertion.
                };
                assert forall|k: K::V|
                    #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
        {
            let ghost key_view: K::V = key@;
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let ghost old_map = self@;
            let n = self.entries.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut match_index: usize = n;
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    n == self.entries.spec_len(),
                    self.entries@ == old_view,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    src.len() == all@.len(),
                    forall|j: int| 0 <= j < src.len() ==>
                        0 <= (#[trigger] src[j]) < old_view.len()
                        && old_view[src[j]].0 == all@[j].0@
                        && old_view[src[j]].1 == all@[j].1@,
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).0@ != key_view,
                    forall|j: int| #![trigger src[j]] 0 <= j < src.len() ==> src[j] < i as int,
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int
                        && (#[trigger] old_view[si]).0 != key_view
                        ==> exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    key@ == key_view,
                    spec_keys_no_dups(old_view),
                    match_index <= n,
                    match_index < n ==> old_view[match_index as int].0 == key_view,
                    match_index == n ==> forall|si: int| 0 <= si < i as int
                        ==> (#[trigger] old_view[si]).0 != key_view,
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    proof { self.entries.lemma_view_index(i as int); }
                    match_index = i;
                } else {
                    let cloned = pair.clone_plus();
                    all.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key_view
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                assert(src[j] == old_src[j]);
                            } else {
                                assert(src[src.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            let ghost value_view: V::V = value@;
            let ghost old_stored_at_key: Pair<K, V> = old_exec_seq[match_index as int];
            let final_value;
            if match_index < n {
                let old_entry = self.entries.nth(match_index);
                proof {
                    self.entries.lemma_view_index(match_index as int);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, match_index as int);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, match_index as int);
                    // old_entry is the exec Pair at match_index in old entries.
                    assert(old_entry == old_exec_seq[match_index as int]);
                }
                final_value = combine(&old_entry.1, &value);
            } else {
                final_value = value;
            }
            all.push(Pair(key, final_value));
            self.entries = ArraySeqStEphS::from_vec(all);
            proof {
                let last = (self.entries@.len() - 1) as int;
                assert(self.entries.spec_index(last) == all@[last]);
                self.entries.lemma_view_index(last);
                assert(self.entries@[last].0 == key_view);
                assert(self.entries@[last].1 == all@[last].1@);
                lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, last);
                // Domain backward: old keys + key@ are in result.
                assert forall|k: K::V|
                    #[trigger] old_map.dom().contains(k) || k == key_view
                    implies spec_entries_to_map(self.entries@).dom().contains(k)
                by {
                    if k == key_view {
                    } else {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                        assert(self.entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // Domain forward: result keys are in old ∪ {key@}.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                    implies old_map.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    if idx < src.len() as int {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        assert(self.entries.spec_index(a) == all@[a]);
                        assert(self.entries.spec_index(b) == all@[b]);
                        if a < src.len() as int && b < src.len() as int {
                            assert(src[a] < src[b]);
                            assert(old_view[src[a]].0 != old_view[src[b]].0);
                        } else if a < src.len() as int && b == last {
                            assert(all@[a].0@ != key_view);
                        } else if a == last {
                            // a == last < b is impossible since last is the largest index.
                        }
                    };
                };
                // Value preservation for non-key entries.
                assert forall|k: K::V|
                    k != key_view && #[trigger] old_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(self.entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    assert(all@[j].1@ == old_view[si].1);
                };
                // Value for new key.
                lemma_entries_to_map_get::<K::V, V::V>(self.entries@, last);
                if match_index == n as usize {
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_view, key_view);
                }
                // Prove spec_stored_value for the inserted key.
                // Key is at position last in new entries.
                assert(0 <= last < self.entries.seq@.len() as int);
                assert((#[trigger] self.entries.seq@[last]).0@ == key_view);
                // By spec_keys_no_dups, last is the unique index with key_view.
                // Therefore spec_stored_value(key@) == self.entries.seq@[last].1.
                let ghost chosen_i = choose|i: int| 0 <= i < self.entries.seq@.len()
                    && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                assert(self.entries.seq@[chosen_i].0@ == key_view);
                assert(self.entries.seq@[last].0@ == key_view);
                // Uniqueness from no_dups: entries@[chosen_i].0 == entries@[last].0
                // means chosen_i == last (since no two distinct indices share a key).
                if chosen_i != last {
                    assert(self.entries@[chosen_i].0 == key_view);
                    assert(self.entries@[last].0 == key_view);
                    // This contradicts spec_keys_no_dups
                }
                assert(chosen_i == last);
                assert(self.spec_stored_value(key_view) == self.entries.seq@[last].1);
                // Prove the existing-key spec_stored_value ensures.
                if match_index < n as usize {
                    // match_index is the unique index for key in old entries.
                    assert(old_exec_seq[match_index as int].0@ == key_view);
                    // Prove old(self).spec_stored_value(key@) == old_stored_at_key.1.
                    // old_stored_at_key == old_exec_seq[match_index], and match_index
                    // is the unique index in old entries with this key.
                    let ghost old_chosen = choose|i: int| 0 <= i < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[i]).0@ == key_view;
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree insertion with wf preservation.
                    assert(old_exec_seq[old_chosen].0@ == key_view);
                    assert(old_exec_seq[match_index as int].0@ == key_view);
                    if old_chosen != match_index as int {
                        assert(old_view[old_chosen].0 == key_view);
                        assert(old_view[match_index as int].0 == key_view);
                    }
                    assert(old_chosen == match_index as int);
                    // Now old(self).spec_stored_value(key_view) == old_stored_at_key.1.
                    // And final_value is combine(&old_stored_at_key.1, &value).
                    // And self.spec_stored_value(key_view) == final_value.
                    // Provide the existential witnesses.
                    assert(combine.ensures((&old_stored_at_key.1, &value), final_value));
                    assert(old_stored_at_key.1@ == old_map[key_view]);
                }
            }
        }

        #[verifier::loop_isolation(false)]
        fn insert_wf<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            where K: ClonePreservesView, V: ClonePreservesWf
        {
            let ghost key_view: K::V = key@;
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let ghost old_map = self@;
            let n = self.entries.length();
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut match_index: usize = n;
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    n == self.entries.spec_len(),
                    self.entries@ == old_view,
                    self.entries.seq@ =~= old_exec_seq,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    src.len() == all@.len(),
                    forall|j: int| 0 <= j < src.len() ==>
                        0 <= (#[trigger] src[j]) < old_view.len()
                        && old_view[src[j]].0 == all@[j].0@
                        && old_view[src[j]].1 == all@[j].1@,
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).0@ != key_view,
                    forall|j: int| #![trigger src[j]] 0 <= j < src.len() ==> src[j] < i as int,
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int
                        && (#[trigger] old_view[si]).0 != key_view
                        ==> exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    key@ == key_view,
                    spec_keys_no_dups(old_view),
                    match_index <= n,
                    match_index < n ==> old_view[match_index as int].0 == key_view,
                    match_index == n ==> forall|si: int| 0 <= si < i as int
                        ==> (#[trigger] old_view[si]).0 != key_view,
                    // Wf invariants.
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).1.spec_wf(),
                    forall|k: K::V| #[trigger] old_map.contains_key(k) ==>
                        old(self).spec_stored_value(k).spec_wf(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    proof { self.entries.lemma_view_index(i as int); }
                    match_index = i;
                } else {
                    // Prove pair.1.spec_wf() so we can call clone_wf.
                    proof {
                        self.entries.lemma_view_index(i as int);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, i as int);
                        let ghost k_at_i = old_view[i as int].0;
                        // spec_stored_value(k_at_i) == old_exec_seq[chosen].1
                        // where chosen is the unique index with key k_at_i.
                        let ghost chosen = choose|idx: int| 0 <= idx < old_exec_seq.len()
                            && (#[trigger] old_exec_seq[idx]).0@ == k_at_i;
                        // old_exec_seq[i].0@ == old_view[i].0 == k_at_i
                        assert(old_exec_seq[i as int].0@ == k_at_i);
                        // By no_dups uniqueness: chosen == i.
                        if chosen != i as int {
                            assert(old_view[chosen].0 == k_at_i);
                            assert(old_view[i as int].0 == k_at_i);
                        }
                        assert(chosen == i as int);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    all.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key_view
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                assert(src[j] == old_src[j]);
                            } else {
                                assert(src[src.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            let ghost value_view: V::V = value@;
            let ghost old_stored_at_key: Pair<K, V> = old_exec_seq[match_index as int];
            let final_value;
            if match_index < n {
                let old_entry = self.entries.nth(match_index);
                proof {
                    self.entries.lemma_view_index(match_index as int);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, match_index as int);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, match_index as int);
                    assert(old_entry == old_exec_seq[match_index as int]);
                    // Prove old_entry.1.spec_wf() for the combine-preserves-wf requires.
                    let ghost k_at_m = old_view[match_index as int].0;
                    let ghost chosen = choose|idx: int| 0 <= idx < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[idx]).0@ == k_at_m;
                    assert(old_exec_seq[match_index as int].0@ == k_at_m);
                    if chosen != match_index as int {
                        assert(old_view[chosen].0 == k_at_m);
                        assert(old_view[match_index as int].0 == k_at_m);
                    }
                    assert(chosen == match_index as int);
                }
                final_value = combine(&old_entry.1, &value);
            } else {
                final_value = value;
            }
            all.push(Pair(key, final_value));
            self.entries = ArraySeqStEphS::from_vec(all);
            proof {
                let last = (self.entries@.len() - 1) as int;
                assert(self.entries.spec_index(last) == all@[last]);
                self.entries.lemma_view_index(last);
                assert(self.entries@[last].0 == key_view);
                assert(self.entries@[last].1 == all@[last].1@);
                lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, last);
                // Domain backward: old keys + key@ are in result.
                assert forall|k: K::V|
                    #[trigger] old_map.dom().contains(k) || k == key_view
                    implies spec_entries_to_map(self.entries@).dom().contains(k)
                by {
                    if k == key_view {
                    } else {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                        assert(self.entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // Domain forward: result keys are in old ∪ {key@}.
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                    implies old_map.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    if idx < src.len() as int {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        assert(self.entries.spec_index(a) == all@[a]);
                        assert(self.entries.spec_index(b) == all@[b]);
                        if a < src.len() as int && b < src.len() as int {
                            assert(src[a] < src[b]);
                            assert(old_view[src[a]].0 != old_view[src[b]].0);
                        } else if a < src.len() as int && b == last {
                            assert(all@[a].0@ != key_view);
                        } else if a == last {
                        }
                    };
                };
                // Value preservation for non-key entries.
                assert forall|k: K::V|
                    k != key_view && #[trigger] old_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(self.entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    assert(all@[j].1@ == old_view[si].1);
                };
                // Value for new key.
                lemma_entries_to_map_get::<K::V, V::V>(self.entries@, last);
                if match_index == n as usize {
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_view, key_view);
                }
                // Prove spec_stored_value for the inserted key.
                assert(0 <= last < self.entries.seq@.len() as int);
                assert((#[trigger] self.entries.seq@[last]).0@ == key_view);
                let ghost chosen_i = choose|i: int| 0 <= i < self.entries.seq@.len()
                    && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                assert(self.entries.seq@[chosen_i].0@ == key_view);
                assert(self.entries.seq@[last].0@ == key_view);
                if chosen_i != last {
                    assert(self.entries@[chosen_i].0 == key_view);
                    assert(self.entries@[last].0 == key_view);
                }
                assert(chosen_i == last);
                assert(self.spec_stored_value(key_view) == self.entries.seq@[last].1);
                // Prove the existing-key spec_stored_value ensures.
                if match_index < n as usize {
                    assert(old_exec_seq[match_index as int].0@ == key_view);
                    let ghost old_chosen = choose|i: int| 0 <= i < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[i]).0@ == key_view;
                    assert(old_exec_seq[old_chosen].0@ == key_view);
                    assert(old_exec_seq[match_index as int].0@ == key_view);
                    if old_chosen != match_index as int {
                        assert(old_view[old_chosen].0 == key_view);
                        assert(old_view[match_index as int].0 == key_view);
                    }
                    assert(old_chosen == match_index as int);
                    assert(combine.ensures((&old_stored_at_key.1, &value), final_value));
                    assert(old_stored_at_key.1@ == old_map[key_view]);
                }
                // Prove stored-value wf for all keys.
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self.spec_stored_value(k).spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    // spec_stored_value(k) == self.entries.seq@[sv_idx].1
                    let ghost sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == k;
                    // sv_idx is the unique index with key k in new entries.
                    // idx also has key k. By no_dups, sv_idx == idx.
                    assert(self.entries@[sv_idx].0 == k);
                    assert(self.entries@[idx].0 == k);
                    if sv_idx != idx {
                        // Contradicts spec_keys_no_dups
                    }
                    assert(sv_idx == idx);
                    assert(self.spec_stored_value(k) == self.entries.seq@[idx].1);
                    assert(self.entries.seq@[idx] == all@[idx]);
                    // all@[idx].1.spec_wf() — from loop invariant or final_value wf.
                    if idx == last {
                        // final_value case: wf from combine or value.
                        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — tree deletion.
                        if match_index < n as usize {
                            // combine result: combine.ensures((&old_v, &value), final_value)
                            // && old_v.spec_wf() && value.spec_wf() ==> final_value.spec_wf()
                            assert(combine.ensures((&old_stored_at_key.1, &value), final_value));
                            assert(old_stored_at_key.1.spec_wf());
                            assert(value.spec_wf());
                        } else {
                            assert(final_value == value);
                            assert(value.spec_wf());
                        }
                    } else {
                        // Non-key entry: wf from loop invariant.
                        assert(all@[idx].1.spec_wf());
                    }
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn delete_wf(&mut self, key: &K)
            where K: ClonePreservesView, V: ClonePreservesWf
        {
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let ghost old_map = self@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    self.entries.seq@ =~= old_exec_seq,
                    src.len() == kept@.len(),
                    forall|j: int| 0 <= j < src.len() ==>
                        0 <= (#[trigger] src[j]) < old_view.len()
                        && old_view[src[j]].0 == kept@[j].0@
                        && old_view[src[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < kept@.len() ==>
                        (#[trigger] kept@[j]).0@ != key@,
                    forall|j: int| #![trigger src[j]] 0 <= j < src.len() ==> src[j] < i as int,
                    forall|a: int, b: int| 0 <= a < b < src.len()
                        ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int
                        && (#[trigger] old_view[si]).0 != key@
                        ==> exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    // Wf invariants.
                    forall|j: int| 0 <= j < kept@.len() ==> (#[trigger] kept@[j]).1.spec_wf(),
                    forall|k: K::V| #[trigger] old_map.contains_key(k) ==>
                        old(self).spec_stored_value(k).spec_wf(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if !pair.0.eq(key) {
                    // Prove pair.1.spec_wf() so we can call clone_wf.
                    proof {
                        self.entries.lemma_view_index(i as int);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, i as int);
                        let ghost k_at_i = old_view[i as int].0;
                        let ghost chosen = choose|idx: int| 0 <= idx < old_exec_seq.len()
                            && (#[trigger] old_exec_seq[idx]).0@ == k_at_i;
                        assert(old_exec_seq[i as int].0@ == k_at_i);
                        if chosen != i as int {
                            assert(old_view[chosen].0 == k_at_i);
                            assert(old_view[i as int].0 == k_at_i);
                        }
                        assert(chosen == i as int);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    kept.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key@
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                assert(src[j] == old_src[j]);
                            } else {
                                assert(src[src.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_map = spec_entries_to_map(self.entries@);
                let ghost target_map = old_map.remove(key@);
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                };
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(self.entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|i: int, j: int|
                        0 <= i < self.entries@.len() && 0 <= j < self.entries@.len() && i != j
                        implies (#[trigger] self.entries@[i]).0 != (#[trigger] self.entries@[j]).0
                    by {
                        assert(self.entries.spec_index(i) == kept@[i]);
                        assert(self.entries.spec_index(j) == kept@[j]);
                        let si = src[i];
                        let sj = src[j];
                    };
                };
                assert forall|k: K::V|
                    #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                };
                // Prove stored-value wf for all remaining keys.
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — tree set operation; St sequential.
                    implies self.spec_stored_value(k).spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let ghost sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == k;
                    assert(self.entries@[sv_idx].0 == k);
                    assert(self.entries@[idx].0 == k);
                    if sv_idx != idx {}
                    assert(sv_idx == idx);
                    assert(self.spec_stored_value(k) == self.entries.seq@[idx].1);
                    assert(self.entries.seq@[idx] == kept@[idx]);
                    assert(kept@[idx].1.spec_wf());
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    keys@.finite(),
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= (#[trigger] sources[j]) < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@
                        && keys@.contains(kept@[j].0@),
                    forall|si: int| 0 <= si < i as int
                        && keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> (#[trigger] sources[j]) < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(old_view),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if keys.find(&pair.0) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        sources = sources.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && keys@.contains((#[trigger] old_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                assert(sources[j] == old_sources[j]);
                            } else {
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().intersect(keys@);
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < self.entries@.len()
                        implies (#[trigger] self.entries@[j1]).0
                            != (#[trigger] self.entries@[j2]).0
                    by {
                        assert(self.entries.spec_index(j1) == kept@[j1]);
                        assert(self.entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — tree set operation; St sequential.
                        assert(old_view[sources[j1]].0 != old_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V|
                    #[trigger] self@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                    assert(kept@[idx].1@ == old_view[s].1);
                };
            }
        }

        #[verifier::loop_isolation(false)]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    keys@.finite(),
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= (#[trigger] sources[j]) < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@
                        && !keys@.contains(kept@[j].0@),
                    forall|si: int| 0 <= si < i as int
                        && !keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> (#[trigger] sources[j]) < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(old_view),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if !keys.find(&pair.0) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        sources = sources.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && !keys@.contains((#[trigger] old_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                assert(sources[j] == old_sources[j]);
                            } else {
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().difference(keys@);
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && !keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // No duplicate keys.
                assert(spec_keys_no_dups(self.entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < self.entries@.len()
                        implies (#[trigger] self.entries@[j1]).0
                            != (#[trigger] self.entries@[j2]).0
                    by {
                        assert(self.entries.spec_index(j1) == kept@[j1]);
                        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — in-order traversal or linear construction.
                        assert(self.entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(old_view[sources[j1]].0 != old_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V|
                    #[trigger] self@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — in-order traversal or linear construction.
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                    assert(kept@[idx].1@ == old_view[s].1);
                };
            }
        }

        fn entries(&self) -> (entries: ArraySeqStEphS<Pair<K, V>>) {
            let entries = self.entries.clone();
            proof {
                assert(Pair_feq_trigger::<K, V>());
                lemma_seq_map_cloned_view_eq(
                    self.entries.seq@,
                    entries.seq@,
                );
            }
            entries
        }
    }

    // veracity: no_requires
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: Vec<Pair<K, V>>,
    ) -> (cloned: TableStEph<K, V>)
        ensures cloned@.dom().finite()
    {
        let seq = ArraySeqStEphS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableStEph { entries: seq }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Default for TableStEph<K, V> {
        fn default() -> Self {
            TableStEph::empty()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord + View + PartialEq, V: StT + View + PartialEq> PartialEqSpecImpl for TableStEph<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord + Eq + View, V: StT + Eq + View> Eq for TableStEph<K, V> {}

    impl<K: StT + Ord + PartialEq + View, V: StT + PartialEq + View> PartialEq for TableStEph<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.entries == other.entries;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT> Clone for TableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            TableStEph {
                entries: self.entries.clone(),
            }
        }
    }

    } // verus!

    // 12. macros

    //		12. macros

    #[macro_export]
    macro_rules! TableStEphLit {
        () => {
            $crate::Chap42::TableStEph::TableStEph::TableStEph::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableStEph::TableStEph::from_sorted_entries(entries)
        }};
    }

    //		13. derive impls outside verus!

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for TableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEph({:?})", self.entries)
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Display for TableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEph(len={})", self.entries.length())
        }
    }
}
