//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation built on `SetStEph<Pair<A,B>>`.

pub mod RelationStEph {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    use vstd::std_specs::clone::*;
    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

    broadcast use {vstd::seq_lib::group_seq_properties, vstd::set::group_set_axioms};

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct RelationStEph<A: StT + Hash, B: StT + Hash> {
        pub pairs: SetStEph<Pair<A, B>>,
    }

    pub trait RelationStEphTrait<X: StT + Hash + Clone + View, Y: StT + Hash + Clone + View> : 
        View<V = Set<(<X as View>::V, <Y as View>::V)>> + Sized {

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures empty@ == Set::<(<X as View>::V, <Y as View>::V)>::empty();

        /// APAS: Work Θ(|pairs|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|pairs|), Span Θ(1)
        fn FromSet(pairs: SetStEph<Pair<X, Y>>) -> (relation: Self)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures relation@ == pairs@;

        fn FromVec(v: Vec<Pair<X, Y>>) -> (relation: Self)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures relation@ == v@.map(|i: int, p: Pair<X, Y>| p@).to_set();

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(|R|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn domain(&self) -> (domain: SetStEph<X>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures domain@ == Set::<X::V>::new(|x: X::V| exists |y: Y::V| self@.contains((x, y)));

        /// APAS: Work Θ(|R|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn range(&self) -> (range: SetStEph<Y>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures range@ == Set::<Y::V>::new(|y: Y::V| exists |x: X::V| self@.contains((x, y)));

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y) -> (contains: B)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures contains == self@.contains((a@, b@));

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, Pair<X, Y>>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() == self@,
                it@.1.no_duplicates();
    }

    impl<A: StT + Hash, B: StT + Hash> View for RelationStEph<A, B> {
        type V = Set<(<A as View>::V, <B as View>::V)>;
        open spec fn view(&self) -> Self::V { self.pairs@ }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for RelationStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { RelationStEph { pairs: self.pairs.clone() } }
    }

    impl<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq> 
        RelationStEphTrait<X, Y> for RelationStEph<X, Y> {

        fn empty() -> RelationStEph<X, Y> { RelationStEph { pairs: SetStEph::empty() }}

        fn FromSet(pairs: SetStEph<Pair<X, Y>>) -> RelationStEph<X, Y> { RelationStEph { pairs } }

        fn FromVec(v: Vec<Pair<X, Y>>) -> RelationStEph<X, Y> {
            RelationStEph { pairs: SetStEph::FromVec(v), } }

        fn size(&self) -> N { self.pairs.size() }

        fn domain(&self) -> SetStEph<X> {
            let mut out = SetStEph::<X>::empty();
            let pairs_iter = self.pairs.iter();
            let mut it = pairs_iter;
            let ghost pairs_seq = it@.1;
            let ghost pairs_view = self.pairs@;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    valid_key_type::<Pair<X, Y>>(),
                    it@.0 <= pairs_seq.len(),
                    it@.1 == pairs_seq,
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == Set::<X::V>::new(|x: X::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && pairs_seq[i]@.0 == x),
                decreases pairs_seq.len() - it@.0,
            {
                match it.next() {
                    Some(pair) => {
                        let Pair(a, _b) = pair;
                        let a_clone = a.clone_plus();
                        let _ = out.insert(a_clone);
                    },
                    None => {
                        proof {
                            // Connect invariant to postcondition
                            assert forall |x: X::V| out@.contains(x) implies 
                                (exists |y: Y::V| self@.contains((x, y))) by {
                                if out@.contains(x) {
                                    let i = choose |i: int| #![auto] 0 <= i < pairs_seq.len() && pairs_seq[i]@.0 == x;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(pairs_seq, i);
                                    assert(self@.contains((x, pairs_seq[i]@.1)));
                                }
                            }
                            assert forall |x: X::V| (exists |y: Y::V| self@.contains((x, y))) implies 
                                out@.contains(x) by {
                                if exists |y: Y::V| self@.contains((x, y)) {
                                    let y = choose |y: Y::V| #![auto] self@.contains((x, y));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(pairs_seq, (x, y));
                                }
                            }
                        }
                        return out;
                    }
                }
            }
        }

        fn range(&self) -> SetStEph<Y> {
            let mut out = SetStEph::<Y>::empty();
            let pairs_iter = self.pairs.iter();
            let mut it = pairs_iter;
            let ghost pairs_seq = it@.1;
            let ghost pairs_view = self.pairs@;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    valid_key_type::<Pair<X, Y>>(),
                    it@.0 <= pairs_seq.len(),
                    it@.1 == pairs_seq,
                    pairs_seq.map(|i: int, p: Pair<X, Y>| p@).to_set() == pairs_view,
                    out@ == Set::<Y::V>::new(|y: Y::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && pairs_seq[i]@.1 == y),
                decreases pairs_seq.len() - it@.0,
            {
                match it.next() {
                    Some(pair) => {
                        let Pair(_a, b) = pair;
                        let b_clone = b.clone_plus();
                        let _ = out.insert(b_clone);
                    },
                    None => {
                        proof {
                            assert forall |y: Y::V| out@.contains(y) implies 
                                (exists |x: X::V| self@.contains((x, y))) by {
                                if out@.contains(y) {
                                    let i = choose |i: int| #![auto] 0 <= i < pairs_seq.len() && pairs_seq[i]@.1 == y;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(pairs_seq, i);
                                    assert(self@.contains((pairs_seq[i]@.0, y)));
                                }
                            }
                            assert forall |y: Y::V| (exists |x: X::V| self@.contains((x, y))) implies 
                                out@.contains(y) by {
                                if exists |x: X::V| self@.contains((x, y)) {
                                    let x = choose |x: X::V| #![auto] self@.contains((x, y));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(pairs_seq, (x, y));
                                }
                            }
                        }
                        return out;
                    }
                }
            }
        }

        fn mem(&self, a: &X, b: &Y) -> B {
            let a_clone = a.clone_plus();
            let b_clone = b.clone_plus();
            self.pairs.mem(&Pair(a_clone, b_clone))
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.pairs.iter() }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for RelationStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.pairs.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for RelationStEph<A, B> {}

    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::empty()
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let mut __pairs = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( let _ = __pairs.insert($crate::Types::Types::Pair($a, $b)); )*
            < $crate::Chap05::RelationStEph::RelationStEph::RelationStEph<_, _> >::FromSet(__pairs)
        }};
    }

  } // verus!

    impl<A: StT + Hash, B: StT + Hash> PartialEq for RelationStEph<A, B> {
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for RelationStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) }
    }
}
