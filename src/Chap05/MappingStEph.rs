//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`.

pub mod MappingStEph {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::map_lib::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Chap05::RelationStEph::RelationStEph::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

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
    };

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

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct MappingStEph<A: StT + Hash, B: StT + Hash> {
        pub mapping: RelationStEph<A, B>,
    }

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

    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> : 
        View<V = Map<X::V, Y::V>> + Sized {

        /// A mapping is finite
        open spec fn spec_finite(&self) -> bool {
            self@.dom().finite()
        }

        spec fn is_functional(&self) -> bool;

        /// APAS: Work Θ(|v|), Span Θ(1)
        fn is_functional_vec(v: &Vec<Pair<X, Y>>) -> (functional: bool)
            requires valid_key_type_Pair::<X, Y>()
            ensures functional == is_functional_seq(v@);

        /// APAS: Work Θ(|v|), Span Θ(1)
        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires valid_key_type_Pair::<X, Y>()
            ensures functional == is_functional_seq_at(v@, p@);

        /// APAS: Work Θ(|s|), Span Θ(1)
        fn is_functional_SetStEph_at(s: &SetStEph<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool)
            requires valid_key_type_Pair::<X, Y>()
            ensures functional == is_functional_set_at(s@, p@);

        /// APAS: Work Θ(|s|), Span Θ(1)
        fn is_functional_SetStEph(s: &SetStEph<Pair<X, Y>>) -> (functional: bool)
            requires valid_key_type_Pair::<X, Y>()
            ensures functional == is_functional_set(s@);

        /// APAS: Work Θ(|r|), Span Θ(1)
        fn is_functional_RelationStEph(r: &RelationStEph<X, Y>) -> (functional: bool)
            requires valid_key_type_Pair::<X, Y>()
            ensures functional == is_functional_relation(*r);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            requires valid_key_type_Pair::<X, Y>()
            ensures 
                empty@.dom().finite(),
                empty@ == Map::<X::V, Y::V>::empty(),
                empty.is_functional();

        /// - APAS: Work Θ(|v|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn from_vec(v: Vec<Pair<X, Y>>) -> (mapping: Self)
            requires valid_key_type_Pair::<X, Y>(), is_functional_seq(v@)
            ensures mapping@.dom().finite(), mapping.is_functional();

        /// - APAS: Work Θ(|r|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|r|), Span Θ(1)
        fn from_relation(r: &RelationStEph<X, Y>) -> (mapping: Self)
            requires valid_key_type_Pair::<X, Y>(), is_functional_relation(*r)
            ensures mapping@.dom().finite(), mapping.is_functional();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N
            requires self.is_functional();

        /// - APAS: Work Θ(|m|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|m|), Span Θ(1)
        fn domain(&self) -> (domain: SetStEph<X>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures domain@.finite(), domain@ == self@.dom();

        /// - APAS: Work Θ(|m|), Span Θ(1)
        /// - Matches vstd Map::values() from map_lib.
        fn range(&self) -> (range: SetStEph<Y>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures 
                range@.finite(), 
                range@ =~= Set::<Y::V>::new(|y: Y::V| exists |x: X::V| #![trigger self@[x]] self@.dom().contains(x) && self@[x] == y),
                range@ == self@.values();  // vstd equivalence

        /// APAS: Work Θ(1), Span Θ(1)
        /// Matches vstd Map::contains_pair() from map_lib.
        fn mem(&self, p: &Pair<X, Y>) -> (contains: B)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures 
                contains == (self@.dom().contains(p@.0) && self@[p@.0] == p@.1),
                contains == self@.contains_pair(p@.0, p@.1);  // vstd equivalence

        /// APAS: Work Θ(1), Span Θ(1)
        fn iter<'a>(&'a self) -> (it: MappingStEphIter<'a, X, Y>)
            requires valid_key_type_Pair::<X, Y>(), self.is_functional()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() == 
                    Set::new(|p: (X::V, Y::V)| self@.dom().contains(p.0) && self@[p.0] == p.1),
                it@.1.no_duplicates();
    }

    impl<A: StT + Hash, B: StT + Hash> View for MappingStEph<A, B> {
        type V = Map<A::V, B::V>;
        
        open spec fn view(&self) -> Self::V {
            Map::new(
                |x: A::V| exists |y: B::V| self.mapping@.contains((x, y)),
                |x: A::V| choose |y: B::V| self.mapping@.contains((x, y))
            )
        }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for MappingStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@, self.is_functional() ==> clone.is_functional()
        { MappingStEph { mapping: self.mapping.clone() } }
    }

    impl<X: StT + Hash, Y: StT + Hash> 
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        open spec fn is_functional(&self) -> bool {
            is_functional_set(self.mapping@)
        }

        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, p: &Pair<X, Y>) -> (functional: bool) {
            let n = v.len();
            for i in 0..n
                invariant
                    obeys_feq_full_Pair::<X, Y>(),
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

        fn is_functional_vec(v: &Vec<Pair<X, Y>>) -> (functional: bool) {
            let n = v.len();
            for i in 0..n
                invariant
                    valid_key_type_Pair::<X, Y>(),
                    n == v@.len(),
                    forall |j: int| #![trigger v@[j]] 0 <= j < i ==> is_functional_seq_at(v@, v@[j]@),
            {
                if !Self::is_functional_vec_at(v, &v[i]) {
                    proof {
                        let pi = v@[i as int]@;
                        let witness_k = choose |k: int| #![trigger v@[k]] 0 <= k < v@.len() && v@[k]@.0 == pi.0 && v@[k]@.1 != pi.1;
                        let the_seq = v@.map(|idx: int, p: Pair<X, Y>| p@);
                        assert(the_seq[i as int] == pi);
                        assert(the_seq[witness_k] == v@[witness_k]@);
                        assert(the_seq.to_set().contains(pi));
                        assert(the_seq.to_set().contains(v@[witness_k]@));
                    }
                    return false;
                }
            }
            // TRY: removed proof block
            true
        }


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
                                proof {
                                    let idx = iter@.0 - 1;
                                    let mapped = the_seq.map(|i: int, pair: Pair<X,Y>| pair@);
                                    assert(mapped[idx] == q@);
                                    assert(mapped.to_set().contains(q@));
                                }
                                return false;
                            }
                        }
                    }
                }
            }
        }

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
                        proof {
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
                            proof {
                                let idx = outer_iter@.0 - 1;
                                let mapped = the_seq.map(|i: int, pair: Pair<X,Y>| pair@);
                                assert(mapped[idx] == p@);
                                assert(mapped.to_set().contains(p@));
                            }
                            return false;
                        }
                    }
                }
            }
        }

        fn is_functional_RelationStEph(r: &RelationStEph<X, Y>) -> (functional: bool) {
            Self::is_functional_SetStEph(&r.pairs)
        }

        fn empty() -> MappingStEph<X, Y> {
            let result = MappingStEph { mapping: RelationStEph::empty() };
            proof { 
               assert(result@.dom() =~= Set::empty());
            }
            result
        }

        fn from_vec(v: Vec<Pair<X, Y>>) -> MappingStEph<X, Y> {
            let pairs = SetStEph::from_vec(v);
            let result = MappingStEph { mapping: RelationStEph::from_set(pairs) };
            proof {
                // The domain of the mapping view equals the first projection of the relation
                assert(result@.dom() =~= result.mapping@.map(|p: (X::V, Y::V)| p.0)) by {
                    assert forall |x: X::V| result@.dom().contains(x) <==> 
                        result.mapping@.map(|p: (X::V, Y::V)| p.0).contains(x) by {
                        // LHS: exists |y| result.mapping@.contains((x, y))
                        // RHS: exists |p| result.mapping@.contains(p) && p.0 == x
                        // These are equivalent by choosing p = (x, y) or y = p.1
                    }
                }
                // Since the relation is finite, and map preserves finiteness
                result.mapping@.lemma_map_finite(|p: (X::V, Y::V)| p.0);
            }
            result
        }

        fn from_relation(r: &RelationStEph<X, Y>) -> MappingStEph<X, Y> {
            let result = MappingStEph { mapping: r.clone() };
            proof {
                // The domain of the mapping view equals the first projection of the relation
                assert(result@.dom() =~= result.mapping@.map(|p: (X::V, Y::V)| p.0)) by {
                    assert forall |x: X::V| result@.dom().contains(x) <==> 
                        result.mapping@.map(|p: (X::V, Y::V)| p.0).contains(x) by {
                        // LHS: exists |y| result.mapping@.contains((x, y))
                        // RHS: exists |p| result.mapping@.contains(p) && p.0 == x
                        // These are equivalent by choosing p = (x, y) or y = p.1
                    }
                }
                // Since the relation is finite, and map preserves finiteness
                result.mapping@.lemma_map_finite(|p: (X::V, Y::V)| p.0);
            }
            result
        }

        fn size(&self) -> N { self.mapping.size() }
        fn mem(&self, p: &Pair<X, Y>) -> B { self.mapping.relates(p) }
        fn domain(&self) -> SetStEph<X> { self.mapping.domain() }

        fn range(&self) -> SetStEph<Y> { 
            let result = self.mapping.range();
            proof {
                assert forall |y: Y::V| result@.contains(y) implies 
                    (exists |x: X::V| #![trigger self@[x]] self@.dom().contains(x) && self@[x] == y) by {
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

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for MappingStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.mapping.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for MappingStEph<A, B> {}

  } // verus!

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
                    let key = &pair.0;
                    if !__seen_keys.insert(key) {
                        panic!("MappingLit!: duplicate domain element {:?}", key);
                    }
                }
            }
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::from_vec(__pairs)
        }};
    }

    impl<A: StT + Hash, B: StT + Hash> PartialEq for MappingStEph<A, B> {
        fn eq(&self, other: &Self) -> bool { self.mapping == other.mapping }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.mapping, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.mapping, f) }
    }

}
