// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Experiment: do weight-balanced trees with (Δ, Γ) = (3, 2) and weight =
//! size + 1 stay balanced after one `balance` step, under the precondition
//! insert and delete produce?
//!
//! Question. Node (l, k, r) with child weights a = w(l), w(r) = b + c where
//! b = w(r.l), c = w(r.r), and w(r.l) = b1 + b2 when r.l is a node. Invariant
//! at a node: 3·w(left) ≥ w(right) and 3·w(right) ≥ w(left). After inserting
//! into r or deleting from l the node satisfies w(r) ≤ 3·a + 3, and r, r.l are
//! balanced. `balance` rotates left when w(r) > 3·a: single if b < 2·c, double
//! otherwise. Is every resulting node balanced? (The right-heavy case; the
//! left-heavy case is its mirror.) The precondition proposed for the verified
//! `balance` is exactly: children balanced, w(r) ≤ 3·w(l) + 3, w(l) ≤ 3·w(r) + 3.
//!
//! Method. Plain Rust, no Verus. Enumerate all a, b1, b2, c in 1..=N.
//!
//! Run: rustc --edition 2021 --test -O src/experiments/wb_balance_arith.rs
//!        -o target/experiments/wb_balance_arith && target/experiments/wb_balance_arith --nocapture
//!
//! RESULT: SUCCEEDS (no counterexample for a, b1, b2, c in 1..=60)
//! DATE: 2026-09-24
//! Toolchain: rustc 1.98.1 (48a229cea 2026-09-01); log logs/experiment-wb_balance_arith.*.log.

fn bal(x: u64, y: u64) -> bool { 3 * x >= y && 3 * y >= x }

/// Returns the first counterexample (a, b1, b2, c, rotation) up to `n`.
fn search(n: u64) -> Option<(u64, u64, u64, u64, &'static str)> {
    for a in 1..=n {
        for c in 1..=n {
            // r.l empty: b = 1 (weight of an empty link).
            for (b, split) in std::iter::once((1u64, None)).chain(
                (1..=n).flat_map(|b1| (1..=n).map(move |b2| (b1 + b2, Some((b1, b2)))))) {
                if let Some((b1, b2)) = split { if !bal(b1, b2) { continue; } }
                if !bal(b, c) { continue; }            // r balanced
                let wr = b + c;
                if !(wr > 3 * a && wr <= 3 * a + 3) { continue; } // right-heavy, precondition
                if b < 2 * c {
                    // Single: (l, k, rl) under (_, rk, rr).
                    if !(bal(a, b) && bal(a + b, c)) {
                        let (b1, b2) = split.unwrap_or((0, 0));
                        return Some((a, b1, b2, c, "single"));
                    }
                } else {
                    let (b1, b2) = match split { Some(s) => s, None => return Some((a, 0, 0, c, "double-on-empty")) };
                    if !(bal(a, b1) && bal(b2, c) && bal(a + b1, b2 + c)) {
                        return Some((a, b1, b2, c, "double"));
                    }
                }
            }
        }
    }
    None
}

#[test]
fn wb_balance_arith() {
    let n = 60;
    let found = search(n);
    println!("N = {}: first counterexample: {:?}", n, found);
    assert!(found.is_none());
}
