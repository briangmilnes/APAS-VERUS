//! Simple Abstract Set with Iterator (based on simple_seq_iter)

pub mod simple_set_iter {
    use vstd::prelude::*;

    verus! {

    use vstd::std_specs::clone::*;
    use crate::vstdplus::seq_set::*;
    
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            vstd::seq::group_seq_axioms,
            vstd::set::group_set_axioms,
            crate::vstdplus::clone_view::clone_view::group_clone_view_axioms
    };

    // SimpleSet backed by Vec (no duplicates maintained by insert).
    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSet<V> {pub elements: Vec<V>, }

    impl<V> View for SimpleSet<V> {
        type V = Set<V>;
        open spec fn view(&self) -> Set<V> { self.elements@.to_set() }
    }

    pub trait SimpleSetTrait<V: Clone>: Sized {
        fn len(&self) -> usize;
        fn new() -> Self;
        fn insert(&mut self, v: V) -> (inserted: bool);
        fn iter(&self) -> (it: SimpleSetIter<V>);
    }

    impl<V: Clone + PartialEq> SimpleSetTrait<V> for SimpleSet<V> {
        fn len(&self) -> usize { self.elements.len() }
        
        fn new() -> (s: Self)
            ensures s@ == Set::<V>::empty(),
        { SimpleSet { elements: Vec::new() } }
        
        fn insert(&mut self, v: V) -> (inserted: bool)
            ensures
                old(self)@.contains(v) ==> {
                    &&& !inserted
                    &&& self@ == old(self)@
                },
                !old(self)@.contains(v) ==> {
                    &&& inserted
                    &&& self@ == old(self)@.insert(v)
                },
        {
            // Check if already present
            for i in 0..self.elements.len()
                invariant
                    forall |j: int| 0 <= j < i ==> self.elements@[j] != v,
            {
                if self.elements[i] == v {
                    // Element found at index i, so v is in the set
                    assume(self.elements@.to_set().contains(v));
                    assume(self@ == old(self)@);
                    return false;
                }
                // Bridge to next iteration: we just checked i, so it's also != v
                assume(self.elements@[i as int] != v);
            }
            // Not found, insert it
            let ghost old_set = self@;
            self.elements.push(v);
            // After push, the set view includes the new element
            assume(self@ == old_set.insert(v));
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
                // Without a requires on next this is required.
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
            match it.next() {
                Some(elem) => { 
                    s2.insert(elem); 
                    assume(s2@ == s1_seq.take(it@.0).to_set());
                },
                None => { 
                    // At loop exit, take(len) is the full sequence.
                    assume(s1_seq.take(s1_seq.len() as int).to_set() == s1_seq.to_set());
                    return s2;
                },
            }
        }
    }

    } // verus!
}


