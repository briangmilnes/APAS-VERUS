//! Simple Abstract Sequence with Embedded Ghost Iterator

pub mod simple_seq_iter {
    use vstd::prelude::*;

    verus! {

    use crate::vstdplus::set_axioms::set_axioms::*;
    use crate::vstdplus::clone_view::clone_view::*;

    broadcast use {vstd::seq_lib::group_seq_properties, 
                   group_clone_view_axioms, 
                   group_set_axioms_plus};

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeq<V> {pub elements: Vec<V>, }

    impl<V> View for SimpleSeq<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.elements@ }
    }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeqIter<V> {
        pub vec: Vec<V>,   // Exec: backing vector
        pub pos: usize,    // Exec: current position
    }

    #[verifier::reject_recursive_types(V)]
    pub struct SimpleSeqIterGhost<V> { pub pos: int, pub vec: Seq<V>,}

    impl<V> View for SimpleSeqIter<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.vec@.skip(self.pos as int) }
    }

// TODO make it int pair. 
    impl<V> View for SimpleSeqIterGhost<V> {
        type V = Seq<V>;
        open spec fn view(&self) -> Seq<V> { self.vec.skip(self.pos as int) }
    }

    pub trait SimpleSeqTrait<V: Clone>: Sized {
        fn new() -> Self;
        fn push(&mut self, v: V);
        fn iter(&self) -> SimpleSeqIter<V>;
    }

    impl<V: Clone> SimpleSeqTrait<V> for SimpleSeq<V> {
        fn new() -> (s: Self)    ensures s@ == Seq::<V>::empty(), { SimpleSeq { elements: Vec::new() } }
        fn push(&mut self, v: V) ensures self@ == old(self)@.push(v), { self.elements.push(v); }

        fn iter(&self) -> (it: SimpleSeqIter<V>)
           ensures it@ == self.elements@
        { SimpleSeqIter { vec: self.elements.clone(), pos: 0, } }
    }

    impl<V: Clone> Iterator for SimpleSeqIter<V> {
        type Item = V;

        fn next(&mut self) -> (result: Option<V>)
            ensures
                self.vec@ == old(self).vec@,
        ({
            match result {
                None => {
                    &&& old(self).pos >= old(self).vec.len()
                    &&& self.pos == old(self).pos
                },
                 Some(_) => {
                    &&& old(self).pos < old(self).vec.len()
                    &&& self.pos == old(self).pos + 1
                }
            }
        })
        {
            if self.pos < self.vec.len() {
                let elem = self.vec[self.pos].clone();
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    impl<V> vstd::pervasive::ForLoopGhostIteratorNew for SimpleSeqIter<V> {
        type GhostIter = SimpleSeqIterGhost<V>;

        open spec fn ghost_iter(&self) -> SimpleSeqIterGhost<V> 
        { SimpleSeqIterGhost { pos: self.pos as int, vec: self.vec@, } }
    }

    impl<V> vstd::pervasive::ForLoopGhostIterator for SimpleSeqIterGhost<V> {
        type ExecIter = SimpleSeqIter<V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &SimpleSeqIter<V>) -> bool {
            &&& self.pos == exec_iter.pos as int
            &&& self.vec == exec_iter.vec@
            &&& self@ == exec_iter@
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.vec == self.vec
                &&& 0 <= self.pos <= self.vec.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool { self.pos == self.vec.len() }

        open spec fn ghost_decrease(&self) -> Option<int> { Some(self.vec.len() - self.pos) }

        open spec fn ghost_peek_next(&self) -> Option<V> {
            if 0 <= self.pos < self.vec.len() {
                Some(self.vec[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, exec_iter: &SimpleSeqIter<V>) -> Self {
            SimpleSeqIterGhost { pos: self.pos + 1, vec: self.vec, }
        }
    }

/*
    pub fn simple_seq_copy(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
//        ensures
//            s2@ =~= s1@,
    {
        let mut sn = SimpleSeq::new();
        
        for elem in it: s1.iter()
//            invariant
//                it.vec == s1.elements@,
//                sn@ =~= it@,
        {
            sn.push(elem);
        }
        
        sn
    }
*/

    pub fn simple_seq_copy_while(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
//        ensures
//            s2@ == s1@,
    {
        let mut sn                      = SimpleSeq::new();
        let mut it : SimpleSeqIter<u32> = s1.iter();
        let mut done                    = false;

        let ghost it_vec : Seq<u32> = it.vec@;
        assert(it.vec@ == s1.elements@);
        
        while !done
            invariant
                it_vec == it.vec@,
                it.vec@ == s1.elements@,
                sn@ == it@,
            decreases it.vec@.len() - it.pos,
        {
            match it.next() {
                Some(elem) => {
                    sn.push(elem);
                },
                None => {
                    done = true;
                },
            }
        }
        
        sn
    }

/*
    pub fn simple_seq_copy_loop(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ =~= s1@,
    {
        let mut sn = SimpleSeq::new();
        let mut it = s1.iter();
        
        loop
            invariant
                it.vec@ =~= s1.elements@,
                sn@ =~= it@,
            decreases it.vec@.len() - it.pos,
        {
            match it.next() {
                Some(elem) => {
                    sn.push(elem);
                },
                None => {
                    break;
                },
            }
        }
        
        sn
    }
*/

    // This is not using our iter.
    pub fn simple_seq_copy_for_range(s1: &SimpleSeq<u32>) -> (s2: SimpleSeq<u32>)
        ensures
            s2@ =~= s1@,
    {
        let mut sn = SimpleSeq::new();
        let len = s1.elements.len();
        
        for i in 0..len
            invariant
                len == s1.elements.len(),
                sn@ =~= s1.elements@.take(i as int),
        {
            sn.push(s1.elements[i]);
        }
        
        sn
    }

    } // verus!
}

