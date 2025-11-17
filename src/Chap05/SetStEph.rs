//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet`.

pub mod SetStEph {

    use vstd::prelude::*;

verus! {

    use std::collections::HashSet;
    use std::collections::hash_set::Iter;
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

//    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    use crate::vstdplus::hash_set_specs::hash_set_specs::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::set_with_view::SetWithView::SetWithView;
    use crate::vstdplus::set_axioms::set_axioms::*;
    use crate::vstdplus::clone_view::clone_view::*;

    use crate::Types::Types::*;

    broadcast use {vstd::seq_lib::group_seq_properties, group_clone_view_axioms};

    pub open spec fn valid_key_type<T: View>() -> bool {
        &&& vstd::std_specs::hash::obeys_key_model::<T>()
        &&& forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SetStEph<T: StT + Hash> {
       pub data: HashSetWithViewPlus<T>,
    }

    pub trait SetStEphTrait<T: StT + Hash + Clone + vstd::view::View> : vstd::view::View<V = Set<<T as vstd::view::View>::V>> + Sized {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self
            requires valid_key_type::<T>();
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                                             -> Self
            requires valid_key_type::<T>();
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                                                 -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T)                                           -> (result: B)
            requires valid_key_type::<T>()
            ensures result == self@.contains(x@);

        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, other: &SetStEph<T>)                           -> (result: Self)
            requires valid_key_type::<T>();
        
        fn union_lowlevel(&self, other: &SetStEph<T>)                  -> (result: Self)
            requires valid_key_type::<T>();

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T)                                     -> (result: bool)
            requires valid_key_type::<T>()
            ensures
                self@ == old(self)@.insert(x@),
                result == !old(self)@.contains(x@);

/*
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, other: &SetStEph<T>)                    -> Self;
        /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &SetStEph<SetStEph<T>>)             -> B;

        /// APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        /// The bool is required here as Verus won't take a &mut Self return yet.
        fn insert(&mut self, x: T)                                     -> bool;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self)                                                 -> Iter<'_, T>;
        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<T>)                                          -> Self;
*/
    }

    impl<T: StT + Hash> View for SetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.data@ }
    }

    impl<T: StT + Hash> Clone for SetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        { SetStEph { data: self.data.clone() } }
    }

    // Custom iterator for SetStEph - wraps HashSet iterator
    // This allows us to add SetStEph-specific ghost state for verification
    pub struct SetStEphIter<'a, T: StT + Hash> {
        pub inner: std::collections::hash_set::Iter<'a, T>,
        // Could add SetStEph-specific fields here for future data structures
    }

    impl<T: StT + Hash> SetStEph<T> {
        pub fn iter(&self) -> (result: SetStEphIter<'_, T>)
            requires valid_key_type::<T>()
            ensures
                result@.0 == 0,
/*
                forall|i: int| #![auto] 0 <= i < result@.1.len() ==> self.data@.contains(result@.1[i]@),
                forall|kv: <T as vstd::view::View>::V| #![auto] self.data@.contains(kv) ==> 
                    exists|i: int| #![auto] 0 <= i < result@.1.len() && result@.1[i]@ == kv,
*/
        { SetStEphIter { inner: self.data.iter() } }
    }

    impl<'a, T: StT + Hash> View for SetStEphIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> Self::V { self.inner@ }
    }

    impl<'a, T: StT + Hash> Iterator for SetStEphIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (result: Option<Self::Item>)
            ensures
                ({
                    let (old_index, old_seq) = old(self).inner@;
                    match result {
                        None => {
                            &&& self.inner@ == old(self).inner@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self.inner@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    // Ghost iterator for SetStEph - enables for loop verification
    pub struct SetStEphIterGhostIterator<'a, T: StT + Hash> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: Option<&'a T>,
        // Could add SetStEph-specific ghost fields here
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for SetStEphIter<'a, T> {
        type GhostIter = SetStEphIterGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> SetStEphIterGhostIterator<'a, T> {
            SetStEphIterGhostIterator { 
                pos: self.inner@.0,
                elements: self.inner@.1, 
                phantom: None 
            }
        }
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIterator for SetStEphIterGhostIterator<'a, T> {
        type ExecIter = SetStEphIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SetStEphIter<'a, T>) -> bool {
            &&& self.pos == exec_iter.inner@.0
            &&& self.elements == exec_iter.inner@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool { self.pos == self.elements.len()}

        open spec fn ghost_decrease(&self) -> Option<int> { Some(self.elements.len() - self.pos)}

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &SetStEphIter<'a, T>) -> SetStEphIterGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Hash> View for SetStEphIterGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // Provide an order-independent Hash so sets of sets can be placed in a HashSet.

    impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
        fn empty() -> SetStEph<T> {
            SetStEph { data: HashSetWithViewPlus::new() }
        }

        fn singleton(x: T) -> SetStEph<T> {
            let mut s = HashSetWithViewPlus::new();
            let  _ = s.insert(x);
            SetStEph { data: s }
        }

        fn size(&self) -> (result: N)
            ensures result == self@.len()
        {
            self.data.len()
        }

        fn mem(&self, x: &T) -> (result: B)
        {
            self.data.contains(x)
        }

        fn union_lowlevel(&self, s2: &SetStEph<T>) -> (result: SetStEph<T>)
            ensures gresult@ == self@.union(s2@)
        {
            let ghost self_at : Set<<T as View>::V> = self@;

            let mut out: SetStEph<T> = self.clone();

            let iter                : SetStEphIter<'_, T> = s2.iter();
            let ghost s2_seq        : Seq<T>              = iter@.1;                       // Seq of s2 elements.
            let ghost s2_seq_mapped : Seq<<T as View>::V> = s2_seq.map(|i: int, x: T| x@); // With elements views.
            let ghost s2_seq_set    : Set<<T as View>::V> = s2_seq_mapped.to_set();        // As a set.
            let ghost s2_at         : Set<<T as View>::V> = s2@;
            assume(s2_seq_set == s2_at);

            for x in it: iter
                invariant
                    it.elements == s2_seq,                                     // is.elements is the full seq of elements to iterator over and is unchanging.
                    s2_seq.map(|i: int, x: T| x@).to_set() == s2@,             // And they are the same as s2@ at the view level.
                    self@ <= out@,                                             // out is increasing 
                    // it@ : Seq<T> = it.elements.take(it.pos), the subsequence of elements iterated so far.
                    // We then convert to a Seq<<T as View>::V>> then a set.
                    out@ == self@.union(it@.map(|i: int, x: T| x@).to_set()),
            {
                let x_clone = x.clone();
                proof {
//                    axiom_clone_preserves_view(x, &x_clone);
                    assert(x@ == x_clone@);
                }
                let _ = out.data.insert(x_clone);
            }
            out
        }

        fn union(&self, s2: &SetStEph<T>) -> (result: SetStEph<T>)
        {  self.clone()
/*
            let mut out = self.clone();
            let s2_iter = s2.iter();
            assert(s2_iter@.0 == 0);
            let ghost g_elements = s2_iter@.1;
            
            for x in it: s2_iter
                invariant
                    it.elements == g_elements,
                    valid_key_type::<T>(),
            {
                let _ = out.insert(x.clone());
            }
            
            out
*/
        }

        fn insert(&mut self, x: T) -> (result: bool)
        {
            let was_present = self.mem(&x);
            let _ = self.data.insert(x);
            !was_present
        }

 }/*

        fn intersection(&self, s2: &SetStEph<T>) -> SetStEph<T>
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len().min(s2.data.len()));
            for x in self.data.intersection(&s2.data) {
                let _ = out.insert(x.clone());
            }
            SetStEph { data: out }
        }

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B {
            for x in self.data.iter() {
                let mut count: N = 0;
                for subset in parts.data.iter() {
                    if subset.data.contains(x) {
                        count += 1;
                        if count > 1 {
                            return false;
                        }
                    }
                }
                if count == 0 {
                    return false;
                }
            }
            true
        }

        fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>
        where
            T: Clone,
        {
            let mut out = HashSet::<Pair<T, U>>::new();
            for a in self.data.iter() {
                for b in other.data.iter() {
                    let _ = out.insert(Pair(a.clone(), b.clone()));
                }
            }
            SetStEph { data: out }
        }

        fn insert(&mut self, x: T) -> &mut Self {
            let _ = self.data.insert(x);
            self
        }

        fn iter(&self) -> Iter<'_, T> { self.data.iter() }

        fn FromVec(v: Vec<T>) -> SetStEph<T> {
            let mut s = HashSet::with_capacity(v.len());
            for x in v {
                let _ = s.insert(x);
            }
            SetStEph { data: s }
        }
    }

    impl<T: Eq + Hash> PartialEq for SetStEph<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: Eq + Hash> Eq for SetStEph<T> {}

    impl<T: Eq + Hash + Debug> Debug for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { f.debug_set().entries(self.data.iter()).finish() }
    }
    impl<T: Eq + Hash + Display> Display for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            let mut first = true;
            for x in self.data.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "}}")
        }
    }

    impl<T: Eq + Hash> Hash for SetStEph<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            use std::collections::hash_map::DefaultHasher;
            let mut element_hashes = Vec::<u64>::with_capacity(self.data.len());
            for e in self.data.iter() {
                let mut h = DefaultHasher::new();
                e.hash(&mut h);
                element_hashes.push(h.finish());
            }
            element_hashes.sort_unstable();
            self.data.len().hash(state);
            for h in element_hashes {
                h.hash(state);
            }
        }
    }

*/

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
