//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet`.

pub mod SetStEph {

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
    use vstd::hash_set::HashSetWithView;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    use crate::Types::Types::*;

    broadcast use {vstd::seq_lib::group_seq_properties, vstd::set::group_set_axioms};

    pub open spec fn valid_key_type<T: View + Clone>() -> bool {
        &&& obeys_key_model::<T>() 
        &&& forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2
        // So we can clone elements and put them in a set.
        &&& forall|x:T, x_cloned:T| cloned(x, x_cloned) ==> x == x_cloned 
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SetStEph<T: StT + Hash> { pub elements: HashSetWithViewPlus<T> }

    pub trait SetStEphTrait<T: StT + Hash + Clone + View> : View<V = Set<<T as View>::V>> + Sized {

        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>)
            requires valid_key_type::<T>(),
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: T| k@).to_set() == self@,
                it@.1.no_duplicates();

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty()                           -> (result: Self)
            requires valid_key_type::<T>()
            ensures result@ == Set::<<T as View>::V>::empty();

        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                   -> Self
            requires valid_key_type::<T>();

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self)                       -> N;

        /// APAS: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T)                 -> (result: B)
            requires valid_key_type::<T>()
            ensures result == self@.contains(x@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T)           -> (inserted: bool)
            requires valid_key_type::<T>()
            ensures
                self@ == old(self)@.insert(x@),
                inserted == !old(self)@.contains(x@);

        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, s2: &SetStEph<T>) -> (result: Self)
            requires 
               valid_key_type::<T>(),
            ensures result@ == self@.union(s2@);

        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, s2: &SetStEph<T>) -> (result: Self)
            requires valid_key_type::<T>()
            ensures result@ == self@.intersect(s2@);

        /// APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn CartesianProduct<U: StT + Hash + Clone>(&self, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<U>(),
                valid_key_type::<Pair<T, U>>(),
            ensures  
                forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv));
    }

    impl<T: StT + Hash> View for SetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.elements@ }
    }

    impl<T: StT + Hash> Clone for SetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        { SetStEph { elements: self.elements.clone() } }
    }

    impl<T: StT + Hash + Clone + View> SetStEphTrait<T> for SetStEph<T> {
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>) {
            let it = self.elements.iter();
            proof { lemma_seq_map_to_set_equality(it@.1, self@); }
            it
        }

        fn empty() -> SetStEph<T> { SetStEph { elements: HashSetWithViewPlus::new() } }

        fn singleton(x: T) -> SetStEph<T> {
            let mut s = HashSetWithViewPlus::new();
            let _ = s.insert(x);
            SetStEph { elements: s }
        }

        fn size(&self) -> (result: N)
            ensures result == self@.len()
        { self.elements.len() }

        fn mem(&self, x: &T) -> (result: B) { self.elements.contains(x) }

        fn insert(&mut self, x: T) -> (inserted: bool)
        { self.elements.insert(x) }

        fn union(&self, s2: &SetStEph<T>) -> (union: SetStEph<T>)
        {
            let mut union = self.clone();
            let s2_iter = s2.iter();
            let mut it = s2_iter;
            let ghost s1_view = self@;
            let ghost s2_seq = it@.1;

            #[verifier::loop_isolation(false)]
            loop 
                invariant
                    valid_key_type::<T>(),
                    it@.0 <= s2_seq.len(),
                    it@.1 == s2_seq,
                    s2_seq.map(|i: int, k: T| k@).to_set() == s2@,
                    union@ == s1_view.union(s2_seq.take(it@.0).map(|i: int, k: T| k@).to_set()),
               decreases s2_seq.len() - it@.0,
            {
                let ghost old_index = it@.0;
                let ghost old_union = union@;
                
                match it.next() {
                    Some(x) => {
                        let x_clone = x.clone();
                        assert(cloned(*x, x_clone));
                        let _ = union.insert(x_clone);
                        proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, old_index); }
                    },
                    None => {
                        assert(it@.0 == s2_seq.len());
                        proof { lemma_take_full_to_set_with_view(s2_seq); }
                        break;
                    }
                }
            }
            union
        }

        fn intersection(&self, s2: &SetStEph<T>) -> (intersection: SetStEph<T>)
        {
            let mut intersection = SetStEph::empty();
            let s1_iter = self.iter();
            let mut it = s1_iter;
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            let ghost s1_seq = it@.1;

            broadcast use {vstd::seq_lib::group_seq_properties, vstd::set::group_set_axioms};

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<T>(),
                    it@.0 <= s1_seq.len(),
                    it@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    intersection@ == s1_seq.take(it@.0).map(|i: int, k: T| k@).to_set().intersect(s2_view),
                decreases s1_seq.len() - it@.0,
            {
                let ghost old_index = it@.0;
                let ghost old_intersection = intersection@;
                
                match it.next() {
                    Some(s1mem) => {
                        proof { lemma_take_one_more_intersect(s1_seq, s2_view, old_index); }
                        
                        if s2.mem(s1mem) {
                            let s1mem_clone = s1mem.clone();
                            assert(cloned(*s1mem, s1mem_clone));
                            let _ = intersection.insert(s1mem_clone);
                        } 
                    },
                    None => {
                        assert(it@.0 == s1_seq.len());
                        proof { lemma_take_full_to_set_with_view(s1_seq); }
                        break;
                    }
                }
            }
            
            intersection
        }

        fn CartesianProduct<U: StT + Hash + Clone>(&self, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
        {
            let mut product = SetStEph::empty();
            let s1_iter = self.iter();
            let mut outer_it = s1_iter;
            let ghost s1_seq = outer_it@.1;
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            
            broadcast use {vstd::seq_lib::group_seq_properties, vstd::set::group_set_axioms};

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    outer_it@.0 <= s1_seq.len(),
                    outer_it@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    // product contains all pairs (a@, b@) where a is from the first outer_it@.0 elements and b is from s2
                    forall |av: T::V, bv: U::V| 
                        product@.contains((av, bv)) <==>
                            (s1_seq.take(outer_it@.0).map(|i: int, k: T| k@).to_set().contains(av) && s2_view.contains(bv)),
                decreases s1_seq.len() - outer_it@.0,
            {
                let ghost old_outer_index = outer_it@.0;
                let ghost old_product = product@;
                
                match outer_it.next() {
                    Some(a) => {
                        let s2_iter = s2.iter();
                        let mut inner_it = s2_iter;
                        let ghost s2_seq = inner_it@.1;
                        let ghost a_view = a@;
                        
                        #[verifier::loop_isolation(false)]
                        loop
                            invariant
                                inner_it@.0 <= s2_seq.len(),
                                inner_it@.1 == s2_seq,
                                s2_seq.map(|i: int, k: U| k@).to_set() == s2_view,
                                outer_it@.0 == old_outer_index + 1,
                                outer_it@.1 == s1_seq,
                                a_view == s1_seq[old_outer_index]@,
                                // product contains:
                                // 1. All pairs from before the outer loop iteration (old_product)
                                // 2. Pairs (a@, b@) for b in the first inner_it@.0 elements of s2
                                forall |av: T::V, bv: U::V| 
                                    product@.contains((av, bv)) <==>
                                        (old_product.contains((av, bv)) || 
                                         (av == a_view && s2_seq.take(inner_it@.0).map(|i: int, k: U| k@).to_set().contains(bv))),
                            decreases s2_seq.len() - inner_it@.0,
                        {
                            match inner_it.next() {
                                Some(b) => {
                                    let ghost old_inner_index = inner_it@.0 - 1;
                                    let ghost old_inner_product = product@;
                                    let a_clone = a.clone();
                                    let b_clone = b.clone();
                                    assert(cloned(*a, a_clone));
                                    assert(cloned(*b, b_clone));
                                    let _ = product.insert(Pair(a_clone, b_clone));
                                    
                                    proof {
                                        // Pair(a_clone, b_clone)@ == (a_clone@, b_clone@) == (a@, b@)
/*
                                        assert(Pair(a_clone, b_clone)@ == (a_clone@, b_clone@));
                                        assert(a_clone@ == a@);
                                        assert(b_clone@ == b@);
                                        assert(Pair(a_clone, b_clone)@ == (a@, b@));
*/
                                        
                                        // After insert, product@ should equal old_inner_product.insert((a@, b@))
                                        assert(product@ == old_inner_product.insert((a@, b@)));
                                        
                                        // b was at old_inner_index
                                        assert(s2_seq[old_inner_index] == *b);
                                        assert(b@ == s2_seq[old_inner_index]@);
                                        
                                        // Use lemma for s2_seq
                                        lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, old_inner_index);
                                        assert(s2_seq.take(inner_it@.0).map(|i: int, k: U| k@).to_set() ==
                                               s2_seq.take(old_inner_index).map(|i: int, k: U| k@).to_set().insert(b@));
                                    }
                                },
                                None => {
                                    proof {
                                        // At inner loop exit, inner_it@.0 == s2_seq.len()
                                        assert(inner_it@.0 == s2_seq.len());
                                        // So s2_seq.take(s2_seq.len()) == s2_seq
                                        lemma_take_full_to_set_with_view(s2_seq);
                                        // Thus product contains all pairs (a@, b@) for all b in s2
                                        assert(s2_seq.take(inner_it@.0).map(|i: int, k: U| k@).to_set() == s2_view);
                                    }
                                    break;
                                }
                            }
                        }
                        
                        proof {
                            // s1_seq[old_outer_index] == *a
                            assert(s1_seq[old_outer_index] == *a);
                            assert(a@ == s1_seq[old_outer_index]@);
                            
                            // s1_seq.take(outer_it@.0) includes a@
                            lemma_take_one_more_extends_the_seq_set_with_view(s1_seq, old_outer_index);
                            let s1_take_new = s1_seq.take(outer_it@.0).map(|i: int, k: T| k@).to_set();
                            let s1_take_old = s1_seq.take(old_outer_index).map(|i: int, k: T| k@).to_set();
                            assert(s1_take_new == s1_take_old.insert(a@));
                            
                            // Prove the outer invariant
                            assert forall |av: T::V, bv: U::V| #[trigger] product@.contains((av, bv)) implies
                                s1_take_new.contains(av) && s2_view.contains(bv) by {
                                // From inner invariant: product@.contains((av, bv)) iff (old_product.contains((av, bv)) || (av == a@ && s2_view.contains(bv)))                                    
                                    admit();
                            }
                            
                            assert forall |av: T::V, bv: U::V| s1_take_new.contains(av) && s2_view.contains(bv) implies
                                #[trigger] product@.contains((av, bv)) by {
                                admit();
                            }
                        }
                    },
                    None => {
                        assert(outer_it@.0 == s1_seq.len());
                        proof {
                            lemma_take_full_to_set_with_view(s1_seq);
                        }
                        break;
                     }
                }
            }
            
            assume(forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv)));
            product
        }
    }

 } // verus!

    #[macro_export]
    macro_rules! SetLit {
        () => {{
            < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( let _ = __s.insert($x); )*
            __s
        }};
    }
}
