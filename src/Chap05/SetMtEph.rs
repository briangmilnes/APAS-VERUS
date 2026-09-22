// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: Brian G. Milnes <briangmilnes@gmail.com> 20206-04-23
//! Chapter 5.1 — Multi-threaded ephemeral Set built on `std::collections::HashSet`.
//! Uses HFSchedulerMtEph for bounded parallel cartesian_product.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 6a. spec fns
//	Section 7a. proof fns/broadcast groups
//	Section 8a. traits
//	Section 9a. impls
//	Section 10a. iterators
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 8c. traits
//	Section 9c. impls
//	Section 11b. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module

// Verus requires parentheses around closures with ensures clauses in function arguments
#[allow(unused_parens)]
pub mod SetMtEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Concurrency::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::vstdplus::accept::accept;

verus!
{


    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;
    use std::collections::HashSet;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::hash::{obeys_key_model, into_iter_hash_keys},
        vstd::std_specs::clone::*,
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::pervasive::strictly_cloned,
        vstd::laws_eq::*,
    };
    use vstd::rwlock::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::hash_specs_plus::hash_specs_plus::lemma_hash_set_clone_eq;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

    //		Section 3. broadcast use


    broadcast use {
        // Set groups
        vstd::set::group_set_lemmas,
        vstd::set_lib::group_set_lib_default,
        vstd::set_lib::group_set_properties,
        // Seq groups
        vstd::seq::group_seq_axioms,
        vstd::prelude::Seq::group_seq_extra,
        vstd::seq_lib::group_seq_lib_default,
        vstd::seq_lib::group_seq_properties,
        // Laws groups
        vstd::laws_eq::group_laws_eq,
        vstd::laws_cmp::group_laws_cmp,
        // Our groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        vstd::std_specs::hash::group_hash_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct SetMtEph<T: StT + Hash> { pub elements: HashSet<T> }

    //		Section 5a. view impls


    impl<T: StT + Hash> View for SetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.elements@.map(|x: T| x@) }
    }

    //		Section 6a. spec fns


    pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
        &&& obeys_key_model::<T>()
        &&& obeys_feq_full::<T>()
    }

    /// Generic wf for non-Self types (Verus cycle workaround).
    /// See `src/standards/spec_wf_standard.rs`.
    pub open spec fn spec_setmteph_wf_generic<V: StT + Hash>(s: &SetMtEph<V>) -> bool {
        valid_key_type::<V>()
    }

    //		Section 7a. proof fns/broadcast groups


    /// Singleton choose: if len == 1 and contains(a), then choose() == a.
    pub broadcast proof fn lemma_singleton_choose<A>(s: Set<A>, a: A)
        requires
            s.len() == 1,
            #[trigger] s.contains(a),
        ensures
            s.choose() == a,
    {
        Set::lemma_is_singleton(s);
    }

    pub broadcast group group_set_mt_eph_lemmas {
        lemma_singleton_choose,
    }

    // The view set is the raw hash set mapped through View. These lemmas cross
    // that map: membership, insertion, the empty set, and length.

    /// The view of a raw element is in the viewed set.
    pub proof fn lemma_viewed_contains<T: StT + Hash>(s: Set<T>, x: T)
        requires
            s.contains(x),
        ensures
            s.map(|k: T| k@).contains(x@),
    {
    }

    /// Under view injectivity, the viewed set holds `x@` exactly when the raw set holds `x`.
    pub proof fn lemma_viewed_mem<T: StT + Hash>(s: Set<T>, x: T)
        requires
            obeys_feq_full::<T>(),
        ensures
            s.map(|k: T| k@).contains(x@) <==> s.contains(x),
    {
        if s.map(|k: T| k@).contains(x@) {
            let a = choose|a: T| #[trigger] s.contains(a) && x@ == a@;
            lemma_reveal_view_injective::<T>();
            assert(a == x);
        }
    }

    /// Inserting a raw element inserts its view.
    pub proof fn lemma_viewed_insert<T: StT + Hash>(s: Set<T>, x: T)
        ensures
            s.insert(x).map(|k: T| k@) == s.map(|k: T| k@).insert(x@),
    {
        s.lemma_set_map_insert_commute(x, |k: T| k@);
    }

    /// The empty raw set views to the empty set.
    pub proof fn lemma_viewed_empty<T: StT + Hash>()
        ensures
            Set::<T>::empty().map(|k: T| k@) == Set::<<T as View>::V>::empty(),
    {
        assert(Set::<T>::empty().map(|k: T| k@) =~= Set::<<T as View>::V>::empty());
    }

    /// Under view injectivity, viewing preserves the length.
    pub proof fn lemma_viewed_len<T: StT + Hash>(s: Set<T>)
        requires
            obeys_feq_full::<T>(),
        ensures
            s.map(|k: T| k@).len() == s.len(),
    {
        lemma_reveal_view_injective::<T>();
        assert(s.injective_on(|k: T| k@));
        vstd::set_lib::lemma_map_size(s, s.map(|k: T| k@), |k: T| k@);
    }

    /// The facts `HashSet::iter` gives about its key sequence, lifted through the view map.
    pub proof fn lemma_iter_keys_view<T: StT + Hash>(s: &SetMtEph<T>, keys: Seq<T>)
        requires
            obeys_feq_full::<T>(),
            keys.to_set() == s.elements@,
            keys.no_duplicates(),
            keys.len() == s.elements@.len(),
        ensures
            keys.map(|i: int, k: T| k@).to_set() == s@,
            keys.len() == s@.len(),
            forall |j: int| 0 <= j < keys.len() ==> s@.contains(#[trigger] keys[j]@),
    {
        // Every key's view is in the set view, and every set-view element is some key's view.
        assert forall |k: T| #![trigger keys.contains(k), s@.contains(k@)] keys.contains(k)
            implies s@.contains(k@) by {
            lemma_viewed_contains(s.elements@, k);
        };
        assert forall |kv: T::V| #[trigger] s@.contains(kv)
            implies exists|k: T| #![trigger keys.contains(k)] keys.contains(k) && k@ == kv by {
            let a = choose|a: T| s.elements@.contains(a) && kv == a@;
            assert(keys.to_set().contains(a));
            assert(keys.contains(a));
        };
        lemma_seq_map_to_set_equality(keys, s@);
        lemma_viewed_len(s.elements@);
        assert forall |j: int| 0 <= j < keys.len() implies s@.contains(#[trigger] keys[j]@) by {
            assert(keys.contains(keys[j]));
            lemma_viewed_contains(s.elements@, keys[j]);
        };
    }

    //		Section 8a. traits


    pub trait SetMtEphTrait<T: StT + Hash> : View<V = Set<<T as View>::V>> + Sized {

        spec fn spec_setmteph_wf(&self) -> bool;
        spec fn spec_valid_key_type() -> bool;

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|v|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — sequential loop, not parallel. Span == Work.
        fn from_vec(v: Vec<T>) -> (s: Self)
            requires Self::spec_valid_key_type()
            ensures s.spec_setmteph_wf(), s@ == v@.map(|i: int, x: T| x@).to_set();

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Creates iterator handle.
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>)
            requires self.spec_setmteph_wf()
            ensures
                IteratorSpec::remaining(&it).unref().map(|i: int, k: T| k@).to_set() == self@,
                IteratorSpec::remaining(&it).unref().no_duplicates(),
                IteratorSpec::remaining(&it).len() == self@.len(),
                forall |j: int| 0 <= j < IteratorSpec::remaining(&it).len() ==> self@.contains(#[trigger] IteratorSpec::remaining(&it).unref()[j]@),
                into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref(),
                IteratorSpec::decrease(&it) is Some;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|self|), Span O(|self|) — iterates set, clones each element.
        fn to_seq(&self) -> (seq: Vec<T>)
            requires self.spec_setmteph_wf()
            ensures
                seq@.no_duplicates(),
                forall |x: T::V| self@.contains(x) <==> seq@.map(|_i: int, t: T| t@).contains(x);

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Allocates empty hash set.
        fn empty()                           -> (empty: Self)
            requires Self::spec_valid_key_type()
            ensures empty.spec_setmteph_wf(), empty@ == Set::<<T as View>::V>::empty();

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. One allocation + one insert.
        fn singleton(x: T)                   -> (s: Self)
            requires Self::spec_valid_key_type()
            ensures s.spec_setmteph_wf(), s@ == Set::empty().insert(x@);

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set len().
        fn size(&self)                       -> (size: usize)
            requires self.spec_setmteph_wf()
            ensures size == self@.len();

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set contains().
        fn mem(&self, x: &T)                 -> (contains: bool)
            requires self.spec_setmteph_wf()
            ensures contains == self@.contains(x@);

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set insert(), amortized.
        fn insert(&mut self, x: T)           -> (inserted: bool)
            requires old(self).spec_setmteph_wf()
            ensures
                self.spec_setmteph_wf(),
                self@ == old(self)@.insert(x@),
                inserted == !old(self)@.contains(x@);

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — sequential loop, not parallel. Span == Work.
        fn union(&self, s2: &Self) -> (union: Self)
            requires
               self.spec_setmteph_wf(),
               s2.spec_setmteph_wf(),
            ensures union.spec_setmteph_wf(), union@ == self@.union(s2@);

        /// - Disjoint union: union of two sets known to be disjoint.
        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — sequential loop, not parallel. Span == Work.
        fn disjoint_union(&self, s2: &Self) -> (union: Self)
            requires
               self.spec_setmteph_wf(),
               s2.spec_setmteph_wf(),
               self@.disjoint(s2@),
            ensures
               union.spec_setmteph_wf(),
               union@ == self@.union(s2@),
               union@.len() == self@.len() + s2@.len();

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — iterates self only, O(1) hash lookup in s2 each. No parallelism.
        fn intersection(&self, s2: &Self) -> (intersection: Self)
            requires
                self.spec_setmteph_wf(),
                s2.spec_setmteph_wf(),
            ensures intersection.spec_setmteph_wf(), intersection@ == self@.intersect(s2@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s2|), Span O(|s2|) — sequential loop, creates one pair per element.
        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            requires
              Self::spec_valid_key_type(),
              spec_setmteph_wf_generic(s2),
              valid_key_type::<Pair<T, U>>(),
            ensures
               spec_setmteph_wf_generic(&product),
               forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (av == a@ && s2@.contains(bv));

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|a| × |b|), Span O(|b|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| × |b|), Span O(|a| × |b|) — spawns |a| parallel elt_cross_set tasks (each O(|b|)), but join phase is sequential disjoint_union over |a| results of size |b|. Sequential join dominates span.
        fn cartesian_product<U: StT + Hash + Clone + Send + Sync + 'static>(&self, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            where T: Send + Sync + 'static, Pair<T, U>: StT + Hash + View<V = (T::V, U::V)>,
            requires
                self.spec_setmteph_wf(),
                spec_setmteph_wf_generic(s2),
                valid_key_type::<Pair<T, U>>(),
            ensures
                spec_setmteph_wf_generic(&product),
                forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|parts|), Span O(|parts|) — iterates parts, O(1) size check each.
        fn all_nonempty(parts: &SetMtEph<SetMtEph<T>>) -> (all_nonempty: bool)
            requires
                Self::spec_valid_key_type(),
                spec_setmteph_wf_generic(parts),
            ensures
                all_nonempty <==> forall |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) ==> s.len() != 0;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|parts|), Span O(|parts|) — iterates parts, O(1) membership check each.
        fn partition_on_elt(x: &T, parts: &SetMtEph<SetMtEph<T>>) -> (partition_on_elt: bool)
            requires
                Self::spec_valid_key_type(),
                spec_setmteph_wf_generic(parts),
            ensures
                partition_on_elt <==> (
                    (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x@)) &&
                    (forall |s1: Set<T::V>, s2: Set<T::V>|
                        #![trigger parts@.contains(s1), parts@.contains(s2)]
                        parts@.contains(s1) && s1.contains(x@) &&
                        parts@.contains(s2) && s2.contains(x@) ==> s1 == s2)
                );

        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|a| × |parts|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| × |parts|), Span O(|a| × |parts|) — sequential loop, not parallel. Span == Work.
        fn partition(&self, parts: &SetMtEph<SetMtEph<T>>) -> (partition: bool)
            requires
                self.spec_setmteph_wf(),
                spec_setmteph_wf_generic(parts),
            ensures
                partition <==> (
                    (forall |x: T::V| self@.contains(x) ==> (
                        (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                        (forall |s1: Set<T::V>, s2: Set<T::V>|
                            #![trigger parts@.contains(s1), parts@.contains(s2)]
                            parts@.contains(s1) && s1.contains(x) &&
                            parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
                    )) &&
                    (forall |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) ==> s.len() != 0)
                );

        /// Split a set into two parts: the first with n elements, the second with the rest.
        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(|self|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|self|), Span O(|self|) — sequential loop, not parallel. Span == Work.
        fn split(&self, n: usize) -> (n_set_rest_set: (Self, Self))
            requires
                self.spec_setmteph_wf(),
                self@.len() >= n,
            ensures
               ({let (n_set, rest_set) = n_set_rest_set;
                  &&& n_set.spec_setmteph_wf()
                  &&& rest_set.spec_setmteph_wf()
                  &&& n_set@.disjoint(rest_set@)
                  &&& n_set@.union(rest_set@) == self@
                  &&& n_set@.len() == n
                  &&& rest_set@.len() == self@.len() - n
               });

        /// Choose an arbitrary element from a non-empty set.
        /// - Alg Analysis: APAS (Ch05 Def 5.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Creates iterator, takes first.
        fn choose(&self) -> (element: T)
            requires
                self.spec_setmteph_wf(),
                self@.len() > 0,
            ensures
                self@.contains(element@);
    }

    //		Section 9a. impls


    impl<T: StT + Hash> SetMtEphTrait<T> for SetMtEph<T> {

        open spec fn spec_setmteph_wf(&self) -> bool {
            valid_key_type::<T>()
            && obeys_feq_full::<T>()
        }

        open spec fn spec_valid_key_type() -> bool {
            valid_key_type::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — iterates vec, O(1) hash insert each.
        fn from_vec(v: Vec<T>) -> SetMtEph<T> {
            let mut s: SetMtEph<T> = SetMtEph::empty();
            let ghost v_seq: Seq<T> = v@;

            for x in iter: v
                invariant
                    valid_key_type::<T>(),
                    iter.seq() == v_seq,
                    s@ == v_seq.take(iter.index()).map(|idx: int, t: T| t@).to_set(),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_extends_the_seq_set_with_view(v_seq, iter.index()); }
                let x_clone: T = x.clone_plus();
                let _ = s.insert(x_clone);
            }
            s
        }

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>) {
            let it = self.elements.iter();
            // Veracity: NEEDED proof block
            proof { lemma_iter_keys_view(self, into_iter_hash_keys(it)); }
            it
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|self|), Span O(|self|) — iterates set, clones each element.
        fn to_seq(&self) -> (seq: Vec<T>) {
            let mut seq: Vec<T> = Vec::new();
            let it = self.iter();
            let ghost iter_seq: Seq<T> = into_iter_hash_keys(it);

            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.seq().unref() == iter_seq,
                    iter_seq.map(|_i: int, k: T| k@).to_set() == self@,
                    iter_seq.no_duplicates(),
                    seq@ == iter_seq.take(iter.index()),
            {
                seq.push(x.clone_plus());
            }
            seq
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty collection.
        fn empty() -> SetMtEph<T> {
            proof { lemma_viewed_empty::<T>(); }
            SetMtEph { elements: HashSet::new() } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — one allocation + one insert.
        fn singleton(x: T) -> (s: SetMtEph<T>) {
            let mut s = HashSet::new();
            let _ = s.insert(x);
            proof {
                lemma_viewed_empty::<T>();
                lemma_viewed_insert(Set::<T>::empty(), x);
            }
            SetMtEph { elements: s }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash set len().
        fn size(&self) -> (size: usize)
            ensures size == self@.len()
        {
            proof { lemma_viewed_len(self.elements@); }
            self.elements.len()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash set contains().
        fn mem(&self, x: &T) -> (contains: bool) {
            proof { lemma_viewed_mem(self.elements@, *x); }
            self.elements.contains(x)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash set insert(), amortized.
        fn insert(&mut self, x: T) -> (inserted: bool)
        {
            proof {
                lemma_viewed_insert(self.elements@, x);
                lemma_viewed_mem(self.elements@, x);
            }
            self.elements.insert(x)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — clone self + iterate s2, insert all. No parallelism.
        fn union(&self, s2: &Self) -> (union: SetMtEph<T>)
        {
            let mut union: SetMtEph<T> = self.clone_plus();
            let it = s2.iter();
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_seq: Seq<T> = into_iter_hash_keys(it);

            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.seq().unref() == s2_seq,
                    s2_seq.map(|i: int, k: T| k@).to_set() == s2@,
                    union@ == s1_view.union(s2_seq.take(iter.index()).map(|i: int, k: T| k@).to_set()),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, iter.index()); }
                let _ = union.insert(x.clone_plus());
            }
            union
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — iterates both sets, insert all. No parallelism.
        fn disjoint_union(&self, s2: &Self) -> (union: SetMtEph<T>)
        {
            let capacity = self.size().saturating_add(s2.size());
            let mut union: SetMtEph<T> = SetMtEph {
                elements: HashSet::with_capacity(capacity)
            };
            proof { lemma_viewed_empty::<T>(); }

            let it1 = self.iter();
            let ghost it1_seq: Seq<T> = into_iter_hash_keys(it1);

            for x in iter1: it1
                invariant
                    valid_key_type::<T>(),
                    iter1.seq().unref() == it1_seq,
                    it1_seq.map(|i: int, k: T| k@).to_set() == self@,
                    union@ == it1_seq.take(iter1.index()).map(|i: int, k: T| k@).to_set(),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_extends_the_seq_set_with_view(it1_seq, iter1.index()); }
                let _ = union.insert(x.clone_plus());
            }

            let it2 = s2.iter();
            let ghost it2_seq: Seq<T> = into_iter_hash_keys(it2);
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_view: Set<T::V> = s2@;

            for x in iter2: it2
                invariant
                    valid_key_type::<T>(),
                    iter2.seq().unref() == it2_seq,
                    it2_seq.map(|i: int, k: T| k@).to_set() == s2_view,
                    s1_view.disjoint(s2_view),
                    union@ == s1_view.union(it2_seq.take(iter2.index()).map(|i: int, k: T| k@).to_set()),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_extends_the_seq_set_with_view(it2_seq, iter2.index()); }
                let _ = union.insert(x.clone_plus());
            }

            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(self@, s2@);
            }

            union
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — iterates self, O(1) hash lookup in s2 each. No parallelism.
        fn intersection(&self, s2: &Self) -> (intersection: SetMtEph<T>)
        {
            let mut intersection: SetMtEph<T> = SetMtEph::empty();
            let it = self.iter();
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_view: Set<T::V> = s2@;
            let ghost s1_seq: Seq<T> = into_iter_hash_keys(it);

            for s1mem in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.seq().unref() == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    s2_view == s2@,
                    intersection@ == s1_seq.take(iter.index()).map(|i: int, k: T| k@).to_set().intersect(s2_view),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_intersect(s1_seq, s2_view, iter.index()); }

                if s2.mem(s1mem) {
                    let _ = intersection.insert(s1mem.clone_plus());
                }
            }

            intersection
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s2|), Span O(|s2|) — iterates s2, creates one pair per element.
        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
        {
            let mut product: SetMtEph<Pair<T, U>> = SetMtEph::empty();
            let it = s2.iter();
            let ghost s2_seq: Seq<U> = into_iter_hash_keys(it);
            let ghost s2_view: Set<U::V> = s2@;
            let ghost a_view: T::V = a@;

            for b in iter: it
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    a_view == a@,
                    iter.seq().unref() == s2_seq,
                    s2_seq.map(|i: int, k: U| k@).to_set() == s2_view,
                    forall |av: T::V, bv: U::V|
                      #![trigger product@.contains((av, bv))]
                       product@.contains((av, bv)) <==>
                       (av == a_view && s2_seq.take(iter.index()).map(|i: int, k: U| k@).to_set().contains(bv)),
            {
                // Veracity: NEEDED proof block
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, iter.index()); }
                let _ = product.insert(Pair(a.clone_plus(), b.clone_plus()));
            }

            product
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| x |b|), Span O(|a| x |b|) — spawns |a| parallel elt_cross_set tasks (each O(|b|)), but sequential disjoint_union join phase dominates span.
        fn cartesian_product<U: StT + Hash + Clone + Send + Sync + 'static>(&self, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            where T: Send + Sync + 'static, Pair<T, U>: StT + Hash + View<V = (T::V, U::V)>,
        {
            let ghost s1_view = self@;
            let ghost s2_view = s2@;

            // Phase 1: Spawn one task per element in s1
            let s1_iter = self.iter();
            let ghost it_seq = into_iter_hash_keys(s1_iter);
            let mut it = s1_iter;
            let ghost mut pos: int = 0;
            let mut handles: Vec<TaskState<SetMtEph<Pair<T, U>>>> = Vec::new();
            let ghost mut spawned_views: Seq<T::V> = Seq::empty();

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= it_seq.len(),
                    IteratorSpec::remaining(&it).len() == it_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == it_seq[pos + i],
                    it_seq.map(|_i: int, k: T| k@).to_set() == s1_view,
                    it_seq.no_duplicates(),
                    s2@ == s2_view,
                    handles@.len() == spawned_views.len(),
                    spawned_views.len() == pos,
                    forall |i: int| #![trigger spawned_views[i]] 0 <= i < spawned_views.len() ==> spawned_views[i] == it_seq[i]@,
                    // Track what each handle's predicate implies (the thread's ensures)
                    forall |i: int, ret: SetMtEph<Pair<T, U>>| (#[trigger] handles@[i].predicate(ret) && 0 <= i < handles@.len()) ==> (
                        forall |av: T::V, bv: U::V| #[trigger] ret@.contains((av, bv)) <==> (av == spawned_views[i] && s2_view.contains(bv))
                    ),
                decreases IteratorSpec::decrease(&it)->0,
            {
                match it.next() {
                    Some(a) => {
                        proof { pos = pos + 1; }
                        let ghost a_view = a@;
                        let ghost idx = pos - 1;

                        // Clone for the task
                        let a_clone = a.clone_plus();
                        let s2_clone = s2.clone();

                        // Veracity: NEEDED proof block
                        proof {
                            lemma_cloned_view_eq(*a, a_clone);
                        }

                        let handle = spawn(
                            (move || -> (r: SetMtEph<Pair<T, U>>)
                                requires
                                    valid_key_type::<T>(),
                                    valid_key_type::<U>(),
                                    valid_key_type::<Pair<T, U>>(),
                                ensures
                                    forall |av: T::V, bv: U::V| r@.contains((av, bv)) <==> (av == a_clone@ && s2_clone@.contains(bv)),
                            {
                                Self::elt_cross_set(&a_clone, &s2_clone)
                            })
                        );

                        handles.push(handle);
                        // Veracity: NEEDED proof block
                        proof { spawned_views = spawned_views.push(a_view); }
                    },
                    None => break,
                }
            }

            // Phase 2: Wait for tasks and disjoint_union results
            // Process in reverse order since we'll pop from the end
            let mut product: SetMtEph<Pair<T, U>> = SetMtEph::empty();
            let ghost mut joined_views: Set<T::V> = Set::empty();
            let ghost n = handles.len();

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    handles@.len() <= n,
                    n == it_seq.len(),
                    spawned_views.len() == n,
                    it_seq.no_duplicates(),
                    forall |j: int| #![trigger spawned_views[j]] 0 <= j < n ==> spawned_views[j] == it_seq[j]@,
                    // joined_views contains the elements we've processed (from the back)
                    forall |j: int| #![trigger spawned_views[j]] handles@.len() <= j < n ==> joined_views.contains(spawned_views[j]),
                    forall |v: T::V| joined_views.contains(v) ==>
                        exists |j: int| handles@.len() <= j < n && v == spawned_views[j],
                    forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (joined_views.contains(av) && s2_view.contains(bv)),
                    // Track what each handle's predicate implies (the thread's ensures)
                    forall |i: int, ret: SetMtEph<Pair<T, U>>| (#[trigger] handles@[i].predicate(ret) && 0 <= i < handles@.len()) ==> (
                        forall |av: T::V, bv: U::V| #[trigger] ret@.contains((av, bv)) <==> (av == spawned_views[i] && s2_view.contains(bv))
                    ),
                decreases handles@.len(),
            {
                if handles.len() == 0 {
                    break;
                }

                let ghost idx = handles.len() - 1;
                let ghost a_view = spawned_views[idx as int];
                let handle: TaskState<SetMtEph<Pair<T, U>>> = handles.pop().unwrap();

                let thread_result: SetMtEph<Pair<T, U>> = wait(handle);

                // Veracity: NEEDED proof block
                proof {
                    // From wait's ensures: handle.predicate(thread_result)
                    // From spawn loop invariant: handle.predicate(ret) ==> (forall av, bv: ...)

                    // Prove a_view is not in the joined_views.
                    // Veracity: NEEDED assert
                    assert(!joined_views.contains(a_view)) by {
                        lemma_reveal_view_injective::<T>();
                        if joined_views.contains(a_view) {
                            // Then exists j in [idx+1, n) with spawned_views[j] == a_view
                            // But spawned_views[idx] == a_view and it_seq.no_duplicates()
                            // This contradicts no_duplicates since it_seq[idx]@ == it_seq[j]@ for j != idx
                        }
                    }

                }

                product = product.disjoint_union(&thread_result);

                // Veracity: NEEDED proof block
                proof {
                    joined_views = joined_views.insert(a_view);
                }
            }

            // Veracity: NEEDED proof block
            proof {
                // Prove joined_views == s1_view
                // At loop end: handles@.len() == 0, so joined_views contains spawned_views[0..n]
                // Veracity: NEEDED assert
                assert(joined_views == s1_view) by {
                    // Veracity: NEEDED assert
                    assert forall |v: T::V| s1_view.contains(v) implies joined_views.contains(v) by {
                        lemma_map_to_set_contains_index(it_seq, v);
                        let j = choose |j: int| #![trigger it_seq[j]] 0 <= j < it_seq.len() && v == it_seq[j]@;
                        // Veracity: NEEDED assert
                        assert(v == spawned_views[j]);
                    }
                }
            }

            product
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|parts|), Span O(|parts|) — iterates parts, O(1) size check each.
        fn all_nonempty(parts: &SetMtEph<SetMtEph<T>>) -> bool {
            let parts_iter       =  parts.iter();
            let ghost parts_seq  = into_iter_hash_keys(parts_iter);
            let ghost parts_view = parts@;
            let mut parts_it     = parts_iter;
            let ghost mut parts_pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetMtEph<T>>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&parts_it),
                    IteratorSpec::decrease(&parts_it) is Some,
                    0 <= parts_pos <= parts_seq.len(),
                    IteratorSpec::remaining(&parts_it).len() == parts_seq.len() - parts_pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&parts_it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&parts_it)[i]) == parts_seq[parts_pos + i],
                    parts_seq.map(|i: int, k: SetMtEph<T>| k@).to_set() == parts_view,
                    forall |i: int| #![trigger parts_seq[i]] 0 <= i < parts_pos ==> parts_seq[i]@.len() != 0,
                decreases IteratorSpec::decrease(&parts_it)->0,
            {
                let ghost old_pos = parts_pos;
                match parts_it.next() {
                    Some(subset) => {
                        proof { parts_pos = parts_pos + 1; }
                        if subset.size() == 0 {
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_seq_index_in_map_to_set(parts_seq, old_pos);
                            }
                            return false;
                        }
                    },
                    None => {
                        return true;
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|parts|), Span O(|parts|) — iterates parts, O(1) membership check each.
        fn partition_on_elt(x: &T, parts: &SetMtEph<SetMtEph<T>>) -> bool {
            let parts_iter = parts.iter();
            let ghost parts_seq = into_iter_hash_keys(parts_iter);
            let ghost parts_view = parts@;
            let ghost x_view = x@;
            let mut parts_it = parts_iter;
            let ghost mut parts_pos: int = 0;
            let mut count: usize = 0;
            let ghost mut found_index: Option<int> = None;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetMtEph<T>>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&parts_it),
                    IteratorSpec::decrease(&parts_it) is Some,
                    0 <= parts_pos <= parts_seq.len(),
                    IteratorSpec::remaining(&parts_it).len() == parts_seq.len() - parts_pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&parts_it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&parts_it)[i]) == parts_seq[parts_pos + i],
                    parts_seq.map(|i: int, k: SetMtEph<T>| k@).to_set() == parts_view,
                    count <= 1,
                    match found_index {
                        Some(idx) => 0 <= idx < parts_pos && parts_seq[idx]@.contains(x_view) && count == 1,
                        None => count == 0,
                    },
                    forall |i: int| #![trigger parts_seq[i]] 0 <= i < parts_pos && parts_seq[i]@.contains(x_view) ==>
                        found_index == Some(i),
                decreases IteratorSpec::decrease(&parts_it)->0,
            {
                let ghost old_pos = parts_pos;
                match parts_it.next() {
                    Some(subset) => {
                        proof { parts_pos = parts_pos + 1; }
                        if subset.mem(x) {
                            let ghost prev_found_index = found_index;
                            count = count + 1;
                            // Veracity: NEEDED proof block
                            proof {
                                found_index = Some(old_pos);
                            }
                            if count > 1 {
                                // Veracity: NEEDED proof block
                                proof {
                                    lemma_reveal_view_injective::<SetMtEph<T>>();
                                    let prev_idx = match prev_found_index { Some(i) => i, None => arbitrary() };
                                    lemma_seq_index_in_map_to_set(parts_seq, prev_idx);
                                    lemma_seq_index_in_map_to_set(parts_seq, old_pos);
                                }
                                return false;
                            }
                        }
                    },
                    None => {
                        if count == 0 {
                            return false;
                        } else {
                          // Veracity: NEEDED proof block
                          proof {
                                let idx = match found_index { Some(i) => i, None => arbitrary() };
                                lemma_seq_index_in_map_to_set(parts_seq, idx);
                            }
                            return true;
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| x |parts|), Span O(|a| x |parts|) — iterates self, calls partition_on_elt (O(|parts|)) each. No parallelism.
        fn partition(&self, parts: &SetMtEph<SetMtEph<T>>) -> bool {
            // First check if all parts are non-empty
            if !Self::all_nonempty(parts) {
                return false;
            }

            let s1_iter = self.iter();
            let ghost s1_seq = into_iter_hash_keys(s1_iter);
            let ghost s1_view = self@;
            let ghost parts_view = parts@;
            let mut s1_it = s1_iter;
            let ghost mut s1_pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetMtEph<T>>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&s1_it),
                    IteratorSpec::decrease(&s1_it) is Some,
                    0 <= s1_pos <= s1_seq.len(),
                    IteratorSpec::remaining(&s1_it).len() == s1_seq.len() - s1_pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&s1_it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&s1_it)[i]) == s1_seq[s1_pos + i],
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    forall |i: int| #![trigger s1_seq[i]] 0 <= i < s1_pos ==> {
                        let x_view = s1_seq[i]@;
                        (exists |s: Set<T::V>| #![trigger parts_view.contains(s)] parts_view.contains(s) && s.contains(x_view)) &&
                        (forall |s1: Set<T::V>, s2: Set<T::V>|
                            #![trigger parts_view.contains(s1), parts_view.contains(s2)]
                            parts_view.contains(s1) && s1.contains(x_view) &&
                            parts_view.contains(s2) && s2.contains(x_view) ==> s1 == s2)
                    },
                decreases IteratorSpec::decrease(&s1_it)->0,
            {
                let ghost old_pos = s1_pos;
                match s1_it.next() {
                    Some(x) => {
                        proof { s1_pos = s1_pos + 1; }
                        if !Self::partition_on_elt(x, parts) {
                            // Veracity: NEEDED proof block
                            proof {
                                lemma_seq_index_in_map_to_set(s1_seq, old_pos);
                            }
                            return false;
                        }
                    },
                    None => {
                        return true;
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|self|), Span O(|self|) — iterates set, O(1) insert each. No parallelism.
        fn split(&self, n: usize) -> (n_set_rest_set: (Self, Self)) {
            let mut first : SetMtEph<T> = SetMtEph::empty();
            let mut second: SetMtEph<T> = SetMtEph::empty();
            let it = self.iter();
            let ghost iter_seq = into_iter_hash_keys(it);
            let ghost self_view = self@;

            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.seq().unref() == iter_seq,
                    iter_seq.map(|_i: int, k: T| k@).to_set() == self_view,
                    iter_seq.no_duplicates(),
                    first@.disjoint(second@),
                    first@.union(second@) == iter_seq.take(iter.index()).map(|_i: int, k: T| k@).to_set(),
                    first@.len() == if iter.index() <= n { iter.index() } else { n as int },
                    second@.len() == if iter.index() <= n { 0 } else { iter.index() - n },
            {
                // Veracity: NEEDED proof block
                proof {
                    lemma_take_one_more_extends_the_seq_set_with_view(iter_seq, iter.index());
                    // Veracity: NEEDED assert
                    assert(!iter_seq.take(iter.index()).map(|_i: int, k: T| k@).to_set().contains(x@)) by {
                        lemma_reveal_view_injective::<T>();
                        if iter_seq.take(iter.index()).map(|_i: int, k: T| k@).to_set().contains(x@) {
                            let mapped = iter_seq.take(iter.index()).map(|_i: int, k: T| k@);
                            let j = mapped.lemma_contains_to_index(x@);
                        }
                    };
                }

                let x_clone = x.clone_plus();
                if first.size() < n {
                    first.insert(x_clone);
                } else {
                    second.insert(x_clone);
                }
            }

            (first, second)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates iterator, takes first element.
        fn choose(&self) -> (element: T) {
            let mut it = self.elements.iter();
            let ghost s: Seq<T> = into_iter_hash_keys(it);

            // Veracity: NEEDED proof block
            proof {
                // s.len() > 0 because self@.len() > 0 and iter ensures bijection
                lemma_viewed_len(self.elements@);
                assert(IteratorSpec::remaining(&it).len() > 0);
            }

            let opt = it.next();
            let element_ref: &T = opt.unwrap();

            // Veracity: NEEDED proof block
            proof {
                // next() ensures element_ref == s[0]
                // Since 0 < s.len(), s.contains(element_ref)
                // Veracity: NEEDED assert
                assert(s.contains(*element_ref)) by {
                    assert(s[0] == *element_ref);
                }
                // From iter ensures: s.contains(k) ==> self@.contains(k@)
                lemma_viewed_contains(self.elements@, *element_ref);
            }

            let result = element_ref.clone_plus();
            // Veracity: NEEDED proof block
            proof {
                lemma_cloned_view_eq(*element_ref, result);
            }
            result
        }
    }

    //		Section 10a. iterators


    // Delegated iteration: `iter()` returns the std hash-set iterator that vstd
    // specifies. The prophetic sequence is tied to the set view through
    // `into_iter_hash_keys`, the non-prophetic contents `peek` reads. An impl
    // of an external trait method may not add `requires`, so the contract is
    // conditional on well-formedness instead.
    impl<'a, T: StT + Hash> std::iter::IntoIterator for &'a SetMtEph<T> {
        type Item = &'a T;
        type IntoIter = std::collections::hash_set::Iter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.spec_setmteph_wf() ==> {
                    &&& IteratorSpec::remaining(&it).unref().map(|i: int, k: T| k@).to_set() == self@
                    &&& IteratorSpec::remaining(&it).unref().no_duplicates()
                    &&& IteratorSpec::remaining(&it).len() == self@.len()
                    &&& into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            let it = self.elements.iter();
            proof {
                if self.spec_setmteph_wf() {
                    lemma_iter_keys_view(self, into_iter_hash_keys(it));
                }
            }
            it
        }
    }

    //		Section 4b. type definitions


    pub struct SetMtEphInv;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct LockedSetMtEph<T: StT + Hash> {
        pub(crate) locked_set: RwLock<SetMtEph<T>, SetMtEphInv>,
        pub(crate) ghost_locked_set: Ghost<Set<<T as View>::V>>,
    }

    //		Section 5c. view impls


    impl<T: StT + Hash> View for LockedSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V {
            self.spec_ghost_locked_set()
        }
    }

    //		Section 8c. traits


    pub trait LockedSetMtEphTrait<T: StT + Hash>
        : View<V = Set<<T as View>::V>> + Sized
    {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates empty set under RwLock.
        fn empty() -> (s: Self)
            requires valid_key_type::<T>()
            ensures s@ == Set::<<T as View>::V>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to size().
        fn size(&self) -> (size: usize)
            ensures size == self@.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to mem().
        fn mem(&self, x: T) -> (contains: bool)
            ensures contains == self@.contains(x@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires write lock, delegates to insert().
        fn insert(&mut self, x: T) -> (inserted: std::result::Result<bool, ()>)
            ensures
                self@ == old(self)@.insert(x@),
                inserted is Ok ==> inserted.unwrap() == !old(self)@.contains(x@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to choose().
        fn choose(&self) -> (element: T)
            requires self@.len() > 0
            ensures self@.contains(element@);
    }

    //		Section 9c. impls


    impl<T: StT + Hash> LockedSetMtEph<T> {
        // The ghost shadow carried `finite()`, which is true by type at 09.13;
        // no invariant on the shadow remains.
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            true
        }

        pub closed spec fn spec_ghost_locked_set(self) -> Set<<T as View>::V> {
            self.ghost_locked_set@
        }
    }

    impl<T: StT + Hash> LockedSetMtEphTrait<T> for LockedSetMtEph<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates empty set under RwLock.
        fn empty() -> (s: Self) {
            let inner = SetMtEph::empty();
            let ghost view = inner@;
            LockedSetMtEph {
                locked_set: RwLock::new(inner, Ghost(SetMtEphInv)),
                ghost_locked_set: Ghost(view),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to size().
        fn size(&self) -> (size: usize) {
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let size = inner.size();
            read_handle.release_read();
            size
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to mem().
        fn mem(&self, x: T) -> (contains: bool) {
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let contains = inner.mem(&x);
            read_handle.release_read();
            contains
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires write lock, delegates to insert().
        fn insert(&mut self, x: T) -> (inserted: std::result::Result<bool, ()>) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            // Veracity: NEEDED proof block
            proof { assume(self.ghost_locked_set@ == locked_val@); }
            let inserted = locked_val.insert(x);
            let ghost new_val = locked_val@;
            self.ghost_locked_set = Ghost(new_val);
            write_handle.release_write(locked_val);
            Ok(inserted)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — acquires read lock, delegates to choose().
        fn choose(&self) -> (element: T) {
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            proof { accept(inner@ == self@); }
            let element = inner.choose();
            read_handle.release_read();
            element
        }
    }

    //		Section 11b. top level coarse locking


    impl<T: StT + Hash> RwLockPredicate<SetMtEph<T>> for SetMtEphInv {
        open spec fn inv(self, v: SetMtEph<T>) -> bool {
            valid_key_type::<T>()
        }
    }

    //		Section 12a. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<T: StT + Hash> PartialEqSpecImpl for SetMtEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<T: StT + Hash> Clone for SetMtEph<T> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        {
            let elements = self.elements.clone();
            proof { lemma_hash_set_clone_eq(self.elements@, elements@); }
            SetMtEph { elements }
        }
    }

    impl<T: StT + Hash> std::hash::Hash for SetMtEph<T> {
        // The body and the trust boundary that `HashSetWithViewPlus::hash` had:
        // `std::collections::HashSet` implements no `Hash`, and Verus 0.2026.09.13
        // does not support calling `core::hash::Hash::hash` on a generic `T`.
        #[verifier::external_body]
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            let mut it = self.elements.iter();
            loop {
                match it.next() {
                    Some(key) => { key.hash(state); },
                    None => { break; },
                }
            }
        }
    }

    impl<T: StT + Hash> Eq for SetMtEph<T> {}

    impl<T: StT + Hash> PartialEq for SetMtEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.elements == other.elements;
            // Veracity: NEEDED proof block
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

  } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! SetMtLit {
        () => {{
            < $crate::Chap05::SetMtEph::SetMtEph::SetMtEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetMtEph::SetMtEph::SetMtEph<_> >::empty();
            $( let _ = __s.insert($x); )*
            __s
        }};
    }

    //		Section 14a. derive impls outside verus!

    impl<T: StT + Hash> std::fmt::Display for SetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMt({})", self.elements.len())
        }
    }

    impl<T: StT + Hash> std::fmt::Debug for SetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMtEph({})", self.elements.len())
        }
    }

    //		Section 14b. derive impls outside verus!

    impl std::fmt::Debug for SetMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMtEphInv")
        }
    }

    impl std::fmt::Display for SetMtEphInv {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMtEphInv")
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<T: StT + Hash> std::fmt::Debug for LockedSetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedSetMtEph")
        }
    }

    impl<T: StT + Hash> std::fmt::Display for LockedSetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "LockedSetMtEph")
        }
    }

}
