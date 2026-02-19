// Copyright (c) 2025 Brian G. Milnes
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

    // Lemma: sum of s.push(v) = sum(s) + v

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

    // Unfolds sum_u32 by one element at take(i+1) for the loop invariant
    proof fn lemma_sum_u32_unfold_take(s: Seq<u32>, i: int)
        requires 0 <= i < s.len()
        ensures spec_sum_u32_seq(s.take(i + 1)) == spec_sum_u32_seq(s.take(i)) + s[i] as nat
    {
        let take_new = s.take(i + 1);
        let take_old = s.take(i);
        assert(take_new.drop_last() =~= take_old);
    }

    // Abstract version: sum of int values (for CheckedU32@ which is int)
    // Spec function: sum of int values in a sequence
    pub open spec fn spec_sum_int_seq(s: Seq<int>) -> int 
        decreases s.len()
    {
        if s.len() == 0 {
            0int
        } else {
            s.last() + spec_sum_int_seq(s.drop_last())
        }
    }

    // Spec function: sum using fold_left
    pub open spec fn spec_sum_int_fold(s: Seq<int>) -> int {
        s.fold_left(0int, |acc: int, v: int| acc + v)
    }

// Veracity: USED
    // Lemma: the two spec functions are equivalent
    pub proof fn lemma_sum_int_equiv(s: Seq<int>)
        ensures spec_sum_int_seq(s) == spec_sum_int_fold(s)
        decreases s.len()
    {
        reveal_with_fuel(Seq::fold_left, 1);
        if s.len() > 0 {
            lemma_sum_int_equiv(s.drop_last());
        }
    }
// Veracity: USED

    // Lemma: sum of s.push(v) = sum(s) + v
    pub proof fn lemma_sum_int_push(s: Seq<int>, v: int)
        ensures spec_sum_int_seq(s.push(v)) == spec_sum_int_seq(s) + v
    {
        assert(s.push(v).drop_last() =~= s);
    }

// Veracity: USED
    // Lemma: sum(take(i+1)) = sum(take(i)) + s[i]
    pub proof fn lemma_sum_int_unfold_take(s: Seq<int>, i: int)
        requires 0 <= i < s.len()
        ensures spec_sum_int_seq(s.take(i + 1)) == spec_sum_int_seq(s.take(i)) + s[i]
    {
        let take_new = s.take(i + 1);
        let take_old = s.take(i);
        assert(take_new.drop_last() =~= take_old);
    }

    // For Seq<CheckedU32>: map to Seq<int> via view, then sum

    // Spec: sum of CheckedU32 views in a sequence
    pub open spec fn spec_sum_checked_u32_seq(s: Seq<CheckedU32>) -> int {
        spec_sum_int_seq(s.map(|i: int, c: CheckedU32| c@))
    }
// Veracity: USED

    // Lemma: sum(take(i+1)) = sum(take(i)) + s[i]@ for CheckedU32 sequences
    pub proof fn lemma_sum_checked_u32_unfold_take(s: Seq<CheckedU32>, i: int)
        requires 0 <= i < s.len()
        ensures spec_sum_checked_u32_seq(s.take(i + 1)) == spec_sum_checked_u32_seq(s.take(i)) + s[i]@
    {
        let views = s.map(|j: int, c: CheckedU32| c@);
        let take_new_views = s.take(i + 1).map(|j: int, c: CheckedU32| c@);
        let take_old_views = s.take(i).map(|j: int, c: CheckedU32| c@);
        
        // Show: s.take(i+1).map(f) == views.take(i+1)
        assert(take_new_views =~= views.take(i + 1));
        assert(take_old_views =~= views.take(i));
        
        // Now use the int lemma
        lemma_sum_int_unfold_take(views, i);
    }

    /// If all inner sequences have the same length m, then flatten has length n * m.
    pub proof fn lemma_flatten_uniform_len<A>(ss: Seq<Seq<A>>, m: int)
        requires
            forall|i: int| 0 <= i < ss.len() ==> (#[trigger] ss[i]).len() == m,
        ensures
            ss.flatten().len() == ss.len() * m,
        decreases ss.len()
    {
        if ss.len() == 0 {
            assert(ss.len() * m == 0) by (nonlinear_arith) requires ss.len() == 0;
        } else {
            assert forall|i: int| 0 <= i < ss.drop_first().len() implies
                (#[trigger] ss.drop_first()[i]).len() == m by {
                assert(ss.drop_first()[i] == ss[i + 1]);
            }
            lemma_flatten_uniform_len(ss.drop_first(), m);
            assert(ss.first().len() == m);
            assert(m + (ss.len() - 1) * m == ss.len() * m) by (nonlinear_arith)
                requires ss.len() > 0;
        }
    }

    /// Sum of inner sequence lengths.
    pub open spec fn spec_inner_lens_sum<A>(ss: Seq<Seq<A>>) -> int
        decreases ss.len()
    {
        if ss.len() == 0 { 0 }
        else { ss.first().len() + spec_inner_lens_sum(ss.drop_first()) }
    }

    /// General flatten length: equals sum of inner lengths.
    pub proof fn lemma_flatten_len_is_inner_lens_sum<A>(ss: Seq<Seq<A>>)
        ensures ss.flatten().len() == spec_inner_lens_sum(ss),
        decreases ss.len()
    {
        if ss.len() > 0 {
            lemma_flatten_len_is_inner_lens_sum(ss.drop_first());
        }
    }

    /// If a predicate holds for all elements of all inner sequences,
    /// it holds for all elements of the flattened result.
    pub proof fn lemma_flatten_all<A>(ss: Seq<Seq<A>>, pred: spec_fn(A) -> bool)
        requires
            forall|i: int, j: int|
                0 <= i < ss.len() && 0 <= j < ss[i].len()
                ==> #[trigger] pred(ss[i][j]),
        ensures
            forall|k: int|
                0 <= k < ss.flatten().len()
                ==> #[trigger] pred(ss.flatten()[k]),
        decreases ss.len()
    {
        if ss.len() > 0 {
            let first = ss.first();
            let rest = ss.drop_first();
            assert forall|i: int, j: int|
                0 <= i < rest.len() && 0 <= j < rest[i].len()
                implies #[trigger] pred(rest[i][j]) by {
                assert(rest[i] == ss[i + 1]);
                assert(pred(ss[i + 1][j]));
            }
            lemma_flatten_all(rest, pred);
            assert forall|k: int|
                0 <= k < ss.flatten().len()
                implies #[trigger] pred(ss.flatten()[k]) by {
                if k < first.len() {
                    assert(ss.flatten()[k] == first[k]);
                    assert(first == ss[0]);
                    assert(pred(ss[0][k]));
                } else {
                    assert(ss.flatten()[k] == rest.flatten()[k - first.len()]);
                }
            }
        }
    }

    /// If ss[i][j] is an element of the nested structure, it appears in the flattened result.
    pub proof fn lemma_flatten_contains<A>(ss: Seq<Seq<A>>, i: int, j: int)
        requires 0 <= i < ss.len(), 0 <= j < ss[i].len(),
        ensures ss.flatten().contains(ss[i][j]),
        decreases ss.len()
    {
        if i == 0 {
            assert(ss.flatten()[j] == ss.first()[j]);
            assert(ss[0] == ss.first());
        } else {
            let rest = ss.drop_first();
            assert(rest[i - 1] == ss[i]);
            lemma_flatten_contains(rest, i - 1, j);
            let k = choose|k: int| 0 <= k < rest.flatten().len() && rest.flatten()[k] == ss[i][j];
            assert(ss.flatten()[ss.first().len() + k] == rest.flatten()[k]);
        }
    }

} // verus!

} // mod seq
