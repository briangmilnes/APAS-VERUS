// Copyright (c) 2025 Brian G. Milnes
//! Intentional proof holes — per veracity/docs/Accepted.md
//!
//! Veracity will info this as a proof hole but not error or warn.
//!
//! We cannot write a declarative macro for #[verifier::external_body] (see
//! experiments/accept_external_body.rs). So for external_body functions,
//! Veracity will use `// accept hole` comments after the attribute to
//! indicate accepted holes, rather than an attribute macro.

use vstd::prelude::*;

verus! {

/// Intentional proof hole. Use instead of `assume()` for accepted workarounds.
/// Veracity: info, not error or warning.
pub proof fn accept(b: bool)
    ensures b,
{
    admit();
}

} // verus!

// Re-export for cargo/runtime builds where proof fn may not be available.
#[cfg(not(verus_keep_ghost))]
pub use cargo_accept::accept;

#[cfg(not(verus_keep_ghost))]
mod cargo_accept {
    /// Stub for cargo/runtime builds. Verus uses the proof fn above.
    pub fn accept(_b: bool) {}
}
