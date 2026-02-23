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

pub fn usize_vec_mem_loop(s: &Vec<usize>, elt: usize) -> (result: bool)
    ensures result == seq_usize_mem(s@, elt)
{
    let ghost original_seq = s@;
    let collection = VecCollection { data: s.clone() };
    let mut iter = collection.iter();
    
    loop
        invariant
            iter@.exec_invariant(&iter),
            iter@.data == original_seq,
            original_seq == s@,
            iter@.cur <= iter@.end,
            iter@.end == original_seq.len(),
            forall|j: int| 0 <= j < iter@.cur ==> original_seq[j] != elt,
        decreases iter.data.len() - iter.cur,
    {
        if iter.cur >= iter.data.len() {
            return false;
        }
        if iter.data[iter.cur] == elt {
            assert(s@[iter@.cur] == elt);
            assert(exists|i: int| 0 <= i < s@.len() && s@[i] == elt);
            return true;
        }
        VecCollection::next(&mut iter);
    }
}

pub open spec fn seq_usize_find(s: Seq<usize>, elt: usize) -> Option<int> {
    if exists|i: int| 0 <= i < s.len() && s[i] == elt {
        Some(choose|i: int| 0 <= i < s.len() && s[i] == elt && (forall|j: int| 0 <= j < i ==> s[j] != elt))
    } else {
        None
    }
}

pub fn usize_vec_find_loop(s: &Vec<usize>, elt: usize) -> (result: Option<usize>)
    ensures
        match result {
            Some(i) => i < s@.len() && s@[i as int] == elt && (forall|j: int| 0 <= j < i ==> s@[j] != elt),
            None => forall|j: int| 0 <= j < s@.len() ==> s@[j] != elt,
        }
{
    let ghost original_seq = s@;
    let collection = VecCollection { data: s.clone() };
    let mut iter = collection.iter();
    
    loop
        invariant
            iter@.exec_invariant(&iter),
            iter@.data == original_seq,
            original_seq == s@,
            iter@.cur <= iter@.end,
            iter@.end == original_seq.len(),
            forall|j: int| 0 <= j < iter@.cur ==> original_seq[j] != elt,
        decreases iter.data.len() - iter.cur,
    {
        if iter.cur >= iter.data.len() {
            return None;
        }
        if iter.data[iter.cur] == elt {
            let found_idx = iter.cur;
            assert(found_idx < s@.len());
            assert(s@[found_idx as int] == elt);
            assert(forall|j: int| 0 <= j < found_idx ==> s@[j] != elt);
            return Some(found_idx);
        }
        VecCollection::next(&mut iter);
    }
}

pub fn vec_length_up_loop(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let ghost original_seq = s@;
    let mut length: usize = 0;
    let collection = VecCollection { data: s.clone() };
    let mut iter = collection.iter();
    
    loop
        invariant
            iter@.exec_invariant(&iter),
            iter@.data == original_seq,
            original_seq == s@,
            length == iter@.cur,
            iter@.cur <= iter@.end,
            iter@.end == original_seq.len(),
        decreases iter.data.len() - iter.cur,
    {
        if iter.cur >= iter.data.len() {
            return length;
        }
        let _result = VecCollection::next(&mut iter);
        length += 1;
    }
}

pub open spec fn seq_usize_count_up(s: Seq<usize>, elt: usize) -> nat
    decreases s.len()
{
    if s.len() == 0 {
        0
    } else {
        (if s.last() == elt { 1nat } else { 0nat }) + seq_usize_count_up(s.drop_last(), elt)
    }
}

pub fn usize_vec_count_up_loop(s: &Vec<usize>, elt: usize) -> (count: usize)
    ensures count <= s@.len()
{
    let ghost original_seq = s@;
    let mut count: usize = 0;
    let collection = VecCollection { data: s.clone() };
    let mut iter = collection.iter();
    
    loop
        invariant
            iter@.exec_invariant(&iter),
            iter@.data == original_seq,
            original_seq == s@,
            iter@.cur <= iter@.end,
            iter@.end == original_seq.len(),
            count <= iter@.cur,
        decreases iter.data.len() - iter.cur,
    {
        if iter.cur >= iter.data.len() {
            return count;
        }
        if iter.data[iter.cur] == elt {
            count += 1;
        }
        VecCollection::next(&mut iter);
    }
}

}
