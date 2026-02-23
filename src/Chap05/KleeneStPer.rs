// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Persistent Kleene Star and Plus over a finite alphabet (Definition 5.4, Exercise 5.1).

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls

//		1. module


pub mod KleeneStPer {

    use vstd::prelude::*;

verus! {

    //		2. imports

    use std::hash::Hash;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;


    //		3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::seq::group_seq_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::set_lib::group_set_lib_default,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct KleeneStPer<T: StT + Hash> {
        pub alphabet: SetStEph<T>,
    }


    //		5. view impls

    impl<T: StT + Hash> View for KleeneStPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.alphabet@ }
    }


    //		6. spec fns

    /// Membership in Σ*: every element of s belongs to the alphabet.
    /// The empty sequence is always in Σ*.
    pub open spec fn in_star<V>(alphabet: Set<V>, s: Seq<V>) -> bool {
        forall|i: int| 0 <= i < s.len() ==> alphabet.contains(#[trigger] s[i])
    }

    /// Membership in Σ+: non-empty and every element in the alphabet.
    pub open spec fn in_plus<V>(alphabet: Set<V>, s: Seq<V>) -> bool {
        s.len() > 0 && in_star(alphabet, s)
    }

    /// Maps exec-level sequence to view-level sequence.
    pub open spec fn viewed<T: View>(s: Seq<T>) -> Seq<T::V> {
        s.map(|_i: int, t: T| t@)
    }


    //		7. proof fns/broadcast groups

    /// Exercise 5.1: Σ* is closed under string concatenation.
    pub proof fn lemma_star_closed_under_concat<V>(alphabet: Set<V>, s1: Seq<V>, s2: Seq<V>)
        requires
            in_star(alphabet, s1),
            in_star(alphabet, s2),
        ensures
            in_star(alphabet, s1.add(s2)),
    {
        assert forall|i: int| 0 <= i < s1.add(s2).len()
            implies alphabet.contains(#[trigger] s1.add(s2)[i]) by {
            if i < s1.len() {
                assert(s1.add(s2)[i] == s1[i]);
            } else {
                assert(s1.add(s2)[i] == s2[i - s1.len()]);
            }
        };
    }

    /// Exercise 5.1: Σ+ is closed under string concatenation.
    pub proof fn lemma_plus_closed_under_concat<V>(alphabet: Set<V>, s1: Seq<V>, s2: Seq<V>)
        requires
            in_plus(alphabet, s1),
            in_plus(alphabet, s2),
        ensures
            in_plus(alphabet, s1.add(s2)),
    {
        lemma_star_closed_under_concat(alphabet, s1, s2);
        assert(s1.add(s2).len() > 0);
    }

    /// PTT: Σ* always contains the empty sequence.
    proof fn ptt_star_contains_empty<V>(alphabet: Set<V>)
        ensures in_star(alphabet, Seq::<V>::empty()),
    {
    }

    /// PTT: Σ+ never contains the empty sequence.
    proof fn ptt_plus_rejects_empty<V>(alphabet: Set<V>)
        ensures !in_plus(alphabet, Seq::<V>::empty()),
    {
    }

    /// PTT: A single alphabet element is in both Σ* and Σ+.
    proof fn ptt_singleton_in_star_and_plus<V>(alphabet: Set<V>, x: V)
        requires alphabet.contains(x)
        ensures
            in_star(alphabet, seq![x]),
            in_plus(alphabet, seq![x]),
    {
    }

    /// PTT: Σ+ is a subset of Σ*.
    proof fn ptt_plus_subset_of_star<V>(alphabet: Set<V>, s: Seq<V>)
        requires in_plus(alphabet, s)
        ensures in_star(alphabet, s)
    {
    }

    /// PTT: If every alphabet symbol satisfies P, every element of any Σ* string satisfies P.
    /// This is the key property transfer pattern for proofs over Kleene strings.
    proof fn ptt_star_property_transfer<V>(
        alphabet: Set<V>,
        s: Seq<V>,
        p: spec_fn(V) -> bool,
    )
        requires
            in_star(alphabet, s),
            forall|x: V| alphabet.contains(x) ==> p(x),
        ensures
            forall|i: int| 0 <= i < s.len() ==> p(#[trigger] s[i]),
    {
    }

    /// PTT: Concatenation of a Σ* string with a Σ+ string yields a Σ+ string.
    /// Exercises the asymmetric closure case.
    proof fn ptt_star_concat_plus_is_plus<V>(alphabet: Set<V>, s1: Seq<V>, s2: Seq<V>)
        requires
            in_star(alphabet, s1),
            in_plus(alphabet, s2),
        ensures
            in_plus(alphabet, s1.add(s2)),
    {
        lemma_star_closed_under_concat(alphabet, s1, s2);
        assert(s1.add(s2).len() > 0) by {
            assert(s2.len() > 0);
        };
    }

    /// PTT: Σ+ string concatenated with Σ* string yields Σ+ string.
    proof fn ptt_plus_concat_star_is_plus<V>(alphabet: Set<V>, s1: Seq<V>, s2: Seq<V>)
        requires
            in_plus(alphabet, s1),
            in_star(alphabet, s2),
        ensures
            in_plus(alphabet, s1.add(s2)),
    {
        lemma_star_closed_under_concat(alphabet, s1, s2);
        assert(s1.add(s2).len() > 0) by {
            assert(s1.len() > 0);
        };
    }


    //		8. traits

    pub trait KleeneStPerTrait<T: StT + Hash> : View<V = Set<<T as View>::V>> + Sized {

        /// Construct from an alphabet Σ.
        /// - APAS: (no cost stated — Chapter 5 is purely definitional)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — stores alphabet, O(1).
        fn new(alphabet: SetStEph<T>) -> (kleene: Self)
            requires valid_key_type::<T>()
            ensures kleene@ == alphabet@;

        /// Membership in Σ*: is every element of s in the alphabet?
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(|s|), Span Θ(|s|) — linear scan, sequential.
        fn mem_star(&self, s: &[T]) -> (member: bool)
            requires valid_key_type::<T>()
            ensures member == in_star(self@, viewed(s@));

        /// Membership in Σ+: non-empty and every element in the alphabet?
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(|s|), Span Θ(|s|) — length check + linear scan.
        fn mem_plus(&self, s: &[T]) -> (member: bool)
            requires valid_key_type::<T>()
            ensures member == in_plus(self@, viewed(s@));

        /// Read-only access to the underlying alphabet.
        /// - APAS: (no cost stated — Chapter 5 is purely definitional)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference return, O(1).
        fn alphabet(&self) -> (alpha: &SetStEph<T>)
            ensures alpha@ == self@;
    }


    //		9. impls

    impl<T: StT + Hash> KleeneStPerTrait<T> for KleeneStPer<T> {

        fn new(alphabet: SetStEph<T>) -> (kleene: Self) {
            KleeneStPer { alphabet }
        }

        fn mem_star(&self, s: &[T]) -> (member: bool) {
            let mut i: usize = 0;
            while i < s.len()
                invariant
                    valid_key_type::<T>(),
                    0 <= i <= s.len(),
                    forall|j: int| 0 <= j < i as int
                        ==> self.alphabet@.contains(#[trigger] s@[j]@),
                decreases s.len() - i,
            {
                if !self.alphabet.mem(&s[i]) {
                    assert(!self.alphabet@.contains(s@[i as int]@));
                    assert(viewed::<T>(s@)[i as int] == s@[i as int]@);
                    return false;
                }
                i += 1;
            }
            proof {
                assert forall|j: int| 0 <= j < viewed::<T>(s@).len()
                    implies self@.contains(#[trigger] viewed::<T>(s@)[j]) by {
                    assert(viewed::<T>(s@)[j] == s@[j]@);
                };
            }
            true
        }

        fn mem_plus(&self, s: &[T]) -> (member: bool) {
            if s.len() == 0 {
                proof { assert(viewed::<T>(s@).len() == 0); }
                false
            } else {
                let r = self.mem_star(s);
                proof { assert(viewed::<T>(s@).len() > 0); }
                r
            }
        }

        fn alphabet(&self) -> (alpha: &SetStEph<T>) {
            &self.alphabet
        }
    }

} // verus!

} // pub mod KleeneStPer
