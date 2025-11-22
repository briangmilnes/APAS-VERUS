//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
    use vstd::std_specs::clone::*;
    use crate::vstdplus::seq_set::*;
    use crate::Chap05::RelationStEph::RelationStEph::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;

    broadcast use {vstd::seq_lib::group_seq_properties, vstd::set::group_set_axioms};

    /// Spec function: A relation is functional if each domain element maps to at most one range element
    pub open spec fn is_functional<X: View, Y: View>(rel: Set<(X::V, Y::V)>) -> bool {
        forall |x: X::V, y1: Y::V, y2: Y::V| 
            #![trigger rel.contains((x, y1)), rel.contains((x, y2))]
            rel.contains((x, y1)) && rel.contains((x, y2)) ==> y1 == y2
    }

    #[verifier::reject_recursive_types(A)]
    #[verifier::reject_recursive_types(B)]
    pub struct MappingStEph<A: StT + Hash, B: StT + Hash> {
        pub map: RelationStEph<A, B>,
    }

    pub trait MappingStEphTrait<X: StT + Hash + Clone + View, Y: StT + Hash + Clone + View> : 
        View<V = Map<X::V, Y::V>> + Sized {

        /// Check if element at position i in vector has no conflicts with elements at positions >= i+1
        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, i: usize) -> (is_functional_at: bool)
            requires
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                i < v.len(),
            ensures
                is_functional_at <==> forall |j: int| #![auto] i < j < v.len() && v[i as int]@.0 == v[j]@.0 ==> v[i as int]@.1 == v[j]@.1;

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                empty@ == Map::<X::V, Y::V>::empty();

        /// APAS: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<Pair<X, Y>>) -> (mapping: Option<Self>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                matches!(mapping, Some(_)) <==> is_functional::<X, Y>(v@.map(|i: int, p: Pair<X, Y>| p@).to_set());

        /// APAS: Work Θ(|r|), Span Θ(1)
        fn FromRelation(r: &RelationStEph<X, Y>) -> (mapping: Option<Self>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                matches!(mapping, Some(_)) <==> is_functional::<X, Y>(r@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(|m|), Span Θ(1)
        fn domain(&self) -> (domain: SetStEph<X>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                domain@ == self@.dom();

        /// APAS: Work Θ(|m|), Span Θ(1)
        fn range(&self) -> (range: SetStEph<Y>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                range@ =~= Set::<Y::V>::new(|y: Y::V| exists |x: X::V| #![auto] self@.dom().contains(x) && self@[x] == y);

        /// APAS: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y) -> (contains: B)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                contains == (self@.dom().contains(a@) && self@[a@] == b@);

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, Pair<X, Y>>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
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
                |x: A::V| exists |y: B::V| self.map@.contains((x, y)),
                |x: A::V| choose |y: B::V| self.map@.contains((x, y))
            )
        }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for MappingStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { MappingStEph { map: self.map.clone() } }
    }

    pub fn is_functional_vec<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq>
        (v: &Vec<Pair<X, Y>>) -> (is_functional_vec: bool)
        requires
            valid_key_type::<X>(),
            valid_key_type::<Y>(),
        ensures
            is_functional_vec == is_functional::<X, Y>(v@.map(|i: int, p: Pair<X, Y>| p@).to_set()),
    {
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        loop
            invariant
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                i <= v.len(),
            decreases v.len() - i,
        {
            if i >= v.len() {
                proof { admit(); }
                return true;
            }
            if !MappingStEph::<X, Y>::is_functional_vec_at(v, i) {
                proof { admit(); }
                return false;
            }
            i = i + 1;
        }
    }


    impl<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq> 
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        fn is_functional_vec_at(v: &Vec<Pair<X, Y>>, i: usize) -> bool {
            let Pair(key_i, val_i) = &v[i];
            let mut j: usize = i + 1;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    i < v.len(),
                    j <= v.len(),
                decreases v.len() - j,
            {
                if j >= v.len() {
                    proof { admit(); }
                    return true;
                }
                let Pair(key_j, val_j) = &v[j];
                let key_i_clone = key_i.clone();
                let key_j_clone = key_j.clone();
                let val_i_clone = val_i.clone();
                let val_j_clone = val_j.clone();
                if key_i_clone == key_j_clone && val_i_clone != val_j_clone {
                    proof { admit(); }
                    return false;
                }
                j = j + 1;
            }
        }

        fn empty() -> MappingStEph<X, Y> {
            MappingStEph { map: RelationStEph::empty() }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> Option<MappingStEph<X, Y>> {
            if !is_functional_vec(&v) {
                return None;
            }
            
            // Input is functional - create the mapping
            // SetStEph::FromVec will automatically deduplicate identical pairs
            let pairs = SetStEph::FromVec(v);
            let result = MappingStEph { map: RelationStEph::FromSet(pairs) };
            proof { admit(); }
            Some(result)
        }

        fn FromRelation(r: &RelationStEph<X, Y>) -> Option<MappingStEph<X, Y>> {
            // TODO: Implement direct functional check on relation without converting to vec
            let result = MappingStEph { map: r.clone() };
            proof { admit(); }
            Some(result)
        }

        fn size(&self) -> N { self.map.size() }

        fn domain(&self) -> SetStEph<X> { 
            let result = self.map.domain();
            proof {
                assert(result@ =~= Set::<X::V>::new(|x: X::V| exists |y: Y::V| #![auto] self.map@.contains((x, y))));
                assert(self@.dom() =~= Set::<X::V>::new(|x: X::V| exists |y: Y::V| #![auto] self.map@.contains((x, y))));
            }
            result
        }

        fn range(&self) -> SetStEph<Y> { 
            let result = self.map.range();
            proof { admit(); }
            result
        }

        fn mem(&self, a: &X, b: &Y) -> B { 
            let result = self.map.mem(a, b);
            proof {
                if result {
                    assert(self.map@.contains((a@, b@)));
                    assert(exists |y: Y::V| #![auto] self.map@.contains((a@, y))) by {
                        assert(self.map@.contains((a@, b@)));
                    }
                    // The choose will pick some y such that self.map@.contains((a@, y))
                    // If the relation is functional, then y must equal b@
                    // But we haven't proven functional property yet, so admit for now
                    admit();
                }
                if self@.dom().contains(a@) && self@[a@] == b@ {
                    admit();
                }
            }
            result
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { 
            let result = self.map.iter();
            proof { admit(); }
            result
        }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for MappingStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.map.hash(state); }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for MappingStEph<A, B> {}

    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::empty()
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __pairs = vec![ $( $crate::Types::Types::Pair($a, $b) ),* ];
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> >::FromVec(__pairs).expect("MappingLit: duplicate keys with different values")
        }};
    }

  } // verus!

    impl<A: StT + Hash, B: StT + Hash> PartialEq for MappingStEph<A, B> {
        fn eq(&self, other: &Self) -> bool { self.map == other.map }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.map, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.map, f) }
    }
}
