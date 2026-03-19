# R41b Agent 4: Strengthen Hash Specs + DoubleHash WF Bridges

## Baseline
- Main at `29641a5e`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- Agent 4 R41a consolidated clone_elem and added diverge(). Now do the real work.

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R41a, you declared the DoubleHash wf bridge assumes (3 holes) "irreducible" because
`compute_second_hash` is `external_body` with no spec connecting the runtime step to
the wf existential. That analysis was lazy. The `external_body` must stay (uses
`std::hash`), but the **ensures can be strengthened**. An `external_body` with weak
ensures is a spec problem, not a Verus limitation.

## Assignment

### Part A: Strengthen compute_second_hash spec

`src/Chap47/ParaHashTableStEph.rs` line 493:

```rust
#[verifier::external_body]
pub fn compute_second_hash<Key: std::hash::Hash>(key: &Key, table_size: usize) -> (step: usize)
    requires table_size > 0,
    ensures step >= 1,
```

Current ensures: only `step >= 1`. This is too weak. Add:
- `ensures step < table_size` — the step must be a valid stride within the table
- Consider adding a `spec fn spec_second_hash(key: Key) -> nat` and connecting:
  `ensures step == spec_second_hash(*key) as usize` (modulo table_size)

This gives the DoubleHash wf bridge something to work with. The external_body stays
(std::hash can't be verified), but the spec contract becomes useful.

Read the function body to see what it actually computes:
```rust
let hash2 = hash_value % (table_size - 1) + 1;
```
So the true postcondition is `1 <= step <= table_size - 1`, i.e., `step >= 1 && step < table_size`.

### Part B: Strengthen call_hash_fn spec

`src/Chap47/ParaHashTableStEph.rs` line 463:

Same issue. Current ensures: `index < table_size`. Check if it can be connected to
`spec_hash` — the ghost `spec_fn(Key) -> nat` parameter. If the function computes
`hash_fn(key, table_size)` and the spec says `spec_hash(key) % table_size == index`,
then add that ensures.

### Part C: Close DoubleHash wf bridge assumes (3 holes)

`src/Chap47/DoubleHashFlatHashTableStEph.rs` lines 116, 395, 541.

These assume that for every occupied slot, the key's hash pattern matches the probing
scheme. With strengthened specs on compute_second_hash and call_hash_fn, you should be
able to prove these from the wf predicate + the hash function specs.

Read the wf predicate (`spec_doublehashflathashsteph_wf`) carefully. It likely says
something like: for every occupied slot j, there exists a step s such that the key at
j is reachable from hash(key) with stride s. The assume bridges "s == compute_second_hash(key)".
With the strengthened ensures, you can instantiate the existential.

If the wf predicate doesn't contain enough information, you may need to strengthen it.
That's fine — strengthening wf predicates to enable proofs is good engineering.

### Part D: StructChainedHashTable resize (1 hole)

`src/Chap47/StructChainedHashTable.rs` line 413: `external_body` on `resize`.

Read the implementation. If it iterates chains, extracts entries, and reinserts into a
new table, the proof follows from insert's postconditions + induction over chains.

### Priority

1. Part A (strengthen compute_second_hash) — prerequisite for Part C
2. Part B (strengthen call_hash_fn) — may also help
3. Part C (close 3 wf bridge assumes) — the real payoff
4. Part D (resize) — bonus

### Expected Results

Conservative: Strengthen 2 specs + close 1-2 wf bridges.
Optimistic: Strengthen 2 specs + close 3 wf bridges + resize = -4 holes.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r41b-report.md`.
