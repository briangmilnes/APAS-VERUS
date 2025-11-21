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

        fn EltCrossSet<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
            requires 
              valid_key_type::<T>(),
              valid_key_type::<U>(),
              valid_key_type::<Pair<T, U>>(),
            ensures  
               forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (av == a@ && s2@.contains(bv));

        /// APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn CartesianProduct<U: StT + Hash + Clone>(&self, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<U>(),
                valid_key_type::<Pair<T, U>>(),
            ensures  
                forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv));

        /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> (partition: bool)
            requires valid_key_type::<T>()
            ensures 
                partition <==> forall |x: T::V| self@.contains(x) ==> (
                    (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                    (forall |s1: Set<T::V>, s2: Set<T::V>|
                        #![trigger parts@.contains(s1), parts@.contains(s2)]
                        parts@.contains(s1) && s1.contains(x) &&
                        parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
                );
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

    impl<T: StT + Hash + Clone + View + Eq> SetStEphTrait<T> for SetStEph<T> {
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
            let mut it = s1_iter;
            let ghost s1_seq = it@.1;
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    it@.0 <= s1_seq.len(),
                    it@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    forall |av: T::V, bv: U::V| 
                        product@.contains((av, bv)) <==>
                            (s1_seq.take(it@.0).map(|i: int, k: T| k@).to_set().contains(av) && s2_view.contains(bv)),
                decreases s1_seq.len() - it@.0,
            {
                let ghost old_index = it@.0;
                let ghost old_product = product@;
                
                match it.next() {
                    Some(a) => {
                        let ghost a_view = a@;
                        let a_cross = Self::EltCrossSet(a, s2);
                        product = product.union(&a_cross);
                        proof { lemma_take_one_more_extends_the_seq_set_with_view(s1_seq, old_index); }
                    },
                    None => {
                        proof { lemma_take_full_to_set_with_view(s1_seq); }
                        break;
                    }
                }
            }
            product
        }

        fn EltCrossSet<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
        {
            let mut product = SetStEph::empty();
            let s2_iter = s2.iter();
            let mut it = s2_iter;
            let ghost s2_seq = it@.1;
            let ghost s2_view = s2@;
            let ghost a_view = a@;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                it@.0 <= s2_seq.len(),
                it@.1 == s2_seq,
                s2_seq.map(|i: int, k: U| k@).to_set() == s2_view,
                forall |av: T::V, bv: U::V| 
                  #![trigger product@.contains((av, bv))]
                   product@.contains((av, bv)) <==>
                   (av == a_view && s2_seq.take(it@.0).map(|i: int, k: U| k@).to_set().contains(bv)),
            decreases s2_seq.len() - it@.0,
            {
                match it.next() {
                    Some(b) => {
                        let ghost old_index = it@.0 - 1;
                        let a_clone = a.clone();
                        let b_clone = b.clone();
                        assert(cloned(*a, a_clone));
                        assert(cloned(*b, b_clone));
                        let _ = product.insert(Pair(a_clone, b_clone));
                        proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, old_index); }
                    },
                    None => {
                        proof { lemma_take_full_to_set_with_view(s2_seq); }
                        break;
                    }
                }
            }
            
            product
        }

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> bool {
            let s1_iter = self.iter();
            let mut s1_it = s1_iter;
            let ghost s1_seq = s1_it@.1;
            let ghost s1_view = self@;
            let ghost parts_view = parts@;
            
            assume(valid_key_type::<SetStEph<T>>());

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<T>(),
                    s1_it@.0 <= s1_seq.len(),
                    s1_it@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    forall |x_processed: T::V| #![trigger s1_view.contains(x_processed)]
                        s1_seq.take(s1_it@.0).map(|i: int, k: T| k@).to_set().contains(x_processed) ==> (
                            (exists |s: Set<T::V>| #![trigger parts_view.contains(s)] parts_view.contains(s) && s.contains(x_processed)) &&
                            (forall |s1: Set<T::V>, s2: Set<T::V>| 
                                parts_view.contains(s1) && s1.contains(x_processed) &&
                                parts_view.contains(s2) && s2.contains(x_processed) ==> s1 == s2)
                        ),
                decreases s1_seq.len() - s1_it@.0,
            {
                let ghost old_s1_index = s1_it@.0;
                match s1_it.next() {
                    Some(x) => {
                        let ghost x_view = x@;
                        let parts_iter = parts.iter();
                        let mut parts_it = parts_iter;
                        let ghost parts_seq = parts_it@.1;
                        let mut count: N = 0;

                        #[verifier::loop_isolation(false)]
                        loop
                            invariant
                                valid_key_type::<T>(),
                                parts_it@.0 <= parts_seq.len(),
                                parts_it@.1 == parts_seq,
                                parts_seq.map(|i: int, k: SetStEph<T>| k@).to_set() == parts_view,
                                count == parts_seq.take(parts_it@.0).map(|i: int, k: SetStEph<T>| k@).filter(|s: Set<T::V>| s.contains(x_view)).len(),
                                count <= 1,
                            decreases parts_seq.len() - parts_it@.0,
                        {
                            match parts_it.next() {
                                Some(subset) => {
                                    let ghost old_parts_index = parts_it@.0 - 1;
                                    if subset.mem(x) {
                                        count = count + 1;
                                        assume(count == parts_seq.take(parts_it@.0).map(|i: int, k: SetStEph<T>| k@).filter(|s: Set<T::V>| s.contains(x_view)).len());
                                        if count > 1 {
                                            assume(!(forall |x: T::V| self@.contains(x) ==> (
                                                (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                                                (forall |s1: Set<T::V>, s2: Set<T::V>|
                                                    #![trigger parts@.contains(s1), parts@.contains(s2)]
                                                    parts@.contains(s1) && s1.contains(x) &&
                                                    parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
                                            )));
                                            return false;
                                        }
                                    } else {
                                        assume(count == parts_seq.take(parts_it@.0).map(|i: int, k: SetStEph<T>| k@).filter(|s: Set<T::V>| s.contains(x_view)).len());
                                    }
                                },
                                None => {
                                    break;
                                }
                            }
                        }
                        if count == 0 {
                            assume(!(forall |x: T::V| self@.contains(x) ==> (
                                (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                                (forall |s1: Set<T::V>, s2: Set<T::V>|
                                    #![trigger parts@.contains(s1), parts@.contains(s2)]
                                    parts@.contains(s1) && s1.contains(x) &&
                                    parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
                            )));
                            return false;
                        }
                    },
                    None => {
                        break;
                    }
                }
                proof {
                    admit();
                }
            }
            assume(forall |x: T::V| self@.contains(x) ==> (
                (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                (forall |s1: Set<T::V>, s2: Set<T::V>|
                    #![trigger parts@.contains(s1), parts@.contains(s2)]
                    parts@.contains(s1) && s1.contains(x) &&
                    parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
            ));
            true
        }

    }

    impl<T: StT + Hash> std::hash::Hash for SetStEph<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.elements.hash(state); }
    }

    impl<T: StT + Hash> Eq for SetStEph<T> {}

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
  } // verus!

    impl<T: StT + Hash> PartialEq for SetStEph<T> {
        fn eq(&self, other: &Self) -> bool { self.elements == other.elements }
    }

#[verifier::external]
impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Display for SetStEph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Set({})", self.elements.len())
    }
}

#[verifier::external]
impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Debug for SetStEph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SetStEph({})", self.elements.len())
    }
}

}
