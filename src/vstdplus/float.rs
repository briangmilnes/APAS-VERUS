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

    } // verus!
} // mod
