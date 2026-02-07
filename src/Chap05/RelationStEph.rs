//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 5.2 ephemeral Relation built on `SetStEph<Pair<A,B>>`.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod RelationStEph {

    use vstd::prelude::*;

verus! {

    //		2. imports

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;


    //		3. broadcast use

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
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct RelationStEph<A: StT + Hash, B: StT + Hash> {
        pub pairs: SetStEph<Pair<A, B>>,
    }

    /// Iterator wrapper to hide SetStEphIter<Pair<X, Y>>.
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct RelationStEphIter<'a, X: StT + Hash, Y: StT + Hash> {
        pub inner: SetStEphIter<'a, Pair<X, Y>>,
    }

    /// Ghost iterator for ForLoopGhostIterator support (for-iter patterns).
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct RelationStEphGhostIterator<'a, X: StT + Hash, Y: StT + Hash> {
        pub pos: int,
        pub elements: Seq<Pair<X, Y>>,
        pub phantom: core::marker::PhantomData<&'a Pair<X, Y>>,
    }


    //		5. view impls

    impl<'a, X: StT + Hash, Y: StT + Hash> View for RelationStEphIter<'a, X, Y> {
        type V = (int, Seq<Pair<X, Y>>);
        open spec fn view(&self) -> (int, Seq<Pair<X, Y>>) { self.inner@ }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> View for RelationStEphGhostIterator<'a, X, Y> {
        type V = Seq<Pair<X, Y>>;

        open spec fn view(&self) -> Seq<Pair<X, Y>> {
            self.elements.take(self.pos)
        }
    }

    impl<A: StT + Hash, B: StT + Hash> View for RelationStEph<A, B> {
        type V = Set<(<A as View>::V, <B as View>::V)>;
        open spec fn view(&self) -> Self::V { self.pairs@ }
    }


    //		6. spec fns

    pub open spec fn iter_invariant<'a, X: StT + Hash, Y: StT + Hash>(it: &RelationStEphIter<'a, X, Y>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    //		8. traits

    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> : 
        View<V = Set<(<X as View>::V, <Y as View>::V)>> + Sized {

        /// A relation is finite
        open spec fn spec_finite(&self) -> bool {
            self@.finite()
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures empty@.finite(), empty@ == Set::<(<X as View>::V, <Y as View>::V)>::empty();

        /// - APAS: Work Θ(|pairs|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|pairs|), Span Θ(1)
        fn from_set(pairs: SetStEph<Pair<X, Y>>) -> (relation: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures relation@.finite(), relation@ == pairs@;

        /// - APAS: Work Θ(|pairs|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|pairs|), Span Θ(1)
        fn from_vec(v: Vec<Pair<X, Y>>) -> (relation: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures relation@.finite(), relation@ == v@.map(|i: int, p: Pair<X, Y>| p@).to_set();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (size: N)
          ensures 
           size == self@.len();

        /// - APAS: Work Θ(|R|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn domain(&self) -> (domain: SetStEph<X>)
            requires valid_key_type_Pair::<X, Y>()
            ensures domain@.finite(), domain@ == Set::<X::V>::new(|x: X::V| exists |y: Y::V| self@.contains((x, y)));

        /// - APAS: Work Θ(|R|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn range(&self) -> (range: SetStEph<Y>)
            requires valid_key_type_Pair::<X, Y>()
            ensures range@.finite(), range@ == Set::<Y::V>::new(|y: Y::V| exists |x: X::V| self@.contains((x, y)));

        /// APAS: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>()
            ensures contains == self@.contains((a@, b@));

        /// APAS: Work Θ(1), Span Θ(1)
        fn relates(&self, p: &Pair<X, Y>) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>()
            ensures contains == self@.contains(p@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn iter<'a>(&'a self) -> (it: RelationStEphIter<'a, X, Y>)
            requires valid_key_type_Pair::<X, Y>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() == self@,
                it@.1.no_duplicates();
    }


    //		9. impls

    impl<X: StT + Hash, Y: StT + Hash> 
        RelationStEphTrait<X, Y> for RelationStEph<X, Y> {

        fn empty() -> RelationStEph<X, Y> { RelationStEph { pairs: SetStEph::empty() }}

        fn from_set(pairs: SetStEph<Pair<X, Y>>) -> RelationStEph<X, Y> { RelationStEph { pairs } }

        fn from_vec(v: Vec<Pair<X, Y>>) -> RelationStEph<X, Y> {
            RelationStEph { pairs: SetStEph::from_vec(v), } }

        fn size(&self) -> N { self.pairs.size() }

        fn domain(&self) -> SetStEph<X> {
            let mut out = SetStEph::<X>::empty();
            let it: RelationStEphIter<X, Y> = self.iter();
            let ghost pairs_seq: Seq<Pair<X, Y>> = it@.1;
            let ghost pairs_view: Set<(X::V, Y::V)> = self@;

            for pair in iter: it
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    iter.elements == pairs_seq,
                    iter.pos <= pairs_seq.len(),
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == Set::<X::V>::new(|x: X::V| 
                        exists |i: int| #![trigger pairs_seq[i]] 0 <= i < iter.pos && pairs_seq[i]@.0 == x),
            {
                let Pair(a, _b) = pair;
                let a_clone = a.clone_plus();
                let _ = out.insert(a_clone);
            }

            proof {
                // Connect invariant to postcondition
                assert forall |x: X::V| out@.contains(x) implies 
                    (exists |y: Y::V| self@.contains((x, y))) by {
                    if out@.contains(x) {
                        let i = choose |i: int| #![trigger pairs_seq[i]] 0 <= i < pairs_seq.len() && pairs_seq[i]@.0 == x;
                        lemma_seq_index_in_map_to_set(pairs_seq, i);
                    }
                }
                assert forall |x: X::V| (exists |y: Y::V| self@.contains((x, y))) implies 
                    out@.contains(x) by {
                    if exists |y: Y::V| self@.contains((x, y)) {
                        let y = choose |y: Y::V| #![trigger self@.contains((x, y))] self@.contains((x, y));
                        lemma_map_to_set_contains_index(pairs_seq, (x, y));
                    }
                }
            }
            out
        }

        fn range(&self) -> SetStEph<Y> {
            let mut out = SetStEph::<Y>::empty();
            let it: RelationStEphIter<X, Y> = self.iter();
            let ghost pairs_seq: Seq<Pair<X, Y>> = it@.1;
            let ghost pairs_view: Set<(X::V, Y::V)> = self@;

            for pair in iter: it
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    iter.elements == pairs_seq,
                    iter.pos <= pairs_seq.len(),
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == Set::<Y::V>::new(|y: Y::V| 
                        exists |i: int| #![trigger pairs_seq[i]] 0 <= i < iter.pos && pairs_seq[i]@.1 == y),
            {
                let Pair(_a, b) = pair;
                let b_clone = b.clone_plus();
                let _ = out.insert(b_clone);
            }

            proof {
                assert forall |y: Y::V| out@.contains(y) implies 
                    (exists |x: X::V| self@.contains((x, y))) by {
                    if out@.contains(y) {
                        let i = choose |i: int| #![trigger pairs_seq[i]] 0 <= i < pairs_seq.len() && pairs_seq[i]@.1 == y;
                        lemma_seq_index_in_map_to_set(pairs_seq, i);
                    }
                }
                assert forall |y: Y::V| (exists |x: X::V| self@.contains((x, y))) implies 
                    out@.contains(y) by {
                    if exists |x: X::V| self@.contains((x, y)) {
                        let x = choose |x: X::V| #![trigger self@.contains((x, y))] self@.contains((x, y));
                        lemma_map_to_set_contains_index(pairs_seq, (x, y));
                    }
                }
            }
            out
        }

        fn mem(&self, a: &X, b: &Y) -> B {
            let a_clone = a.clone_plus();
            let b_clone = b.clone_plus();
            self.pairs.mem(&Pair(a_clone, b_clone))
        }

        fn relates(&self, p: &Pair<X, Y>) -> B {
            self.mem(&p.0, &p.1)
        }

        fn iter(&self) -> RelationStEphIter<'_, X, Y> {
            RelationStEphIter { inner: self.pairs.iter() }
        }
    }

    impl<A: StT + Hash, B: StT + Hash> PartialEqSpecImpl for RelationStEph<A, B> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::Iterator for RelationStEphIter<'a, X, Y> {
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

    impl<'a, X: StT + Hash, Y: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for RelationStEphIter<'a, X, Y> {
        type GhostIter = RelationStEphGhostIterator<'a, X, Y>;

        open spec fn ghost_iter(&self) -> RelationStEphGhostIterator<'a, X, Y> {
            RelationStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> vstd::pervasive::ForLoopGhostIterator for RelationStEphGhostIterator<'a, X, Y> {
        type ExecIter = RelationStEphIter<'a, X, Y>;
        type Item = Pair<X, Y>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &RelationStEphIter<'a, X, Y>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &RelationStEphIter<'a, X, Y>) -> RelationStEphGhostIterator<'a, X, Y> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> std::iter::IntoIterator for &'a RelationStEph<X, Y> {
        type Item = &'a Pair<X, Y>;
        type IntoIter = RelationStEphIter<'a, X, Y>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type_Pair::<X, Y>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() == self@,
                it@.1.no_duplicates(),
        { self.iter() }
    }


    //		11. derive impls in verus!

    impl<A: StT + Hash, B: StT + Hash> Clone for RelationStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@.finite(), clone@ == self@
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


    //		12. macros

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


    //		13. derive impls outside verus!

    impl<A: StT + Hash, B: StT + Hash> Debug for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) }
    }

}
