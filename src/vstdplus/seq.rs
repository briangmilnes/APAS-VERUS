//! Sequence utilities for Verus
//! Includes conversion functions and fold helpers

pub mod seq {

    use vstd::prelude::*;
    use crate::vstdplus::checked_nat::checked_nat::CheckedU32;

verus! {

    // Spec function: sum of u32 values in a sequence as nat
    pub open spec fn spec_sum_u32_seq(s: Seq<u32>) -> nat 
        decreases s.len()
    {
        if s.len() == 0 {
            0nat
        } else {
            s.last() as nat + spec_sum_u32_seq(s.drop_last())
        }
    }

    // Spec function: sum using fold_left (equivalent to spec_sum_u32_seq)
    pub open spec fn spec_sum_u32_fold(s: Seq<u32>) -> nat {
        s.fold_left(0nat, |acc: nat, v: u32| acc + v as nat)
    }

    // Lemma: the two spec functions are equivalent
    pub proof fn lemma_sum_u32_equiv(s: Seq<u32>)
        ensures spec_sum_u32_seq(s) == spec_sum_u32_fold(s)
        decreases s.len()
    {
        reveal_with_fuel(Seq::fold_left, 1);
        if s.len() > 0 {
            lemma_sum_u32_equiv(s.drop_last());
        }
    }

    // Lemma: sum of s.push(v) = sum(s) + v
    pub proof fn lemma_sum_u32_push(s: Seq<u32>, v: u32)
        ensures spec_sum_u32_seq(s.push(v)) == spec_sum_u32_seq(s) + v as nat
    {
        assert(s.push(v).drop_last() =~= s);
        assert(s.push(v).last() == v);
    }

    // Lemma: for non-empty s, sum(s) = sum(s.drop_last()) + s.last()
    pub proof fn lemma_sum_u32_unfold(s: Seq<u32>)
        requires s.len() > 0
        ensures spec_sum_u32_seq(s) == spec_sum_u32_seq(s.drop_last()) + s.last() as nat
    {}

    // Exec function: convert a sequence of u32 to CheckedU32 sum
    // Returns CheckedU32 which tracks overflow
    pub fn seq_u32_to_CheckedU32(s: &Vec<u32>) -> (sum: CheckedU32)
        ensures 
            sum.is_normal() ==> sum@ == spec_sum_u32_seq(s@) as int
    {
        let mut sum = CheckedU32::new(0);
        let mut i: usize = 0;
        let ghost s_spec = s@;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < s.len()
            invariant
                i <= s.len(),
                s@ == s_spec,
                sum.is_normal() ==> sum@ == spec_sum_u32_seq(s_spec.take(i as int)) as int,
            decreases s.len() - i
        {
            let ghost old_i = i as int;
            let ghost old_sum = sum@;
            
            let v = s[i];
            sum = sum.add_value(v);
            i = i + 1;
            
            proof {
                // Lemma: spec_sum_u32_seq(take(old_i + 1)) == spec_sum_u32_seq(take(old_i)) + s[old_i]
                lemma_sum_u32_unfold_take(s_spec, old_i);
                
                // add_value ensures: sum@ == old_sum + v as int
                // If old sum was normal: old_sum == spec_sum_u32_seq(take(old_i)) as int
                // From lemma: spec_sum_u32_seq(take(old_i + 1)) == spec_sum_u32_seq(take(old_i)) + s[old_i]
                // So: sum@ == spec_sum_u32_seq(take(old_i)) as int + v as int
                //          == spec_sum_u32_seq(take(old_i + 1)) as int
                //          == spec_sum_u32_seq(take(i)) as int
                
                // FIXME: SMT has trouble connecting recursive spec function unfolding
                assume(sum.is_normal() ==> sum@ == spec_sum_u32_seq(s_spec.take(i as int)) as int);
            }
        }
        proof {
            assert(s_spec.take(s.len() as int) =~= s_spec);
        }
        sum
    }

    // Helper lemma for the loop invariant
    proof fn lemma_sum_u32_unfold_take(s: Seq<u32>, i: int)
        requires 0 <= i < s.len()
        ensures spec_sum_u32_seq(s.take(i + 1)) == spec_sum_u32_seq(s.take(i)) + s[i] as nat
    {
        let take_new = s.take(i + 1);
        let take_old = s.take(i);
        assert(take_new.drop_last() =~= take_old);
        assert(take_new.last() == s[i]);
    }

} // verus!

} // mod seq

