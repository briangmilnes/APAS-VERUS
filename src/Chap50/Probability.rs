//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Probability wrapper type for OBST algorithms.
//!
//! Note: Provides a probability wrapper that implements Eq/Ord for f64 values
//! while maintaining compatibility with APAS MtVal trait requirements.

pub mod Probability {

    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::{Hash, Hasher};
    use std::ops::{Add, Div, Mul, Sub};

    use crate::Types::Types::*;

    /// Trait for probability operations
    pub trait ProbabilityTrait {
        /// Create new probability from f64
        /// APAS: Work Θ(1), Span Θ(1)
        fn new(value: f64) -> Self;

        /// Get the underlying f64 value
        /// APAS: Work Θ(1), Span Θ(1)
        fn value(&self)    -> f64;
    }

    #[derive(Clone, Copy)]
    pub struct Probability(pub f64);

    impl Probability {
        /// Create a new probability value
        /// Claude Work: O(1) - constant time construction
        /// Claude Span: O(1) - constant time construction
        pub fn new(value: f64) -> Self {
            debug_assert!(value >= 0.0, "Probability must be non-negative");
            // Allow values > 1.0 for costs in dynamic programming
            Probability(value)
        }

        /// Get the inner f64 value
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn value(&self) -> f64 { self.0 }

        /// Create infinity value for fold operations
        /// Claude Work: O(1) - constant time construction
        /// Claude Span: O(1) - constant time construction
        pub fn infinity() -> Self { Probability(f64::INFINITY) }

        /// Create zero probability
        /// Claude Work: O(1) - constant time construction
        /// Claude Span: O(1) - constant time construction
        pub fn zero() -> Self { Probability(0.0) }
    }

    impl Default for Probability {
        fn default() -> Self { Probability::zero() }
    }

    impl PartialEq for Probability {
        fn eq(&self, other: &Self) -> bool {
            // Use bit-level comparison for exact equality
            self.0.to_bits() == other.0.to_bits()
        }
    }

    impl Eq for Probability {}

    impl PartialOrd for Probability {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl Ord for Probability {
        fn cmp(&self, other: &Self) -> Ordering {
            // Handle NaN and infinity cases properly
            match (self.0.is_nan(), other.0.is_nan()) {
                | (true, true) => Ordering::Equal,
                | (true, false) => Ordering::Greater, // NaN > everything
                | (false, true) => Ordering::Less,    // everything < NaN
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
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.to_bits().hash(state); }
    }

    impl Debug for Probability {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "Probability({})", self.0) }
    }

    impl Display for Probability {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "{}", self.0) }
    }

    impl From<f64> for Probability {
        fn from(value: f64) -> Self { Probability(value) }
    }

    impl From<Probability> for f64 {
        fn from(prob: Probability) -> Self { prob.0 }
    }

    // Arithmetic operations
    impl Add for Probability {
        type Output = Self;

        fn add(self, other: Self) -> Self { Probability(self.0 + other.0) }
    }

    impl Sub for Probability {
        type Output = Self;

        fn sub(self, other: Self) -> Self { Probability(self.0 - other.0) }
    }

    impl Mul for Probability {
        type Output = Self;

        fn mul(self, other: Self) -> Self { Probability(self.0 * other.0) }
    }

    impl Div for Probability {
        type Output = Self;

        fn div(self, other: Self) -> Self { Probability(self.0 / other.0) }
    }

    // APAS trait implementations are automatic due to blanket impl in Types.rs
    // Probability implements Eq + Clone + Display + Debug + Sized, so it gets StT automatically
    // Probability implements StT + Send + Sync, so it gets StTInMtT automatically
    // Probability implements StTInMtT + 'static, so it gets MtVal automatically

    // Convenience macro for creating probability literals
    #[macro_export]
    macro_rules! prob {
        ($value:expr) => {
            $crate::Chap50::Probability::Probability::Probability::new($value)
        };
    }
}
