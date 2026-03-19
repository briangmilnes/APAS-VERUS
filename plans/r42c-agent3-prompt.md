# R42c Agent 3: Chap47 + Chap41 + Chap43 Cleanup

## Baseline
- Main at `c010cf2a`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4333 verified, 146 holes, 30 clean chapters
- (Your R42b work is not yet merged — work from main)

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**
**DO NOT move code outside `verus!{}` or add `#[cfg(not(verus_keep_ghost))]` to
dodge verification.** All algorithm implementations belong inside `verus!{}`.
If you can't prove it, leave the `external_body` and report what you tried.

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Assignment

### Part A: Chap47 Quick Wins (3-5 holes)

**1. StructChainedHashTable.rs resize** (line 403, 1 external_body)

Read the file. The resize function creates a new larger table and re-inserts
all entries. Proof strategy:
- Loop invariant: entries processed so far are in new table
- `spec_chain_to_map` connects linked list chains to abstract entries
- Insert postcondition preserves previously inserted entries

**2. DoubleHashFlatHashTableStEph.rs** (line 359) and **LinProbFlatHashTableStEph.rs**
(line 355) — both have `assume(false)` without `diverge()`. Add `diverge()`
after the `assume(false)` (this is the established pattern for unreachable
table-full branches with load factor < 1). That's 2 easy fixes.

**3. ParaHashTableStEph.rs warnings** (lines 589-677) — 4 fn_missing_wf warnings.
Add real `requires table.spec_hashtable_wf()` to insert, lookup, delete, metrics.
Add `ensures table.spec_hashtable_wf()` to createTable. Read the functions to
confirm these are the right predicates.

### Part B: Chap41 AVLTreeSetStEph assumes (2 holes)

`src/Chap41/AVLTreeSetStEph.rs` lines 1085 and 1352: both are
`assume(new_vec@.len() < usize::MAX)`. These are in `from_vec` or similar
construction functions.

Strategy: the assume says the Vec won't overflow usize. If the input has
bounded length (which it does — it comes from an existing collection), you
can either:
- Add `requires input@.len() < usize::MAX` to the function (propagate the bound)
- Prove it from existing invariants (the input collection is already in memory,
  so its length is already < usize::MAX)

Read the function. Understand where `new_vec` comes from. If it's built by
pushing elements from an existing collection of known size, the bound may be
provable without adding requires.

### Part C: Chap43 AugOrderedTableMtEph (1 hole)

`src/Chap43/AugOrderedTableMtEph.rs` line 669: 1 external_body. Read the
function. If it's an RwLock delegation to a StPer method, it follows the
standard Mt pattern.

### Part D: Chap43 OrderedSet warnings (2 fn_missing_requires)

`OrderedSetStEph.rs` line 1385 and `OrderedSetStPer.rs` line 1157: both on
`from_sorted_elements`. Read the function bodies. If they truly have no
precondition (just builds from Vec), report it. If they assume sorted input
or no duplicates, add the real requires.

### Priority

1. Part A items 2-3: diverge fixes + wf warnings (quick, 2 holes + 4 warnings)
2. Part B: AVLTreeSetStEph assumes (2 holes)
3. Part A item 1: StructChained resize (1 hole, moderate)
4. Part C: AugOrderedTableMtEph (1 hole)
5. Part D: OrderedSet warnings (2 warnings)

### Expected Results

Conservative: 4-5 holes closed + 4-6 warnings fixed.
Optimistic: 6 holes closed + 6 warnings fixed.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r42c-report.md`.

## Continue

Commit early, commit often. Push after each successful validation.
