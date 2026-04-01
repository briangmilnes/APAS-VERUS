//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 42 single-threaded persistent table implementation using ArraySeq as backing store.

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

pub mod TableStPer {

    use std::cmp::Ordering;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
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
    pub struct TableStPer<K: StT + Ord, V: StT> {
        pub entries: ArraySeqStPerS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStPer<K, V>;

    //		5. view impls

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for TableStPer<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }

    //		6. spec fns

    // 6. spec fns

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

    // 6. spec fns

    /// Values in `pairs` whose key equals `k`, preserving order.
    pub open spec fn spec_collect_key<KV, VV>(pairs: Seq<(KV, VV)>, k: KV) -> Seq<VV>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect_key(pairs.drop_last(), k);
            if pairs.last().0 == k {
                rest.push(pairs.last().1)
            } else {
                rest
            }
        }
    }

    /// Set of keys that appear in `pairs`.
    pub open spec fn spec_collect_domain<KV, VV>(pairs: Seq<(KV, VV)>) -> Set<KV>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Set::empty()
        } else {
            spec_collect_domain(pairs.drop_last()).insert(pairs.last().0)
        }
    }

    //		7. proof fns/broadcast groups

    // 7. proof fns

    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

    // 7. proof fns

    // When keys are unique, spec_entries_to_map length equals seq length.
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
                // idx is the last element — it's inserted last, so map[k] == v.
            } else {
                // idx is in the prefix.
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
                // last key differs from k (unique keys).
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

    // If entries[idx] has key k, the map contains k.
    pub proof fn lemma_entries_to_map_contains_key<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
        requires 0 <= idx < entries.len(),
        ensures spec_entries_to_map(entries).contains_key(entries[idx].0),
        decreases entries.len(),
    {
        if entries.len() > 0 {
            let last = entries.last();
            if idx == entries.len() - 1 {
                // Last element — inserted directly.
            } else {
                lemma_entries_to_map_contains_key::<KV, VV>(entries.drop_last(), idx);
            }
        }
    }

    // If no entry has key k, spec_entries_to_map does not contain k.
    pub proof fn lemma_entries_to_map_no_key<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires forall|i: int| 0 <= i < entries.len() ==> (#[trigger] entries[i]).0 != k,
        ensures !spec_entries_to_map(entries).contains_key(k),
    {
        if spec_entries_to_map(entries).contains_key(k) {
            lemma_entries_to_map_key_in_seq(entries, k);
        }
    }

    // If a key is in spec_entries_to_map, it appears in the seq.
    pub proof fn lemma_entries_to_map_key_in_seq<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires spec_entries_to_map(entries).contains_key(k),
        ensures exists|i: int| 0 <= i < entries.len() && (#[trigger] entries[i]).0 == k,
        decreases entries.len(),
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

    // When keys are unique, spec_entries_to_map length equals seq length.
    pub proof fn lemma_entries_to_map_len<KV, VV>(entries: Seq<(KV, VV)>)
        requires spec_keys_no_dups(entries),
        ensures spec_entries_to_map(entries).len() == entries.len(),
        decreases entries.len(),
    {
        if entries.len() > 0 {
            let prefix = entries.drop_last();
            let last = entries.last();
            let last_idx = entries.len() - 1;
            // Prefix also has unique keys.
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
            lemma_entries_to_map_len::<KV, VV>(prefix);
            // Last key is not in the prefix map.
            let prefix_map = spec_entries_to_map(prefix);
            assert(!prefix_map.contains_key(last.0)) by {
                if prefix_map.contains_key(last.0) {
                    lemma_entries_to_map_key_in_seq(prefix, last.0);
                    let idx = choose|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == last.0;
                    assert(0 <= idx && idx < entries.len());
                    assert(entries[idx].0 == last.0);
                    assert(entries[last_idx].0 == last.0);
                    assert(idx != last_idx);
                }
            };
            // Insert of new key increases map length by 1.
            assert(prefix_map.dom().finite()) by {
                lemma_entries_to_map_finite::<KV, VV>(prefix);
            };
            assert(spec_entries_to_map(entries) =~=
                prefix_map.insert(last.0, last.1));
            assert(prefix_map.insert(last.0, last.1).len() ==
                prefix_map.len() + 1);
        }
    }

    // Domain of spec_entries_to_map is preserved when keys are unchanged.
    proof fn lemma_entries_to_map_dom_same_keys<KV, VV1, VV2>(
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

    // Domain of collect grows by one key per element processed.
    proof fn lemma_spec_collect_domain_step<KV, VV>(pairs: Seq<(KV, VV)>, i: int)
        requires 0 <= i < pairs.len()
        ensures spec_collect_domain::<KV, VV>(pairs.subrange(0, i + 1))
            =~= spec_collect_domain::<KV, VV>(pairs.subrange(0, i)).insert(pairs[i].0)
    {
        assert(pairs.subrange(0, i + 1).drop_last() =~= pairs.subrange(0, i));
    }

    // Collected values grow by push when key matches, unchanged otherwise.
    proof fn lemma_spec_collect_key_step<KV, VV>(pairs: Seq<(KV, VV)>, k: KV, i: int)
        requires 0 <= i < pairs.len()
        ensures spec_collect_key::<KV, VV>(pairs.subrange(0, i + 1), k)
            =~= if pairs[i].0 == k {
                spec_collect_key::<KV, VV>(pairs.subrange(0, i), k).push(pairs[i].1)
            } else {
                spec_collect_key::<KV, VV>(pairs.subrange(0, i), k)
            }
    {
        assert(pairs.subrange(0, i + 1).drop_last() =~= pairs.subrange(0, i));
    }

    // Key absent from domain implies empty collected sequence.
    proof fn lemma_spec_collect_key_not_in_domain<KV, VV>(pairs: Seq<(KV, VV)>, k: KV)
        ensures !spec_collect_domain::<KV, VV>(pairs).contains(k)
            ==> spec_collect_key::<KV, VV>(pairs, k) =~= Seq::empty()
        decreases pairs.len()
    {
        if pairs.len() > 0 && !spec_collect_domain::<KV, VV>(pairs).contains(k) {
            lemma_spec_collect_key_not_in_domain::<KV, VV>(pairs.drop_last(), k);
        }
    }

    // Collected sequence length is bounded by input length.
    proof fn lemma_spec_collect_key_len_bound<KV, VV>(pairs: Seq<(KV, VV)>, k: KV)
        ensures spec_collect_key::<KV, VV>(pairs, k).len() <= pairs.len()
        decreases pairs.len()
    {
        if pairs.len() > 0 {
            lemma_spec_collect_key_len_bound::<KV, VV>(pairs.drop_last(), k);
        }
    }

    //		8. traits

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42.
    pub trait TableStPerTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_tablestper_wf(&self) -> bool;

        /// Returns the concrete stored value for a given key.
        /// Useful for transferring exec-level properties (e.g., wf) through find_ref.
        spec fn spec_stored_value(&self, key: K::V) -> V;

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            requires self.spec_tablestper_wf(),
            ensures count == self@.len();

        /// - APAS Cost Spec 42.5: Work 1, Span 1
        /// - Alg Analysis: APAS (Ch42 ref): Work O(1), Span O(1) -- agrees with APAS.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- agrees with APAS. — matches APAS
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_tablestper_wf();

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(key: K, value: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures tree@ == Map::<K::V, V::V>::empty().insert(key@, value@), tree.spec_tablestper_wf();

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential key extraction
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), domain.spec_arraysetsteph_wf();

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|s| * W(f)), Span O(lg |s| + S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|·W(f)), Span O(|s|·W(f)) — DIFFERS: sequential loop
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@);

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·W(f)), Span O(n·W(f)) — DIFFERS: sequential loop
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (mapped: Self)
            requires
                self.spec_tablestper_wf(),
                forall|v: &V| f.requires((v,)),
                obeys_feq_full::<K>(),
            ensures
                mapped@.dom() == self@.dom(),
                mapped.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] mapped@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && mapped@[k] == result@);

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(k,v))), Span O(n + Σ W(f(k,v))) — DIFFERS: sequential loop
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_tablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                filtered.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k);

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (common: Self)
            requires
                self.spec_tablestper_wf(),
                other.spec_tablestper_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<K>(),
            ensures
                common@.dom() =~= self@.dom().intersect(other@.dom()),
                common.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] common@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && common@[k] == r@);

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (combined: Self)
            requires
                self.spec_tablestper_wf(),
                other.spec_tablestper_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                combined@.dom() =~= self@.dom().union(other@.dom()),
                combined.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] self@.contains_key(k) && !other@.contains_key(k)
                    ==> combined@[k] == self@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !self@.contains_key(k)
                    ==> combined@[k] == other@[k],
                forall|k: K::V| #[trigger] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && combined@[k] == r@);

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_tablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures
                remaining@.dom() =~= self@.dom().difference(other@.dom()),
                remaining.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] remaining@.contains_key(k) ==> remaining@[k] == self@[k];

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan on unsorted array
        fn find(&self, key: &K) -> (found: Option<V>)
            requires self.spec_tablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };

        /// Like find, but returns a reference to the stored value.
        /// The ensures `*v == self.spec_stored_value(key@)` lets callers transfer
        /// exec-level properties (e.g., wf) from the stored value to the result.
        fn find_ref(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_tablestper_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@
                        && *v == self.spec_stored_value(key@),
                    None => !self@.contains_key(key@),
                };

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan + copy
        fn delete(&self, key: &K) -> (updated: Self)
            requires
                self.spec_tablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures updated@ =~= self@.remove(key@), updated.spec_tablestper_wf();

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: linear scan + copy
        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (updated: Self)
            requires
                self.spec_tablestper_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures
                updated@.contains_key(key@),
                updated.spec_tablestper_wf(),
                updated@.dom() =~= self@.dom().insert(key@),
                forall|k: K::V| k != key@ && #[trigger] self@.contains_key(k) ==> updated@[k] == self@[k],
                !self@.contains_key(key@) ==> updated@[key@] == value@
                    && updated.spec_stored_value(key@) == value,
                self@.contains_key(key@) ==> (exists|old_v: V, r: V|
                    old_v@ == self@[key@] && combine.ensures((&old_v, &value), r)
                    && updated@[key@] == r@
                    && old_v == self.spec_stored_value(key@)
                    && updated.spec_stored_value(key@) == r);

        /// Like insert, but additionally ensures all stored values preserve well-formedness.
        fn insert_wf<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (updated: Self)
            where K: ClonePreservesView, V: ClonePreservesWf
            requires
                self.spec_tablestper_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                value.spec_wf(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    self.spec_stored_value(k).spec_wf(),
                forall|v1: &V, v2: &V, r: V|
                    #[trigger] combine.ensures((v1, v2), r) && v1.spec_wf() && v2.spec_wf()
                    ==> r.spec_wf(),
            ensures
                updated@.contains_key(key@),
                updated.spec_tablestper_wf(),
                updated@.dom() =~= self@.dom().insert(key@),
                forall|k: K::V| k != key@ && #[trigger] self@.contains_key(k) ==> updated@[k] == self@[k],
                !self@.contains_key(key@) ==> updated@[key@] == value@
                    && updated.spec_stored_value(key@) == value,
                self@.contains_key(key@) ==> (exists|old_v: V, r: V|
                    old_v@ == self@[key@] && combine.ensures((&old_v, &value), r)
                    && updated@[key@] == r@
                    && old_v == self.spec_stored_value(key@)
                    && updated.spec_stored_value(key@) == r),
                forall|k: K::V| #[trigger] updated@.contains_key(k) ==>
                    updated.spec_stored_value(k).spec_wf();

        /// Like delete, but additionally ensures all remaining stored values preserve well-formedness.
        /// Requires K: ClonePreservesView, V: ClonePreservesWf.
        fn delete_wf(&self, key: &K) -> (updated: Self)
            where K: ClonePreservesView, V: ClonePreservesWf
            requires
                self.spec_tablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    self.spec_stored_value(k).spec_wf(),
            ensures
                updated@ =~= self@.remove(key@),
                updated.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] updated@.contains_key(k) ==>
                    updated.spec_stored_value(k).spec_wf();

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            requires self.spec_tablestper_wf(), obeys_feq_full::<Pair<K, V>>(), keys@.finite(),
            ensures
                restricted@.dom() =~= self@.dom().intersect(keys@),
                restricted.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] restricted@.contains_key(k) ==> restricted@[k] == self@[k];

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — DIFFERS: nested linear scans on array
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
            requires self.spec_tablestper_wf(), obeys_feq_full::<Pair<K, V>>(), keys@.finite(),
            ensures
                subtracted@.dom() =~= self@.dom().difference(keys@),
                subtracted.spec_tablestper_wf(),
                forall|k: K::V| #[trigger] subtracted@.contains_key(k) ==> subtracted@[k] == self@[k];

        /// - Alg Analysis: APAS (Ch42 ref): Work O(|a|), Span O(lg |a|) -- from Algorithm 42.3
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- returns backing entries directly. — matches APAS
        fn collect(&self) -> (collected: ArraySeqStPerS<Pair<K, V>>)
            ensures spec_entries_to_map(collected@) == self@;
    }

    //		9. impls

    impl<K: StT + Ord, V: StT> TableStPer<K, V> {
        pub open spec fn spec_tablestper_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }

        pub open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            self.entries.seq@[i].1
        }

        /// The view of spec_stored_value(k) equals the map value self@[k].
        pub proof fn lemma_spec_stored_value_view(&self, k: K::V)
            requires self.spec_tablestper_wf(), self@.contains_key(k)
            ensures self.spec_stored_value(k)@ == self@[k]
        {
            lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
            let view_idx = choose|i: int| 0 <= i < self.entries@.len()
                && (#[trigger] self.entries@[i]).0 == k;
            self.entries.lemma_view_index(view_idx);
            let sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == k;
            assert(self.entries@[sv_idx].0 == k) by {
                self.entries.lemma_view_index(sv_idx);
            };
            lemma_entries_to_map_get::<K::V, V::V>(self.entries@, sv_idx);
            self.entries.lemma_view_index(sv_idx);
        }
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> TableStPerTrait<K, V> for TableStPer<K, V> {
        open spec fn spec_tablestper_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }

        open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            self.entries.seq@[i].1
        }

        fn size(&self) -> (count: usize)
        {
            proof {
                lemma_entries_to_map_len::<K::V, V::V>(self.entries@);
            }
            self.entries.length()
        }

        fn empty() -> (empty: Self)
        {
            let entries = ArraySeqStPerS::empty();
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            TableStPer { entries }
        }

        fn singleton(key: K, value: V) -> (tree: Self)
        {
            let entries = ArraySeqStPerS::singleton(Pair(key, value));
            assert(entries@ =~= seq![(key@, value@)]);
            proof {
                let s = entries@;
                assert(s.len() == 1);
                assert(s.drop_last() =~= Seq::<(K::V, V::V)>::empty());
                assert(spec_entries_to_map(s.drop_last()) =~= Map::<K::V, V::V>::empty());
                assert(s.last() == (key@, value@));
            }
            TableStPer { entries }
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
                    // Forward: new key is in the set, old keys remain.
                    assert forall|j: int| 0 <= j < i as int + 1
                        implies keys@.contains((#[trigger] self.entries@[j]).0)
                    by {
                        if j < i as int {
                            assert(old_keys.contains(self.entries@[j].0));
                        }
                    };
                    // Backward: every key in the set came from an entry.
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

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            let key_seq = keys.to_seq();
            let mut entry_vec: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < key_seq.length()
                invariant
                    i <= key_seq.spec_len(),
                    entry_vec@.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] entry_vec@[j]).0@ == key_seq@[j],
                    forall|j: int| #![trigger key_seq.seq@[j]] 0 <= j < i as int ==>
                        f.ensures((&key_seq.seq@[j],), entry_vec@[j].1),
                    forall|k: &K| f.requires((k,)),
                    obeys_feq_full::<K>(),
                decreases key_seq.spec_len() - i,
            {
                let key = key_seq.nth(i);
                let value = f(key);
                let key_clone = key.clone_plus();
                proof { lemma_cloned_view_eq::<K>(*key, key_clone); }
                entry_vec.push(Pair(key_clone, value));
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(entry_vec);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(entries@);
                assert forall|j: int| 0 <= j < entries@.len()
                    implies (#[trigger] entries@[j]).0 == key_seq@[j]
                by {
                    assert(entries.spec_index(j) == entry_vec@[j]);
                    assert(entry_vec@[j].0@ == key_seq@[j]);
                };
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|i: int, j: int|
                        0 <= i < j < entries@.len()
                        implies (#[trigger] entries@[i]).0 != (#[trigger] entries@[j]).0
                    by {
                        assert(entries@[i].0 == key_seq@[i]);
                        assert(entries@[j].0 == key_seq@[j]);
                    };
                };
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).dom().contains(k) == keys@.contains(k)
                by {
                    if spec_entries_to_map(entries@).dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                        let j = choose|j: int| 0 <= j < entries@.len()
                            && (#[trigger] entries@[j]).0 == k;
                        assert(key_seq@[j] == k);
                    }
                    if keys@.contains(k) {
                        let j = choose|j: int| 0 <= j < key_seq@.len()
                            && key_seq@[j] == k;
                        assert(entries@[j].0 == k);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                    }
                };
                // Closure ensures postcondition.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && spec_entries_to_map(entries@)[k] == result@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let j = choose|j: int| 0 <= j < entries@.len()
                        && (#[trigger] entries@[j]).0 == k;
                    assert(entries.spec_index(j) == entry_vec@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, j);
                    assert(f.ensures((&key_seq.seq@[j],), entry_vec@[j].1));
                    assert(key_seq.seq@[j]@ == key_seq@[j]);
                    assert(entries@[j].1 == entry_vec@[j].1@);
                };
            }
            TableStPer { entries }
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> (mapped: Self)
        {
            let ghost old_view = self.entries@;
            let mut new_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    new_entries@.len() == i as int,
                    self.entries@ == old_view,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] new_entries@[j]).0@ == old_view[j].0,
                    forall|j: int| #![trigger new_entries@[j]] 0 <= j < i as int ==>
                        f.ensures((&self.entries.spec_index(j).1,), new_entries@[j].1),
                    forall|v: &V| f.requires((v,)),
                    obeys_feq_full::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                let new_value = f(&pair.1);
                let key_clone = pair.0.clone_plus();
                new_entries.push(Pair(key_clone, new_value));
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(new_entries);
            proof {
                assert forall|j: int| 0 <= j < entries@.len()
                    implies (#[trigger] entries@[j]).0 == old_view[j].0
                by {
                    assert(entries.spec_index(j) == new_entries@[j]);
                    assert(new_entries@[j].0@ == old_view[j].0);
                };
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(
                    entries@, old_view,
                );
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|i: int, j: int|
                        0 <= i < j < entries@.len()
                        implies (#[trigger] entries@[i]).0 != (#[trigger] entries@[j]).0
                    by {
                        assert(entries@[i].0 == old_view[i].0);
                        assert(entries@[j].0 == old_view[j].0);
                        assert(old_view[i].0 != old_view[j].0);
                    };
                };
                // Closure ensures postcondition.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies exists|old_val: V, result: V|
                        old_val@ == self@[k] && f.ensures((&old_val,), result)
                        && spec_entries_to_map(entries@)[k] == result@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let j = choose|j: int| 0 <= j < entries@.len()
                        && (#[trigger] entries@[j]).0 == k;
                    assert(entries.spec_index(j) == new_entries@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, j);
                    assert(f.ensures((&self.entries.spec_index(j).1,), new_entries@[j].1));
                    assert(entries@[j].1 == new_entries@[j].1@);
                };
            }
            TableStPer { entries }
        }

        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    forall|k: K, v: V, keep: bool|
                        f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@
                        && self_view[sources[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    forall|si: int| 0 <= si < i as int
                        && spec_pred((#[trigger] self_view[si]).0, self_view[si].1)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    spec_keys_no_dups(self_view),
                    obeys_feq_full::<Pair<K, V>>(),
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
                            && spec_pred((#[trigger] self_view[si]).0, self_view[si].1)
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
                        assert(!spec_pred(self_view[i as int].0, self_view[i as int].1));
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                // Prove subset directly: any key in filtered map is also in self map.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).dom().contains(k)
                    implies spec_entries_to_map(self_view).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let i = choose|i: int| 0 <= i < entries@.len()
                        && (#[trigger] entries@[i]).0 == k;
                    assert(entries.spec_index(i) == kept@[i]);
                    let s = sources[i];
                    assert(self_view[s].0 == kept@[i].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
                // Prove no duplicate keys: sources are strictly increasing
                // and map to original keys which have no dups.
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < entries@.len()
                        implies (#[trigger] entries@[j1]).0 != (#[trigger] entries@[j2]).0
                    by {
                        assert(entries.spec_index(j1) == kept@[j1]);
                        assert(entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(self_view[sources[j1]].0 != self_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies spec_entries_to_map(entries@)[k]
                        == spec_entries_to_map(self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(kept@[idx].1@ == self_view[s].1);
                };
                // Completeness: every key satisfying spec_pred was kept.
                assert forall|k: K::V|
                    spec_entries_to_map(self_view).dom().contains(k)
                    && spec_pred(k, spec_entries_to_map(self_view)[k])
                    implies #[trigger] spec_entries_to_map(entries@).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, si);
                    assert(spec_pred(self_view[si].0, self_view[si].1));
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                    assert(entries.spec_index(j) == kept@[j]);
                    assert(kept@[j].0@ == self_view[si].0);
                    assert(entries@[j].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                };
            }
            TableStPer { entries }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (common: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut self_srcs: Seq<int> = Seq::empty();
            let ghost mut other_srcs: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    other.entries@ == other_view,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    self_srcs.len() == kept@.len(),
                    other_srcs.len() == kept@.len(),
                    forall|j: int| 0 <= j < self_srcs.len() ==>
                        0 <= #[trigger] self_srcs[j] < self_view.len()
                        && self_view[self_srcs[j]].0 == kept@[j].0@,
                    forall|j: int| 0 <= j < other_srcs.len() ==>
                        0 <= #[trigger] other_srcs[j] < other_view.len()
                        && other_view[other_srcs[j]].0 == kept@[j].0@,
                    forall|si: int| #![trigger self_view[si]] 0 <= si < i as int
                        && (exists|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == self_view[si].0)
                        ==> exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si,
                    forall|si: int| #![trigger self_view[si]] 0 <= si < i as int
                        && !(exists|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == self_view[si].0)
                        ==> !spec_entries_to_map(other_view).contains_key(self_view[si].0),
                    forall|j: int| 0 <= j < self_srcs.len() ==> #[trigger] self_srcs[j] < i as int,
                    forall|j1: int, j2: int| #![trigger self_srcs[j1], self_srcs[j2]]
                        0 <= j1 < j2 < self_srcs.len() ==> self_srcs[j1] < self_srcs[j2],
                    forall|k: int| #![trigger kept@[k]] 0 <= k < kept@.len() ==>
                        combine.ensures(
                            (&self.entries.spec_index(self_srcs[k]).1,
                             &other.entries.spec_index(other_srcs[k]).1),
                            kept@[k].1),
                    spec_keys_no_dups(self_view),
                    spec_keys_no_dups(other_view),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<K>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                let mut found = false;
                let mut found_idx: usize = 0;
                let mut j: usize = 0;
                while j < other.entries.length() && !found
                    invariant
                        j <= other.entries.spec_len(),
                        other.entries@ == other_view,
                        found ==> found_idx < other.entries.spec_len()
                            && other_view[found_idx as int].0 == pair.0@,
                        !found ==> forall|jj: int| 0 <= jj < j as int ==>
                            (#[trigger] other_view[jj]).0 != pair.0@,
                        obeys_view_eq::<K>(),
                    decreases other.entries.spec_len() - j,
                {
                    let other_pair = other.entries.nth(j);
                    proof { reveal(obeys_view_eq); }
                    if pair.0.eq(&other_pair.0) {
                        found = true;
                        found_idx = j;
                    }
                    j += 1;
                }
                if found {
                    let other_val = other.entries.nth(found_idx);
                    let combined = combine(&pair.1, &other_val.1);
                    let key_clone = pair.0.clone_plus();
                    kept.push(Pair(key_clone, combined));
                    proof {
                        let ghost old_self_srcs = self_srcs;
                        self_srcs = self_srcs.push(i as int);
                        other_srcs = other_srcs.push(found_idx as int);
                        // Re-establish coverage for si in 0..i+1.
                        assert forall|si: int| #![trigger self_view[si]] 0 <= si < i as int + 1
                            && (exists|oj: int| 0 <= oj < other_view.len()
                                && (#[trigger] other_view[oj]).0 == self_view[si].0)
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
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, pair.0@);
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                // Forward: result keys are in self ∩ other.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).dom().contains(k)
                    implies self@.dom().contains(k)
                        && other@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = self_srcs[idx];
                    assert(self_view[s].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                    let os = other_srcs[idx];
                    assert(other_view[os].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(other_view, os);
                };
                // Backward: keys in self ∩ other are in result.
                assert forall|k: K::V| #[trigger]
                    self@.dom().contains(k) && other@.dom().contains(k)
                    implies spec_entries_to_map(entries@).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other_view, k);
                    let oj = choose|oj: int| 0 <= oj < other_view.len()
                        && (#[trigger] other_view[oj]).0 == k;
                    // Coverage: si was processed and other had a match.
                    let j = choose|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si;
                    assert(entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                };
                // Prove no duplicate keys via strictly increasing self_srcs.
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < entries@.len()
                        implies (#[trigger] entries@[j1]).0 != (#[trigger] entries@[j2]).0
                    by {
                        assert(entries.spec_index(j1) == kept@[j1]);
                        assert(entries.spec_index(j2) == kept@[j2]);
                        assert(self_srcs[j1] < self_srcs[j2]);
                        assert(self_view[self_srcs[j1]].0 != self_view[self_srcs[j2]].0);
                    };
                };
                // Value: combine.ensures postcondition.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r) && spec_entries_to_map(entries@)[k] == r@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    let s = self_srcs[idx];
                    let os = other_srcs[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, os);
                    assert(combine.ensures(
                        (&self.entries.spec_index(s).1, &other.entries.spec_index(os).1),
                        kept@[idx].1));
                    assert(self.entries.spec_index(s).1@ == self_view[s].1);
                    assert(other.entries.spec_index(os).1@ == other_view[os].1);
                    assert(kept@[idx].1@ == entries@[idx].1);
                };
            }
            TableStPer { entries }
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (combined: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            // Clone ensures combined.entries@ == self.entries@, so spec_tablestper_wf transfers.
            let mut combined = self.clone();
            let mut j: usize = 0;
            while j < other.entries.length()
                invariant
                    j <= other.entries.spec_len(),
                    other.entries@ == other_view,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    combined.spec_tablestper_wf(),
                    forall|k: K::V| spec_entries_to_map(self_view).contains_key(k)
                        ==> combined@.contains_key(k),
                    forall|oj: int| 0 <= oj < j as int
                        ==> combined@.contains_key(
                            (#[trigger] other.entries@[oj]).0),
                    // Forward: combined keys come from self or processed other entries.
                    forall|k: K::V| combined@.contains_key(k) ==>
                        spec_entries_to_map(self_view).contains_key(k)
                        || exists|oj: int| 0 <= oj < j as int
                            && (#[trigger] other_view[oj]).0 == k,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    spec_keys_no_dups(other_view),
                    // Self-only values preserved.
                    forall|k: K::V| #[trigger]
                        spec_entries_to_map(self_view).contains_key(k)
                        && !spec_entries_to_map(other_view).contains_key(k)
                        ==> combined@[k] == spec_entries_to_map(self_view)[k],
                    // Other-only values preserved (processed entries).
                    forall|oj: int| 0 <= oj < j as int
                        && !spec_entries_to_map(self_view).contains_key((#[trigger] other_view[oj]).0)
                        ==> combined@[other_view[oj].0] == other_view[oj].1,
                    // Self values preserved for both-keys not yet processed from other.
                    forall|k: K::V| #[trigger]
                        spec_entries_to_map(self_view).contains_key(k)
                        && spec_entries_to_map(other_view).contains_key(k)
                        && (forall|oj: int| 0 <= oj < j as int ==> (#[trigger] other_view[oj]).0 != k)
                        ==> combined@[k] == spec_entries_to_map(self_view)[k],
                    // Combine.ensures for both-keys already processed from other.
                    forall|oj: int| 0 <= oj < j as int
                        && spec_entries_to_map(self_view).contains_key((#[trigger] other_view[oj]).0)
                        ==> (exists|v1: V, v2: V, r: V|
                            v1@ == spec_entries_to_map(self_view)[other_view[oj].0]
                            && v2@ == other_view[oj].1
                            && combine.ensures((&v1, &v2), r)
                            && combined@[other_view[oj].0] == r@),
                decreases other.entries.spec_len() - j,
            {
                let pair = other.entries.nth(j);
                let cloned = pair.clone_plus();
                let ghost old_combined = combined@;
                combined = combined.insert(cloned.0, cloned.1, &combine);
                proof {
                    // Maintain forward invariant: new combined keys come from self or other[0..j+1].
                    assert forall|k: K::V| combined@.contains_key(k) implies
                        spec_entries_to_map(self_view).contains_key(k)
                        || exists|oj: int| 0 <= oj < j as int + 1
                            && (#[trigger] other_view[oj]).0 == k
                    by {
                        if k == cloned.0@ {
                            assert(other_view[j as int].0 == k);
                        } else if old_combined.contains_key(k) {
                            if spec_entries_to_map(self_view).contains_key(k) {
                            } else {
                                let oj = choose|oj: int| 0 <= oj < j as int
                                    && (#[trigger] other_view[oj]).0 == k;
                                assert(oj < j as int + 1);
                            }
                        }
                    };
                    // Self-only values: insert preserves since key is from other.
                    assert forall|k: K::V| #[trigger]
                        spec_entries_to_map(self_view).contains_key(k)
                        && !spec_entries_to_map(other_view).contains_key(k)
                        implies combined@[k] == spec_entries_to_map(self_view)[k]
                    by {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            other_view, j as int);
                        // cloned.0@ is in other@, but k is not, so k != cloned.0@.
                        assert(old_combined.contains_key(k));
                    };
                    // Other-only values: previous entries preserved, new entry added.
                    assert forall|oj: int| 0 <= oj < j as int + 1
                        && !spec_entries_to_map(self_view).contains_key((#[trigger] other_view[oj]).0)
                        implies combined@[other_view[oj].0] == other_view[oj].1
                    by {
                        let ok = other_view[oj].0;
                        if oj < j as int {
                            // Previous entry: key differs from cloned.0@ by no-dups.
                            assert(ok != other_view[j as int].0);
                            assert(old_combined.contains_key(ok));
                        } else {
                            // oj == j: fresh key, insert sets value.
                            assert(!old_combined.contains_key(ok)) by {
                                if old_combined.contains_key(ok) {
                                    if spec_entries_to_map(self_view).contains_key(ok) {
                                    } else {
                                        let prev = choose|prev: int| 0 <= prev < j as int
                                            && (#[trigger] other_view[prev]).0 == ok;
                                        assert(other_view[prev].0 == other_view[j as int].0);
                                        assert(prev != j as int);
                                    }
                                }
                            };
                        }
                    };
                    // Unprocessed both-keys: self values preserved.
                    assert forall|k: K::V| #[trigger]
                        spec_entries_to_map(self_view).contains_key(k)
                        && spec_entries_to_map(other_view).contains_key(k)
                        && (forall|oj: int| 0 <= oj < j as int + 1 ==> (#[trigger] other_view[oj]).0 != k)
                        implies combined@[k] == spec_entries_to_map(self_view)[k]
                    by {
                        // Instantiate forall at oj = j: other_view[j].0 != k.
                        assert(other_view[j as int].0 != k);
                        // clone_plus + nth: cloned.0@ == other_view[j].0.
                        assert(cloned.0@ == other_view[j as int].0);
                        // So insert preserved combined@[k] == old_combined[k] == self@[k].
                    };
                    // Combine.ensures for processed both-keys.
                    assert forall|oj: int| 0 <= oj < j as int + 1
                        && spec_entries_to_map(self_view).contains_key((#[trigger] other_view[oj]).0)
                        implies exists|v1: V, v2: V, r: V|
                            v1@ == spec_entries_to_map(self_view)[other_view[oj].0]
                            && v2@ == other_view[oj].1
                            && combine.ensures((&v1, &v2), r)
                            && combined@[other_view[oj].0] == r@
                    by {
                        if oj < j as int {
                            // Prior entry: key differs by no-dups, value preserved.
                            assert(other_view[oj].0 != other_view[j as int].0);
                            assert(other_view[oj].0 != cloned.0@);
                            assert(combined@[other_view[oj].0] == old_combined[other_view[oj].0]);
                        } else {
                            // oj == j: insert applied combine on key in self.
                            assert(old_combined.contains_key(cloned.0@));
                            assert(old_combined[cloned.0@]
                                == spec_entries_to_map(self_view)[cloned.0@]);
                        }
                    };
                }
                j += 1;
            }
            proof {
                // Backward: self∪other keys are in combined.
                assert forall|k: K::V|
                    spec_entries_to_map(self_view).dom().contains(k)
                    || spec_entries_to_map(other_view).dom().contains(k)
                    implies combined@.dom().contains(k)
                by {
                    if spec_entries_to_map(self_view).dom().contains(k) {
                    } else {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(other_view, k);
                        let oj = choose|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == k;
                    }
                };
                // Forward: combined keys are in self∪other.
                assert forall|k: K::V|
                    combined@.dom().contains(k)
                    implies self@.dom().contains(k) || other@.dom().contains(k)
                by {
                    if spec_entries_to_map(self_view).contains_key(k) {
                    } else {
                        let oj = choose|oj: int| 0 <= oj < other_view.len()
                            && (#[trigger] other_view[oj]).0 == k;
                        lemma_entries_to_map_contains_key::<K::V, V::V>(other_view, oj);
                    }
                };
                // Self-only value preservation (from loop invariant).
                // Other-only value preservation: connect per-entry to per-key.
                assert forall|k: K::V| #[trigger]
                    other@.contains_key(k) && !self@.contains_key(k)
                    implies combined@[k] == other@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other_view, k);
                    let oj = choose|oj: int| 0 <= oj < other_view.len()
                        && (#[trigger] other_view[oj]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, oj);
                };
                // Combine value: both self and other contain k.
                assert forall|k: K::V| #[trigger]
                    self@.contains_key(k) && other@.contains_key(k)
                    implies exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && combined@[k] == r@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other_view, k);
                    let oj = choose|oj: int| 0 <= oj < other_view.len()
                        && (#[trigger] other_view[oj]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, oj);
                };
            }
            combined
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    other.entries@ == other_view,
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@
                        && self_view[sources[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < kept@.len() ==>
                        !spec_entries_to_map(other_view).contains_key((#[trigger] kept@[j]).0@),
                    // Coverage: every self entry not in other has been added.
                    forall|si: int| 0 <= si < i as int
                        && !spec_entries_to_map(other_view).contains_key(
                            (#[trigger] self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(self_view),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                // Linear scan of other for this key.
                let mut found = false;
                let ghost mut found_pos: int = -1int;
                let mut j: usize = 0;
                while j < other.entries.length() && !found
                    invariant
                        j <= other.entries.spec_len(),
                        other.entries@ == other_view,
                        !found ==> forall|jj: int| 0 <= jj < j as int ==>
                            (#[trigger] other_view[jj]).0 != pair.0@,
                        found ==> 0 <= found_pos < other_view.len()
                            && other_view[found_pos].0 == pair.0@,
                        obeys_view_eq::<K>(),
                    decreases other.entries.spec_len() - j,
                {
                    let other_pair = other.entries.nth(j);
                    proof { reveal(obeys_view_eq); }
                    if pair.0.eq(&other_pair.0) {
                        found = true;
                        proof { found_pos = j as int; }
                    }
                    j += 1;
                }
                if !found {
                    proof {
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, pair.0@);
                    }
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        sources = sources.push(i as int);
                        assert forall|si: int| 0 <= si < i as int + 1
                            && !spec_entries_to_map(other_view).contains_key(
                                (#[trigger] self_view[si]).0)
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
                        lemma_entries_to_map_contains_key::<K::V, V::V>(other_view, found_pos);
                        assert(spec_entries_to_map(other_view).contains_key(self_view[i as int].0));
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                // Forward: result keys are in self \ other.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).dom().contains(k)
                    implies self@.dom().contains(k)
                        && !other@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    assert(self_view[s].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
                // Backward: keys in self \ other are in result.
                assert forall|k: K::V| #[trigger]
                    self@.dom().contains(k) && !other@.dom().contains(k)
                    implies spec_entries_to_map(entries@).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                    assert(entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                };
                // Prove no duplicate keys via strictly increasing sources.
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < entries@.len()
                        implies (#[trigger] entries@[j1]).0 != (#[trigger] entries@[j2]).0
                    by {
                        assert(entries.spec_index(j1) == kept@[j1]);
                        assert(entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(self_view[sources[j1]].0 != self_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies spec_entries_to_map(entries@)[k]
                        == spec_entries_to_map(self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(kept@[idx].1@ == self_view[s].1);
                };
            }
            TableStPer { entries }
        }

        fn find(&self, key: &K) -> (found: Option<V>)
        {
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.spec_tablestper_wf(),
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] self.entries@[j]).0 != key@,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
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
                    self.spec_tablestper_wf(),
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
                                assert(self.entries@[j].0 != key@);
                            } else if j > i as int {
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

        fn delete(&self, key: &K) -> (updated: Self)
        {
            let ghost self_view = self.entries@;
            let mut updated: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.spec_tablestper_wf(),
                    forall|j: int| 0 <= j < updated@.len() ==>
                        (#[trigger] updated@[j]).0@ != key@,
                    src.len() == updated@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && updated@[j].0@ == self_view[src[j]].0
                        && updated@[j].1@ == self_view[src[j]].1
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int && (#[trigger] self_view[si]).0 != key@ ==>
                        exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if !pair.0.eq(key) {
                    let cloned = pair.clone_plus();
                    updated.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int|
                            0 <= si < i as int + 1
                            && (#[trigger] self_view[si]).0 != key@
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
                } else {
                    proof {
                        assert(self_view[i as int].0 == key@);
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(updated);
            proof {
                assert forall|j: int| 0 <= j < entries@.len()
                    implies (#[trigger] entries@[j]).0 != key@
                by {
                    assert(entries.spec_index(j) == updated@[j]);
                    assert(updated@[j].0@ != key@);
                };
                lemma_entries_to_map_no_key::<K::V, V::V>(entries@, key@);
                // Prove spec_keys_no_dups(entries@).
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == updated@[a]);
                        assert(entries.spec_index(b) == updated@[b]);
                        assert(src[a] < src[b]);
                        assert(0 <= src[a] < self_view.len());
                        assert(0 <= src[b] < self_view.len());
                        assert(self_view[src[a]].0 != self_view[src[b]].0);
                    };
                };
                // Prove updated@ =~= self@.remove(key@).
                let ghost result_map = spec_entries_to_map(entries@);
                let ghost target_map = self@.remove(key@);
                // Domain: forward.
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == updated@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
                // Domain: backward.
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    assert(self@.contains_key(k) && k != key@);
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(entries.spec_index(j) == updated@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                };
                // Values.
                assert forall|k: K::V| #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == updated@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(updated@[idx].1@ == self_view[s].1);
                };
                assert(result_map =~= target_map);
            }
            TableStPer { entries }
        }

        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (updated: Self)
        {
            let ghost key_view: K::V = key@;
            let ghost self_view = self.entries@;
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut found_idx: Option<usize> = None;
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.spec_tablestper_wf(),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    src.len() == all@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && all@[j].0@ == self_view[src[j]].0
                        && all@[j].1@ == self_view[src[j]].1
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).0@ != key_view,
                    forall|si: int| 0 <= si < i as int && (#[trigger] self_view[si]).0 != key_view ==>
                        exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    key@ == key_view,
                    match found_idx {
                        Some(fi) => fi < i
                            && self_view[fi as int].0 == key_view,
                        None => forall|si: int|
                            0 <= si < i as int ==> (#[trigger] self_view[si]).0 != key_view,
                    },
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    found_idx = Some(i);
                } else {
                    let cloned = pair.clone_plus();
                    all.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int|
                            0 <= si < i as int + 1
                            && (#[trigger] self_view[si]).0 != key_view
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
            let ghost mut witness_old_v: V = value;
            let ghost mut witness_r: V = value;
            let final_value = match found_idx {
                Some(idx) => {
                    let old_entry = self.entries.nth(idx);
                    let combined = combine(&old_entry.1, &value);
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(self_view, idx as int);
                        witness_old_v = old_entry.1;
                        witness_r = combined;
                    }
                    combined
                },
                None => value,
            };
            all.push(Pair(key, final_value));
            let entries = ArraySeqStPerS::from_vec(all);
            proof {
                let last = (entries@.len() - 1) as int;
                assert(entries.spec_index(last) == all@[last]);
                lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, last);
                assert(entries@[last].0 == key_view);
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == all@[a]);
                        assert(entries.spec_index(b) == all@[b]);
                        if b < entries@.len() - 1 {
                            assert(src[a] < src[b]);
                            assert(0 <= src[a] < self_view.len());
                            assert(0 <= src[b] < self_view.len());
                            assert(self_view[src[a]].0 != self_view[src[b]].0);
                        } else {
                            assert(all@[a].0@ != key_view);
                        }
                    };
                };
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(self_view).contains_key(k)
                    implies spec_entries_to_map(entries@).contains_key(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    if self_view[si].0 == key_view {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, last);
                    } else {
                        let j = choose|j: int| 0 <= j < src.len()
                            && (#[trigger] src[j]) == si;
                        assert(entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                    }
                };
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies self@.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == all@[idx]);
                    if idx < entries@.len() - 1 {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                    }
                };
                assert forall|k: K::V|
                    k != key_view && #[trigger] self@.contains_key(k)
                    implies spec_entries_to_map(entries@)[k] == self@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, si);
                    assert(all@[j].1@ == self_view[si].1);
                };
                if !spec_entries_to_map(self_view).contains_key(key_view) {
                    assert(found_idx.is_none()) by {
                        if found_idx.is_some() {
                            let fi = choose|fi: usize| found_idx == Some(fi);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, fi as int);
                        }
                    };
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, last);
                }
                assert(spec_entries_to_map(self_view).contains_key(key_view)
                    ==> found_idx.is_some()) by {
                    if spec_entries_to_map(self_view).contains_key(key_view)
                        && found_idx.is_none() {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, key_view);
                        let si = choose|si: int| 0 <= si < self_view.len()
                            && (#[trigger] self_view[si]).0 == key_view;
                    }
                };
                if spec_entries_to_map(self_view).contains_key(key_view) {
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, last);
                    assert(found_idx.is_some());
                    assert(witness_old_v@ == spec_entries_to_map(self_view)[key_view]);
                    assert(spec_entries_to_map(entries@)[key_view] == witness_r@);
                    assert(combine.ensures((&witness_old_v, &value), witness_r));
                }
                // Prove spec_stored_value for the inserted key.
                assert(entries.spec_index(last) == all@[last]);
                assert((#[trigger] entries.seq@[last]).0@ == key_view);
                let ghost chosen_i = choose|i: int| 0 <= i < entries.seq@.len()
                    && (#[trigger] entries.seq@[i]).0@ == key_view;
                if chosen_i != last {
                    assert(entries@[chosen_i].0 == key_view);
                    assert(entries@[last].0 == key_view);
                }
                assert(chosen_i == last);
                // Existing-key stored_value proof.
                if spec_entries_to_map(self_view).contains_key(key_view) {
                    assert(found_idx.is_some());
                    let ghost fi = choose|fi: usize| found_idx == Some(fi);
                    assert(self_view[fi as int].0 == key_view);
                    let ghost old_chosen = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                    if old_chosen != fi as int {
                        assert(self.entries@[old_chosen].0 == key_view);
                        assert(self.entries@[fi as int].0 == key_view);
                    }
                    assert(old_chosen == fi as int);
                    assert(combine.ensures((&witness_old_v, &value), witness_r));
                    assert(exists|old_v: V, r: V|
                        old_v@ == spec_entries_to_map(self_view)[key_view]
                        && combine.ensures((&old_v, &value), r)
                        && spec_entries_to_map(entries@)[key_view] == r@
                        && old_v == self.entries.seq@[old_chosen].1
                        && entries.seq@[chosen_i].1 == r);
                }
            }
            TableStPer { entries }
        }

        fn insert_wf<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (updated: Self)
            where K: ClonePreservesView, V: ClonePreservesWf
        {
            let ghost key_view: K::V = key@;
            let ghost self_view = self.entries@;
            let ghost self_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut found_idx: Option<usize> = None;
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.entries.seq@ =~= self_exec_seq,
                    self.spec_tablestper_wf(),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    src.len() == all@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && all@[j].0@ == self_view[src[j]].0
                        && all@[j].1@ == self_view[src[j]].1
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).0@ != key_view,
                    forall|si: int| 0 <= si < i as int && (#[trigger] self_view[si]).0 != key_view ==>
                        exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    key@ == key_view,
                    match found_idx {
                        Some(fi) => fi < i
                            && self_view[fi as int].0 == key_view,
                        None => forall|si: int|
                            0 <= si < i as int ==> (#[trigger] self_view[si]).0 != key_view,
                    },
                    // Wf invariants.
                    forall|j: int| 0 <= j < all@.len() ==> (#[trigger] all@[j]).1.spec_wf(),
                    forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                        self.spec_stored_value(k).spec_wf(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    found_idx = Some(i);
                } else {
                    // Prove pair.1.spec_wf() so we can call clone_wf.
                    proof {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, i as int);
                        let ghost k_at_i = self_view[i as int].0;
                        let ghost chosen = choose|idx: int| 0 <= idx < self_exec_seq.len()
                            && (#[trigger] self_exec_seq[idx]).0@ == k_at_i;
                        assert(self_exec_seq[i as int].0@ == k_at_i);
                        if chosen != i as int {
                            assert(self_view[chosen].0 == k_at_i);
                            assert(self_view[i as int].0 == k_at_i);
                        }
                        assert(chosen == i as int);
                        // Trigger the stored-value-wf quantifier.
                        assert(self@.contains_key(k_at_i));
                        assert(self.spec_stored_value(k_at_i) == self_exec_seq[i as int].1);
                        // Now pair.1 == self_exec_seq[i].1, and spec_wf follows.
                        assert(*pair == self.entries.seq@[i as int]);
                        assert((*pair).1 == self_exec_seq[i as int].1);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    all.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int|
                            0 <= si < i as int + 1
                            && (#[trigger] self_view[si]).0 != key_view
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
            let ghost mut witness_old_v: V = value;
            let ghost mut witness_r: V = value;
            let final_value = match found_idx {
                Some(idx) => {
                    let old_entry = self.entries.nth(idx);
                    proof {
                        lemma_entries_to_map_get::<K::V, V::V>(self_view, idx as int);
                        // Prove old_entry.1.spec_wf() for combine-preserves-wf.
                        let ghost k_at_idx = self_view[idx as int].0;
                        let ghost chosen = choose|ci: int| 0 <= ci < self_exec_seq.len()
                            && (#[trigger] self_exec_seq[ci]).0@ == k_at_idx;
                        assert(self_exec_seq[idx as int].0@ == k_at_idx);
                        if chosen != idx as int {
                            assert(self_view[chosen].0 == k_at_idx);
                            assert(self_view[idx as int].0 == k_at_idx);
                        }
                        assert(chosen == idx as int);
                    }
                    let combined = combine(&old_entry.1, &value);
                    proof {
                        witness_old_v = old_entry.1;
                        witness_r = combined;
                    }
                    combined
                },
                None => value,
            };
            all.push(Pair(key, final_value));
            let entries = ArraySeqStPerS::from_vec(all);
            proof {
                let last = (entries@.len() - 1) as int;
                assert(entries.spec_index(last) == all@[last]);
                lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, last);
                assert(entries@[last].0 == key_view);
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == all@[a]);
                        assert(entries.spec_index(b) == all@[b]);
                        if b < entries@.len() - 1 {
                            assert(src[a] < src[b]);
                            assert(0 <= src[a] < self_view.len());
                            assert(0 <= src[b] < self_view.len());
                            assert(self_view[src[a]].0 != self_view[src[b]].0);
                        } else {
                            assert(all@[a].0@ != key_view);
                        }
                    };
                };
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(self_view).contains_key(k)
                    implies spec_entries_to_map(entries@).contains_key(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    if self_view[si].0 == key_view {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, last);
                    } else {
                        let j = choose|j: int| 0 <= j < src.len()
                            && (#[trigger] src[j]) == si;
                        assert(entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                    }
                };
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies self@.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == all@[idx]);
                    if idx < entries@.len() - 1 {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                    }
                };
                assert forall|k: K::V|
                    k != key_view && #[trigger] self@.contains_key(k)
                    implies spec_entries_to_map(entries@)[k] == self@[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, si);
                    assert(all@[j].1@ == self_view[si].1);
                };
                if !spec_entries_to_map(self_view).contains_key(key_view) {
                    assert(found_idx.is_none()) by {
                        if found_idx.is_some() {
                            let fi = choose|fi: usize| found_idx == Some(fi);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, fi as int);
                        }
                    };
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, last);
                }
                assert(spec_entries_to_map(self_view).contains_key(key_view)
                    ==> found_idx.is_some()) by {
                    if spec_entries_to_map(self_view).contains_key(key_view)
                        && found_idx.is_none() {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, key_view);
                        let si = choose|si: int| 0 <= si < self_view.len()
                            && (#[trigger] self_view[si]).0 == key_view;
                    }
                };
                if spec_entries_to_map(self_view).contains_key(key_view) {
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, last);
                    assert(found_idx.is_some());
                    assert(witness_old_v@ == spec_entries_to_map(self_view)[key_view]);
                    assert(spec_entries_to_map(entries@)[key_view] == witness_r@);
                    assert(combine.ensures((&witness_old_v, &value), witness_r));
                }
                // Prove spec_stored_value for the inserted key.
                assert(entries.spec_index(last) == all@[last]);
                assert((#[trigger] entries.seq@[last]).0@ == key_view);
                let ghost chosen_i = choose|i: int| 0 <= i < entries.seq@.len()
                    && (#[trigger] entries.seq@[i]).0@ == key_view;
                if chosen_i != last {
                    assert(entries@[chosen_i].0 == key_view);
                    assert(entries@[last].0 == key_view);
                }
                assert(chosen_i == last);
                // Existing-key stored_value proof.
                if spec_entries_to_map(self_view).contains_key(key_view) {
                    assert(found_idx.is_some());
                    let ghost fi = choose|fi: usize| found_idx == Some(fi);
                    assert(self_view[fi as int].0 == key_view);
                    let ghost old_chosen = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                    if old_chosen != fi as int {
                        assert(self.entries@[old_chosen].0 == key_view);
                        assert(self.entries@[fi as int].0 == key_view);
                    }
                    assert(old_chosen == fi as int);
                    assert(combine.ensures((&witness_old_v, &value), witness_r));
                    assert(exists|old_v: V, r: V|
                        old_v@ == spec_entries_to_map(self_view)[key_view]
                        && combine.ensures((&old_v, &value), r)
                        && spec_entries_to_map(entries@)[key_view] == r@
                        && old_v == self.entries.seq@[old_chosen].1
                        && entries.seq@[chosen_i].1 == r);
                }
                // Prove stored-value wf for all keys.
                assert forall|k: K::V| #[trigger] spec_entries_to_map(entries@).contains_key(k)
                    implies entries.seq@[
                        choose|i: int| 0 <= i < entries.seq@.len()
                            && (#[trigger] entries.seq@[i]).0@ == k
                    ].1.spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == all@[idx]);
                    let ghost sv_idx = choose|i: int| 0 <= i < entries.seq@.len()
                        && (#[trigger] entries.seq@[i]).0@ == k;
                    assert(entries@[sv_idx].0 == k);
                    assert(entries@[idx].0 == k);
                    if sv_idx != idx {}
                    assert(sv_idx == idx);
                    assert(entries.seq@[idx] == all@[idx]);
                    if idx == last {
                        if found_idx.is_some() {
                            assert(combine.ensures((&witness_old_v, &value), witness_r));
                            assert(witness_old_v.spec_wf());
                            assert(value.spec_wf());
                        } else {
                            assert(all@[last].1 == value);
                        }
                    } else {
                        assert(all@[idx].1.spec_wf());
                    }
                };
            }
            TableStPer { entries }
        }

        fn delete_wf(&self, key: &K) -> (updated: Self)
            where K: ClonePreservesView, V: ClonePreservesWf
        {
            let ghost self_view = self.entries@;
            let ghost self_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.entries.seq@ =~= self_exec_seq,
                    self.spec_tablestper_wf(),
                    forall|j: int| 0 <= j < kept@.len() ==>
                        (#[trigger] kept@[j]).0@ != key@,
                    src.len() == kept@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && kept@[j].0@ == self_view[src[j]].0
                        && kept@[j].1@ == self_view[src[j]].1
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int && (#[trigger] self_view[si]).0 != key@ ==>
                        exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    // Wf invariants.
                    forall|j: int| 0 <= j < kept@.len() ==> (#[trigger] kept@[j]).1.spec_wf(),
                    forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                        self.spec_stored_value(k).spec_wf(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if !pair.0.eq(key) {
                    // Prove pair.1.spec_wf() so we can call clone_wf.
                    proof {
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, i as int);
                        let ghost k_at_i = self_view[i as int].0;
                        let ghost chosen = choose|idx: int| 0 <= idx < self_exec_seq.len()
                            && (#[trigger] self_exec_seq[idx]).0@ == k_at_i;
                        assert(self_exec_seq[i as int].0@ == k_at_i);
                        if chosen != i as int {
                            assert(self_view[chosen].0 == k_at_i);
                            assert(self_view[i as int].0 == k_at_i);
                        }
                        assert(chosen == i as int);
                        assert(self@.contains_key(k_at_i));
                        assert(self.spec_stored_value(k_at_i) == self_exec_seq[i as int].1);
                        assert(*pair == self.entries.seq@[i as int]);
                        assert((*pair).1 == self_exec_seq[i as int].1);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    kept.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        assert forall|si: int|
                            0 <= si < i as int + 1
                            && (#[trigger] self_view[si]).0 != key@
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
                } else {
                    proof {
                        assert(self_view[i as int].0 == key@);
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                assert forall|j: int| 0 <= j < entries@.len()
                    implies (#[trigger] entries@[j]).0 != key@
                by {
                    assert(entries.spec_index(j) == kept@[j]);
                    assert(kept@[j].0@ != key@);
                };
                lemma_entries_to_map_no_key::<K::V, V::V>(entries@, key@);
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == kept@[a]);
                        assert(entries.spec_index(b) == kept@[b]);
                        assert(src[a] < src[b]);
                        assert(0 <= src[a] < self_view.len());
                        assert(0 <= src[b] < self_view.len());
                        assert(self_view[src[a]].0 != self_view[src[b]].0);
                    };
                };
                let ghost result_map = spec_entries_to_map(entries@);
                let ghost target_map = self@.remove(key@);
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    assert(self@.contains_key(k) && k != key@);
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                    let si = choose|si: int| 0 <= si < self_view.len()
                        && (#[trigger] self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    assert(entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                };
                assert forall|k: K::V| #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(kept@[idx].1@ == self_view[s].1);
                };
                assert(result_map =~= target_map);
                // Prove stored-value wf for all remaining keys.
                assert forall|k: K::V| #[trigger] spec_entries_to_map(entries@).contains_key(k)
                    implies entries.seq@[
                        choose|i: int| 0 <= i < entries.seq@.len()
                            && (#[trigger] entries.seq@[i]).0@ == k
                    ].1.spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let ghost sv_idx = choose|i: int| 0 <= i < entries.seq@.len()
                        && (#[trigger] entries.seq@[i]).0@ == k;
                    assert(entries@[sv_idx].0 == k);
                    assert(entries@[idx].0 == k);
                    if sv_idx != idx {}
                    assert(sv_idx == idx);
                    assert(entries.seq@[idx] == kept@[idx]);
                    assert(kept@[idx].1.spec_wf());
                };
            }
            TableStPer { entries }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
        {
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@
                        && self_view[sources[j]].1 == kept@[j].1@,
                    // Coverage: every self entry in keys has been kept.
                    forall|si: int| 0 <= si < i as int
                        && keys@.contains((#[trigger] self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    // Kept entries are in keys.
                    forall|j: int| 0 <= j < kept@.len() ==>
                        keys@.contains((#[trigger] kept@[j]).0@),
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(self_view),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
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
                            && keys@.contains((#[trigger] self_view[si]).0)
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
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(entries@).dom();
                let ghost target_dom = spec_entries_to_map(self_view).dom().intersect(keys@);
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        // Forward: result key must be in self and in keys.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                        let idx = choose|idx: int| 0 <= idx < entries@.len()
                            && (#[trigger] entries@[idx]).0 == k;
                        assert(entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        assert(self_view[s].0 == kept@[idx].0@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                    }
                    if spec_entries_to_map(self_view).dom().contains(k) && keys@.contains(k) {
                        // Backward: key in self ∩ keys must be in result.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                        let si = choose|si: int| 0 <= si < self_view.len()
                            && (#[trigger] self_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        assert(entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                    }
                };
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < entries@.len()
                        implies (#[trigger] entries@[j1]).0 != (#[trigger] entries@[j2]).0
                    by {
                        assert(entries.spec_index(j1) == kept@[j1]);
                        assert(entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(self_view[sources[j1]].0 != self_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies spec_entries_to_map(entries@)[k]
                        == spec_entries_to_map(self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(kept@[idx].1@ == self_view[s].1);
                };
            }
            TableStPer { entries }
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
        {
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    sources.len() == kept@.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@
                        && self_view[sources[j]].1 == kept@[j].1@,
                    // Kept entries are not in keys.
                    forall|j: int| 0 <= j < kept@.len() ==>
                        !keys@.contains((#[trigger] kept@[j]).0@),
                    // Coverage: every self entry not in keys has been kept.
                    forall|si: int| 0 <= si < i as int
                        && !keys@.contains((#[trigger] self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int| #![trigger sources[j1], sources[j2]]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    spec_keys_no_dups(self_view),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
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
                            && !keys@.contains((#[trigger] self_view[si]).0)
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
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(entries@).dom();
                let ghost target_dom = spec_entries_to_map(self_view).dom().difference(keys@);
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        // Forward: result key is in self and not in keys.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                        let idx = choose|idx: int| 0 <= idx < entries@.len()
                            && (#[trigger] entries@[idx]).0 == k;
                        assert(entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        assert(self_view[s].0 == kept@[idx].0@);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                    }
                    if spec_entries_to_map(self_view).dom().contains(k) && !keys@.contains(k) {
                        // Backward: key in self \ keys is in result.
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self_view, k);
                        let si = choose|si: int| 0 <= si < self_view.len()
                            && (#[trigger] self_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        assert(entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, j);
                    }
                };
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|j1: int, j2: int|
                        0 <= j1 < j2 < entries@.len()
                        implies (#[trigger] entries@[j1]).0 != (#[trigger] entries@[j2]).0
                    by {
                        assert(entries.spec_index(j1) == kept@[j1]);
                        assert(entries.spec_index(j2) == kept@[j2]);
                        assert(sources[j1] < sources[j2]);
                        assert(self_view[sources[j1]].0 != self_view[sources[j2]].0);
                    };
                };
                // Value preservation.
                assert forall|k: K::V| #[trigger]
                    spec_entries_to_map(entries@).contains_key(k)
                    implies spec_entries_to_map(entries@)[k]
                        == spec_entries_to_map(self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(self_view, s);
                    assert(kept@[idx].1@ == self_view[s].1);
                };
            }
            TableStPer { entries }
        }

        fn collect(&self) -> (collected: ArraySeqStPerS<Pair<K, V>>)
        {
            let collected = self.entries.clone();
            proof {
                assert(Pair_feq_trigger::<K, V>());
                lemma_seq_map_cloned_view_eq(
                    self.entries.seq@,
                    collected.seq@,
                );
            }
            collected
        }
    }

    // veracity: no_requires
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: Vec<Pair<K, V>>,
    ) -> (cloned: TableStPer<K, V>)
        ensures cloned@.dom().finite()
    {
        let seq = ArraySeqStPerS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableStPer { entries: seq }
    }

    /// APAS Algorithm 42.3: collect groups a sequence of (key, value) pairs into a table
    /// mapping each key to the subsequence of values for that key, preserving order.
    pub fn collect_by_key<K: StT + Ord + Eq, V: StT>(
        pairs: &ArraySeqStPerS<Pair<K, V>>,
    ) -> (grouped: TableStPer<K, ArraySeqStPerS<V>>)
        requires
            obeys_view_eq::<K>(),
            obeys_feq_clone::<V>(),
            obeys_feq_clone::<K>(),
            obeys_feq_full::<ArraySeqStPerS<V>>(),
            obeys_feq_full::<Pair<K, ArraySeqStPerS<V>>>(),
        ensures
            grouped.spec_tablestper_wf(),
            forall|k: K::V| grouped@.contains_key(k)
                <==> spec_collect_domain::<K::V, V::V>(pairs@).contains(k),
            forall|k: K::V| #[trigger] grouped@.contains_key(k)
                ==> grouped@[k] == spec_collect_key::<K::V, V::V>(pairs@, k),
    {
        let mut result = TableStPer::<K, ArraySeqStPerS<V>>::empty();
        let ghost pairs_view = pairs@;
        let mut i: usize = 0;
        while i < pairs.length()
            invariant
                i <= pairs.spec_len(),
                pairs@ == pairs_view,
                result.spec_tablestper_wf(),
                result@.dom().finite(),
                forall|k: K::V| #[trigger] result@.contains_key(k)
                    <==> spec_collect_domain::<K::V, V::V>(
                        pairs_view.subrange(0, i as int)).contains(k),
                forall|k: K::V| #[trigger] result@.contains_key(k)
                    ==> result@[k] == spec_collect_key::<K::V, V::V>(
                        pairs_view.subrange(0, i as int), k),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<V>(),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<ArraySeqStPerS<V>>(),
                obeys_feq_full::<Pair<K, ArraySeqStPerS<V>>>(),
            decreases pairs.spec_len() - i,
        {
            let ghost sub_i = pairs_view.subrange(0, i as int);
            let ghost sub_i1 = pairs_view.subrange(0, (i + 1) as int);
            let ghost old_result_view = result@;

            let pair = pairs.nth(i);
            let ghost key_view: K::V = pair.0@;
            let ghost pair_val_view: V::V = pair.1@;

            let val_clone = pair.1.clone_plus();
            proof { reveal(obeys_feq_clone); }
            let val_seq = ArraySeqStPerS::<V>::singleton(val_clone);

            let existing = result.find(&pair.0);
            let mut key_existed: bool = false;

            // Build new_seq and prove new_seq@ =~= spec_collect_key(sub_i1, key_view)
            // inside each match arm where seq/val_seq are still in scope.
            let new_seq = match existing {
                Some(seq) => {
                    key_existed = true;
                    proof {
                        lemma_spec_collect_key_len_bound::<K::V, V::V>(sub_i, key_view);
                    }
                    let appended = ArraySeqStPerS::append(&seq, &val_seq);
                    proof {
                        lemma_spec_collect_key_step::<K::V, V::V>(
                            pairs_view, key_view, i as int);
                        assert(seq@ == spec_collect_key::<K::V, V::V>(
                            sub_i, key_view));
                        // Trigger second append ensures via val_seq.seq@ term.
                        assert(val_seq.seq@[0int] == val_clone);
                        assert(val_clone@ == pair_val_view);
                        assert forall|j: int| 0 <= j < appended@.len()
                            implies #[trigger] appended@[j]
                                == seq@.push(pair_val_view)[j]
                        by {
                            // Fire append ensures trigger via spec_index call.
                            let ghost ai = appended.spec_index(j);
                            if j < seq@.len() {
                                let ghost si = seq.spec_index(j);
                                assert(ai@ == si@);
                            } else {
                                assert(val_seq.seq@[0int] == val_clone);
                            }
                        };
                        assert(appended@
                            =~= seq@.push(pair_val_view));
                    }
                    appended
                },
                None => {
                    proof {
                        lemma_spec_collect_key_not_in_domain::<K::V, V::V>(
                            sub_i, key_view);
                        lemma_spec_collect_key_step::<K::V, V::V>(
                            pairs_view, key_view, i as int);
                        assert(val_seq@
                            =~= Seq::<V::V>::empty().push(pair_val_view));
                    }
                    val_seq
                },
            };

            let ghost new_seq_view = new_seq@;
            assert(new_seq_view
                =~= spec_collect_key::<K::V, V::V>(sub_i1, key_view));

            if key_existed {
                result = result.delete(&pair.0);
            }
            proof { assert(!result@.contains_key(key_view)); }

            let key_clone = pair.0.clone_plus();
            proof { assert(key_clone@ == key_view) by { reveal(obeys_feq_clone); }; }
            result = result.insert(
                key_clone, new_seq,
                |_old: &ArraySeqStPerS<V>, new_val: &ArraySeqStPerS<V>| new_val.clone());

            proof {
                lemma_spec_collect_domain_step::<K::V, V::V>(pairs_view, i as int);
                assert(result@[key_view] == new_seq_view);

                assert forall|k: K::V| #[trigger] result@.contains_key(k)
                    implies result@[k]
                        == spec_collect_key::<K::V, V::V>(sub_i1, k)
                by {
                    if k == key_view {
                    } else {
                        lemma_spec_collect_key_step::<K::V, V::V>(
                            pairs_view, k, i as int);
                    }
                };
            }

            i += 1;
        }
        proof {
            assert(pairs_view.subrange(0, pairs.spec_len() as int) =~= pairs_view);
        }
        result
    }

    //		11. derive impls in verus!

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord + View + PartialEq, V: StT + View + PartialEq> PartialEqSpecImpl for TableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord + Eq + View, V: StT + Eq + View> Eq for TableStPer<K, V> {}

    impl<K: StT + Ord + PartialEq + View, V: StT + PartialEq + View> PartialEq for TableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.entries == other.entries;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT> Clone for TableStPer<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned.entries@ == self.entries@
        {
            let cloned = TableStPer { entries: self.entries.clone() };
            proof {
                assume(obeys_feq_clone::<Pair<K, V>>());
                lemma_seq_map_cloned_view_eq(
                    self.entries.seq@,
                    cloned.entries.seq@,
                );
            }
            cloned
        }
    }

    } // verus!

    // 12. macros

    //		12. macros

    /// Macro for creating table literals.
    #[macro_export]
    macro_rules! TableStPerLit {
        () => {
            $crate::Chap42::TableStPer::TableStPer::TableStPer::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableStPer::TableStPer::from_sorted_entries(entries)
        }};
    }

    // 13. derive impls outside verus!

    //		13. derive impls outside verus!

    impl<K: StT + Ord, V: StT> std::fmt::Debug for TableStPer<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TableStPer").finish()
        }
    }

    impl<K: StT + Ord, V: StT> std::fmt::Display for TableStPer<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TableStPer(len={})", self.entries.length())
        }
    }
}
