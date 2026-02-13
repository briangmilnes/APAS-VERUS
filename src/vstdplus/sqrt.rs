// Copyright (c) 2025 Brian G. Milnes
//! Verus specifications for Rust's integer `isqrt()` method (stabilized in Rust 1.80).
//! - ensures root * root <= n
//! - ensures (root + 1) * (root + 1) > n

pub mod sqrt {

use vstd::prelude::*;

verus! {

pub assume_specification [<usize>::isqrt] (n: usize) -> (root: usize)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

pub assume_specification [<u64>::isqrt] (n: u64) -> (root: u64)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

pub assume_specification [<u32>::isqrt] (n: u32) -> (root: u32)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

pub assume_specification [<u16>::isqrt] (n: u16) -> (root: u16)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

pub assume_specification [<u8>::isqrt] (n: u8) -> (root: u8)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

pub assume_specification [<u128>::isqrt] (n: u128) -> (root: u128)
    ensures
        root as int * root as int <= n as int,
        (root as int + 1) * (root as int + 1) > n as int;

} // verus!

} // mod sqrt
