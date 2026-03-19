# R42b Agent 3: Revert cfg Shuffle, Prove Graph Algorithms

## Baseline
- Main at `5200ffed`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4320 verified, 153 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**
**DO NOT move functions outside verus!{} to dodge hole counts.**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## What Happened (Why You're Getting This Prompt)

Your R42 commit (`18b64e12`) moved 38 cfg-gated exec functions outside `verus!{}`
and removed `#[verifier::external_body]`. This made veracity stop counting them
as holes — but the code is equally unverified. The hole count dropped from 153
to 115 without any proof work being done. That is not acceptable.

Moving code outside verus!{} to reduce the hole count is the same as deleting a
test to make the test suite pass. The functions are still unverified. The project
convention is: **all algorithm implementations belong inside `verus!{}`**.

## Assignment

### Part A: Revert Your R42 Commit

```bash
git revert HEAD --no-edit
```

This restores the 9 files to their pre-R42 state with functions inside verus!{}
and external_body annotations. Verify: `scripts/validate.sh` must show 4320
verified, 0 errors.

### Part B: Actually Prove Graph Functions

Now do the real work. These files have algorithm implementations inside verus!{}
with `#[verifier::external_body]` AND `#[cfg(not(verus_keep_ghost))]`. For each
function you prove:

1. Remove `#[verifier::external_body]`
2. Remove `#[cfg(not(verus_keep_ghost))]`
3. Write the actual proof (loop invariants, assertions, lemma calls)
4. Validate

**Start with the smallest files and simplest functions.**

#### Tier 1: Simple Delegations (MtEph files)

MtEph files typically delegate to StEph counterparts through RwLock. If the
StEph function is already proved (inside verus with no external_body), the
MtEph version just acquires the lock and calls it.

Check which StEph functions are already proved (inside verus, no external_body,
no cfg gate). For each proved StEph function, the MtEph wrapper should be a
straightforward RwLock delegation.

**BUT**: most of the StEph functions in these graph chapters are ALSO cfg-gated
and unproved. So the MtEph delegation pattern only works if you first prove the
StEph version.

#### Tier 2: Helpers and Small Functions

Look for small helper functions (< 20 lines) in:
- `src/Chap65/PrimStEph.rs`: `pq_entry_new` (6 lines — just struct construction)
- `src/Chap65/PrimStEph.rs`: `mst_weight` (5 lines — iterate and sum)
- `src/Chap65/KruskalStEph.rs`: `mst_weight` (same pattern)
- `src/Chap64/TSPApproxStEph.rs`: `get_neighbors`, `get_edge_weight` (small wrappers)

These may be provable with modest effort.

#### Tier 3: Core Algorithms

The main algorithms (prim_mst, kruskal_mst, star_contract, edge_contract,
connected_components, etc.) are substantial. Each needs:
- Loop invariants maintaining graph properties
- Proof that transformations preserve connectivity/weight properties
- Possibly new lemmas about Set operations on graphs

Read the trait ensures for each function. Understand what needs to be proved.
Start with the function that has the weakest/simplest ensures.

### Priority

1. Revert the R42 commit (mandatory, do first)
2. Prove helper functions in Chap65 (pq_entry_new, mst_weight) — likely 2-4 holes
3. Prove small wrappers in Chap64 (get_neighbors, get_edge_weight) — likely 2 holes
4. Attempt one core algorithm if time remains

### Expected Results

Conservative: Revert + 2-4 real holes proved.
Optimistic: Revert + 6-8 real holes proved.

The hole count after revert will be back to 153. Every hole you close from there
is real proof work.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r42b-report.md`.
