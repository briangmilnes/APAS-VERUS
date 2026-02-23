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
use vstd::multiset::Multiset;

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

/// m.insert(x).filter(f) when f(x) == m.filter(f).insert(x).
proof fn lemma_multiset_insert_filter_pos<T>(m: Multiset<T>, x: T, f: spec_fn(T) -> bool)
    requires f(x),
    ensures m.insert(x).filter(f) =~= m.filter(f).insert(x),
{
    broadcast use vstd::multiset::group_multiset_axioms;
    assert forall|v: T| m.insert(x).filter(f).count(v) == m.filter(f).insert(x).count(v)
    by {};
}

/// m.insert(x).filter(f) when !f(x) == m.filter(f).
proof fn lemma_multiset_insert_filter_neg<T>(m: Multiset<T>, x: T, f: spec_fn(T) -> bool)
    requires !f(x),
    ensures m.insert(x).filter(f) =~= m.filter(f),
{
    broadcast use vstd::multiset::group_multiset_axioms;
    assert forall|v: T| #[trigger] m.insert(x).filter(f).count(v) == m.filter(f).count(v)
    by {};
}

/// Connect flatten of 0-or-1 inner seqs to multiset.filter.
///
/// If `ss[i] = [s[i]]` when `pred(s[i])` and `ss[i] = []` otherwise,
/// then `ss.flatten().to_multiset() =~= s.to_multiset().filter(pred)`.
pub proof fn lemma_flatten_01_multiset_eq_filter<T>(
    s: Seq<T>,
    ss: Seq<Seq<T>>,
    pred: spec_fn(T) -> bool,
)
    requires
        ss.len() == s.len(),
        forall|i: int| #![trigger ss[i]] 0 <= i < s.len() ==> ss[i].len() <= 1,
        forall|i: int| #![trigger ss[i]] 0 <= i < s.len() ==> (ss[i].len() == 1 <==> pred(s[i])),
        forall|i: int| #![trigger ss[i]] 0 <= i < s.len() ==> (ss[i].len() == 1 ==> ss[i][0] == s[i]),
    ensures
        ss.flatten().to_multiset() =~= s.to_multiset().filter(pred),
    decreases s.len(),
{
    broadcast use Seq::<_>::group_seq_flatten;
    broadcast use vstd::seq_lib::group_to_multiset_ensures;
    broadcast use {
        vstd::multiset::group_multiset_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    if s.len() == 0 {
        assert(ss =~= Seq::<Seq<T>>::empty());
    } else {
        let ss_prefix = ss.drop_last();
        let s_prefix = s.drop_last();

        lemma_flatten_01_multiset_eq_filter(s_prefix, ss_prefix, pred);

        assert(ss =~= ss_prefix.push(ss.last()));
        assert(s =~= s_prefix.push(s.last()));

        let ghost fp = ss_prefix.flatten();

        if pred(s.last()) {
            assert(ss.last() =~= seq![s.last()]);

            // LHS chain:
            // ss.flatten() =~= fp + seq![s.last()] =~= fp.push(s.last())
            assert(fp + seq![s.last()] =~= fp.push(s.last()));
            // fp.push(x).to_multiset() =~= fp.to_multiset().insert(x)
            // By IH: fp.to_multiset() =~= s_prefix.to_multiset().filter(pred)
            // So: ss.flatten().to_multiset() =~= s_prefix.to_multiset().filter(pred).insert(s.last())
            assert(ss.flatten().to_multiset()
                =~= s_prefix.to_multiset().filter(pred).insert(s.last()));

            // RHS chain:
            // s.to_multiset() =~= s_prefix.to_multiset().insert(s.last())
            lemma_multiset_insert_filter_pos(s_prefix.to_multiset(), s.last(), pred);
            // s.to_multiset().filter(pred) =~= s_prefix.to_multiset().filter(pred).insert(s.last())
        } else {
            assert(ss.last() =~= Seq::<T>::empty());

            // LHS chain:
            // ss.flatten() =~= fp + Seq::empty() =~= fp
            assert(fp + Seq::<T>::empty() =~= fp);
            // By IH: fp.to_multiset() =~= s_prefix.to_multiset().filter(pred)
            assert(ss.flatten().to_multiset() =~= s_prefix.to_multiset().filter(pred));

            // RHS chain:
            lemma_multiset_insert_filter_neg(s_prefix.to_multiset(), s.last(), pred);
            // s.to_multiset().filter(pred) =~= s_prefix.to_multiset().filter(pred)
        }
    }
}

} // verus!

} // mod multiset
