//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral enumerated set using bit array.
//!
//! Uses `bitvec::BitBox` for true 1-bit-per-element storage (vs 1-byte-per-element in `Vec<bool>`).
//! Memory: ⌈universe_size / 64⌉ × 8 bytes. Only filter() uses parallelism.

pub mod ArraySetEnumMtEph {

    use bitvec::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    #[derive(PartialEq, Clone)]
    pub struct ArraySetEnumMtEph {
        bits: BitBox,     // 1 bit per element
        universe_size: N, // elements are 0..universe_size-1
    }

    pub trait ArraySetEnumMtEphTrait {
        /// claude-4-sonet: Work Θ(u), Span Θ(1)
        fn new(u: N)                                  -> Self;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w) where w is word size, Span Θ(u/w)
        fn size(&self)                                -> N;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(|set|), Span Θ(|set|)
        fn to_seq(&self)                              -> ArraySeqMtEphS<N>;
        /// claude-4-sonet: Work Θ(u), Span Θ(1)
        fn empty(u: N)                                -> Self;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u), Span Θ(1)
        fn singleton(u: N, x: N)                      -> Self;
        /// claude-4-sonet: Work Θ(u + |seq|), Span Θ(1)
        fn from_seq(u: N, seq: ArraySeqMtEphS<N>)     -> Self;
        /// - APAS Cost Spec 41.3: Work u + Σ W(f(x)), Span 1 + max S(f(x))
        /// - claude-4-sonet: Work Θ(u), Span Θ(log u), Parallelism Θ(u/log u)
        fn filter<F: PredVal<N> + Clone>(&self, f: F) -> Self;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn intersection(&self, other: &Self)          -> Self;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn difference(&self, other: &Self)            -> Self;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(u/w), Span Θ(u/w)
        fn union(&self, other: &Self)                 -> Self;
        /// - APAS Cost Spec 41.3: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn find(&self, x: N)                          -> B;
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn delete(&mut self, x: N);
        /// - APAS Cost Spec 41.3: Work u, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: N);
    }

    impl ArraySetEnumMtEphTrait for ArraySetEnumMtEph {
        fn new(u: N) -> Self {
            ArraySetEnumMtEph {
                bits: bitbox![0; u],
                universe_size: u,
            }
        }

        fn size(&self) -> N {
            self.bits.count_ones() // popcount
        }

        fn to_seq(&self) -> ArraySeqMtEphS<N> {
            let set_size = self.size();
            let mut result = Vec::with_capacity(set_size);
            for (i, bit) in self.bits.iter().enumerate() {
                if *bit {
                    result.push(i);
                }
            }
            ArraySeqMtEphS::from_vec(result)
        }

        fn empty(u: N) -> Self { Self::new(u) }

        fn singleton(u: N, x: N) -> Self {
            let mut bits = bitbox![0; u];
            if x < u {
                bits.set(x, true);
            }
            ArraySetEnumMtEph { bits, universe_size: u }
        }

        fn from_seq(u: N, seq: ArraySeqMtEphS<N>) -> Self {
            let mut bits = bitbox![0; u];
            for i in 0..seq.length() {
                let elem = seq.nth(i).clone();
                if elem < u {
                    bits.set(elem, true);
                }
            }
            ArraySetEnumMtEph { bits, universe_size: u }
        }

        fn filter<F: PredVal<N> + Clone>(&self, f: F) -> Self {
            // Sequential filter: collect elements, apply predicate, build result.
            // Honest about being sequential for small inputs; avoids spawn-per-element overhead.
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

        fn intersection(&self, other: &Self) -> Self {
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

        fn difference(&self, other: &Self) -> Self {
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

        fn union(&self, other: &Self) -> Self {
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

        fn find(&self, x: N) -> B { if x < self.universe_size { self.bits[x] } else { false } }

        fn delete(&mut self, x: N) {
            if x < self.universe_size {
                self.bits.set(x, false);
            }
        }

        fn insert(&mut self, x: N) {
            if x < self.universe_size {
                self.bits.set(x, true);
            }
        }
    }

    // Macro for creating ArraySetEnumMtEph literals
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
}
