//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEphWithIter<A,B>`.
//! This version uses RelationStEphWithIter internally.

pub mod MappingStEphWithIter {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap05::RelationStEphWithIter::RelationStEphWithIter::*;
    use crate::Chap05::SetStEphWithIter::SetStEphWithIter::*;
    use crate::Types::Types::*;

    broadcast use {
        vstd::seq_lib::group_seq_properties, 
        vstd::seq::group_seq_axioms,
        vstd::set::group_set_axioms, 
        crate::vstdplus::feq::feq::group_feq_axioms
    };

    pub open spec fn is_functional_set<X, Y>(s: Set<(X, Y)>) -> bool {
        forall |x: X, y1: Y, y2: Y| 
            #![trigger s.contains((x, y1)), s.contains((x, y2))]
            s.contains((x, y1)) && s.contains((x, y2)) ==> y1 == y2
    }

    pub open spec fn is_functional_seq<X: View, Y: View>(s: Seq<Pair<X, Y>>) -> bool {
        is_functional_set(s.map(|i: int, p: Pair<X, Y>| p@).to_set())
    }

    pub open spec fn is_functional_relation<X: StT + Hash, Y: StT + Hash>(r: RelationStEphWithIter<X, Y>) -> bool {
        is_functional_set(r@)
    }

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct MappingStEphWithIter<A: StT + Hash, B: StT + Hash> {
        pub mapping: RelationStEphWithIter<A, B>,
    }

    // Iterator wrapper - delegates to RelationStEphIter
    #[verifier::reject_recursive_types(X)]
    #[verifier::reject_recursive_types(Y)]
    pub struct MappingStEphIter<'a, X: StT + Hash, Y: StT + Hash> {
        pub inner: RelationStEphIter<'a, X, Y>,
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> View for MappingStEphIter<'a, X, Y> {
        type V = SetStEphIterView<(X::V, Y::V)>;
        open spec fn view(&self) -> SetStEphIterView<(X::V, Y::V)> { self.inner@ }
    }

    impl<'a, X: StT + Hash, Y: StT + Hash> MappingStEphIter<'a, X, Y> {
        pub fn next(&mut self) -> (result: Option<&'a Pair<X, Y>>)
            requires
                old(self).inner.inner.inner@.1.no_duplicates(),
                obeys_feq_full::<Pair<X, Y>>(),
            ensures
                self.inner.inner.inner@.1 == old(self).inner.inner.inner@.1,
                self.inner.inner.inner@.1.no_duplicates(),
                ({
                    let old_view = old(self)@;
                    let new_view = self@;
                    match result {
                        None => {
                            &&& old_view.remaining.is_empty()
                            &&& new_view == old_view
                        },
                        Some(element) => {
                            &&& old_view.remaining.contains(element@)
                            &&& new_view.visited == old_view.visited.union(option_to_set(old_view.current))
                            &&& new_view.current == Some(element@)
                            &&& new_view.remaining == old_view.remaining.remove(element@)
                        },
                    }
                })
        {
            self.inner.next()
        }
    }

    pub trait MappingStEphWithIterTrait<X: StT + Hash, Y: StT + Hash> : 
        View<V = Map<X::V, Y::V>> + Sized {

        spec fn is_functional(&self) -> bool;

        fn empty() -> (empty: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures 
                empty@ == Map::<X::V, Y::V>::empty(),
                empty.is_functional();

        fn FromVec(v: Vec<Pair<X, Y>>) -> (mapping: Self)
            requires valid_key_type_Pair::<X, Y>(), is_functional_seq(v@)
            ensures mapping.is_functional();

        fn FromRelation(r: &RelationStEphWithIter<X, Y>) -> (mapping: Self)
            requires valid_key_type_Pair::<X, Y>(), is_functional_relation(*r)
            ensures mapping.is_functional();

        fn size(&self) -> N
            requires self.is_functional();

        fn domain(&self) -> (domain: SetStEphWithIter<X>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures domain@ == self@.dom();

        fn range(&self) -> (range: SetStEphWithIter<Y>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures range@ =~= Set::<Y::V>::new(|y: Y::V| exists |x: X::V| #![auto] self@.dom().contains(x) && self@[x] == y);

        fn mem(&self, p: &Pair<X, Y>) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures contains == (self@.dom().contains(p@.0) && self@[p@.0] == p@.1);

        fn iter<'a>(&'a self) -> (it: MappingStEphIter<'a, X, Y>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures
                it@.visited == Set::<<Pair<X, Y> as View>::V>::empty(),
                it@.current.is_none(),
                it@.remaining == Set::new(|p: (X::V, Y::V)| self@.dom().contains(p.0) && self@[p.0] == p.1),
                it.inner.inner.inner@.1.no_duplicates();
    }

    impl<A: StT + Hash, B: StT + Hash> View for MappingStEphWithIter<A, B> {
        type V = Map<A::V, B::V>;
        
        open spec fn view(&self) -> Self::V {
            Map::new(
                |x: A::V| exists |y: B::V| self.mapping@.contains((x, y)),
                |x: A::V| choose |y: B::V| self.mapping@.contains((x, y))
            )
        }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for MappingStEphWithIter<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@, self.is_functional() ==> clone.is_functional()
        { MappingStEphWithIter { mapping: self.mapping.clone() } }
    }

    impl<X: StT + Hash, Y: StT + Hash> 
        MappingStEphWithIterTrait<X, Y> for MappingStEphWithIter<X, Y> {

        open spec fn is_functional(&self) -> bool {
            is_functional_set(self.mapping@)
        }

        fn empty() -> MappingStEphWithIter<X, Y> {
            MappingStEphWithIter { mapping: RelationStEphWithIter::empty() }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> MappingStEphWithIter<X, Y> {
            let pairs = SetStEphWithIter::FromVec(v);
            MappingStEphWithIter { mapping: RelationStEphWithIter::FromSet(pairs) }
        }

        fn FromRelation(r: &RelationStEphWithIter<X, Y>) -> MappingStEphWithIter<X, Y> {
            MappingStEphWithIter { mapping: r.clone() }
        }

        fn size(&self) -> N { self.mapping.size() }
        fn mem(&self, p: &Pair<X, Y>) -> B { self.mapping.relates(p) }
        fn domain(&self) -> SetStEphWithIter<X> { self.mapping.domain() }

        fn range(&self) -> SetStEphWithIter<Y> { 
            let result = self.mapping.range();
            proof {
                assert forall |y: Y::V| result@.contains(y) implies 
                    (exists |x: X::V| #![auto] self@.dom().contains(x) && self@[x] == y) by {
                    if result@.contains(y) {
                        let witness_x = choose |x: X::V| self.mapping@.contains((x, y));
                        let chosen_y = choose |y_prime: Y::V| self.mapping@.contains((witness_x, y_prime));
                        assert(self@[witness_x] == chosen_y);
                    }
                }
            }
            result
        }

        fn iter(&self) -> MappingStEphIter<'_, X, Y> { 
            MappingStEphIter { inner: self.mapping.iter() }
        }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for MappingStEphWithIter<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.mapping.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for MappingStEphWithIter<A, B> {}

  } // verus!

    impl<A: StT + Hash, B: StT + Hash> PartialEq for MappingStEphWithIter<A, B> {
        fn eq(&self, other: &Self) -> bool { self.mapping == other.mapping }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEphWithIter<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.mapping, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEphWithIter<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.mapping, f) }
    }
}
