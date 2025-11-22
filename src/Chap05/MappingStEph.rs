//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`.

pub mod MappingStEph {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;
    use std::collections::HashMap;

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
        pub rel: RelationStEph<A, B>,
    }

    pub trait MappingStEphTrait<X: StT + Hash + Clone + View, Y: StT + Hash + Clone + View> : 
        View<V = Map<X::V, Y::V>> + Sized {

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
                match mapping {
                    Some(mapping) => {
                        &&& is_functional::<X, Y>(v@.map(|i: int, p: Pair<X, Y>| p@).to_set())
                        &&& mapping@.dom() =~= v@.map(|i: int, p: Pair<X, Y>| p@.0).to_set()
                        &&& forall |x: X::V| #![auto] mapping@.dom().contains(x) ==> 
                            exists |i: int| #![auto] 0 <= i < v.len() && v[i]@.0 == x && mapping@[x] == v[i]@.1
                    },
                    None => true,
                };

        /// APAS: Work Θ(|r|), Span Θ(1)
        fn FromRelation(r: &RelationStEph<X, Y>) -> (mapping: Option<Self>)
            requires 
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X, Y>>(),
            ensures 
                match mapping {
                    Some(mapping) => {
                        &&& is_functional::<X, Y>(r@)
                        &&& mapping@.dom() =~= Set::<X::V>::new(|x: X::V| exists |y: Y::V| r@.contains((x, y)))
                        &&& forall |x: X::V| #![auto] mapping@.dom().contains(x) ==> r@.contains((x, mapping@[x]))
                    },
                    None => true,
                };

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
                |x: A::V| exists |y: B::V| self.rel@.contains((x, y)),
                |x: A::V| choose |y: B::V| self.rel@.contains((x, y))
            )
        }
    }

    impl<A: StT + Hash, B: StT + Hash> Clone for MappingStEph<A, B> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { MappingStEph { rel: self.rel.clone() } }
    }

    // Helper function to ensure functional property (last value wins for each key)
    #[verifier::external_body]
    fn unique_pairs<X: StT + Hash, Y: StT + Hash>(v: Vec<Pair<X, Y>>) -> (unique_pairs: Vec<Pair<X, Y>>)
        ensures
            unique_pairs@.len() <= v@.len(),
    {
        let mut map = HashMap::<X, Y>::new();
        for Pair(a, b) in v {
            map.insert(a, b);
        }
        map.into_iter().map(|(a, b)| Pair(a, b)).collect()
    }

    impl<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq> 
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        fn empty() -> MappingStEph<X, Y> {
            MappingStEph { rel: RelationStEph::empty() }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> Option<MappingStEph<X, Y>> {
            // Check if input is functional (no duplicate keys with different values)
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
                    break;
                }
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
                        break;
                    }
                    let Pair(key_j, val_j) = &v[j];
                    let key_i_clone = key_i.clone();
                    let key_j_clone = key_j.clone();
                    let val_i_clone = val_i.clone();
                    let val_j_clone = val_j.clone();
                    if key_i_clone == key_j_clone && val_i_clone != val_j_clone {
                        return None;
                    }
                    j = j + 1;
                }
                i = i + 1;
            }
            
            let pairs_vec = unique_pairs(v);
            let pairs = SetStEph::FromVec(pairs_vec);
            let result = MappingStEph { rel: RelationStEph::FromSet(pairs) };
            proof { admit(); }
            Some(result)
        }

        fn FromRelation(r: &RelationStEph<X, Y>) -> Option<MappingStEph<X, Y>> {
            // Convert iterator to Vec first
            let mut pairs_vec: Vec<Pair<X, Y>> = Vec::new();
            let rel_iter = r.iter();
            let mut it = rel_iter;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    valid_key_type::<Pair<X, Y>>(),
                    it@.0 <= it@.1.len(),
                decreases it@.1.len() - it@.0,
            {
                match it.next() {
                    Some(pair) => {
                        let Pair(a, b) = pair;
                        let a_clone = a.clone();
                        let b_clone = b.clone();
                        assert(cloned(*a, a_clone));
                        assert(cloned(*b, b_clone));
                        pairs_vec.push(Pair(a_clone, b_clone));
                    },
                    None => {
                        break;
                    }
                }
            }
            
            // Check if the relation is functional
            let mut i: usize = 0;
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    i <= pairs_vec.len(),
                decreases pairs_vec.len() - i,
            {
                if i >= pairs_vec.len() {
                    break;
                }
                let Pair(key_i, val_i) = &pairs_vec[i];
                let mut j: usize = i + 1;
                #[verifier::loop_isolation(false)]
                loop
                    invariant
                        valid_key_type::<X>(),
                        valid_key_type::<Y>(),
                        i < pairs_vec.len(),
                        j <= pairs_vec.len(),
                    decreases pairs_vec.len() - j,
                {
                    if j >= pairs_vec.len() {
                        break;
                    }
                    let Pair(key_j, val_j) = &pairs_vec[j];
                    let key_i_clone = key_i.clone();
                    let key_j_clone = key_j.clone();
                    let val_i_clone = val_i.clone();
                    let val_j_clone = val_j.clone();
                    if key_i_clone == key_j_clone && val_i_clone != val_j_clone {
                        return None;
                    }
                    j = j + 1;
                }
                i = i + 1;
            }
            
            let unique_pairs_vec = unique_pairs(pairs_vec);
            let pairs = SetStEph::FromVec(unique_pairs_vec);
            let result = MappingStEph { rel: RelationStEph::FromSet(pairs) };
            proof { admit(); }
            Some(result)
        }

        fn size(&self) -> N { self.rel.size() }

        fn domain(&self) -> SetStEph<X> { 
            let result = self.rel.domain();
            proof {
                // self@.dom() is defined as Set::new(|x| exists |y| self.rel@.contains((x, y)))
                // result@ is self.rel.domain()@ which equals Set::new(|x| exists |y| self.rel@.contains((x, y)))
                // These are the same by definition
                assert(result@ =~= Set::<X::V>::new(|x: X::V| exists |y: Y::V| #![auto] self.rel@.contains((x, y))));
                assert(self@.dom() =~= Set::<X::V>::new(|x: X::V| exists |y: Y::V| #![auto] self.rel@.contains((x, y))));
            }
            result
        }

        fn range(&self) -> SetStEph<Y> { 
            let result = self.rel.range();
            proof { admit(); }
            result
        }

        fn mem(&self, a: &X, b: &Y) -> B { 
            let result = self.rel.mem(a, b);
            proof {
                // result is true iff self.rel@.contains((a@, b@))
                // By definition of view:
                // - self@.dom().contains(a@) iff exists |y| self.rel@.contains((a@, y))
                // - self@[a@] is choose |y| self.rel@.contains((a@, y))
                // We need to show: result == (self@.dom().contains(a@) && self@[a@] == b@)
                
                if result {
                    // self.rel@.contains((a@, b@)) is true
                    // So exists |y| self.rel@.contains((a@, y)) is true, hence self@.dom().contains(a@)
                    assert(self.rel@.contains((a@, b@)));
                    assert(exists |y: Y::V| #![auto] self.rel@.contains((a@, y))) by {
                        assert(self.rel@.contains((a@, b@)));
                    }
                    // The choose will pick some y such that self.rel@.contains((a@, y))
                    // If the relation is functional, then y must equal b@
                    // But we haven't proven functional property yet, so admit for now
                    admit();
                }
                if self@.dom().contains(a@) && self@[a@] == b@ {
                    // Similar reasoning in reverse
                    admit();
                }
            }
            result
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { 
            let result = self.rel.iter();
            proof { admit(); }
            result
        }
    }

    impl<A: StT + Hash, B: StT + Hash> std::hash::Hash for MappingStEph<A, B> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.rel.hash(state); }
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
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }
    }
}
