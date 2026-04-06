//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 30: Probability wrapper type.
//!
//! Uses f64 for probability values. Impls use external_body for f64 operations.
//! This is really a very minimal shell until we get better float operations in Verus.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod Probability {


    //		Section 2. imports

    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, Div, Mul, Sub};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::ops::AddSpecImpl;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::ops::SubSpecImpl;

    use crate::Types::Types::*;

    verus! 
{

    //		Section 4. type definitions


    #[derive(Clone, Copy)]
    pub struct Probability(pub f64);

    //		Section 8. traits


    /// Trait for probability operations
    pub trait ProbabilityTrait: Sized {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 wrapper construction.
        fn new(p: f64) -> Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 field access.
        fn value(&self) -> f64;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 constant construction.
        fn infinity() -> Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 constant construction.
        fn zero() -> Self;
    }

    //		Section 9. impls


    impl ProbabilityTrait for Probability {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 wrapper construction.
        fn new(p: f64) -> Self { Probability(p) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 field access.
        fn value(&self) -> f64 { self.0 }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 constant construction.
        #[verifier::external_body] // accept hole
        fn infinity() -> Self { Probability(f64::INFINITY) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 constant construction.
        fn zero() -> Self { Probability(0.0) }
    }

    impl PartialOrd for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to cmp().
        #[verifier::external_body] // accept hole
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl Ord for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — NaN-aware f64 comparison.

        #[verifier::external_body] // accept hole
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

    impl From<f64> for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 wrapping.
        #[verifier::external_body] // accept hole
        fn from(value: f64) -> Self { Probability(value) }
    }

    impl From<Probability> for f64 {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 unwrapping.
        #[verifier::external_body] // accept hole
        fn from(prob: Probability) -> Self { prob.0 }
    }

    impl Add for Probability {
        type Output = Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 addition.
        #[verifier::external_body] // accept hole
        fn add(self, other: Self) -> Self { Probability(self.0 + other.0) }
    }

    impl Sub for Probability {
        type Output = Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 subtraction.
        #[verifier::external_body] // accept hole
        fn sub(self, other: Self) -> Self { Probability(self.0 - other.0) }
    }

    impl Mul for Probability {
        type Output = Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 multiplication.
        #[verifier::external_body] // accept hole
        fn mul(self, other: Self) -> Self { Probability(self.0 * other.0) }
    }

    impl Div for Probability {
        type Output = Self;

        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — f64 division.
        #[verifier::external_body] // accept hole
        fn div(self, other: Self) -> Self { Probability(self.0 / other.0) }
    }

    #[cfg(verus_keep_ghost)]
    impl AddSpecImpl for Probability {
        open spec fn obeys_add_spec() -> bool { false }
        open spec fn add_req(self, rhs: Probability) -> bool { true }
        open spec fn add_spec(self, rhs: Probability) -> Probability { arbitrary() }
    }

    #[cfg(verus_keep_ghost)]
    impl SubSpecImpl for Probability {
        open spec fn obeys_sub_spec() -> bool { false }
        open spec fn sub_req(self, rhs: Probability) -> bool { true }
        open spec fn sub_spec(self, rhs: Probability) -> Probability { arbitrary() }
    }

    //		Section 12. derive impls in verus!


    // 11. derive impls
    impl Default for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — delegates to zero().
        fn default() -> Self { <Probability as ProbabilityTrait>::zero() }
    }

    impl PartialEq for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — bit-level f64 comparison.

        #[verifier::external_body] // accept hole
        fn eq(&self, other: &Self) -> bool {
            self.0.to_bits() == other.0.to_bits()
        }
    }

    impl Eq for Probability {}

    impl Hash for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — hash f64 bits.

        #[verifier::external_body]  // accept hole
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.to_bits().hash(state); }
    }
    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! prob {
        ($value:expr) => {
            <$crate::Chap30::Probability::Probability::Probability as $crate::Chap30::Probability::Probability::ProbabilityTrait>::new($value)
        };
    }

    //		Section 14. derive impls outside verus!

    impl Debug for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format f64 to debug string.
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "Probability({})", self.0) }
    }

    impl Display for Probability {
        /// - Alg Analysis: APAS: (no cost stated)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format f64 to display string.
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{}", self.0) }
    }
}
