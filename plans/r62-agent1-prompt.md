# Agent 1 — Round 62

You are Agent 1 working in `~/projects/APAS-VERUS-agent1`.
You are a **senior proof engineer**. Your job is to prove, not to catalog
reasons you can't. Read the code, read the error, write the proof.

## Baseline

- Main: 4496 verified, 0 errors, 11 holes, ~2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Target 1: Close OrderedSetStEph `select` — 1 hole (Chap43, line 1158)

The assume is `assume(spec_seq_sorted(vals))` inside `select`. The backing
AVLTreeSetStEph IS sorted — `spec_elements_sorted` already exists and is
already maintained by every AVL operation (insert, delete, filter,
intersection, difference, union). The infrastructure is there. The gap: it's
not part of `spec_orderedsetsteph_wf`.

### Fix path (clean, mostly mechanical)

1. Read `src/Chap41/AVLTreeSetStEph.rs`:
   - Line 298: `spec fn spec_elements_sorted(&self) -> bool` (trait abstract spec)
   - Line 1231: `open spec fn spec_elements_sorted` (impl)
   - Lines 305-319: insert/delete already `requires old(self).spec_elements_sorted()` and `ensures self.spec_elements_sorted()`
   - Lines 127-143: `empty()` proves `spec_elements_sorted` (vacuously true for empty seq)

2. Read `src/Chap43/OrderedSetStEph.rs`:
   - Line 277: `spec_orderedsetsteph_wf` = `base_set.spec_avltreesetsteph_wf() && obeys_feq_full::<T>()`
   - Line 1158: `assume(spec_seq_sorted(vals))` in select — THIS IS YOUR TARGET

3. **Add `self.base_set.spec_elements_sorted()` to `spec_orderedsetsteph_wf`**:
   ```rust
   open spec fn spec_orderedsetsteph_wf(&self) -> bool {
       self.base_set.spec_avltreesetsteph_wf()
       && self.base_set.spec_elements_sorted()
       && obeys_feq_full::<T>()
   }
   ```

4. Verify. Since every AVLTreeSetStEph operation already maintains sorted
   (requires + ensures), and OrderedSetStEph delegates to AVLTreeSetStEph,
   the wf propagation should be automatic. If any function fails: it's
   because it calls an AVL operation without establishing sorted first.
   Read that AVL function's requires and ensure the caller can satisfy it.

5. **Remove the assume in select** (line 1158). Replace with an assertion
   that `spec_elements_sorted` (which is now in wf) implies
   `spec_seq_sorted(vals)`. Read the definition of `spec_elements_sorted`
   to see if it's literally `spec_seq_sorted(spec_inorder_values(...))` —
   if so, the proof is trivial.

6. Validate: `scripts/validate.sh` — must show 0 errors.

### Watch out for

- Functions that call `construct_from_vec` or `from_seq` — these build new
  sets from raw data and may not establish sorted. They may need a sort step
  or sortedness proof.
- The `tabulate` function — creates a set from a function. May need sorted proof.
- If adding sorted to wf causes cascading failures, DON'T REVERT. Fix forward.
  Each failure tells you which function needs a sorted proof added to its ensures.

## Target 2: Close OrderedSetStPer `select` — 1 hole (Chap43, line 1124)

Same pattern as StEph but for the persistent variant.

1. Read `src/Chap41/AVLTreeSetStPer.rs` — find the sorted infrastructure
   (should mirror StEph: `spec_elements_sorted`, maintained by operations).
2. Read `src/Chap43/OrderedSetStPer.rs` — find the wf predicate and the
   assume in `select`.
3. Add `spec_elements_sorted` to `spec_orderedsetstper_wf`.
4. Remove the assume.
5. Validate.

## Target 3: Chap47 StructChainedHashTable clone bridge — 1 hole (line 201)

You created this assume in R61 as a lateral move from `clone_elem`. The
assume is `assume(v == node.value)` in `chain_lookup`. You noted it's behind
`EntryTrait` which can't carry feq.

Try: can you add `obeys_feq_clone::<V>()` to `chain_lookup`'s own requires
(not EntryTrait's)? The outer `lookup` in ParaHashTableStEph calls
`chain_lookup` directly (not through EntryTrait), so it CAN pass feq. Then
use the same `assert(cloned(*x, c))` pattern from your R61 `clone_elem` fix.

If this doesn't work because `chain_lookup` is also called from other paths,
report exactly which call sites can't satisfy the new requires.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent1-round62-report.md`. Push to `agent1/ready`.
