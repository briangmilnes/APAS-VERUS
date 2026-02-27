// Copyright (c) 2025 Brian G. Milnes
//! Proof: Can we prove a ToVec-like function on simpler types?

pub mod ToVecProof {
    use vstd::prelude::*;

    verus! {

#[cfg(verus_keep_ghost)]
use vstd::std_specs::hash::obeys_key_model;

#[cfg(verus_keep_ghost)]
        broadcast use {vstd::std_specs::hash::group_hash_axioms, crate::vstdplus::clone_view::clone_view::group_clone_view_axioms};

        pub fn vec_to_vec<T: Copy>(input: &Vec<T>) -> (result: Vec<T>)
            ensures 
                result@.len() == input@.len(),
                forall |i: int| #![trigger result@[i]] 0 <= i < result@.len() ==> result@[i] == input@[i]
        {
            let ghost input_seq = input@;
            let mut v: Vec<T> = Vec::new();
            
            for x in it: input.iter()
                invariant
                    it.elements == input_seq,
                    v@.len() == it.pos,
                    it.pos <= input_seq.len(),
                    forall |j: int| #![trigger v@[j]] 0 <= j < it.pos ==> v@[j] == input_seq[j],
            {
                v.push(*x);
            }
            v
        }

/*
        // While loop version (does not verify - kept for reference)
        pub fn vec_to_vec_while<T: Copy>(input: &Vec<T>) -> (result: Vec<T>)
        {
            if input.len() == 0 {
                return Vec::new();
            }
            else {
                let ghost input_seq = input@;
                let ghost input_seq_len = input@.len();
                let mut v: Vec<T> = Vec::new();
                let mut iter: std::slice::Iter<'_, T> = input.iter();
                // After creating iterator from Vec, these should be true:
                assert(iter@.0 == 0);
                assert(iter@.1 == input_seq);
                assert(v@.len() == 0);
                let mut more = true;
                while more
                    invariant
                              iter@.1 == input_seq,
                              v@.len() == iter@.0,
                              iter@.0 <= iter@.1.len(),
                              more ==> iter@.0 < iter@.1.len(),
                //            forall |j: int| #![trigger v@[j]] 0 <= j < iter@.0 ==> v@[j] == iter@.1[j],
                decreases input_seq_len - iter@.0,
                { 
                   // Before calling next(), at the start of loop body:
                   let ghost old_pos = iter@.0;
                   let ghost old_seq = iter@.1;
                   match iter.next() {
                        Some(x) => {
                            assert(iter@.0 == old_pos + 1);
                            assert(iter@.1 == old_seq);
                            assert(*x == old_seq[old_pos]);
                            assert(old_pos < old_seq.len());
                            v.push(*x);
                        },
                        None => {
                            assert(iter@.0 >= iter@.1.len());
                            more = false;
                        }
                    }
                }
                v
            }
        }
*/
        } // verus!
}
