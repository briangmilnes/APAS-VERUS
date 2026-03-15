//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 42 multi-threaded ephemeral table implementation using ArraySeqMtEph as backing store.

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


pub mod TableMtEph {

    use std::cmp::Ordering;
    use std::sync::Arc;

    use vstd::prelude::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_seq_map_cloned_view_eq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, obeys_view_eq_trigger};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

    verus! {

//		3. broadcast use

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};


//		4. type definitions

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 3. broadcast use (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableMtEph<K: MtKey, V: MtVal> {
        pub entries: ArraySeqMtEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableMtEph<K, V>;


//		5. view impls

    impl<K: MtKey, V: MtVal> View for TableMtEph<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }


//		6. spec fns

    // 5. view impls

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

    pub proof fn lemma_entries_to_map_finite<KV, VV>(entries: Seq<(KV, VV)>)
        ensures spec_entries_to_map(entries).dom().finite()
        decreases entries.len()
    {
        if entries.len() > 0 {
            lemma_entries_to_map_finite::<KV, VV>(entries.drop_last());
        }
    }

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

    // Value preservation for subsequences: if filtered is a subsequence of entries
    // (via strictly increasing sources) that includes all entries with key k, the
    // map values agree on k.
    proof fn lemma_entries_to_map_subseq_value<KV, VV>(
        entries: Seq<(KV, VV)>,
        filtered: Seq<(KV, VV)>,
        sources: Seq<int>,
        k: KV,
    )
        requires
            filtered.len() == sources.len(),
            forall|j: int| 0 <= j < sources.len() ==>
                0 <= #[trigger] sources[j] < entries.len()
                && filtered[j] == entries[sources[j]],
            forall|j1: int, j2: int| 0 <= j1 < j2 < sources.len()
                ==> sources[j1] < sources[j2],
            forall|i: int| 0 <= i < entries.len() && (#[trigger] entries[i]).0 == k
                ==> exists|j: int| 0 <= j < sources.len() && sources[j] == i,
            spec_entries_to_map(filtered).contains_key(k),
        ensures
            spec_entries_to_map(entries).contains_key(k),
            spec_entries_to_map(filtered)[k] == spec_entries_to_map(entries)[k],
        decreases entries.len(),
    {
        // Base case: entries empty implies filtered empty, contradicting contains_key.
        if entries.len() == 0 {
            if sources.len() > 0 {
                assert(0 <= sources[0] < entries.len());
            }
            assert(sources.len() == 0);
            assert(filtered.len() == 0);
            return;
        }
        let last = entries.last();
        let n = entries.len() - 1;
        let prefix = entries.drop_last();
        if last.0 == k {
            // Last entry has key k, so it was kept via sources.
            assert(entries[n as int].0 == k);
            let j_last = choose|j: int| 0 <= j < sources.len() && sources[j] == n;
            // j_last is the last source (largest).
            assert(j_last == sources.len() - 1) by {
                if j_last < sources.len() - 1 {
                    assert(sources[j_last] < sources[j_last + 1]);
                    assert(sources[j_last + 1] < entries.len());
                    assert(n < entries.len());
                }
            };
            assert(filtered.len() > 0);
            assert(filtered[filtered.len() - 1] == entries[n as int]);
            assert(filtered.last() == last);
            // Both maps have k -> last.1 because last entry with key k determines the value.
        } else {
            // last.0 != k. Map value for k comes from prefix.
            let last_kept = exists|j: int| 0 <= j < sources.len() && sources[j] == n;
            if last_kept {
                // Last was kept, so filtered.last() == last.
                let j_last = choose|j: int| 0 <= j < sources.len() && sources[j] == n;
                assert(j_last == sources.len() - 1) by {
                    if j_last < sources.len() - 1 {
                        assert(sources[j_last] < sources[j_last + 1]);
                        assert(sources[j_last + 1] < entries.len());
                    }
                };
                assert(filtered.len() > 0);
                assert(filtered[filtered.len() - 1] == entries[n as int]);
                assert(filtered.last() == last);
                let f_prefix = filtered.drop_last();
                let s_prefix = sources.drop_last();
                assert(f_prefix.len() == s_prefix.len());
                // filtered map = f_prefix map .insert(last.0, last.1), last.0 != k.
                assert(spec_entries_to_map(f_prefix).contains_key(k));
                // Establish preconditions for recursive call.
                assert forall|j: int| 0 <= j < s_prefix.len() implies
                    0 <= #[trigger] s_prefix[j] < prefix.len()
                    && f_prefix[j] == prefix[s_prefix[j]]
                by {
                    assert(j < sources.len() - 1);
                    assert(j < sources.len());
                    assert(0 <= j < sources.len());
                    assert(s_prefix[j] == sources[j]);
                    assert(sources[j] < sources[j_last]);
                    assert(sources[j_last] == n);
                    assert(sources[j] < n);
                    assert(0 <= sources[j] < n);
                    assert(f_prefix[j] == filtered[j]);
                    assert(0 <= sources[j] < entries.len());
                    assert(filtered[j] == entries[sources[j]]);
                    assert(sources[j] < prefix.len());
                    assert(prefix[sources[j]] == entries[sources[j]]);
                };
                assert forall|j1: int, j2: int| 0 <= j1 < j2 < s_prefix.len()
                    implies s_prefix[j1] < s_prefix[j2]
                by {
                    assert(0 <= j1 < sources.len());
                    assert(0 <= j2 < sources.len());
                    assert(s_prefix[j1] == sources[j1]);
                    assert(s_prefix[j2] == sources[j2]);
                };
                assert forall|i: int| 0 <= i < prefix.len()
                    && (#[trigger] prefix[i]).0 == k
                    implies exists|j: int| 0 <= j < s_prefix.len() && s_prefix[j] == i
                by {
                    assert(0 <= i < entries.len());
                    assert(entries[i].0 == k);
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == i;
                    assert(j < sources.len() - 1) by {
                        if j == sources.len() - 1 {
                            assert(j == j_last);
                            assert(sources[j] == n);
                            assert(i == n);
                            assert(i < prefix.len());
                            assert(n == prefix.len());
                        }
                    };
                    assert(0 <= j < s_prefix.len());
                    assert(s_prefix[j] == sources[j]);
                };
                lemma_entries_to_map_subseq_value::<KV, VV>(prefix, f_prefix, s_prefix, k);
            } else {
                // Last was not kept. All sources point into prefix.
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < prefix.len()
                    && filtered[j] == prefix[sources[j]]
                by {
                    assert(0 <= sources[j] < entries.len());
                    if sources[j] == n {
                        // sources[j] == n but last not kept: contradiction.
                    }
                    assert(sources[j] != n);
                    assert(sources[j] < n);
                    assert(sources[j] < prefix.len());
                    assert(prefix[sources[j]] == entries[sources[j]]);
                };
                assert forall|i: int| 0 <= i < prefix.len()
                    && (#[trigger] prefix[i]).0 == k
                    implies exists|j: int| 0 <= j < sources.len() && sources[j] == i
                by {
                    assert(0 <= i < entries.len());
                    assert(entries[i].0 == k);
                };
                lemma_entries_to_map_subseq_value::<KV, VV>(prefix, filtered, sources, k);
            }
        }
    }


//		8. traits

    // 8. traits

    /// Trait defining the Table ADT operations from Chapter 42.
    pub trait TableMtEphTrait<K: MtKey, V: MtVal>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_tablemteph_wf(&self) -> bool;

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_tablemteph_wf()
            ensures count == self@.dom().len();
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_tablemteph_wf();
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(key: K, value: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(key@, value@), tree.spec_tablemteph_wf();
        /// APAS: Work Θ(|a|), Span Θ(lg |a|)
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures domain@ =~= self@.dom();
        /// APAS: Work Θ(|s| * W(f)), Span Θ(lg |s| + S(f))
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys@.finite()
            ensures tabulated@.dom() =~= keys@;
        /// APAS: Work Θ(Σ W(f(v))), Span Θ(lg |a| + max S(f(v)))
        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom();
        /// APAS: Work Θ(Σ W(p(k,v))), Span Θ(lg |a| + max S(p(k,v)))
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F)
            ensures
                self@.dom().subset_of(old(self)@.dom()),
                forall|k: K::V| #![auto] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            ensures self@.dom() =~= old(self)@.dom().intersect(other@.dom());
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                forall|k: K::V| #![auto] old(self)@.contains_key(k) && !other@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                forall|k: K::V| #![auto] other@.contains_key(k) && !old(self)@.contains_key(k)
                    ==> self@[k] == other@[k];
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn difference(&mut self, other: &Self)
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #![auto] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn find(&self, key: &K) -> (found: Option<V>)
            requires self.spec_tablemteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn delete(&mut self, key: &K)
            ensures self@ =~= old(self)@.remove(key@);
        /// APAS: Work Θ(lg |a|), Span Θ(lg |a|)
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, key: K, value: V, combine: F)
            ensures
                self@.contains_key(key@),
                self@.dom() =~= old(self)@.dom().insert(key@),
                forall|k: K::V| #![auto] k != key@ && old(self)@.contains_key(k) ==> self@[k] == old(self)@[k],
                !old(self)@.contains_key(key@) ==> self@[key@] == value@;
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires keys@.finite()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #![auto] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// APAS: Work Θ(m * lg(1 + n/m)), Span Θ(lg(n + m))
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires keys@.finite()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #![auto] self@.contains_key(k) ==> self@[k] == old(self)@[k];

        fn entries(&self) -> (entries: ArraySeqMtEphS<Pair<K, V>>)
            ensures spec_entries_to_map(entries@) == self@;
    }


//		9. impls

    impl<K: MtKey, V: MtVal> TableMtEphTrait<K, V> for TableMtEph<K, V> {
        open spec fn spec_tablemteph_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
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
            let entries = ArraySeqMtEphS::empty();
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            TableMtEph { entries }
        }

        fn singleton(key: K, value: V) -> (tree: Self)
        {
            proof { assert(Pair_feq_trigger::<K, V>()); }
            let entries = ArraySeqMtEphS::singleton(Pair(key, value));
            let tree = TableMtEph { entries };
            proof {
                let e = tree.entries@;
                assert(e.len() == 1);
                assert(e[0] == (key@, value@));
                assert(e.last() == e[e.len() - 1]);
                assert(e.drop_last().len() == 0);
                assert(spec_entries_to_map::<K::V, V::V>(e.drop_last()) =~= Map::<K::V, V::V>::empty());
                assert(spec_entries_to_map(e) =~=
                    spec_entries_to_map::<K::V, V::V>(e.drop_last()).insert(e.last().0, e.last().1));
            }
            tree
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let mut keys = ArraySetStEph::empty();
            let mut i: usize = 0;
            proof { assert(obeys_feq_full_trigger::<K>()); }
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    keys.spec_arraysetsteph_wf(),
                    keys@.finite(),
                    forall|j: int| #![auto] 0 <= j < i as int
                        ==> keys@.contains(self.entries@[j].0),
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
                    assert forall|j: int| #![auto] 0 <= j < i as int + 1
                        implies keys@.contains(self.entries@[j].0)
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
                assert forall|k: K::V| #![auto] keys@.contains(k) == self@.dom().contains(k)
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

        #[verifier::external_body]
        fn tabulate<F: Fn(&K) -> V + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            let key_seq = keys.to_seq();
            let f = Arc::new(f);
            let len = key_seq.length();

            if len == 0 {
                return TableMtEph::empty();
            }

            if len == 1 {
                let key = key_seq.nth(0);
                let value = f(key);
                return TableMtEph::singleton(key.clone(), value);
            }

            let mid = len / 2;
            let left_seq = key_seq.subseq_copy(0, mid);
            let right_seq = key_seq.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let f1 = move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let key = left_seq.nth(i);
                        let value = f_clone(key);
                        Pair(key.clone(), value)
                    },
                    left_seq.length(),
                )
            };
            let f2 = move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let key = right_seq.nth(i);
                        let value = f(key);
                        Pair(key.clone(), value)
                    },
                    right_seq.length(),
                )
            };
            let (left_entries, right_entries) = join(f1, f2);

            let total_len = left_entries.length() + right_entries.length();
            let mut entries = Vec::with_capacity(total_len);
            for i in 0..left_entries.length() {
                entries.push(left_entries.nth(i).clone());
            }
            for i in 0..right_entries.length() {
                entries.push(right_entries.nth(i).clone());
            }
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            TableMtEph {
                entries: ArraySeqMtEphS::from_vec(entries),
            }
        }

        #[verifier::external_body]
        fn map<F: Fn(&V) -> V + Send + Sync + 'static>(&mut self, f: F)
            ensures self@.dom() == old(self)@.dom()
        {
            let f = Arc::new(f);
            let len = self.entries.length();

            if len <= 1 {
                if len == 1 {
                    let pair = self.entries.nth(0).clone();
                    let new_value = f(&pair.1);
                    self.entries = ArraySeqMtEphS::singleton(Pair(pair.0, new_value));
                }
                return;
            }

            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let f1 = move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let pair = left_entries.nth(i).clone();
                        let new_value = f_clone(&pair.1);
                        Pair(pair.0, new_value)
                    },
                    left_entries.length(),
                )
            };
            let f2 = move || {
                ArraySeqMtEphS::tabulate(
                    &|i| {
                        let pair = right_entries.nth(i).clone();
                        let new_value = f(&pair.1);
                        Pair(pair.0, new_value)
                    },
                    right_entries.length(),
                )
            };
            let (left_mapped, right_mapped) = join(f1, f2);

            let mut mapped_entries = Vec::with_capacity(len);
            for i in 0..left_mapped.length() {
                mapped_entries.push(left_mapped.nth(i).clone());
            }
            for i in 0..right_mapped.length() {
                mapped_entries.push(right_mapped.nth(i).clone());
            }

            self.entries = ArraySeqMtEphS::from_vec(mapped_entries);
        }

        #[verifier::external_body]
        fn filter<F: Fn(&K, &V) -> B + Send + Sync + 'static>(&mut self, f: F)
        {
            let f = Arc::new(f);
            let len = self.entries.length();

            if len == 0 {
                return;
            }

            if len == 1 {
                let pair = self.entries.nth(0).clone();
                if !f(&pair.0, &pair.1) {
                    self.entries = ArraySeqMtEphS::empty();
                }
                return;
            }

            let mid = len / 2;
            let left_entries = self.entries.subseq_copy(0, mid);
            let right_entries = self.entries.subseq_copy(mid, len - mid);
            let f_clone = f.clone();

            let f1 = move || {
                let mut left_filtered = Vec::new();
                for i in 0..left_entries.length() {
                    let pair = left_entries.nth(i).clone();
                    if f_clone(&pair.0, &pair.1) {
                        left_filtered.push(pair);
                    }
                }
                left_filtered
            };
            let f2 = move || {
                let mut right_filtered = Vec::new();
                for i in 0..right_entries.length() {
                    let pair = right_entries.nth(i).clone();
                    if f(&pair.0, &pair.1) {
                        right_filtered.push(pair);
                    }
                }
                right_filtered
            };
            let (left_filtered, right_filtered) = join(f1, f2);

            let mut filtered_entries = Vec::with_capacity(left_filtered.len() + right_filtered.len());
            filtered_entries.extend(left_filtered.iter().cloned());
            filtered_entries.extend(right_filtered.iter().cloned());

            self.entries = ArraySeqMtEphS::from_vec(filtered_entries);
        }

        #[verifier::external_body]
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
            ensures self@.dom() =~= old(self)@.dom().intersect(other@.dom())
        {
            let combine = Arc::new(combine);
            let mut intersection_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i).clone();
                let pair2 = other.entries.nth(j).clone();

                match pair1.0.cmp(&pair2.0) {
                    | Ordering::Less => i += 1,
                    | Ordering::Greater => j += 1,
                    | Ordering::Equal => {
                        let combined_value = combine(&pair1.1, &pair2.1);
                        intersection_entries.push(Pair(pair1.0.clone(), combined_value));
                        i += 1;
                        j += 1;
                    }
                }
            }

            self.entries = ArraySeqMtEphS::from_vec(intersection_entries);
        }

        #[verifier::external_body]
        fn union<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
            ensures self@.dom() =~= old(self)@.dom().union(other@.dom())
        {
            let combine = Arc::new(combine);
            let mut union_entries = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < self.entries.length() && j < other.entries.length() {
                let pair1 = self.entries.nth(i).clone();
                let pair2 = other.entries.nth(j).clone();

                match pair1.0.cmp(&pair2.0) {
                    | Ordering::Less => {
                        union_entries.push(pair1.clone());
                        i += 1;
                    }
                    | Ordering::Greater => {
                        union_entries.push(pair2.clone());
                        j += 1;
                    }
                    | Ordering::Equal => {
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

            self.entries = ArraySeqMtEphS::from_vec(union_entries);
        }

        fn difference(&mut self, other: &Self)
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                assert(obeys_view_eq_trigger::<K>());
            }
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
                    forall|k: int| #![auto] 0 <= k < sources.len() ==>
                        0 <= sources[k] < old_self_view.len()
                        && old_self_view[sources[k]].0 == kept@[k].0@
                        && old_self_view[sources[k]].1 == kept@[k].1@
                        && !spec_entries_to_map(other_view).contains_key(kept@[k].0@),
                    forall|si: int| 0 <= si < i as int
                        && !spec_entries_to_map(other_view).contains_key(
                            (#[trigger] old_self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| #![auto] 0 <= j < sources.len() ==> sources[j] < i as int,
                    forall|j1: int, j2: int| #![auto]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    obeys_feq_full::<Pair<K, V>>(),
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
                            forall|jj: int| #![auto] 0 <= jj < j as int ==>
                                other_view[jj].0 != key_view,
                        key_view == pair_i.0@,
                        obeys_view_eq::<K>(),
                    decreases other_len - j,
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_self_view).dom().difference(
                    other@.dom());
                // View-level subsequence connection.
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_self_view.len()
                    && self.entries@[j] == old_self_view[sources[j]]
                by {
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
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
                assert forall|k: K::V| #![auto]
                    self@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_self_view)[k]
                by {
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        old_self_view, self.entries@, sources, k);
                };
            }
        }

        fn find(&self, key: &K) -> (found: Option<V>)
        {
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.spec_tablemteph_wf(),
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

        fn delete(&mut self, key: &K)
            ensures self@ =~= old(self)@.remove(key@)
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                assert(obeys_view_eq_trigger::<K>());
            }
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
                    forall|j: int| #![auto] 0 <= j < src.len() ==>
                        0 <= src[j] < old_view.len()
                        && old_view[src[j]].0 == kept@[j].0@
                        && old_view[src[j]].1 == kept@[j].1@,
                    forall|j: int| #![auto] 0 <= j < kept@.len() ==>
                        kept@[j].0@ != key@,
                    forall|j: int| #![trigger src[j]] 0 <= j < src.len() ==> src[j] < i as int,
                    forall|a: int, b: int| 0 <= a < b < src.len()
                        ==> src[a] < src[b],
                    forall|si: int| 0 <= si < i as int
                        && (#[trigger] old_view[si]).0 != key@
                        ==> exists|j: int| 0 <= j < src.len() && src[j] == si,
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                let ghost result_map = spec_entries_to_map(self.entries@);
                let ghost target_map = old_map.remove(key@);
                // View-level subsequence connection.
                assert forall|j: int| 0 <= j < src.len() implies
                    0 <= #[trigger] src[j] < old_view.len()
                    && self.entries@[j] == old_view[src[j]]
                by {
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
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
                assert forall|k: K::V| #![auto]
                    result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        old_view, self.entries@, src, k);
                };
            }
        }

        #[verifier::external_body]
        fn insert<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, key: K, value: V, combine: F)
            ensures
                self@.contains_key(key@),
                self@.dom() =~= old(self)@.dom().insert(key@)
        {
            if let Some(existing_value) = self.find(&key) {
                let combined_value = combine(&existing_value, &value);
                let len = self.entries.length();

                if len == 1 {
                    self.entries = ArraySeqMtEphS::singleton(Pair(key, combined_value));
                    return;
                }

                let mid = len / 2;
                let left_entries = self.entries.subseq_copy(0, mid);
                let right_entries = self.entries.subseq_copy(mid, len - mid);
                let key_clone = key.clone();
                let combined_clone = combined_value.clone();

                let f1 = move || {
                    ArraySeqMtEphS::tabulate(
                        &|i| {
                            let pair = left_entries.nth(i).clone();
                            if pair.0 == key_clone {
                                Pair(key_clone.clone(), combined_clone.clone())
                            } else {
                                pair
                            }
                        },
                        left_entries.length(),
                    )
                };
                let f2 = move || {
                    ArraySeqMtEphS::tabulate(
                        &|i| {
                            let pair = right_entries.nth(i).clone();
                            if pair.0 == key {
                                Pair(key.clone(), combined_value.clone())
                            } else {
                                pair
                            }
                        },
                        right_entries.length(),
                    )
                };
                let (left_updated, right_updated) = join(f1, f2);

                let mut updated_entries = Vec::with_capacity(len);
                for i in 0..left_updated.length() {
                    updated_entries.push(left_updated.nth(i).clone());
                }
                for i in 0..right_updated.length() {
                    updated_entries.push(right_updated.nth(i).clone());
                }

                self.entries = ArraySeqMtEphS::from_vec(updated_entries);
            } else {
                let new_pair = Pair(key, value);
                let new_entries = ArraySeqMtEphS::tabulate(
                    &|i| {
                        if i < self.entries.length() {
                            self.entries.nth(i).clone()
                        } else {
                            new_pair.clone()
                        }
                    },
                    self.entries.length() + 1,
                );
                let mut entries_vec = Vec::with_capacity(new_entries.length());
                for i in 0..new_entries.length() {
                    entries_vec.push(new_entries.nth(i).clone());
                }
                entries_vec.sort_by(|a, b| a.0.cmp(&b.0));
                self.entries = ArraySeqMtEphS::from_vec(entries_vec);
            }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
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
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@
                        && keys@.contains(kept@[j].0@),
                    forall|si: int| 0 <= si < i as int
                        && keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| #![auto] 0 <= j < sources.len() ==> sources[j] < i as int,
                    forall|j1: int, j2: int| #![auto]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    obeys_feq_full::<Pair<K, V>>(),
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().intersect(keys@);
                // View-level subsequence connection.
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_view.len()
                    && self.entries@[j] == old_view[sources[j]]
                by {
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
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
                assert forall|k: K::V| #![auto]
                    self@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_view)[k]
                by {
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        old_view, self.entries@, sources, k);
                };
            }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_view = self.entries@;
            let ghost old_map = spec_entries_to_map(old_view);
            assert(obeys_feq_full_trigger::<K>());
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut sources: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_view,
                    keys@.finite(),
                    sources.len() == kept@.len(),
                    forall|j: int| #![auto] 0 <= j < sources.len() ==>
                        0 <= sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@
                        && !keys@.contains(kept@[j].0@),
                    forall|si: int| 0 <= si < i as int
                        && !keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| #![auto] 0 <= j < sources.len() ==> sources[j] < i as int,
                    forall|j1: int, j2: int| #![auto]
                        0 <= j1 < j2 < sources.len() ==> sources[j1] < sources[j2],
                    obeys_feq_full::<Pair<K, V>>(),
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().difference(keys@);
                // View-level subsequence connection.
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_view.len()
                    && self.entries@[j] == old_view[sources[j]]
                by {
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
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
                assert forall|k: K::V| #![auto]
                    self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        old_view, self.entries@, sources, k);
                };
            }
        }

        fn entries(&self) -> (entries: ArraySeqMtEphS<Pair<K, V>>) {
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

    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: Vec<Pair<K, V>>) -> (constructed: TableMtEph<K, V>)
        requires true,
        ensures constructed@.dom().finite()
    {
        let seq = ArraySeqMtEphS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableMtEph { entries: seq }
    }




    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: MtKey, V: MtVal> PartialEqSpecImpl for TableMtEph<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: MtKey, V: MtVal> Eq for TableMtEph<K, V> {}

    impl<K: MtKey, V: MtVal> PartialEq for TableMtEph<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.entries == other.entries;
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: MtKey, V: MtVal> Clone for TableMtEph<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = TableMtEph {
                entries: self.entries.clone(),
            };
            proof { accept(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }

    } // verus!

    // 13. derive impls outside verus!


    //		13. derive impls outside verus!

    impl<K: MtKey, V: MtVal> std::fmt::Debug for TableMtEph<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TableMtEph")
                .field("size", &self.entries.length())
                .finish()
        }
    }

    impl<K: MtKey, V: MtVal> std::fmt::Display for TableMtEph<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TableMtEph(len={})", self.entries.length())
        }
    }

    // 12. macros


    //		12. macros

    /// Macro for creating multi-threaded ephemeral table literals
    #[macro_export]
    macro_rules! TableMtEphLit {
        () => {
            $crate::Chap42::TableMtEph::TableMtEph::TableMtEph::empty()
        };
        ($($key:expr => $value:expr),+ $(,)?) => {{
            let mut entries = vec![$($crate::Types::Types::Pair($key, $value)),+];
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            $crate::Chap42::TableMtEph::TableMtEph::from_sorted_entries(entries)
        }};
    }
}
