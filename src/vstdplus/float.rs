// Copyright (c) 2025 Brian G. Milnes
//! Floating-point total order axioms for finite (non-NaN, non-infinite) values.
//!
//! Verus deliberately omits axioms about floating point from vstd because the desired
//! set of useful and sound axioms may vary by project and platform.  This module provides
//! the axioms APAS-VERUS needs: a total order on finite f32/f64 values, grounded in
//! vstd's uninterpreted `le_ensures`.
//!
//! Usage:
//!   broadcast use crate::vstdplus::float::float::group_float_finite_total_order;
//!
//! The trait `FloatTotalOrder` guards every axiom with `Self::float_wf(x)`, which for
//! f32/f64 means `is_finite_spec()` (excludes NaN and infinity).

pub mod float {
    use core::cmp::Ordering;
    use vstd::prelude::*;
    use vstd::float::FloatBitsProperties;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::le_ensures;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // A floating-point total order, conditional on well-formedness (finite, non-NaN,
    // non-infinite).  Every axiom requires well-formedness of its operands.
    pub trait FloatTotalOrder: Sized {
        /// True when x is a well-formed value (finite, non-NaN, non-infinite).
        spec fn float_wf(x: Self) -> bool;

        /// Spec-level <=, defined via the uninterpreted `le_ensures`.
        spec fn le(self, other: Self) -> bool;

        // Reflexive: a <= a.
        proof fn reflexive(a: Self)
            requires Self::float_wf(a),
            ensures  Self::le(a, a);

        // Antisymmetric: a <= b && b <= a ==> a == b.
        proof fn antisymmetric(a: Self, b: Self)
            requires Self::float_wf(a), Self::float_wf(b),
                     Self::le(a, b), Self::le(b, a),
            ensures  a == b;

        // Transitive: a <= b && b <= c ==> a <= c.
        proof fn transitive(a: Self, b: Self, c: Self)
            requires Self::float_wf(a), Self::float_wf(b), Self::float_wf(c),
                     Self::le(a, b), Self::le(b, c),
            ensures  Self::le(a, c);

        // Totality (strongly connected): a <= b || b <= a.
        proof fn totality(a: Self, b: Self)
            requires Self::float_wf(a), Self::float_wf(b),
            ensures  Self::le(a, b) || Self::le(b, a);

        /// Exec comparison returning Ordering.
        fn float_cmp(&self, other: &Self) -> (c: Ordering)
            requires Self::float_wf(*self), Self::float_wf(*other),
            ensures
                (match c {
                    Ordering::Less    => self.le(*other) && !other.le(*self),
                    Ordering::Equal   => self.le(*other) && other.le(*self),
                    Ordering::Greater => other.le(*self) && !self.le(*other),
                });
    }

    // Every element of a sequence is well-formed.
    pub open spec fn all_float_wf<T: FloatTotalOrder>(s: Seq<T>) -> bool {
        forall|i: int| #![trigger s[i]] 0 <= i < s.len() ==> T::float_wf(s[i])
    }

    // Pairwise ordered under the float total order.
    pub open spec fn spec_float_sorted<T: FloatTotalOrder>(s: Seq<T>) -> bool {
        forall|a: int, b: int| 0 <= a < b < s.len() ==> #[trigger] s[a].le(s[b])
    }

    // f64

    impl FloatTotalOrder for f64 {
        open spec fn float_wf(x: f64) -> bool {
            x.is_finite_spec()
        }

        open spec fn le(self, other: Self) -> bool {
            le_ensures::<f64>(self, other, true)
        }

        proof fn reflexive(a: Self) {
            axiom_f64_reflexive(a);
        }

        proof fn antisymmetric(a: Self, b: Self) {
            axiom_f64_antisymmetric(a, b);
        }

        proof fn transitive(a: Self, b: Self, c: Self) {
            axiom_f64_transitive(a, b, c);
        }

        proof fn totality(a: Self, b: Self) {
            axiom_f64_totality(a, b);
        }

        #[verifier::external_body]
        fn float_cmp(&self, other: &Self) -> (c: Ordering) {
            match self.partial_cmp(other) {
                Some(c) => c,
                None => Ordering::Less, // unreachable for finite values
            }
        }
    }

    // f64 broadcast axioms.  Each is an IEEE 754 truth for finite values.

    pub broadcast axiom fn axiom_f64_le_functional(a: f64, b: f64)
        ensures
            #![trigger le_ensures::<f64>(a, b, true), le_ensures::<f64>(a, b, false)]
            !(le_ensures::<f64>(a, b, true) && le_ensures::<f64>(a, b, false));

    pub broadcast axiom fn axiom_f64_reflexive(a: f64)
        requires a.is_finite_spec(),
        ensures  #[trigger] le_ensures::<f64>(a, a, true);

    pub broadcast axiom fn axiom_f64_antisymmetric(a: f64, b: f64)
        requires a.is_finite_spec(), b.is_finite_spec(),
                 le_ensures::<f64>(a, b, true), le_ensures::<f64>(b, a, true),
        ensures
            #![trigger le_ensures::<f64>(a, b, true), le_ensures::<f64>(b, a, true)]
            a == b;

    pub broadcast axiom fn axiom_f64_transitive(a: f64, b: f64, c: f64)
        requires a.is_finite_spec(), b.is_finite_spec(), c.is_finite_spec(),
                 le_ensures::<f64>(a, b, true), le_ensures::<f64>(b, c, true),
        ensures
            #![trigger le_ensures::<f64>(a, b, true), le_ensures::<f64>(b, c, true)]
            le_ensures::<f64>(a, c, true);

    pub broadcast axiom fn axiom_f64_totality(a: f64, b: f64)
        requires a.is_finite_spec(), b.is_finite_spec(),
        ensures
            #![trigger le_ensures::<f64>(a, b, true), le_ensures::<f64>(b, a, true)]
            le_ensures::<f64>(a, b, true) || le_ensures::<f64>(b, a, true);

    // f32

    impl FloatTotalOrder for f32 {
        open spec fn float_wf(x: f32) -> bool {
            x.is_finite_spec()
        }

        open spec fn le(self, other: Self) -> bool {
            le_ensures::<f32>(self, other, true)
        }

        proof fn reflexive(a: Self) {
            axiom_f32_reflexive(a);
        }

        proof fn antisymmetric(a: Self, b: Self) {
            axiom_f32_antisymmetric(a, b);
        }

        proof fn transitive(a: Self, b: Self, c: Self) {
            axiom_f32_transitive(a, b, c);
        }

        proof fn totality(a: Self, b: Self) {
            axiom_f32_totality(a, b);
        }

        #[verifier::external_body]
        fn float_cmp(&self, other: &Self) -> (c: Ordering) {
            match self.partial_cmp(other) {
                Some(c) => c,
                None => Ordering::Less,
            }
        }
    }

    // f32 broadcast axioms.

    pub broadcast axiom fn axiom_f32_le_functional(a: f32, b: f32)
        ensures
            #![trigger le_ensures::<f32>(a, b, true), le_ensures::<f32>(a, b, false)]
            !(le_ensures::<f32>(a, b, true) && le_ensures::<f32>(a, b, false));

    pub broadcast axiom fn axiom_f32_reflexive(a: f32)
        requires a.is_finite_spec(),
        ensures  #[trigger] le_ensures::<f32>(a, a, true);

    pub broadcast axiom fn axiom_f32_antisymmetric(a: f32, b: f32)
        requires a.is_finite_spec(), b.is_finite_spec(),
                 le_ensures::<f32>(a, b, true), le_ensures::<f32>(b, a, true),
        ensures
            #![trigger le_ensures::<f32>(a, b, true), le_ensures::<f32>(b, a, true)]
            a == b;

    pub broadcast axiom fn axiom_f32_transitive(a: f32, b: f32, c: f32)
        requires a.is_finite_spec(), b.is_finite_spec(), c.is_finite_spec(),
                 le_ensures::<f32>(a, b, true), le_ensures::<f32>(b, c, true),
        ensures
            #![trigger le_ensures::<f32>(a, b, true), le_ensures::<f32>(b, c, true)]
            le_ensures::<f32>(a, c, true);

    pub broadcast axiom fn axiom_f32_totality(a: f32, b: f32)
        requires a.is_finite_spec(), b.is_finite_spec(),
        ensures
            #![trigger le_ensures::<f32>(a, b, true), le_ensures::<f32>(b, a, true)]
            le_ensures::<f32>(a, b, true) || le_ensures::<f32>(b, a, true);

    // WrappedF64: newtype wrapper giving f64 a View impl for use in ArraySeq and other
    // Verus containers that require View.

    #[derive(Clone, Copy)]
    pub struct WrappedF64 {
        pub val: f64,
    }

    impl View for WrappedF64 {
        type V = f64;
        open spec fn view(&self) -> f64 { self.val }
    }

    impl WrappedF64 {
        pub open spec fn spec_is_finite(&self) -> bool {
            self.val.is_finite_spec()
        }

        #[verifier::external_body]
        pub fn is_finite(&self) -> (b: bool)
            ensures b == self.spec_is_finite()
        {
            self.val.is_finite()
        }

        #[verifier::external_body]
        pub fn eq(&self, other: &Self) -> (b: bool)
            ensures b == (self@ == other@)
        {
            self.val == other.val
        }

        #[verifier::external_body]
        pub fn dist_le(&self, other: &Self) -> (b: bool)
            requires self.spec_is_finite(), other.spec_is_finite(),
            ensures b == self.val.le(other.val)
        {
            self.val <= other.val
        }

        #[verifier::external_body]
        pub fn dist_lt(&self, other: &Self) -> (b: bool)
            requires self.spec_is_finite(), other.spec_is_finite(),
            ensures b == (self.val.le(other.val) && self.val != other.val)
        {
            self.val < other.val
        }

        #[verifier::external_body]
        pub fn dist_add(&self, other: &Self) -> (r: Self)
            ensures r@ == f64_add_spec(self@, other@)
        {
            WrappedF64 { val: self.val + other.val }
        }

        #[verifier::external_body]
        pub fn dist_sub(&self, other: &Self) -> (r: Self)
            ensures r@ == f64_sub_spec(self@, other@)
        {
            WrappedF64 { val: self.val - other.val }
        }

        /// Approximately equal within epsilon (1e-9). Used for path-weight validation.
        #[verifier::external_body]
        pub fn approx_eq(&self, other: &Self) -> (b: bool)
            requires self.spec_is_finite(), other.spec_is_finite(),
            ensures b == f64_approx_eq_spec(self@, other@)
        {
            let diff = self.val - other.val;
            let abs_diff = if diff >= 0.0 { diff } else { -diff };
            abs_diff <= 1e-9
        }
    }

    // Exec bridge for f64::is_finite().
    #[verifier::external_body]
    pub fn f64_is_finite(x: f64) -> (b: bool)
        ensures b == x.is_finite_spec()
    {
        x.is_finite()
    }

    // Uninterpreted sentinel for unreachable distance (f64::INFINITY at runtime).
    pub uninterp spec fn UNREACHABLE_SPEC() -> f64;

    // f64 constant axioms.

    pub broadcast axiom fn axiom_f64_zero_is_finite()
        ensures #[trigger] (0.0f64).is_finite_spec();

    pub broadcast axiom fn axiom_f64_unreachable_not_finite()
        ensures #[trigger] UNREACHABLE_SPEC().is_finite_spec() == false;

    #[verifier::external_body]
    pub fn unreachable_dist() -> (d: WrappedF64)
        ensures d@ == UNREACHABLE_SPEC(),
                !d.spec_is_finite(),
    {
        WrappedF64 { val: f64::INFINITY }
    }

    #[verifier::external_body]
    pub fn zero_dist() -> (d: WrappedF64)
        ensures d.spec_is_finite(),
                d@ == 0.0f64,
    {
        WrappedF64 { val: 0.0 }
    }

    pub fn finite_dist(v: f64) -> (d: WrappedF64)
        requires v.is_finite_spec(),
        ensures d.spec_is_finite(),
                d@ == v,
    {
        WrappedF64 { val: v }
    }

    // Uninterpreted spec functions for f64 arithmetic (Verus has no spec_add for f64).

    pub uninterp spec fn f64_add_spec(a: f64, b: f64) -> f64;
    pub uninterp spec fn f64_sub_spec(a: f64, b: f64) -> f64;
    pub uninterp spec fn f64_approx_eq_spec(a: f64, b: f64) -> bool;

    // f64 arithmetic axioms.

    pub broadcast axiom fn axiom_f64_add_zero_right(a: f64)
        requires a.is_finite_spec(),
        ensures #[trigger] f64_add_spec(a, 0.0f64) == a;

    pub broadcast axiom fn axiom_f64_add_commutative(a: f64, b: f64)
        ensures
            #![trigger f64_add_spec(a, b), f64_add_spec(b, a)]
            f64_add_spec(a, b) == f64_add_spec(b, a);

    pub broadcast axiom fn axiom_f64_add_finite_preserves(a: f64, b: f64)
        requires a.is_finite_spec(), b.is_finite_spec(),
                 f64_add_spec(a, b).is_finite_spec(),
        ensures
            #[trigger] f64_add_spec(a, b).is_finite_spec();

    pub broadcast axiom fn axiom_f64_add_monotone_left(a: f64, b: f64, c: f64)
        requires a.is_finite_spec(), b.is_finite_spec(), c.is_finite_spec(),
                 le_ensures::<f64>(a, b, true),
                 f64_add_spec(a, c).is_finite_spec(),
                 f64_add_spec(b, c).is_finite_spec(),
        ensures
            #![trigger le_ensures::<f64>(a, b, true), f64_add_spec(a, c), f64_add_spec(b, c)]
            le_ensures::<f64>(f64_add_spec(a, c), f64_add_spec(b, c), true);

    // Single broadcast group for both float types.
    pub broadcast group group_float_finite_total_order {
        axiom_f64_le_functional,
        axiom_f64_reflexive,
        axiom_f64_antisymmetric,
        axiom_f64_transitive,
        axiom_f64_totality,
        axiom_f32_le_functional,
        axiom_f32_reflexive,
        axiom_f32_antisymmetric,
        axiom_f32_transitive,
        axiom_f32_totality,
    }

    pub broadcast group group_float_arithmetic {
        axiom_f64_zero_is_finite,
        axiom_f64_unreachable_not_finite,
        axiom_f64_add_zero_right,
        axiom_f64_add_commutative,
        axiom_f64_add_finite_preserves,
        axiom_f64_add_monotone_left,
    }

    } // verus!

    impl std::fmt::Debug for WrappedF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "WrappedF64({})", self.val)
        }
    }

    impl std::fmt::Display for WrappedF64 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }

    impl PartialEq for WrappedF64 {
        fn eq(&self, other: &Self) -> bool { self.val == other.val }
    }

    impl Eq for WrappedF64 {}

    impl std::hash::Hash for WrappedF64 {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.val.to_bits().hash(state);
        }
    }

    impl PartialOrd for WrappedF64 {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.val.partial_cmp(&other.val)
        }
    }

    impl Ord for WrappedF64 {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.val.partial_cmp(&other.val).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    impl core::ops::Add for WrappedF64 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            WrappedF64 { val: self.val + rhs.val }
        }
    }

    impl core::ops::AddAssign for WrappedF64 {
        fn add_assign(&mut self, rhs: Self) {
            self.val += rhs.val;
        }
    }
} // mod
