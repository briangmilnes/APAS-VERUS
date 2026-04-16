// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Extensions to vstd::arithmetic::power2 for powers of 2.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 7. proof fns/broadcast groups

#[cfg(verus_keep_ghost)]

//		Section 1. module

pub mod power2_plus {

    //		Section 2. imports

    use vstd::prelude::*;
    use vstd::arithmetic::power2::{pow2, lemma_pow2_pos, lemma_pow2_strictly_increases, lemma2_to64_rest};

    verus! 
{

    //		Section 7. proof fns/broadcast groups


        /// pow2 is monotonic: if a <= b then pow2(a) <= pow2(b)
        pub proof fn lemma_pow2_mono(a: nat, b: nat)
            requires a <= b,
            ensures pow2(a) <= pow2(b),
        {
            if a < b {
                lemma_pow2_strictly_increases(a, b);
            }
        }

        /// - pow2(46) < u64::MAX
        /// - This is provable via lemma2_to64_rest which gives pow2(46) == 0x400000000000
        pub proof fn lemma_pow2_46_lt_u64_max()
            ensures pow2(46) < u64::MAX as nat,
        {
            lemma2_to64_rest();
        }

        /// pow2(63) < u64::MAX (since u64::MAX = 2^64 - 1)
        pub proof fn lemma_pow2_63_lt_u64_max()
            ensures pow2(63) < u64::MAX as nat,
        {
            lemma2_to64_rest();
        }

        /// pow2(n) < u64::MAX for n < 64
        pub proof fn lemma_pow2_lt_u64_max(n: nat)
            requires n < 64,
            ensures pow2(n) < u64::MAX as nat,
        {
            lemma_pow2_63_lt_u64_max();
            if n < 63 {
                lemma_pow2_strictly_increases(n, 63);
            }
        }

        /// pow2(31) < u32::MAX
        pub proof fn lemma_pow2_31_lt_u32_max()
            ensures pow2(31) < u32::MAX as nat,
        {
            vstd::arithmetic::power2::lemma2_to64();
        }

        /// pow2(n) < u32::MAX for n < 32
        pub proof fn lemma_pow2_lt_u32_max(n: nat)
            requires n < 32,
            ensures pow2(n) < u32::MAX as nat,
        {
            lemma_pow2_31_lt_u32_max();
            if n < 31 {
                lemma_pow2_strictly_increases(n, 31);
            }
        }

    } // verus!
} // mod
