// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: Brian G. Milnes <briangmilnes@gmail.com> 20206-04-23
//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`.

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

pub mod MappingStEph {


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
    };
    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::map_lib::*,
    };
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::Chap05::RelationStEph::RelationStEph::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

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
        vstd::map::group_map_lemmas,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct MappingStEph<A: StT + Hash, B: StT + Hash> {
        pub mapping: RelationStEph<A, B>,
    }

    //		Section 5. view impls


    /// The map whose domain is the relation's first projection and whose value
    /// at `x` is the `y` the relation pairs with `x` (unique under functionality).
    impl<A: StT + Hash, B: StT + Hash> View for MappingStEph<A, B> {
        type V = Map<A::V, B::V>;

        open spec fn view(&self) -> Self::V {
            Map::new(
                self.mapping@.map(|p: (A::V, B::V)| p.0),
                |x: A::V| choose |y: B::V| self.mapping@.contains((x, y))
            )
        }
    }

    //		Section 6. spec fns


    pub open spec fn is_functional_set<X, Y>(s: Set<(X, Y)>) -> bool {
        forall |x: X, y1: Y, y2: Y|
            #![trigger s.contains((x, y1)), s.contains((x, y2))]
            s.contains((x, y1)) && s.contains((x, y2)) ==> y1 == y2
    }

    pub open spec fn is_functional_seq<X: View, Y: View>(s: Seq<Pair<X, Y>>) -> bool {
        is_functional_set(s.map(|i: int, p: Pair<X, Y>| p@).to_set())
    }

    pub open spec fn is_functional_seq_at<X: View, Y: View>(s: Seq<Pair<X, Y>>, p: (X::V, Y::V)) -> bool {
        forall |i: int| #![trigger s[i]]
            0 <= i < s.len() && s[i]@.0 == p.0 ==> s[i]@.1 == p.1
    }

    pub open spec fn is_functional_relation<X: StT + Hash, Y: StT + Hash>(r: RelationStEph<X, Y>) -> bool {
        is_functional_set(r@)
    }

    pub open spec fn is_functional_set_at<X, Y>(s: Set<(X, Y)>, p: (X, Y)) -> bool {
        forall |q: (X, Y)| #![trigger s.contains(q)] s.contains(q) && q.0 == p.0 ==> q.1 == p.1
    }

    //		Section 7. proof fns/broadcast groups


    /// Under functionality, the map view holds `x -> y` exactly when the relation holds `(x, y)`.
    pub proof fn lemma_view_contains_pair<X: StT + Hash, Y: StT + Hash>(m: &MappingStEph<X, Y>, x: X::V, y: Y::V)
        requires
            is_functional_set(m.mapping@),
        ensures
            m.mapping@.contains((x, y)) <==> (m@.dom().contains(x) && m@[x] == y),
    {
        let r = m.mapping@;
        if r.contains((x, y)) {
            assert(m@.dom().contains(x));
            let chosen_y = choose |y2: Y::V| r.contains((x, y2));
            assert(r.contains((x, chosen_y)));
            assert(m@[x] == chosen_y);
        } else if m@.dom().contains(x) {
            let q = choose |q: (X::V, Y::V)| #[trigger] r.contains(q) && x == q.0;
            assert(r.contains((x, q.1)));
            let chosen_y = choose |y2: Y::V| r.contains((x, y2));
            assert(r.contains((x, chosen_y)));
            assert(m@[x] == chosen_y);
        }
    }

    /// Under functionality, the map view's key-value pairs are the relation.
    pub proof fn lemma_view_kv_pairs<X: StT + Hash, Y: StT + Hash>(m: &MappingStEph<X, Y>)
        requires
            is_functional_set(m.mapping@),
        ensures
            m@.kv_pairs() == m.mapping@,
    {
        assert forall |p: (X::V, Y::V)| #[trigger] m@.kv_pairs().contains(p) implies m.mapping@.contains(p) by {
            let k = choose |k: X::V| #[trigger] m@.dom().contains(k) && p == (k, m@[k]);
            lemma_view_contains_pair(m, k, m@[k]);
        }
        assert forall |p: (X::V, Y::V)| #[trigger] m.mapping@.contains(p) implies m@.kv_pairs().contains(p) by {
            assert(m.mapping@.contains((p.0, p.1)));
            lemma_view_contains_pair(m, p.0, p.1);
        }
        assert(m@.kv_pairs() =~= m.mapping@);
    }

    //		Section 8. traits


    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> :
        View<V = Map<X::V, Y::V>> + Sized {

        spec fn spec_mappingsteph_wf(&self) -> bool;
        spec fn spec_valid_key_type() -> bool;

        spec fn is_functional(&self) -> bool;

        /// - Alg Analysis: APAS (Ch05 Def 5.6): definitional predicate, no cost specified.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|^2), Span O(|v|^2). For each element calls is_functional_vec_at which is O(|v|).
        fn is_functional_vec(v: &Vec<Pair<X, Y>>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_seq(v@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): definitional predicate, no cost specified.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|). Scans vec for duplicate domain key at p.
        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_seq_at(v@, p@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): definitional predicate, no cost specified.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|), Span O(|s|). Iterates set, checks domain key at p.
        fn is_functional_SetStEph_at(s: &SetStEph<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_set_at(s@, p@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): definitional predicate, no cost specified.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|²), Span O(|s|²). Iterates set and for each pair calls is_functional_SetStEph_at which is O(|s|).
        fn is_functional_SetStEph(s: &SetStEph<Pair<X, Y>>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_set(s@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): definitional predicate, no cost specified.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|r|²), Span O(|r|²). Delegates to is_functional_SetStEph which is O(|s|²).
        fn is_functional_RelationStEph(r: &RelationStEph<X, Y>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_relation(*r);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            requires Self::spec_valid_key_type()
            ensures
                empty.spec_mappingsteph_wf(),
                empty@ == Map::<X::V, Y::V>::empty();

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|v|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn from_vec(v: Vec<Pair<X, Y>>) -> (mapping: Self)
            requires Self::spec_valid_key_type(), is_functional_seq(v@)
            ensures
                mapping.spec_mappingsteph_wf(),
                forall |i: int| #![trigger v@[i]] 0 <= i < v@.len() ==>
                    mapping@.dom().contains(v@[i]@.0) && mapping@[v@[i]@.0] == v@[i]@.1;

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|r|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|r|), Span O(|r|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn from_relation(r: &RelationStEph<X, Y>) -> (mapping: Self)
            requires Self::spec_valid_key_type(), is_functional_relation(*r)
            ensures
                mapping.spec_mappingsteph_wf(),
                forall |x: X::V, y: Y::V| r@.contains((x, y)) ==>
                    mapping@.dom().contains(x) && mapping@[x] == y;

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (size: usize)
            requires self.spec_mappingsteph_wf()
            ensures size == self@.dom().len();

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|m|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn domain(&self) -> (domain: SetStEph<X>)
            requires self.spec_mappingsteph_wf()
            ensures domain@ == self@.dom();

        /// - The range `{y | exists x. m(x) = y}` is vstd's `Map::values()`.
        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|m|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — agrees on work. Iterates pairs, inserts each value.
        /// - Matches vstd Map::values() from map_lib.
        fn range(&self) -> (range: SetStEph<Y>)
            requires self.spec_mappingsteph_wf()
            ensures
                range@ == self@.values();

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set contains() on the pair.
        /// - Matches vstd Map::contains_pair() from map_lib.
        fn mem(&self, p: &Pair<X, Y>) -> (contains: bool)
            requires self.spec_mappingsteph_wf()
            ensures
                contains == (self@.dom().contains(p@.0) && self@[p@.0] == p@.1),
                contains == self@.contains_pair(p@.0, p@.1);  // vstd equivalence

        /// - The iterator's pairs are the map's key-value pairs, vstd's `Map::kv_pairs()`.
        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Creates iterator handle.
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, Pair<X, Y>>)
            requires self.spec_mappingsteph_wf()
            ensures
                IteratorSpec::remaining(&it).unref().map(|i: int, p: Pair<X, Y>| p@).to_set() == self@.kv_pairs(),
                IteratorSpec::remaining(&it).unref().no_duplicates(),
                into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref(),
                IteratorSpec::decrease(&it) is Some;
    }

    //		Section 9. impls


    impl<X: StT + Hash, Y: StT + Hash>
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        open spec fn spec_mappingsteph_wf(&self) -> bool {
               valid_key_type_Pair::<X, Y>()
            && is_functional_set(self.mapping@)
            && obeys_feq_full::<Pair<X, Y>>()
        }

        open spec fn spec_valid_key_type() -> bool {
            valid_key_type_Pair::<X, Y>()
        }

        open spec fn is_functional(&self) -> bool {
            is_functional_set(self.mapping@)
        }

        #[verifier::loop_isolation(false)]
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — linear scan checking each element against p.
        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool) {
            let n = v.len();
            for i in 0..n
                invariant
                    n == v@.len(),
                    forall |k: int| #![trigger v@[k]] 0 <= k < i && v@[k]@.0 == p@.0 ==> v@[k]@.1 == p@.1,
            {
                if feq(&v[i].0, &p.0) {
                    if !feq(&v[i].1, &p.1) {
                        return false;
                    }
                }
            }
            true
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|^2), Span O(|v|^2) — for each element calls is_functional_vec_at which is O(|v|).
        fn is_functional_vec(v: &Vec<Pair<X, Y>>) -> (functional: bool) {
            let n = v.len();
            for i in 0..n
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    n == v@.len(),
                    forall |j: int| #![trigger v@[j]] 0 <= j < i ==> is_functional_seq_at(v@, v@[j]@),
            {
                if !Self::is_functional_vec_at(v, &v[i]) {
                    // Veracity: NEEDED proof block
                    proof {
                        let pi = v@[i as int]@;
                        let witness_k = choose |k: int| #![trigger v@[k]] 0 <= k < v@.len() && v@[k]@.0 == pi.0 && v@[k]@.1 != pi.1;
                        let the_seq = v@.map(|idx: int, p: Pair<X, Y>| p@);
                        // Veracity: NEEDED assert
                        assert(the_seq[i as int] == pi);
                        // Veracity: NEEDED assert
                        assert(the_seq[witness_k] == v@[witness_k]@);
                        // Veracity: NEEDED assert
                        assert(the_seq.to_set().contains(pi));
                        // Veracity: NEEDED assert
                        assert(the_seq.to_set().contains(v@[witness_k]@));
                    }
                    return false;
                }
            }
            // TRY: removed proof block
            true
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|), Span O(|s|) — iterates set, compares each element's key against p.
        fn is_functional_SetStEph_at(s: &SetStEph<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool) {
            let s_iter = s.iter();
            let ghost the_seq = into_iter_hash_keys(s_iter);
            let mut iter = s_iter;
            let ghost mut pos: int = 0;
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&iter),
                    IteratorSpec::decrease(&iter) is Some,
                    0 <= pos <= the_seq.len(),
                    IteratorSpec::remaining(&iter).len() == the_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&iter).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&iter)[i]) == the_seq[pos + i],
                    the_seq.map(|i: int, pair: Pair<X,Y>| pair@).to_set() == s@,
                    forall |k: int| #![trigger the_seq[k]] 0 <= k < pos && the_seq[k]@.0 == p@.0 ==> the_seq[k]@.1 == p@.1,
                decreases IteratorSpec::decrease(&iter)->0,
            {
                let ghost old_pos = pos;
                match iter.next() {
                    None => { return true; }
                    Some(q) => {
                        proof { pos = pos + 1; }
                        if feq(&q.0, &p.0) {
                            if !feq(&q.1, &p.1) {
                                // Veracity: NEEDED proof block
                                proof {
                                    let idx = old_pos;
                                    let mapped = the_seq.map(|i: int, pair: Pair<X,Y>| pair@);
                                    // Veracity: NEEDED assert
                                    assert(mapped[idx] == q@);
                                    // Veracity: NEEDED assert
                                    assert(mapped.to_set().contains(q@));
                                }
                                return false;
                            }
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|^2), Span O(|s|^2) — for each element calls is_functional_SetStEph_at which is O(|s|).
        fn is_functional_SetStEph(s: &SetStEph<Pair<X, Y>>) -> (functional: bool) {
            let s_iter = s.iter();
            let ghost the_seq = into_iter_hash_keys(s_iter);
            let mut outer_iter = s_iter;
            let ghost mut pos: int = 0;
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&outer_iter),
                    IteratorSpec::decrease(&outer_iter) is Some,
                    0 <= pos <= the_seq.len(),
                    IteratorSpec::remaining(&outer_iter).len() == the_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&outer_iter).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&outer_iter)[i]) == the_seq[pos + i],
                    the_seq.map(|i: int, pair: Pair<X,Y>| pair@).to_set() == s@,
                    forall |k: int| #![trigger the_seq[k]] 0 <= k < pos ==> is_functional_set_at(s@, the_seq[k]@),
                decreases IteratorSpec::decrease(&outer_iter)->0,
            {
                let ghost old_pos = pos;
                match outer_iter.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |x: X::V, y1: Y::V, y2: Y::V|
                                #![trigger s@.contains((x, y1)), s@.contains((x, y2))]
                                s@.contains((x, y1)) && s@.contains((x, y2)) implies y1 == y2 by {
                                if s@.contains((x, y1)) && s@.contains((x, y2)) {
                                    let mapped = the_seq.map(|i: int, pair: Pair<X,Y>| pair@);
                                    let i1 = choose |i: int| #![trigger mapped[i]] 0 <= i < mapped.len() && mapped[i] == (x, y1);
                                }
                            }
                        }
                        return true;
                    }
                    Some(p) => {
                        proof { pos = pos + 1; }
                        if !Self::is_functional_SetStEph_at(s, p) {
                            // Veracity: NEEDED proof block
                            proof {
                                let idx = old_pos;
                                let mapped = the_seq.map(|i: int, pair: Pair<X,Y>| pair@);
                                // Veracity: NEEDED assert
                                assert(mapped[idx] == p@);
                                // Veracity: NEEDED assert
                                assert(mapped.to_set().contains(p@));
                            }
                            return false;
                        }
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|r|^2), Span O(|r|^2) — delegates to is_functional_SetStEph.
        fn is_functional_RelationStEph(r: &RelationStEph<X, Y>) -> (functional: bool) {
            Self::is_functional_SetStEph(&r.pairs)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty collection.
        fn empty() -> MappingStEph<X, Y> {
            let result = MappingStEph { mapping: RelationStEph::empty() };
            // Veracity: NEEDED proof block
            proof {
                assert(result@.dom() =~= Set::<X::V>::empty());
                assert(result@ =~= Map::<X::V, Y::V>::empty());
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — delegates to SetStEph::from_vec + RelationStEph::from_set.
        fn from_vec(v: Vec<Pair<X, Y>>) -> MappingStEph<X, Y> {
            let ghost v_seq = v@;
            let pairs = SetStEph::from_vec(v);
            let result = MappingStEph { mapping: RelationStEph::from_set(pairs) };
            // Veracity: NEEDED proof block
            proof {
                // result.mapping@ == pairs@ == v_seq.map(|i, p: Pair<X, Y>| p@).to_set()
                // is_functional_seq(v_seq) == is_functional_set(v_seq.map(|i, p| p@).to_set())
                //                          == is_functional_set(result.mapping@)
                // Veracity: NEEDED assert
                assert forall |i: int| #![trigger v_seq[i]] 0 <= i < v_seq.len() implies
                    result@.dom().contains(v_seq[i]@.0) && result@[v_seq[i]@.0] == v_seq[i]@.1 by {
                    lemma_seq_index_in_map_to_set(v_seq, i);
                    let pair_view = v_seq[i]@;
                    assert(result.mapping@.contains((pair_view.0, pair_view.1)));
                    lemma_view_contains_pair(&result, pair_view.0, pair_view.1);
                }
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|r|), Span O(|r|) — clones the relation.
        fn from_relation(r: &RelationStEph<X, Y>) -> MappingStEph<X, Y> {
            let result = MappingStEph { mapping: r.clone() };
            // Veracity: NEEDED proof block
            proof {
                // result.mapping@ == r@ (from clone ensures).
                // Veracity: NEEDED assert
                assert forall |x: X::V, y: Y::V| r@.contains((x, y)) implies
                    result@.dom().contains(x) && result@[x] == y by {
                    lemma_view_contains_pair(&result, x, y);
                }
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to relation size().
        fn size(&self) -> (size: usize) {
            let size = self.mapping.size();
            // Veracity: NEEDED proof block
            proof {
                let s = self.mapping@;
                let proj = |p: (X::V, Y::V)| -> X::V { p.0 };
                // Projection is injective on s because is_functional_set(s).
                // Veracity: NEEDED assert
                assert forall |p1: (X::V, Y::V), p2: (X::V, Y::V)|
                    s.contains(p1) && s.contains(p2) && #[trigger] proj(p1) == #[trigger] proj(p2)
                    implies p1 == p2 by {
                    // p1.0 == p2.0. By functionality, p1.1 == p2.1. So p1 == p2.
                }
                // s.map(proj) == self@.dom()
                // Veracity: NEEDED assert
                assert(s.map(proj) =~= self@.dom());
                vstd::set_lib::lemma_map_size(s, self@.dom(), proj);
            }
            size
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to relation relates().
        fn mem(&self, p: &Pair<X, Y>) -> bool {
            let contains = self.mapping.relates(p);
            // Veracity: NEEDED proof block
            proof {
                assert(self.mapping@.contains((p@.0, p@.1)) == self.mapping@.contains(p@));
                lemma_view_contains_pair(self, p@.0, p@.1);
            }
            contains
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — delegates to relation domain().
        fn domain(&self) -> SetStEph<X> { self.mapping.domain() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — delegates to relation range().
        fn range(&self) -> SetStEph<Y> {
            let result = self.mapping.range();
            // Veracity: NEEDED proof block
            proof {
                let m = self.mapping@;
                // Veracity: NEEDED assert
                assert forall |y: Y::V| #[trigger] result@.contains(y) implies self@.values().contains(y) by {
                    let p = choose |p: (X::V, Y::V)| #[trigger] m.contains(p) && y == p.1;
                    assert(m.contains((p.0, p.1)));
                    lemma_view_contains_pair(self, p.0, p.1);
                }
                // Veracity: NEEDED assert
                assert forall |y: Y::V| #[trigger] self@.values().contains(y) implies result@.contains(y) by {
                    let x = choose |x: X::V| #[trigger] self@.dom().contains(x) && y == self@[x];
                    lemma_view_contains_pair(self, x, y);
                }
                assert(result@ =~= self@.values());
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates iterator handle.
        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> {
            proof { lemma_view_kv_pairs(self); }
            self.mapping.iter()
        }
    }

    //		Section 10. iterators


    // Re-exposes the relation's std iterator (wrapping_iterators_standard.rs, pattern A).
    // An impl of an external trait method may not add `requires`, so the
    // contract is conditional on well-formedness instead.
    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::IntoIterator for &'a MappingStEph<X, Y> {
        type Item = &'a Pair<X, Y>;
        type IntoIter = std::collections::hash_set::Iter<'a, Pair<X, Y>>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.spec_mappingsteph_wf() ==> {
                    &&& IteratorSpec::remaining(&it).unref().map(|i: int, p: Pair<X, Y>| p@).to_set() == self@.kv_pairs()
                    &&& IteratorSpec::remaining(&it).unref().no_duplicates()
                    &&& into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            proof {
                if self.spec_mappingsteph_wf() {
                    lemma_view_kv_pairs(self);
                }
            }
            (&self.mapping).into_iter()
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<A: StT + Hash, B: StT + Hash> PartialEqSpecImpl for MappingStEph<A, B> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<A: StT + Hash, B: StT + Hash> Clone for MappingStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@, self.is_functional() ==> clone.is_functional()
        { MappingStEph { mapping: self.mapping.clone() } }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for MappingStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.mapping.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for MappingStEph<A, B> {}

    impl<A: StT + Hash, B: StT + Hash> PartialEq for MappingStEph<A, B> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let r = self.mapping == other.mapping;
            // Veracity: NEEDED proof block
            proof {
                if r {
                }
            }
            // Verus BUG is preventing this as of Version: 0.2026.02.05.80fb5a4.
            // Veracity: NEEDED proof block
            proof { accept(r == (self@ == other@)); }
            r
        }
    }

  } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::empty()
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __pairs = vec![ $( $crate::Types::Types::Pair($a, $b) ),* ];
            // Check for duplicate domain elements (runtime only, skipped in Verus proof mode)
            #[cfg(not(verus_keep_ghost))]
            {
                let mut __seen_keys = std::collections::HashSet::new();
                for pair in &__pairs {
                    let key = pair.0.clone();
                    if !__seen_keys.insert(key) {
                        panic!("MappingLit!: duplicate domain element {:?}", key);
                    }
                }
            }
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::from_vec(__pairs)
        }};
    }

    //		Section 14. derive impls outside verus!

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.mapping, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.mapping, f) }
    }

}
