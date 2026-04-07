//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Using Rand Standard: how to use randomness in verified code.
//!
//! APAS algorithms that use randomness (coin flips, random sampling, random pivots)
//! must use `vstdplus::rand::rand::random_usize_range` — never raw `rand` crate calls.
//!
//! The verified wrapper provides:
//!   - `random_usize_range(lo, hi) -> result` with `ensures lo <= result < hi`
//!   - The external_body trust boundary: rand returns a value in the requested range.
//!   - No cfg gates on the public API — callable from inside verus!.
//!
//! Pattern: Factor randomness out of algorithmic logic.
//!
//!   BAD — entire function cfg-gated because it imports rand:
//!
//!     #[cfg(not(verus_keep_ghost))]
//!     fn star_partition(graph: &G) -> Partition {
//!         use rand::Rng;
//!         let mut rng = rand::rng();
//!         for v in vertices {
//!             let coin: bool = rng.random();  // Unverifiable.
//!             ...algorithmic logic using coin...
//!         }
//!     }
//!
//!   GOOD — randomness factored into vstdplus::rand call, logic is verifiable:
//!
//!     use crate::vstdplus::rand::rand::random_usize_range;
//!
//!     fn star_partition(graph: &G) -> Partition {
//!         for v in vertices {
//!             let coin = random_usize_range(0, 2);  // Verified: ensures 0 <= coin < 2.
//!             let is_head = coin == 0;
//!             ...algorithmic logic using is_head (verifiable)...
//!         }
//!     }
//!
//! For boolean coin flips, use `random_usize_range(0, 2) == 0`.
//! For k-way choices, use `random_usize_range(0, k)`.
//! For seeded deterministic randomness (tests, reproducibility), add a seed parameter
//! and a separate external_body helper that wraps `StdRng::seed_from_u64`.
//!
//! What NOT to do:
//!   - Do NOT `use rand::Rng` inside function bodies — makes the body uncompilable under Verus.
//!   - Do NOT cfg-gate a function just because it uses randomness.
//!   - Do NOT put `#[verifier::external_body]` on an entire algorithm because of one rand call.
//!   - Do NOT use `rand::rng()` directly — it requires `use rand::RngExt` which is cfg-gated.
//!
//! See: `src/vstdplus/rand.rs` for the implementation.

pub mod using_rand_standard {}
