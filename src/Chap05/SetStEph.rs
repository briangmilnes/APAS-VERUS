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

    broadcast use vstd::seq_lib::group_seq_properties;

    pub open spec fn valid_key_type<T: View>() -> bool {
        &&& obeys_key_model::<T>() &&& forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SetStEph<T: StT + Hash> { 
        pub elements: HashSetWithViewPlus<T>,
    }

    pub trait SetStEphTrait<T: StT + Hash + Clone + vstd::view::View> : vstd::view::View<V = Set<<T as vstd::view::View>::V>> + Sized {
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> Self
            requires valid_key_type::<T>();

        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> Self
            requires valid_key_type::<T>();

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T) -> (result: B)
            requires valid_key_type::<T>()
            ensures result == self@.contains(x@);

        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, other: &SetStEph<T>) -> (result: Self)
            requires valid_key_type::<T>()
            ensures result@ == self@.union(other@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T) -> (result: bool)
            requires valid_key_type::<T>()
            ensures
                self@ == old(self)@.insert(x@),
                result == !old(self)@.contains(x@);
        
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>)
            requires valid_key_type::<T>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: T| k@).to_set() == self@,
                it@.1.no_duplicates();
    }

    impl<T: StT + Hash> vstd::view::View for SetStEph<T> {
        type V = Set<<T as vstd::view::View>::V>;
        open spec fn view(&self) -> Self::V { 
            self.elements@ 
        }
    }

    impl<T: StT + Hash> Clone for SetStEph<T> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        { 
            SetStEph { elements: self.elements.clone() }
        }
    }

    impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
        fn empty() -> SetStEph<T> { 
            SetStEph { elements: HashSetWithViewPlus::new() } 
        }

        fn singleton(x: T) -> SetStEph<T> {
            let mut s = HashSetWithViewPlus::new();
            let _ = s.insert(x);
            SetStEph { elements: s }
        }

        fn size(&self) -> (result: N)
            ensures result == self@.len()
        { 
            self.elements.len() 
        }

        fn mem(&self, x: &T) -> (result: B) { 
            self.elements.contains(x) 
        }

        fn union(&self, other: &SetStEph<T>) -> (result: SetStEph<T>)
            ensures result@ == self@.union(other@)
        {
            let mut out = self.clone();
            
            for x in other.elements.iter() {
                let _ = out.elements.insert(x.clone());
            }
            
            proof {
                // With vstd's HashSet specs, for-loop auto-invariants handle the iteration.
                // Still need to prove iterating and inserting all elements gives union.
                assume(out@ == self@.union(other@));
            }
            
            out
        }

        fn insert(&mut self, x: T) -> (result: bool) { self.elements.insert(x) }
        
        fn iter<'a>(&'a self) -> (it: std::collections::hash_set::Iter<'a, T>) {
            let it = self.elements.iter();
            assume(it@.0 == 0int);
            assume(it@.1.map(|i: int, k: T| k@).to_set() == self@);
            assume(it@.1.no_duplicates());
            it
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
