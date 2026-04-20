// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 42 single-threaded ephemeral table implementation using ArraySeq as backing store.

//  Table of Contents
//	Section 1. module
//	Section 2. imports (above)
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod TableStEph {

    use std::cmp::Ordering;
    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    pub use crate::Chap42::TableSpecsAndLemmas::TableSpecsAndLemmas::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::clone_view::clone_view::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::accept::accept;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    //		Section 2. imports (above)
    //		Section 3. broadcast use
    //		Section 4. type definitions
    //		Section 5. view impls
    //		Section 6. spec fns
    //		Section 7. proof fns
    //		Section 8. traits
    //		Section 9. impls
    //		Section 12. derive impls in verus!
    //		Section 13. macros
    //		Section 14. derive impls outside verus!

    verus! {


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEph<K: StT + Ord, V: StT> {
        pub entries: ArraySeqStEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableStEph<K, V>;


    impl<K: StT + Ord, V: StT> View for TableStEph<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }


    /// Trait defining the Table ADT operations from Chapter 42
    pub trait TableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_tablesteph_wf(&self) -> bool;

        /// Returns the concrete stored value for a given key.
        /// Useful for transferring exec-level properties (e.g., wf) through find_ref.
        spec fn spec_stored_value(&self, key: K::V) -> V;

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_tablesteph_wf()
            ensures count == self@.len();
        /// - APAS Cost Spec 42.5: Work 1, Span 1
        /// - Alg Analysis: APAS (Ch42 ref): Work O(1), Span O(1) -- agrees with APAS.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- agrees with APAS.
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: K, value: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@ == Map::<K::V, V::V>::empty().insert(key@, value@), tree.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: sequential key extraction
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), domain.spec_arraysetsteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|s| * W(f)), Span O(lg |s| + S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|·W(f)), Span O(|s|·W(f)) — ACCEPTED DIFFERENCE: sequential loop
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·W(f)), Span O(n·W(f)) — ACCEPTED DIFFERENCE: sequential loop
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(k,v))), Span O(n + Σ W(f(k,v))) — ACCEPTED DIFFERENCE: sequential loop
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: nested linear scans on array
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: nested linear scans on array
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: nested linear scans on array
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: linear scan on unsorted array
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan on flat array
        fn find_ref(&self, key: &K) -> (found: Option<&V>)
            requires self.spec_tablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@
                        && *v == self.spec_stored_value(key@),
                    None => !self@.contains_key(key@),
                };
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: linear scan + copy
        fn delete(&mut self, key: &K)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@ =~= old(self)@.remove(key@), self.spec_tablesteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: linear scan + copy
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan + rebuild with wf preservation
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan + rebuild with wf preservation
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: nested linear scans on array
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: nested linear scans on array
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires
                old(self).spec_tablesteph_wf(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.finite(),
            ensures
                self.spec_tablesteph_wf(),
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone of backing array.
        /// Returns a flat sequence of (K, V) pairs in key order.
        fn entries(&self) -> (entries: ArraySeqStEphS<Pair<K, V>>)
            ensures spec_entries_to_map(entries@) == self@;
    }


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
            // Veracity: NEEDED assert (speed hint)
            assert(self.entries@[sv_idx].0 == k) by {
                self.entries.lemma_view_index(sv_idx);
            };
// Veracity: UNNEEDED assert             assert(self.entries@[view_idx].0 == k);
            // lemma_entries_to_map_get tells us self@[k] == entries@[sv_idx].1.
            lemma_entries_to_map_get::<K::V, V::V>(self.entries@, sv_idx);
            // entries@[sv_idx].1 == entries.seq@[sv_idx].1@ (from View for Pair).
            self.entries.lemma_view_index(sv_idx);
            // spec_stored_value(k) == entries.seq@[sv_idx].1, so spec_stored_value(k)@ == self@[k].
        }

        /// Returns an iterator over table entries (key-value pairs).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1).
        pub fn iter<'a>(&'a self) -> (it: TableStEphIter<'a, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.entries.seq@,
                iter_invariant_table(&it),
        {
            TableStEphIter { inner: self.entries.iter() }
        }
    }


    impl<K: StT + Ord, V: StT> TableStEphTrait<K, V> for TableStEph<K, V> {
        open spec fn spec_tablesteph_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            self.entries.seq@[i].1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            // Veracity: NEEDED proof block
            proof {
                lemma_entries_to_map_len::<K::V, V::V>(self.entries@);
            }
            self.entries.length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            let entries = ArraySeqStEphS::empty();
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert             assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            proof {
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<V>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            TableStEph { entries }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: K, value: V) -> (tree: Self)
        {
            let entries = ArraySeqStEphS::singleton(Pair(key, value));
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert (speed hint)
            assert(entries@ =~= seq![(key@, value@)]);
            proof {
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<V>());
                // Veracity: NEEDED assert
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                let s = entries@;
// Veracity: UNNEEDED assert                 assert(s.len() == 1);
                // Veracity: NEEDED assert (speed hint)
                assert(s.drop_last() =~= Seq::<(K::V, V::V)>::empty());
                // Veracity: NEEDED assert
                assert(spec_entries_to_map(s.drop_last()) =~= Map::<K::V, V::V>::empty());
                // Veracity: NEEDED assert (speed hint)
                assert(s.last() == (key@, value@));
            }
            TableStEph { entries }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                // Veracity: NEEDED proof block
                let ghost old_keys = keys@;
                let key_clone = pair.0.clone_plus();
                keys.insert(key_clone);
                proof {
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|j: int| 0 <= j < i as int + 1
                        implies keys@.contains((#[trigger] self.entries@[j]).0)
                    by {
                        if j < i as int {
// Veracity: UNNEEDED assert                             assert(old_keys.contains(self.entries@[j].0));
                        }
                    };
                    // Veracity: NEEDED assert
                    assert forall|k: K::V| keys@.contains(k)
                        implies exists|j: int| 0 <= j < i as int + 1
                            && (#[trigger] self.entries@[j]).0 == k
                    by {
                        if old_keys.contains(k) {
                            let j = choose|j: int| 0 <= j < i as int
                                && (#[trigger] self.entries@[j]).0 == k;
                        } else {
                            // Veracity: NEEDED assert
                            assert(self.entries@[i as int].0 == k);
                        }
                    // Veracity: NEEDED proof block
                    };
                }
                i += 1;
            }
            proof {
                // Veracity: NEEDED assert
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
// Veracity: UNNEEDED assert                       assert(obeys_feq_full_trigger::<K>());
           // Veracity: NEEDED assert
           assert(obeys_feq_full_trigger::<V>());
           // Veracity: NEEDED assert
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
                // Veracity: NEEDED proof block
                decreases key_seq.spec_len() - i,
            {
                let key = key_seq.nth(i);
                let value = f(key);
                let key_clone = key.clone_plus();
                proof {
                    // Veracity: NEEDED proof block
                    lemma_cloned_view_eq::<K>(*key, key_clone);
                }
                entries.push(Pair(key_clone, value));
                i += 1;
            }
            let seq = ArraySeqStEphS::from_vec(entries);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(seq@);
                // Each entry key matches the corresponding key_seq element.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < seq@.len()
                    implies (#[trigger] seq@[j]).0 == key_seq@[j]
                by {
                    // Veracity: NEEDED assert
                    assert(seq.spec_index(j) == entries@[j]);
                };
                // No duplicate keys since key_seq has no duplicates.
                // Veracity: NEEDED assert (speed hint)
                assert(spec_keys_no_dups(seq@)) by {
                    // Veracity: NEEDED assert
                    assert forall|i: int, j: int|
                        0 <= i < j < seq@.len()
                        implies (#[trigger] seq@[i]).0 != (#[trigger] seq@[j]).0
                    by {};
                };
                // Domain matches keys@.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(seq@).dom().contains(k) == keys@.contains(k)
                by {
                    if spec_entries_to_map(seq@).dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                        let j = choose|j: int| 0 <= j < seq@.len()
                            && (#[trigger] seq@[j]).0 == k;
                        let _ = j;
                    }
                    if keys@.contains(k) {
                        let j = choose|j: int| 0 <= j < key_seq@.len()
                            && key_seq@[j] == k;
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                    }
                };
                // Closure ensures postcondition.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(seq@).contains_key(k)
                    implies exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && spec_entries_to_map(seq@)[k] == result@
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == k;
                    // Veracity: NEEDED assert
                    assert(seq.spec_index(j) == entries@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
// Veracity: UNNEEDED assert                     assert(key_seq.seq@[j]@ == key_seq@[j]);
                };
            }
            TableStEph { entries: seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                // Veracity: NEEDED proof block
                let pair = self.entries.nth(i);
                let new_value = f(&pair.1);
                let key_clone = pair.0.clone_plus();
                mapped.push(Pair(key_clone, new_value));
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(mapped);
            proof {
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < self.entries@.len()
                    implies (#[trigger] self.entries@[i]).0 == old_entries[i].0
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(i) == mapped@[i]);
                };
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(old_entries, self.entries@);
                // No duplicate keys (inherited from old).
                // Veracity: NEEDED assert (speed hint)
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {};
                };
                // Closure ensures postcondition.
                // Veracity: NEEDED assert
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
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == mapped@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_entries, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                        // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && spec_pred((#[trigger] old_view[si]).0, old_view[si].1)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED assert
                                assert(sources[j] == old_sources[j]);
                            } else {
                                // Veracity: NEEDED proof block
                                // Veracity: NEEDED assert
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                // Bridge: connect view-level entries to kept via spec_index.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_view.len()
                    && self.entries@[j] == old_view[sources[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_view, self.entries@, sources);
                lemma_subseq_dom_forward::<K::V, V::V>(old_view, self.entries@, sources);
                lemma_subseq_value_agrees::<K::V, V::V>(old_view, self.entries@, sources);
                // Completeness: every key satisfying spec_pred was kept.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    spec_entries_to_map(old_view).dom().contains(k)
                    && spec_pred(k, spec_entries_to_map(old_view)[k])
                    implies #[trigger] self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries.spec_index(j) == kept@[j]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries@[j].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
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
                        // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (exists|oj: int| 0 <= oj < other_view.len()
                                && (#[trigger] other_view[oj]).0 == (#[trigger] old_self_view[si]).0)
                            implies exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si
                        by {
                            // Veracity: NEEDED proof block
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_self_srcs.len() && old_self_srcs[j] == si;
                                // Veracity: NEEDED assert
                                assert(self_srcs[j] == old_self_srcs[j]);
                            } else {
                                // Veracity: NEEDED proof block
                                // Veracity: NEEDED assert
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
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        // Veracity: NEEDED assert
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
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // No duplicate keys (self_srcs monotone + old has no dups).
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < self_srcs.len() implies
                    0 <= #[trigger] self_srcs[j] < old_self_view.len()
                    && self.entries@[j].0 == old_self_view[self_srcs[j]].0
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_self_view, self.entries@, self_srcs);
                // Value preservation: combine.ensures postcondition.
                // Veracity: NEEDED assert
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
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s1 = self_srcs[idx];
                    let s2 = other_srcs[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, s1);
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, s2);
                    // Veracity: NEEDED assert (speed hint)
                    assert(combine.ensures(
                        (&old_self_raw[s1].1, &other_raw[s2].1), kept@[idx].1));
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
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
// Veracity: UNNEEDED proof block                         j <= other_len,
// Veracity: UNNEEDED proof block                         i < self.entries.spec_len(),
// Veracity: UNNEEDED proof block                         self.entries@ == old_self_view,
// Veracity: UNNEEDED proof block                         other_len as int == other.entries.spec_len(),
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
                    // Veracity: NEEDED proof block
                    proof {
                        reveal(obeys_view_eq);
                        other.entries.lemma_view_index(j as int);
                    }
                    if pair_i.0 == pair_j.0 {
                        match_idx = j;
                    }
                    // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block (speed hint)
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
                // Veracity: NEEDED proof block
                let mut found: bool = false;
                let ghost mut found_idx: int = -1int;
                let mut ii: usize = 0;
                while ii < self_len
                    invariant
                        // Veracity: NEEDED proof block (speed hint)
                        ii <= self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        self_len as int == self.entries.spec_len(),
                        found ==> (0 <= found_idx < old_self_view.len()
                            && old_self_view[found_idx].0 == key_view),
                        !found ==> forall|kk: int| 0 <= kk < ii as int ==>
                            (#[trigger] old_self_view[kk]).0 != key_view,
                        key_view == pair_j.0@,
                        // Veracity: NEEDED proof block
                        obeys_view_eq::<K>(),
                    decreases self.entries.spec_len() - ii,
                {
                    let pair_ii = self.entries.nth(ii);
                    proof {
                        // Veracity: NEEDED proof block
                        reveal(obeys_view_eq);
                        self.entries.lemma_view_index(ii as int);
                    }
                    if pair_j.0 == pair_ii.0 {
                        // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED assert
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
                            // Veracity: NEEDED proof block
                            } else {
                                // Had a witness k in old_phase2_sources.
                                let k = choose|k: int|
                                    0 <= k < old_phase2_sources.len()
                                    && (#[trigger] old_phase2_sources[k]) == oj;
                                // Veracity: NEEDED assert
                                assert(phase2_sources[k] == oj);
                            }
                        } else {
                            // oj == j: the current entry.
                            if !found {
                                // We just pushed j onto phase2_sources.
                                let k = phase2_sources.len() - 1;
                                // Veracity: NEEDED assert
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
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    self.entries.lemma_view_index(si);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, si);
                };
                // Every other key is in the output.
                // Veracity: NEEDED assert
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
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(
                            old_self_view, k);
                        let si = choose|si: int| 0 <= si < old_self_view.len()
                            && (#[trigger] old_self_view[si]).0 == k;
                        self.entries.lemma_view_index(si);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, si);
                    } else {
                        let kidx = choose|kidx: int|
                            0 <= kidx < phase2_sources.len()
                            && (#[trigger] phase2_sources[kidx]) == oj;
                        let out_idx = phase1_len + kidx;
                        self.entries.lemma_view_index(out_idx);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, out_idx);
                    }
                };
                // Reverse: every output key is in old self or other.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #![trigger spec_entries_to_map(old_self_view).dom().contains(k)]
                    #![trigger other@.dom().contains(k)]
                    self@.dom().contains(k)
                    implies spec_entries_to_map(old_self_view).dom().contains(k)
                        || other@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
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
                // Veracity: NEEDED assert
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(a) == kept@[a]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(b) == kept@[b]);
                        if a < phase1_len && b < phase1_len {
                            // Both from phase 1 — keys from old_self_view which has no dups.
                        } else if a < phase1_len && b >= phase1_len {
                            // a from phase 1 (key in old_self), b from phase 2 (key NOT in old_self).
                            let kidx_b = b - phase1_len;
                            let src_b = phase2_sources[kidx_b];
                            // Phase 2 entries are NOT in old_self; phase 1 key IS in old_self.
                            // Veracity: NEEDED assert (speed hint)
                            assert(!spec_entries_to_map(old_self_view).contains_key(
                                other.entries@[src_b].0));
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, a);
                        } else {
                            // Both from phase 2 — monotone sources + other has no dups.
                            let kidx_a = a - phase1_len;
                            let kidx_b = b - phase1_len;
                            let sa = phase2_sources[kidx_a];
                            let sb = phase2_sources[kidx_b];
                        }
                    };
                };
                // Value: key only in self (not in other) => value unchanged.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).contains_key(k)
                    && !other@.contains_key(k)
                    implies self@[k] == spec_entries_to_map(old_self_view)[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(si) == kept@[si]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, si);
                    // Contrapositive: match >= 0 would imply contains_key, contradiction.
                    if phase1_matches[si] >= 0 {
// Veracity: UNNEEDED assert                         assert(spec_entries_to_map(other.entries@).contains_key(
// Veracity: UNNEEDED assert                             old_self_view[si].0));
// Veracity: UNNEEDED assert                         assert(false);
                    }
                };
                // Value: key only in other (not in self) => value is other's.
                // Veracity: NEEDED assert
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
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(out_idx) == kept@[out_idx]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, out_idx);
                    lemma_entries_to_map_get::<K::V, V::V>(other.entries@, oj);
                };
                // Value: key in both => combined.
                // Veracity: NEEDED assert
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
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(si) == kept@[si]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, si);
                    // Contrapositive: match < 0 would imply !contains_key, contradiction.
                    if phase1_matches[si] < 0 {
// Veracity: UNNEEDED assert                         assert(!spec_entries_to_map(other.entries@).contains_key(
// Veracity: UNNEEDED assert                             old_self_view[si].0));
// Veracity: UNNEEDED assert                         assert(false);
                    }
                    let oj = phase1_matches[si];
                    lemma_entries_to_map_get::<K::V, V::V>(other.entries@, oj);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
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
            // Veracity: NEEDED proof block
            {
                let pair_i = self.entries.nth(i);
                let ghost key_view: K::V = old_self_view[i as int].0;
                let mut match_idx: usize = other_len;
                let mut j: usize = 0;
                while j < other_len
                    invariant
                        j <= other_len,
                        i < self.entries.spec_len(),
                        // Veracity: NEEDED proof block (speed hint)
                        self.entries@ == old_self_view,
                        other.entries@ == other_view,
                        other_len as int == other.entries.spec_len(),
                        match_idx <= other_len,
                        // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_entries_to_map_no_key::<K::V, V::V>(other_view, key_view);
                    }
                    let cloned = pair_i.clone_plus();
                    kept.push(cloned);
                    proof {
                        let ghost old_sources = sources;
                        // Veracity: NEEDED proof block
                        sources = sources.push(i as int);
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && !spec_entries_to_map(other_view).contains_key(
                                (#[trigger] old_self_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED assert
                                assert(sources[j] == old_sources[j]);
                            } else {
                                // Veracity: NEEDED assert
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
                // Bridge: connect view-level entries to kept via spec_index.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_self_view.len()
                    && self.entries@[j] == old_self_view[sources[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_self_view, self.entries@, sources);
                lemma_subseq_value_agrees::<K::V, V::V>(old_self_view, self.entries@, sources);
                // Forward: result keys are in old \ other.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                    implies spec_entries_to_map(old_self_view).dom().contains(k)
                        && !other@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = sources[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, s);
                };
                // Backward: old \ other keys are in result.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    spec_entries_to_map(old_self_view).dom().contains(k)
                    && !other@.dom().contains(k)
                    // Veracity: NEEDED proof block
                    implies #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                by {
                    // Veracity: NEEDED proof block
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries.spec_index(j) == kept@[j]);
                    // Veracity: NEEDED proof block
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                // Veracity: NEEDED proof block
                let pair = self.entries.nth(i);
                // Veracity: NEEDED proof block
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find_ref(&self, key: &K) -> (found: Option<&V>)
        {
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert
                        assert forall|j: int|
                            0 <= j < self.entries.seq@.len()
                            && (#[trigger] self.entries.seq@[j]).0@ == key@
                            implies j == i as int
                        by {
                            if j < i as int {
                                // From the view-level entries: entries@[j].0 != key@
                                // entries@[j].0 == entries.seq@[j].0@ (by View definition)
                                // Veracity: NEEDED assert
                                assert(self.entries@[j].0 != key@);
                            } else if j > i as int {
                                // From spec_keys_no_dups: distinct indices have distinct keys.
// Veracity: UNNEEDED assert                                 assert(self.entries@[i as int].0 != self.entries@[j].0);
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
            // Veracity: NEEDED proof block
            None
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key@
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                // Veracity: NEEDED assert
                                assert(src[j] == old_src[j]);
                            } else {
                                // Veracity: NEEDED assert
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
                // Bridge: connect view-level entries to kept via spec_index.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < src.len() implies
                    0 <= #[trigger] src[j] < old_view.len()
                    && self.entries@[j] == old_view[src[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_view, self.entries@, src);
                lemma_subseq_value_agrees::<K::V, V::V>(old_view, self.entries@, src);
                // Forward: result keys are in old \ {key}.
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
// Veracity: UNNEEDED assert                     assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                };
                // Backward: target keys (old \ {key}) are in result.
                // Veracity: NEEDED assert
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
// Veracity: UNNEEDED assert                     assert(self.entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
        // Veracity: NEEDED proof block
        {
            // Veracity: NEEDED proof block
            let ghost key_view: K::V = key@;
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            let ghost old_map = self@;
            // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                } else {
                    let cloned = pair.clone_plus();
                    all.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key_view
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                // Veracity: NEEDED assert
                                assert(src[j] == old_src[j]);
                            } else {
                                // Veracity: NEEDED assert
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
                }
                final_value = combine(&old_entry.1, &value);
            } else {
                final_value = value;
            }
            all.push(Pair(key, final_value));
            self.entries = ArraySeqStEphS::from_vec(all);
            proof {
                let last = (self.entries@.len() - 1) as int;
                // Veracity: NEEDED assert (speed hint)
                assert(self.entries.spec_index(last) == all@[last]);
                self.entries.lemma_view_index(last);
                lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, last);
                // Domain backward: old keys + key@ are in result.
                // Veracity: NEEDED assert
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
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // Domain forward: result keys are in old ∪ {key@}.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                    implies old_map.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    if idx < src.len() as int {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                };
                // No duplicate keys.
                // Veracity: NEEDED assert
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(a) == all@[a]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(b) == all@[b]);
                        if a < src.len() as int && b < src.len() as int {
                            // Veracity: NEEDED assert
                            assert(src[a] < src[b]);
                        } else if a < src.len() as int && b == last {
// Veracity: UNNEEDED assert                             assert(all@[a].0@ != key_view);
                        } else if a == last {
                        }
                    };
                };
                // Value preservation for non-key entries.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    k != key_view && #[trigger] old_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                };
                // Value for new key.
                lemma_entries_to_map_get::<K::V, V::V>(self.entries@, last);
                if match_index == n as usize {
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_view, key_view);
                }
                // Prove spec_stored_value for the inserted key.
                // Key is at position last in new entries.
                let ghost chosen_i = choose|i: int| 0 <= i < self.entries.seq@.len()
                    && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                if chosen_i != last {
                    // Veracity: NEEDED assert
                    assert(self.entries@[chosen_i].0 == key_view);
                }
                // Veracity: NEEDED assert (speed hint)
                assert(chosen_i == last);
                // Veracity: NEEDED assert (speed hint)
                assert(self.spec_stored_value(key_view) == self.entries.seq@[last].1);
                // Prove the existing-key spec_stored_value ensures.
                if match_index < n as usize {
                    let ghost old_chosen = choose|i: int| 0 <= i < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[i]).0@ == key_view;
                    if old_chosen != match_index as int {
                        // Veracity: NEEDED assert
                        assert(old_view[old_chosen].0 == key_view);
                    }
// Veracity: UNNEEDED assert                     assert(old_chosen == match_index as int);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn insert_wf<F: Fn(&V, &V) -> V>(&mut self, key: K, value: V, combine: F)
            where K: ClonePreservesView, V: ClonePreservesWf
        // Veracity: NEEDED proof block
        {
            // Veracity: NEEDED proof block
            let ghost key_view: K::V = key@;
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED proof block
                        self.entries.lemma_view_index(i as int);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, i as int);
                        let ghost k_at_i = old_view[i as int].0;
                        // spec_stored_value(k_at_i) == old_exec_seq[chosen].1
                        // where chosen is the unique index with key k_at_i.
                        let ghost chosen = choose|idx: int| 0 <= idx < old_exec_seq.len()
                            && (#[trigger] old_exec_seq[idx]).0@ == k_at_i;
                        // old_exec_seq[i].0@ == old_view[i].0 == k_at_i
                        if chosen != i as int {
                            // Veracity: NEEDED assert
                            assert(old_view[chosen].0 == k_at_i);
                        }
                        // Veracity: NEEDED assert (speed hint)
                        assert(chosen == i as int);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    all.push(cloned);
                    proof {
                        // Veracity: NEEDED proof block
                        let ghost old_src = src;
                        src = src.push(i as int);
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key_view
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                // Veracity: NEEDED assert
                                assert(src[j] == old_src[j]);
                            } else {
                                // Veracity: NEEDED assert
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
                    // Prove old_entry.1.spec_wf() for the combine-preserves-wf requires.
                    let ghost k_at_m = old_view[match_index as int].0;
                    let ghost chosen = choose|idx: int| 0 <= idx < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[idx]).0@ == k_at_m;
                    if chosen != match_index as int {
                        // Veracity: NEEDED assert
                        assert(old_view[chosen].0 == k_at_m);
                    }
                    // Veracity: NEEDED assert (speed hint)
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
                // Veracity: NEEDED assert (speed hint)
                assert(self.entries.spec_index(last) == all@[last]);
                self.entries.lemma_view_index(last);
                lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, last);
                // Domain backward: old keys + key@ are in result.
                // Veracity: NEEDED assert
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
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(j) == all@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
                // Domain forward: result keys are in old ∪ {key@}.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(self.entries@).dom().contains(k)
                    implies old_map.dom().contains(k) || k == key_view
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    if idx < src.len() as int {
                        let s = src[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                };
                // No duplicate keys.
                // Veracity: NEEDED assert
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(a) == all@[a]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(b) == all@[b]);
                        if a < src.len() as int && b < src.len() as int {
                            // Veracity: NEEDED assert
                            assert(src[a] < src[b]);
                        } else if a < src.len() as int && b == last {
                            // Veracity: NEEDED assert (speed hint)
                            assert(all@[a].0@ != key_view);
                        } else if a == last {
                        }
                    };
                };
                // Value preservation for non-key entries.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    k != key_view && #[trigger] old_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == all@[j]);
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                };
                // Value for new key.
                lemma_entries_to_map_get::<K::V, V::V>(self.entries@, last);
                if match_index == n as usize {
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_view, key_view);
                }
                // Prove spec_stored_value for the inserted key.
                let ghost chosen_i = choose|i: int| 0 <= i < self.entries.seq@.len()
                    && (#[trigger] self.entries.seq@[i]).0@ == key_view;
                if chosen_i != last {
                    // Veracity: NEEDED assert
                    assert(self.entries@[chosen_i].0 == key_view);
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries@[last].0 == key_view);
                }
// Veracity: UNNEEDED assert                 assert(chosen_i == last);
                // Veracity: NEEDED assert (speed hint)
                assert(self.spec_stored_value(key_view) == self.entries.seq@[last].1);
                // Prove the existing-key spec_stored_value ensures.
                if match_index < n as usize {
                    let ghost old_chosen = choose|i: int| 0 <= i < old_exec_seq.len()
                        && (#[trigger] old_exec_seq[i]).0@ == key_view;
                    if old_chosen != match_index as int {
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_view[old_chosen].0 == key_view);
                    }
// Veracity: UNNEEDED assert                     assert(old_chosen == match_index as int);
                }
                // Prove stored-value wf for all keys.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self.spec_stored_value(k).spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == all@[idx]);
                    // spec_stored_value(k) == self.entries.seq@[sv_idx].1
                    let ghost sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == k;
                    // sv_idx is the unique index with key k in new entries.
                    // idx also has key k. By no_dups, sv_idx == idx.
                    // Veracity: NEEDED assert
                    assert(self.entries@[sv_idx].0 == k);
                    if sv_idx != idx {
                        // Contradicts spec_keys_no_dups
                    }
// Veracity: UNNEEDED assert                     assert(sv_idx == idx);
// Veracity: UNNEEDED assert                     assert(self.spec_stored_value(k) == self.entries.seq@[idx].1);
                    // all@[idx].1.spec_wf() — from loop invariant or final_value wf.
                    if idx == last {
                        // final_value case: wf from combine or value.
                        if match_index < n as usize {
                            // combine result: combine.ensures((&old_v, &value), final_value)
                            // && old_v.spec_wf() && value.spec_wf() ==> final_value.spec_wf()
                            // Veracity: NEEDED assert (speed hint)
                            // Veracity: NEEDED proof block (speed hint)
                            assert(combine.ensures((&old_stored_at_key.1, &value), final_value));
                            assert(old_stored_at_key.1.spec_wf());
                        // Veracity: NEEDED proof block
                        }
                    } else {
                        // Non-key entry: wf from loop invariant.
                        assert(all@[idx].1.spec_wf());
                    }
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn delete_wf(&mut self, key: &K)
            where K: ClonePreservesView, V: ClonePreservesWf
        {
            let ghost old_view = self.entries@;
            let ghost old_exec_seq: Seq<Pair<K, V>> = self.entries.seq@;
            // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED proof block
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
                        if chosen != i as int {
                            // Veracity: NEEDED assert
                            assert(old_view[chosen].0 == k_at_i);
                        }
                        // Veracity: NEEDED assert (speed hint)
                        assert(chosen == i as int);
                    }
                    let kc = pair.0.clone_view();
                    let vc = pair.1.clone_wf();
                    let cloned = Pair(kc, vc);
                    kept.push(cloned);
                    proof {
                        let ghost old_src = src;
                        src = src.push(i as int);
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (#[trigger] old_view[si]).0 != key@
                            implies exists|j: int| 0 <= j < src.len() && src[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_src.len() && old_src[j] == si;
                                // Veracity: NEEDED assert
                                assert(src[j] == old_src[j]);
                            } else {
                                // Veracity: NEEDED assert
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
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                };
                // Veracity: NEEDED assert
                assert forall|k: K::V| target_map.dom().contains(k)
                    implies result_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                    let si = choose|si: int| 0 <= si < old_view.len()
                        && (#[trigger] old_view[si]).0 == k;
                    let j = choose|j: int| 0 <= j < src.len() && src[j] == si;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
                // Veracity: NEEDED assert
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|i: int, j: int|
                        0 <= i < self.entries@.len() && 0 <= j < self.entries@.len() && i != j
                        implies (#[trigger] self.entries@[i]).0 != (#[trigger] self.entries@[j]).0
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(i) == kept@[i]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(j) == kept@[j]);
                        let si = src[i];
                        let sj = src[j];
                    };
                };
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s = src[idx];
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, s);
                };
                // Prove stored-value wf for all remaining keys.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self.spec_stored_value(k).spec_wf()
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    // Veracity: NEEDED proof block
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let ghost sv_idx = choose|i: int| 0 <= i < self.entries.seq@.len()
                        && (#[trigger] self.entries.seq@[i]).0@ == k;
                    // Veracity: NEEDED assert
                    assert(self.entries@[sv_idx].0 == k);
                    if sv_idx != idx {}
                    // Veracity: NEEDED assert (speed hint)
                    assert(sv_idx == idx);
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.spec_stored_value(k) == self.entries.seq@[idx].1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries.seq@[idx] == kept@[idx]);
                    assert(kept@[idx].1.spec_wf());
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        #[verifier::loop_isolation(false)]
        // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && keys@.contains((#[trigger] old_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED assert
                                assert(sources[j] == old_sources[j]);
                            } else {
                                // Veracity: NEEDED assert
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                // Bridge: connect view-level entries to kept via spec_index.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_view.len()
                    && self.entries@[j] == old_view[sources[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_view, self.entries@, sources);
                lemma_subseq_value_agrees::<K::V, V::V>(old_view, self.entries@, sources);
                // Dom equivalence: result = old ∩ keys@.
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().intersect(keys@);
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                // Veracity: NEEDED proof block
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
// Veracity: UNNEEDED assert                         assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
            }
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
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
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && !keys@.contains((#[trigger] old_view[si]).0)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                // Veracity: NEEDED proof block
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED assert
                                assert(sources[j] == old_sources[j]);
                            } else {
                                // Veracity: NEEDED assert
                                assert(sources[sources.len() - 1] == i as int);
                            }
                        };
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqStEphS::from_vec(kept);
            proof {
                // Bridge: connect view-level entries to kept via spec_index.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < sources.len() implies
                    0 <= #[trigger] sources[j] < old_view.len()
                    // Veracity: NEEDED proof block
                    && self.entries@[j] == old_view[sources[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_view, self.entries@, sources);
                lemma_subseq_value_agrees::<K::V, V::V>(old_view, self.entries@, sources);
                // Dom equivalence: result = old \ keys@.
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_view).dom().difference(keys@);
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
// Veracity: UNNEEDED assert                         assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && !keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        // Veracity: NEEDED proof block
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn entries(&self) -> (entries: ArraySeqStEphS<Pair<K, V>>) {
            let entries = self.entries.clone();
            proof {
                // Veracity: NEEDED assert
                assert(Pair_feq_trigger::<K, V>());
                lemma_seq_map_cloned_view_eq(
                    self.entries.seq@,
                    entries.seq@,
                );
            }
            entries
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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


    //		Section 10. iterators


    /// Wrapping iterator over a TableStEph — delegates to the backing ArraySeqStEphIter.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEphIter<'a, K: StT + Ord, V: StT> {
        pub inner: ArraySeqStEphIter<'a, Pair<K, V>>,
    }

    impl<'a, K: StT + Ord, V: StT> View for TableStEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant_table<'a, K: StT + Ord, V: StT>(it: &TableStEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for TableStEphIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        fn next(&mut self) -> (next: Option<&'a Pair<K, V>>)
            ensures
                ({
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
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for for-loop support over TableStEphIter.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableStEphGhostIterator<'a, K: StT + Ord, V: StT> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a Pair<K, V>>,
    }

    impl<'a, K: StT + Ord, V: StT> View for TableStEphGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIteratorNew for TableStEphIter<'a, K, V> {
        type GhostIter = TableStEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> TableStEphGhostIterator<'a, K, V> {
            TableStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIterator for TableStEphGhostIterator<'a, K, V> {
        type ExecIter = TableStEphIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &TableStEphIter<'a, K, V>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<Pair<K, V>> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &TableStEphIter<'a, K, V>) -> TableStEphGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::IntoIterator for &'a TableStEph<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = TableStEphIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.entries.seq@,
                iter_invariant_table(&it),
        {
            self.iter()
        }
    }

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
            proof { accept(equal == (self@ == other@)); }
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

    impl<'a, K: StT + Ord + fmt::Debug, V: StT + fmt::Debug> fmt::Debug for TableStEphIter<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEphIter")
        }
    }

    impl<'a, K: StT + Ord, V: StT> fmt::Display for TableStEphIter<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEphIter")
        }
    }

    impl<'a, K: StT + Ord, V: StT> fmt::Debug for TableStEphGhostIterator<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEphGhostIterator")
        }
    }

    impl<'a, K: StT + Ord, V: StT> fmt::Display for TableStEphGhostIterator<'a, K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TableStEphGhostIterator")
        }
    }
}
