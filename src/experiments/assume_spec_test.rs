//! Test: Can we use assume_specification on our own Iterator::next with requires?

use vstd::prelude::*;

verus! {

#[verifier::reject_recursive_types(V)]
pub struct TestIter<V> {
    pub vec: Vec<V>,
    pub pos: usize,
}

pub open spec fn iter_invariant<V>(it: &TestIter<V>) -> bool {
    it.pos <= it.vec@.len()
}

impl<V> View for TestIter<V> {
    type V = (int, Seq<V>);
    open spec fn view(&self) -> (int, Seq<V>) {
        (self.pos as int, self.vec@)
    }
}

// Try to use assume_specification with requires on our own type
pub assume_specification<V: Clone>[ TestIter::<V>::next ](it: &mut TestIter<V>) -> (result: Option<V>)
    requires
        iter_invariant(&old(it)),  // Try adding requires!
    ensures
        iter_invariant(it),
        ({
            let (old_index, old_seq) = old(it)@;
            match result {
                None => {
                    &&& it@ == old(it)@
                    &&& old_index == old_seq.len()
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
;

impl<V: Clone> Iterator for TestIter<V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        if self.pos < self.vec.len() {
            let elem = self.vec[self.pos].clone();
            self.pos = self.pos + 1;
            Some(elem)
        } else {
            None
        }
    }
}

pub fn test() {
    let mut it = TestIter { vec: vec![1u32, 2, 3], pos: 0 };
    let x = it.next();
    assert(x == Some(1u32));
}


} // verus!

