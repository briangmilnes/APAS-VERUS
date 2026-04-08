//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared contraction lemmas for Reduce and Scan algorithms (Chapter 27).
//! All four contract/scan variants import these instead of duplicating them.

//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  7. proof fns/broadcast groups

pub mod ContractSpecsAndLemmas {

    use vstd::prelude::*;

    verus! {

    use crate::vstdplus::monoid::monoid::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    //		Section 7. proof fns/broadcast groups


    /// Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f))
    /// when (f, id) is a monoid.
    pub proof fn lemma_fold_left_monoid<T>(s: Seq<T>, x: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures s.fold_left(x, f) == f(x, s.fold_left(id, f)),
        decreases s.len(),
    {
        reveal(Seq::fold_left);
        if s.len() == 0 {
        } else {
            lemma_fold_left_monoid::<T>(s.drop_last(), x, f, id);
            lemma_fold_left_monoid::<T>(s.drop_last(), id, f, id);
        }
    }

    /// Helper: fold_left of a 2-element sequence equals f(a, b) under a monoid.
    pub proof fn lemma_fold_left_pair<T>(a: T, b: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures seq![a, b].fold_left(id, f) == f(a, b),
    {
        let s = seq![a, b];
        reveal_with_fuel(Seq::fold_left, 3);
    }

    /// Helper: fold_left of a 1-element sequence equals a under a monoid.
    pub proof fn lemma_fold_left_singleton<T>(a: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures seq![a].fold_left(id, f) == a,
    {
        reveal_with_fuel(Seq::fold_left, 2);
    }

    /// Contraction lemma: for an even-length sequence, folding the original equals
    /// folding the contracted (pairwise-combined) sequence under a monoid.
    pub proof fn lemma_contraction_even<T>(s: Seq<T>, f: spec_fn(T, T) -> T, id: T)
        requires
            spec_monoid(f, id),
            s.len() >= 2,
            s.len() % 2 == 0,
        ensures
            s.fold_left(id, f) == Seq::new(
                (s.len() / 2) as nat,
                |i: int| f(s[2 * i], s[2 * i + 1]),
            ).fold_left(id, f),
        decreases s.len(),
    {
        let n = s.len();
        let half = (n / 2) as nat;
        let b = Seq::new(half, |i: int| f(s[2 * i], s[2 * i + 1]));

        if n == 2 {
            // Veracity: NEEDED assert
            assert(s =~= seq![s[0], s[1]]);
            lemma_fold_left_pair::<T>(s[0], s[1], f, id);
            // Veracity: NEEDED assert
            assert(b =~= seq![f(s[0], s[1])]);
            lemma_fold_left_singleton::<T>(f(s[0], s[1]), f, id);
        } else {
            let s_tail = s.subrange(2, n as int);
            let b_tail = b.subrange(1, b.len() as int);

            s.lemma_fold_left_split(id, f, 2);
            let s_head = s.subrange(0, 2);
            // Veracity: NEEDED assert
            assert(s_head =~= seq![s[0], s[1]]);

            lemma_fold_left_pair::<T>(s[0], s[1], f, id);

            lemma_fold_left_monoid::<T>(s_tail, b[0], f, id);
            let s_tail_result = s_tail.fold_left(id, f);

            // Veracity: NEEDED assert
            assert(b_tail =~= Seq::new(
                (s_tail.len() / 2) as nat,
                |i: int| f(s_tail[2 * i], s_tail[2 * i + 1]),
            ));

            lemma_contraction_even::<T>(s_tail, f, id);
            let b_tail_result = b_tail.fold_left(id, f);

            lemma_fold_left_monoid::<T>(b_tail, b[0], f, id);

            b.lemma_fold_left_split(id, f, 1);
            // Veracity: NEEDED assert
            assert(b.subrange(0, 1) =~= seq![b[0]]);
            lemma_fold_left_singleton::<T>(b[0], f, id);
        }
    }

    /// Prefix contraction lemma: fold_left of an even-length prefix s.take(2k)
    /// equals fold_left of the first k elements of the contracted sequence b.
    pub proof fn lemma_prefix_contraction<T>(s: Seq<T>, b: Seq<T>, f: spec_fn(T, T) -> T, id: T, k: int)
        requires
            spec_monoid(f, id),
            k >= 1,
            2 * k <= s.len(),
            b.len() >= k,
            forall|i: int| #![trigger b[i]] 0 <= i < b.len() ==> b[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            s.take(2 * k).fold_left(id, f) == b.take(k).fold_left(id, f),
    {
        let prefix = s.take(2 * k);
        lemma_contraction_even::<T>(prefix, f, id);
        let contracted = Seq::new(
            (prefix.len() / 2) as nat,
            |i: int| f(prefix[2 * i], prefix[2 * i + 1]),
        );
        // Veracity: NEEDED assert
        assert(contracted =~= b.take(k));
    }

    /// Expand even step: b_seq.take(j).fold_left(id, f) == s.take(2j).fold_left(id, f).
    pub proof fn lemma_expand_even<T>(s: Seq<T>, b_seq: Seq<T>, f: spec_fn(T, T) -> T, id: T, j: int)
        requires
            spec_monoid(f, id),
            j >= 0,
            2 * j <= s.len(),
            b_seq.len() >= j,
            forall|i: int| #![trigger b_seq[i]] 0 <= i < b_seq.len() ==> b_seq[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            b_seq.take(j).fold_left(id, f) == s.take(2 * j).fold_left(id, f),
    {
        if j > 0 {
            lemma_prefix_contraction::<T>(s, b_seq, f, id, j);
        } else {
            reveal(Seq::fold_left);
        }
    }

    /// Expand odd step: f(s.take(2j).fold_left(id, f), s[2j]) == s.take(2j+1).fold_left(id, f).
    pub proof fn lemma_expand_odd<T>(s: Seq<T>, f: spec_fn(T, T) -> T, id: T, j: int)
        requires
            spec_monoid(f, id),
            j >= 0,
            2 * j + 1 <= s.len(),
        ensures
            f(s.take(2 * j).fold_left(id, f), s[2 * j]) == s.take(2 * j + 1).fold_left(id, f),
    {
        let take_2j1 = s.take(2 * j + 1);
        take_2j1.lemma_fold_left_split(id, f, 2 * j);
        // Veracity: NEEDED assert
        assert(take_2j1.subrange(0, 2 * j) =~= s.take(2 * j));
        reveal(Seq::fold_left);
    }

    /// Expand odd-length tail: last element step when n is odd.
    pub proof fn lemma_expand_odd_tail<T>(
        s: Seq<T>, b_seq: Seq<T>, f: spec_fn(T, T) -> T, id: T, half: int,
    )
        requires
            spec_monoid(f, id),
            half >= 1,
            s.len() == 2 * half + 1,
            b_seq.len() == half,
            forall|i: int| #![trigger b_seq[i]] 0 <= i < b_seq.len() ==> b_seq[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            f(b_seq.take(half - 1).fold_left(id, f), b_seq[half - 1])
                == s.take(2 * half).fold_left(id, f),
    {
        b_seq.lemma_fold_left_split(id, f, half - 1);
        reveal(Seq::fold_left);
        lemma_prefix_contraction::<T>(s, b_seq, f, id, half);
    }

    } // verus!
} // mod
