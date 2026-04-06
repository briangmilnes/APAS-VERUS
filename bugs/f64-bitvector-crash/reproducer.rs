// Verus crash: is_nan_spec() inside by(bit_vector) panics on f64
//
// Verus revision: c78aa4958372cfa69e6cb38fd31997c881473271
// Date: 2026-03-27
//
// Expected: Verus attempts the bitvector proof (may succeed or hit rlimit).
// Actual: Verus panics: "internal error: unexpected float to bits coercion"
//         at bitvector_to_air.rs:424.
//
// The crash triggers when is_nan_spec() (from FloatBitsProperties) is used
// inside a by(bit_vector) assertion on f64. Range-bound assertions without
// is_nan_spec work fine on f64.

//  Table of Contents
//	Section 7. proof fns/broadcast groups

use vstd::prelude::*;
use vstd::float::FloatBitsProperties;

verus! {

    //		Section 7. proof fns/broadcast groups


// CRASHES: is_nan_spec() inside by(bit_vector) on f64.
proof fn f64_reflexive_nan(x: f64)
    ensures !x.is_nan_spec() ==> x <= x,
{
    assert(!x.is_nan_spec() ==> x <= x) by(bit_vector);
}

} // verus!

fn main() {}
