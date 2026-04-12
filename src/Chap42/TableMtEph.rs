//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 42 multi-threaded ephemeral table implementation using ArraySeqMtEph as backing store.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod TableMtEph {


    //		Section 2. imports

    use std::cmp::Ordering;
    use std::sync::Arc;

    use vstd::prelude::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    pub use crate::Chap42::TableSpecsAndLemmas::TableSpecsAndLemmas::*;
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{lemma_seq_map_cloned_view_eq, obeys_feq_clone, obeys_feq_full, obeys_feq_full_trigger, obeys_view_eq_trigger};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_fulls;
    use crate::vstdplus::accept::accept;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableMtEph<K: MtKey, V: MtVal> {
        pub entries: ArraySeqMtEphS<Pair<K, V>>,
    }

    pub type TableS<K, V> = TableMtEph<K, V>;

    //		Section 5. view impls


    impl<K: MtKey, V: MtVal> View for TableMtEph<K, V> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> {
            spec_entries_to_map(self.entries@)
        }
    }

    //		Section 8. traits


    /// Trait defining the Table ADT operations from Chapter 42.
    pub trait TableMtEphTrait<K: MtKey, V: MtVal>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_tablemteph_wf(&self) -> bool;

        /// Returns the concrete stored value for a given key.
        spec fn spec_stored_value(&self, key: K::V) -> V;

        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_tablemteph_wf()
            ensures count == self@.dom().len();
        /// - APAS Cost Spec 42.5: Work 1, Span 1
        /// - Alg Analysis: APAS (Ch42 ref): Work O(1), Span O(1) -- agrees with APAS.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- agrees with APAS.
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_tablemteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: K, value: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@ == Map::<K::V, V::V>::empty().insert(key@, value@), tree.spec_tablemteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: array-backed unordered table; sequential key extraction
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), domain.spec_arraysetsteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(|s| * W(f)), Span O(lg |s| + S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|·W(f)), Span O(lg |s| + S(f)) — parallel D&C tabulate via join
        fn tabulate<F: Fn(&K) -> V + Clone + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_tablemteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·W(f)), Span O(lg n + max W(f)) — parallel D&C map via join
        fn map<F: Fn(&V) -> V + Clone + Send + Sync + 'static>(&mut self, f: F)
            requires
                old(self).spec_tablemteph_wf(),
                forall|v: &V| f.requires((v,)),
                obeys_feq_clone::<K>(),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom() == old(self)@.dom(),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == old(self)@[k]
                        && f.ensures((&old_val,), result)
                        && self@[k] == result@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(Σ W(f(.))), Span O(lg |a| + max S(f(.)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + Σ W(f(k,v))), Span O(n + Σ W(f(k,v))) — ACCEPTED DIFFERENCE: array-backed unordered table; sequential loop; D&C no-dup proof pending
        fn filter<F: Fn(&K, &V) -> bool + Clone + Send + Sync + 'static>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        )
            requires
                old(self).spec_tablemteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom().subset_of(old(self)@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                forall|k: K::V| old(self)@.dom().contains(k) && spec_pred(k, old(self)@[k])
                    ==> #[trigger] self@.dom().contains(k);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: array-backed unordered table; nested linear scans on array
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            requires
                old(self).spec_tablemteph_wf(),
                other.spec_tablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom() =~= old(self)@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: array-backed unordered table; nested linear scans on array
        fn union<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, combine: F)
            requires
                old(self).spec_tablemteph_wf(),
                other.spec_tablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self.spec_tablemteph_wf(),
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: array-backed unordered table; nested linear scans on array
        fn difference(&mut self, other: &Self)
            requires
                old(self).spec_tablemteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — linear scan on flat array
        fn find(&self, key: &K) -> (found: Option<V>)
            requires self.spec_tablemteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(key@) && self@[key@] == v@,
                    None => !self@.contains_key(key@),
                };
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: array-backed unordered table; linear scan + copy
        fn delete(&mut self, key: &K)
            requires old(self).spec_tablemteph_wf(), obeys_view_eq::<K>()
            ensures self@ =~= old(self)@.remove(key@), self.spec_tablemteph_wf();
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: array-backed unordered table; linear scan + copy
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, key: K, value: V, combine: F)
            requires
                old(self).spec_tablemteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<K>(),
            ensures
                self.spec_tablemteph_wf(),
                self@.contains_key(key@),
                self@.dom() =~= old(self)@.dom().insert(key@),
                forall|k: K::V| k != key@ && #[trigger] old(self)@.contains_key(k) ==> self@[k] == old(self)@[k],
                !old(self)@.contains_key(key@) ==> self@[key@] == value@,
                old(self)@.contains_key(key@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[key@] && combine.ensures((&old_v, &value), r)
                    && self@[key@] == r@);
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: array-backed unordered table; nested linear scans on array
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires
                old(self).spec_tablemteph_wf(),
                keys@.finite(),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];
        /// - Alg Analysis: APAS (Ch42 CS 42.5): Work O(m * lg(1+n/m)), Span O(lg(n+m))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n·m), Span O(n·m) — ACCEPTED DIFFERENCE: array-backed unordered table; nested linear scans on array
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires
                old(self).spec_tablemteph_wf(),
                keys@.finite(),
            ensures
                self.spec_tablemteph_wf(),
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone of backing array.
        fn entries(&self) -> (entries: ArraySeqMtEphS<Pair<K, V>>)
            ensures spec_entries_to_map(entries@) == self@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter(&self) -> (it: TableMtEphIter<'_, K, V>)
            requires self.spec_tablemteph_wf()
            ensures it@.0 == 0, iter_invariant_tablemteph(&it);
    }

    //		Section 9. impls


//		7b. parallel D&C helpers

    /// Parallel D&C map for table entries: clone keys, apply f to values.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C map via join.
    fn map_table_dc<K: MtKey, V: MtVal, F: Fn(&V) -> V + Clone + Send + Sync + 'static>(
        entries: &ArraySeqMtEphS<Pair<K, V>>,
        f: &F,
    ) -> (mapped: ArraySeqMtEphS<Pair<K, V>>)
        requires
            forall|v: &V| f.requires((v,)),
            obeys_feq_clone::<Pair<K, V>>(),
            obeys_feq_clone::<K>(),
        ensures
            mapped.seq@.len() == entries.seq@.len(),
            forall|i: int| 0 <= i < entries.seq@.len() ==>
                (#[trigger] mapped.seq@[i]).0@ == entries.seq@[i].0@,
            forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < entries.seq@.len() ==>
                f.ensures((&entries.seq@[i].1,), mapped.seq@[i].1),
        decreases entries.seq@.len()
    {
        let len = entries.seq.len();
        if len == 0 {
            ArraySeqMtEphS { seq: Vec::new() }
        } else if len == 1 {
            let pair = entries.nth(0);
            let new_val = f(&pair.1);
            let key_clone = pair.0.clone_plus();
            let mut v = Vec::with_capacity(1);
            v.push(Pair(key_clone, new_val));
            let result = ArraySeqMtEphS::from_vec(v);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(result.spec_index(0) == v@[0]);
                // Veracity: NEEDED assert (speed hint)
                assert(v@[0].0@ == entries.seq@[0].0@);
            }
            result
        } else {
            let mid = len / 2;
            let left = entries.subseq_copy(0, mid);
            let right = entries.subseq_copy(mid, len - mid);
            let f1 = clone_fn(f);
            let f2 = clone_fn(f);

            let ghost e_raw = entries.seq@;
            let ghost l_raw = left.seq@;
            let ghost r_raw = right.seq@;
            let l_len = Ghost(mid as nat);
            let r_len = Ghost((len - mid) as nat);
// Veracity: NEEDED proof block

            proof {
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < l_raw.len() implies
                    #[trigger] l_raw[i] == e_raw[i]
                // Veracity: NEEDED assert
                by { assert(left.spec_index(i) == entries.spec_index(0 + i)); }
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < r_raw.len() implies
                    #[trigger] r_raw[j] == e_raw[mid as int + j]
                // Veracity: NEEDED assert
                by { assert(right.spec_index(j) == entries.spec_index(mid as int + j)); }
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < left.seq@.len() implies
                    #[trigger] f1.requires((&left.seq@[i].1,))
                // Veracity: NEEDED assert
                by { assert(l_raw[i] == e_raw[i]); }
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < right.seq@.len() implies
                    #[trigger] f2.requires((&right.seq@[i].1,))
                // Veracity: NEEDED assert
                by { assert(r_raw[i] == e_raw[mid as int + i]); }
            }

            let (lm, rm) = join(
                move || -> (result: ArraySeqMtEphS<Pair<K, V>>)
                    ensures
                        result.seq@.len() == l_len@,
                        forall|i: int| 0 <= i < l_len@ as int ==>
                            (#[trigger] result.seq@[i]).0@ == l_raw[i].0@,
                        forall|i: int| #![trigger result.seq@[i]] 0 <= i < l_len@ as int ==>
                            f1.ensures((&l_raw[i].1,), result.seq@[i].1),
                {
                    map_table_dc(&left, &f1)
                },
                move || -> (result: ArraySeqMtEphS<Pair<K, V>>)
                    ensures
                        result.seq@.len() == r_len@,
                        forall|i: int| 0 <= i < r_len@ as int ==>
                            (#[trigger] result.seq@[i]).0@ == r_raw[i].0@,
                        forall|i: int| #![trigger result.seq@[i]] 0 <= i < r_len@ as int ==>
                            f2.ensures((&r_raw[i].1,), result.seq@[i].1),
                {
                    map_table_dc(&right, &f2)
                },
            );
// Veracity: NEEDED proof block

            let appended = ArraySeqMtEphS::<Pair<K, V>>::append(&lm, &rm);
            proof {
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < e_raw.len() implies
                    (#[trigger] appended.seq@[i]).0@ == e_raw[i].0@
                by {
                    if i < mid as int {
                        // Trigger append left ensures.
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(i) == lm.spec_index(i));
                    } else {
                        let j = i - mid as int;
                        // Trigger append right ensures.
                        // Veracity: NEEDED assert (speed hint)
                        assert(rm.spec_index(j) == rm.spec_index(j));
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(lm.spec_len() as int + j)
                            == rm.spec_index(j));
                    }
                }
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger appended.seq@[i]]
                    0 <= i < e_raw.len() implies
                    f.ensures((&e_raw[i].1,), appended.seq@[i].1)
                by {
                    if i < mid as int {
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(i) == lm.spec_index(i));
                    } else {
                        let j = i - mid as int;
                        // Veracity: NEEDED assert (speed hint)
                        assert(rm.spec_index(j) == rm.spec_index(j));
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(lm.spec_len() as int + j)
                            == rm.spec_index(j));
                    }
                }
            }
            appended
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C tabulate via join.
    /// Parallel D&C tabulate for table entries: apply f to each key.
    fn tabulate_table_dc<K: MtKey, V: MtVal, F: Fn(&K) -> V + Clone + Send + Sync + 'static>(
        f: &F,
        key_seq: &ArraySeqStEphS<K>,
    ) -> (entries: ArraySeqMtEphS<Pair<K, V>>)
        requires
            forall|k: &K| f.requires((k,)),
            obeys_feq_full::<K>(),
            obeys_feq_clone::<Pair<K, V>>(),
        ensures
            entries.seq@.len() == key_seq.seq@.len(),
            forall|j: int| 0 <= j < key_seq.seq@.len() ==>
                (#[trigger] entries.seq@[j]).0@ == key_seq.seq@[j]@,
            forall|j: int| #![trigger key_seq.seq@[j]] 0 <= j < key_seq.seq@.len() ==>
                f.ensures((&key_seq.seq@[j],), entries.seq@[j].1),
        decreases key_seq.seq@.len()
    {
        let len = key_seq.seq.len();
        if len == 0 {
            ArraySeqMtEphS { seq: Vec::new() }
        } else if len == 1 {
            let key = key_seq.nth(0);
            let value = f(key);
            let key_clone = key.clone_plus();
            // Veracity: NEEDED proof block
            let mut v = Vec::with_capacity(1);
            v.push(Pair(key_clone, value));
            let result = ArraySeqMtEphS::from_vec(v);
            proof {
                // Veracity: NEEDED assert
                assert(result.spec_index(0) == v@[0]);
                // Veracity: NEEDED assert (speed hint)
                assert(v@[0].0@ == key_seq.seq@[0]@);
            }
            result
        } else {
            let mid = len / 2;
            let left_keys = key_seq.subseq_copy(0, mid);
            let right_keys = key_seq.subseq_copy(mid, len - mid);
            let f1 = clone_fn(f);
            let f2 = clone_fn(f);

            let ghost k_raw = key_seq.seq@;
            let ghost lk_raw = left_keys.seq@;
            // Veracity: NEEDED proof block
            let ghost rk_raw = right_keys.seq@;
            let l_len = Ghost(mid as nat);
            let r_len = Ghost((len - mid) as nat);

            proof {
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < lk_raw.len() implies
                    #[trigger] lk_raw[i] == k_raw[i]
                // Veracity: NEEDED assert
                by { assert(left_keys.spec_index(i) == key_seq.spec_index(0 + i)); }
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < rk_raw.len() implies
                    #[trigger] rk_raw[j] == k_raw[mid as int + j]
                // Veracity: NEEDED assert
                by { assert(right_keys.spec_index(j) == key_seq.spec_index(mid as int + j)); }
                // Veracity: NEEDED assert (speed hint)
                assert forall|k: &K| #[trigger] f1.requires((k,)) by {}
                // Veracity: NEEDED assert (speed hint)
                assert forall|k: &K| #[trigger] f2.requires((k,)) by {}
            }

            let (lm, rm) = join(
                move || -> (result: ArraySeqMtEphS<Pair<K, V>>)
                    ensures
                        result.seq@.len() == l_len@,
                        forall|j: int| 0 <= j < l_len@ as int ==>
                            (#[trigger] result.seq@[j]).0@ == lk_raw[j]@,
                        forall|j: int| #![trigger lk_raw[j]] 0 <= j < l_len@ as int ==>
                            f1.ensures((&lk_raw[j],), result.seq@[j].1),
                {
                    tabulate_table_dc(&f1, &left_keys)
                },
                move || -> (result: ArraySeqMtEphS<Pair<K, V>>)
                    ensures
                        result.seq@.len() == r_len@,
                        forall|j: int| 0 <= j < r_len@ as int ==>
                            (#[trigger] result.seq@[j]).0@ == rk_raw[j]@,
                        forall|j: int| #![trigger rk_raw[j]] 0 <= j < r_len@ as int ==>
                            f2.ensures((&rk_raw[j],), result.seq@[j].1),
                {
                    // Veracity: NEEDED proof block
                    tabulate_table_dc(&f2, &right_keys)
                },
            );

            let appended = ArraySeqMtEphS::<Pair<K, V>>::append(&lm, &rm);
            proof {
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < k_raw.len() implies
                    (#[trigger] appended.seq@[j]).0@ == k_raw[j]@
                by {
                    if j < mid as int {
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(j) == lm.spec_index(j));
                    } else {
                        let jj = j - mid as int;
                        // Veracity: NEEDED assert (speed hint)
                        assert(rm.spec_index(jj) == rm.spec_index(jj));
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(lm.spec_len() as int + jj)
                            == rm.spec_index(jj));
                    }
                }
                // Veracity: NEEDED assert
                assert forall|j: int| #![trigger k_raw[j]]
                    0 <= j < k_raw.len() implies
                    f.ensures((&k_raw[j],), appended.seq@[j].1)
                by {
                    if j < mid as int {
                        // Veracity: NEEDED assert
                        assert(appended.spec_index(j) == lm.spec_index(j));
                        // Veracity: NEEDED assert (speed hint)
                        assert(lk_raw[j] == k_raw[j]);
                    } else {
                        let jj = j - mid as int;
                        // Veracity: NEEDED assert (speed hint)
                        assert(rm.spec_index(jj) == rm.spec_index(jj));
                        // Veracity: NEEDED assert (speed hint)
                        assert(appended.spec_index(lm.spec_len() as int + jj)
                            == rm.spec_index(jj));
                        // Veracity: NEEDED assert (speed hint)
                        assert(rk_raw[jj] == k_raw[mid as int + jj]);
                    }
                }
            }
            appended
        }
    }


    impl<K: MtKey, V: MtVal> TableMtEphTrait<K, V> for TableMtEph<K, V> {
        open spec fn spec_tablemteph_wf(&self) -> bool {
            spec_keys_no_dups(self.entries@)
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
        }

        open spec fn spec_stored_value(&self, key: K::V) -> V {
            let i = choose|i: int| 0 <= i < self.entries.seq@.len()
                && (#[trigger] self.entries.seq@[i]).0@ == key;
            // Veracity: NEEDED proof block
            self.entries.seq@[i].1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        {
            proof {
                lemma_entries_to_map_len::<K::V, V::V>(self.entries@);
            }
            self.entries.length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<K>());
           // Veracity: NEEDED assert
           assert(obeys_feq_full_trigger::<V>());
           // Veracity: NEEDED assert
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            let entries = ArraySeqMtEphS::empty();
            // Veracity: NEEDED assert (speed hint)
            assert(entries@ =~= Seq::<(K::V, V::V)>::empty());
            TableMtEph { entries }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(key: K, value: V) -> (tree: Self)
        {
                      // Veracity: NEEDED proof block (speed hint)
                      // Veracity: NEEDED assert
                      assert(obeys_feq_full_trigger::<K>());
           // Veracity: NEEDED proof block
           // Veracity: NEEDED assert
           assert(obeys_feq_full_trigger::<V>());
           // Veracity: NEEDED assert (speed hint)
           assert(obeys_feq_full_trigger::<Pair<K, V>>());
            // Veracity: NEEDED assert (speed hint)
            proof { assert(Pair_feq_trigger::<K, V>()); }
            let entries = ArraySeqMtEphS::singleton(Pair(key, value));
            let tree = TableMtEph { entries };
            proof {
                let e = tree.entries@;
                // Veracity: NEEDED assert (speed hint)
                assert(e.len() == 1);
// Veracity: UNNEEDED assert                 assert(e[0] == (key@, value@));
                // Veracity: NEEDED assert (speed hint)
                assert(e.last() == e[e.len() - 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(e.drop_last().len() == 0);
// Veracity: UNNEEDED assert                 assert(spec_entries_to_map::<K::V, V::V>(e.drop_last()) =~= Map::<K::V, V::V>::empty());
                // Veracity: NEEDED assert (speed hint)
                assert(spec_entries_to_map(e) =~=
                    spec_entries_to_map::<K::V, V::V>(e.drop_last()).insert(e.last().0, e.last().1));
            }
            // Veracity: NEEDED proof block
            tree
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let mut keys = ArraySetStEph::empty();
            let mut i: usize = 0;
            // Veracity: NEEDED assert (speed hint)
            proof { assert(obeys_feq_full_trigger::<K>()); }
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    keys.spec_arraysetsteph_wf(),
                    keys@.finite(),
                    forall|j: int| 0 <= j < i as int
                        ==> keys@.contains((#[trigger] self.entries@[j]).0),
                    // Veracity: NEEDED proof block
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
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|j: int| 0 <= j < i as int + 1
                        implies keys@.contains((#[trigger] self.entries@[j]).0)
                    by {
                        if j < i as int {
                            // Veracity: NEEDED assert (speed hint)
                            assert(old_keys.contains(self.entries@[j].0));
                        }
                    };
                    // Veracity: NEEDED assert
                    assert forall|k: K::V| keys@.contains(k)
                        implies exists|j: int| 0 <= j < i as int + 1
                            && (#[trigger] self.entries@[j]).0 == k
                    by {
                        if old_keys.contains(k) {
                            let j = choose|j: int| 0 <= j < i as int
                                // Veracity: NEEDED proof block
                                && (#[trigger] self.entries@[j]).0 == k;
                            // Veracity: NEEDED assert (speed hint)
                            assert(j < i as int + 1);
                        } else {
                            // Veracity: NEEDED assert
                            assert(self.entries@[i as int].0 == k);
                        }
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
        // Veracity: NEEDED proof block
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        fn tabulate<F: Fn(&K) -> V + Clone + Send + Sync + 'static>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
        {
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<K>());
            // Veracity: NEEDED assert
            assert(obeys_feq_full_trigger::<V>());
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            // Veracity: NEEDED assert (speed hint)
            proof { assert(Pair_feq_trigger::<K, V>()); }
            let key_seq = keys.to_seq();
            let seq = tabulate_table_dc(&f, &key_seq);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(seq@);
                // Each entry key matches the corresponding key_seq element.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < seq@.len()
                    implies (#[trigger] seq@[j]).0 == key_seq@[j]
                by {};
                // No duplicate keys since key_seq has no duplicates.
                // Veracity: NEEDED assert (speed hint)
                assert(spec_keys_no_dups(seq@)) by {
                    // Veracity: NEEDED assert
                    assert forall|i: int, j: int|
                        0 <= i < j < seq@.len()
                        implies (#[trigger] seq@[i]).0 != (#[trigger] seq@[j]).0
                    by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(seq@[i].0 == key_seq@[i]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(seq@[j].0 == key_seq@[j]);
                    };
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
                        // Veracity: NEEDED assert (speed hint)
                        assert(key_seq@[j] == k);
                    }
                    if keys@.contains(k) {
                        let j = choose|j: int| 0 <= j < key_seq@.len()
                            && key_seq@[j] == k;
                        // Veracity: NEEDED assert (speed hint)
                        assert(seq@[j].0 == k);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(seq@, j);
                    }
                };
                // Closure ensures: for each key k, witness the key_arg and result.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] spec_entries_to_map(seq@).contains_key(k) implies
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && spec_entries_to_map(seq@)[k] == result@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(seq@, k);
                    let j = choose|j: int| 0 <= j < seq@.len()
                        && (#[trigger] seq@[j]).0 == k;
                    // Veracity: NEEDED assert (speed hint)
                    assert(seq.seq@[j].0@ == key_seq.seq@[j]@);
// Veracity: UNNEEDED assert                     assert(key_seq@[j] == k);
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                     assert(f.ensures((&key_seq.seq@[j],), seq.seq@[j].1));
                    lemma_entries_to_map_get::<K::V, V::V>(seq@, j);
                };
            }
            TableMtEph { entries: seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        fn map<F: Fn(&V) -> V + Clone + Send + Sync + 'static>(&mut self, f: F)
        {
            let ghost old_entries = self.entries@;
            let ghost old_raw = self.entries.seq@;
            let mapped = map_table_dc(&self.entries, &f);
            self.entries = mapped;
            proof {
                // map_table_dc ensures: same keys at each position, f.ensures for values.
                // Veracity: NEEDED assert
                assert forall|i: int| 0 <= i < self.entries@.len()
                    implies (#[trigger] self.entries@[i]).0 == old_entries[i].0
                by {};
                lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(old_entries, self.entries@);
                // Veracity: NEEDED assert (speed hint)
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
// Veracity: UNNEEDED assert                         assert(self.entries@[a].0 == old_entries[a].0);
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries@[b].0 == old_entries[b].0);
                        // Veracity: NEEDED assert (speed hint)
                        assert(old_entries[a].0 != old_entries[b].0);
                    };
                };
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k) implies
                    (exists|old_val: V, result: V|
                        old_val@ == spec_entries_to_map(old_entries)[k]
                        && f.ensures((&old_val,), result)
                        && self@[k] == result@)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let j = choose|j: int| 0 <= j < self.entries@.len()
                        && (#[trigger] self.entries@[j]).0 == k;
                    // Veracity: NEEDED assert (speed hint)
                    assert(f.ensures((&old_raw[j].1,), self.entries.seq@[j].1));
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, j);
                    lemma_entries_to_map_get::<K::V, V::V>(old_entries, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn filter<F: Fn(&K, &V) -> bool + Clone + Send + Sync + 'static>(
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
                        0 <= #[trigger] sources[j] < old_view.len()
                        // Veracity: NEEDED proof block
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int|
                        0 <= j1 < j2 < sources.len() ==> #[trigger] sources[j1] < #[trigger] sources[j2],
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
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED proof block
                        assert forall|si: int| 0 <= si < i as int + 1
                            && spec_pred((#[trigger] old_view[si]).0, old_view[si].1)
                            implies exists|j: int| 0 <= j < sources.len() && sources[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert (speed hint)
                        assert(!spec_pred(old_view[i as int].0, old_view[i as int].1));
                    }
                }
                i += 1;
            }
            self.entries = ArraySeqMtEphS::from_vec(kept);
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
                    // Veracity: NEEDED proof block
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
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersection<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
        {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<K>());
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<Pair<K, V>>());
// Veracity: UNNEEDED assert                 assert(obeys_view_eq_trigger::<K>());
            }
            let ghost old_self_view = self.entries@;
            let ghost old_self_raw = self.entries.seq@;
            let ghost other_view = other.entries@;
            let ghost other_raw = other.entries.seq@;
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            let ghost mut self_srcs: Seq<int> = Seq::empty();
            let ghost mut other_srcs: Seq<int> = Seq::empty();
            let mut i: usize = 0;
            while i < self.entries.length()
                invariant
                    i <= self.entries.spec_len(),
                    self.entries@ == old_self_view,
                    self.entries.seq@ == old_self_raw,
                    other.entries@ == other_view,
                    other.entries.seq@ == other_raw,
                    self_srcs.len() == kept@.len(),
                    other_srcs.len() == kept@.len(),
                    forall|k: int| 0 <= k < self_srcs.len() ==>
                        0 <= #[trigger] self_srcs[k] < old_self_view.len()
                        && old_self_view[self_srcs[k]].0 == kept@[k].0@,
                    forall|k: int| 0 <= k < other_srcs.len() ==>
                        0 <= #[trigger] other_srcs[k] < other_view.len()
                        && other_view[other_srcs[k]].0 == kept@[k].0@,
                    // Combine ensures for each kept entry.
                    forall|k: int| #![trigger kept@[k]] 0 <= k < kept@.len() ==>
                        combine.ensures((&old_self_raw[self_srcs[k]].1,
                            &other_raw[other_srcs[k]].1), kept@[k].1),
                    // Strict ordering of self_srcs.
                    forall|j1: int, j2: int|
                        0 <= j1 < j2 < self_srcs.len()
                        ==> #[trigger] self_srcs[j1] < #[trigger] self_srcs[j2],
                    forall|j: int| 0 <= j < self_srcs.len() ==> #[trigger] self_srcs[j] < i as int,
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    obeys_feq_clone::<K>(),
                    obeys_view_eq::<K>(),
                    forall|si: int| 0 <= si < i as int
                        && (exists|oj: int| 0 <= oj < other_view.len()
                            && #[trigger] other_view[oj].0 == (#[trigger] old_self_view[si]).0)
                        ==> exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si,
                    forall|si: int| 0 <= si < i as int
                        && !(exists|oj: int| 0 <= oj < other_view.len()
                            && #[trigger] other_view[oj].0 == (#[trigger] old_self_view[si]).0)
                        ==> !spec_entries_to_map(other_view).contains_key(old_self_view[si].0),
                decreases self.entries.spec_len() - i,
            {
                let pair_i = self.entries.nth(i);
                // Veracity: NEEDED proof block
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
                            // Veracity: NEEDED proof block
                            ==> (#[trigger] other_view[jj]).0 != key_view,
                        key_view == pair_i.0@,
                        obeys_view_eq::<K>(),
                    decreases other.entries.spec_len() - j,
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
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
                        // Veracity: NEEDED proof block
                        let ghost old_self_srcs = self_srcs;
                        self_srcs = self_srcs.push(i as int);
                        other_srcs = other_srcs.push(found_idx as int);
                        // Veracity: NEEDED assert
                        assert forall|si: int| 0 <= si < i as int + 1
                            && (exists|oj: int| 0 <= oj < other_view.len()
                                // Veracity: NEEDED proof block
                                && #[trigger] other_view[oj].0 == (#[trigger] old_self_view[si]).0)
                            implies exists|j: int| 0 <= j < self_srcs.len() && self_srcs[j] == si
                        by {
                            if si < i as int {
                                let j = choose|j: int|
                                    0 <= j < old_self_srcs.len() && old_self_srcs[j] == si;
                                // Veracity: NEEDED assert
                                assert(self_srcs[j] == old_self_srcs[j]);
                            } else {
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
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
                // No duplicate keys in result.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < self_srcs.len() implies
                    0 <= #[trigger] self_srcs[j] < old_self_view.len()
                    && self.entries@[j].0 == old_self_view[self_srcs[j]].0
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                lemma_subseq_no_dups::<K::V, V::V>(old_self_view, self.entries@, self_srcs);
                // Combine ensures: for each key k in result, witness v1, v2, r.
                // Veracity: NEEDED assert
                assert forall|k: K::V| #[trigger] self@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == spec_entries_to_map(old_self_view)[k]
                        && v2@ == spec_entries_to_map(other_view)[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@)
                // Veracity: NEEDED proof block
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(idx) == kept@[idx]);
                    let s1 = self_srcs[idx];
                    let s2 = other_srcs[idx];
                    // The combine was applied with self_raw[s1].1 and other_raw[s2].1.
                    // Veracity: NEEDED assert (speed hint)
                    assert(combine.ensures(
                        (&old_self_raw[s1].1, &other_raw[s2].1), kept@[idx].1));
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, idx);
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, s1);
                    lemma_entries_to_map_get::<K::V, V::V>(other_view, s2);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        #[verifier::loop_isolation(false)]
        fn union<F: Fn(&V, &V) -> V + Send + Sync>(&mut self, other: &Self, combine: F)
        {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_view_eq_trigger::<K>());
            }
            let ghost old_self_view = self.entries@;
            let ghost old_self_raw = self.entries.seq@;
            let ghost other_raw = other.entries.seq@;
            let other_len = other.entries.length();
            let self_len = self.entries.length();
            let mut kept: Vec<Pair<K, V>> = Vec::new();
            // Phase 1 ghost: track which other index each self entry was combined with.
            let ghost mut combine_idx: Seq<int> = Seq::empty();
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
                    combine_idx.len() == i as int,
                    forall|k: int| 0 <= k < i as int ==>
                        (#[trigger] kept@[k]).0@ == old_self_view[k].0,
                    forall|k: int| 0 <= k < i as int
                        && !spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[k].0)
                        ==> (#[trigger] kept@[k]).1@ == old_self_view[k].1,
                    // Combine tracking: for overlap entries, combine was applied.
                    forall|k: int| 0 <= k < i as int
                        && spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[k].0)
                        ==> 0 <= #[trigger] combine_idx[k] < other.entries@.len()
                            && other.entries@[combine_idx[k]].0 == old_self_view[k].0
                            && combine.ensures((&old_self_raw[k].1,
                                &other_raw[combine_idx[k]].1), kept@[k].1),
                    forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                    obeys_feq_clone::<K>(),
                    obeys_view_eq::<K>(),
                // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED proof block
                        other_len as int == other.entries.spec_len(),
                        match_idx <= other_len,
                        match_idx < other_len ==>
                            other.entries@[match_idx as int].0 == key_view,
                        match_idx == other_len ==>
                            forall|jj: int| 0 <= jj < j as int ==>
                                (#[trigger] other.entries@[jj]).0 != key_view,
                        // Veracity: NEEDED proof block
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
                if match_idx < other_len {
                    let pair_j = other.entries.nth(match_idx);
                    let key_clone = pair_i.0.clone_plus();
                    let combined_value = combine(&pair_i.1, &pair_j.1);
                    kept.push(Pair(key_clone, combined_value));
                    proof {
                        combine_idx = combine_idx.push(match_idx as int);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            other.entries@, match_idx as int);
                    }
                } else {
                    let cloned = pair_i.clone_plus();
                    kept.push(cloned);
                    proof {
                        combine_idx = combine_idx.push(-1int);
                        lemma_entries_to_map_no_key::<K::V, V::V>(
                            other.entries@, key_view);
                    }
                }
                i += 1;
            }
            let ghost phase1_len: int = kept@.len() as int;
            let ghost phase1_kept = kept@;
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
                    forall|k: int| 0 <= k < phase1_len
                        && !spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[k].0)
                        ==> (#[trigger] kept@[k]).1@ == old_self_view[k].1,
                    // Phase 1 entries unchanged by Phase 2 appends.
                    forall|k: int| 0 <= k < phase1_len ==> kept@[k] == #[trigger] phase1_kept[k],
                    // Phase 1 combine ensures (carried through Phase 2).
                    forall|k: int| 0 <= k < phase1_len
                        && spec_entries_to_map(other.entries@).contains_key(
                            old_self_view[k].0)
                        ==> combine.ensures((&old_self_raw[k].1,
                            &other_raw[#[trigger] combine_idx[k]].1), phase1_kept[k].1),
                    forall|k: int| 0 <= k < phase2_sources.len() ==>
                        0 <= #[trigger] phase2_sources[k] < other.entries@.len()
                        && other.entries@[phase2_sources[k]].0
                            == kept@[(phase1_len + k) as int].0@
                        && other.entries@[phase2_sources[k]].1
                            == kept@[(phase1_len + k) as int].1@
                        && !spec_entries_to_map(old_self_view).contains_key(
                            other.entries@[phase2_sources[k]].0),
                    forall|oj: int| 0 <= oj < j as int ==>
                        spec_entries_to_map(old_self_view).contains_key(
                            (#[trigger] other.entries@[oj]).0)
                        || (exists|k: int| 0 <= k < phase2_sources.len()
                            && (#[trigger] phase2_sources[k]) == oj),
                    forall|j1: int, j2: int|
                        // Veracity: NEEDED proof block
                        0 <= j1 < j2 < phase2_sources.len()
                        ==> #[trigger] phase2_sources[j1] < #[trigger] phase2_sources[j2],
                    forall|k: int| 0 <= k < phase2_sources.len()
                        ==> #[trigger] phase2_sources[k] < j as int,
                    // Veracity: NEEDED proof block (speed hint)
                    obeys_view_eq::<K>(),
                decreases other_len - j,
            {
                let pair_j = other.entries.nth(j);
                let ghost key_view: K::V = other.entries@[j as int].0;
                // Scan self for matching key.
                let mut found: bool = false;
                let ghost mut found_idx: int = -1int;
                // Veracity: NEEDED proof block
                let mut ii: usize = 0;
                while ii < self_len
                    invariant
                        ii <= self.entries.spec_len(),
                        self.entries@ == old_self_view,
                        // Veracity: NEEDED proof block
                        self_len as int == self.entries.spec_len(),
                        found ==> (0 <= found_idx < old_self_view.len()
                            && old_self_view[found_idx].0 == key_view),
                        !found ==> forall|kk: int| 0 <= kk < ii as int ==>
                            // Veracity: NEEDED proof block
                            (#[trigger] old_self_view[kk]).0 != key_view,
                        key_view == pair_j.0@,
                        obeys_view_eq::<K>(),
                    decreases self.entries.spec_len() - ii,
                {
                    let pair_ii = self.entries.nth(ii);
                    proof {
                        reveal(obeys_view_eq);
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
                // Veracity: NEEDED proof block
                }
                proof {
                    // Veracity: NEEDED assert
                    assert forall|oj: int| 0 <= oj < j + 1 implies
                        spec_entries_to_map(old_self_view).contains_key(
                            (#[trigger] other.entries@[oj]).0)
                        || (exists|k: int| 0 <= k < phase2_sources.len()
                            && (#[trigger] phase2_sources[k]) == oj)
                    by {
                        if oj < j as int {
                            if spec_entries_to_map(old_self_view).contains_key(
                                other.entries@[oj].0)
                            {
                            } else {
                                let k = choose|k: int|
                                    0 <= k < old_phase2_sources.len()
                                    && (#[trigger] old_phase2_sources[k]) == oj;
                                // Veracity: NEEDED assert
                                assert(phase2_sources[k] == oj);
                            }
                        } else {
                            if !found {
                                let k = phase2_sources.len() - 1;
                                // Veracity: NEEDED assert
                                assert(phase2_sources[k] == oj);
                            }
                        }
                    };
                }
                j += 1;
            }
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                // Every old self key is in the output (Phase 1).
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    spec_entries_to_map(old_self_view).dom().contains(k)
                    implies self@.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= si && si < kept@.len());
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(si) == kept@[si]);
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
                        // Veracity: NEEDED assert (speed hint)
                        assert(0 <= si && si < kept@.len());
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(si) == kept@[si]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, si);
                    } else {
                        let kidx = choose|kidx: int|
                            0 <= kidx < phase2_sources.len()
                            && (#[trigger] phase2_sources[kidx]) == oj;
                        let out_idx = phase1_len + kidx;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(out_idx) == kept@[out_idx]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(
                            self.entries@, out_idx);
                    }
                };
                // Reverse: every output key is in old self or other.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
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
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, idx);
                    } else {
                        let kidx = idx - phase1_len;
                        let src = phase2_sources[kidx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(other.entries@, src);
                    }
                };
                // Value preservation: self-only keys keep self value.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).dom().contains(k)
                    && !other@.dom().contains(k)
                    implies self@[k] == spec_entries_to_map(old_self_view)[k]
                by {
                    // Phase 2 entries are other-only, so none has key k.
                    // Veracity: NEEDED assert
                    assert forall|idx: int| phase1_len <= idx < self.entries@.len()
                        implies (#[trigger] self.entries@[idx]).0 != k
                    by {
                        let kidx = idx - phase1_len;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let src = phase2_sources[kidx];
                        if self.entries@[idx].0 == k {
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                other.entries@, src);
                        }
                    };
                    // self@[k] comes from Phase 1 prefix (ignore suffix).
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_entries_to_map(self.entries@).contains_key(k));
                    lemma_entries_to_map_ignore_suffix::<K::V, V::V>(
                        self.entries@, phase1_len, k);
                    let ghost prefix = self.entries@.subrange(0, phase1_len);
                    // Phase 1 prefix has same keys as old_self_view at each position.
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < prefix.len()
                        implies (#[trigger] prefix[i]).0 == (#[trigger] old_self_view[i]).0
                    by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(prefix[i] == self.entries@[i]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(i) == kept@[i]);
                    };
                    // For self-only key k, values at key-k positions match old_self_view.
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < prefix.len() && (#[trigger] prefix[i]).0 == k
                        implies prefix[i].1 == old_self_view[i].1
                    by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(prefix[i] == self.entries@[i]);
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(i) == kept@[i]);
                    };
                    // Maps agree at k since keys match and key-k values match.
                    lemma_entries_to_map_agree_on_key::<K::V, V::V>(
                        prefix, old_self_view, k);
                };
                // Value preservation: other-only keys keep other value.
                let ghost suffix = self.entries@.subrange(
                    phase1_len, self.entries@.len() as int);
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] other@.dom().contains(k)
                    && !spec_entries_to_map(old_self_view).dom().contains(k)
                    implies self@[k] == other@[k]
                by {
                    // Phase 1 has no entries with key k.
                    // Veracity: NEEDED assert
                    assert forall|idx: int| 0 <= idx < phase1_len
                        implies (#[trigger] self.entries@[idx]).0 != k
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(kept@[idx].0@ == old_self_view[idx].0);
                        if old_self_view[idx].0 == k {
                            lemma_entries_to_map_contains_key::<K::V, V::V>(
                                old_self_view, idx);
                        }
                    };
                    // suffix == Phase 2 portion of self.entries@.
// Veracity: UNNEEDED assert                     assert(suffix.len() == phase2_sources.len());
                    // suffix[j] == other.entries@[phase2_sources[j]] for all j.
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < suffix.len()
                        implies #[trigger] suffix[j] == other.entries@[phase2_sources[j]]
                    by {
                        let out_idx = phase1_len + j;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(out_idx) == kept@[out_idx]);
// Veracity: UNNEEDED assert                         assert(suffix[j] == self.entries@[out_idx]);
                    };
                    // All other entries with key k are in phase2_sources.
                    // Veracity: NEEDED assert
                    assert forall|oi: int| 0 <= oi < other.entries@.len()
                        && (#[trigger] other.entries@[oi]).0 == k
                        implies exists|j: int| 0 <= j < phase2_sources.len()
                            && phase2_sources[j] == oi
                    by {};
                    // suffix contains key k.
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(other.entries@, k);
                    let oi = choose|oi: int| 0 <= oi < other.entries@.len()
                        && (#[trigger] other.entries@[oi]).0 == k;
                    let ji = choose|ji: int| 0 <= ji < phase2_sources.len()
                        && phase2_sources[ji] == oi;
                    // Veracity: NEEDED assert (speed hint)
                    assert(suffix[ji].0 == k);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(suffix, ji);
                    // Apply subseq_value: suffix is a subsequence of other.entries@.
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        other.entries@, suffix, phase2_sources, k);
                    // spec_entries_to_map(suffix)[k] == other@[k].
                    // Now connect: self@[k] == spec_entries_to_map(suffix)[k].
                    lemma_entries_to_map_skip_prefix::<K::V, V::V>(
                        self.entries@, phase1_len, k);
                    // spec_entries_to_map(self.entries@.subrange(phase1_len, ...))[k]
                    //   == spec_entries_to_map(self.entries@)[k] == self@[k].
                };
                // No duplicate keys in result.
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
// Veracity: UNNEEDED assert                             assert(old_self_view[a].0 != old_self_view[b].0);
                        } else if a < phase1_len && b >= phase1_len {
                            let kidx = b - phase1_len;
                            let src = phase2_sources[kidx];
                            // Phase 2 entry NOT in old_self; phase 1 key IS in old_self.
                            // Veracity: NEEDED assert (speed hint)
                            assert(!spec_entries_to_map(old_self_view).contains_key(
                                other.entries@[src].0));
                            // Veracity: NEEDED assert (speed hint)
                            assert(kept@[b].0@ == other.entries@[src].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, a);
                        } else {
                            let ka = a - phase1_len;
                            let kb = b - phase1_len;
                            let sa = phase2_sources[ka];
                            let sb = phase2_sources[kb];
// Veracity: UNNEEDED assert                             assert(other.entries@[sa].0 != other.entries@[sb].0);
                        }
                    };
                };
                // Combine ensures: for overlap keys, witness v1, v2, r.
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] spec_entries_to_map(old_self_view).contains_key(k)
                    && other@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == spec_entries_to_map(old_self_view)[k]
                        && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && self@[k] == r@)
                by {
                    // Key k is in old_self_view, find the index si.
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                    let si = choose|si: int| 0 <= si < old_self_view.len()
                        && (#[trigger] old_self_view[si]).0 == k;
                    // Trigger the combine_idx invariant for index si.
// Veracity: UNNEEDED assert                     assert(spec_entries_to_map(other.entries@).contains_key(k));
                    let ci = combine_idx[si];
                    // From Phase 1 combine invariant.
// Veracity: UNNEEDED assert                     assert(combine.ensures(
// Veracity: UNNEEDED assert                         (&old_self_raw[si].1, &other_raw[ci].1), phase1_kept[si].1));
                    // Veracity: NEEDED proof block
                    lemma_entries_to_map_get::<K::V, V::V>(old_self_view, si);
                    lemma_entries_to_map_get::<K::V, V::V>(other.entries@, ci);
                    // self@[k]: Phase 2 doesn't contain k (k is in self, so Phase 2 skipped it).
                    // Veracity: NEEDED assert
                    assert forall|idx: int| phase1_len <= idx < self.entries@.len()
                        implies (#[trigger] self.entries@[idx]).0 != k
                    by {
                        let kidx = idx - phase1_len;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let src = phase2_sources[kidx];
// Veracity: UNNEEDED assert                         assert(!spec_entries_to_map(old_self_view).contains_key(
// Veracity: UNNEEDED assert                             other.entries@[src].0));
                        // Veracity: NEEDED assert (speed hint)
                        assert(kept@[idx].0@ == other.entries@[src].0);
                    };
                    // self@[k] comes from Phase 1.
                    lemma_entries_to_map_ignore_suffix::<K::V, V::V>(
                        self.entries@, phase1_len, k);
                    let ghost prefix = self.entries@.subrange(0, phase1_len);
                    // Veracity: NEEDED assert (speed hint)
                    assert(prefix[si] == self.entries@[si]);
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(si) == kept@[si]);
                    lemma_entries_to_map_get::<K::V, V::V>(prefix, si);
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        #[verifier::loop_isolation(false)]
        fn difference(&mut self, other: &Self)
        {
            proof {
// Veracity: UNNEEDED assert                 assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
// Veracity: UNNEEDED assert                 assert(obeys_view_eq_trigger::<K>());
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
                    forall|k: int| 0 <= k < sources.len() ==>
                        0 <= #[trigger] sources[k] < old_self_view.len()
                        && old_self_view[sources[k]].0 == kept@[k].0@
                        // Veracity: NEEDED proof block
                        && old_self_view[sources[k]].1 == kept@[k].1@
                        && !spec_entries_to_map(other_view).contains_key(kept@[k].0@),
                    forall|si: int| 0 <= si < i as int
                        && !spec_entries_to_map(other_view).contains_key(
                            (#[trigger] old_self_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int|
                        // Veracity: NEEDED proof block
                        0 <= j1 < j2 < sources.len() ==> #[trigger] sources[j1] < #[trigger] sources[j2],
                    obeys_view_eq::<K>(),
                decreases self.entries.spec_len() - i,
            {
                // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                {
                    let pair_j = other.entries.nth(j);
                    proof {
                        reveal(obeys_view_eq);
                    }
                    if pair_i.0 == pair_j.0 {
                        match_idx = j;
                    // Veracity: NEEDED proof block
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
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
                // Dom equivalence: result = old \ other.
                let ghost result_dom = spec_entries_to_map(self.entries@).dom();
                let ghost target_dom = spec_entries_to_map(old_self_view).dom().difference(
                    other@.dom());
                // Veracity: NEEDED assert
                // Veracity: NEEDED proof block
                assert forall|k: K::V| result_dom.contains(k) == target_dom.contains(k)
                by {
                    // Veracity: NEEDED proof block
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
// Veracity: UNNEEDED assert                         assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        // Veracity: NEEDED proof block
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_self_view, s);
                    }
                    if spec_entries_to_map(old_self_view).dom().contains(k)
                        && !other@.dom().contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_self_view, k);
                        let si = choose|si: int| 0 <= si < old_self_view.len()
                            && (#[trigger] old_self_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        // Veracity: NEEDED assert (speed hint)
                        // Veracity: NEEDED proof block
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
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
                    self.spec_tablemteph_wf(),
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
                // Veracity: NEEDED proof block
                lemma_entries_to_map_no_key::<K::V, V::V>(self.entries@, key@);
            }
            None
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn delete(&mut self, key: &K)
        {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                // Veracity: NEEDED assert (speed hint)
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
                    // Veracity: NEEDED proof block
                    self.entries@ == old_view,
                    src.len() == kept@.len(),
                    forall|j: int| 0 <= j < src.len() ==>
                        0 <= #[trigger] src[j] < old_view.len()
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
            proof {
                let ghost result_map = spec_entries_to_map(self.entries@);
                let ghost target_map = old_map.remove(key@);
                // View-level subsequence connection.
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < src.len() implies
                    0 <= #[trigger] src[j] < old_view.len()
                    && self.entries@[j] == old_view[src[j]]
                by {
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(j) == kept@[j]);
                };
                // Veracity: NEEDED assert
                assert forall|k: K::V| result_map.dom().contains(k)
                    implies target_map.dom().contains(k)
                by {
                    lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                    let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                        && (#[trigger] self.entries@[idx]).0 == k;
                    // Veracity: NEEDED assert (speed hint)
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
// Veracity: UNNEEDED assert                     assert(self.entries.spec_index(j) == kept@[j]);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                };
                // Veracity: NEEDED assert
                assert forall|k: K::V|
                    #[trigger] result_map.dom().contains(k) && target_map.dom().contains(k)
                    implies result_map[k] == target_map[k]
                by {
                    // Veracity: NEEDED proof block
                    lemma_entries_to_map_subseq_value::<K::V, V::V>(
                        old_view, self.entries@, src, k);
                };
                // Prove spec_keys_no_dups for wf ensures.
                // Veracity: NEEDED assert
                assert(spec_keys_no_dups(self.entries@)) by {
                    // Veracity: NEEDED assert
                    assert forall|a: int, b: int|
                        0 <= a < b < self.entries@.len()
                        implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
                    by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED proof block
                        assert(self.entries@[a] == old_view[src[a]]);
                        // Veracity: NEEDED assert
                        assert(self.entries@[b] == old_view[src[b]]);
// Veracity: UNNEEDED assert                         assert(src[a] < src[b]);
                    };
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        #[verifier::loop_isolation(false)]
        fn insert<F: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, key: K, value: V, combine: F)
        {
            let ghost key_view: K::V = key@;
            let ghost old_view = self.entries@;
            let ghost old_map = self@;
            let n = self.entries.length();
            // Phase 1: find whether the key exists.
            let mut match_index: usize = n;
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == self.entries.spec_len(),
                    self.entries@ == old_view,
                    obeys_view_eq::<K>(),
                    key@ == key_view,
                    match_index <= n,
                    match_index < n ==> old_view[match_index as int].0 == key_view,
                    match_index == n ==> forall|si: int| 0 <= si < i as int
                        ==> (#[trigger] old_view[si]).0 != key_view,
                decreases n - i,
            {
                let pair = self.entries.nth(i);
                proof { reveal(obeys_view_eq); }
                if pair.0 == key {
                    match_index = i;
                    i = n;
                } else {
                    i += 1;
                }
            }
            // Phase 2: build the result array, preserving entry order.
            let ghost value_view: V::V = value@;
            if match_index < n {
                // Key found: rebuild array in-place, replacing the value at match_index.
                let old_entry = self.entries.nth(match_index);
                // Veracity: NEEDED proof block
                proof {
                    lemma_entries_to_map_get::<K::V, V::V>(old_view, match_index as int);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, match_index as int);
                }
                let ghost old_entry_raw = self.entries.seq@[match_index as int];
                let final_value = combine(&old_entry.1, &value);
                let ghost combine_result = final_value;
                let mut all: Vec<Pair<K, V>> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant
                        j <= n,
                        n == self.entries.spec_len(),
                        self.entries@ == old_view,
                        all@.len() == j as int,
                        obeys_feq_clone::<K>(),
                        key@ == key_view,
                        match_index < n,
                        old_view[match_index as int].0 == key_view,
                        forall|k: int| 0 <= k < j as int ==>
                            old_view[k].0 == (#[trigger] all@[k]).0@,
                        forall|k: int| 0 <= k < j as int && k != match_index as int ==>
                            old_view[k].1 == (#[trigger] all@[k]).1@,
                        // Track the combined value at match_index.
                        match_index < j ==> all@[match_index as int].1@ == final_value@,
                    decreases n - j,
                {
                    let entry_ref = self.entries.nth(j);
                    if j == match_index {
                        let key_clone = entry_ref.0.clone_plus();
                        let val_clone = final_value.clone_plus();
// Veracity: UNNEEDED assert                         assert(key_clone@ == old_view[j as int].0);
                        all.push(Pair(key_clone, val_clone));
                        // Veracity: NEEDED assert (speed hint)
                        assert(all@[j as int].0@ == old_view[j as int].0);
                    } else {
                        let cloned = entry_ref.clone_plus();
// Veracity: UNNEEDED assert                         assert(cloned@ == old_view[j as int]);
// Veracity: UNNEEDED assert                         assert(cloned.0@ == old_view[j as int].0);
// Veracity: UNNEEDED assert                         assert(cloned.1@ == old_view[j as int].1);
                        all.push(cloned);
                        // Veracity: NEEDED assert (speed hint)
                        assert(all@[j as int].0@ == old_view[j as int].0);
// Veracity: UNNEEDED assert                         assert(all@[j as int].1@ == old_view[j as int].1);
                    }
                    j += 1;
                }
                self.entries = ArraySeqMtEphS::from_vec(all);
                proof {
                    // Same length, same key positions.
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.entries@.len() == old_view.len());
                    // Veracity: NEEDED assert
                    assert forall|k: int| 0 <= k < self.entries@.len()
                        implies (#[trigger] self.entries@[k]).0 == old_view[k].0
                    by {
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(k) == all@[k]);
                    };
                    lemma_entries_to_map_dom_same_keys::<K::V, V::V, V::V>(old_view, self.entries@);
                    // No duplicate keys.
// Veracity: UNNEEDED assert                     assert(spec_keys_no_dups(self.entries@)) by {
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                         assert forall|a: int, b: int|
// Veracity: UNNEEDED assert                             0 <= a < b < self.entries@.len()
// Veracity: UNNEEDED assert                             implies (#[trigger] self.entries@[a]).0 != (#[trigger] self.entries@[b]).0
// Veracity: UNNEEDED assert                         by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(self.entries@[a].0 == old_view[a].0);
// Veracity: UNNEEDED assert                             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                             assert(self.entries@[b].0 == old_view[b].0);
// Veracity: UNNEEDED assert                         };
// Veracity: UNNEEDED assert                     };
                    // Value preservation for non-key entries.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V|
                        k != key_view && #[trigger] old_map.contains_key(k)
                        // Veracity: NEEDED proof block
                        implies self@[k] == old_map[k]
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(si) == all@[si]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(si != match_index as int);
                        // Veracity: NEEDED assert (speed hint)
                        assert(all@[si].1@ == old_view[si].1);
                        lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                        lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    };
                    // Key is in result.
// Veracity: UNNEEDED assert                     assert(self.entries@[match_index as int].0 == key_view);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, match_index as int);
                    // Combine ensures: witness old_v = old_entry_raw.1, r = combine_result.
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, match_index as int);
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(match_index as int) == all@[match_index as int]);
// Veracity: UNNEEDED assert                     assert(self.entries@[match_index as int].1 == final_value@);
                    // Veracity: NEEDED assert (speed hint)
                    assert(combine.ensures((&old_entry_raw.1, &value), combine_result));
// Veracity: UNNEEDED assert                     assert(old_entry_raw.1@ == old_view[match_index as int].1);
// Veracity: UNNEEDED assert                     assert(old_view[match_index as int].1 == old_map[key_view]);
                }
            } else {
                // Key not found: copy all entries, append new pair.
                let mut all: Vec<Pair<K, V>> = Vec::new();
                let mut j: usize = 0;
                while j < n
                    invariant
                        j <= n,
                        n == self.entries.spec_len(),
                        self.entries@ == old_view,
                        all@.len() == j as int,
                        key@ == key_view,
                        forall|k: int| 0 <= k < j as int ==>
                            old_view[k].0 == (#[trigger] all@[k]).0@
                            && old_view[k].1 == all@[k].1@,
                    decreases n - j,
                {
                    let cloned = self.entries.nth(j).clone_plus();
                    all.push(cloned);
                    j += 1;
                }
                all.push(Pair(key, value));
                self.entries = ArraySeqMtEphS::from_vec(all);
                proof {
                    let last = (self.entries@.len() - 1) as int;
                    // Veracity: NEEDED assert
                    assert(self.entries.spec_index(last) == all@[last]);
// Veracity: UNNEEDED assert                     assert(self.entries@[last].0 == key_view);
                    lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, last);
                    // Domain backward.
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
                            // Veracity: NEEDED assert
                            assert(self.entries.spec_index(si) == all@[si]);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, si);
                        }
                    };
                    // Domain forward.
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
                        if idx < n as int {
                            // Veracity: NEEDED assert (speed hint)
                            assert(self.entries@[idx].0 == old_view[idx].0);
                            lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, idx);
                        }
                    };
                    // Veracity: NEEDED proof block
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
                            if a < n as int && b < n as int {
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.entries@[a].0 == old_view[a].0);
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.entries@[b].0 == old_view[b].0);
                            } else if a < n as int && b == last {
                                // Veracity: NEEDED assert (speed hint)
                                assert(self.entries@[a].0 == old_view[a].0);
                                // Veracity: NEEDED assert (speed hint)
                                assert((#[trigger] old_view[a]).0 != key_view);
                            }
                        };
                    };
                    // Value preservation.
                    // Veracity: NEEDED assert
                    assert forall|k: K::V|
                        k != key_view && #[trigger] old_map.contains_key(k)
                        implies self@[k] == old_map[k]
                    by {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        // Veracity: NEEDED proof block
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        // Veracity: NEEDED assert
                        assert(self.entries.spec_index(si) == all@[si]);
                        lemma_entries_to_map_get::<K::V, V::V>(self.entries@, si);
                        lemma_entries_to_map_get::<K::V, V::V>(old_view, si);
                    };
                    // New key value.
                    lemma_entries_to_map_get::<K::V, V::V>(self.entries@, last);
                    lemma_entries_to_map_no_key::<K::V, V::V>(old_view, key_view);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        #[verifier::loop_isolation(false)]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(obeys_feq_full_trigger::<K>());
                // Veracity: NEEDED assert (speed hint)
                // Veracity: NEEDED proof block
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
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < old_view.len()
                        && old_view[sources[j]].0 == kept@[j].0@
                        && old_view[sources[j]].1 == kept@[j].1@
                        && keys@.contains(kept@[j].0@),
                    forall|si: int| 0 <= si < i as int
                        && keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int|
                        0 <= j1 < j2 < sources.len() ==> #[trigger] sources[j1] < #[trigger] sources[j2],
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
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
                by {
                    if result_dom.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(self.entries@, k);
                        let idx = choose|idx: int| 0 <= idx < self.entries@.len()
                            && (#[trigger] self.entries@[idx]).0 == k;
// Veracity: UNNEEDED assert                         assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    // Veracity: NEEDED proof block
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        #[verifier::loop_isolation(false)]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_view = self.entries@;
            let ghost old_map = spec_entries_to_map(old_view);
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<K>());
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert (speed hint)
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
                    forall|j: int| 0 <= j < sources.len() ==>
                        0 <= #[trigger] sources[j] < old_view.len(),
                    forall|j: int| 0 <= j < sources.len() ==>
                        old_view[#[trigger] sources[j]].0 == kept@[j].0@,
                    forall|j: int| 0 <= j < sources.len() ==>
                        old_view[#[trigger] sources[j]].1 == kept@[j].1@,
                    forall|j: int| 0 <= j < sources.len() ==>
                        !keys@.contains((#[trigger] kept@[j]).0@),
                    forall|si: int| 0 <= si < i as int
                        && !keys@.contains((#[trigger] old_view[si]).0)
                        ==> exists|j: int| 0 <= j < sources.len() && sources[j] == si,
                    forall|j: int| 0 <= j < sources.len() ==> #[trigger] sources[j] < i as int,
                    forall|j1: int, j2: int|
                        0 <= j1 < j2 < sources.len() ==> #[trigger] sources[j1] < #[trigger] sources[j2],
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
                                let j = choose|j: int|
                                    0 <= j < old_sources.len() && old_sources[j] == si;
                                // Veracity: NEEDED proof block
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
            self.entries = ArraySeqMtEphS::from_vec(kept);
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
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(idx) == kept@[idx]);
                        let s = sources[idx];
                        lemma_entries_to_map_contains_key::<K::V, V::V>(old_view, s);
                    // Veracity: NEEDED proof block
                    }
                    if spec_entries_to_map(old_view).dom().contains(k) && !keys@.contains(k) {
                        lemma_entries_to_map_key_in_seq::<K::V, V::V>(old_view, k);
                        let si = choose|si: int| 0 <= si < old_view.len()
                            && (#[trigger] old_view[si]).0 == k;
                        let j = choose|j: int| 0 <= j < sources.len() && sources[j] == si;
                        // Veracity: NEEDED assert (speed hint)
                        assert(self.entries.spec_index(j) == kept@[j]);
                        lemma_entries_to_map_contains_key::<K::V, V::V>(self.entries@, j);
                    }
                };
            // Veracity: NEEDED proof block
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn entries(&self) -> (entries: ArraySeqMtEphS<Pair<K, V>>) {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn iter(&self) -> TableMtEphIter<'_, K, V> {
            TableMtEphIter { inner: self.entries.iter() }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
    // veracity: no_requires
    pub fn from_sorted_entries<K: MtKey, V: MtVal>(entries: Vec<Pair<K, V>>) -> (constructed: TableMtEph<K, V>)
        ensures constructed@.dom().finite()
    {
        let seq = ArraySeqMtEphS::from_vec(entries);
        proof {
            lemma_entries_to_map_finite::<K::V, V::V>(seq@);
        }
        TableMtEph { entries: seq }
    }

    //		Section 10. iterators — TableMtEph

    /// Reference iterator over TableMtEph entries, wrapping ArraySeqMtEphIter.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableMtEphIter<'a, K: MtKey, V: MtVal> {
        pub inner: ArraySeqMtEphIter<'a, Pair<K, V>>,
    }

    impl<'a, K: MtKey, V: MtVal> View for TableMtEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) {
            self.inner@
        }
    }

    pub open spec fn iter_invariant_tablemteph<'a, K: MtKey, V: MtVal>(it: &TableMtEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: MtKey, V: MtVal> std::iter::Iterator for TableMtEphIter<'a, K, V> {
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

    /// Ghost iterator for for-loop support over TableMtEphIter.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct TableMtEphGhostIterator<'a, K: MtKey, V: MtVal> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a Pair<K, V>>,
    }

    impl<'a, K: MtKey, V: MtVal> View for TableMtEphGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<'a, K: MtKey, V: MtVal> vstd::pervasive::ForLoopGhostIteratorNew for TableMtEphIter<'a, K, V> {
        type GhostIter = TableMtEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> TableMtEphGhostIterator<'a, K, V> {
            TableMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: MtKey, V: MtVal> vstd::pervasive::ForLoopGhostIterator for TableMtEphGhostIterator<'a, K, V> {
        type ExecIter = TableMtEphIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &TableMtEphIter<'a, K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &TableMtEphIter<'a, K, V>) -> TableMtEphGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: MtKey, V: MtVal> std::iter::IntoIterator for &'a TableMtEph<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = TableMtEphIter<'a, K, V>;
        fn into_iter(self) -> (it: TableMtEphIter<'a, K, V>)
            requires self.spec_tablemteph_wf()
            ensures it@.0 == 0, iter_invariant_tablemteph(&it),
        {
            self.iter()
        }
    }

    //		Section 12. derive impls in verus!


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

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

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
}
