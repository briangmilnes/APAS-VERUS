// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//
// Minimal reproducer for:
//   Verus Internal Error: var_local_id failed
//   rustc_mir_build/src/builder/mod.rs:275
//
// Trigger: -V new-mut-ref with a loop invariant that contains a
// match expression binding a variable from an Option via Some(k).
//
// Run with:
//   verus reproducer.rs --crate-type lib -V new-mut-ref
//
// Expected: panic "Verus Internal Error: var_local_id failed: ..."
// Without -V new-mut-ref: 2 verified, 0 errors

use vstd::prelude::*;

verus! {

// Minimal case: loop invariant with `match found { Some(k) => expr(k), None => ... }`.
// No &mut, no Vec, no ghost variables — just the match-in-invariant pattern.
fn minimal(n: u64) {
    let mut found: Option<u64> = None;
    let mut i: u64 = 0;
    while i < n
        invariant
            match found { Some(k) => k < 1000u64, None => true },
        decreases n - i,
    {
        found = Some(i);
        i += 1;
    }
}

// Slightly larger variant closer to the APAS-VERUS triggering code:
// Option search with break + match invariant.
fn search_with_break(n: u64, target: u64) {
    let mut found: Option<u64> = None;
    let mut i: u64 = 0;
    while i < n
        invariant
            i <= n,
            match found { Some(k) => k < n, None => true },
        decreases n - i,
    {
        if i == target { found = Some(i); break; }
        i += 1;
    }
}

} // verus!
