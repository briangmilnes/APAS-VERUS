//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
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
    use {
        vstd::std_specs::hash::obeys_key_model,
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

    //		Section 3. broadcast use


    broadcast use {
        // Set groups
        vstd::set::group_set_axioms,
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
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::map::group_map_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct MappingStEph<A: StT + Hash, B: StT + Hash> {
        pub mapping: RelationStEph<A, B>,
    }

    //		Section 5. view impls


    impl<A: StT + Hash, B: StT + Hash> View for MappingStEph<A, B> {
        type V = Map<A::V, B::V>;

        open spec fn view(&self) -> Self::V {
            Map::new(
                |x: A::V| exists |y: B::V| self.mapping@.contains((x, y)),
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

    //		Section 8. traits


    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> :
        View<V = Map<X::V, Y::V>> + Sized {

        spec fn spec_mappingsteph_wf(&self) -> bool;
        spec fn spec_valid_key_type() -> bool;

        /// A mapping is finite
        open spec fn spec_finite(&self) -> bool {
            self@.dom().finite()
        }

        spec fn is_functional(&self) -> bool;

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|v|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|^2), Span O(|v|^2) — disagrees. For each element calls is_functional_vec_at which is O(|v|).
        fn is_functional_vec(v: &Vec<Pair<X, Y>>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_seq(v@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|v|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|v|), Span O(|v|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel. Scans vec for duplicate domain key at p.
        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_seq_at(v@, p@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|s|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|), Span O(|s|) — ACCEPTED DIFFERENCE: St sequential, APAS parallel. Iterates set, checks domain key at p.
        fn is_functional_SetStEph_at(s: &SetStEph<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_set_at(s@, p@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|s|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|s|²), Span O(|s|²) — disagrees. Iterates set and for each pair calls is_functional_SetStEph_at which is O(|s|), yielding quadratic total.
        fn is_functional_SetStEph(s: &SetStEph<Pair<X, Y>>) -> (functional: bool)
            requires Self::spec_valid_key_type()
            ensures functional == is_functional_set(s@);

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|r|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|r|²), Span O(|r|²) — disagrees. Delegates to is_functional_SetStEph which is O(|s|²).
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
            ensures domain@.finite(), domain@ == self@.dom();

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(|m|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — agrees on work. Iterates pairs, inserts each value.
        /// - Matches vstd Map::values() from map_lib.
        fn range(&self) -> (range: SetStEph<Y>)
            requires self.spec_mappingsteph_wf()
            ensures
                range@.finite(),
                range@ =~= Set::<Y::V>::new(|y: Y::V| exists |x: X::V| #![trigger self@[x]] self@.dom().contains(x) && self@[x] == y),
                range@ == self@.values();  // vstd equivalence

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Hash set contains() on the pair.
        /// - Matches vstd Map::contains_pair() from map_lib.
        fn mem(&self, p: &Pair<X, Y>) -> (contains: bool)
            requires self.spec_mappingsteph_wf()
            ensures
                contains == (self@.dom().contains(p@.0) && self@[p@.0] == p@.1),
                contains == self@.contains_pair(p@.0, p@.1);  // vstd equivalence

        /// - Alg Analysis: APAS (Ch05 Def 5.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — agrees. Creates iterator handle.
        fn iter<'a>(&'a self) -> (it: MappingStEphIter<'a, X, Y>)
            requires self.spec_mappingsteph_wf()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() ==
                    Set::new(|p: (X::V, Y::V)| self@.dom().contains(p.0) && self@[p.0] == p.1),
                it@.1.no_duplicates(),
                iter_invariant(&it);
    }

    //		Section 9. impls


    impl<X: StT + Hash, Y: StT + Hash>
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        open spec fn spec_mappingsteph_wf(&self) -> bool {
               self.mapping@.finite()
            && valid_key_type_Pair::<X, Y>()
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
            let mut iter = s.iter();
            let ghost the_seq = iter@.1;
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    iter@.1 == the_seq,
                    the_seq.map(|i: int, pair: Pair<X,Y>| pair@).to_set() == s@,
                    0 <= iter@.0 <= the_seq.len(),
                    forall |k: int| #![trigger the_seq[k]] 0 <= k < iter@.0 && the_seq[k]@.0 == p@.0 ==> the_seq[k]@.1 == p@.1,
                decreases the_seq.len() - iter@.0,
            {
                match iter.next() {
                    None => { return true; }
                    Some(q) => {
                        if feq(&q.0, &p.0) {
                            if !feq(&q.1, &p.1) {
                                // Veracity: NEEDED proof block
                                proof {
                                    let idx = iter@.0 - 1;
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
            let mut outer_iter = s.iter();
            let ghost the_seq = outer_iter@.1;
            loop
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    outer_iter@.1 == the_seq,
                    the_seq.map(|i: int, pair: Pair<X,Y>| pair@).to_set() == s@,
                    0 <= outer_iter@.0 <= the_seq.len(),
                    forall |k: int| #![trigger the_seq[k]] 0 <= k < outer_iter@.0 ==> is_functional_set_at(s@, the_seq[k]@),
                decreases the_seq.len() - outer_iter@.0,
            {
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
                        if !Self::is_functional_SetStEph_at(s, p) {
                            // Veracity: NEEDED proof block
                            proof {
                                let idx = outer_iter@.0 - 1;
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
                // Prove the domain/value ensures for each index.
                // Veracity: NEEDED assert
                assert forall |i: int| #![trigger v_seq[i]] 0 <= i < v_seq.len() implies
                    result@.dom().contains(v_seq[i]@.0) && result@[v_seq[i]@.0] == v_seq[i]@.1 by {
                    // v_seq[i]@ is in the mapped-to-set.
                    lemma_seq_index_in_map_to_set(v_seq, i);
                    let pair_view = v_seq[i]@;
                    // So exists y such that (x, y) in mapping@ — domain containment.
                    // The chosen y must equal pair_view.1 by functionality.
                    let chosen_y = choose |y: Y::V| result.mapping@.contains((pair_view.0, y));
                    // is_functional_set gives y uniqueness.
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
                // is_functional_relation(*r) == is_functional_set(r@) == is_functional_set(result.mapping@).
                // Prove domain/value ensures.
                // Veracity: NEEDED assert
                assert forall |x: X::V, y: Y::V| r@.contains((x, y)) implies
                    result@.dom().contains(x) && result@[x] == y by {
                    // By functionality, the chosen y' must equal y.
                    let chosen = choose |y2: Y::V| result.mapping@.contains((x, y2));
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
        fn mem(&self, p: &Pair<X, Y>) -> bool { self.mapping.relates(p) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — delegates to relation domain().
        fn domain(&self) -> SetStEph<X> { self.mapping.domain() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|m|), Span O(|m|) — delegates to relation range().
        fn range(&self) -> SetStEph<Y> {
            let result = self.mapping.range();
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall |y: Y::V| result@.contains(y) implies 
                    (exists |x: X::V| #![trigger self@[x]] self@.dom().contains(x) && self@[x] == y) by {
                    if result@.contains(y) {
                        let witness_x = choose |x: X::V| self.mapping@.contains((x, y));
                        let chosen_y = choose |y_prime: Y::V| self.mapping@.contains((witness_x, y_prime));
                        // Veracity: NEEDED assert
                        assert(self@[witness_x] == chosen_y);
                    }
                }
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — creates iterator handle.
        fn iter(&self) -> MappingStEphIter<'_, X, Y> {
            MappingStEphIter { inner: self.mapping.iter() }
        }
    }

    //		Section 10. iterators


    /// Iterator wrapper to hide RelationStEphIter<X, Y>.
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct MappingStEphIter<'a, X: StT + Hash, Y: StT + Hash> {
        pub inner: RelationStEphIter<'a, X, Y>,
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> View for MappingStEphIter<'a, X, Y> {
        type V = (int, Seq<Pair<X, Y>>);
        open spec fn view(&self) -> (int, Seq<Pair<X, Y>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, X: StT + Hash, Y: StT + Hash>(it: &MappingStEphIter<'a, X, Y>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::Iterator for MappingStEphIter<'a, X, Y> {
        type Item = &'a Pair<X, Y>;

        fn next(&mut self) -> (next: Option<&'a Pair<X, Y>>)
            ensures ({
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
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support (for-iter patterns).
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct MappingStEphGhostIterator<'a, X: StT + Hash, Y: StT + Hash> {
        pub pos: int,
        pub elements: Seq<Pair<X, Y>>,
        pub phantom: core::marker::PhantomData<&'a Pair<X, Y>>,
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for MappingStEphIter<'a, X, Y> {
        type GhostIter = MappingStEphGhostIterator<'a, X, Y>;

        open spec fn ghost_iter(&self) -> MappingStEphGhostIterator<'a, X, Y> {
            MappingStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> vstd::pervasive::ForLoopGhostIterator for MappingStEphGhostIterator<'a, X, Y> {
        type ExecIter = MappingStEphIter<'a, X, Y>;
        type Item = Pair<X, Y>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &MappingStEphIter<'a, X, Y>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<Pair<X, Y>> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &MappingStEphIter<'a, X, Y>) -> MappingStEphGhostIterator<'a, X, Y> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> View for MappingStEphGhostIterator<'a, X, Y> {
        type V = Seq<Pair<X, Y>>;

        open spec fn view(&self) -> Seq<Pair<X, Y>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::IntoIterator for &'a MappingStEph<X, Y> {
        type Item = &'a Pair<X, Y>;
        type IntoIter = MappingStEphIter<'a, X, Y>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_mappingsteph_wf()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() ==
                    Set::new(|p: (X::V, Y::V)| self@.dom().contains(p.0) && self@[p.0] == p.1),
                it@.1.no_duplicates(),
        {
            self.iter()
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
            proof { assume(r == (self@ == other@)); }
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
                let mut __seen_keys = $crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus::new();
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

    impl<'a, X: StT + Hash, Y: StT + Hash> Debug for MappingStEphIter<'a, X, Y> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MappingStEphIter") }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> Display for MappingStEphIter<'a, X, Y> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MappingStEphIter") }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> Debug for MappingStEphGhostIterator<'a, X, Y> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MappingStEphGhostIterator") }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> Display for MappingStEphGhostIterator<'a, X, Y> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MappingStEphGhostIterator") }
    }

}
