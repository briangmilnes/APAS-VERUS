//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

#[cfg(verus_keep_ghost)]
pub mod sigma_pi {

    use vstd::prelude::*;
    use vstd::set::fold::is_fun_commutative;
    use vstd::arithmetic::mul::group_mul_properties;

verus! {

    broadcast use group_mul_properties;

    pub trait SigmaPi<T: Integer> {
        spec fn zero() -> T;
        spec fn one() -> T;
        spec fn min_val() -> T;
        spec fn max_val() -> T;
        spec fn add(n1: T, n2: T) -> T;
        spec fn mul(n1: T, n2: T) -> T;

        spec fn spec_sigma(s: Set<T>, init: T, f: spec_fn(T, T) -> T) -> T
            recommends s.finite(), is_fun_commutative(f);

        spec fn spec_pi(s: Set<T>, init: T, f: spec_fn(T, T) -> T) -> T
            recommends s.finite(), is_fun_commutative(f);

        proof fn add_commutative_at(n1: T, n2: T, n3: T)
            requires
               Self::min_val() as int <= (n3 as int) + (n1 as int)               <= Self::max_val() as int,
               Self::min_val() as int <= (n3 as int) + (n2 as int)               <= Self::max_val() as int,
               Self::min_val() as int <= (n3 as int) + (n1 as int) + (n2 as int) <= Self::max_val() as int,
            ensures Self::add(Self::add(n3, n2), n1) == Self::add(Self::add(n3, n1), n2);

        proof fn lemma_add_commutative()
            ensures is_fun_commutative::<T, T>(|acc: T, x: T| Self::add(acc, x));

        proof fn mul_commutative_at(n1: T, n2: T, n3: T)
            requires 
              Self::min_val() as int <= Self::mul(n3, n1) as int                       <= Self::max_val() as int,
              Self::min_val() as int <= Self::mul(n3, n2) as int                       <= Self::max_val() as int,
              Self::min_val() as int <= Self::mul(Self::mul(n3, n1), n2) as int        <= Self::max_val() as int,
              Self::min_val() as int <= Self::mul(Self::mul(n3, n2), n1) as int        <= Self::max_val() as int,
            ensures Self::mul(Self::mul(n3, n2), n1) == Self::mul(Self::mul(n3, n1), n2);

        proof fn lemma_mul_commutative()
            ensures is_fun_commutative::<T, T>(|acc: T, x: T| Self::mul(acc, x));

        open spec fn sigma(s: Set<T>) -> T { Self::spec_sigma(s, Self::zero(), |acc: T, x: T| Self::add(acc, x)) }
        
        open spec fn pi(s: Set<T>) -> T { Self::spec_pi(s, Self::one(), |acc: T, x: T| Self::mul(acc,x)) }

    }

    impl SigmaPi<i32> for i32 {
        open spec fn zero() -> i32 { 0 }
        open spec fn one() -> i32 { 1 }
        open spec fn min_val() -> i32 { i32::MIN } 
        open spec fn max_val() -> i32 { i32::MAX }
        open spec fn add(a: i32, b: i32) -> i32 { (a + b) as i32 }
        open spec fn mul(a: i32, b: i32) -> i32 { (a * b) as i32 }

        open spec fn spec_sigma(s: Set<i32>, init: i32, f: spec_fn(i32, i32) -> i32) -> i32 { s.fold(init, f) }
        open spec fn spec_pi   (s: Set<i32>, init: i32, f: spec_fn(i32, i32) -> i32) -> i32 { s.fold(init, f) }

        proof fn add_commutative_at(n1: i32, n2: i32, n3: i32)
            ensures Self::add(Self::add(n3, n2), n1) == Self::add(Self::add(n3, n1), n2)
        {}

        proof fn lemma_add_commutative() { admit(); }

        proof fn mul_commutative_at(n1: i32, n2: i32, n3: i32)
            ensures Self::mul(Self::mul(n3, n2), n1) == Self::mul(Self::mul(n3, n1), n2)
        { admit() }

        proof fn lemma_mul_commutative() { admit(); }
    }

/*
    impl SigmaPi<u32> for u32 {
        open spec fn zero() -> u32 { 0 }
        open spec fn one() -> u32 { 1 }
        open spec fn min_val() -> u32 { u32::MIN }
        open spec fn max_val() -> u32 { u32::MAX }
        open spec fn add(a: u32, b: u32) -> u32 { (a + b) as u32 }
        open spec fn mul(a: u32, b: u32) -> u32 { (a * b) as u32 }

        open spec fn spec_sigma(s: Set<u32>, init: u32, f: spec_fn(u32, u32) -> u32) -> u32 { s.fold(init, f) }
        open spec fn spec_pi(s: Set<u32>, init: u32, f: spec_fn(u32, u32) -> u32) -> u32 { s.fold(init, f) }

        proof fn lemma_add_commutative() {
            // General commutativity requires no overflow - use u32_add_commutative_at for specific values
            admit();
        }
        proof fn lemma_mul_commutative() {
            // General commutativity requires no overflow - use u32_mul_commutative_at for specific values
            admit();
    // Helper proofs for u32 commutativity at specific values
    pub proof fn u32_add_commutative_at(n1: u32, n2: u32, n3: u32)
        requires n3 + n1 + n2 <= u32::MAX
        ensures <u32 as SigmaPi<u32>>::add(<u32 as SigmaPi<u32>>::add(n3, n2), n1) 
             == <u32 as SigmaPi<u32>>::add(<u32 as SigmaPi<u32>>::add(n3, n1), n2)
    {}

    pub proof fn u32_mul_commutative_at(n1: u32, n2: u32, n3: u32)
        requires 
            n3 * n1 <= u32::MAX,
            n3 * n2 <= u32::MAX,
            (n3 * n1) * n2 <= u32::MAX,
            (n3 * n2) * n1 <= u32::MAX,
        ensures <u32 as SigmaPi<u32>>::mul(<u32 as SigmaPi<u32>>::mul(n3, n2), n1) 
             == <u32 as SigmaPi<u32>>::mul(<u32 as SigmaPi<u32>>::mul(n3, n1), n2)
    {}
        }
    }
*/

 }
}
