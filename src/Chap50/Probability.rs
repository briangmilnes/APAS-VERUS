//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Probability wrapper type for OBST algorithms.
//!
//! Uses f64 for probability values. Impls use external_body for f64 operations.

pub mod Probability {

    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, Div, Mul, Sub};

    use vstd::prelude::*;

    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    #[derive(Clone, Copy)]
    pub struct Probability(pub f64);

    // 8. traits
    /// Trait for probability operations
    pub trait ProbabilityTrait: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 wrapper construction
        fn new(value: f64) -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 field access
        fn value(&self) -> f64;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 constant construction
        fn infinity() -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 constant construction
        fn zero() -> Self;
    }

    // 9. impls
    impl ProbabilityTrait for Probability {
        #[verifier::external_body]
        fn new(value: f64) -> Self {
            debug_assert!(value >= 0.0, "Probability must be non-negative");
            Probability(value)
        }

        #[verifier::external_body]
        fn value(&self) -> f64 { self.0 }

        #[verifier::external_body]
        fn infinity() -> Self { Probability(f64::INFINITY) }

        #[verifier::external_body]
        fn zero() -> Self { Probability(0.0) }
    }

    // 11. derive impls
    impl Default for Probability {
        #[verifier::external_body]
        fn default() -> Self { <Probability as ProbabilityTrait>::zero() }
    }

    impl PartialEq for Probability {
        #[verifier::external_body]
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — bit-level f64 comparison
        fn eq(&self, other: &Self) -> bool {
            self.0.to_bits() == other.0.to_bits()
        }
    }

    impl Eq for Probability {}

    impl PartialOrd for Probability {
        #[verifier::external_body]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl Ord for Probability {
        #[verifier::external_body]
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — NaN-aware f64 comparison
        fn cmp(&self, other: &Self) -> Ordering {
            match (self.0.is_nan(), other.0.is_nan()) {
                | (true, true) => Ordering::Equal,
                | (true, false) => Ordering::Greater,
                | (false, true) => Ordering::Less,
                | (false, false) => {
                    if self.0 < other.0 {
                        Ordering::Less
                    } else if self.0 > other.0 {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }
        }
    }

    impl Hash for Probability {
        #[verifier::external_body]
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — hash f64 bits
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.to_bits().hash(state); }
    }

    impl From<f64> for Probability {
        #[verifier::external_body]
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 wrapping
        fn from(value: f64) -> Self { Probability(value) }
    }

    impl From<Probability> for f64 {
        #[verifier::external_body]
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — f64 unwrapping
        fn from(prob: Probability) -> Self { prob.0 }
    }

    impl Add for Probability {
        type Output = Self;

        #[verifier::external_body]
        fn add(self, other: Self) -> Self { Probability(self.0 + other.0) }
    }

    impl Sub for Probability {
        type Output = Self;

        #[verifier::external_body]
        fn sub(self, other: Self) -> Self { Probability(self.0 - other.0) }
    }

    impl Mul for Probability {
        type Output = Self;

        #[verifier::external_body]
        fn mul(self, other: Self) -> Self { Probability(self.0 * other.0) }
    }

    impl Div for Probability {
        type Output = Self;

        #[verifier::external_body]
        fn div(self, other: Self) -> Self { Probability(self.0 / other.0) }
    }

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for Probability {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format f64 to debug string
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "Probability({})", self.0) }
    }

    impl Display for Probability {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format f64 to display string
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{}", self.0) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! prob {
        ($value:expr) => {
            <$crate::Chap50::Probability::Probability::Probability as $crate::Chap50::Probability::Probability::ProbabilityTrait>::new($value)
        };
    }
}
