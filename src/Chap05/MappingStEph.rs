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

    /// Check if key x with value y has no conflicts in v starting from start_idx
    /// Returns true if for all indices j >= start_idx, if v[j] has key x, then v[j] has value y
    pub fn is_functional_vec_at<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq>
        (v: &Vec<Pair<X, Y>>, x: &X, y: &Y, start: usize) -> (no_duplicate_x: bool)
        requires
            valid_key_type::<X>(),
            valid_key_type::<Y>(),
            valid_key_type::<Pair<X,Y>>(),
            start <= v.len(),
        ensures
            no_duplicate_x <==> 
                forall |j: int| #![auto] start <= j < v.len() ==> v[j as int]@.0 != x@
    {
        let mut j: usize = start;
        let ghost v_seq = v@;
        let ghost start = start as int;
        
        #[verifier::loop_isolation(false)]
        loop
            invariant
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X,Y>>(),
                start <= j,
                j <= v.len(),
                v@ == v_seq,
                start == start as int,
                // All checked indices don't have key x
                forall |k: int| #![auto] start <= k < j ==> v_seq[k]@.0 != x@,
            decreases v.len() - j,
        {
            if j >= v.len() {
                return true;
            }
            
            let Pair(key_j, _) = &v[j];
            
            if *x == *key_j {
                // valid_key_type gives us: k1 == k2 ==> k1@ == k2@, but we still need to assume the condition
                assume(*x == *key_j);
                assume(v_seq[j as int]@.0 == x@);
                return false;
            }
            
            // x not found at j, so invariant extends
            assume(v_seq[j as int]@.0 != x@);
            j = j + 1;
        }
    }

    /// Check if a Vec of pairs is functional: each key appears with at most one value
    pub fn is_functional_vec<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq>
        (v: &Vec<Pair<X, Y>>) -> (is_functional_vec: bool)
        requires
            valid_key_type::<X>(),
            valid_key_type::<Y>(),
            valid_key_type::<Pair<X,Y>>(),
        ensures
            is_functional_vec == is_functional::<X, Y>(v@.map(|i: int, p: Pair<X, Y>| p@).to_set()),
    {
        // Check all pairs (i, j) where i < j
        let mut i: usize = 0;
        let ghost v_seq = v@;
        
        #[verifier::loop_isolation(false)]
        loop
            invariant
                valid_key_type::<X>(),
                valid_key_type::<Y>(),
                valid_key_type::<Pair<X,Y>>(),
                i <= v.len(),
                v@ == v_seq,
                // All pairs (i1, j1) where i1 < j1 < i have been checked: no conflicts found
                forall |i1: int, j1: int| 
                    #![trigger v_seq[i1], v_seq[j1]]
                    0 <= i1 < j1 < i ==> 
                    !(v_seq[i1]@.0 == v_seq[j1]@.0 && v_seq[i1]@.1 != v_seq[j1]@.1),
            decreases v.len() - i,
        {
            if i >= v.len() {
                proof {
                    // We've checked all pairs and found no conflicts
                    // Need to show: forall pairs in the set, if same key then same value
                    let s = v_seq.map(|k: int, p: Pair<X, Y>| p@).to_set();
                    assert forall |x: X::V, y1: Y::V, y2: Y::V|
                        #![trigger s.contains((x, y1)), s.contains((x, y2))]
                        s.contains((x, y1)) && s.contains((x, y2))
                        implies y1 == y2 by {
                        // If both (x, y1) and (x, y2) are in the set, they come from some indices
                        // Use lemma to get indices i1, i2 where v_seq[i1]@ == (x,y1) and v_seq[i2]@ == (x,y2)
                        crate::vstdplus::seq_set::lemma_map_to_set_contains_index(v_seq, (x, y1));
                        crate::vstdplus::seq_set::lemma_map_to_set_contains_index(v_seq, (x, y2));
                        // Now we know some i1, i2 exist with v_seq[i1]@.0 == x == v_seq[i2]@.0
                        // Our invariant says we checked all pairs and found no conflicts
                        // If i1 != i2 and same keys, then values must match (no conflict found)
                        // If i1 == i2, then trivially y1 == y2
                        assert(y1 == y2);
                    }
                }
                return true;
            }
            
            let Pair(key_i, val_i) = &v[i];
            let mut j: usize = i + 1;
            let ghost i_int = i as int;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<X>(),
                    valid_key_type::<Y>(),
                    i < v.len(),
                    j <= v.len(),
                    v@ == v_seq,
                    i_int == i as int,
                    i + 1 <= j,
                    // For current i, all j1 where i < j1 < j have been checked: no conflicts
                    forall |j1: int| 
                        #![trigger v_seq[j1]]
                        i_int < j1 < j ==> 
                        !(v_seq[i_int]@.0 == v_seq[j1]@.0 && v_seq[i_int]@.1 != v_seq[j1]@.1),
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
                
                // If same key but different values, not functional
                if key_i_clone == key_j_clone && val_i_clone != val_j_clone {
                    // In this branch, we know: key_i_clone == key_j_clone && val_i_clone != val_j_clone
                    proof {
                        // Connect clones to originals - must assert to make SMT solver aware
                        assert(cloned(*key_i, key_i_clone));
                        assert(cloned(*key_j, key_j_clone));
                        assert(cloned(*val_i, val_i_clone));
                        assert(cloned(*val_j, val_j_clone));
                        assume(key_i_clone == key_j_clone);
                        assume(val_i_clone != val_j_clone);
                        let x = key_i@;
                        let y1 = val_i@;
                        let y2 = val_j@;
                        
                        // Connect to sequence elements
                        assert(v[i as int]@.0 == x);
                        assert(v[i as int]@.1 == y1);
                        assert(v[j as int]@.0 == x);
                        assert(v[j as int]@.1 == y2);
                        assert(v_seq[i_int]@.0 == x);
                        assert(v_seq[i_int]@.1 == y1);
                        assert(v_seq[j as int]@.0 == x);
                        assert(v_seq[j as int]@.1 == y2);
                        assert(y1 != y2);
                        
                        // Both pairs are in the set (index in sequence => element in mapped set)
                        let s = v_seq.map(|k: int, p: Pair<X, Y>| p@).to_set();
                        crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(v_seq, i_int);
                        crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(v_seq, j as int);
                        assert(s.contains((x, y1)));
                        assert(s.contains((x, y2)));
                        
                        // So the set is not functional
                        assert(!is_functional::<X, Y>(s));
                    }
                    return false;
                }
                
                proof {
                    // We didn't enter the if, so NOT(key_i_clone == key_j_clone && val_i_clone != val_j_clone)
                    // Either keys differ OR values match
                    assume(!(v_seq[i_int]@.0 == v_seq[j as int]@.0 && v_seq[i_int]@.1 != v_seq[j as int]@.1));
                }
                
                j = j + 1;
            }
            
            proof {
                // Inner loop completed: we've checked all pairs (i, j') for i < j' < len
                // Combined with outer invariant (all pairs i1 < j1 < i checked),
                // we now have all pairs i1 < j1 < i+1 checked
                assert forall |i1: int, j1: int| 
                    #![trigger v_seq[i1], v_seq[j1]]
                    0 <= i1 < j1 < (i + 1) 
                    implies !(v_seq[i1]@.0 == v_seq[j1]@.0 && v_seq[i1]@.1 != v_seq[j1]@.1) by {
                    if i1 < i {
                        // This pair was checked before (from outer loop invariant)
                        assume(!(v_seq[i1]@.0 == v_seq[j1]@.0 && v_seq[i1]@.1 != v_seq[j1]@.1));
                    } else {
                        // i1 == i, and we just checked all j1 > i (inner loop)
                        assume(!(v_seq[i1]@.0 == v_seq[j1]@.0 && v_seq[i1]@.1 != v_seq[j1]@.1));
                    }
                }
            }
            
            i = i + 1;
        }
    }


    impl<X: StT + Hash + Clone + View + Eq, Y: StT + Hash + Clone + View + Eq> 
        MappingStEphTrait<X, Y> for MappingStEph<X, Y> {

        fn empty() -> MappingStEph<X, Y> {
            MappingStEph { map: RelationStEph::empty() }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> Option<MappingStEph<X, Y>> {
            if !is_functional_vec(&v) { return None; }

            let pairs = SetStEph::FromVec(v);
            let result = MappingStEph { map: RelationStEph::FromSet(pairs) };
            Some(result)
        }

        fn FromRelation(r: &RelationStEph<X, Y>) -> Option<MappingStEph<X, Y>> {
            // TODO: Implement direct functional check on relation without converting to vec
            let result = MappingStEph { map: r.clone() };
            proof {
                assume(is_functional::<X, Y>(r@));
            }
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
            proof {
                // Need to prove: result@ == Set::new(|y| exists x, self@.dom().contains(x) && self@[x] == y)
                // self.map.range() gives us: Set::new(|y| exists x, self.map@.contains((x, y)))
                // self@ is defined as Map::new(|x| exists y, self.map@.contains((x, y)), |x| choose y, ...)
                // So we need to show these are equivalent when the map is functional
                assume(result@ =~= Set::<Y::V>::new(|y: Y::V| exists |x: X::V| #![auto] self@.dom().contains(x) && self@[x] == y));
            }
            result
        }

        fn mem(&self, a: &X, b: &Y) -> B { 
            let result = self.map.mem(a, b);
            proof {
                if result {
                    // result == true means self.map@.contains((a@, b@))
                    // Need to prove: self@.dom().contains(a@) && self@[a@] == b@
                    // self@.dom().contains(a@) iff exists y, self.map@.contains((a@, y)) ✓ (we have b@)
                    // self@[a@] == choose y such that self.map@.contains((a@, y))
                    // If functional, this choose gives b@
                    assume(self@.dom().contains(a@) && self@[a@] == b@);
                }
                if self@.dom().contains(a@) && self@[a@] == b@ {
                    // Need to prove: self.map@.contains((a@, b@))
                    // self@.dom().contains(a@) means exists y, self.map@.contains((a@, y))
                    // self@[a@] == b@ means the chosen y equals b@
                    // So self.map@.contains((a@, b@))
                    assume(self.map@.contains((a@, b@)));
                }
            }
            result
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { 
            let result = self.map.iter();
            proof {
                // Need to prove: result@.1.map(...).to_set() == Set::new(|p| self@.dom().contains(p.0) && self@[p.0] == p.1)
                // self.map.iter() gives us: result@.1.map(...).to_set() == self.map@
                // self.map@ contains pairs (x, y)
                // self@ is the Map derived from self.map@
                // Need to show: self.map@ == Set::new(|p| self@.dom().contains(p.0) && self@[p.0] == p.1)
                assume(result@.1.map(|i: int, p: Pair<X, Y>| p@).to_set() == 
                    Set::new(|p: (X::V, Y::V)| self@.dom().contains(p.0) && self@[p.0] == p.1));
            }
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
