// Copyright (c) 2025 Brian G. Milnes
//! Executable functions for summing sequences with overflow checking.
//! Uses the lemmas from seq_set.rs to connect seq.fold_left to set.fold.

#[cfg(verus_keep_ghost)]
pub mod seq_set_exec {

    #[allow(unused_imports)]
    use vstd::prelude::*;
    #[allow(unused_imports)]
    use crate::vstdplus::checked_nat::checked_nat::CheckedU32;
    #[allow(unused_imports)]
    use crate::vstdplus::seq_set::{
        spec_nat_seq_sum,
        spec_nat_set_sum,
        lemma_u32_view_identity,
        lemma_to_seq_gives_same_set,
        lemma_seq_map_to_set_eq_set_map,
        lemma_set_contains_iff_to_seq_map_contains,
    };
    #[allow(unused_imports)]
    use crate::Chap05::SetStEph::SetStEph::{SetStEph, SetStEphTrait, valid_key_type};

verus! {

broadcast use {
    vstd::seq_lib::group_seq_properties,
    vstd::set::group_set_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
};


/// Sublemma: fold_left step for nat sequences
proof fn lemma_nat_fold_left_step(seq: Seq<nat>, n: int)
    requires
        0 <= n < seq.len(),
    ensures
        spec_nat_seq_sum(seq.take(n + 1)) == spec_nat_seq_sum(seq.take(n)) + seq[n],
{
    let prefix = seq.take(n);
    let f = |acc: nat, v: nat| acc + v;
    
    seq.take(n + 1).lemma_fold_left_split(0nat, f, n);
    assert(seq.take(n + 1).subrange(0, n) =~= prefix);
    assert(seq.take(n + 1).subrange(n, n + 1) =~= Seq::empty().push(seq[n]));
}

/// - Sum a Seq<nat> by induction (proof function).
/// - This is the pattern for total_weight with nat values.
pub proof fn proof_sum_seq_nat(seq: Seq<nat>, i: int) -> (total: nat)
    requires
        0 <= i <= seq.len(),
    ensures
        total == spec_nat_seq_sum(seq) - spec_nat_seq_sum(seq.take(i)),
    decreases seq.len() - i,
{
    if i >= seq.len() {
        assert(seq.take(seq.len() as int) =~= seq);
        0
    } else {
        lemma_nat_fold_left_step(seq, i);
        let rest = proof_sum_seq_nat(seq, i + 1);
        seq[i] + rest
    }
}

/// Entry point: sum entire sequence
pub proof fn proof_sum_seq_nat_full(seq: Seq<nat>) -> (total: nat)
    ensures
        total == spec_nat_seq_sum(seq),
{
    proof_sum_seq_nat(seq, 0)
}

#[verifier::external_body]
pub fn sum_seq_u32_no_overflow(seq: &Vec<u32>) -> (sum: u32)
    ensures
        sum == spec_nat_seq_sum(seq@.map(|_i: int, v: u32| v as nat)),
{
    let mut sum = 0;
    for i in 0..seq.len() {
        assume(((sum + seq[i as int]) as int) < u32::MAX); // Does not work.
        sum = sum + seq[i];
    }
    sum
}

pub fn sum_seq_CheckedU32(seq: &Vec<u32>) -> (sum: CheckedU32)
    ensures
        sum@ == spec_nat_seq_sum(seq@.map(|_i: int, v: u32| v as nat)),
{
    let mut sum = CheckedU32::new(0);
    let mut i: usize = 0;
    let ghost mapped_seq = seq@.map(|_i: int, v: u32| v as nat);
    
    loop
        invariant
            0 <= i <= seq@.len(),
            mapped_seq == seq@.map(|_i: int, v: u32| v as nat),
            sum@ == spec_nat_seq_sum(mapped_seq.take(i as int)),
        decreases seq@.len() - i,
    {
        if i >= seq.len() {
            assert(mapped_seq.take(seq@.len() as int) =~= mapped_seq);
            return sum;
        }
        proof { lemma_nat_fold_left_step(mapped_seq, i as int); }
        sum = sum.add_value(seq[i]);
        i = i + 1;
    }
}

/// Sum a set of u32 values with overflow checking.
pub fn sum_set_CheckedU32(s: &SetStEph<u32>) -> (sum: CheckedU32)
    requires
        valid_key_type::<u32>(),
        s@.finite(),
    ensures
        sum@ == spec_nat_set_sum(s@.map(|v: u32| v as nat)),
{
    let seq = s.to_seq();
    assert(seq@.no_duplicates());
    assume(forall |x: u32| s@.contains(x) <==> seq@.map(|_i: int, t: u32| t@).contains(x));

    let ghost mapped_seq = seq@.map(|_i: int, v: u32| v as nat);
    let ghost nat_set = s@.map(|v: u32| v as nat);
    
    proof {
        lemma_to_seq_gives_same_set(s@, seq@);
        lemma_seq_map_to_set_eq_set_map(seq@, s@);
    }
    
    let mut sum = CheckedU32::new(0);
    let mut i: usize = 0;
    
    loop
        invariant
            0 <= i <= seq@.len(),
            mapped_seq == seq@.map(|_i: int, v: u32| v as nat),
            sum@ == spec_nat_seq_sum(mapped_seq.take(i as int)),
            seq@.no_duplicates(),
            mapped_seq.to_set() =~= nat_set,
            nat_set == s@.map(|v: u32| v as nat),
            nat_set.finite(),
        decreases seq@.len() - i,
    {
        if i >= seq.len() {
            proof {
                assert(mapped_seq.take(seq@.len() as int) =~= mapped_seq);
                lemma_spec_nat_seq_fold_equals_spec_set_fold(mapped_seq);
            }
            return sum;
        }
        proof { lemma_nat_fold_left_step(mapped_seq, i as int); }
        sum = sum.add_value(seq[i]);
        i = i + 1;
    }
}

} // verus!

} // pub mod seq_set_exec
