// Copyright (c) 2025 Brian G. Milnes
//! REVIEWED: NO
//! Specifications for std::collections::HashSet methods not covered by vstd


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 9. impls

//		Section 1. module

pub mod hash_set_specs {


//		Section 2. imports

use vstd::prelude::*;
use std::collections::HashSet;
use core::hash::Hash;

verus! 
{

    //		Section 9. impls


pub assume_specification<T, S, A> [<std::collections::HashSet<T, S, A> as std::clone::Clone>::clone] (_0: &std::collections::HashSet<T, S, A>) -> std::collections::HashSet<T, S, A>
where
    S: std::clone::Clone,
    T: std::clone::Clone,
    A: std::clone::Clone + std::alloc::Allocator,
;

} // verus!

} // mod hash_set_specs
