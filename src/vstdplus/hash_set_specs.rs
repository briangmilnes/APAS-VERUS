//! Specifications for std::collections::HashSet methods not covered by vstd

pub mod hash_set_specs {

use vstd::prelude::*;
use std::collections::HashSet;
use core::hash::Hash;

verus! {

pub assume_specification<T, S> [<std::collections::HashSet<T, S> as std::clone::Clone>::clone] (_0: &std::collections::HashSet<T, S>) -> std::collections::HashSet<T, S>
where
    S: std::clone::Clone,
    T: std::clone::Clone,
;

} // verus!

} // mod hash_set_specs

