//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral enumerated set using bit array.
//!
//! Uses `bitvec::BitBox` for true 1-bit-per-element storage (vs 1-byte-per-element in `Vec<bool>`).
//! Memory: ⌈universe_size / 64⌉ × 8 bytes. Only filter() uses parallelism.

pub mod ArraySetEnumMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use bitvec::prelude::*;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    pub struct ArraySetEnumMtEph {
        bits: BitBox,
        universe_size: usize,
    }

    // 5. view impls

    impl View for ArraySetEnumMtEph {
        type V = Set<usize>;
        #[verifier::external_body]
        open spec fn view(&self) -> Set<usize> { Set::empty() }
    }

    // 8. traits

    pub trait ArraySetEnumMtEphTrait {
        /// claude-4-sonet: Work Θ(u), Span Θ(1)
        fn new(u: usize) -> (result: Self)
            ensures result@ == Set::<usize>::empty();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w) where w is word size, Span Θ(u/w)
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(|set|), Span Θ(|set|)
        fn to_seq(&self) -> (result: ArraySeqMtEphS<usize>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(u), Span Θ(1)
        fn empty(u: usize) -> (result: Self)
            ensures result@ == Set::<usize>::empty();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u), Span Θ(1)
        fn singleton(u: usize, x: usize) -> (result: Self)
            ensures
                (x < u ==> result@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> result@ == Set::<usize>::empty()),
                result@.finite();
        /// claude-4-sonet: Work Θ(u + |seq|), Span Θ(1)
        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (result: Self)
            ensures result@.finite();
        /// - APAS Cost Spec 41.3: Work u + Σ W(f(x)), Span 1 + max S(f(x))
        /// - claude-4-sonet: Work Θ(u), Span Θ(log u), Parallelism Θ(u/log u)
        fn filter<F: PredVal<usize> + Clone>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// - APAS Cost Spec 41.3: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn find(&self, x: usize) -> (result: B)
            ensures result == self@.contains(x);
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn delete(&mut self, x: usize)
            ensures self@ == old(self)@.remove(x), self@.finite();
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: usize)
            ensures self@ == old(self)@.insert(x), self@.finite();
    }

    // 9. impls

    impl ArraySetEnumMtEphTrait for ArraySetEnumMtEph {
        #[verifier::external_body]
        fn new(u: usize) -> (result: Self)
            ensures result@ == Set::<usize>::empty()
        {
            ArraySetEnumMtEph {
                bits: bitbox![0; u],
                universe_size: u,
            }
        }

        #[verifier::external_body]
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite()
        {
            self.bits.count_ones()
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (result: ArraySeqMtEphS<usize>)
            ensures self@.finite()
        {
            let set_size = self.size();
            let mut result_vec = Vec::with_capacity(set_size);
            for (i, bit) in self.bits.iter().enumerate() {
                if *bit {
                    result_vec.push(i);
                }
            }
            ArraySeqMtEphS::from_vec(result_vec)
        }

        #[verifier::external_body]
        fn empty(u: usize) -> (result: Self)
            ensures result@ == Set::<usize>::empty()
        {
            Self::new(u)
        }

        #[verifier::external_body]
        fn singleton(u: usize, x: usize) -> (result: Self)
            ensures
                (x < u ==> result@ == Set::<usize>::empty().insert(x)),
                (x >= u ==> result@ == Set::<usize>::empty()),
                result@.finite()
        {
            let mut bits = bitbox![0; u];
            if x < u {
                bits.set(x, true);
            }
            ArraySetEnumMtEph { bits, universe_size: u }
        }

        #[verifier::external_body]
        fn from_seq(u: usize, seq: ArraySeqMtEphS<usize>) -> (result: Self)
            ensures result@.finite()
        {
            let mut bits = bitbox![0; u];
            for i in 0..seq.length() {
                let elem = seq.nth(i).clone();
                if elem < u {
                    bits.set(elem, true);
                }
            }
            ArraySetEnumMtEph { bits, universe_size: u }
        }

        #[verifier::external_body]
        fn filter<F: PredVal<usize> + Clone>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@)
        {
            let mut new_bits = bitbox![0; self.universe_size];

            for i in 0..self.universe_size {
                if self.bits[i] && f(i) {
                    new_bits.set(i, true);
                }
            }

            ArraySetEnumMtEph {
                bits: new_bits,
                universe_size: self.universe_size,
            }
        }

        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite()
        {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = bitbox![0; self.universe_size];
            for i in 0..self.universe_size {
                result_bits.set(i, self.bits[i] && other.bits[i]);
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite()
        {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = bitbox![0; self.universe_size];
            for i in 0..self.universe_size {
                result_bits.set(i, self.bits[i] && !other.bits[i]);
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        #[verifier::external_body]
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite()
        {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = bitbox![0; self.universe_size];
            for i in 0..self.universe_size {
                result_bits.set(i, self.bits[i] || other.bits[i]);
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        #[verifier::external_body]
        fn find(&self, x: usize) -> (result: B)
            ensures result == self@.contains(x)
        {
            if x < self.universe_size { self.bits[x] } else { false }
        }

        #[verifier::external_body]
        fn delete(&mut self, x: usize)
            ensures self@ == old(self)@.remove(x), self@.finite()
        {
            if x < self.universe_size {
                self.bits.set(x, false);
            }
        }

        #[verifier::external_body]
        fn insert(&mut self, x: usize)
            ensures self@ == old(self)@.insert(x), self@.finite()
        {
            if x < self.universe_size {
                self.bits.set(x, true);
            }
        }
    }

    // 11. derive impls in verus!

    impl Clone for ArraySetEnumMtEph {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            ArraySetEnumMtEph {
                bits: self.bits.clone(),
                universe_size: self.universe_size,
            }
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

    impl PartialEq for ArraySetEnumMtEph {
        fn eq(&self, other: &Self) -> bool {
            self.universe_size == other.universe_size && {
                for i in 0..self.universe_size {
                    if self.bits[i] != other.bits[i] {
                        return false;
                    }
                }
                true
            }
        }
    }
}
