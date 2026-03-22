<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 55 Prompt

## Branch

Work on `agent3/ready`. Base: `045bf2ce9`.

## DO NOT TOUCH

- Chap47 (any file)
- Chap41/AVLTreeSetStEph.rs (Agent 2 is working there)
- Chap41/AVLTreeSetMtEph.rs
- Chap43/AugOrderedTableMtEph.rs (Agent 4)
- Chap43/AugOrderedTableStPer.rs (Agent 4)
- Chap43/AugOrderedTableStEph.rs (Agent 4)

## Assignment: Capacity chain Chap37 → Chap41 → Chap43 (close 1 hole)

### Goal

Close the `assume(len < usize::MAX)` hole in `src/Chap43/OrderedTableMtPer.rs:316`
(the `domain` method). This requires adding a capacity bound to the wf chain from
Chap37 through Chap41 to Chap43.

### Step 1: Add capacity bound to Chap37/AVLTreeSeqStPer.rs

**File:** `src/Chap37/AVLTreeSeqStPer.rs`

The `spec_avltreeseqstper_wf` predicate currently does NOT include a capacity bound.
The StEph version (`spec_avltreeseqsteph_wf` in Chap37/AVLTreeSeqStEph.rs) DOES include
`node.left_size + node.right_size + 1 < usize::MAX`. Mirror this pattern.

Read the StEph version first to see exactly how the capacity bound is stated, then add
an equivalent bound to the StPer wf predicate.

After adding the bound, you may need to prove it is maintained by `insert`, `delete`,
and other operations. These operations likely already have the arithmetic — the bound
just needs to be part of the wf spec so callers can see it.

Run `scripts/validate.sh` after this step. Fix any cascading failures (operations that
now need to prove they maintain the bound).

### Step 2: Propagate through Chap41/AVLTreeSetStPer.rs

**File:** `src/Chap41/AVLTreeSetStPer.rs`

The `spec_avltreesetstper_wf` predicate wraps the Chap37 avltreeseq wf. Once Chap37
has the capacity bound, this should automatically flow through. Verify that
`spec_avltreesetstper_wf` implies `self@.len() < usize::MAX` or similar.

If needed, add an explicit capacity bound to `spec_avltreesetstper_wf`.

### Step 3: Propagate through Chap43 and close the hole

**File:** `src/Chap43/OrderedTableStPer.rs`

Verify that `spec_orderedtablestper_wf` now implies a capacity bound through the chain.

**File:** `src/Chap43/OrderedTableMtPer.rs`

The `domain` method at line 316 has `assume(len < usize::MAX)`. Once the wf chain
includes the capacity bound, this assume should be replaceable with a proof assertion.

The domain method acquires a read lock, gets the inner table, calls `collect()` to get
entries, then iterates inserting keys into an `OrderedSetMtEph`. The `len` is the number
of entries. If the wf predicate guarantees `len < usize::MAX`, the assume can be removed.

Remove the `assume(len < usize::MAX)` and prove the bound from the wf chain.

### Bonus: Add sortedness ensures to AVLTreeSetStPer.rs

If time permits after closing the capacity hole, add `spec_elements_sorted` ensures to
`filter`, `intersection`, `union`, `difference` in `AVLTreeSetStPer.rs` (mirroring what
Agent 2 is doing for StEph). This unblocks Chap43 OrderedSetStPer select in a future round.

## Validation

Run `scripts/validate.sh` after each step. Show full output. Fix all warnings.
Do not leave trigger warnings.

## Report

Write `plans/agent3-round55-report.md` with holes before/after table including
Chap column. Document each step of the capacity chain propagation.
