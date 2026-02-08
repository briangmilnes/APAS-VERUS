// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Set built on `std::collections::HashSet` as wrapped by vstd and vstdplus.

pub mod SetStEph {

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
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::strictly_cloned;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;

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

    pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
        &&& obeys_key_model::<T>() 
        &&& obeys_feq_full::<T>()
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SetStEph<T: StT + Hash> { 
        pub elements: HashSetWithViewPlus<T>  // Public for open spec fn view()
    }

    pub trait SetStEphTrait<T: StT + Hash> : View<V = Set<<T as View>::V>> + Sized {

        /// A set is finite.
        open spec fn spec_finite(&self) -> bool {
            self@.finite()
        }

        /// APAS: Work Θ(|v|), Span Θ(1)
        fn from_vec(v: Vec<T>) -> (s: SetStEph<T>)
            requires valid_key_type::<T>()
            ensures s@.finite(), s@ == v@.map(|i: int, x: T| x@).to_set();

        /// APAS: Work Θ(1), Span Θ(1)
        fn iter<'a>(&'a self) -> (it: SetStEphIter<'a, T>)
            requires valid_key_type::<T>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: T| k@).to_set() == self@,
                it@.1.no_duplicates();

        fn to_seq(&self) -> (seq: Vec<T>)
            requires valid_key_type::<T>()
            ensures
                seq@.no_duplicates(),
                forall |x: T::V| self@.contains(x) <==> seq@.map(|_i: int, t: T| t@).contains(x);

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty()                           -> (empty: Self)
            requires valid_key_type::<T>()
            ensures empty@.finite(), empty@ == Set::<<T as View>::V>::empty();

        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                   -> (s: Self)
            requires valid_key_type::<T>()
            ensures s@.finite(), s@ == Set::empty().insert(x@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn size(&self)                       -> (size: N)
            ensures size == self@.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T)                 -> (contains: B)
            requires valid_key_type::<T>()
            ensures contains == self@.contains(x@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T)           -> (inserted: bool)
            requires valid_key_type::<T>()
            ensures
                self@ == old(self)@.insert(x@),
                inserted == !old(self)@.contains(x@);

        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, s2: &SetStEph<T>) -> (union: Self)
            requires 
               valid_key_type::<T>(),
            ensures union@.finite(), union@ == self@.union(s2@);

        /// - Disjoint union: union of two sets known to be disjoint.
        /// - APAS: Work Θ(|a| + |b|), Span Θ(1)
        fn disjoint_union(&self, s2: &SetStEph<T>) -> (union: Self)
            requires 
               valid_key_type::<T>(),
               self@.disjoint(s2@),
            ensures 
               union@.finite(),
               union@ == self@.union(s2@),
               union@.len() == self@.len() + s2@.len();

        /// - APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, s2: &SetStEph<T>) -> (intersection: Self)
            requires valid_key_type::<T>()
            ensures intersection@.finite(), intersection@ == self@.intersect(s2@);

        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
            requires 
              valid_key_type::<T>(),
              valid_key_type::<U>(),
              valid_key_type::<Pair<T, U>>(),
            ensures  
               product@.finite(),
               forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (av == a@ && s2@.contains(bv));

        /// - APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn cartesian_product<U: StT + Hash + Clone>(&self, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<U>(),
                valid_key_type::<Pair<T, U>>(),
            ensures  
                product@.finite(),
                forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv));

        fn all_nonempty(parts: &SetStEph<SetStEph<T>>) -> (all_nonempty: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetStEph<T>>(),
            ensures 
                all_nonempty <==> forall |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) ==> s.len() != 0;

        fn partition_on_elt(x: &T, parts: &SetStEph<SetStEph<T>>) -> (partition_on_elt: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetStEph<T>>(),
            ensures 
                partition_on_elt <==> (
                    (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x@)) &&
                    (forall |s1: Set<T::V>, s2: Set<T::V>|
                        #![trigger parts@.contains(s1), parts@.contains(s2)]
                        parts@.contains(s1) && s1.contains(x@) &&
                        parts@.contains(s2) && s2.contains(x@) ==> s1 == s2)
                );

        /// - APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> (partition: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetStEph<T>>(),
            ensures 
                partition <==> (
                    (forall |x: T::V| self@.contains(x) ==> (
                        (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x)) &&
                        (forall |s1: Set<T::V>, s2: Set<T::V>|
                            #![trigger parts@.contains(s1), parts@.contains(s2)]
                            parts@.contains(s1) && s1.contains(x) &&
                            parts@.contains(s2) && s2.contains(x) ==> s1 == s2)
                    )) &&
                    (forall |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) ==> s.len() != 0)
                );

        /// - Split a set into two parts: the first with n elements, the second with the rest.
        /// - APAS: Work Θ(|self|), Span Θ(1)
        fn split(&self, n: usize) -> (n_set_rest_set: (SetStEph<T>, SetStEph<T>))
            requires 
                valid_key_type::<T>(),
                self@.len() >= n,
            ensures 
               ({let (n_set, rest_set) = n_set_rest_set;
                  &&& n_set@.finite()
                  &&& rest_set@.finite()
                  &&& n_set@.disjoint(rest_set@)
                  &&& n_set@.union(rest_set@) == self@
                  &&& n_set@.len() == n
                  &&& rest_set@.len() == self@.len() - n
               });

        /// - Choose an arbitrary element from a non-empty set.
        /// - Matches vstd Set::choose() spec.
        /// - APAS: Work Θ(1), Span Θ(1)
        fn choose(&self) -> (element: T)
            requires 
                valid_key_type::<T>(),
                self@.len() > 0,
            ensures 
                self@.contains(element@);
    }


    /// - Iterator wrapper with CLOSED spec view for encapsulation.
    /// - Inner is private; closed view() can access it but external code cannot see it.
    #[verifier::reject_recursive_types(T)]
    pub struct SetStEphIter<'a, T: StT + Hash> {
        inner: std::collections::hash_set::Iter<'a, T>,  // PRIVATE
    }

    impl<'a, T: StT + Hash> View for SetStEphIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { 
            self.inner@
        }
    }

    pub open spec fn iter_invariant<'a, T: StT + Hash>(it: &SetStEphIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T: StT + Hash> std::iter::Iterator for SetStEphIter<'a, T> {
        type Item = &'a T;

        // Relies on vstd's assume_specification for hash_set::Iter::next
        // which provides the same postcondition we need here.
        fn next(&mut self) -> (next: Option<&'a T>)
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

    /// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
    #[verifier::reject_recursive_types(T)]
    pub struct SetStEphGhostIterator<'a, T: StT + Hash> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for SetStEphIter<'a, T> {
        type GhostIter = SetStEphGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> SetStEphGhostIterator<'a, T> {
            SetStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIterator for SetStEphGhostIterator<'a, T> {
        type ExecIter = SetStEphIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SetStEphIter<'a, T>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &SetStEphIter<'a, T>) -> SetStEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Hash> View for SetStEphGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    impl<T: StT + Hash> View for SetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.elements@ }
    }

    impl<T: StT + Hash> Clone for SetStEph<T> {
        fn clone(&self) -> (clone: Self)
            ensures clone@.finite(), clone@ == self@
        { SetStEph { elements: self.elements.clone() } }
    }

    impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {

        fn from_vec(v: Vec<T>) -> SetStEph<T> {
            let mut s: SetStEph<T> = SetStEph::empty();
            let ghost v_seq: Seq<T> = v@;
            
            for x in iter: v
                invariant
                    valid_key_type::<T>(),
                    iter.elements == v_seq,
                    s@ == v_seq.take(iter.pos).map(|idx: int, t: T| t@).to_set(),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(v_seq, iter.pos); }
                let x_clone: T = x.clone_plus();
                let _ = s.insert(x_clone);
            }
            s
        }

        fn iter<'a>(&'a self) -> (it: SetStEphIter<'a, T>) {
            let inner = self.elements.iter();
            proof { lemma_seq_map_to_set_equality(inner@.1, self@); }
            SetStEphIter { inner }
        }

        fn to_seq(&self) -> (seq: Vec<T>) {
            let mut seq: Vec<T> = Vec::new();
            let it = self.iter();
            let ghost iter_seq = it@.1;
            
            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == iter_seq,
                    iter_seq.map(|_i: int, k: T| k@).to_set() == self@,
                    iter_seq.no_duplicates(),
                    seq@ == iter_seq.take(iter.pos),
            {
                seq.push(x.clone_plus());
            }
            seq
        }

        fn empty() -> SetStEph<T> { SetStEph { elements: HashSetWithViewPlus::new() } }

        fn singleton(x: T) -> (s: SetStEph<T>) {
            let mut s = HashSetWithViewPlus::new();
            let _ = s.insert(x);
            SetStEph { elements: s }
        }

        fn size(&self) -> (size: N)
            ensures size == self@.len()
        { self.elements.len() }

        fn mem(&self, x: &T) -> (contains: B) { self.elements.contains(x) }

        fn insert(&mut self, x: T) -> (inserted: bool)
        { self.elements.insert(x) }

        fn union(&self, s2: &SetStEph<T>) -> (union: SetStEph<T>)
        {
            let mut union = self.clone_plus();
            let it = s2.iter();
            let ghost s1_view = self@;
            let ghost s2_seq = it@.1;

            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == s2_seq,
                    s2_seq.map(|i: int, k: T| k@).to_set() == s2@,
                    union@ == s1_view.union(s2_seq.take(iter.pos).map(|i: int, k: T| k@).to_set()),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, iter.pos); }
                let x_clone = x.clone_plus();
                let _ = union.insert(x_clone);
            }
            union
        }

        fn disjoint_union(&self, s2: &SetStEph<T>) -> (union: SetStEph<T>)
        {
            let capacity = self.size().saturating_add(s2.size());
            let mut union: SetStEph<T> = SetStEph { 
                elements: HashSetWithViewPlus::with_capacity(capacity) 
            };
            
            // Insert all elements from self.
            let it1 = self.iter();
            let ghost it1_seq = it1@.1;
            
            for x in iter1: it1
                invariant
                    valid_key_type::<T>(),
                    iter1.elements == it1_seq,
                    it1_seq.map(|i: int, k: T| k@).to_set() == self@,
                    union@ == it1_seq.take(iter1.pos).map(|i: int, k: T| k@).to_set(),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(it1_seq, iter1.pos); }
                let _ = union.insert(x.clone_plus());
            }
            
            // Insert all elements from s2 (guaranteed no duplicates due to disjointness).
            let it2 = s2.iter();
            let ghost it2_seq = it2@.1;
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            
            for x in iter2: it2
                invariant
                    valid_key_type::<T>(),
                    iter2.elements == it2_seq,
                    it2_seq.map(|i: int, k: T| k@).to_set() == s2_view,
                    s1_view.disjoint(s2_view),
                    s1_view == self@,
                    union@ == s1_view.union(it2_seq.take(iter2.pos).map(|i: int, k: T| k@).to_set()),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(it2_seq, iter2.pos); }
                let _ = union.insert(x.clone_plus());
            }
            
            proof {
                vstd::set_lib::lemma_set_disjoint_lens(self@, s2@);
            }
            
            union
        }

        fn intersection(&self, s2: &SetStEph<T>) -> (intersection: SetStEph<T>)
        {
            let mut intersection = SetStEph::empty();
            let it = self.iter();
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            let ghost s1_seq = it@.1;

            for s1mem in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    s2_view == s2@,
                    intersection@ == s1_seq.take(iter.pos).map(|i: int, k: T| k@).to_set().intersect(s2_view),
                    // Current element identity
                    iter.pos < iter.elements.len() ==> s1mem == iter.elements[iter.pos as int],
            {
                proof { lemma_take_one_more_intersect(s1_seq, s2_view, iter.pos); }
                
                if s2.mem(s1mem) {
                    let s1mem_clone = s1mem.clone_plus();
                    let _ = intersection.insert(s1mem_clone);
                } 
            }
            
            intersection
        }
        
        fn cartesian_product<U: StT + Hash + Clone>(&self, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
        {
            let mut product = SetStEph::empty();
            let it = self.iter();
            let ghost s1_seq = it@.1;
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            
            for a in iter: it
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    iter.elements == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    s2_view == s2@,
                    forall |av: T::V, bv: U::V| 
                        product@.contains((av, bv)) <==>
                            (s1_seq.take(iter.pos).map(|i: int, k: T| k@).to_set().contains(av) && s2_view.contains(bv)),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s1_seq, iter.pos); }
                let a_cross = Self::elt_cross_set(a, s2);
                product = product.union(&a_cross);
            }
            product
        }

        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
        {
            let mut product = SetStEph::empty();
            let it = s2.iter();
            let ghost s2_seq = it@.1;
            let ghost s2_view = s2@;
            let ghost a_view = a@;
            
            for b in iter: it
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    a_view == a@,
                    iter.elements == s2_seq,
                    s2_seq.map(|i: int, k: U| k@).to_set() == s2_view,
                    forall |av: T::V, bv: U::V| 
                        #![trigger product@.contains((av, bv))]
                        product@.contains((av, bv)) <==>
                        (av == a_view && s2_seq.take(iter.pos).map(|i: int, k: U| k@).to_set().contains(bv)),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, iter.pos); }
                let a_clone = a.clone_plus();
                let b_clone = b.clone_plus();
                let _ = product.insert(Pair(a_clone, b_clone));
            }
            
            product
        }

        // NOTE: Kept as loop-loop due to early return + postcondition proving
        fn all_nonempty(parts: &SetStEph<SetStEph<T>>) -> bool {
            let parts_iter       = parts.iter();
            let mut parts_it     = parts_iter;
            let ghost parts_seq  = parts_it@.1;
            let ghost parts_view = parts@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetStEph<T>>(),
                    parts_it@.0 <= parts_seq.len(),
                    parts_it@.1 == parts_seq,
                    parts_seq.map(|i: int, k: SetStEph<T>| k@).to_set() == parts_view,
                    forall |i: int| #![trigger parts_seq[i]] 0 <= i < parts_it@.0 ==> parts_seq[i]@.len() != 0,
                decreases parts_seq.len() - parts_it@.0,
            {
                let ghost old_pos = parts_it@.0;
                match parts_it.next() {
                    Some(subset) => {
                        if subset.size() == 0 {
                            proof {
                                lemma_seq_index_in_map_to_set(parts_seq, old_pos);
                            }
                            return false;
                        }
                    },
                    None => {
                        return true;
                    }
                }
            }
        }


        fn partition_on_elt(x: &T, parts: &SetStEph<SetStEph<T>>) -> bool {
            let parts_iter = parts.iter();
            let mut parts_it = parts_iter;
            let ghost parts_seq = parts_it@.1;
            let ghost parts_view = parts@;
            let ghost x_view = x@;
            let mut count: N = 0;
            let ghost mut found_index: Option<int> = None;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetStEph<T>>(),
                    parts_it@.0 <= parts_seq.len(),
                    parts_it@.1 == parts_seq,
                    parts_seq.map(|i: int, k: SetStEph<T>| k@).to_set() == parts_view,
                    count <= 1,
                    match found_index {
                        Some(idx) => 0 <= idx < parts_it@.0 && parts_seq[idx]@.contains(x_view) && count == 1,
                        None => count == 0,
                    },
                    forall |i: int| #![trigger parts_seq[i]] 0 <= i < parts_it@.0 && parts_seq[i]@.contains(x_view) ==> 
                        found_index == Some(i),
                decreases parts_seq.len() - parts_it@.0,
            {
                let ghost old_pos = parts_it@.0;
                match parts_it.next() {
                    Some(subset) => {
                        if subset.mem(x) {
                            let ghost prev_found_index = found_index;
                            count = count + 1;
                            proof {
                                found_index = Some(old_pos);
                            }
                            if count > 1 {
                                proof {
                                    let prev_idx = match prev_found_index { Some(i) => i, None => arbitrary() };
                                    lemma_seq_index_in_map_to_set(parts_seq, prev_idx);
                                    lemma_seq_index_in_map_to_set(parts_seq, old_pos);
                                }
                                return false;
                            }
                        }
                    },
                    None => {
                        if count == 0 {
                            return false;
                        } else {
                          proof {
                                let idx = match found_index { Some(i) => i, None => arbitrary() };
                                lemma_seq_index_in_map_to_set(parts_seq, idx);
                            }
                            return true;
                        }
                    }
                }
            }
        }

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> bool {
            // First, check if all parts are non-empty.
            if !Self::all_nonempty(parts) {
                return false;
            }
            
            let s1_iter = self.iter();
            let mut s1_it = s1_iter;
            let ghost s1_seq = s1_it@.1;
            let ghost s1_view = self@;
            let ghost parts_view = parts@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetStEph<T>>(),
                    s1_it@.0 <= s1_seq.len(),
                    s1_it@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    forall |i: int| #![trigger s1_seq[i]] 0 <= i < s1_it@.0 ==> {
                        let x_view = s1_seq[i]@;
                        (exists |s: Set<T::V>| #![trigger parts_view.contains(s)] parts_view.contains(s) && s.contains(x_view)) &&
                        (forall |s1: Set<T::V>, s2: Set<T::V>| 
                            #![trigger parts_view.contains(s1), parts_view.contains(s2)]
                            parts_view.contains(s1) && s1.contains(x_view) &&
                            parts_view.contains(s2) && s2.contains(x_view) ==> s1 == s2)
                    },
                decreases s1_seq.len() - s1_it@.0,
            {
                let ghost old_pos = s1_it@.0;
                match s1_it.next() {
                    Some(x) => {
                        if !Self::partition_on_elt(x, parts) {
                            proof {
                                lemma_seq_index_in_map_to_set(s1_seq, old_pos);
                            }
                            return false;
                        }
                    },
                    None => {
                        return true;
                    }
                }
            }
        }

        fn split(&self, n: usize) -> (n_set_rest_set: (SetStEph<T>, SetStEph<T>)) {
            let mut first : SetStEph<T> = SetStEph::empty();
            let mut second: SetStEph<T> = SetStEph::empty();
            let it = self.iter();
            let ghost iter_seq = it@.1;
            let ghost self_view = self@;
            
            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == iter_seq,
                    iter_seq.map(|_i: int, k: T| k@).to_set() == self_view,
                    iter_seq.no_duplicates(),
                    first@.finite(),
                    second@.finite(),
                    first@.disjoint(second@),
                    first@.union(second@) == iter_seq.take(iter.pos).map(|_i: int, k: T| k@).to_set(),
                    first@.len() == if iter.pos <= n { iter.pos } else { n as int },
                    second@.len() == if iter.pos <= n { 0 } else { iter.pos - n },
            {
                proof { 
                    lemma_take_one_more_extends_the_seq_set_with_view(iter_seq, iter.pos);
                    assert(!iter_seq.take(iter.pos).contains(*x)) by {
                        if iter_seq.take(iter.pos).contains(*x) {
                            let j = choose |j: int| 0 <= j < iter.pos && iter_seq.take(iter.pos)[j] == *x;
                        }
                    };
                    assert(!iter_seq.take(iter.pos).map(|_i: int, k: T| k@).to_set().contains(x@)) by {
                        if iter_seq.take(iter.pos).map(|_i: int, k: T| k@).to_set().contains(x@) {
                            let mapped = iter_seq.take(iter.pos).map(|_i: int, k: T| k@);
                            let j = mapped.lemma_contains_to_index(x@);
                        }
                    };
                }
                
                let x_clone = x.clone_plus();
                if first.size() < n {
                    first.insert(x_clone);
                } else {
                    second.insert(x_clone);
                }
            }
            
            (first, second)
        }

        fn choose(&self) -> (element: T) {
            use crate::vstdplus::feq::feq::*;
            
            let mut it = self.elements.iter();
            let ghost s: Seq<T> = it@.1;
            
            proof {
                // s.len() > 0 because self@.len() > 0 and iter ensures bijection
                assert(s.len() > 0) by {
                    if s.len() == 0 {
                        // Contradiction: self@ is non-empty but s is empty
                    }
                }
            }
            
            let opt = it.next();
            let element_ref: &T = opt.unwrap();
            
            proof {
                // next() ensures element_ref == s[0]
                // Since 0 < s.len(), s.contains(element_ref)
                assert(s.contains(*element_ref)) by {
                    assert(s[0] == *element_ref);
                    assert(0 <= 0 < s.len());
                }
                // From iter ensures: s.contains(k) ==> self@.contains(k@)
                assert(self@.contains(element_ref@));
            }
            
            let result = element_ref.clone_plus();
            proof {
                lemma_cloned_view_eq(*element_ref, result);
            }
            result
        }
    }

    /// Singleton choose: if len == 1 and contains(a), then choose() == a.
    pub broadcast proof fn lemma_singleton_choose<A>(s: Set<A>, a: A)
        requires
            s.finite(),
            s.len() == 1,
            #[trigger] s.contains(a),
        ensures
            s.choose() == a,
    {
        Set::lemma_is_singleton(s);
    }

    pub broadcast group group_set_st_eph_lemmas {
        lemma_singleton_choose,
    }

    impl<T: StT + Hash> std::hash::Hash for SetStEph<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.elements.hash(state); }
    }

    impl<T: StT + Hash> PartialEqSpecImpl for SetStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Hash> Eq for SetStEph<T> {}

    impl<T: StT + Hash> PartialEq for SetStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.elements == other.elements;
            // HashSetWithView* eq is external_body so we have to trust it here.
            proof { assume(equal == (self@ == other@)); }
            equal
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

     impl<T: StT + Hash> std::fmt::Display for SetStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Set({})", self.elements.len())
        }
    }

    impl<T: StT + Hash> std::fmt::Debug for SetStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetStEph({})", self.elements.len())
        }
    }

}
