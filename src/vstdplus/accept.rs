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
