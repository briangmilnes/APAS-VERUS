// Copyright (c) 2025 Brian G. Milnes
//! Checked natural (unsigned) integer types that track overflow.
//! This version has a view that includes overflow state, not just the int value.
//! Stronger guarantees than signed: if final sum fits, all partial sums fit.

#[cfg(verus_keep_ghost)]
pub mod checked_nat_with_checked_view {

    #[allow(unused_imports)]
    use vstd::prelude::*;
    #[allow(unused_imports)]
    use vstd::view::View;
    #[allow(unused_imports)]
    #[cfg(verus_keep_ghost)]
    use vstd::arithmetic::mul::{lemma_mul_by_zero_is_zero, lemma_mul_inequality, lemma_mul_is_commutative};

verus! {

        // Spec functions for folding operations on nat
    pub open spec fn spec_add(a: nat, b: nat) -> nat { a + b }
    pub open spec fn spec_mul(a: nat, b: nat) -> nat { a * b }

        // Spec predicates for algebraic properties (compatible with Set.fold's is_fun_commutative)
    pub open spec fn spec_is_add_commutative() -> bool {
        forall |a1: nat, a2: nat, b: nat| #[trigger] spec_add(spec_add(b, a2), a1) == spec_add(spec_add(b, a1), a2)
    }

    pub open spec fn spec_is_mul_commutative() -> bool {
        forall |a1: nat, a2: nat, b: nat| #[trigger] spec_mul(spec_mul(b, a2), a1) == spec_mul(spec_mul(b, a1), a2)
    }

    pub open spec fn spec_is_add_associative() -> bool {
        forall |a: nat, b: nat, c: nat| #[trigger] spec_add(spec_add(a, b), c) == spec_add(a, spec_add(b, c))
    }

    pub open spec fn spec_is_mul_associative() -> bool {
        forall |a: nat, b: nat, c: nat| #[trigger] spec_mul(spec_mul(a, b), c) == spec_mul(a, spec_mul(b, c))
    }

    pub open spec fn spec_mul_distributes_over_add() -> bool {
        forall |a: nat, b: nat, c: nat| #[trigger] spec_mul(a, spec_add(b, c)) == spec_add(spec_mul(a, b), spec_mul(a, c))
    }

        // Proofs that these properties hold





        /// - Trait for checked natural (unsigned) integer types with BoundedNat view.
        /// - Stronger guarantees: only overflow possible (no underflow for addition).
        /// - Key property: if final sum <= MAX, all partial sums <= MAX (monotonic).
    pub trait CheckedNatWithCheckedViewTrait: Sized + Clone {
        type ViewType;
        
        spec fn spec_max() -> nat;
        spec fn spec_view(&self) -> Self::ViewType;
        spec fn spec_value(&self) -> int;
        spec fn spec_is_normal(&self) -> bool;
        spec fn spec_is_overflow(&self) -> bool;

        fn is_normal(&self) -> (normal: bool)
            ensures normal == self.spec_is_normal();

        fn is_overflow(&self) -> (overflow: bool)
            ensures overflow == self.spec_is_overflow();

        fn add_checked(&self, other: &Self) -> (sum: Self)
            ensures sum.spec_value() == self.spec_value() + other.spec_value();

        fn mul_checked(&self, other: &Self) -> (product: Self)
            ensures product.spec_value() == self.spec_value() * other.spec_value();

        // Commutativity proofs (on ghost int)

        // Distributivity

    impl BoundedNat<int> {
        pub open spec fn value(self) -> int {
            match self {
                BoundedNat::Normal(v) => v,
                BoundedNat::Overflow(v) => v,
            }
        }

        pub open spec fn is_normal(self) -> bool {
            match self {
                BoundedNat::Normal(_) => true,
                BoundedNat::Overflow(_) => false,
            }
        }

        pub open spec fn is_overflow(self) -> bool {
            match self {
                BoundedNat::Normal(_) => false,
                BoundedNat::Overflow(_) => true,
            }
        }
    }

 } // verus!

// Hand coded for U32 to start.
verus! {

    pub ghost struct CheckedU32View { 
        pub i: int,
        pub state: BoundedNat<u32>
    }

    pub struct CheckedU32 { n: BoundedNat<u32> }

    impl View for CheckedU32 {
        type V = CheckedU32View;

        closed spec fn view(&self) -> CheckedU32View { 
            CheckedU32View {
                i: self.i@,
                state: if self.i@ <= (u32::MAX as int) {
                    BoundedNat::Normal(self.i@)
                } else {
                    BoundedNat::Overflow(self.i@)
                },
            }
        }
    }

/*

    impl Clone for CheckedU32 {
        exec fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
        {
            Self { i: self.n }
        }
    }


    impl $nat_type {
        #[verifier::type_invariant]
        spec fn well_formed(self) -> bool {
            match self.v {
                Some(v) => self.i@ == v as int,
                None => self.i@ > ($max as int),
            }
        }

        pub open spec fn spec_max() -> nat { $max as nat }

        pub closed spec fn spec_new(v: $concUtype) -> $nat_type { $nat_type { i: Ghost(v as int), v: Some(v) } }

        #[verifier::when_used_as_spec(spec_new)]
        pub exec fn new(v: $concUtype) -> (checked: Self)
        ensures checked@.value() == v as int, checked@.is_normal()
        {
            Self { i: Ghost(v as int), v: Some(v) }
        }

        pub exec fn new_overflow(Ghost(i): Ghost<int>) -> (checked: Self)
        requires i > ($max as int)
        ensures checked@.value() == i, checked@.is_overflow()
        {
            Self { i: Ghost(i), v: None }
        }

        pub open spec fn spec_is_normal(&self) -> bool { self@.is_normal() }

        #[verifier::when_used_as_spec(spec_is_normal)]
        pub exec fn is_normal(&self) -> (normal: bool)
        ensures normal == self@.is_normal()
        {
            proof { use_type_invariant(self); }
            self.v.is_some()
        }

        pub open spec fn spec_is_overflow(&self) -> bool { self@.is_overflow() }

        #[verifier::when_used_as_spec(spec_is_overflow)]
        pub exec fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == self@.is_overflow()
        {
            proof { use_type_invariant(self); }
            self.v.is_none()
        }

        pub exec fn unwrap(&self) -> (value: $concUtype)
        requires self@.is_normal()
        ensures value as int == self@.value()
        {
            proof { use_type_invariant(self); }
            self.v.unwrap()
        }

        pub exec fn to_option(&self) -> (option: Option<$concUtype>)
        ensures
            option.is_some() == self@.is_normal(),
            option.is_some() ==> option.unwrap() as int == self@.value()
        {
            proof { use_type_invariant(self); }
            self.v
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_value(&self, v2: $concUtype) -> (sum: Self)
        ensures sum@.value() == self@.value() + v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@.value() + v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_add(v2) },
                None => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_checked(&self, v2: &$nat_type) -> (sum: Self)
        ensures sum@.value() == self@.value() + v2@.value()
        {
            let new_i: Ghost<int> = Ghost(self@.value() + v2@.value());
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                _ => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_value(&self, v2: $concUtype) -> (product: Self)
        ensures product@.value() == self@.value() * v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@.value() * v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_mul(v2) },
                None => { if v2 == 0 { Self { i: new_i, v: Some(0) } } else { Self { i: new_i, v: None } }
                }
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_checked(&self, v2: &$nat_type) -> (product: Self)
        ensures product@.value() == self@.value() * v2@.value()
        {
            let new_i: Ghost<int> = Ghost(self@.value() * v2@.value());
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                _ => Self { i: new_i, v: None },
            }
        }

                    // Ghost commutativity
        pub proof fn lemma_add_commutative_ghost(a: $nat_type, b: $nat_type)
            ensures a@.value() + b@.value() == b@.value() + a@.value()
        {}

        pub proof fn lemma_mul_commutative_ghost(a: $nat_type, b: $nat_type)
            ensures a@.value() * b@.value() == b@.value() * a@.value()
        {}

                    // Normal commutativity (if result fits)
        pub proof fn lemma_add_commutative_normal(a: $nat_type, b: $nat_type)
            requires 0 <= a@.value() + b@.value() <= ($max as int)
            ensures (a@.value() + b@.value()) as $concUtype == (b@.value() + a@.value()) as $concUtype
        {}

        pub proof fn lemma_mul_commutative_normal(a: $nat_type, b: $nat_type)
            requires 0 <= a@.value() * b@.value() <= ($max as int)
            ensures (a@.value() * b@.value()) as $concUtype == (b@.value() * a@.value()) as $concUtype
        {}

                    // Associativity
        pub proof fn lemma_add_associative_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures (a@.value() + b@.value()) + c@.value() == a@.value() + (b@.value() + c@.value())
        {}

        pub proof fn lemma_mul_associative_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures (a@.value() * b@.value()) * c@.value() == a@.value() * (b@.value() * c@.value())
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@.value(), b@.value(), c@.value()); }

        pub proof fn lemma_add_associative_normal(a: $nat_type, b: $nat_type, c: $nat_type)
            requires 0 <= a@.value() + b@.value() + c@.value() <= ($max as int)
            ensures ((a@.value() + b@.value()) + c@.value()) as $concUtype == (a@.value() + (b@.value() + c@.value())) as $concUtype
        {}

        pub proof fn lemma_mul_associative_normal(a: $nat_type, b: $nat_type, c: $nat_type)
            requires
                0 <= a@.value() * b@.value() <= ($max as int),
                0 <= b@.value() * c@.value() <= ($max as int),
                0 <= (a@.value() * b@.value()) * c@.value() <= ($max as int),
            ensures ((a@.value() * b@.value()) * c@.value()) as $concUtype == (a@.value() * (b@.value() * c@.value())) as $concUtype
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@.value(), b@.value(), c@.value()); }

                    // Distributivity
        pub proof fn lemma_mul_distributes_over_add_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures a@.value() * (b@.value() + c@.value()) == a@.value() * b@.value() + a@.value() * c@.value()
        { vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@.value(), b@.value(), c@.value()); }

                    // STRONGER PROOFS FOR UNSIGNED

                    /// Monotonicity: adding non-negatives increases the sum
        pub proof fn lemma_sum_monotonic(a: $nat_type, b: $nat_type)
            requires
                a@.value() >= 0,
                b@.value() >= 0,
            ensures
                a@.value() <= a@.value() + b@.value(),
                b@.value() <= a@.value() + b@.value(),
        {}

                    /// Key lemma: for non-negative values, if final sum fits, the sum is in range
        pub proof fn lemma_add_normal_if_sum_fits(a: $nat_type, b: $nat_type)
            requires
                a@.value() >= 0,
                b@.value() >= 0,
                a@.value() + b@.value() <= ($max as int),
            ensures 0 <= a@.value() + b@.value() <= ($max as int),
        {}

                    /// Partial sums bounded: if total sum of non-negatives fits, any partial sum fits
        pub proof fn lemma_partial_sum_bounded(total: int, partial: int)
            requires
                0 <= partial <= total,
                total <= ($max as int),
            ensures partial <= ($max as int),
        {}
    }
*/
 } // verus
}

/*
macro_rules! checked_nat_gen {
        ($nat_type:ty, $exec_type:ident, $view_type:ident, $max:expr) => {
verus! {

    pub struct ghost $view_type { 
        pub i: int,
        pub state: BoundedNat<$nat_type>,
    }

    pub struct $exec_type { n: BoundedNat<$nat_type> }

    impl $NatTypeView {
        pub open spec fn is_normal(self)   -> bool { self.state.is_normal() }
        pub open spec fn is_overflow(self) -> bool { self.state.is_overflow() }
        pub open spec fn value(self)       -> int  { self.i }
    }


    impl View for $exec_type {
        type V = $exec_type

        closed spec fn view(&self) -> $nat_typeView { 
            $nat_typeView {
                i: self.i@,
                state: if self.i@ <= ($max as int) {
                    BoundedNat::Normal(self.i@)
                } else {
                    BoundedNat::Overflow(self.i@)
                },
            }
        }
    }

    impl Clone for $nat_type {
        exec fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
        {
            proof { use_type_invariant(self); }
            Self { i: self.i, v: self.v }
        }
    }

    impl $nat_type {
        #[verifier::type_invariant]
        spec fn well_formed(self) -> bool {
            match self.v {
                Some(v) => self.i@ == v as int,
                None => self.i@ > ($max as int),
            }
        }

        pub open spec fn spec_max() -> nat { $max as nat }

        pub closed spec fn spec_new(v: $concUtype) -> $nat_type { $nat_type { i: Ghost(v as int), v: Some(v) } }

        #[verifier::when_used_as_spec(spec_new)]
        pub exec fn new(v: $concUtype) -> (checked: Self)
        ensures checked@.value() == v as int, checked@.is_normal()
        {
            Self { i: Ghost(v as int), v: Some(v) }
        }

        pub exec fn new_overflow(Ghost(i): Ghost<int>) -> (checked: Self)
        requires i > ($max as int)
        ensures checked@.value() == i, checked@.is_overflow()
        {
            Self { i: Ghost(i), v: None }
        }

        pub open spec fn spec_is_normal(&self) -> bool { self@.is_normal() }

        #[verifier::when_used_as_spec(spec_is_normal)]
        pub exec fn is_normal(&self) -> (normal: bool)
        ensures normal == self@.is_normal()
        {
            proof { use_type_invariant(self); }
            self.v.is_some()
        }

        pub open spec fn spec_is_overflow(&self) -> bool { self@.is_overflow() }

        #[verifier::when_used_as_spec(spec_is_overflow)]
        pub exec fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == self@.is_overflow()
        {
            proof { use_type_invariant(self); }
            self.v.is_none()
        }

        pub exec fn unwrap(&self) -> (value: $concUtype)
        requires self@.is_normal()
        ensures value as int == self@.value()
        {
            proof { use_type_invariant(self); }
            self.v.unwrap()
        }

        pub exec fn to_option(&self) -> (option: Option<$concUtype>)
        ensures
            option.is_some() == self@.is_normal(),
            option.is_some() ==> option.unwrap() as int == self@.value()
        {
            proof { use_type_invariant(self); }
            self.v
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_value(&self, v2: $concUtype) -> (sum: Self)
        ensures sum@.value() == self@.value() + v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@.value() + v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_add(v2) },
                None => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_checked(&self, v2: &$nat_type) -> (sum: Self)
        ensures sum@.value() == self@.value() + v2@.value()
        {
            let new_i: Ghost<int> = Ghost(self@.value() + v2@.value());
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                _ => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_value(&self, v2: $concUtype) -> (product: Self)
        ensures product@.value() == self@.value() * v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@.value() * v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_mul(v2) },
                None => { if v2 == 0 { Self { i: new_i, v: Some(0) } } else { Self { i: new_i, v: None } }
                }
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_checked(&self, v2: &$nat_type) -> (product: Self)
        ensures product@.value() == self@.value() * v2@.value()
        {
            let new_i: Ghost<int> = Ghost(self@.value() * v2@.value());
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                _ => Self { i: new_i, v: None },
            }
        }

                    // Ghost commutativity
        pub proof fn lemma_add_commutative_ghost(a: $nat_type, b: $nat_type)
            ensures a@.value() + b@.value() == b@.value() + a@.value()
        {}

        pub proof fn lemma_mul_commutative_ghost(a: $nat_type, b: $nat_type)
            ensures a@.value() * b@.value() == b@.value() * a@.value()
        {}

                    // Normal commutativity (if result fits)
        pub proof fn lemma_add_commutative_normal(a: $nat_type, b: $nat_type)
            requires 0 <= a@.value() + b@.value() <= ($max as int)
            ensures (a@.value() + b@.value()) as $concUtype == (b@.value() + a@.value()) as $concUtype
        {}

        pub proof fn lemma_mul_commutative_normal(a: $nat_type, b: $nat_type)
            requires 0 <= a@.value() * b@.value() <= ($max as int)
            ensures (a@.value() * b@.value()) as $concUtype == (b@.value() * a@.value()) as $concUtype
        {}

                    // Associativity
        pub proof fn lemma_add_associative_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures (a@.value() + b@.value()) + c@.value() == a@.value() + (b@.value() + c@.value())
        {}

        pub proof fn lemma_mul_associative_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures (a@.value() * b@.value()) * c@.value() == a@.value() * (b@.value() * c@.value())
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@.value(), b@.value(), c@.value()); }

        pub proof fn lemma_add_associative_normal(a: $nat_type, b: $nat_type, c: $nat_type)
            requires 0 <= a@.value() + b@.value() + c@.value() <= ($max as int)
            ensures ((a@.value() + b@.value()) + c@.value()) as $concUtype == (a@.value() + (b@.value() + c@.value())) as $concUtype
        {}

        pub proof fn lemma_mul_associative_normal(a: $nat_type, b: $nat_type, c: $nat_type)
            requires
                0 <= a@.value() * b@.value() <= ($max as int),
                0 <= b@.value() * c@.value() <= ($max as int),
                0 <= (a@.value() * b@.value()) * c@.value() <= ($max as int),
            ensures ((a@.value() * b@.value()) * c@.value()) as $concUtype == (a@.value() * (b@.value() * c@.value())) as $concUtype
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@.value(), b@.value(), c@.value()); }

                    // Distributivity
        pub proof fn lemma_mul_distributes_over_add_ghost(a: $nat_type, b: $nat_type, c: $nat_type)
            ensures a@.value() * (b@.value() + c@.value()) == a@.value() * b@.value() + a@.value() * c@.value()
        { vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@.value(), b@.value(), c@.value()); }

                    // STRONGER PROOFS FOR UNSIGNED

                    /// Monotonicity: adding non-negatives increases the sum
        pub proof fn lemma_sum_monotonic(a: $nat_type, b: $nat_type)
            requires
                a@.value() >= 0,
                b@.value() >= 0,
            ensures
                a@.value() <= a@.value() + b@.value(),
                b@.value() <= a@.value() + b@.value(),
        {}

                    /// Key lemma: for non-negative values, if final sum fits, the sum is in range
        pub proof fn lemma_add_normal_if_sum_fits(a: $nat_type, b: $nat_type)
            requires
                a@.value() >= 0,
                b@.value() >= 0,
                a@.value() + b@.value() <= ($max as int),
            ensures 0 <= a@.value() + b@.value() <= ($max as int),
        {}

                    /// Partial sums bounded: if total sum of non-negatives fits, any partial sum fits
        pub proof fn lemma_partial_sum_bounded(total: int, partial: int)
            requires
                0 <= partial <= total,
                total <= ($max as int),
            ensures partial <= ($max as int),
        {}
    }
}
};
}

checked_nat_gen!(u32, CheckedU32WithCheckedView, CheckedU32WithCheckedViewView, u32::MAX);

}

*/
