// Copyright (c) 2025 Brian G. Milnes
//! Test: Can we prove iter() honors invariant and next() works with that invariant as a precondition?

use vstd::prelude::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

use vstd::std_specs::clone::*;

#[verifier::reject_recursive_types(V)]
pub struct TestIter<V> {
    pub vec: Vec<V>,
    pub pos: usize,
}

// The invariant we want to maintain
pub open spec fn iter_invariant<V>(iter: &TestIter<V>) -> bool {
    iter.pos <= iter.vec@.len()
}

impl<V> View for TestIter<V> {
    type V = (int, Seq<V>);
    open spec fn view(&self) -> (int, Seq<V>) {
        (self.pos as int, self.vec@)
    }
}

// Prove: iter() creates an iterator satisfying the invariant
pub fn new_iter<V>(vec: Vec<V>) -> (it: TestIter<V>)
    ensures
        it@ == (0int, vec@),
        iter_invariant(&it),  // PROVE: invariant holds initially
{
    TestIter { vec, pos: 0 }
}

// Version of next() that REQUIRES the invariant and ENSURES it's preserved
pub fn next_with_invariant<V: Clone>(it: &mut TestIter<V>) -> (result: Option<V>)
    requires
        iter_invariant(&old(it)),  // REQUIRE: invariant on entry
    ensures
        iter_invariant(it),  // ENSURE: invariant on exit
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
        // Prove invariant preserved: old pos < vec.len, so new pos <= vec.len
        assert(it.pos <= it.vec.len());
        Some(elem)
    } else {
        // From requires: it.pos <= it.vec.len()
        // From if condition: it.pos >= it.vec.len()
        // Therefore: it.pos == it.vec.len()
        assert(it.pos == it.vec.len());  // Should prove without assume!
        None
    }
}
o
// Lemma: If two sequences have same length and element-wise equality, they're equal
proof fn lemma_iter_lengths<V>(s1: Seq<V>, s2: Seq<V>)
    requires
        s1.len() == s2.len(),
        forall |j: int| 0 <= j < s1.len() ==> #[trigger] s1[j] == s2[j],
    ensures
        s1 == s2,
{
    assert(s1 =~= s2);
}

// Prove: Using this next with the invariant works for a copy loop
pub fn test_copy(vec: &Vec<u32>) -> (result: Vec<u32>)
    ensures result@ == vec@,
{
    let ghost orig_seq = vec@;
    let mut it = new_iter(vec.clone());
    let mut result = Vec::new();
    
    assert(it@.1 == orig_seq);  // vec.clone()@ == vec@
    
    loop
        invariant
            iter_invariant(&it),  // Maintained throughout
            it@.1 == orig_seq,
            orig_seq == vec@,  // ADD: maintain connection to vec
            result@.len() == it@.0,
            forall |j: int| #![trigger result@[j]] 0 <= j < it@.0 ==> result@[j] == orig_seq[j],
        decreases orig_seq.len() - it@.0,
    {
        match next_with_invariant(&mut it) {
            Some(elem) => {
                result.push(elem);
            },
            None => {
                // From None postcondition: it@.0 == orig_seq.len()
                // From loop invariant: result@.len() == it@.0
                // Therefore: result@.len() == orig_seq.len()
                // And: forall j in 0..result@.len(), result@[j] == orig_seq[j]
                proof {
                    assert(result@.len() == orig_seq.len());
                    assert(forall |j: int| 0 <= j < result@.len() ==> #[trigger] result@[j] == orig_seq[j]);
                    lemma_iter_lengths(result@, orig_seq);
                    assert(result@ == orig_seq);
                    assert(orig_seq == vec@);
                    assert(result@ == vec@);
                }
                return result;
            },
        }
    }
}

} // verus!
