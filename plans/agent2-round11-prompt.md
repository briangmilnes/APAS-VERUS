# Agent 2 — Round 11 Prompt

## Mission

Continue Chap42 (16 holes) and Chap47 (10 holes). Focus on Chap42 — Chap47's remaining
holes are mostly genuine Verus limitations.

## Your Files

**Chap42** (16 holes across 4 files — PRIMARY):
- `TableStEph.rs` — holes TBD
- `TableStPer.rs` — holes TBD (1 blocked by weak insert spec per your report)
- `TableMtEph.rs` — 11 functions need fork-join proofs (per your report)
- `Example42_1.rs` — SKIP (Example file)

**Chap47** (10 holes across 9 files — SECONDARY):
- `ParaHashTableStEph.rs` — 4 (call_hash_fn, linear_probe, quadratic_probe, double_hash_probe)
- `LinProbFlatHashTableStEph.rs` — 1 (probe)
- `QuadProbFlatHashTableStEph.rs` — 1 (probe)
- `DoubleHashFlatHashTableStEph.rs` — 2 (probe, second_hash)
- `ChainedHashTable.rs` — 2 (insert_chained, delete_chained — tuple Clone)

## Priority Order

1. **Chap42 StEph/StPer** — Prove what you can in the sequential variants first.
2. **Chap42 TableMtEph** — Fork-join proofs. Read the HFScheduler standard
   (`src/standards/hfscheduler_standard.rs`) and closure standard for patterns.
3. **Chap47** — Only if you get traction on workarounds for Fn closures or tuple Clone.

## Specific Guidance for Chap42

### TableMtEph (11 fork-join functions)

You said these need "verified fork-join proofs." The pattern is:
1. Split the table into halves
2. Fork: each half processed in parallel via `join(f1, f2)`
3. Join: merge results

Read `src/standards/hfscheduler_standard.rs` and `src/standards/using_closures_standard.rs`.
The closures need explicit `ensures` on the named closure variables. Ghost state captures
the pre-fork view.

For each fork-join function:
- Bind closures to named variables with `ensures`
- Capture ghost copies of relevant state before fork
- The join result inherits both ensures

### TableStPer weak insert spec

You said 1 StPer hole is blocked by weak insert spec. Strengthen the insert ensures:
- `ensures self@.dom().contains(key@), self@[key@] == value@`
- Size increases by 1 if key was not present, stays same if it was
- All other entries unchanged

### Chap42 depends on Chap41::ArraySetStEph

Agent 4 is working on ArraySetStEph (3 holes). Some of your proofs may be blocked until
those are fixed. Prove what you can independently.

## Chap47 Remaining 10

You said 8 are blocked by opaque Fn closures, 2 by tuple Clone. These ARE hard:

- **Fn closure workaround**: If the closure is a simple hash function, try replacing the
  Fn parameter with a concrete hash trait method. Or write an `external_body` helper that
  calls the closure with tight ensures.
- **Tuple Clone**: If `(K, V)` doesn't implement Clone in Verus, try destructuring and
  cloning each field separately.
- **second_hash (DefaultHasher)**: Genuinely external. Leave as external_body.

If you can get even 2-3 of these, that's good progress.

## DO NOT TOUCH (other agents' files)

- Chap38, Chap39 — Agent 3
- Chap41, Chap53 — Agent 4

## Rules

- Read `src/standards/*.rs` before modifying code.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- When adding requires to traits, UPDATE ALL CALLERS.
- Push to `agent2/ready`. Write `plans/agent2-round11-report.md`.
