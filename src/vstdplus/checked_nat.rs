//! Checked natural (unsigned) integer types that track overflow.
//! Stronger guarantees than signed: if final sum fits, all partial sums fit.

#[cfg(verus_keep_ghost)]
pub mod checked_nat {

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
    pub proof fn lemma_spec_add_commutative()
        ensures spec_is_add_commutative()
    {}

    pub proof fn lemma_spec_mul_commutative()
        ensures spec_is_mul_commutative()
    {
        assert forall |a1: nat, a2: nat, b: nat| #[trigger] spec_mul(spec_mul(b, a2), a1) == spec_mul(spec_mul(b, a1), a2) by {
            lemma_mul_is_commutative(b as int, a2 as int);
            lemma_mul_is_commutative(b as int, a1 as int);
            lemma_mul_is_commutative((b * a2) as int, a1 as int);
            lemma_mul_is_commutative((b * a1) as int, a2 as int);
            vstd::arithmetic::mul::lemma_mul_is_associative(b as int, a2 as int, a1 as int);
            vstd::arithmetic::mul::lemma_mul_is_associative(b as int, a1 as int, a2 as int);
        }
    }

    pub proof fn lemma_spec_add_associative()
        ensures spec_is_add_associative()
    {}

    pub proof fn lemma_spec_mul_associative()
        ensures spec_is_mul_associative()
    {
        assert forall |a: nat, b: nat, c: nat| #[trigger] spec_mul(spec_mul(a, b), c) == spec_mul(a, spec_mul(b, c)) by {
            vstd::arithmetic::mul::lemma_mul_is_associative(a as int, b as int, c as int);
        }
    }

    pub proof fn lemma_spec_mul_distributes_over_add()
        ensures spec_mul_distributes_over_add()
    {
        assert forall |a: nat, b: nat, c: nat| #[trigger] spec_mul(a, spec_add(b, c)) == spec_add(spec_mul(a, b), spec_mul(a, c)) by {
            vstd::arithmetic::mul::lemma_mul_is_distributive_add(a as int, b as int, c as int);
        }
    }

        /// Trait for checked natural (unsigned) integer types.
        /// Stronger guarantees: only overflow possible (no underflow for addition).
        /// Key property: if final sum <= MAX, all partial sums <= MAX (monotonic).
    pub trait CheckedNatTrait: View<V = int> + Sized + Clone {
        spec fn spec_max()                  -> nat;

        fn is_normal(&self)                 -> (normal: bool)
            ensures normal == (0 <= self@ <= Self::spec_max() as int);

        fn is_overflow(&self)               -> (overflow: bool)
            ensures overflow == (self@ > Self::spec_max() as int);

        fn add_checked(&self, other: &Self) -> (sum: Self)
            ensures sum@ == self@ + other@;

        fn mul_checked(&self, other: &Self) -> (product: Self)
            ensures product@ == self@ * other@;
            // Commutativity proofs (on ghost int)

        proof fn lemma_add_commutative_ghost(a: Self, b: Self)
            ensures a@ + b@ == b@ + a@;

        proof fn lemma_mul_commutative_ghost(a: Self, b: Self)
            ensures a@ * b@ == b@ * a@;

            // Associativity proofs

        proof fn lemma_add_associative_ghost(a: Self, b: Self, c: Self)
            ensures (a@ + b@) + c@ == a@ + (b@ + c@);

        proof fn lemma_mul_associative_ghost(a: Self, b: Self, c: Self)
            ensures (a@ * b@) * c@ == a@ * (b@ * c@)
            { vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@); }

            // Distributivity
        proof fn lemma_mul_distributes_over_add_ghost(a: Self, b: Self, c: Self)
            ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@;

            // STRONGER GUARANTEE for unsigned: monotonic sums
            // If all values are non-negative and final sum fits, no intermediate overflow

        proof fn lemma_sum_monotonic(a: Self, b: Self)
            requires
                a@ >= 0,
                b@ >= 0,
            ensures
                a@ <= a@ + b@,
                b@ <= a@ + b@;

            // Key lemma: if final sum of non-negatives fits, the sum is in range
        proof fn lemma_add_normal_if_sum_fits(a: Self, b: Self)
            requires
                a@ >= 0,
                b@ >= 0,
                a@ + b@ <= Self::spec_max() as int,
            ensures 0 <= a@ + b@ <= Self::spec_max() as int;
    }

 } // verus!

macro_rules! checked_nat_gen {
        ($uty:ty, $cty:ident, $max:expr) => {
verus! {

    pub struct $cty { i: Ghost<int>, v: Option<$uty> }

    impl View for $cty {
        type V = int;

        closed spec fn view(&self) -> int { self.i@ }
    }

    impl Clone for $cty {
        exec fn clone(&self) -> (clone: Self)
        ensures clone@ == self@
        {
            proof { use_type_invariant(self); }
            Self { i: self.i, v: self.v }
        }
    }

    impl $cty {
        #[verifier::type_invariant]
        spec fn well_formed(self) -> bool {
            match self.v {
                Some(v) => self.i@ == v as int,
                None => self.i@ > ($max as int),
            }
        }

        pub open spec fn spec_max() -> nat { $max as nat }

        pub closed spec fn spec_new(v: $uty) -> $cty { $cty { i: Ghost(v as int), v: Some(v) } }

        #[verifier::when_used_as_spec(spec_new)]
        pub exec fn new(v: $uty) -> (checked: Self)
        ensures checked@ == v as int
        {
            Self { i: Ghost(v as int), v: Some(v) }
        }

        pub exec fn new_overflow(Ghost(i): Ghost<int>) -> (checked: Self)
        requires i > ($max as int)
        ensures checked@ == i
        {
            Self { i: Ghost(i), v: None }
        }

        pub open spec fn spec_is_normal(&self) -> bool { 0 <= self@ <= ($max as int) }

        #[verifier::when_used_as_spec(spec_is_normal)]
        pub exec fn is_normal(&self) -> (normal: bool)
        ensures normal == self.spec_is_normal()
        {
            proof { use_type_invariant(self); }
            self.v.is_some()
        }

        pub open spec fn spec_is_overflow(&self) -> bool { self@ > ($max as int) }

        #[verifier::when_used_as_spec(spec_is_overflow)]
        pub exec fn is_overflow(&self) -> (overflow: bool)
        ensures overflow == self.spec_is_overflow()
        {
            proof { use_type_invariant(self); }
            self.v.is_none()
        }

        pub exec fn unwrap(&self) -> (value: $uty)
        requires self.is_normal()
        ensures value as int == self@
        {
            proof { use_type_invariant(self); }
            self.v.unwrap()
        }

        pub exec fn to_option(&self) -> (option: Option<$uty>)
        ensures
            option.is_some() == self.is_normal(),
            option.is_some() ==> option.unwrap() as int == self@
        {
            proof { use_type_invariant(self); }
            self.v
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_value(&self, v2: $uty) -> (sum: Self)
        ensures sum@ == self@ + v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@ + v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_add(v2) },
                None => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn add_checked(&self, v2: &$cty) -> (sum: Self)
        ensures sum@ == self@ + v2@
        {
            let new_i: Ghost<int> = Ghost(self@ + v2@);
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                _ => Self { i: new_i, v: None },
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_value(&self, v2: $uty) -> (product: Self)
        ensures product@ == self@ * v2 as int
        {
            let new_i: Ghost<int> = Ghost(self@ * v2 as int);
            match self.v {
                Some(v1) => Self { i: new_i, v: v1.checked_mul(v2) },
                None => { if v2 == 0 { Self { i: new_i, v: Some(0) } } else { Self { i: new_i, v: None } }
                }
            }
        }

        #[inline]
        #[verifier::external_body]
        pub exec fn mul_checked(&self, v2: &$cty) -> (product: Self)
        ensures product@ == self@ * v2@
        {
            let new_i: Ghost<int> = Ghost(self@ * v2@);
            match (&self.v, &v2.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                _ => Self { i: new_i, v: None },
            }
        }

                    // Ghost commutativity
        pub proof fn lemma_add_commutative_ghost(a: $cty, b: $cty)
            ensures a@ + b@ == b@ + a@
        {}

        pub proof fn lemma_mul_commutative_ghost(a: $cty, b: $cty)
            ensures a@ * b@ == b@ * a@
        {}

                    // Normal commutativity (if result fits)
        pub proof fn lemma_add_commutative_normal(a: $cty, b: $cty)
            requires 0 <= a@ + b@ <= ($max as int)
            ensures (a@ + b@) as $uty == (b@ + a@) as $uty
        {}

        pub proof fn lemma_mul_commutative_normal(a: $cty, b: $cty)
            requires 0 <= a@ * b@ <= ($max as int)
            ensures (a@ * b@) as $uty == (b@ * a@) as $uty
        {}

                    // Associativity
        pub proof fn lemma_add_associative_ghost(a: $cty, b: $cty, c: $cty)
            ensures (a@ + b@) + c@ == a@ + (b@ + c@)
        {}

        pub proof fn lemma_mul_associative_ghost(a: $cty, b: $cty, c: $cty)
            ensures (a@ * b@) * c@ == a@ * (b@ * c@)
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@); }

        pub proof fn lemma_add_associative_normal(a: $cty, b: $cty, c: $cty)
            requires 0 <= a@ + b@ + c@ <= ($max as int)
            ensures ((a@ + b@) + c@) as $uty == (a@ + (b@ + c@)) as $uty
        {}

        pub proof fn lemma_mul_associative_normal(a: $cty, b: $cty, c: $cty)
            requires
                0 <= a@ * b@ <= ($max as int),
                0 <= b@ * c@ <= ($max as int),
                0 <= (a@ * b@) * c@ <= ($max as int),
            ensures ((a@ * b@) * c@) as $uty == (a@ * (b@ * c@)) as $uty
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@); }

                    // Distributivity
        pub proof fn lemma_mul_distributes_over_add_ghost(a: $cty, b: $cty, c: $cty)
            ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@
        { vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@, b@, c@); }

                    // STRONGER PROOFS FOR UNSIGNED

                    /// Monotonicity: adding non-negatives increases the sum
        pub proof fn lemma_sum_monotonic(a: $cty, b: $cty)
            requires
                a@ >= 0,
                b@ >= 0,
            ensures
                a@ <= a@ + b@,
                b@ <= a@ + b@,
        {}

                    /// Key lemma: for non-negative values, if final sum fits, the sum is in range
        pub proof fn lemma_add_normal_if_sum_fits(a: $cty, b: $cty)
            requires
                a@ >= 0,
                b@ >= 0,
                a@ + b@ <= ($max as int),
            ensures 0 <= a@ + b@ <= ($max as int),
        {}

                    /// Partial sums bounded: if total sum of non-negatives fits, any partial sum fits
        pub proof fn lemma_partial_sum_bounded(total: int, partial: int)
            requires
                0 <= partial <= total,
                total <= ($max as int),
            ensures partial <= ($max as int),
        {}
    }

    impl CheckedNatTrait for $cty {
        open spec fn spec_max() -> nat { $max as nat }

        fn is_normal(&self) -> (normal: bool)
            ensures normal == (0 <= self@ <= Self::spec_max() as int)
        {
            proof { use_type_invariant(self); }
            self.v.is_some()
        }

        fn is_overflow(&self) -> (overflow: bool)
            ensures overflow == (self@ > Self::spec_max() as int)
        {
            proof { use_type_invariant(self); }
            self.v.is_none()
        }

        #[verifier::external_body]
        fn add_checked(&self, other: &Self) -> (sum: Self)
            ensures sum@ == self@ + other@
        {
            let new_i: Ghost<int> = Ghost(self@ + other@);
            match (&self.v, &other.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_add(*n2) },
                _ => Self { i: new_i, v: None },
            }
        }

        #[verifier::external_body]
        fn mul_checked(&self, other: &Self) -> (product: Self)
            ensures product@ == self@ * other@
        {
            let new_i: Ghost<int> = Ghost(self@ * other@);
            match (&self.v, &other.v) {
                (Some(n1), Some(n2)) => Self { i: new_i, v: n1.checked_mul(*n2) },
                (Some(n1), None) if *n1 == 0 => Self { i: new_i, v: Some(0) },
                (None, Some(n2)) if *n2 == 0 => Self { i: new_i, v: Some(0) },
                _ => Self { i: new_i, v: None },
            }
        }

        proof fn lemma_add_commutative_ghost(a: Self, b: Self)
            ensures a@ + b@ == b@ + a@
        {}

        proof fn lemma_mul_commutative_ghost(a: Self, b: Self)
            ensures a@ * b@ == b@ * a@
        {}

        proof fn lemma_add_associative_ghost(a: Self, b: Self, c: Self)
            ensures (a@ + b@) + c@ == a@ + (b@ + c@)
        {}

        proof fn lemma_mul_associative_ghost(a: Self, b: Self, c: Self)
            ensures (a@ * b@) * c@ == a@ * (b@ * c@)
        { vstd::arithmetic::mul::lemma_mul_is_associative(a@, b@, c@); }

        proof fn lemma_mul_distributes_over_add_ghost(a: Self, b: Self, c: Self)
            ensures a@ * (b@ + c@) == a@ * b@ + a@ * c@
        { vstd::arithmetic::mul::lemma_mul_is_distributive_add(a@, b@, c@); }

        proof fn lemma_sum_monotonic(a: Self, b: Self)
        {}

        proof fn lemma_add_normal_if_sum_fits(a: Self, b: Self)
        {}
    }
}
};
}

checked_nat_gen!(u8, CheckedU8, u8::MAX);
checked_nat_gen!(u16, CheckedU16, u16::MAX);
checked_nat_gen!(u32, CheckedU32, u32::MAX);
checked_nat_gen!(u64, CheckedU64, u64::MAX);
checked_nat_gen!(u128, CheckedU128, u128::MAX);
checked_nat_gen!(usize, CheckedUsize, usize::MAX);

}

/// Non-Verus version for cargo test
#[cfg(not(verus_keep_ghost))]
pub mod checked_nat {

    macro_rules! checked_nat_gen {
        ($uty:ty, $cty:ident, $max:expr) => {
            #[derive(Clone)]
            pub struct $cty { v: Option<$uty> }

            impl $cty {
                pub fn new(v: $uty) -> Self { Self { v: Some(v) } }

                pub fn is_normal(&self) -> bool { self.v.is_some() }

                pub fn is_overflow(&self) -> bool { self.v.is_none() }

                pub fn unwrap(&self) -> $uty { self.v.unwrap() }

                pub fn to_option(&self) -> Option<$uty> { self.v }

                pub fn add_value(&self, v2: $uty) -> Self {
                    match self.v {
                        Some(v1) => Self { v: v1.checked_add(v2) },
                        None => Self { v: None },
                    }
                }

                pub fn add_checked(&self, v2: &$cty) -> Self {
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { v: n1.checked_add(*n2) },
                        _ => Self { v: None },
                    }
                }

                pub fn mul_value(&self, v2: $uty) -> Self {
                    match self.v {
                        Some(v1) => Self { v: v1.checked_mul(v2) },
                        None => { if v2 == 0 { Self { v: Some(0) } } else { Self { v: None } }
                        }
                    }
                }

                pub fn mul_checked(&self, v2: &$cty) -> Self {
                    match (&self.v, &v2.v) {
                        (Some(n1), Some(n2)) => Self { v: n1.checked_mul(*n2) },
                        (Some(n1), None) if *n1 == 0 => Self { v: Some(0) },
                        (None, Some(n2)) if *n2 == 0 => Self { v: Some(0) },
                        _ => Self { v: None },
                    }
                }
            }
        };
    }

    checked_nat_gen!(u8,    CheckedU8,       u8::MAX);
    checked_nat_gen!(u16,   CheckedU16,     u16::MAX);
    checked_nat_gen!(u32,   CheckedU32,     u32::MAX);
    checked_nat_gen!(u64,   CheckedU64,     u64::MAX);
    checked_nat_gen!(u128,  CheckedU128,   u128::MAX);
    checked_nat_gen!(usize, CheckedUsize, usize::MAX);

}
