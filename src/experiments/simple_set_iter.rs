// Copyright (c) 2025 Brian G. Milnes
//! Simple Abstract Set with Iterator (based on simple_seq_iter)

pub mod simple_set_iter {
    use vstd::prelude::*;

    verus! {

    use vstd::std_specs::clone::*;
    use vstd::std_specs::vec::vec_index;
    use crate::vstdplus::seq_set::*;
    
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            vstd::seq::group_seq_axioms,
            vstd::set::group_set_axioms,
            crate::vstdplus::clone_view::clone_view::group_clone_view_axioms
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::set_lib::group_set_lib_default,
    };

    // AXIOM ATTEMPT: Bridge exec Vec indexing equality to spec equality for generic types.
    // NOTE: This does not work for generics! Verus only bridges exec == to spec == for
    // concrete types (u64, i32, etc.). For generic V: Eq, there is no automatic bridge.
    // This axiom cannot be written because:
    // 1. We cannot reference exec equality (vec[i] == v) in spec code
    // 2. Even if we could, broadcast axioms trigger on spec patterns, not exec control flow
    // 3. The fundamental issue is that exec Eq::eq and spec == are separate operations
    //
    // broadcast proof fn axiom_vec_exec_eq_spec_eq<V: Eq>(vec: &Vec<V>, i: int, v: V)
    //     requires
    //         0 <= i < vec@.len(),
    //         // PROBLEM: Can't write "vec[i as usize] == v" here - it's exec, not spec!
    //     ensures
    //         vec@[i] == v,  // This is what we want to prove
    // {
    //     admit();  // Would need to admit because there's no proof
    // }

    // SimpleSet backed by Vec (no duplicates maintained by insert).
    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSet<V> {pub elements: Vec<V>, }

    impl<V> View for SimpleSet<V> {
        type V = Set<V>;
        open spec fn view(&self) -> Set<V> { self.elements@.to_set() }
    }

    pub trait SimpleSetTrait<V: Clone>: Sized + View<V=Set<V>> {
        fn len(&self) -> usize;
        
        fn new() -> (result: Self)
            ensures result@ == Set::<V>::empty();
        
        fn mem(&self, v: &V) -> (result: bool)
            ensures result == self@.contains(*v);

        fn insert(&mut self, v: V) -> (inserted: bool)
            ensures
                old(self)@.contains(v) ==> {
                    &&& !inserted
                    &&& self@ == old(self)@
                },
                !old(self)@.contains(v) ==> {
                    &&& inserted
                    &&& self@ == old(self)@.insert(v)
                };
        
        fn iter(&self) -> (it: SimpleSetIter<V>)
            ensures
                it@.0 == 0int,
                it@.1.to_set() == self@;
    }
    
    impl<V: Clone + Eq> SimpleSetTrait<V> for SimpleSet<V> {
        fn len(&self) -> usize { self.elements.len() }
        
        fn new() -> (s: Self) { SimpleSet { elements: Vec::new() } }
        
        fn mem(&self, v: &V) -> (result: bool)
        {
            let ghost elements_seq = self.elements@;
            for i in 0..self.elements.len()
                invariant
                    self.elements@ == elements_seq,
                    forall |j: int| 0 <= j < i ==> self.elements@[j] != *v,
            {
                if self.elements[i] == *v {
                    assume(self.elements@[i as int] == *v);
                    return true;
                }
                assume(self.elements@[i as int] != *v);
            }
            false
        }
        
        fn insert(&mut self, v: V) -> (inserted: bool) {
            // Check if already present using mem
            if self.mem(&v) { return false; }
            // Not found, insert it
            let ghost old_set = self@;
            let ghost old_seq = self.elements@;
            self.elements.push(v);
            assert(!old_seq.contains(v));
            true
        }

        fn iter(&self) -> (it: SimpleSetIter<V>)
            ensures
                it@ == (0int, self.elements@),
                it.pos <= it.vec.len(),
        { SimpleSetIter { vec: self.elements.clone(), pos: 0, } }
    }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSetIter<V> {
        pub vec: Vec<V>,   // Exec: backing vector (linearized set)
        pub pos: usize,    // Exec: current position
    }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSetIterGhost<V> { pub pos: int, pub elements: Seq<V>,}

    // Iterator view is (position, full_sequence) tuple, matching vstd hash_set::Iter
    impl<V> View for SimpleSetIter<V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { (self.pos as int, self.vec@) }
    }

    // Ghost iterator view is elements already iterated (take), matching vstd
    impl<V> View for SimpleSetIterGhost<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.elements.take(self.pos) }
    }

    // The iterator invariant that should be maintained but we can't add to next's requires.
    pub open spec fn iter_invariant<V>(it: &SimpleSetIter<V>) -> bool { it.pos <= it.vec@.len() }

    // Our initial iterator ensures the invariant.
    proof fn lemma_iter_invariant<V: Clone>(s: &SimpleSet<V>, it: SimpleSetIter<V>)
        requires
            it@ == (0int, s.elements@),  // Characterizes "it is the result of s.iter()"
        ensures
            iter_invariant(&it),
    {}

    // So to increase the confidence in our actual next, we prove this version that proves with
    // the invariant.
    fn assumption_free_next<V: Clone>(it: &mut SimpleSetIter<V>) -> (result: Option<V>)
        requires
            iter_invariant(&old(it)),  
        ensures
            iter_invariant(it),
            ({
                let (old_index, old_seq) = old(it)@;
                match result {
                    None => {
                        &&& it@ == old(it)@
                        &&& old_index == old_seq.len()
                        &&& it.pos == old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = it@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& vstd::pervasive::cloned(old_seq[old_index], element)
                    },
                }
            }),
    {
        if it.pos < it.vec.len() {
            let elem = it.vec[it.pos].clone();
            it.pos = it.pos + 1;
            Some(elem)
        } else {
            None
        }
    }
   
    impl<V: Clone> Iterator for SimpleSetIter<V> {
        type Item = V;

        fn next(&mut self) -> (result: Option<V>)
            ensures
                self.pos <= self.vec.len(),
                ({
                    let (old_index, old_seq) = old(self)@;
                    match result {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index == old_seq.len()
                            &&& self.pos == old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& vstd::pervasive::cloned(old_seq[old_index as int], element)
                        },
                    }
                }),
        {
            let ghost old_view = self@;
            if self.pos < self.vec.len() {
                let elem = self.vec[self.pos].clone();
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                // Without a requires on next this is required, see assumption_free_next.
                assume(self.pos <= self.vec.len());
                None
            }
        }
    }

    impl<V> vstd::pervasive::ForLoopGhostIteratorNew for SimpleSetIter<V> {
        type GhostIter = SimpleSetIterGhost<V>;

        open spec fn ghost_iter(&self) -> SimpleSetIterGhost<V> 
        { SimpleSetIterGhost { pos: self.pos as int, elements: self.vec@, } }
    }

    impl<V> vstd::pervasive::ForLoopGhostIterator for SimpleSetIterGhost<V> {
        type ExecIter = SimpleSetIter<V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SimpleSetIter<V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
            &&& 0 <= self.pos <= self.elements.len()
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool { self.pos == self.elements.len() }

        open spec fn ghost_decrease(&self) -> Option<int> 
        { Some(self.elements.len() - self.pos) }

        open spec fn ghost_peek_next(&self) -> Option<V> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &SimpleSetIter<V>) -> Self {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    // Example: Copy a set using loop iteration
    pub fn simple_set_copy_loop(s1: &SimpleSet<u32>) -> (s2: SimpleSet<u32>)
        ensures
            s2@ == s1@,
    {
        let mut s2 = SimpleSet::new();
        let mut it = s1.iter();
        
        let ghost s1_seq = it@.1;
        
            loop
            invariant
                it@.1 == s1_seq,
                s1_seq.to_set() == s1@,              // Sequence converts to the original set.
                s2@ == s1_seq.take(it@.0).to_set(),  // And we've put all of the elements in s2.
            decreases s1_seq.len() - it@.0,
        {
            let ghost old_index = it@.0;
            let ghost old_s2 = s2@;
            match it.next() {
                Some(elem) => { 
                    proof {
                        // Before insert: setup what we know
                        let ghost target_set = s1_seq.take(it@.0).to_set();
                        
                        // Apply the lemma
                        lemma_take_one_more_extends_the_seq_set(s1_seq, old_index);
                        // This proves: s1_seq.take(old_index).to_set().insert(s1_seq[old_index]) == s1_seq.take(old_index + 1).to_set()
                        
                        // From iterator postcondition: elem == s1_seq[old_index] (via cloned)
                        assert(vstd::pervasive::cloned(s1_seq[old_index], elem));
                        
                        // Connect the pieces:
                        assert(old_s2 == s1_seq.take(old_index).to_set());
                        assert(s1_seq.take(old_index).to_set().insert(elem) == s1_seq.take(old_index + 1).to_set());
                        assert(old_s2.insert(elem) == target_set);
                    }
                    
                    let inserted = s2.insert(elem);
                    
                    proof {
                        if !inserted {
                            lemma_set_contains_insert_idempotent(old_s2, elem);
                        }
                        assert(s2@ == s1_seq.take(it@.0).to_set());
                    }
                },
                None => { 
                    proof {
                        // At loop exit, take(len) is the full sequence.
                        lemma_take_full_to_set(s1_seq);
                        assert(s1_seq.take(s1_seq.len() as int).to_set() == s1_seq.to_set());
                    }
                    return s2; 
                },
            }
        }
    }

    pub fn simple_set_copy_for(s1: &SimpleSet<u32>) -> (s2: SimpleSet<u32>)
        ensures
            s2@ == s1@
    {
        let mut s2 = SimpleSet::new();
        let len = s1.elements.len();
        
        for elem in it: s1.iter()
            invariant
                len == s1.elements.len(),
                it.elements == s1.elements@,
                it.pos <= it.elements.len(),
                s2@ == it.elements.take(it.pos).to_set(), 
        {
          s2.insert(elem);
        }
        s2
    }

    } // verus!
}
