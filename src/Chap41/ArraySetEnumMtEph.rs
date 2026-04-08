//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Multi-threaded ephemeral enumerated set using bit array.
//!
//! Uses `Vec<u64>` for verified 1-bit-per-element storage.
//! Memory: ⌈universe_size / 64⌉ × 8 bytes. Only filter() uses parallelism.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod ArraySetEnumMtEph {


    //		Section 2. imports

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Types::Types::*;

    // Bit manipulation macros (must precede verus! for use in spec fns).

    macro_rules! get_bit64_macro {
        ($a:expr, $b:expr) => {{ (0x1u64 & ($a >> $b)) == 1 }};
    }

    #[allow(unused_macros)]
    macro_rules! get_bit64 {
        ($($a:tt)*) => { verus_proof_macro_exprs!(get_bit64_macro!($($a)*)) };
    }

    macro_rules! set_bit64_macro {
        ($a:expr, $b:expr, $c:expr) => {{
            if $c { $a | 1u64 << $b } else { $a & (!(1u64 << $b)) }
        }};
    }

    #[allow(unused_macros)]
    macro_rules! set_bit64 {
        ($($a:tt)*) => { verus_proof_macro_exprs!(set_bit64_macro!($($a)*)) };
    }

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    //		Section 4. type definitions


    pub struct ArraySetEnumMtEph {
        pub bits: Vec<u64>,
        pub universe_size: usize,
    }

    //		Section 5. view impls


    impl View for ArraySetEnumMtEph {
        type V = Set<usize>;
        open spec fn view(&self) -> Set<usize> {
            Set::new(|i: usize|
                (i as int) < self.universe_size as int
                && u64_view(self.bits@[i as int / 64])[i as int % 64]
            )
        }
    }

    //		Section 6. spec fns


    pub open spec fn u64_view(u: u64) -> Seq<bool> {
        Seq::new(64, |i: int| get_bit64!(u, i as u64))
    }

    pub open spec fn num_words(universe_size: int) -> int {
        if universe_size <= 0 { 0 } else { (universe_size - 1) / 64 + 1 }
    }

    //		Section 7. proof fns/broadcast groups


    #[verifier::bit_vector]
    proof fn zero_bit_false(i: u64)
        requires i < 64,
        ensures !get_bit64!(0u64, i),
    {}

    #[verifier::bit_vector]
    proof fn set_bit64_proof(bv_new: u64, bv_old: u64, index: u64, bit: bool)
        requires
            bv_new == set_bit64!(bv_old, index, bit),
            index < 64,
        ensures
            get_bit64!(bv_new, index) == bit,
            forall|loc2: u64| #![trigger (0x1u64 & (bv_new >> loc2))] (loc2 < 64 && loc2 != index) ==>
                (get_bit64!(bv_new, loc2) == get_bit64!(bv_old, loc2)),
    {}

    #[verifier::bit_vector]
    proof fn bit_or_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 | bv2,
        ensures forall|i: u64| #![trigger (0x1u64 & (bv_new >> i))] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) || get_bit64!(bv2, i)),
    {}

    #[verifier::bit_vector]
    proof fn bit_and_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 & bv2,
        ensures forall|i: u64| #![trigger (0x1u64 & (bv_new >> i))] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) && get_bit64!(bv2, i)),
    {}

    #[verifier::bit_vector]
    proof fn bit_andnot_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 & !bv2,
        ensures forall|i: u64| #![trigger (0x1u64 & (bv_new >> i))] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) && !get_bit64!(bv2, i)),
    {}

    /// A set of usize values bounded by n is finite.
    proof fn lemma_bounded_usize_set_finite(n: usize)
        ensures Set::new(|i: usize| (i as int) < n as int).finite()
        decreases n
    {
        if n == 0 {
            // Veracity: NEEDED assert
            assert(Set::new(|i: usize| (i as int) < 0int) =~= Set::<usize>::empty());
        } else {
            lemma_bounded_usize_set_finite((n - 1) as usize);
            let smaller = Set::new(|i: usize| (i as int) < (n - 1) as int);
            let bigger = Set::new(|i: usize| (i as int) < n as int);
            // Veracity: NEEDED assert
            assert(bigger =~= smaller.insert((n - 1) as usize));
        }
    }

    proof fn lemma_view_finite(bits: Seq<u64>, universe_size: usize)
        requires bits.len() == num_words(universe_size as int),
        ensures Set::new(|i: usize|
            (i as int) < universe_size as int
            && u64_view(bits[i as int / 64])[i as int % 64]
        ).finite(),
    {
        let our_set = Set::new(|i: usize|
            (i as int) < universe_size as int
            && u64_view(bits[i as int / 64])[i as int % 64]
        );
        let range_set = Set::new(|i: usize| (i as int) < universe_size as int);
        // Veracity: NEEDED assert
        assert(our_set.subset_of(range_set));
        lemma_bounded_usize_set_finite(universe_size);
        // range_set is finite, our_set is a subset — lemma_set_subset_finite fires.
    }

    //		Section 8. traits


    pub trait ArraySetEnumMtEphTrait: Sized + View<V = Set<usize>> {
        spec fn spec_arraysetenummteph_wf(&self) -> bool;
        spec fn spec_universe_size(&self) -> usize;

        /// - Alg Analysis: APAS (Ch41 ref): Work O(u/w), Span O(u/w) -- allocates u/64 words.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u/w), Span O(u/w) -- allocates u/64 words. — matches APAS
        fn new(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; sequential bit scan; APAS CS 41.3 Span O(1) assumes PRAM, not fork-join
        fn size(&self) -> (count: usize)
            requires self.spec_arraysetenummteph_wf(),
            ensures count == self@.len(), self@.finite();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; sequential scan, APAS assumes parallel
        fn to_seq(&self) -> (seq: ArraySeqMtEphS<usize>)
            requires self.spec_arraysetenummteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);

        /// - Alg Analysis: APAS (Ch41 ref): Work O(u/w), Span O(u/w) -- same as new.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u/w), Span O(u/w) -- same as new. — matches APAS
        fn empty(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; matches APAS Work, Span sequential (PRAM assumes O(1) parallel)
        fn singleton(u: usize, x: usize) -> (tree: Self)
            ensures
                (x < u ==> tree@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> tree@ == Set::<usize>::empty()),
                tree.spec_arraysetenummteph_wf(),
                tree.spec_universe_size() == u;

        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u + n), Span O(u + n) — ACCEPTED DIFFERENCE: PRAM gap; sequential init + insert loop
        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (constructed: Self)
            ensures
                constructed.spec_arraysetenummteph_wf(),
                constructed.spec_universe_size() == u;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u + Σ W(f(x))), Span O(u + Σ W(f(x))) — ACCEPTED DIFFERENCE: PRAM gap; sequential loop; APAS CS 41.3 Span O(1) assumes PRAM, not fork-join
        fn filter<F: Fn(usize) -> bool + Send + Sync + 'static + Clone>(&self, f: F) -> (filtered: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                forall|i: usize| i < self.spec_universe_size() ==> #[trigger] f.requires((i,)),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_arraysetenummteph_wf(),
                filtered.spec_universe_size() == self.spec_universe_size();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; sequential word-AND, APAS assumes parallel
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_arraysetenummteph_wf(),
                common.spec_universe_size() == self.spec_universe_size();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; sequential word-AND-NOT, APAS assumes parallel
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_arraysetenummteph_wf(),
                remaining.spec_universe_size() == self.spec_universe_size();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(u), Span O(u) — ACCEPTED DIFFERENCE: PRAM gap; sequential word-OR, APAS assumes parallel
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                combined@ == self@.union(other@),
                combined.spec_arraysetenummteph_wf(),
                combined.spec_universe_size() == self.spec_universe_size();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.3
        fn find(&self, x: usize) -> (found: bool)
            requires self.spec_arraysetenummteph_wf(),
            ensures found == self@.contains(x);

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — ACCEPTED DIFFERENCE: PRAM gap; ephemeral bit clear, APAS O(u) assumes copy
        fn delete(&mut self, x: usize)
            requires old(self).spec_arraysetenummteph_wf(),
            ensures
                self@ == old(self)@.remove(x),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size();

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — ACCEPTED DIFFERENCE: PRAM gap; ephemeral bit set, APAS O(u) assumes copy
        fn insert(&mut self, x: usize)
            requires old(self).spec_arraysetenummteph_wf(),
            ensures
                (x < old(self).spec_universe_size() ==> self@ == old(self)@.insert(x)),
                (x >= old(self).spec_universe_size() ==> self@ == old(self)@),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size();
    }

    //		Section 9. impls


    impl ArraySetEnumMtEph {
        pub open spec fn spec_arraysetenummteph_wf(&self) -> bool {
            self.bits@.len() == num_words(self.universe_size as int)
        }
    }


    impl ArraySetEnumMtEphTrait for ArraySetEnumMtEph {

        open spec fn spec_arraysetenummteph_wf(&self) -> bool {
            self.bits@.len() == num_words(self.universe_size as int)
        }

        open spec fn spec_universe_size(&self) -> usize {
            self.universe_size
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn new(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u,
        {
            let word_count: usize = if u == 0 { 0 } else { (u - 1) / 64 + 1 };
            let mut bits: Vec<u64> = Vec::new();
            let mut j: usize = 0;
            while j < word_count
                invariant
                    j <= word_count,
                    bits@.len() == j as int,
                    forall|k: int| 0 <= k < j ==> bits@[k] == 0u64,
                    word_count as int == num_words(u as int),
                decreases word_count - j,
            {
                bits.push(0u64);
                j = j + 1;
            }
            let result = ArraySetEnumMtEph { bits, universe_size: u };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|i: usize| !(#[trigger] result@.contains(i)) by {
                    if (i as int) < u as int {
                        let word_idx = i as int / 64;
                        let bit_idx = i as int % 64;
                        // Veracity: NEEDED assert
                        assert(0 <= word_idx < word_count as int);
                        // Veracity: NEEDED assert
                        assert(result.bits@[word_idx] == 0u64);
                        zero_bit_false(bit_idx as u64);
                    }
                }
                // Veracity: NEEDED assert
                assert(result@ =~= Set::<usize>::empty());
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite(),
        {
            // Veracity: NEEDED proof block
            proof { lemma_view_finite(self.bits@, self.universe_size); }
            let mut count: usize = 0;
            let ghost mut partial_set: Set<usize> = Set::empty();

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for i in 0..self.universe_size
                invariant
                    partial_set.finite(),
                    count as int == partial_set.len(),
                    count <= i,
                    partial_set =~= Set::new(|j: usize| (j as int) < i as int && self@.contains(j)),
                    self.spec_arraysetenummteph_wf(),
            {
                let word_idx = i / 64;
                let bit_idx = (i % 64) as u64;
                if get_bit64_macro!(self.bits[word_idx], bit_idx) {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(self@.contains(i));
                        // Veracity: NEEDED assert
                        assert(!partial_set.contains(i));
                        partial_set = partial_set.insert(i);
                    }
                    count = count + 1;
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(!self@.contains(i));
                    }
                }
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(partial_set =~= Set::new(|j: usize|
                        (j as int) < (i + 1) as int && self@.contains(j)));
                    let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                    // Veracity: NEEDED assert
                    assert(partial_set.subset_of(range_set));
                    lemma_bounded_usize_set_finite((i + 1) as usize);
                }
            }
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(partial_set =~= self@);
            }
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: ArraySeqMtEphS<usize>)
        {
            // Veracity: NEEDED proof block
            proof { lemma_view_finite(self.bits@, self.universe_size); }
            let mut result_vec: Vec<usize> = Vec::new();
            let ghost mut collected: Set<usize> = Set::empty();

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for i in 0..self.universe_size
                invariant
                    self.spec_arraysetenummteph_wf(),
                    result_vec@.len() <= i,
                    collected.finite(),
                    // Ghost set matches the partial view.
                    collected =~= Set::new(|j: usize|
                        (j as int) < i as int && self@.contains(j)),
                    // All collected elements are in self@ and below i.
                    forall|k: int| #![trigger result_vec@[k]]
                        0 <= k < result_vec@.len() ==> (
                            collected.contains(result_vec@[k])
                        ),
                    // Strictly increasing — implies no duplicates.
                    forall|k: int, l: int| 0 <= k < l < result_vec@.len() ==>
                        (#[trigger] result_vec@[k]) < (#[trigger] result_vec@[l]),
                    // Vec covers the ghost set: every collected element is in vec.
                    forall|x: usize| #[trigger] collected.contains(x) ==>
                        result_vec@.contains(x),
                    // Length matches.
                    result_vec@.len() as int == collected.len(),
            {
                let word_idx = i / 64;
                let bit_idx = (i % 64) as u64;
                if get_bit64_macro!(self.bits[word_idx], bit_idx) {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(self@.contains(i));
                        // Veracity: NEEDED assert
                        assert(!collected.contains(i));
                    }
                    let ghost old_collected = collected;
                    let old_len = result_vec.len();
                    let ghost old_view = result_vec@;
                    result_vec.push(i);
                    // Veracity: NEEDED proof block
                    proof {
                        collected = collected.insert(i);
                        // Push preserves existing elements.
                        // Veracity: NEEDED assert
                        assert forall|k: int| 0 <= k < old_len as int implies
                            #[trigger] result_vec@[k] == old_view[k] by {}
                        // Ordering: all previous elements < i.
                        // Veracity: NEEDED assert
                        assert forall|k: int, l: int|
                            0 <= k < l < result_vec@.len() implies
                            (#[trigger] result_vec@[k]) < (#[trigger] result_vec@[l]) by
                        {
                            if l == old_len as int {
                                // Veracity: NEEDED assert
                                assert(result_vec@[k] == old_view[k]);
                            }
                        }
                        // Coverage: collected elements are in vec.
                        // Veracity: NEEDED assert
                        assert forall|x: usize| #[trigger] collected.contains(x) implies
                            result_vec@.contains(x) by
                        {
                            if x == i {
                                // Veracity: NEEDED assert
                                assert(result_vec@[old_len as int] == i);
                            } else {
                                // Veracity: NEEDED assert
                                assert(old_collected.contains(x));
                                // Veracity: NEEDED assert
                                assert(old_view.contains(x));
                                let k = choose|k: int|
                                    0 <= k < old_view.len() && old_view[k] == x;
                                // Veracity: NEEDED assert
                                assert(result_vec@[k] == old_view[k]);
                                // Veracity: NEEDED assert
                                assert(result_vec@[k] == x);
                            }
                        }
                        // Ghost set update.
                        // Veracity: NEEDED assert
                        assert(collected =~= Set::new(|j: usize|
                            (j as int) < (i + 1) as int && self@.contains(j)));
                        let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                        // Veracity: NEEDED assert
                        assert(collected.subset_of(range_set));
                        lemma_bounded_usize_set_finite((i + 1) as usize);
                    }
                } else {
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(!self@.contains(i));
                        // Veracity: NEEDED assert
                        assert(collected =~= Set::new(|j: usize|
                            (j as int) < (i + 1) as int && self@.contains(j)));
                        let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                        // Veracity: NEEDED assert
                        assert(collected.subset_of(range_set));
                        lemma_bounded_usize_set_finite((i + 1) as usize);
                    }
                }
            }

            let seq = ArraySeqMtEphS::from_vec(result_vec);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(collected =~= self@);
                // seq@ is result_vec@ mapped through view (identity for usize).
                // Veracity: NEEDED assert
                assert(seq@ =~= result_vec@) by {
                    // Veracity: NEEDED assert
                    assert forall|k: int| 0 <= k < result_vec@.len() implies
                        #[trigger] seq@[k] == result_vec@[k] by
                    {
                        // Veracity: NEEDED assert
                        assert(seq.spec_index(k) == result_vec@[k]);
                    }
                }
                // seq@.to_set() =~= self@.
                // Veracity: NEEDED assert
                assert forall|x: usize|
                    seq@.to_set().contains(x) <==> #[trigger] self@.contains(x) by
                {
                    if seq@.to_set().contains(x) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == x;
                        // Veracity: NEEDED assert
                        assert(result_vec@[j] == x);
                        // Veracity: NEEDED assert
                        assert(collected.contains(x));
                    }
                    if self@.contains(x) {
                        // Veracity: NEEDED assert
                        assert(collected.contains(x));
                        // Veracity: NEEDED assert
                        assert(result_vec@.contains(x));
                        let k = choose|k: int|
                            0 <= k < result_vec@.len() && result_vec@[k] == x;
                        // Veracity: NEEDED assert
                        assert(seq@[k] == x);
                    }
                }
            }
            seq
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u,
        {
            Self::new(u)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(u: usize, x: usize) -> (tree: Self)
            ensures
                (x < u ==> tree@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> tree@ == Set::<usize>::empty()),
                tree.spec_arraysetenummteph_wf(),
                tree.spec_universe_size() == u,
        {
            let mut s = Self::new(u);
            if x < u {
                s.insert(x);
                // Veracity: NEEDED proof block
                proof {
                    lemma_view_finite(s.bits@, u);
                }
            }
            s
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n)
        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (constructed: Self)
            ensures
                constructed.spec_arraysetenummteph_wf(),
                constructed.spec_universe_size() == u,
        {
            let word_count: usize = if u == 0 { 0 } else { (u - 1) / 64 + 1 };
            let mut bits: Vec<u64> = Vec::new();
            let mut j: usize = 0;
            while j < word_count
                invariant
                    j <= word_count,
                    bits@.len() == j as int,
                    word_count as int == num_words(u as int),
                decreases word_count - j,
            {
                bits.push(0u64);
                j = j + 1;
            }

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            for i in 0..seq.length()
                invariant
                    bits@.len() == word_count as int,
                    word_count as int == num_words(u as int),
            {
                let elem = *seq.nth(i);
                if elem < u {
                    let word_idx = elem / 64;
                    let bit_idx = (elem % 64) as u64;
                    let old_word = bits[word_idx];
                    let new_word = set_bit64_macro!(old_word, bit_idx, true);
                    bits.set(word_idx, new_word);
                }
            }
            let constructed = ArraySetEnumMtEph { bits, universe_size: u };
            // Veracity: NEEDED proof block
            proof { lemma_view_finite(constructed.bits@, u); }
            constructed
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Fn(usize) -> bool + Send + Sync + 'static + Clone>(&self, f: F) -> (filtered: Self)
            ensures
                filtered@.subset_of(self@),
                filtered.spec_arraysetenummteph_wf(),
                filtered.spec_universe_size() == self.spec_universe_size(),
        {
            let word_count = self.bits.len();
            let mut new_bits: Vec<u64> = Vec::new();
            let mut j: usize = 0;
            while j < word_count
                invariant
                    j <= word_count,
                    new_bits@.len() == j as int,
                    forall|k: int| 0 <= k < j as int ==> new_bits@[k] == 0u64,
                decreases word_count - j,
            {
                new_bits.push(0u64);
                j = j + 1;
            }
            // Establish: all bits in new_bits are zero (subset invariant holds vacuously).
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|k: int, b: int| #![trigger u64_view(new_bits@[k])[b]]
                    0 <= k < word_count as int && 0 <= b < 64 && u64_view(new_bits@[k])[b]
                    implies u64_view(self.bits@[k])[b] by
                {
                    // Veracity: NEEDED assert
                    assert(new_bits@[k] == 0u64);
                    zero_bit_false(b as u64);
                }
            }
            let mut i: usize = 0;
            while i < self.universe_size
                invariant
                    i <= self.universe_size,
                    new_bits@.len() == word_count as int,
                    word_count == self.bits@.len(),
                    word_count as int == num_words(self.universe_size as int),
                    self.spec_arraysetenummteph_wf(),
                    forall|k: int, b: int| #![trigger u64_view(new_bits@[k])[b]]
                        (0 <= k < word_count as int && 0 <= b < 64 && u64_view(new_bits@[k])[b])
                        ==> u64_view(self.bits@[k])[b],
                    forall|ii: usize| ii < self.spec_universe_size()
                        ==> #[trigger] f.requires((ii,)),
                decreases self.universe_size - i,
            {
                let word_idx = i / 64;
                let bit_idx = (i % 64) as u64;
                if get_bit64_macro!(self.bits[word_idx], bit_idx) && f(i) {
                    let ghost old_new_bits = new_bits@;
                    let old_word = new_bits[word_idx];
                    let new_word = set_bit64_macro!(old_word, bit_idx, true);
                    // Veracity: NEEDED proof block
                    proof {
                        set_bit64_proof(new_word, old_word, bit_idx, true);
                    }
                    new_bits.set(word_idx, new_word);
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|k: int, b: int| #![trigger u64_view(new_bits@[k])[b]]
                            0 <= k < word_count as int && 0 <= b < 64
                                && u64_view(new_bits@[k])[b]
                            implies u64_view(self.bits@[k])[b] by
                        {
                            if k == word_idx as int {
                                if b == bit_idx as int {
                                    // We set this bit; self.bits has it (checked by if condition).
                                } else {
                                    // Other bits in same word: preserved by set_bit64_proof.
                                    // Veracity: NEEDED assert
                                    assert(u64_view(new_word)[b] == u64_view(old_word)[b]);
                                    // Veracity: NEEDED assert
                                    assert(old_new_bits[k] == old_word);
                                }
                            } else {
                                // Different word: unchanged by Vec::set.
                                // Veracity: NEEDED assert
                                assert(new_bits@[k] == old_new_bits[k]);
                            }
                        }
                    }
                }
                i = i + 1;
            }
            let filtered = ArraySetEnumMtEph { bits: new_bits, universe_size: self.universe_size };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|elem: usize|
                    #[trigger] filtered@.contains(elem)
                    implies self@.contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let b = elem as int % 64;
                        // Veracity: NEEDED assert
                        assert(0 <= k < word_count as int);
                        // Veracity: NEEDED assert
                        assert(0 <= b < 64);
                    }
                }
                // Veracity: NEEDED assert
                assert(filtered@.subset_of(self@));
                lemma_view_finite(filtered.bits@, self.universe_size);
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures
                common@ == self@.intersect(other@),
                common.spec_arraysetenummteph_wf(),
                common.spec_universe_size() == self.spec_universe_size(),
        {
            let n = self.bits.len();
            let mut result_bits: Vec<u64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == self.bits@.len(),
                    n == other.bits@.len(),
                    result_bits@.len() == i as int,
                    forall|k: int, j: int| #![trigger u64_view(result_bits@[k])[j]]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] && u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let and_word: u64 = w1 & w2;
                // Veracity: NEEDED proof block
                proof {
                    bit_and_64_proof(w1, w2, and_word);
                }
                result_bits.push(and_word);
                i = i + 1;
            }
            let common = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|elem: usize|
                    #[trigger] common@.contains(elem) == self@.intersect(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        // Veracity: NEEDED assert
                        assert(0 <= k < n as int);
                        // Veracity: NEEDED assert
                        assert(0 <= j < 64);
                        // Veracity: NEEDED assert
                        assert(u64_view(common.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] && u64_view(other.bits@[k])[j]));
                    }
                }
                // Veracity: NEEDED assert
                assert(common@ =~= self@.intersect(other@));
                lemma_view_finite(common.bits@, self.universe_size);
            }
            common
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_arraysetenummteph_wf(),
                remaining.spec_universe_size() == self.spec_universe_size(),
        {
            let n = self.bits.len();
            let mut result_bits: Vec<u64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == self.bits@.len(),
                    n == other.bits@.len(),
                    result_bits@.len() == i as int,
                    forall|k: int, j: int| #![trigger u64_view(result_bits@[k])[j]]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] && !u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let andnot_word: u64 = w1 & !w2;
                // Veracity: NEEDED proof block
                proof {
                    bit_andnot_64_proof(w1, w2, andnot_word);
                }
                result_bits.push(andnot_word);
                i = i + 1;
            }
            let remaining = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|elem: usize|
                    #[trigger] remaining@.contains(elem) == self@.difference(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        // Veracity: NEEDED assert
                        assert(0 <= k < n as int);
                        // Veracity: NEEDED assert
                        assert(0 <= j < 64);
                        // Veracity: NEEDED assert
                        assert(u64_view(remaining.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] && !u64_view(other.bits@[k])[j]));
                    }
                }
                // Veracity: NEEDED assert
                assert(remaining@ =~= self@.difference(other@));
                lemma_view_finite(remaining.bits@, self.universe_size);
            }
            remaining
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union(&self, other: &Self) -> (combined: Self)
            ensures
                combined@ == self@.union(other@),
                combined.spec_arraysetenummteph_wf(),
                combined.spec_universe_size() == self.spec_universe_size(),
        {
            let n = self.bits.len();
            let mut result_bits: Vec<u64> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == self.bits@.len(),
                    n == other.bits@.len(),
                    result_bits@.len() == i as int,
                    forall|k: int, j: int| #![trigger u64_view(result_bits@[k])[j]]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] || u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let or_word: u64 = w1 | w2;
                // Veracity: NEEDED proof block
                proof {
                    bit_or_64_proof(w1, w2, or_word);
                }
                result_bits.push(or_word);
                i = i + 1;
            }
            let combined = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|elem: usize|
                    #[trigger] combined@.contains(elem) == self@.union(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        // Veracity: NEEDED assert
                        assert(0 <= k < n as int);
                        // Veracity: NEEDED assert
                        assert(0 <= j < 64);
                        // Veracity: NEEDED assert
                        assert(u64_view(combined.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] || u64_view(other.bits@[k])[j]));
                    }
                }
                // Veracity: NEEDED assert
                assert(combined@ =~= self@.union(other@));
                lemma_view_finite(combined.bits@, self.universe_size);
            }
            combined
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn find(&self, x: usize) -> (found: bool)
            ensures found == self@.contains(x),
        {
            if x >= self.universe_size {
                false
            } else {
                let word_idx: usize = x / 64;
                let bit_idx: u64 = (x % 64) as u64;
                let word: u64 = self.bits[word_idx];
                get_bit64_macro!(word, bit_idx)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn delete(&mut self, x: usize)
            ensures
                self@ == old(self)@.remove(x),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size(),
        {
            if x < self.universe_size {
                let word_idx: usize = x / 64;
                let bit_idx: u64 = (x % 64) as u64;
                let old_word: u64 = self.bits[word_idx];
                let new_word: u64 = set_bit64_macro!(old_word, bit_idx, false);
                // Veracity: NEEDED proof block
                proof {
                    set_bit64_proof(new_word, old_word, bit_idx, false);
                }
                self.bits.set(word_idx, new_word);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|elem: usize|
                        #[trigger] self@.contains(elem) == old(self)@.remove(x).contains(elem) by
                    {
                        if (elem as int) < self.universe_size as int {
                            let ek = elem as int / 64;
                            let ej = elem as int % 64;
                            if ek == word_idx as int {
                                // Same word: use set_bit64_proof
                                if elem == x {
                                    // Deleted bit
                                } else {
                                    // Veracity: NEEDED assert
                                    assert(ej != bit_idx as int);
                                    // Different bit in same word
                                }
                            } else {
                                // Different word: bits unchanged
                            }
                        }
                    }
                    // Veracity: NEEDED assert
                    assert(self@ =~= old(self)@.remove(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            } else {
                // x not in set (x >= universe_size), so remove(x) is identity.
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(self@ =~= old(self)@.remove(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn insert(&mut self, x: usize)
            ensures
                (x < old(self).spec_universe_size() ==> self@ == old(self)@.insert(x)),
                (x >= old(self).spec_universe_size() ==> self@ == old(self)@),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size(),
        {
            if x < self.universe_size {
                let word_idx: usize = x / 64;
                let bit_idx: u64 = (x % 64) as u64;
                let old_word: u64 = self.bits[word_idx];
                let new_word: u64 = set_bit64_macro!(old_word, bit_idx, true);
                // Veracity: NEEDED proof block
                proof {
                    set_bit64_proof(new_word, old_word, bit_idx, true);
                }
                self.bits.set(word_idx, new_word);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|elem: usize|
                        #[trigger] self@.contains(elem) == old(self)@.insert(x).contains(elem) by
                    {
                        if elem == x {
                            // Inserted bit: set_bit64_proof ensures it's true
                        } else if (elem as int) < self.universe_size as int {
                            let ek = elem as int / 64;
                            let ej = elem as int % 64;
                            if ek == word_idx as int {
                                // Veracity: NEEDED assert
                                assert(ej != bit_idx as int);
                            }
                        }
                    }
                    // Veracity: NEEDED assert
                    assert(self@ =~= old(self)@.insert(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            } else {
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(self@ =~= old(self)@);
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            }
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for ArraySetEnumMtEph {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl Eq for ArraySetEnumMtEph {}

    impl PartialEq for ArraySetEnumMtEph {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.universe_size == other.universe_size && self.bits == other.bits;
            // Veracity: NEEDED proof block
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl Clone for ArraySetEnumMtEph {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = ArraySetEnumMtEph {
                bits: self.bits.clone(),
                universe_size: self.universe_size,
            };
            // Veracity: NEEDED proof block
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! ArraySetEnumMtEphLit {
        ( $u:expr ; ) => {{
            < $crate::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::ArraySetEnumMtEph as $crate::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::ArraySetEnumMtEphTrait >::empty($u)
        }};
        ( $u:expr ; $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::ArraySetEnumMtEph as $crate::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::ArraySetEnumMtEphTrait >::empty($u);
            $( __set.insert($x); )*
            __set
        }};
    }

    //		Section 14. derive impls outside verus!

    impl fmt::Debug for ArraySetEnumMtEph {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let count = self.size();
            write!(f, "ArraySetEnumMtEph({}/{})", count, self.universe_size)
        }
    }

    impl fmt::Display for ArraySetEnumMtEph {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let mut first = true;
            for i in 0..self.universe_size {
                let word_idx = i / 64;
                let bit_idx = (i % 64) as u64;
                if word_idx < self.bits.len() && get_bit64_macro!(self.bits[word_idx], bit_idx) {
                    if !first { write!(f, ", ")?; }
                    write!(f, "{}", i)?;
                    first = false;
                }
            }
            write!(f, "}}")
        }
    }
}
