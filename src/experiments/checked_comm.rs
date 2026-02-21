//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Hypothesis: Can we prove commutativity lemmas for checked arithmetic?
//! NOTE: These proofs are now in vstdplus/checked_int.rs and vstdplus/checked_nat.rs.
//! This file is kept for reference.

pub mod checked_comm {

use vstd::prelude::*;
use crate::vstdplus::checked_nat::checked_nat::*;
use crate::vstdplus::checked_int::checked_int::*;

/// Macro for signed types (has sub_checked)
macro_rules! checked_int_comm_proofs {
    ($cty:ty, $ity:ty, $min:expr, $max:expr) => {
        verus! {
            pub mod proofs {
                use super::*;

                pub fn plus(a: &$cty, b: &$cty) -> (result: $cty)
                    ensures result@ == a@ + b@
                {
                    a.add_checked(b)
                }
                
                pub proof fn lemma_plus_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ + b@ == b@ + a@
                {}
                
                pub proof fn lemma_plus_commutative_normal(a: $cty, b: $cty)
                    requires $min <= a@ + b@ <= $max
                    ensures (a@ + b@) as $ity == (b@ + a@) as $ity,
                {}

                pub fn mul(a: &$cty, b: &$cty) -> (result: $cty)
                    ensures result@ == a@ * b@
                {
                    a.mul_checked(b)
                }
                
                pub proof fn lemma_mul_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ * b@ == b@ * a@
                {}
                
                pub proof fn lemma_mul_commutative_normal(a: $cty, b: $cty)
                    requires $min <= a@ * b@ <= $max
                    ensures (a@ * b@) as $ity == (b@ * a@) as $ity,
                {}

                pub fn minus(a: &$cty, b: &$cty) -> (result: $cty)
                    ensures result@ == a@ - b@
                {
                    a.sub_checked(b)
                }
                
                pub proof fn lemma_minus_ghost(a: $cty, b: $cty)
                    ensures a@ - b@ == -(b@ - a@)
                {}
                
                pub proof fn lemma_minus_normal(a: $cty, b: $cty)
                    requires $min <= a@ - b@ <= $max
                    ensures (a@ - b@) as $ity == (-(b@ - a@)) as $ity,
                {}
            }
        }
    };
}

/// Macro for unsigned types (no sub_checked - subtraction can underflow)
macro_rules! checked_nat_comm_proofs {
    ($cty:ty, $ity:ty, $max:expr) => {
        verus! {
            pub mod proofs {
                use super::*;

                pub fn plus(a: &$cty, b: &$cty) -> (result: $cty)
                    ensures result@ == a@ + b@
                {
                    a.add_checked(b)
                }
                
                pub proof fn lemma_plus_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ + b@ == b@ + a@
                {}
                
                pub proof fn lemma_plus_commutative_normal(a: $cty, b: $cty)
                    requires 0 <= a@ + b@ <= $max
                    ensures (a@ + b@) as $ity == (b@ + a@) as $ity,
                {}

                pub fn mul(a: &$cty, b: &$cty) -> (result: $cty)
                    ensures result@ == a@ * b@
                {
                    a.mul_checked(b)
                }
                
                pub proof fn lemma_mul_commutative_ghost(a: $cty, b: $cty)
                    ensures a@ * b@ == b@ * a@
                {}
                
                pub proof fn lemma_mul_commutative_normal(a: $cty, b: $cty)
                    requires 0 <= a@ * b@ <= $max
                    ensures (a@ * b@) as $ity == (b@ * a@) as $ity,
                {}
            }
        }
    };
}

// Generate proofs for unsigned types (now in checked_nat)
pub mod u8_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedU8, u8, u8::MAX as int);
}

pub mod u16_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedU16, u16, u16::MAX as int);
}

pub mod u32_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedU32, u32, u32::MAX as int);
}

pub mod u64_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedU64, u64, u64::MAX as int);
}

pub mod u128_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedU128, u128, u128::MAX as int);
}

pub mod usize_comm {
    use super::*;
    checked_nat_comm_proofs!(CheckedUsize, usize, usize::MAX as int);
}

// Generate proofs for signed types (in checked_int)
pub mod i8_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedI8, i8, i8::MIN as int, i8::MAX as int);
}

pub mod i16_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedI16, i16, i16::MIN as int, i16::MAX as int);
}

pub mod i32_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedI32, i32, i32::MIN as int, i32::MAX as int);
}

pub mod i64_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedI64, i64, i64::MIN as int, i64::MAX as int);
}

pub mod i128_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedI128, i128, i128::MIN as int, i128::MAX as int);
}

pub mod isize_comm {
    use super::*;
    checked_int_comm_proofs!(CheckedIsize, isize, isize::MIN as int, isize::MAX as int);
}

}
