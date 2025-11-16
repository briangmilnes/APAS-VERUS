//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `vstd::hash_set::HashSetWithView`.
//!
//! SetStEph implements SetWithView (verified specs) and extends with APAS-specific API.

pub mod SetStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::{Hash, Hasher};
    use vstd::prelude::*;

    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::set_with_view::SetWithView::SetWithView;
    use crate::Types::Types::*;

    verus! {

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::{obeys_key_model, SetIterAdditionalSpecFns};

    #[cfg(verus_keep_ghost)]
    broadcast use {vstd::std_specs::hash::group_hash_axioms, 
                   crate::vstdplus::clone_view::clone_view::group_clone_view_axioms, 
                   crate::Types::Types::group_pair_axioms,
                   crate::vstdplus::set_axioms::set_axioms::group_set_axioms_plus};

    #[verifier::ext_equal]
    #[verifier::reject_recursive_types(T)]
    pub struct SetStEph<T: View + Eq + Hash + Clone> {
        pub data: HashSetWithViewPlus<T>,
    }

    pub trait SetStEphTrait<T: StT + Hash>: SetWithView<T> {
        fn singleton(x: T) -> (result: Self)
            requires obeys_key_model::<T>(),
                     forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@)
        {
            let mut s = Self::empty();
            s.insert(x);
            s
        }

        fn size(&self) -> (result: N) ensures result == self@.len() { self.len() }

        fn mem(&self, x: &T) -> (result: B) ensures result == self@.contains(x@) { self.contains(x) }

        fn iter(&self) -> (result: SetStEphIter<'_, T>)
            ensures
                result@.elements == self@,
                result@.previous.is_empty(),
                result@.future == self@,
                result@.curr is None,
                result@.exhausted == false;

        fn FromVec(v: Vec<T>) -> Self
            requires obeys_key_model::<T>(),
                     forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2;

        fn ToVec(&self) -> (result: Vec<T>);
    //        requires obeys_key_model::<T>(),
    //                 forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
    //        ensures forall |i: int| #![trigger result@[i]@] 0 <= i < result@.len() ==> self@.contains(result@[i]@);

        // fn ToVecHashSetIterator(&self) -> (result: Vec<T>);
        fn ToVecSetIteratorWhile(&self) -> (result: Vec<T>)
            requires
                obeys_key_model::<T>(),
                forall|t1: T, t2: T| t1@ == t2@ ==> t1 == t2,
        ;
//            ensures
//                result@.len() == self@.len(),
//                result@.map(|i: int, t: T| t@).to_set() =~= self@;

        fn ToVecSetIteratorLoop(&self) -> (result: Vec<T>);

        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> (result: SetStEph<Pair<T, U>>)
            requires obeys_key_model::<Pair<T, U>>(),
            ensures
                forall|t: T::V, u: U::V|
                    #![trigger result@.contains((t, u))]
                    result@.contains((t, u)) <==> (self@.contains(t) && other@.contains(u)),
            ;

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B;
    }

    /// Ghost iterator for SetStEph that tracks iteration state using abstract sets.
    pub struct SetStEphGhostIterator<'a, T: View> {
        pub elements : Set<T::V>,       // The complete set of elements being iterated over.
        pub previous : Set<T::V>,       // The subset of elements that have already been consumed during iteration.
        pub curr     : Option<T::V>,    // The current element being examined (None if at start or end of iteration).
        pub future   : Set<T::V>,       // The subset of elements that remain to be iterated over (excluding curr).
        pub exhausted: bool,            // True if next() has returned None (iterator exhausted).
        pub phantom  : Option<&'a T>,   // A phantom data marker that establishes the lifetime relationship between
                                        // the iterator and the borrowed SetStEph instance.
    }

    impl<'a, T: View + Eq + Hash> SetStEphGhostIterator<'a, T> {
        /// Step 1: exec_invariant - connects executable iterator state to ghost state
        /// This is the core partition property that always holds
        pub open spec fn exec_invariant(&self, exec_iter: &SetStEphIter<'a, T>) -> bool {
            &&& self.elements  == exec_iter@.elements
            &&& self.previous  == exec_iter@.previous
            &&& self.curr      == exec_iter@.curr
            &&& self.future    == exec_iter@.future
            &&& self.exhausted == exec_iter@.exhausted
            // Core partition property
            &&& match self.curr {
                Some(c) => {
                    &&& self.elements == self.previous + Set::empty().insert(c) + self.future
                    &&& self.previous.disjoint(Set::empty().insert(c))
                    &&& self.previous.disjoint(self.future)
                    &&& Set::empty().insert(c).disjoint(self.future)
                },
                None => {
                    &&& self.elements == self.previous + self.future
                    &&& self.previous.disjoint(self.future)
                },
            }
        }

        /// Step 1: ghost_invariant - additional invariants about iterator progression
        pub open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                // Initial state properties
                &&& init.exhausted == false
                &&& init.curr is None
                &&& init.previous.is_empty()
                &&& init.elements == self.elements  // elements never change
                // State consistency
                &&& (self.exhausted <==> (self.curr is None && self.future.is_empty()))
                // Monotonicity: previous grows, future shrinks
                &&& init.previous.subset_of(self.previous)
                &&& self.future.subset_of(init.future)
            }
        }

        /// Step 2: ghost_ensures - what's true when iteration is complete
        pub open spec fn ghost_ensures(&self) -> bool {
                    self.exhausted == true
        }

        /// Step 2: ghost_decrease - termination measure
        pub open spec fn ghost_decrease(&self) -> Option<int> {
            if self.exhausted {
                Some(0int)
            } else {
                // Count remaining elements: future + current (if any)
                Some(self.future.len() + if self.curr is Some { 1int } else { 0int })
            }
        }

        /// Step 3: ghost_peek_next - what will next() return?
        pub open spec fn ghost_peek_next(&self) -> Option<T::V> {
            if self.exhausted {
                None
            } else if self.curr is Some {
                self.curr
            } else if self.future.is_empty() {
                None
            } else {
                // Nondeterministic - some element from future
                Some(self.future.choose())
            }
        }

        /// Step 3: ghost_advance - specification for state transition after next()
        pub open spec fn ghost_advance(&self, _exec_iter: &SetStEphIter<'a, T>) -> SetStEphGhostIterator<'a, T> {
            if self.exhausted {
                *self  // No change if already exhausted
            } else if self.curr is Some {
                // Currently have an element - it will be returned
                let c = self.curr->0;
                SetStEphGhostIterator {
                    elements: self.elements,
                    previous: self.previous.insert(c),
                    curr: if self.future.is_empty() { None } else { Some(self.future.choose()) },
                    future: if self.future.is_empty() { self.future } else { self.future.remove(self.future.choose()) },
                    exhausted: self.future.is_empty(),
                    phantom: None,
                }
            } else {
                // curr is None, future non-empty - grab from future
                let c = self.future.choose();
                SetStEphGhostIterator {
                    elements: self.elements,
                    previous: self.previous,
                    curr: Some(c),
                    future: self.future.remove(c),
                    exhausted: false,
                    phantom: None,
                }
            }
        }
    }

    pub struct SetStEphIter<'a, T: View + Eq + Hash> {
        pub hash_set_iter: std::collections::hash_set::Iter<'a, T>,
        pub curr: Option<&'a T>,
        pub exhausted: bool,
    }

    impl<'a, T: View + Eq + Hash> View for SetStEphIter<'a, T> {
        type V = SetStEphGhostIterator<'a, T>;
        
        open spec fn view(&self) -> SetStEphGhostIterator<'a, T> {
            let pos = self.hash_set_iter@.0;
            let elements_seq = self.hash_set_iter@.1;
            let elements_set = elements_seq.map(|i: int, t: T| t@).to_set();
            
            match (self.curr, self.exhausted, pos == 0, elements_seq.len() == 0) {
                // State 1: Initial empty (curr=None, exhausted=false, pos=0, len=0)
                (None, false, true, true) => {
                    SetStEphGhostIterator {
                        elements: elements_set,
                        previous: Set::empty(),
                        curr: None,
                        future: Set::empty(),
                        exhausted: false,
                        phantom: None,
                    }
                },
                
                // State 2: Initial non-empty (curr=None, exhausted=false, pos=0, len>0)
                (None, false, true, false) => {
                    SetStEphGhostIterator {
                        elements: elements_set,
                        previous: Set::empty(),
                        curr: None,
                        future: elements_set,
                        exhausted: false,
                        phantom: None,
                    }
                },
                
                // State 3: Stepping (curr=Some(x), exhausted=false)
                (Some(c), false, _, _) => {
                    SetStEphGhostIterator {
                        elements: elements_set,
                        previous: elements_seq.take(pos - 1).map(|i: int, t: T| t@).to_set(),
                        curr: Some(c@),
                        future: elements_seq.skip(pos).map(|i: int, t: T| t@).to_set(),
                        exhausted: false,
                        phantom: None,
                    }
                },
                
                // State 4: Exhausted (curr=None, exhausted=true)
                (None, true, _, _) => {
                    SetStEphGhostIterator {
                        elements: elements_set,
                        previous: elements_seq.take(pos).map(|i: int, t: T| t@).to_set(),
                        curr: None,
                        future: Set::empty(),
                        exhausted: true,
                        phantom: None,
                    }
                },
                
                // All other combinations are invalid (unreachable in practice)
                _ => {
                    // Return a valid ghost state (should never happen)
                    SetStEphGhostIterator {
                        elements: elements_set,
                        previous: elements_set,
                        curr: None,
                        future: Set::empty(),
                        exhausted: true,
                        phantom: None,
                    }
                }
            }
        }
    }

    impl<'a, T: View + Eq + Hash> Iterator for SetStEphIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (result: Option<Self::Item>)
            ensures
                // Ghost state updated correctly
                self@.exec_invariant(self),
                
                // Elements never change
                self@.elements == old(self)@.elements,
                
                // Result matches what ghost_peek_next predicted
                match result {
                    Some(x) => old(self)@.ghost_peek_next() == Some(x@),
                    None => old(self)@.ghost_peek_next() is None,
                },
                
                // Ghost state transitions correctly
                result is Some ==> self@ == old(self)@.ghost_advance(&old(self)),
                
                // Termination: measure decreased or stayed at 0
                match (old(self)@.ghost_decrease(), self@.ghost_decrease()) {
                    (Some(old_m), Some(new_m)) => new_m <= old_m,
                    _ => true,
                },
        {
            if self.exhausted {
                self.curr = None;
            } else {
                self.curr = self.hash_set_iter.next();
                
                if matches!(self.curr, None) {
                    self.exhausted = true;
                }
            }
            self.curr
        }
    }

    impl<'a, T: View> SetStEphGhostIterator<'a, T> {
        /// Axiom: Elements decomposes into disjoint union of previous, curr (if present), and future.
        pub open spec fn setsteph_iter_partition_axiom(&self) -> bool {
            match self.curr {
                Some(c) => {
                    &&& self.elements == self.previous + Set::empty().insert(c) + self.future
                    &&& self.previous.disjoint(Set::empty().insert(c))
                    &&& self.previous.disjoint(self.future)
                    &&& Set::empty().insert(c).disjoint(self.future)
                },
                None => {
                    &&& self.elements == self.previous + self.future
                    &&& self.previous.disjoint(self.future)
                },
            }
        }

        /// Axiom: Previous, curr, and future are all subsets or members of elements.
        pub open spec fn setsteph_iter_subset_axiom(&self) -> bool {
            &&& self.previous.subset_of(self.elements)
            &&& self.future.subset_of(self.elements)
            &&& (self.curr is Some ==> self.elements.contains(self.curr->0))
        }

        /// Axiom: At iteration start, previous is empty, curr is None, future is all elements.
        pub open spec fn setsteph_iter_start_axiom(&self) -> bool {
            (self.previous.is_empty() && self.curr is None) ==> self.future == self.elements
        }

        /// Axiom: At iteration end, previous is all elements, curr is None, future is empty.
        pub open spec fn setsteph_iter_end_axiom(&self) -> bool {
            (self.future.is_empty() && self.curr is None) ==> self.previous == self.elements
        }

        /// Combined invariant that should hold throughout SetStEph iteration.
        pub open spec fn setsteph_iter_invariant(&self) -> bool {
            &&& self.setsteph_iter_partition_axiom()
            &&& self.setsteph_iter_subset_axiom()
            &&& self.setsteph_iter_start_axiom()
            &&& self.setsteph_iter_end_axiom()
        }

        /// Specification for advancing iteration: move curr to previous, select new curr from future.
        pub open spec fn setsteph_iter_advance_spec(&self, new_iter: SetStEphGhostIterator<'a, T>) -> bool {
            match (self.curr, new_iter.curr) {
                (Some(old_c), Some(new_c)) => {
                    &&& new_iter.elements == self.elements
                    &&& new_iter.previous == self.previous.insert(old_c)
                    &&& self.future.contains(new_c)
                    &&& new_iter.future == self.future.remove(new_c)
                },
                (Some(old_c), None) => {
                    &&& new_iter.elements == self.elements
                    &&& new_iter.previous == self.previous.insert(old_c)
                    &&& new_iter.future == self.future
                    &&& self.future.is_empty()
                },
                _ => false, // Can't advance from None state
            }
        }
    }

    impl<T: View + Eq + Hash + Clone> SetStEph<T> {
        pub open spec fn view(&self) -> Set<<T as View>::V> { self.data@ }
    }

    impl<T: View + Eq + Hash + Clone> View for SetStEph<T> {
        type V = Set<<T as View>::V>;

        open spec fn view(&self) -> Set<<T as View>::V> { Self::view(self) }
    }


    impl<T: StT + Hash> SetWithView<T> for SetStEph<T> {
        fn empty() -> (result: Self) { SetStEph { data: HashSetWithViewPlus::new() } }
        fn contains(&self, x: &T) -> (result: bool) { self.data.contains(x) }
        fn insert(&mut self, x: T) { self.data.insert(x); }
        fn remove(&mut self, x: &T) { self.data.remove(x); }
        fn union(&self, other: &Self) -> (result: Self) { SetStEph { data: self.data.union(&other.data) } }
        fn intersect(&self, other: &Self) -> (result: Self) { SetStEph { data: self.data.intersection(&other.data) } }
        fn difference(&self, other: &Self) -> (result: Self) { SetStEph { data: self.data.difference(&other.data) } }
        fn len(&self) -> (result: usize) { self.data.len() }
        fn is_empty(&self) -> (result: bool) { self.data.len() == 0 }
    }


   impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
        #[verifier::external_body]
        fn iter(&self) -> (result: SetStEphIter<'_, T>) {
            SetStEphIter { 
                hash_set_iter: self.data.iter(),
                curr: None,
                exhausted: false,
            }
        }

        fn FromVec(v: Vec<T>) -> (result: Self)
            ensures forall |i: int| #![trigger result@.contains(v@[i]@)] 0 <= i < v@.len() ==> result@.contains(v@[i]@)
        {
            let mut s = Self::empty();
            for idx in iter: 0..v.len()
                invariant
                    forall |j: int| #![trigger s@.contains(v@[j]@)] 0 <= j < iter.cur ==> s@.contains(v@[j]@),
            {
                s.insert(v[idx].clone());
            }
            s
        }

        // STUB
        fn ToVec(&self) -> (result: Vec<T>) { Vec::new() }

        /*
        fn ToVecHashSetIterator(&self) -> (result: Vec<T>)
        {
            let ghost input_set: Set<T::V> = self@;
            let mut v: Vec<T> = Vec::new();

            for x in it: self.data.iter()
                invariant
                    // The starting ones here:
                    it.elements.map(|i: int, t: T| t@).to_set() =~= input_set,
                    // The maintaining ones here:
                    v@.len() == it.pos,
                    v@ == it@,
                    forall |i: int| #![trigger v@[i]@] 0 <= i < v@.len() ==> input_set.contains(v@[i]@),
                    // The ending ones here:
            {
                v.push(x.clone());
            }
            v
        }

        */

            // FOR loop version - doesn't work because SetStEphIter doesn't implement ForLoopGhostIteratorNew
            /*
        fn ToVecSetIteratorWhile(&self) -> (result: Vec<T>)
            let mut iter: SetStEphIter<'_, T> = self.iter();
            for x in it: iter
                invariant
                    // The starting ones here:
                    it.elements.map(|i: int, t: T| t@).to_set() =~= input_set,
                    // The maintaining ones here:
                    v@.len() == it.pos,
                    v@ == it@,
                    forall |i: int| #![trigger v@[i]@] 0 <= i < v@.len() ==> input_set.contains(v@[i]@),
                    // The ending ones here:
            {
                v.push(x.clone());
            }
            */

        #[verifier::exec_allows_no_decreases_clause]
        fn ToVecSetIteratorWhile(&self) -> (result: Vec<T>)
        {
            let ghost input_set: Set<T::V> = self@;
            let mut v: Vec<T> = Vec::new();

            let mut iter: SetStEphIter<'_, T> = self.iter();
            
//            proof { proof_initial_state_iter(self, iter);}
            
            let mut done = false;
            while !done
                invariant
                    input_set == self@,
                    iter@.elements == input_set,
                    
                    // Partition axiom: elements = previous + curr + future (with disjointness)
                    match iter@.curr {
                        Some(c) => {
                            &&& input_set == iter@.previous + Set::empty().insert(c) + iter@.future
                            &&& iter@.previous.disjoint(Set::empty().insert(c))
                            &&& iter@.previous.disjoint(iter@.future)
                            &&& Set::empty().insert(c).disjoint(iter@.future)
                        },
                        None => {
                            &&& input_set == iter@.previous + iter@.future
                            &&& iter@.previous.disjoint(iter@.future)
                        },
                    },
                    
                   // When done, we're exhausted; when not done, we can still call next()
                   done == iter@.exhausted,
           {
                let ghost old_iter = iter@;
                let result = iter.next();
                match result {
                    Some(x) => { 
                        v.push(x.clone()); 
                    },
                    None => { 
                        done = true;
                        assume(iter@.previous == input_set);
                    },
                 }
             }
            v
        }

        #[verifier::exec_allows_no_decreases_clause]
        fn ToVecSetIteratorLoop(&self) -> (result: Vec<T>)
        {
            let ghost input_set: Set<T::V> = self@;
            let mut v: Vec<T> = Vec::new();

            let mut iter = self.iter();
            loop
                invariant
                    input_set == self@,
            {
                match iter.next() {
                    Some(x) => v.push(x.clone()),
                    None => break,
                }
            }
            v
        }

        #[verifier::external_body]
        
        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>> {
            let mut result = SetStEph::empty();

            let mut self_vec: Vec<T> = Vec::new();
            let mut self_iter = self.iter();
            let mut self_count: usize = 0;
            let self_max = self.data.len();
            while self_count < self_max
                invariant
                    self_count <= self_max,
                    self_vec@.len() == self_count,
                decreases self_max - self_count,
            {
                match self_iter.next() {
                    Some(x) => {
                        self_vec.push(x.clone());
                        self_count = self_count + 1;
                    },
                    None => break,
                }
            }

            let mut other_vec: Vec<U> = Vec::new();
            let mut other_iter = other.iter();
            let mut other_count: usize = 0;
            let other_max = other.data.len();
            while other_count < other_max
                invariant
                    other_count <= other_max,
                    other_vec@.len() == other_count,
                decreases other_max - other_count,
            {
                match other_iter.next() {
                    Some(y) => {
                        other_vec.push(y.clone());
                        other_count = other_count + 1;
                    },
                    None => break,
                }
            }

            let mut i: usize = 0;
            while i < self_vec.len()
                invariant
                    i <= self_vec.len(),
                    forall |ii: int, jj: int|
                        #![trigger result@.contains((self_vec@[ii]@, other_vec@[jj]@))]
                        0 <= ii < i && 0 <= jj < other_vec@.len() ==>
                            result@.contains((self_vec@[ii]@, other_vec@[jj]@)),
                decreases self_vec.len() - i,
            {
                let mut j: usize = 0;
                while j < other_vec.len()
                    invariant
                        i < self_vec.len(),
                        j <= other_vec.len(),
                        forall |ii: int, jj: int|
                            #![trigger result@.contains((self_vec@[ii]@, other_vec@[jj]@))]
                            0 <= ii < i && 0 <= jj < other_vec@.len() ==>
                                result@.contains((self_vec@[ii]@, other_vec@[jj]@)),
                        forall |jj: int|
                            #![trigger result@.contains((self_vec@[i as int]@, other_vec@[jj]@))]
                            0 <= jj < j ==>
                                result@.contains((self_vec@[i as int]@, other_vec@[jj]@)),
                    decreases other_vec.len() - j,
                {
                    result.insert(Pair(self_vec[i].clone(), other_vec[j].clone()));
                    j = j + 1;
                }
                i = i + 1;
            }
            result
        }


        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B {
            let mut self_iter = self.iter();
            let mut self_count: usize = 0;
            let self_max = self.data.len();
            while self_count < self_max
                invariant self_count <= self_max,
                decreases self_max - self_count,
            {
                match self_iter.next() {
                    None => break,
                    Some(x) => {
                        let mut count: N = 0;
                        let mut parts_iter = parts.iter();
                        let mut parts_count: usize = 0;
                        let parts_max = parts.data.len();
                        while parts_count < parts_max
                            invariant parts_count <= parts_max,
                            decreases parts_max - parts_count,
                        {
                            match parts_iter.next() {
                                None => break,
                                Some(part) => {
                                    if part.contains(x) {
                                        if count >= 1 {
                                            return false;
                                        }
                                        count = count + 1;
                                    }
                                    parts_count = parts_count + 1;
                                }
                            }
                        }
                        if count == 0 {
                            return false;
                        }
                        self_count = self_count + 1;
                    }
                }
            }
            true
        }

    }

    impl<T: StT + Hash> SetStEph<T> {
        pub fn intersection(&self, other: &Self) -> Self {
            self.intersect(other)
        }
    }

        impl<T: StT + Hash> PartialEq for SetStEph<T> {
            #[verifier::external_body]
            fn eq(&self, other: &Self) -> (result: bool)
                ensures result == (self@ == other@)
            {
                if self.data.len() != other.data.len() {
                    return false;
                }
                for x in self.data.iter() {
                    if !other.data.contains(x) {
                        return false;
                    }
                }
                true
            }
        }

        impl<T: StT + Hash> Eq for SetStEph<T> {}

        impl<T: StT + Hash> Clone for SetStEph<T> {
            fn clone(&self) -> (result: Self)
                ensures result@ == self@
            {
                SetStEph { data: self.data.clone() }
            }
        }

        #[macro_export]
        macro_rules! SetLit {
            () => {{
                < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty()
            }};
            ($($x:expr),* $(,)?) => {{
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( __s.insert($x); )*
                __s
            }};
        }
      } // verus!

    // These won't prove in varus due to complex mut.
    impl<T: StT + Hash> Hash for SetStEph<T> {
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

    impl<T: StT + Hash> Debug for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_set().entries(self.data.iter()).finish()
        }
    }

    impl<T: StT + Hash> Display for SetStEph<T> {
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
}
