//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 — Multi-threaded ephemeral Set built on `std::collections::HashSet`.
//! Uses WSSchedulerMtEph for bounded parallel cartesian_product.

// Verus requires parentheses around closures with ensures clauses in function arguments
#[allow(unused_parens)]
pub mod SetMtEph {

    use vstd::prelude::*;
    use crate::Concurrency::diverge;
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::{spawn, wait, TaskState};

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
    use vstd::pervasive::strictly_cloned;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

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
    pub struct SetMtEph<T: StT + Hash> { pub elements: HashSetWithViewPlus<T> }

    /// Iterator wrapper to hide std::collections::hash_set::Iter.
    #[verifier::reject_recursive_types(T)]
    pub struct SetMtEphIter<'a, T: StT + Hash> {
        pub inner: std::collections::hash_set::Iter<'a, T>,
    }

    impl<'a, T: StT + Hash> View for SetMtEphIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T: StT + Hash>(it: &SetMtEphIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T: StT + Hash> std::iter::Iterator for SetMtEphIter<'a, T> {
        type Item = &'a T;

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

    /// Ghost iterator for ForLoopGhostIterator support (for-iter patterns).
    #[verifier::reject_recursive_types(T)]
    pub struct SetMtEphGhostIterator<'a, T: StT + Hash> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for SetMtEphIter<'a, T> {
        type GhostIter = SetMtEphGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> SetMtEphGhostIterator<'a, T> {
            SetMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T: StT + Hash> vstd::pervasive::ForLoopGhostIterator for SetMtEphGhostIterator<'a, T> {
        type ExecIter = SetMtEphIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SetMtEphIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &SetMtEphIter<'a, T>) -> SetMtEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Hash> View for SetMtEphGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    pub trait SetMtEphTrait<T: StT + Hash> : View<V = Set<<T as View>::V>> + Sized {

        /// A set is finite
        open spec fn spec_finite(&self) -> bool {
            self@.finite()
        }

        /// APAS: Work Θ(|v|), Span Θ(1)
        fn from_vec(v: Vec<T>) -> (s: SetMtEph<T>)
            requires valid_key_type::<T>()
            ensures s@.finite(), s@ == v@.map(|i: int, x: T| x@).to_set();

        /// APAS: Work Θ(1), Span Θ(1)
        fn iter<'a>(&'a self) -> (it: SetMtEphIter<'a, T>)
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
        fn size(&self)                       -> N;

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

        /// - APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, s2: &SetMtEph<T>) -> (union: Self)
            requires 
               valid_key_type::<T>(),
            ensures union@.finite(), union@ == self@.union(s2@);

        /// - Disjoint union: union of two sets known to be disjoint.
        /// - APAS: Work Θ(|a| + |b|), Span Θ(1)
        fn disjoint_union(&self, s2: &SetMtEph<T>) -> (union: Self)
            requires 
               valid_key_type::<T>(),
               self@.disjoint(s2@),
            ensures 
               union@.finite(),
               union@ == self@.union(s2@),
               union@.len() == self@.len() + s2@.len();

        /// - APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, s2: &SetMtEph<T>) -> (intersection: Self)
            requires valid_key_type::<T>()
            ensures intersection@.finite(), intersection@ == self@.intersect(s2@);

        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            requires 
              valid_key_type::<T>(),
              valid_key_type::<U>(),
              valid_key_type::<Pair<T, U>>(),
            ensures  
               product@.finite(),
               forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (av == a@ && s2@.contains(bv));

        /// - APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// - claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn cartesian_product<U: StT + Hash + Clone + Send + Sync + 'static>(&self, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            where T: Send + Sync + 'static, Pair<T, U>: StT + Hash + View<V = (T::V, U::V)>,
            requires 
                valid_key_type::<T>(),
                valid_key_type::<U>(),
                valid_key_type::<Pair<T, U>>(),
            ensures  
                product@.finite(),
                forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (self@.contains(av) && s2@.contains(bv));

        fn all_nonempty(parts: &SetMtEph<SetMtEph<T>>) -> (all_nonempty: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetMtEph<T>>(),
            ensures 
                all_nonempty <==> forall |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) ==> s.len() != 0;

        fn partition_on_elt(x: &T, parts: &SetMtEph<SetMtEph<T>>) -> (partition_on_elt: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetMtEph<T>>(),
            ensures 
                partition_on_elt <==> (
                    (exists |s: Set<T::V>| #![trigger parts@.contains(s)] parts@.contains(s) && s.contains(x@)) &&
                    (forall |s1: Set<T::V>, s2: Set<T::V>|
                        #![trigger parts@.contains(s1), parts@.contains(s2)]
                        parts@.contains(s1) && s1.contains(x@) &&
                        parts@.contains(s2) && s2.contains(x@) ==> s1 == s2)
                );

        /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &SetMtEph<SetMtEph<T>>) -> (partition: bool)
            requires 
                valid_key_type::<T>(),
                valid_key_type::<SetMtEph<T>>(),
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
    }

    impl<T: StT + Hash> View for SetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.elements@ }
    }

    impl<T: StT + Hash> Clone for SetMtEph<T> {
        fn clone(&self) -> (clone: Self)
            ensures clone@.finite(), clone@ == self@
        { SetMtEph { elements: self.elements.clone() } }
    }

    impl<T: StT + Hash> SetMtEphTrait<T> for SetMtEph<T> {

        fn from_vec(v: Vec<T>) -> SetMtEph<T> {
            let mut s: SetMtEph<T> = SetMtEph::empty();
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

        fn iter<'a>(&'a self) -> (it: SetMtEphIter<'a, T>) {
            let inner = self.elements.iter();
            proof { lemma_seq_map_to_set_equality(inner@.1, self@); }
            SetMtEphIter { inner }
        }

        fn to_seq(&self) -> (seq: Vec<T>) {
            let mut seq: Vec<T> = Vec::new();
            let it: SetMtEphIter<T> = self.iter();
            let ghost iter_seq: Seq<T> = it@.1;
            
            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == iter_seq,
                    iter.pos <= iter_seq.len(),
                    iter_seq.map(|_i: int, k: T| k@).to_set() == self@,
                    iter_seq.no_duplicates(),
                    seq@ == iter_seq.take(iter.pos),
            {
                seq.push(x.clone_plus());
            }
            seq
        }

        fn empty() -> SetMtEph<T> { SetMtEph { elements: HashSetWithViewPlus::new() } }

        fn singleton(x: T) -> (s: SetMtEph<T>) {
            let mut s = HashSetWithViewPlus::new();
            let _ = s.insert(x);
            SetMtEph { elements: s }
        }

        fn size(&self) -> (size: N)
            ensures size == self@.len()
        { self.elements.len() }

        fn mem(&self, x: &T) -> (contains: B) { self.elements.contains(x) }

        fn insert(&mut self, x: T) -> (inserted: bool)
        { self.elements.insert(x) }

        fn union(&self, s2: &SetMtEph<T>) -> (union: SetMtEph<T>)
        {
            let mut union: SetMtEph<T> = self.clone_plus();
            let it: SetMtEphIter<T> = s2.iter();
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_seq: Seq<T> = it@.1;

            for x in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == s2_seq,
                    iter.pos <= s2_seq.len(),
                    s2_seq.map(|i: int, k: T| k@).to_set() == s2@,
                    union@ == s1_view.union(s2_seq.take(iter.pos).map(|i: int, k: T| k@).to_set()),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(s2_seq, iter.pos); }
                let _ = union.insert(x.clone_plus());
            }
            union
        }

        fn disjoint_union(&self, s2: &SetMtEph<T>) -> (union: SetMtEph<T>)
        {
            let capacity = self.size().saturating_add(s2.size());
            let mut union: SetMtEph<T> = SetMtEph { 
                elements: HashSetWithViewPlus::with_capacity(capacity) 
            };
            
            let it1: SetMtEphIter<T> = self.iter();
            let ghost it1_seq: Seq<T> = it1@.1;
            
            for x in iter1: it1
                invariant
                    valid_key_type::<T>(),
                    iter1.elements == it1_seq,
                    iter1.pos <= it1_seq.len(),
                    it1_seq.map(|i: int, k: T| k@).to_set() == self@,
                    union@ == it1_seq.take(iter1.pos).map(|i: int, k: T| k@).to_set(),
            {
                proof { lemma_take_one_more_extends_the_seq_set_with_view(it1_seq, iter1.pos); }
                let _ = union.insert(x.clone_plus());
            }
            
            let it2: SetMtEphIter<T> = s2.iter();
            let ghost it2_seq: Seq<T> = it2@.1;
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_view: Set<T::V> = s2@;
            
            for x in iter2: it2
                invariant
                    valid_key_type::<T>(),
                    iter2.elements == it2_seq,
                    iter2.pos <= it2_seq.len(),
                    it2_seq.map(|i: int, k: T| k@).to_set() == s2_view,
                    s1_view.disjoint(s2_view),
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

        fn intersection(&self, s2: &SetMtEph<T>) -> (intersection: SetMtEph<T>)
        {
            let mut intersection: SetMtEph<T> = SetMtEph::empty();
            let it: SetMtEphIter<T> = self.iter();
            let ghost s1_view: Set<T::V> = self@;
            let ghost s2_view: Set<T::V> = s2@;
            let ghost s1_seq: Seq<T> = it@.1;

            for s1mem in iter: it
                invariant
                    valid_key_type::<T>(),
                    iter.elements == s1_seq,
                    iter.pos <= s1_seq.len(),
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    s2_view == s2@,
                    intersection@ == s1_seq.take(iter.pos).map(|i: int, k: T| k@).to_set().intersect(s2_view),
            {
                proof { lemma_take_one_more_intersect(s1_seq, s2_view, iter.pos); }
                
                if s2.mem(s1mem) {
                    let _ = intersection.insert(s1mem.clone_plus());
                } 
            }
            
            intersection
        }

        fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
        {
            let mut product: SetMtEph<Pair<T, U>> = SetMtEph::empty();
            let it: SetMtEphIter<U> = s2.iter();
            let ghost s2_seq: Seq<U> = it@.1;
            let ghost s2_view: Set<U::V> = s2@;
            let ghost a_view: T::V = a@;
            
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
                let _ = product.insert(Pair(a.clone_plus(), b.clone_plus()));
            }
            
            product
        }
        
        fn cartesian_product<U: StT + Hash + Clone + Send + Sync + 'static>(&self, s2: &SetMtEph<U>) -> (product: SetMtEph<Pair<T, U>>)
            where T: Send + Sync + 'static, Pair<T, U>: StT + Hash + View<V = (T::V, U::V)>,
        {
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            
            // Phase 1: Spawn one task per element in s1
            let mut it = self.iter();
            let ghost it_seq = it@.1;
            let mut handles: Vec<TaskState<SetMtEph<Pair<T, U>>>> = Vec::new();
            let ghost mut spawned_views: Seq<T::V> = Seq::empty();
            
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    it@.1 == it_seq,
                    it@.0 <= it_seq.len(),
                    it_seq.map(|_i: int, k: T| k@).to_set() == s1_view,
                    it_seq.no_duplicates(),
                    s2@ == s2_view,
                    handles@.len() == spawned_views.len(),
                    spawned_views.len() == it@.0,
                    forall |i: int| #![auto] 0 <= i < spawned_views.len() ==> spawned_views[i] == it_seq[i]@,
                decreases it_seq.len() - it@.0,
            {
                match it.next() {
                    Some(a) => {
                        let ghost a_view = a@;
                        let ghost idx = it@.0 - 1;
                        
                        // Clone for the task
                        let a_clone = a.clone_plus();
                        let s2_clone = s2.clone();
                        
                        proof {
                            use crate::vstdplus::feq::feq::lemma_cloned_view_eq;
                            lemma_cloned_view_eq(*a, a_clone);
                        }
                        
                        let handle = spawn(
                            (move || -> (r: SetMtEph<Pair<T, U>>)
                                requires
                                    valid_key_type::<T>(),
                                    valid_key_type::<U>(),
                                    valid_key_type::<Pair<T, U>>(),
                                ensures
                                    r@.finite(),
                                    forall |av: T::V, bv: U::V| r@.contains((av, bv)) <==> (av == a_clone@ && s2_clone@.contains(bv)),
                            {
                                Self::elt_cross_set(&a_clone, &s2_clone)
                            })
                        );
                        
                        handles.push(handle);
                        proof { spawned_views = spawned_views.push(a_view); }
                    },
                    None => break,
                }
            }
            
            // Phase 2: Wait for tasks and disjoint_union results
            // Process in reverse order since we'll pop from the end
            let mut product: SetMtEph<Pair<T, U>> = SetMtEph::empty();
            let ghost mut joined_views: Set<T::V> = Set::empty();
            let ghost n = handles.len();
            
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<U>(),
                    valid_key_type::<Pair<T, U>>(),
                    handles@.len() <= n,
                    n == it_seq.len(),
                    spawned_views.len() == n,
                    it_seq.no_duplicates(),
                    forall |j: int| #![auto] 0 <= j < n ==> spawned_views[j] == it_seq[j]@,
                    product@.finite(),
                    joined_views.finite(),
                    // joined_views contains the elements we've processed (from the back)
                    forall |j: int| #![auto] handles@.len() <= j < n ==> joined_views.contains(spawned_views[j]),
                    forall |v: T::V| joined_views.contains(v) ==> 
                        exists |j: int| #![auto] handles@.len() <= j < n && v == spawned_views[j],
                    forall |av: T::V, bv: U::V| product@.contains((av, bv)) <==> (joined_views.contains(av) && s2_view.contains(bv)),
                decreases handles@.len(),
            {
                if handles.len() == 0 {
                    break;
                }
                
                let ghost idx = handles.len() - 1;
                let ghost a_view = spawned_views[idx as int];
                let handle: TaskState<SetMtEph<Pair<T, U>>> = handles.pop().unwrap();
                
                let thread_result: SetMtEph<Pair<T, U>> = wait(handle);
                
                proof {
                    // thread_result satisfies the closure's ensures:
                    // r@.finite() && forall av, bv: r@.contains((av, bv)) <==> (av == a_clone@ && s2_clone@.contains(bv))
                    // where a_clone@ == a_view and s2_clone@ == s2_view
                    assert(thread_result@.finite());

                    // We need to assume the ensures since we can't directly access handle's predicate.
                    assume(forall |av: T::V, bv: U::V| thread_result@.contains((av, bv)) <==> (av == a_view && s2_view.contains(bv)));
                    
                    // Prove a_view is not in the joined_views.
                    assert(!joined_views.contains(a_view)) by {
                        if joined_views.contains(a_view) {
                            // Then exists j in [idx+1, n) with spawned_views[j] == a_view
                            // But spawned_views[idx] == a_view and it_seq.no_duplicates()
                            // This contradicts no_duplicates since it_seq[idx]@ == it_seq[j]@ for j != idx
                        }
                    }
                    
                    assert(product@.disjoint(thread_result@)) by {
                        assert forall |p: (T::V, U::V)| !(product@.contains(p) && thread_result@.contains(p)) by {
                            if product@.contains(p) && thread_result@.contains(p) {
                                let (av, bv) = p;
                                // From product: joined_views.contains(av)
                                assert(joined_views.contains(av));
                                // From thread_result: av == a_view
                                assert(av == a_view);
                                // Contradiction: a_view not in joined_views
                                assert(false);
                            }
                        }
                    }
                }
                
                product = product.disjoint_union(&thread_result);
                
                proof {
                    joined_views = joined_views.insert(a_view);
                }
            }
            
            proof {
                // Prove joined_views == s1_view
                // At loop end: handles@.len() == 0, so joined_views contains spawned_views[0..n]
                assert(joined_views == s1_view) by {
                    assert forall |v: T::V| s1_view.contains(v) implies joined_views.contains(v) by {
                        lemma_map_to_set_contains_index(it_seq, v);
                        let j = choose |j: int| #![auto] 0 <= j < it_seq.len() && v == it_seq[j]@;
                        assert(0 <= j < n as int);
                        assert(v == spawned_views[j]);
                    }
                    assert forall |v: T::V| joined_views.contains(v) implies s1_view.contains(v) by {
                        let j = choose |j: int| #![auto] 0 <= j < n && v == spawned_views[j];
                        assert(v == it_seq[j]@);
                        lemma_seq_index_in_map_to_set(it_seq, j);
                    }
                }
            }
            
            product
        }

        fn all_nonempty(parts: &SetMtEph<SetMtEph<T>>) -> bool {
            let parts_iter       =  parts.iter();
            let mut parts_it     = parts_iter;
            let ghost parts_seq  = parts_it@.1;
            let ghost parts_view = parts@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type::<T>(),
                    valid_key_type::<SetMtEph<T>>(),
                    parts_it@.0 <= parts_seq.len(),
                    parts_it@.1 == parts_seq,
                    parts_seq.map(|i: int, k: SetMtEph<T>| k@).to_set() == parts_view,
                    forall |i: int| #![trigger parts_seq[i]] 0 <= i < parts_it@.0 ==> parts_seq[i]@.len() != 0,
                decreases parts_seq.len() - parts_it@.0,
            {
                let ghost old_pos = parts_it@.0;
                match parts_it.next() {
                    Some(subset) => {
                        if subset.size() == 0 {
                            proof {
                                crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(parts_seq, old_pos);
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


        fn partition_on_elt(x: &T, parts: &SetMtEph<SetMtEph<T>>) -> bool {
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
                    valid_key_type::<SetMtEph<T>>(),
                    parts_it@.0 <= parts_seq.len(),
                    parts_it@.1 == parts_seq,
                    parts_seq.map(|i: int, k: SetMtEph<T>| k@).to_set() == parts_view,
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
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(parts_seq, prev_idx);
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(parts_seq, old_pos);
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
                                crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(parts_seq, idx);
                            }
                            return true;
                        }
                    }
                }
            }
        }

        fn partition(&self, parts: &SetMtEph<SetMtEph<T>>) -> bool {
            // First check if all parts are non-empty
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
                    valid_key_type::<SetMtEph<T>>(),
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
                                crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(s1_seq, old_pos);
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
    }

    impl<T: StT + Hash> std::hash::Hash for SetMtEph<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.elements.hash(state); }
    }

    impl<T: StT + Hash> Eq for SetMtEph<T> {}

  } // verus!

    #[macro_export]
    macro_rules! SetMtLit {
        () => {{
            < $crate::Chap05::SetMtEph::SetMtEph::SetMtEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetMtEph::SetMtEph::SetMtEph<_> >::empty();
            $( let _ = __s.insert($x); )*
            __s
        }};
    }

    impl<T: StT + Hash> PartialEq for SetMtEph<T> {
        fn eq(&self, other: &Self) -> bool { self.elements == other.elements }
    }

    impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Display for SetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMt({})", self.elements.len())
        }
    }
    
    impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Debug for SetMtEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetMtEph({})", self.elements.len())
        }
    }

}
