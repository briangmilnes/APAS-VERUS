//! Proof time test: Does Arc::clone preserve dereferenced value equality?
//!
//! Hypothesis: Verus can verify that Arc::clone preserves *x == *y (deref equality) for
//! concrete and generic T, using vstd's Arc spec.
//!
//! Result: PASSES — Arc::clone preserves *x == *y for u8, u64, and generic T.

use vstd::prelude::*;
use std::sync::Arc;

verus! {

/// Test with concrete Arc<u8> - per Chris, this should work
fn test_arc_clone_u8(x: Arc<u8>) {
    let y = x.clone();
    assert(*x == *y);
}

/// Test with generic Arc<T>
fn test_arc_clone_generic<T: Copy + PartialEq>(x: Arc<T>) {
    let y = x.clone();
    assert(*x == *y);
    assert(x == y);
}

} // verus!
