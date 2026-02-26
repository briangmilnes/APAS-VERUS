//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 single-threaded persistent table implementation using ArraySeq as backing store.

pub mod TableStPer {

    use std::cmp::Ordering;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

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

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::map::group_map_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

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

    // 7. proof fns

    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStPer<K: StT + Ord, V: StT> {
        pub entries: ArraySeqStPerS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for TableStPer<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }

    // 6. spec fns

    // Keys in the entry sequence are unique.
    pub open spec fn spec_keys_no_dups<KV, VV>(entries: Seq<(KV, VV)>) -> bool {
        forall|i: int, j: int|
            0 <= i < j < entries.len() ==> (#[trigger] entries[i]).0 != (#[trigger] entries[j]).0
    }

    impl<K: StT + Ord, V: StT> TableStPer<K, V> {
        pub open spec fn spec_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }
    }

    // 7. proof fns

    // When keys are unique, spec_entries_to_map length equals seq length.
    // If entries[idx] = (k, v) and keys are unique, map contains k with value v.
    proof fn lemma_entries_to_map_get<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
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
    proof fn lemma_entries_to_map_dom_subset<KV, VV>(
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
    proof fn lemma_entries_to_map_contains_key<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
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
    proof fn lemma_entries_to_map_no_key<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires forall|i: int| 0 <= i < entries.len() ==> (#[trigger] entries[i]).0 != k,
        ensures !spec_entries_to_map(entries).contains_key(k),
    {
        if spec_entries_to_map(entries).contains_key(k) {
            lemma_entries_to_map_key_in_seq(entries, k);
        }
    }

    // If a key is in spec_entries_to_map, it appears in the seq.
    proof fn lemma_entries_to_map_key_in_seq<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
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
    proof fn lemma_entries_to_map_len<KV, VV>(entries: Seq<(KV, VV)>)
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

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42.
    pub trait TableStPerTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_wf(&self) -> bool;

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: usize)
            requires self.spec_wf(),
            ensures result == self@.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty(), result.spec_wf();

        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> (result: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures result@ == Map::<K::V, V::V>::empty().insert(key@, value@), result.spec_wf();

        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite();

        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            requires forall|k: &K| f.requires((k,)),
            ensures result@.dom().finite();

        /// APAS: Work Θ(|a| * W(f)), Span Θ(lg |a| + S(f))
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (result: Self)
            requires forall|v: &V| f.requires((v,)),
            ensures result@.dom() == self@.dom();

        /// APAS: Work Θ(|a| * W(f)), Span Θ(lg |a| + S(f))
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
            requires forall|k: &K, v: &V| f.requires((k, v)),
            ensures result@.dom().subset_of(self@.dom());

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (result: Self)
            requires forall|v1: &V, v2: &V| combine.requires((v1, v2)),
            ensures result@.dom().subset_of(self@.dom().intersect(other@.dom()));

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (result: Self)
            requires self.spec_wf(), forall|v1: &V, v2: &V| combine.requires((v1, v2)),
            ensures self@.dom().union(other@.dom()).subset_of(result@.dom()), result.spec_wf();

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().subset_of(self@.dom().difference(other@.dom()));

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn find(&self, key: &K) -> (result: Option<V>)
            requires self.spec_wf(),
            ensures
                match result {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn delete(&self, key: &K) -> (result: Self)
            requires self.spec_wf(), obeys_feq_clone::<Pair<K, V>>(),
            ensures !result@.contains_key(key@), result.spec_wf();

        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (result: Self)
            requires self.spec_wf(), forall|v1: &V, v2: &V| combine.requires((v1, v2)),
            ensures result@.contains_key(key@), result.spec_wf();

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().subset_of(self@.dom());

        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().subset_of(self@.dom());

        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn collect(&self) -> (result: ArraySeqStPerS<Pair<K, V>>);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> TableStPerTrait<K, V> for TableStPer<K, V> {
        open spec fn spec_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }

        fn size(&self) -> (result: usize)
        {
            proof {
                lemma_entries_to_map_len::<K::V, V::V>(self.entries@);
            }
            self.entries.length()
        }

        fn empty() -> (result: Self)
        {
            let entries = ArraySeqStPerS::empty();
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            TableStPer { entries }
        }

        fn singleton(key: K, value: V) -> (result: Self)
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

        fn domain(&self) -> (result: ArraySetStEph<K>)
        {
            let mut keys: Vec<K> = Vec::new();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant i <= self.entries.spec_len(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                keys.push(pair.0.clone());
                i += 1;
            }
            ArraySetStEph::from_seq(ArraySeqStEphS::from_vec(keys))
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
        {
            let key_seq = keys.to_seq();
            let len = key_seq.length();
            let entries = ArraySeqStPerS::tabulate(
                &(|i: usize| -> (r: Pair<K, V>)
                    requires
                        i < key_seq.spec_len(),
                        forall|k: &K| f.requires((k,)),
                {
                    let key = key_seq.nth(i);
                    let value = f(key);
                    Pair(key.clone_plus(), value)
                }),
                len,
            );
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(entries@);
            }
            TableStPer { entries }
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> (result: Self)
        {
            proof { assume(obeys_feq_full::<K>()); }
            let ghost old_view = self.entries@;
            let mut new_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    new_entries@.len() == i as int,
                    self.entries@ == old_view,
                    forall|j: int| #![auto] 0 <= j < i as int ==>
                        new_entries@[j].0@ == old_view[j].0,
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
            }
            TableStPer { entries }
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (result: Self)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@,
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if f(&pair.0, &pair.1) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof { sources = sources.push(i as int); }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                // Prove subset directly: any key in result map is also in self map.
                assert forall|k: K::V| #![auto]
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
            }
            TableStPer { entries }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (result: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut self_srcs: Seq<int> = Seq::empty();
            let ghost mut other_srcs: Seq<int> = Seq::empty();
            proof {
                assume(obeys_view_eq::<K>());
                assume(obeys_feq_full::<K>());
            }
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    other.entries@ == other_view,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    self_srcs.len() == kept@.len(),
                    other_srcs.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < self_srcs.len() ==>
                        0 <= self_srcs[j] < self_view.len()
                        && self_view[self_srcs[j]].0 == kept@[j].0@,
                    forall|j: int| #![auto] 0 <= j < other_srcs.len() ==>
                        0 <= other_srcs[j] < other_view.len()
                        && other_view[other_srcs[j]].0 == kept@[j].0@,
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
                        self_srcs = self_srcs.push(i as int);
                        other_srcs = other_srcs.push(found_idx as int);
                    }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
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
            }
            TableStPer { entries }
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: F) -> (result: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            let mut result = self.clone_plus();
            proof {
                // clone preserves entries@, so spec_wf transfers.
                assume(result.spec_wf());
            }
            let mut j: usize = 0;
            while j < other.entries.length()
                invariant
                    j <= other.entries.spec_len(),
                    other.entries@ == other_view,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    result.spec_wf(),
                decreases other.entries.spec_len() - j,
            {
                let pair = other.entries.nth(j);
                let cloned = pair.clone_plus();
                result = result.insert(cloned.0, cloned.1, &combine);
                j += 1;
            }
            proof {
                assume(self@.dom().union(other@.dom()).subset_of(result@.dom()));
            }
            result
        }

        fn difference(&self, other: &Self) -> (result: Self)
        {
            let ghost self_view = self.entries@;
            let ghost other_view = other.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            proof {
                assume(obeys_view_eq::<K>());
                assume(obeys_feq_full::<Pair<K, V>>());
            }
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    other.entries@ == other_view,
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@,
                    forall|j: int| #![auto] 0 <= j < kept@.len() ==>
                        !spec_entries_to_map(other_view).contains_key(kept@[j].0@),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                // Linear scan of other for this key.
                let mut found = false;
                let mut j: usize = 0;
                while j < other.entries.length() && !found
                    invariant
                        j <= other.entries.spec_len(),
                        other.entries@ == other_view,
                        !found ==> forall|jj: int| #![auto] 0 <= jj < j as int ==>
                            other_view[jj].0 != pair.0@,
                        obeys_view_eq::<K>(),
                    decreases other.entries.spec_len() - j,
                {
                    let other_pair = other.entries.nth(j);
                    proof { reveal(obeys_view_eq); }
                    if pair.0.eq(&other_pair.0) {
                        found = true;
                    }
                    j += 1;
                }
                if !found {
                    proof {
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, pair.0@);
                    }
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof { sources = sources.push(i as int); }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
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
            }
            TableStPer { entries }
        }

        fn find(&self, key: &K) -> (result: Option<V>)
        {
            proof {
                assume(obeys_view_eq::<K>());
                assume(obeys_feq_full::<V>());
            }
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.spec_wf(),
                    forall|j: int| #![auto] 0 <= j < i as int ==>
                        self.entries@[j].0 != key@,
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

        fn delete(&self, key: &K) -> (result: Self)
        {
            proof {
                assume(obeys_view_eq::<K>());
                assume(obeys_feq_full::<Pair<K, V>>());
            }
            let ghost self_view = self.entries@;
            let mut result: Vec<Pair<K, V>> = Vec::new();
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.spec_wf(),
                    forall|j: int| #![auto] 0 <= j < result@.len() ==>
                        result@[j].0@ != key@,
                    src.len() == result@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && result@[j].0@ == self_view[src[j]].0
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if !pair.0.eq(key) {
                    let cloned = pair.clone_plus();
                    result.push(cloned);
                    proof { src = src.push(i as int); }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(result);
            proof {
                assert forall|j: int| 0 <= j < entries@.len()
                    implies (#[trigger] entries@[j]).0 != key@
                by {
                    assert(entries.spec_index(j) == result@[j]);
                    assert(result@[j].0@ != key@);
                };
                lemma_entries_to_map_no_key::<K::V, V::V>(entries@, key@);
                // Prove spec_keys_no_dups(entries@).
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == result@[a]);
                        assert(entries.spec_index(b) == result@[b]);
                        assert(src[a] < src[b]);
                        assert(0 <= src[a] < self_view.len());
                        assert(0 <= src[b] < self_view.len());
                        assert(self_view[src[a]].0 != self_view[src[b]].0);
                    };
                };
            }
            TableStPer { entries }
        }

        fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (result: Self)
        {
            proof {
                assume(obeys_view_eq::<K>());
                assume(obeys_feq_full::<Pair<K, V>>());
            }
            let ghost key_view: K::V = key@;
            let ghost self_view = self.entries@;
            let mut all: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let ghost mut src: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    self.spec_wf(),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    src.len() == all@.len(),
                    forall|j: int| 0 <= j < src.len() ==> (
                        0 <= #[trigger] src[j] < i
                        && all@[j].0@ == self_view[src[j]].0
                    ),
                    forall|a: int, b: int| 0 <= a < b < src.len() ==> src[a] < src[b],
                    forall|j: int| #![auto] 0 <= j < all@.len() ==> all@[j].0@ != key_view,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    key@ == key_view,
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    found_value = Some(pair.1.clone_plus());
                } else {
                    let cloned = pair.clone_plus();
                    all.push(cloned);
                    proof { src = src.push(i as int); }
                }
                i += 1;
            }
            let final_value = match found_value {
                Some(old_val) => combine(&old_val, &value),
                None => value,
            };
            all.push(Pair(key, final_value));
            let entries = ArraySeqStPerS::from_vec(all);
            proof {
                let last = (entries@.len() - 1) as int;
                assert(entries.spec_index(last) == all@[last]);
                lemma_entries_to_map_contains_key::<K::V, V::V>(entries@, last);
                assert(entries@[last].0 == key_view);
                // Prove spec_keys_no_dups(entries@).
                assert(spec_keys_no_dups(entries@)) by {
                    assert forall|a: int, b: int|
                        0 <= a < b < entries@.len()
                        implies (#[trigger] entries@[a]).0 != (#[trigger] entries@[b]).0
                    by {
                        assert(entries.spec_index(a) == all@[a]);
                        assert(entries.spec_index(b) == all@[b]);
                        if b < entries@.len() - 1 {
                            // Both in the non-key portion.
                            assert(src[a] < src[b]);
                            assert(0 <= src[a] < self_view.len());
                            assert(0 <= src[b] < self_view.len());
                            assert(self_view[src[a]].0 != self_view[src[b]].0);
                        } else {
                            // b is the key entry, a is a non-key entry.
                            assert(all@[a].0@ != key_view);
                        }
                    };
                };
            }
            TableStPer { entries }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@,
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if keys.find(&pair.0) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof { sources = sources.push(i as int); }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
                    spec_entries_to_map(entries@).dom().contains(k)
                    implies spec_entries_to_map(self_view).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    assert(self_view[s].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
            }
            TableStPer { entries }
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost self_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == self_view,
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < self_view.len()
                        && self_view[sources[j]].0 == kept@[j].0@,
                    obeys_feq_full::<Pair<K, V>>(),
                decreases self.entries.spec_len() - i,
            {
                let pair = self.entries.nth(i);
                if !keys.find(&pair.0) {
                    let cloned = pair.clone_plus();
                    kept.push(cloned);
                    proof { sources = sources.push(i as int); }
                }
                i += 1;
            }
            let entries = ArraySeqStPerS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
                    spec_entries_to_map(entries@).dom().contains(k)
                    implies spec_entries_to_map(self_view).dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(entries@, k);
                    let idx = choose|idx: int| 0 <= idx < entries@.len()
                        && (#[trigger] entries@[idx]).0 == k;
                    assert(entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    assert(self_view[s].0 == kept@[idx].0@);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self_view, s);
                };
            }
            TableStPer { entries }
        }

        fn collect(&self) -> (result: ArraySeqStPerS<Pair<K, V>>)
        {
            self.entries.clone()
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for TableStPer<K, V> {
        fn clone(&self) -> (result: Self)
            ensures result.entries@ == self.entries@
        {
            let result = TableStPer { entries: self.entries.clone() };
            proof {
                assume(obeys_feq_clone::<Pair<K, V>>());
                lemma_seq_map_cloned_view_eq(
                    self.entries.seq@,
                    result.entries.seq@,
                );
            }
            result
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: Vec<Pair<K, V>>,
    ) -> (result: TableStPer<K, V>)
        ensures result@.dom().finite()
    {
        let seq = ArraySeqStPerS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableStPer { entries: seq }
    }

    } // verus!

    // 12. macros

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

    impl<K: StT + Ord, V: StT> PartialEq for TableStPer<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }

    impl<K: StT + Ord, V: StT> std::fmt::Debug for TableStPer<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TableStPer").finish()
        }
    }
}
