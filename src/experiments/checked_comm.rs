//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Commutativity lemmas for checked arithmetic.

pub mod checked_comm {

use vstd::prelude::*;

/// Macro to generate commutativity proofs for any Checked* type
macro_rules! checked_comm_proofs {
    ($cty:ty, $ity:ty, $min:expr, $max:expr) => {
        verus! {
            pub mod proofs {
                use super::*;
                use crate::vstdplus::checked_int::checked_int::*;

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

// Generate proofs for unsigned types
pub mod u8_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedU8, u8, 0int, u8::MAX as int);
}

pub mod u16_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedU16, u16, 0int, u16::MAX as int);
}

pub mod u32_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedU32, u32, 0int, u32::MAX as int);
}

pub mod u64_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedU64, u64, 0int, u64::MAX as int);
}

pub mod u128_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedU128, u128, 0int, u128::MAX as int);
}

pub mod usize_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedUsize, usize, 0int, usize::MAX as int);
}

// Generate proofs for signed types
pub mod i8_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedI8, i8, i8::MIN as int, i8::MAX as int);
}

pub mod i16_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedI16, i16, i16::MIN as int, i16::MAX as int);
}

pub mod i32_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedI32, i32, i32::MIN as int, i32::MAX as int);
}

pub mod i64_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedI64, i64, i64::MIN as int, i64::MAX as int);
}

pub mod i128_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedI128, i128, i128::MIN as int, i128::MAX as int);
}

pub mod isize_comm {
    use super::*;
    checked_comm_proofs!(crate::vstdplus::checked_int::checked_int::CheckedIsize, isize, isize::MIN as int, isize::MAX as int);
}

}
