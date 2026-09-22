// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: Brian G. Milnes <briangmilnes@gmail.com> 20206-04-23
//! Chapter 5.2 ephemeral Relation built on `SetStEph<Pair<A,B>>`.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod RelationStEph {


    //		Section 2. imports

    use vstd::prelude::*;

verus!
{


    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::hash::{obeys_key_model, into_iter_hash_keys},
        vstd::std_specs::clone::*,
        vstd::std_specs::cmp::PartialEqSpecImpl,
    };
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

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
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct RelationStEph<A: StT + Hash, B: StT + Hash> {
        pub pairs: SetStEph<Pair<A, B>>,
    }

    //		Section 5. view impls


    impl<A: StT + Hash, B: StT + Hash> View for RelationStEph<A, B> {
        type V = Set<(<A as View>::V, <B as View>::V)>;
        open spec fn view(&self) -> Self::V { self.pairs@ }
    }

    //		Section 8. traits


    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> :
        View<V = Set<(<X as View>::V, <Y as View>::V)>> + Sized {

        spec fn spec_relationsteph_wf(&self) -> bool;
        spec fn spec_valid_key_type() -> bool;

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            requires Self::spec_valid_key_type()
            ensures empty.spec_relationsteph_wf(), empty@ == Set::<(<X as View>::V, <Y as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(|pairs|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|pairs|), Span O(|pairs|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn from_set(pairs: SetStEph<Pair<X, Y>>) -> (relation: Self)
            requires Self::spec_valid_key_type()
            ensures relation.spec_relationsteph_wf(), relation@ == pairs@;

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(|pairs|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|pairs|), Span O(|pairs|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn from_vec(v: Vec<Pair<X, Y>>) -> (relation: Self)
            requires Self::spec_valid_key_type()
            ensures relation.spec_relationsteph_wf(), relation@ == v@.map(|i: int, p: Pair<X, Y>| p@).to_set();

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (size: usize)
            requires self.spec_relationsteph_wf()
            ensures size == self@.len();

        /// - The domain is the relation's first projection: `{x | (x, y) in R}`.
        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(|R|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|R|), Span O(|R|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn domain(&self) -> (domain: SetStEph<X>)
            requires self.spec_relationsteph_wf()
            ensures domain@ == self@.map(|p: (X::V, Y::V)| p.0);

        /// - The range is the relation's second projection: `{y | (x, y) in R}`.
        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(|R|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|R|), Span O(|R|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn range(&self) -> (range: SetStEph<Y>)
            requires self.spec_relationsteph_wf()
            ensures range@ == self@.map(|p: (X::V, Y::V)| p.1);

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set contains() on the pair.
        fn mem(&self, a: &X, b: &Y) -> (contains: bool)
            requires self.spec_relationsteph_wf()
            ensures contains == self@.contains((a@, b@));

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Delegates to set mem.
        fn relates(&self, p: &Pair<X, Y>) -> (contains: bool)
            requires self.spec_relationsteph_wf()
            ensures contains == self@.contains(p@);

        /// - Alg Analysis: APAS (Ch05 Def 5.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Creates iterator handle.
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, Pair<X, Y>>)
            requires self.spec_relationsteph_wf()
            ensures
                IteratorSpec::remaining(&it).unref().map(|i: int, p: Pair<X, Y>| p@).to_set() == self@,
                IteratorSpec::remaining(&it).unref().no_duplicates(),
                IteratorSpec::remaining(&it).len() == self@.len(),
                into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref(),
                IteratorSpec::decrease(&it) is Some;
    }

    //		Section 9. impls


    impl<X: StT + Hash, Y: StT + Hash>
        RelationStEphTrait<X, Y> for RelationStEph<X, Y> {

        open spec fn spec_relationsteph_wf(&self) -> bool {
            valid_key_type_Pair::<X, Y>()
        }

        open spec fn spec_valid_key_type() -> bool {
            valid_key_type_Pair::<X, Y>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty collection.
        fn empty() -> RelationStEph<X, Y> { RelationStEph { pairs: SetStEph::empty() }}

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — moves ownership, no copy.
        fn from_set(pairs: SetStEph<Pair<X, Y>>) -> RelationStEph<X, Y> { RelationStEph { pairs } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — delegates to SetStEph::from_vec which iterates vec.
        fn from_vec(v: Vec<Pair<X, Y>>) -> RelationStEph<X, Y> {
            RelationStEph { pairs: SetStEph::from_vec(v), } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to set size().
        fn size(&self) -> usize { self.pairs.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|R|), Span O(|R|) — iterates pairs, inserts each first element.
        fn domain(&self) -> SetStEph<X> {
            let mut out = SetStEph::<X>::empty();
            let it = self.iter();
            let ghost pairs_seq: Seq<Pair<X, Y>> = into_iter_hash_keys(it);
            let ghost pairs_view: Set<(X::V, Y::V)> = self@;

            for pair in iter: it
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    iter.seq().unref() == pairs_seq,
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == pairs_seq.take(iter.index()).map(|i: int, p: Pair<X, Y>| p@).to_set().map(|p: (X::V, Y::V)| p.0),
            {
                // Veracity: NEEDED proof block
                proof {
                    lemma_take_one_more_extends_the_seq_set_with_view(pairs_seq, iter.index());
                    pairs_seq.take(iter.index()).map(|i: int, p: Pair<X, Y>| p@).to_set()
                        .lemma_set_map_insert_commute(pair@, |p: (X::V, Y::V)| p.0);
                }
                let Pair(a, _b) = pair;
                let a_clone = a.clone_plus();
                let _ = out.insert(a_clone);
            }
            out
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|R|), Span O(|R|) — iterates pairs, inserts each second element.
        fn range(&self) -> SetStEph<Y> {
            let mut out = SetStEph::<Y>::empty();
            let it = self.iter();
            let ghost pairs_seq: Seq<Pair<X, Y>> = into_iter_hash_keys(it);
            let ghost pairs_view: Set<(X::V, Y::V)> = self@;

            for pair in iter: it
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    iter.seq().unref() == pairs_seq,
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == pairs_seq.take(iter.index()).map(|i: int, p: Pair<X, Y>| p@).to_set().map(|p: (X::V, Y::V)| p.1),
            {
                // Veracity: NEEDED proof block
                proof {
                    lemma_take_one_more_extends_the_seq_set_with_view(pairs_seq, iter.index());
                    pairs_seq.take(iter.index()).map(|i: int, p: Pair<X, Y>| p@).to_set()
                        .lemma_set_map_insert_commute(pair@, |p: (X::V, Y::V)| p.1);
                }
                let Pair(_a, b) = pair;
                let b_clone = b.clone_plus();
                let _ = out.insert(b_clone);
            }
            out
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash set lookup via clone + mem.
        fn mem(&self, a: &X, b: &Y) -> bool {
            let a_clone = a.clone_plus();
            let b_clone = b.clone_plus();
            self.pairs.mem(&Pair(a_clone, b_clone))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to mem.
        fn relates(&self, p: &Pair<X, Y>) -> bool {
            self.mem(&p.0, &p.1)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates iterator handle.
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> {
            self.pairs.iter()
        }
    }

    //		Section 10. iterators


    // Re-exposes the pair set's std iterator (wrapping_iterators_standard.rs, pattern A).
    // An impl of an external trait method may not add `requires`, so the
    // contract is conditional on well-formedness instead.
    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::IntoIterator for &'a RelationStEph<X, Y> {
        type Item = &'a Pair<X, Y>;
        type IntoIter = std::collections::hash_set::Iter<'a, Pair<X, Y>>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.spec_relationsteph_wf() ==> {
                    &&& IteratorSpec::remaining(&it).unref().map(|i: int, p: Pair<X, Y>| p@).to_set() == self@
                    &&& IteratorSpec::remaining(&it).unref().no_duplicates()
                    &&& IteratorSpec::remaining(&it).len() == self@.len()
                    &&& into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            (&self.pairs).into_iter()
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<A: StT + Hash, B: StT + Hash> PartialEqSpecImpl for RelationStEph<A, B> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<A: StT + Hash, B: StT + Hash> Clone for RelationStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { RelationStEph { pairs: self.pairs.clone() } }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for RelationStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.pairs.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for RelationStEph<A, B> {}

    impl<A: StT + Hash, B: StT + Hash> PartialEq for RelationStEph<A, B> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        { self.pairs == other.pairs }
    }

  } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::empty()
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let mut __pairs = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( let _ = __pairs.insert($crate::Types::Types::Pair($a, $b)); )*
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::from_set(__pairs)
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<A: StT + Hash, B: StT + Hash> Debug for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) }
    }

}
