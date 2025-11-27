//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod sigma_pi {

    use vstd::prelude::*;
    use vstd::set::fold::is_fun_commutative;
    use vstd::arithmetic::mul::group_mul_properties;

verus! {

    broadcast use group_mul_properties;

    pub trait SigmaPi<T> {
        spec fn zero()                                          -> T;
        spec fn one()                                           -> T;
        spec fn add(a: T, b: T)                                 -> T;
        spec fn mul(a: T, b: T)                                 -> T;

        spec fn spec_sigma(s: Set<T>, init: T, f: spec_fn(T, T) -> T) -> T
            recommends s.finite(), is_fun_commutative(f);

        spec fn spec_pi(s: Set<T>, init: T, f: spec_fn(T, T)    -> T) -> T
            recommends s.finite(), is_fun_commutative(f);

        proof fn lemma_add_commutative()
            ensures is_fun_commutative::<T, T>(|acc: T, x: T| Self::add(acc, x));

        proof fn lemma_mul_commutative()
            ensures
                is_fun_commutative::<T, T>(|acc: T, x: T| Self::mul(acc, x));

        open spec fn sigma(s: Set<T>) -> T { Self::spec_sigma(s, Self::zero(), |acc: T, x: T| Self::add(acc, x)) }
        
        open spec fn pi(s: Set<T>) -> T { Self::spec_pi(s, Self::one(), |acc: T, x: T| Self::mul(acc, x)) }

    }

    impl SigmaPi<i32> for i32 {
        open spec fn zero() -> i32 { 0 }
        open spec fn one() -> i32 { 1 }
        open spec fn add(a: i32, b: i32) -> i32 { (a + b) as i32 }
        open spec fn mul(a: i32, b: i32) -> i32 { (a * b) as i32 }

        open spec fn spec_sigma(s: Set<i32>, init: i32, f: spec_fn(i32, i32) -> i32) -> i32 { s.fold(init, f) }
        open spec fn spec_pi   (s: Set<i32>, init: i32, f: spec_fn(i32, i32) -> i32) -> i32 { s.fold(init, f) }

        proof fn lemma_add_commutative() { admit(); }
        proof fn lemma_mul_commutative() { admit(); }
    }

    impl SigmaPi<u32> for u32 {
        open spec fn zero() -> u32 { 0 }
        open spec fn one() -> u32 { 1 }
        open spec fn add(a: u32, b: u32) -> u32 { (a + b) as u32 }
        open spec fn mul(a: u32, b: u32) -> u32 { (a * b) as u32 }

        open spec fn spec_sigma(s: Set<u32>, init: u32, f: spec_fn(u32, u32) -> u32) -> u32 { s.fold(init, f) }

        open spec fn spec_pi(s: Set<u32>, init: u32, f: spec_fn(u32, u32) -> u32) -> u32 { s.fold(init, f) }

        proof fn lemma_add_commutative() { admit(); }
        proof fn lemma_mul_commutative() { admit(); }
    }

    pub proof fn lemma_int_add_commutative()
        ensures is_fun_commutative::<int, int>(|acc: int, x: int| acc + x)
    {}

    pub proof fn lemma_int_mul_commutative()
        ensures is_fun_commutative::<int, int>(|acc: int, x: int| acc * x)
    {}

    pub proof fn lemma_u32_add_commutative_bounded(a1: u32, a2: u32, a3: u32)
        requires a3 + a1 + a2 <= u32::MAX
        ensures ((a3 + a2) as u32 + a1) as u32 == ((a3 + a1) as u32 + a2) as u32
    {}

    pub proof fn lemma_u32_mul_commutative_bounded(a1: u32, a2: u32, a3: u32)
        requires 
            a3 * a1 <= u32::MAX,
            a3 * a2 <= u32::MAX,
            (a3 * a1) * a2 <= u32::MAX,
            (a3 * a2) * a1 <= u32::MAX,
        ensures ((a3 * a2) as u32 * a1) as u32 == ((a3 * a1) as u32 * a2) as u32
    {}

    pub proof fn lemma_i32_add_commutative_bounded(a1: i32, a2: i32, a3: i32)
        requires 
            i32::MIN <= a3 + a1 <= i32::MAX,
            i32::MIN <= a3 + a2 <= i32::MAX,
            i32::MIN <= a3 + a1 + a2 <= i32::MAX,
        ensures ((a3 + a2) as i32 + a1) as i32 == ((a3 + a1) as i32 + a2) as i32
    {}

    pub proof fn lemma_i32_mul_commutative_bounded(a1: i32, a2: i32, a3: i32)
        requires 
            i32::MIN <= a3 * a1 <= i32::MAX,
            i32::MIN <= a3 * a2 <= i32::MAX,
            i32::MIN <= (a3 * a1) * a2 <= i32::MAX,
            i32::MIN <= (a3 * a2) * a1 <= i32::MAX,
        ensures ((a3 * a2) as i32 * a1) as i32 == ((a3 * a1) as i32 * a2) as i32
    {}

    } // verus!
}
