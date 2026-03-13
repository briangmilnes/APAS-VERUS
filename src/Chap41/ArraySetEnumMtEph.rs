//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral enumerated set using bit array.
//!
//! Uses `Vec<u64>` for verified 1-bit-per-element storage.
//! Memory: ⌈universe_size / 64⌉ × 8 bytes. Only filter() uses parallelism.

pub mod ArraySetEnumMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

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

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    pub struct ArraySetEnumMtEph {
        pub bits: Vec<u64>,
        pub universe_size: usize,
    }

    // 5. view impls

    impl View for ArraySetEnumMtEph {
        type V = Set<usize>;
        open spec fn view(&self) -> Set<usize> {
            Set::new(|i: usize|
                (i as int) < self.universe_size as int
                && u64_view(self.bits@[i as int / 64])[i as int % 64]
            )
        }
    }

    // 6. spec fns

    pub open spec fn u64_view(u: u64) -> Seq<bool> {
        Seq::new(64, |i: int| get_bit64!(u, i as u64))
    }

    pub open spec fn num_words(universe_size: int) -> int {
        if universe_size <= 0 { 0 } else { (universe_size - 1) / 64 + 1 }
    }

    impl ArraySetEnumMtEph {
        pub open spec fn spec_arraysetenummteph_wf(&self) -> bool {
            self.bits@.len() == num_words(self.universe_size as int)
        }
    }

    // 7. proof fns

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
            forall|loc2: u64| #![auto] (loc2 < 64 && loc2 != index) ==>
                (get_bit64!(bv_new, loc2) == get_bit64!(bv_old, loc2)),
    {}

    #[verifier::bit_vector]
    proof fn bit_or_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 | bv2,
        ensures forall|i: u64| #![auto] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) || get_bit64!(bv2, i)),
    {}

    #[verifier::bit_vector]
    proof fn bit_and_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 & bv2,
        ensures forall|i: u64| #![auto] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) && get_bit64!(bv2, i)),
    {}

    #[verifier::bit_vector]
    proof fn bit_andnot_64_proof(bv1: u64, bv2: u64, bv_new: u64)
        requires bv_new == bv1 & !bv2,
        ensures forall|i: u64| #![auto] (i < 64) ==>
            get_bit64!(bv_new, i) == (get_bit64!(bv1, i) && !get_bit64!(bv2, i)),
    {}

    /// A set of usize values bounded by n is finite.
    proof fn lemma_bounded_usize_set_finite(n: usize)
        ensures Set::new(|i: usize| (i as int) < n as int).finite()
        decreases n
    {
        if n == 0 {
            assert(Set::new(|i: usize| (i as int) < 0int) =~= Set::<usize>::empty());
        } else {
            lemma_bounded_usize_set_finite((n - 1) as usize);
            let smaller = Set::new(|i: usize| (i as int) < (n - 1) as int);
            let bigger = Set::new(|i: usize| (i as int) < n as int);
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
        assert(our_set.subset_of(range_set));
        lemma_bounded_usize_set_finite(universe_size);
        // range_set is finite, our_set is a subset — lemma_set_subset_finite fires.
    }

    // 8. traits

    pub trait ArraySetEnumMtEphTrait: Sized + View<V = Set<usize>> {
        spec fn spec_arraysetenummteph_wf(&self) -> bool;
        spec fn spec_universe_size(&self) -> usize;

        /// Work Θ(u/w), Span Θ(u/w) where w is word size.
        fn new(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u;

        /// - APAS Cost Spec 41.3: Work u, Span 1
        fn size(&self) -> (count: usize)
            requires self.spec_arraysetenummteph_wf(),
            ensures count == self@.len(), self@.finite();

        /// - APAS Cost Spec 41.3: Work u, Span 1
        fn to_seq(&self) -> (seq: ArraySeqMtEphS<usize>)
            requires self.spec_arraysetenummteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);

        /// Work Θ(u/w), Span Θ(u/w).
        fn empty(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u;

        /// - APAS Cost Spec 41.3: Work u, Span 1
        fn singleton(u: usize, x: usize) -> (tree: Self)
            ensures
                (x < u ==> tree@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> tree@ == Set::<usize>::empty()),
                tree@.finite(),
                tree.spec_arraysetenummteph_wf(),
                tree.spec_universe_size() == u;

        /// Work Θ(u + |seq|), Span Θ(1).
        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (constructed: Self)
            ensures
                constructed@.finite(),
                constructed.spec_arraysetenummteph_wf(),
                constructed.spec_universe_size() == u;

        /// - APAS Cost Spec 41.3: Work u + Σ W(f(x)), Span 1 + max S(f(x))
        fn filter<F: PredVal<usize> + Clone>(&self, f: F) -> (filtered: Self)
            requires self.spec_arraysetenummteph_wf(),
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                filtered.spec_arraysetenummteph_wf(),
                filtered.spec_universe_size() == self.spec_universe_size();

        /// - APAS Cost Spec 41.3: Work u/w, Span u/w
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                common@ == self@.intersect(other@),
                common@.finite(),
                common.spec_arraysetenummteph_wf(),
                common.spec_universe_size() == self.spec_universe_size();

        /// - APAS Cost Spec 41.3: Work u/w, Span u/w
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                remaining@ == self@.difference(other@),
                remaining@.finite(),
                remaining.spec_arraysetenummteph_wf(),
                remaining.spec_universe_size() == self.spec_universe_size();

        /// - APAS Cost Spec 41.3: Work u/w, Span u/w
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_arraysetenummteph_wf(),
                other.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == other.spec_universe_size(),
            ensures
                combined@ == self@.union(other@),
                combined@.finite(),
                combined.spec_arraysetenummteph_wf(),
                combined.spec_universe_size() == self.spec_universe_size();

        /// - APAS Cost Spec 41.3: Work 1, Span 1
        fn find(&self, x: usize) -> (found: B)
            requires self.spec_arraysetenummteph_wf(),
            ensures found == self@.contains(x);

        /// - APAS Cost Spec 41.3: Work 1, Span 1
        fn delete(&mut self, x: usize)
            requires old(self).spec_arraysetenummteph_wf(),
            ensures
                self@ == old(self)@.remove(x),
                self@.finite(),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size();

        /// - APAS Cost Spec 41.3: Work 1, Span 1
        fn insert(&mut self, x: usize)
            requires old(self).spec_arraysetenummteph_wf(),
            ensures
                (x < old(self).spec_universe_size() ==> self@ == old(self)@.insert(x)),
                (x >= old(self).spec_universe_size() ==> self@ == old(self)@),
                self@.finite(),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size();
    }

    // 9. impls

    impl ArraySetEnumMtEphTrait for ArraySetEnumMtEph {

        open spec fn spec_arraysetenummteph_wf(&self) -> bool {
            self.bits@.len() == num_words(self.universe_size as int)
        }

        open spec fn spec_universe_size(&self) -> usize {
            self.universe_size
        }

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
            proof {
                assert forall|i: usize| !(#[trigger] result@.contains(i)) by {
                    if (i as int) < u as int {
                        let word_idx = i as int / 64;
                        let bit_idx = i as int % 64;
                        assert(0 <= word_idx < word_count as int);
                        assert(result.bits@[word_idx] == 0u64);
                        zero_bit_false(bit_idx as u64);
                    }
                }
                assert(result@ =~= Set::<usize>::empty());
            }
            result
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite(),
        {
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
                    proof {
                        assert(self@.contains(i));
                        assert(!partial_set.contains(i));
                        partial_set = partial_set.insert(i);
                    }
                    count = count + 1;
                } else {
                    proof {
                        assert(!self@.contains(i));
                    }
                }
                proof {
                    assert(partial_set =~= Set::new(|j: usize|
                        (j as int) < (i + 1) as int && self@.contains(j)));
                    let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                    assert(partial_set.subset_of(range_set));
                    lemma_bounded_usize_set_finite((i + 1) as usize);
                }
            }
            proof {
                assert(partial_set =~= self@);
            }
            count
        }

        fn to_seq(&self) -> (seq: ArraySeqMtEphS<usize>)
        {
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
                    proof {
                        assert(self@.contains(i));
                        assert(!collected.contains(i));
                    }
                    let ghost old_collected = collected;
                    let old_len = result_vec.len();
                    let ghost old_view = result_vec@;
                    result_vec.push(i);
                    proof {
                        collected = collected.insert(i);
                        // Push preserves existing elements.
                        assert forall|k: int| 0 <= k < old_len as int implies
                            #[trigger] result_vec@[k] == old_view[k] by {}
                        // Ordering: all previous elements < i.
                        assert forall|k: int, l: int|
                            0 <= k < l < result_vec@.len() implies
                            (#[trigger] result_vec@[k]) < (#[trigger] result_vec@[l]) by
                        {
                            if l == old_len as int {
                                assert(result_vec@[k] == old_view[k]);
                            }
                        }
                        // Coverage: collected elements are in vec.
                        assert forall|x: usize| #[trigger] collected.contains(x) implies
                            result_vec@.contains(x) by
                        {
                            if x == i {
                                assert(result_vec@[old_len as int] == i);
                            } else {
                                assert(old_collected.contains(x));
                                assert(old_view.contains(x));
                                let k = choose|k: int|
                                    0 <= k < old_view.len() && old_view[k] == x;
                                assert(result_vec@[k] == old_view[k]);
                                assert(result_vec@[k] == x);
                            }
                        }
                        // Ghost set update.
                        assert(collected =~= Set::new(|j: usize|
                            (j as int) < (i + 1) as int && self@.contains(j)));
                        let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                        assert(collected.subset_of(range_set));
                        lemma_bounded_usize_set_finite((i + 1) as usize);
                    }
                } else {
                    proof {
                        assert(!self@.contains(i));
                        assert(collected =~= Set::new(|j: usize|
                            (j as int) < (i + 1) as int && self@.contains(j)));
                        let range_set = Set::new(|j: usize| (j as int) < (i + 1) as int);
                        assert(collected.subset_of(range_set));
                        lemma_bounded_usize_set_finite((i + 1) as usize);
                    }
                }
            }

            let seq = ArraySeqMtEphS::from_vec(result_vec);
            proof {
                assert(collected =~= self@);
                // seq@ is result_vec@ mapped through view (identity for usize).
                assert(seq@ =~= result_vec@) by {
                    assert forall|k: int| 0 <= k < result_vec@.len() implies
                        #[trigger] seq@[k] == result_vec@[k] by
                    {
                        assert(seq.spec_index(k) == result_vec@[k]);
                    }
                }
                // seq@.to_set() =~= self@.
                assert forall|x: usize|
                    seq@.to_set().contains(x) <==> #[trigger] self@.contains(x) by
                {
                    if seq@.to_set().contains(x) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == x;
                        assert(result_vec@[j] == x);
                        assert(collected.contains(x));
                    }
                    if self@.contains(x) {
                        assert(collected.contains(x));
                        assert(result_vec@.contains(x));
                        let k = choose|k: int|
                            0 <= k < result_vec@.len() && result_vec@[k] == x;
                        assert(seq@[k] == x);
                    }
                }
            }
            seq
        }

        fn empty(u: usize) -> (empty: Self)
            ensures
                empty@ == Set::<usize>::empty(),
                empty.spec_arraysetenummteph_wf(),
                empty.spec_universe_size() == u,
        {
            Self::new(u)
        }

        fn singleton(u: usize, x: usize) -> (tree: Self)
            ensures
                (x < u ==> tree@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> tree@ == Set::<usize>::empty()),
                tree@.finite(),
                tree.spec_arraysetenummteph_wf(),
                tree.spec_universe_size() == u,
        {
            let mut s = Self::new(u);
            if x < u {
                s.insert(x);
                proof {
                    lemma_view_finite(s.bits@, u);
                }
            }
            s
        }

        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (constructed: Self)
            ensures
                constructed@.finite(),
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
            proof { lemma_view_finite(constructed.bits@, u); }
            constructed
        }

        #[verifier::external_body]
        fn filter<F: PredVal<usize> + Clone>(&self, f: F) -> (filtered: Self)
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                filtered.spec_arraysetenummteph_wf(),
                filtered.spec_universe_size() == self.spec_universe_size(),
        {
            let word_count = self.bits.len();
            let mut new_bits = vec![0u64; word_count];
            for i in 0..self.universe_size {
                let word_idx = i / 64;
                let bit_idx = (i % 64) as u64;
                if get_bit64_macro!(self.bits[word_idx], bit_idx) && f(i) {
                    new_bits[word_idx] = set_bit64_macro!(new_bits[word_idx], bit_idx, true);
                }
            }
            ArraySetEnumMtEph {
                bits: new_bits,
                universe_size: self.universe_size,
            }
        }

        fn intersection(&self, other: &Self) -> (common: Self)
            ensures
                common@ == self@.intersect(other@),
                common@.finite(),
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
                    forall|k: int, j: int| #![auto]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] && u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let and_word: u64 = w1 & w2;
                proof {
                    bit_and_64_proof(w1, w2, and_word);
                }
                result_bits.push(and_word);
                i = i + 1;
            }
            let common = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            proof {
                assert forall|elem: usize|
                    #[trigger] common@.contains(elem) == self@.intersect(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        assert(0 <= k < n as int);
                        assert(0 <= j < 64);
                        assert(u64_view(common.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] && u64_view(other.bits@[k])[j]));
                    }
                }
                assert(common@ =~= self@.intersect(other@));
                lemma_view_finite(common.bits@, self.universe_size);
            }
            common
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures
                remaining@ == self@.difference(other@),
                remaining@.finite(),
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
                    forall|k: int, j: int| #![auto]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] && !u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let andnot_word: u64 = w1 & !w2;
                proof {
                    bit_andnot_64_proof(w1, w2, andnot_word);
                }
                result_bits.push(andnot_word);
                i = i + 1;
            }
            let remaining = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            proof {
                assert forall|elem: usize|
                    #[trigger] remaining@.contains(elem) == self@.difference(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        assert(0 <= k < n as int);
                        assert(0 <= j < 64);
                        assert(u64_view(remaining.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] && !u64_view(other.bits@[k])[j]));
                    }
                }
                assert(remaining@ =~= self@.difference(other@));
                lemma_view_finite(remaining.bits@, self.universe_size);
            }
            remaining
        }

        fn union(&self, other: &Self) -> (combined: Self)
            ensures
                combined@ == self@.union(other@),
                combined@.finite(),
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
                    forall|k: int, j: int| #![auto]
                        0 <= k < i && 0 <= j < 64 ==>
                            u64_view(result_bits@[k])[j] ==
                                (u64_view(self.bits@[k])[j] || u64_view(other.bits@[k])[j]),
                decreases n - i,
            {
                let w1 = self.bits[i];
                let w2 = other.bits[i];
                let or_word: u64 = w1 | w2;
                proof {
                    bit_or_64_proof(w1, w2, or_word);
                }
                result_bits.push(or_word);
                i = i + 1;
            }
            let combined = ArraySetEnumMtEph { bits: result_bits, universe_size: self.universe_size };
            proof {
                assert forall|elem: usize|
                    #[trigger] combined@.contains(elem) == self@.union(other@).contains(elem) by
                {
                    if (elem as int) < self.universe_size as int {
                        let k = elem as int / 64;
                        let j = elem as int % 64;
                        assert(0 <= k < n as int);
                        assert(0 <= j < 64);
                        assert(u64_view(combined.bits@[k])[j] ==
                            (u64_view(self.bits@[k])[j] || u64_view(other.bits@[k])[j]));
                    }
                }
                assert(combined@ =~= self@.union(other@));
                lemma_view_finite(combined.bits@, self.universe_size);
            }
            combined
        }

        fn find(&self, x: usize) -> (found: B)
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

        fn delete(&mut self, x: usize)
            ensures
                self@ == old(self)@.remove(x),
                self@.finite(),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size(),
        {
            if x < self.universe_size {
                let word_idx: usize = x / 64;
                let bit_idx: u64 = (x % 64) as u64;
                let old_word: u64 = self.bits[word_idx];
                let new_word: u64 = set_bit64_macro!(old_word, bit_idx, false);
                proof {
                    set_bit64_proof(new_word, old_word, bit_idx, false);
                }
                self.bits.set(word_idx, new_word);
                proof {
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
                                    assert(ej != bit_idx as int);
                                    // Different bit in same word
                                }
                            } else {
                                // Different word: bits unchanged
                            }
                        }
                    }
                    assert(self@ =~= old(self)@.remove(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            } else {
                // x not in set (x >= universe_size), so remove(x) is identity.
                proof {
                    assert(self@ =~= old(self)@.remove(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            }
        }

        fn insert(&mut self, x: usize)
            ensures
                (x < old(self).spec_universe_size() ==> self@ == old(self)@.insert(x)),
                (x >= old(self).spec_universe_size() ==> self@ == old(self)@),
                self@.finite(),
                self.spec_arraysetenummteph_wf(),
                self.spec_universe_size() == old(self).spec_universe_size(),
        {
            if x < self.universe_size {
                let word_idx: usize = x / 64;
                let bit_idx: u64 = (x % 64) as u64;
                let old_word: u64 = self.bits[word_idx];
                let new_word: u64 = set_bit64_macro!(old_word, bit_idx, true);
                proof {
                    set_bit64_proof(new_word, old_word, bit_idx, true);
                }
                self.bits.set(word_idx, new_word);
                proof {
                    assert forall|elem: usize|
                        #[trigger] self@.contains(elem) == old(self)@.insert(x).contains(elem) by
                    {
                        if elem == x {
                            // Inserted bit: set_bit64_proof ensures it's true
                        } else if (elem as int) < self.universe_size as int {
                            let ek = elem as int / 64;
                            let ej = elem as int % 64;
                            if ek == word_idx as int {
                                assert(ej != bit_idx as int);
                            }
                        }
                    }
                    assert(self@ =~= old(self)@.insert(x));
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            } else {
                proof {
                    assert(self@ =~= old(self)@);
                    lemma_view_finite(self.bits@, self.universe_size);
                }
            }
        }
    }

    // 11. derive impls in verus!

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
            proof { accept(equal == (self@ == other@)); }  // accept hole: PartialEq
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
            proof { accept(cloned@ == self@); }  // accept hole: Vec::clone external_body
            cloned
        }
    }

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

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
