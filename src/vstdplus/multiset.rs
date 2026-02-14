// Copyright (c) 2025 Brian G. Milnes
//! Seq-level recursive filter-length spec and lemmas.
//!
//! Pattern adapted from human-eval-verus (human_eval_146.rs).
//! Instead of going through multisets, define a recursive spec function on Seq
//! that computes the length of the filtered result, then track it as a loop invariant.
//!
//! - `spec_filter_len` — length of the filtered subsequence (elements satisfying pred)
//! - Loop invariant pattern: `counter == spec_filter_len(seq.subrange(0, index))`

pub mod multiset {

use vstd::prelude::*;

verus! {

/// Recursive spec: length of the subsequence of s whose elements satisfy pred.
pub open spec fn spec_filter_len<T>(s: Seq<T>, pred: spec_fn(T) -> bool) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        spec_filter_len(s.drop_last(), pred)
            + if pred(s.last()) { 1 as int } else { 0 as int }
    }
}

/// spec_filter_len is non-negative.
pub proof fn lemma_spec_filter_len_nonneg<T>(s: Seq<T>, pred: spec_fn(T) -> bool)
    ensures spec_filter_len(s, pred) >= 0,
    decreases s.len(),
{
    if s.len() > 0 {
        lemma_spec_filter_len_nonneg(s.drop_last(), pred);
    }
}

/// spec_filter_len is bounded by the sequence length.
pub proof fn lemma_spec_filter_len_le_len<T>(s: Seq<T>, pred: spec_fn(T) -> bool)
    ensures spec_filter_len(s, pred) <= s.len(),
    decreases s.len(),
{
    if s.len() > 0 {
        lemma_spec_filter_len_le_len(s.drop_last(), pred);
    }
}

/// Connect flatten of 0-or-1 inner seqs to spec_filter_len.
///
/// If `ss` is a sequence of sequences where each `ss[i]` has length 0 or 1, and
/// `ss[i].len() == 1` iff `pred(s[i])`, then `ss.flatten().len() == spec_filter_len(s, pred)`.
pub proof fn lemma_flatten_01_eq_spec_filter_len<T>(
    s: Seq<T>,
    ss: Seq<Seq<T>>,
    pred: spec_fn(T) -> bool,
)
    requires
        ss.len() == s.len(),
        forall|i: int| #![trigger ss[i]] 0 <= i < s.len() ==> ss[i].len() <= 1,
        forall|i: int| #![trigger ss[i]] 0 <= i < s.len() ==> (ss[i].len() == 1 <==> pred(s[i])),
    ensures
        ss.flatten().len() == spec_filter_len(s, pred),
    decreases s.len(),
{
    broadcast use Seq::<_>::group_seq_flatten;

    if s.len() == 0 {
        assert(ss =~= Seq::<Seq<T>>::empty());
    } else {
        let ss_prefix = ss.drop_last();
        let s_prefix = s.drop_last();

        // Recurse on the prefix.
        lemma_flatten_01_eq_spec_filter_len(s_prefix, ss_prefix, pred);

        // ss == ss_prefix.push(ss.last()), so flatten decomposes via lemma_flatten_push.
        assert(ss =~= ss_prefix.push(ss.last()));
        // lemma_flatten_push: ss_prefix.push(ss.last()).flatten() =~= ss_prefix.flatten() + ss.last()
        // So ss.flatten().len() == ss_prefix.flatten().len() + ss.last().len()
    }
}

} // verus!

} // mod multiset
