//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Checked integer using enum for overflow/underflow state.
//! Once overflow or underflow, stays that way (like NaN).

pub mod checked_int_enum {
/*
use vstd::prelude::*;
use num_traits::ops::checked::CheckedAdd;
use core::ops::{Add, Div, Mul, Rem, Shl, Shr, Sub};    

verus! {

    pub enum CheckedInt<T: Integer> { Normal(T), Underflow, Overflow, }

    pub trait CheckedIntTrait<T: Integer + CheckedSub> {
        // Limits on T.
        spec fn min_val() -> int;
        spec fn max_val() -> int;

        // What does the computation actually yeild?
        spec fn actual(&self) -> int;

        // Operations to match 
        spec fn add(n1:T, n2:T) -> T;
        spec fn sub(n1:T, n2:T) -> T;
        spec fn mul(n1:T, n2:T) -> T;

        spec fn is_normal(&self)    -> bool { matches!(*self, CheckedInt::Normal   (i)) }
        spec fn is_underflow(&self) -> bool { matches!(*self, CheckedInt::Underflow(i)) }
        spec fn is_overflow(&self)  -> bool { matches!(*self, CheckedInt::Overflow (i)) }
    }

    pub ghost enum GhostCheckedInt { Normal(int), Underflow(int), Overflow(int),}


/*

    pub trait GhostCheckedIntStateTrait{
        // Limits on T.
        spec fn min_val() -> int;
        spec fn max_val() -> int;

        // What would the computation yield on ints?
        spec fn intended(&self) -> int;

        spec fn add(n1:T, n2:T) -> T;
        spec fn sub(n1:T, n2:T) -> T;
        spec fn mul(n1:T, n2:T) -> T;

        spec fn is_normal(&self)    -> bool { matches!(*self, GhostCheckedInt::Normal   (i)) }
        spec fn is_underflow(&self) -> bool { matches!(*self, GhostCheckedInt::Underflow(i)) }
        spec fn is_overflow(&self)  -> bool { matches!(*self, GhostCheckedInt::Overflow (i)) }
    }


    pub enum CheckedInt<T: Integer> { Normal(T), Underflow, Overflow, }


    impl GhostCheckedIntState<u32> for u32 {
        spec fn intended(&self) -> int {
        spec fn min_val() -> int;
        spec fn max_val() -> int;
        spec fn add(n1:T, n2:T) -> T;
        spec fn sub(n1:T, n2:T) -> T;
        spec fn mul(n1:T, n2:T) -> T;

        open spec fn is_normal(&self)    -> bool { matches!(*self, GhostCheckedInt::Normal   (_)) }
        open spec fn is_underflow(&self) -> bool { matches!(*self, GhostCheckedInt::Underflow(_)) }
        open spec fn is_overflow(&self)  -> bool { matches!(*self, GhostCheckedInt::Overflow (_)) }
    }


    pub trait CheckedIntTrait<T: Integer>: View<V = GhostCheckedInt> + Sized {
        spec fn min_val() -> int;
        spec fn max_val() -> int;
        
        fn is_normal(&self) -> (result: bool)     ensures result == self@.is_normal();
        fn is_underflow(&self) -> (result: bool)  ensures result == self@.is_underflow();
        fn is_overflow(&self) -> (result: bool)   ensures result == self@.is_overflow();
    }
*/

} // verus!
*/
} // mod
