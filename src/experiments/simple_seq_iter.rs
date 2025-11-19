//! Simple Abstract Sequence with Embedded Ghost Iterator

pub mod simple_seq_iter {
    use vstd::prelude::*;

    verus! {

    use vstd::std_specs::clone::*;
    broadcast use {
            vstd::seq_lib::group_seq_properties,
            crate::vstdplus::clone_view::clone_view::group_clone_view_axioms
    };

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeq<V> {pub elements: Vec<V>, }

    impl<V> View for SimpleSeq<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.elements@ }
    }

    pub trait SimpleSeqTrait<V: Clone>: Sized {
        fn len(&self) -> usize;
        fn new() -> Self;
        fn push(&mut self, v: V);
        fn iter(&self) -> (it: SimpleSeqIter<V>);
    }

    impl<V: Clone> SimpleSeqTrait<V> for SimpleSeq<V> {
        fn len(&self) -> usize  { self.elements.len()} 
        fn new() -> (s: Self)    ensures s@ == Seq::<V>::empty(),     { SimpleSeq { elements: Vec::new() } }
        fn push(&mut self, v: V) ensures self@ == old(self)@.push(v), { self.elements.push(v); }

        fn iter(&self) -> (it: SimpleSeqIter<V>)
            ensures
                it@ == (0int, self.elements@),
                it.pos <= it.vec.len(),
        { SimpleSeqIter { vec: self.elements.clone(), pos: 0, } }
    }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeqIter<V> {
        pub vec: Vec<V>,   // Exec: backing vector
        pub pos: usize,    // Exec: current position
    }

// Does not work due to the private field requirements. 
//    #[verifier::type_invariant]
//    spec fn simple_seq_iter_invariant<V>(s: SimpleSeqIter<V>) -> bool { s.pos <= s.vec@.len()  }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeqIterGhost<V> { pub pos: int, pub elements: Seq<V>,}

    // Iterator view is (position, full_sequence) tuple, matching vstd slice::Iter
    impl<V> View for SimpleSeqIter<V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { (self.pos as int, self.vec@) }
    }

    // Ghost iterator view is elements already iterated (take), matching vstd. 
    impl<V> View for SimpleSeqIterGhost<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.elements.take(self.pos) }
    }

    // As we have to have an assume in next, we show a clean next here with the right requires and ensures
    // and show that it proves, e.g., if it has the right requires it maintains our invariant.

    // The iterator invariant that should be maintained but we can't add to next's requires.
    pub open spec fn iter_invariant<V>(it: &SimpleSeqIter<V>) -> bool { it.pos <= it.vec@.len() }

    // Our initial iterator ensures the invariant.
    proof fn lemma_iter_invariant<V: Clone>(s: &SimpleSeq<V>, it: SimpleSeqIter<V>)
        requires
            it@ == (0int, s.elements@),  // Characterizes "it is the result of s.iter()"
        ensures
            iter_invariant(&it),
    {}

   // So to increase the confidence in our actual next, we prove this version that proves with
   // the invariant.
    fn assumption_free_next<V: Clone>(it: &mut SimpleSeqIter<V>) -> (result: Option<V>)
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
   
    impl<V: Clone> Iterator for SimpleSeqIter<V> {
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
                // Without a requires on next 
                assume(self.pos <= self.vec.len());
                None
            }
        }
    }

    impl<V> vstd::pervasive::ForLoopGhostIteratorNew for SimpleSeqIter<V> {
        type GhostIter = SimpleSeqIterGhost<V>;

        open spec fn ghost_iter(&self) -> SimpleSeqIterGhost<V> 
        { SimpleSeqIterGhost { pos: self.pos as int, elements: self.vec@, } }
    }

    impl<V> vstd::pervasive::ForLoopGhostIterator for SimpleSeqIterGhost<V> {
        type ExecIter = SimpleSeqIter<V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SimpleSeqIter<V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &SimpleSeqIter<V>) -> Self {
            Self { pos: self.pos + 1, ..*self }
        }
    }


/* The decreases gives us proof problems.
    pub fn simple_seq_copy_while(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ == s1@,
    {
        let mut sn = SimpleSeq::new();
        let mut it = s1.iter();
        let mut done = false;

        let ghost full_seq = it@.1;
        
        while !done
            invariant
                it@.1 == full_seq,
                full_seq == s1.elements@,
                sn@ == full_seq.take(it@.0 as int),
                it.pos <= it.vec.len(),
            decreases full_seq.len() - it@.0,
        {
            match it.next() {
                Some(elem) => {
                    sn.push(elem);
                },
                None => {
                    proof {
                        // From next()'s postcondition: None => old_index >= old_seq.len()
                        // So it@.0 >= full_seq.len(), thus decreases <= 0
                        assume(it@.0 == full_seq.len());
                    }
                    done = true;
                },
            }
        }
        
        proof {
            // At loop exit: it@.0 == full_seq.len()
            // So: sn@ == full_seq.take(full_seq.len()) == full_seq == s1@
            assume(it@.0 == full_seq.len());
            assert(full_seq.take(full_seq.len() as int) == full_seq);
        }
        
        sn
    }
*/

    pub fn simple_seq_copy_loop(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ == s1@,
    {
        let mut s2 = SimpleSeq::new();
        let mut it = s1.iter();
        
        let ghost s1_seq = it@.1;
        
        loop
            invariant
                it@.1 == s1_seq,
                s1_seq == s1@,
                s2@.len() == it@.0,
                forall |j: int| #![trigger s2@[j]] 0 <= j < it@.0 ==> s2@[j] == s1_seq[j],
            decreases s1_seq.len() - it@.0,
        {
            match it.next() {
                Some(elem) => { s2.push(elem); },
                None       => { return s2; },
            }
        }
    }

    pub fn simple_seq_copy_for_iter(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ =~= s1@,
    {
        let mut s2 = SimpleSeq::new();
        let len = s1.elements.len();
        
        for elem in it: s1.iter()
            invariant
                len == s1.elements.len(),
                it.elements == s1.elements@,
                s2@.len() == it.pos,
                it.pos <= it.elements.len(),
                forall |j: int| #![trigger s2@[j]] 0 <= j < it.pos ==> s2@[j] == it.elements[j],
        {
            s2.push(elem);
        }
        
        s2
    }

    // This is not using our iter but it works nicely.
    pub fn simple_seq_copy_for_range(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ =~= s1@,
    {
        let mut s2 = SimpleSeq::new();
        let len = s1.elements.len();
        
        for i in 0..len
            invariant
                len == s1.elements.len(),
                s2@ =~= s1.elements@.take(i as int),
        {
            s2.push(s1.elements[i]);
        }
        
        s2
    }

    } // verus!
}

