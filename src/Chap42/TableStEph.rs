//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store.

pub mod TableStEph {

    use std::cmp::Ordering;
    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
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
    // 6. spec fns
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

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEph<K: StT + Ord, V: StT> {
        pub entries: ArraySeqStEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStEph<K, V>;

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

    impl<K: StT + Ord, V: StT> View for TableStEph<K, V> {
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

    impl<K: StT + Ord, V: StT> TableStEph<K, V> {
        pub open spec fn spec_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
        }
    }

    // 7. proof fns

    // If a key is in spec_entries_to_map, it appears in the seq.
    proof fn lemma_entries_to_map_key_in_seq<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
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
    proof fn lemma_entries_to_map_contains_key<KV, VV>(entries: Seq<(KV, VV)>, idx: int)
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
    proof fn lemma_entries_to_map_len<KV, VV>(entries: Seq<(KV, VV)>)
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
    proof fn lemma_entries_to_map_no_key<KV, VV>(entries: Seq<(KV, VV)>, k: KV)
        requires forall|i: int| 0 <= i < entries.len() ==> (#[trigger] entries[i]).0 != k
        ensures !spec_entries_to_map(entries).contains_key(k)
    {
        if spec_entries_to_map(entries).contains_key(k) {
            lemma_entries_to_map_key_in_seq(entries, k);
        }
    }

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

    // If two sequences have the same keys at each position, their maps have the same domain.
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

    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_wf(&self) -> bool;

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: usize)
            requires self.spec_wf()
            ensures result == self@.len();
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> (result: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures result@ == Map::<K::V, V::V>::empty().insert(key@, value@);
        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite();
        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        /// APAS: Work Θ(Σ W(f(v))), Span Θ(lg |a| + max S(f(v)))
        fn map<F: Fn(&V) -> V>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom();
        /// APAS: Work Θ(Σ W(p(k,v))), Span Θ(lg |a| + max S(p(k,v)))
        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures self@.dom().subset_of(old(self)@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures self@.dom().subset_of(old(self)@.dom().intersect(other@.dom()));
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures old(self)@.dom().union(other@.dom()).subset_of(self@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn difference(&mut self, other: &Self)
            ensures self@.dom().subset_of(old(self)@.dom().difference(other@.dom()));
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn find(&self, key: &K) -> (result: Option<V>)
            requires self.spec_wf()
            ensures
                match result {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn delete(&mut self, key: &K)
            ensures !self@.contains_key(key@);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            ensures self@.contains_key(key@);
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().subset_of(old(self)@.dom());

        /// Returns a flat sequence of (K, V) pairs in key order.
        fn entries(&self) -> (result: ArraySeqStEphS<Pair<K, V>>);
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> TableStEphTrait<K, V> for TableStEph<K, V> {
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
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            let entries = ArraySeqStEphS::empty();
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            TableStEph { entries }
        }

        fn singleton(key: K, value: V) -> (result: Self)
        {
            let entries = ArraySeqStEphS::singleton(Pair(key, value));
            assert(entries@ =~= seq![(key@, value@)]);
            proof {
                let s = entries@;
                assert(s.len() == 1);
                assert(s.drop_last() =~= Seq::<(K::V, V::V)>::empty());
                assert(spec_entries_to_map(s.drop_last()) =~= Map::<K::V, V::V>::empty());
                assert(s.last() == (key@, value@));
            }
            TableStEph { entries }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures result@.finite()
        {
            let mut keys = ArraySetStEph::empty();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                keys.insert(pair.0.clone());
            }
            keys
        }

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let key_seq = keys.to_seq();
            let mut entries = Vec::with_capacity(key_seq.length());
            for i in 0..key_seq.length() {
                let key = key_seq.nth(i);
                let value = f(key);
                entries.push(Pair(key.clone(), value));
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            TableStEph {
                entries: ArraySeqStEphS::from_vec(entries),
            }
        }

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom()
        {
            let mapped_entries = ArraySeqStEphS::tabulate(
                &|i| {
                    let pair = self.entries.nth(i);
                    let new_value = f(&pair.1);
                    Pair(pair.0.clone(), new_value)
                },
                self.entries.length(),
            );
            self.entries = mapped_entries;
        }

        fn filter<F: Fn(&K, &V) -> B>(&mut self, f: F)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    forall|k: &K, v: &V| f.requires((k, v)),
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@,
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
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
                    self@.dom().contains(k)
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
            }
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures self@.dom().subset_of(old(self)@.dom().intersect(other@.dom()))
        {
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => i += 1,
                    Ordering::Greater => j += 1,
                    Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        intersection_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            self.entries = ArraySeqStEphS::from_vec(intersection_entries);
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, combine: F)
            ensures old(self)@.dom().union(other@.dom()).subset_of(self@.dom())
        {
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => {
                        union_entries.push(pair1.clone());
                        i += 1;
                    }
                    Ordering::Greater => {
                        union_entries.push(pair2.clone());
                        j += 1;
                    }
                    Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        union_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            while i < self.entries.length() {
                union_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            while j < other.entries.length() {
                union_entries.push(other.entries.nth(j).clone());
                j += 1;
            }

            self.entries = ArraySeqStEphS::from_vec(union_entries);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@.dom().subset_of(old(self)@.dom().difference(other@.dom()))
        {
            let mut difference_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i);
                let pair2 = other.entries.nth(j);

                match pair1.0.cmp(&pair2.0) {
                    Ordering::Less => {
                        difference_entries.push(pair1.clone());
                        i += 1;
                    }
                    Ordering::Greater => j += 1,
                    Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }

            while i < self.entries.length() {
                difference_entries.push(self.entries.nth(i).clone());
                i += 1;
            }

            self.entries = ArraySeqStEphS::from_vec(difference_entries);
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

        #[verifier::external_body]
        fn delete(&mut self, key: &K)
            ensures !self@.contains_key(key@)
        {
            let mut result = Vec::new();
            for i in 0..self.entries.length() {
                let pair = self.entries.nth(i);
                if &pair.0 != key {
                    result.push(pair.clone());
                }
            }
            self.entries = ArraySeqStEphS::from_vec(result);
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            ensures self@.contains_key(key@)
        {
            if let Some(existing_value) = self.find(&key) {
                let combined_value = combine(&existing_value, &value);
                let mut updated_entries = Vec::new();
                for i in 0..self.entries.length() {
                    let pair = self.entries.nth(i);
                    if pair.0 != key {
                        updated_entries.push(pair.clone());
                    }
                }
                updated_entries.push(Pair(key, combined_value));
                updated_entries.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqStEphS::from_vec(updated_entries);
            } else {
                let mut new_entries = Vec::with_capacity(self.entries.length() + 1);
                for i in 0..self.entries.length() {
                    new_entries.push(self.entries.nth(i).clone());
                }
                new_entries.push(Pair(key, value));
                new_entries.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqStEphS::from_vec(new_entries);
            }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@,
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
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
                    self@.dom().contains(k)
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
            }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            proof { assume(obeys_feq_full::<Pair<K, V>>()); }
            let ghost old_view = self.entries@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@,
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
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                assert forall|k: K::V| #![auto]
                    self@.dom().contains(k)
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
            }
        }

        fn entries(&self) -> (result: ArraySeqStEphS<Pair<K, V>>) {
            self.entries.clone()
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for TableStEph<K, V> {
        fn clone(&self) -> (result: Self) {
            TableStEph {
                entries: self.entries.clone(),
            }
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: Vec<Pair<K, V>>,
    ) -> (result: TableStEph<K, V>)
        ensures result@.dom().finite()
    {
        let seq = ArraySeqStEphS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableStEph { entries: seq }
    }

    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<K: StT + Ord, V: StT> Default for TableStEph<K, V> {
        fn default() -> Self {
            TableStEph::empty()
        }
    }

    impl<K: StT + Ord, V: StT> PartialEq for TableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }

    impl<K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for TableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEph({:?})", self.entries)
        }
    }
}
