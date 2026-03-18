# R38 Agent 1: Chap47 Flat Hash Tables + ParaHashTable Warnings

## Baseline
- Main at `485299d3`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4332 verified, 204 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
If you write `accept(` in any file, you have failed your assignment. Use `assume()`
for eq/clone bridges inside Clone::clone and PartialEq::eq bodies ONLY. Everywhere
else, PROVE the obligation or leave the existing assume in place.

**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true`.** Add real preconditions or leave the warning.
**DO NOT add `// veracity: no_requires`.** Only the user adds those.

Read CLAUDE.md and `src/standards/partial_eq_eq_clone_standard.rs` before starting.

## Assignment

You are Agent 1 for R38. Your scope is **Chap47 flat hash tables + ParaHashTable warnings**.

## Task 1: ParaHashTableStEph.rs warnings (8 warnings + 1 external_body)

File: `src/Chap47/ParaHashTableStEph.rs`

Fix these fn_missing_wf warnings by adding real `requires`/`ensures`:
- Line 572: `createTable` — add `ensures table.spec_hashtable_wf()`
- Line 623: `insert` — add `requires table.spec_hashtable_wf()`
- Line 637: `lookup` — add `requires table.spec_hashtable_wf()`
- Line 647: `delete` — add `requires table.spec_hashtable_wf()`
- Line 660: `metrics` — add `requires table.spec_hashtable_wf()`
- Line 669: `loadAndSize` — add `requires table.spec_hashtable_wf()`
- Line 686: `resize` — add `requires table.spec_hashtable_wf()` AND `ensures resized.spec_hashtable_wf()`

The `call_hash_fn` external_body (line 463) is a stretch goal — prove if time permits.

## Task 2: Prove eq/clone bridge assumes in flat hash tables

These files all have the same two assume patterns:

**Pattern A — Eq bridge:** `assume(eq == spec_flat_has_key(table.table@[slot], key))`
After calling `k == key` (PartialEq::eq), the result `eq` should equal the spec.
The proof: `PartialEq::eq` ensures `r == (self@ == other@)`. Assert that
`(k@ == key@) == spec_flat_has_key(...)` from the spec definition.

**Pattern B — Clone bridge:** `assume(key == pairs@[j].0)` / `assume(value == pairs@[j].1)`
After cloning key/value, the clone equals the original at the View level.
The proof: `Clone::clone` ensures `cloned@ == self@`. Use that directly.

**Pattern C — assume(false) table full:** `assume(false); // Table full`
Prove that with load factor < 1, the table always has an empty slot. This requires
showing that `count < m` implies at least one slot is Empty. Use pigeonhole: if all
m slots are Occupied, count >= m, contradicting count < m.

Files and holes:

1. `src/Chap47/LinProbFlatHashTableStEph.rs` — 6 holes
   - Line 132: eq bridge in insert
   - Line 348: assume(false) table full
   - Line 391: eq bridge in lookup
   - Line 497: eq bridge in delete
   - Lines 715-716: clone bridges in resize

2. `src/Chap47/QuadProbFlatHashTableStEph.rs` — 6 holes
   - Line 110: eq bridge in insert
   - Line 366: assume(false) table full
   - Line 404: eq bridge in lookup
   - Line 563: eq bridge in delete
   - Lines 835-836: clone bridges in resize

3. `src/Chap47/DoubleHashFlatHashTableStEph.rs` — 9 holes
   - Line 96: wf bridge in insert (forall quantifier)
   - Line 149: eq bridge in insert
   - Line 362: assume(false) table full
   - Line 375: wf bridge in lookup
   - Line 425: eq bridge in lookup
   - Line 521: wf bridge in delete
   - Line 573: eq bridge in delete
   - Lines 807-808: clone bridges in resize

4. `src/Chap47/StructChainedHashTable.rs` — 4 holes + 1 external_body
   - Line 124: eq bridge in insert
   - Line 163: eq bridge in lookup
   - Line 167: clone bridge in lookup
   - Line 205: eq bridge in delete
   - Line 398: external_body resize (stretch goal)

## Strategy

Start with LinProbFlatHashTableStEph.rs — it's the simplest (linear probing).
Crack the eq bridge and clone bridge patterns there, then replicate to QuadProb
and DoubleHash. Do StructChainedHashTable last (slightly different structure).

Read `src/standards/partial_eq_eq_clone_standard.rs` to understand the eq/clone
ensures pattern.

## Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Do NOT run validate concurrently with other agents.
Write your report to `plans/agent1-r38-report.md`.
