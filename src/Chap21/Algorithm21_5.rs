//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21 — Algorithm 21.5: Brute Force Solution to the Primes Problem.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	9. impls

//		1. module

pub mod Algorithm21_5 {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap21::Exercise21_8::Exercise21_8::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		4. type definitions

    pub type T = usize;

    //		9. impls

    /// Algorithm 21.5 (Brute Force Solution to the Primes Problem)
    /// primesBF n = { i in 2..n : isPrime(i) }
    ///
    /// Finds all prime numbers less than n using brute force primality testing.
    ///
    /// - APAS: Work Θ(n^{3/2}), Span Θ(lg n)
    /// - Claude-Opus-4.6: Work Θ(n^{3/2}), Span Θ(n^{3/2}) — sequential StPer tabulate + filter.
    pub fn primes_bf(n: usize) -> (primes: ArraySeqStPerS<usize>)
        ensures
            n <= 2 ==> primes.spec_len() == 0,
            n > 2  ==> primes.spec_len() <= n - 2,
            forall|i: int| 0 <= i < primes.spec_len()
                ==> spec_is_prime(#[trigger] primes.spec_index(i) as int),
            forall|p: int| 2 <= p < n as int && spec_is_prime(p) ==>
                Seq::new(primes.spec_len(), |i: int| primes.spec_index(i))
                    .contains(#[trigger] (p as usize)),
    {
        if n <= 2 {
            return ArraySeqStPerS::from_vec(Vec::new());
        }
        let all: ArraySeqStPerS<usize> = ArraySeqStPerS::tabulate(
            &(|i: usize| -> (v: usize)
                requires i < n - 2,
                ensures v == i + 2,
            { i + 2 }),
            n - 2,
        );
        let pred = |x: &usize| -> (keep: bool)
            ensures keep == spec_is_prime(*x as int),
        { is_prime(*x) };
        let ghost spec_pred: spec_fn(usize) -> bool = |x: usize| spec_is_prime(x as int);
        let filtered: ArraySeqStPerS<usize> = ArraySeqStPerS::filter(
            &all,
            &pred,
            Ghost(spec_pred),
        );
        proof {
            let ghost all_seq = Seq::new(all.seq@.len(), |i: int| all.seq@[i]);
            let ghost filt_seq = Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i));
            // Filter ensures: filt_seq.to_multiset() =~= all_seq.to_multiset().filter(spec_pred)
            assert forall|p: int| 2 <= p < n as int && spec_is_prime(p) implies
                filt_seq.contains(#[trigger] (p as usize)) by {
                let idx = (p - 2) as int;
                assert(0 <= idx < all.seq@.len());
                assert(all.spec_index(idx) == p as usize);
                assert(all_seq[idx] == p as usize);
                all_seq.to_multiset_ensures();
                assert(all_seq.to_multiset().count(p as usize) > 0);
                assert(spec_pred(p as usize));
                // axiom_filter_count: m.filter(f).count(v) == if f(v) { m.count(v) } else { 0 }
                assert(all_seq.to_multiset().filter(spec_pred).count(p as usize)
                    == all_seq.to_multiset().count(p as usize));
                assert(filt_seq.to_multiset().count(p as usize) > 0);
                filt_seq.to_multiset_ensures();
            }
        }
        filtered
    }

    } // verus!
}
