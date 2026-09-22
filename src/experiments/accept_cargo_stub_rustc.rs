// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment (r207, agent 2): does the cargo (non-Verus) path of
//! `src/vstdplus/accept.rs`, section 14 only, compile under rustc 1.98.1 with
//! `verus_keep_ghost` declared as a check-cfg? Plain Rust, so compiled with
//! `rustc --crate-type=lib`, not verified.
//!
//! RESULT: SUCCEEDS — rustc 1.98.1 (48a229cea 2026-09-01), exit 0, no warnings.
//! DATE: 2026-09-20
//! LOG: logs/rustc-experiment-accept_cargo_stub_rustc.20260920-160804.log
//! Command: rustc --crate-type=lib --edition 2021 --check-cfg 'cfg(verus_keep_ghost)' src/experiments/accept_cargo_stub_rustc.rs

// Standalone copy of the cargo (non-verus) path of src/vstdplus/accept.rs, sections 14 only.
#[cfg(not(verus_keep_ghost))]
pub use cargo_accept::accept;

#[cfg(not(verus_keep_ghost))]
mod cargo_accept {
    /// Stub for cargo/runtime builds. Verus uses the proof fn above.
    pub fn accept(_b: bool) {}
}
