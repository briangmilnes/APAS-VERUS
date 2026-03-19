# R42 Agent 4: StructChained Resize + Chap59 Johnson + Warnings

## Baseline
- Main after R41 merge, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R41b you closed the DoubleHash wf bridges using the spec_second_hash technique.
Now tackle the StructChained resize (deferred from R41) and start on graph algorithms.

## Assignment

### Part A: StructChainedHashTable resize (1 hole)

`src/Chap47/StructChainedHashTable.rs` line 413: `external_body` on `resize`.

Your R41b report estimated 50-80 lines of proof. The resize function:
1. Creates a new larger table
2. Iterates all chains in the old table
3. Re-inserts each entry into the new table using the hash function

**Strategy**: The proof needs to show that after rehashing, the new table's abstract
map equals the old table's abstract map. Key steps:
- Loop invariant: entries processed so far are in the new table
- Chain traversal: `spec_chain_to_map` connects linked list to abstract entries
- Insert preservation: each insert into new table preserves previously inserted entries

Read the `spec_chain_to_map` spec function and the insert postconditions.

### Part B: Chap59 JohnsonMtEphI64 (5 holes)

`src/Chap59/JohnsonMtEphI64.rs` — 5 external_body methods. Johnson's algorithm for
all-pairs shortest paths.

Read the file. The MtEph variant likely wraps a StEph implementation with RwLock.
If the pattern is the same as other Mt modules (acquire lock, call inner St method,
release, wrap result), these should be straightforward delegation proofs.

Check if `src/Chap59/JohnsonStEphI64.rs` is clean — if so, the MtEph can delegate to it.

### Part C: Chap59 JohnsonStEphI64 remaining holes (1 assume)

Check the Chap59 analysis log. JohnsonStEphI64.rs had 1 assume + 2 fn_missing_requires
warnings per the earlier analysis. Fix the warnings and try the assume.

### Part D: Chap43 OrderedSet from_sorted_elements warnings (2 warnings)

From R41: OrderedSetStEph.rs line 1385 and OrderedSetStPer.rs line 1157 have
`fn_missing_requires` on `from_sorted_elements`. Your R41 report said "No real
precondition — from_vec has no requires." If that's true, these need user annotation.
But double-check: does the function assume the input is sorted? Does it assume no
duplicates? If the function name says "sorted" but doesn't require it, the function
has a bug in its contract.

Read the function body. If it truly works on any input (just builds from Vec without
sorting), then report it. If it assumes sorted input, add the real requires.

### Priority

1. Part B (JohnsonMtEph 5 holes) — likely quick if delegation pattern
2. Part C (JohnsonStEph 1 hole + warnings) — small
3. Part A (resize 1 hole) — moderate complexity
4. Part D (2 warnings) — cleanup

### Expected Results

Conservative: 4-5 holes closed.
Optimistic: 7 holes closed. Close Chap59.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r42-report.md`.
