// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Shared spec functions and proof lemmas for dynamic programming algorithms (Chapter 51).
//! All eight DP variant files import these instead of duplicating them.

//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  6. spec fns
//  7. proof fns/broadcast groups

pub mod SeqSpecsAndLemmas {

    use vstd::prelude::*;

    verus! {

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        vstd::map::group_map_axioms,
    };

    //		Section 6. spec fns


    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    /// Minimum edit distance spec — standalone so SMT congruence works across state changes.
    pub open spec fn spec_med_fn(s: Seq<char>, t: Seq<char>, i: nat, j: nat) -> nat
        decreases i + j,
    {
        if i == 0 { j }
        else if j == 0 { i }
        else if s[i as int - 1] == t[j as int - 1] {
            spec_med_fn(s, t, (i - 1) as nat, (j - 1) as nat)
        } else {
            let del = spec_med_fn(s, t, (i - 1) as nat, j);
            let ins = spec_med_fn(s, t, i, (j - 1) as nat);
            1 + spec_min(del, ins)
        }
    }

    /// Memo correctness: every cached value equals the spec.
    pub open spec fn spec_memo_correct(
        memo: Map<(usize, usize), usize>,
        s: Seq<char>,
        t: Seq<char>,
    ) -> bool {
        forall|a: usize, b: usize| #[trigger] memo.contains_key((a, b)) ==>
            memo[(a, b)] as nat == spec_med_fn(s, t, a as nat, b as nat)
    }

    //		Section 7. proof fns/broadcast groups


    pub proof fn lemma_spec_med_fn_bounded(s: Seq<char>, t: Seq<char>, i: nat, j: nat)
        ensures spec_med_fn(s, t, i, j) <= i + j,
        decreases i + j,
    {
        if i == 0 || j == 0 {
        } else if s[i as int - 1] == t[j as int - 1] {
            lemma_spec_med_fn_bounded(s, t, (i - 1) as nat, (j - 1) as nat);
        } else {
            lemma_spec_med_fn_bounded(s, t, (i - 1) as nat, j);
            lemma_spec_med_fn_bounded(s, t, i, (j - 1) as nat);
        }
    }

    } // verus!
} // mod
