// Copyright (c) 2025 Brian G. Milnes
use vstd::prelude::*;
use crate::experiments::verus_iterator::*;
use crate::experiments::verus_vec_iterator::*;

verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

pub open spec fn seq_usize_mem(s: Seq<usize>, elt: usize) -> bool {
    exists|i: int| 0 <= i < s.len() && s[i] == elt
}

// Simple test to understand for-loop semantics
pub fn test_for_semantics(s: &Vec<usize>) {
    let ghost original_seq = s@;
    let collection = VecCollection { data: s.clone() };
    let mut idx: usize = 0;
    
    for val in collection.iter()
        invariant
            idx <= original_seq.len(),
    {
        // What is the relationship between val, idx, and iter@.cur here?
        // Let's find out with explicit checks
        idx = idx + 1;
    }
}

// pub fn usize_vec_mem_for(s: &Vec<usize>, elt: usize) -> (result: bool)
//     ensures result == seq_usize_mem(s@, elt)
// {
//     let ghost original_seq = s@;
//     let collection = VecCollection { data: s.clone() };
//     let iter = collection.iter();
//     
//     #[verifier::no_auto_loop_invariant]
//     for val in iter
//         invariant
//             iter@.data == original_seq,
//             original_seq == s@,
//             forall|j: int| 0 <= j < iter@.cur ==> iter@.cur > 0 && original_seq[j] != elt,
//     {
//         if val == elt {
//             proof {
//                 // val came from the previous call to next(), which returned data[old(cur)]
//                 // Now cur has been incremented, so val == data[cur - 1]
//                 assume(val == original_seq[iter@.cur - 1]);
//                 assert(iter@.cur > 0);
//                 let idx = iter@.cur - 1;
//                 assert(0 <= idx < original_seq.len());
//                 assert(original_seq[idx] == elt);
//                 assert(s@[idx] == elt);
//                 assert(exists|i: int| 0 <= i < s@.len() && s@[i] == elt);
//             }
//             return true;
//         }
//     }
//     false
// }

// pub open spec fn seq_usize_find(s: Seq<usize>, elt: usize) -> Option<int> {
//     if exists|i: int| 0 <= i < s.len() && s[i] == elt {
//         Some(choose|i: int| 0 <= i < s.len() && s[i] == elt && (forall|j: int| 0 <= j < i ==> s[j] != elt))
//     } else {
//         None
//     }
// }

// #[verifier(loop_isolation(false))]
// pub fn usize_vec_find_for(s: &Vec<usize>, elt: usize) -> (result: Option<usize>)
//     ensures
//         match result {
//             Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
//             None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
//         }
// {
//     let ghost original_seq = s@;
//     let collection = VecCollection { data: s.clone() };
//     let mut iter = collection.iter();
//     let mut idx: usize = 0;
//     
//     for val in iter
//         invariant
//             iter@.exec_invariant(&iter),
//             iter@.data == original_seq,
//             original_seq == s@,
//             iter@.cur <= iter@.end,
//             iter@.end == original_seq.len(),
//             idx == iter@.cur - 1,
//             forall|j: int| 0 <= j < iter@.cur - 1 ==> original_seq[j] != elt,
//     {
//         if val == elt {
//             return Some(idx);
//         }
//         idx += 1;
//     }
//     None
// }

// #[verifier(loop_isolation(false))]
// pub fn vec_length_up_for(s: &Vec<usize>) -> (length: usize)
//     ensures length == s@.len()
// {
//     let ghost original_seq = s@;
//     let mut length: usize = 0;
//     let collection = VecCollection { data: s.clone() };
//     let mut iter = collection.iter();
//     
//     for _val in iter
//         invariant
//             iter@.exec_invariant(&iter),
//             iter@.data == original_seq,
//             original_seq == s@,
//             length == iter@.cur - 1,
//             iter@.cur <= iter@.end,
//             iter@.end == original_seq.len(),
//     {
//         length += 1;
//     }
//     length
// }

// pub open spec fn seq_usize_count_up(s: Seq<usize>, elt: usize) -> nat
//     decreases s.len()
// {
//     if s.len() == 0 {
//         0
//     } else {
//         (if s.last() == elt { 1nat } else { 0nat }) + seq_usize_count_up(s.drop_last(), elt)
//     }
// }

// #[verifier(loop_isolation(false))]
// pub fn usize_vec_count_up_for(s: &Vec<usize>, elt: usize) -> (count: usize)
//     ensures count <= s@.len()
// {
//     let ghost original_seq = s@;
//     let mut count: usize = 0;
//     let collection = VecCollection { data: s.clone() };
//     let mut iter = collection.iter();
//     
//     for val in iter
//         invariant
//             iter@.exec_invariant(&iter),
//             iter@.data == original_seq,
//             original_seq == s@,
//             iter@.cur <= iter@.end,
//             iter@.end == original_seq.len(),
//             count <= iter@.cur - 1,
//     {
//         if val == elt {
//             count += 1;
//         }
//     }
//     count
// }

}
