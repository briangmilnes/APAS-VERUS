# R88 Agent 1 — Fix rand files: VertexMatching + StarPartitionMtEph, STEP 20

## Objective

Fix 3 files that use raw `rand` crate calls. Replace with `random_usize_range`
from `vstdplus::rand::rand`. Then the 5 chain-dependent files should compile too.

Files to fix:
1. `src/Chap61/VertexMatchingStEph.rs`
2. `src/Chap61/VertexMatchingMtEph.rs`
3. `src/Chap62/StarPartitionMtEph.rs`

Chain dependents (should compile once the 3 above are fixed):
4. `src/Chap61/EdgeContractionStEph.rs`
5. `src/Chap61/EdgeContractionMtEph.rs`
6. `src/Chap62/StarContractionMtEph.rs`
7. `src/Chap63/ConnectivityMtEph.rs`
8. `src/Chap64/SpanTreeMtEph.rs`

## lib.rs — uncomment your files

Uncomment ALL 8 files in lib.rs (the 3 you're fixing + the 5 chain dependents).
They are currently commented out with `// BROKEN`. Remove the comment prefix.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap64
```

This pulls in Chap61, 62, 63, 64 + all their transitive deps.
Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## What to fix

The files use `rand::Rng`, `rand::rng()`, or similar. Verus can't link the rand
crate directly. The project provides a verified wrapper:

```rust
use crate::vstdplus::rand::rand::random_usize_range;

// random_usize_range(lo, hi) -> result
//   requires lo < hi
//   ensures lo <= result < hi
let coin = random_usize_range(0, 2);  // 0 or 1
let idx = random_usize_range(0, n);   // 0..n-1
```

Read `src/vstdplus/rand.rs` and `src/standards/using_rand_standard.rs` for the
full pattern.

For each file:
1. Remove `use rand::*` or `use rand::Rng` imports
2. Add `use crate::vstdplus::rand::rand::random_usize_range;`
3. Replace `rng.gen_range(lo..hi)` or similar with `random_usize_range(lo, hi)`
4. Remove any `#[cfg(not(verus_keep_ghost))]` gates that were hiding rand usage

## Important

- Do NOT add `assume` or `accept`.
- Do NOT weaken ensures clauses.
- If chain dependents have their own compile errors (beyond the rand dep),
  fix those too. Use `external_body` on functions that are too hard to prove.
- Read the St variant for each file as reference for the expected API.

## STEP 20

## Report

Write `plans/agent1-round88-report.md`.
