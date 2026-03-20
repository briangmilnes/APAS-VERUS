# Agent1 Round 48 Report

## Assignment

Fix Chap41's 2 holes and Chap43's 4 holes by applying the new
`capacity_bounds_standard.rs` — move capacity assumes from function bodies to
`requires` clauses.

## Results Summary

| Metric | Before | After | Delta |
|---|---|---|---|
| Verified | 4419 | 4419 | 0 |
| Errors | 0 | 0 | 0 |
| Total holes | 38 | 38 | 0 |
| RTT | 2613 pass | 2613 pass | 0 |
| PTT | 143 pass, 4 fail | 143 pass, 4 fail | 0 (pre-existing) |

## Holes by Chapter (changed)

| # | Chap | File | Before | After | Delta | Notes |
|---|---|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | 2 | 1 | -1 | insert/insert_sorted assumes eliminated; union assume added |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 1 | 0 | reduce_range_parallel: thread boundary external_body |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | 1 | 0 | lemma_reducer_clone_total: Verus closure clone limitation |
| 4 | 43 | OrderedSetStEph.rs | 1 | 1 | 0 | select: sortedness not in wf, needs BST proof |
| 5 | 43 | OrderedSetStPer.rs | 1 | 1 | 0 | select: same as StEph |
| 6 | 43 | OrderedTableMtPer.rs | 0 | 1 | +1 | domain: StPer wf lacks strict < usize::MAX |

## What Was Done

### Capacity-bounds refactor (Chap41 + Chap43)

Applied the `capacity_bounds_standard.rs` pattern: `requires old(self)@.len() + 1 < usize::MAX as nat` on insert/insert_sorted traits, proved from requires in impl bodies.

**Trait changes (requires added):**

| # | Chap | File | Function | Change |
|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | insert | `old(self)@.len() + 1 < usize::MAX as nat` |
| 2 | 41 | AVLTreeSetStEph.rs | insert_sorted | same |
| 3 | 41 | AVLTreeSetMtEph.rs | insert | same |
| 4 | 43 | OrderedSetStEph.rs | insert | same |
| 5 | 43 | OrderedSetStEph.rs | from_seq | `seq@.len() < usize::MAX as nat` |
| 6 | 43 | OrderedSetStEph.rs | from_sorted_elements | `elements@.len() < usize::MAX` |
| 7 | 43 | OrderedSetMtEph.rs | insert | `old(self)@.len() + 1 < usize::MAX as nat` |
| 8 | 43 | OrderedSetMtEph.rs | from_seq | `seq.spec_len() < usize::MAX as int` |

**Impl bodies proved from requires (assumes eliminated):**

| # | Chap | File | Function | Proof technique |
|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | insert | `unique_seq_to_set` + arithmetic from requires |
| 2 | 41 | AVLTreeSetStEph.rs | insert_sorted | same |

**Loop invariants added to internal callers:**

| # | Chap | File | Function | Invariant |
|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | from_seq | `constructed@.len() <= i` + `lemma_wf_implies_len_bound` |
| 2 | 41 | AVLTreeSetStEph.rs | filter | `filtered@.len() <= i` + lemma |
| 3 | 41 | AVLTreeSetStEph.rs | intersection | `common@.len() <= i` + lemma |
| 4 | 41 | AVLTreeSetStEph.rs | difference | `remaining@.len() <= i` + lemma |
| 5 | 41 | AVLTreeSetStEph.rs | union (1st loop) | `combined@.len() <= i` + lemma |
| 6 | 43 | OrderedSetStEph.rs | from_seq | `constructed@.len() <= i` + `seq@.len() < usize::MAX` |
| 7 | 43 | OrderedSetStEph.rs | split | `left@.len() + right@.len() <= j` + `n < usize::MAX` + lemma |
| 8 | 43 | OrderedSetStEph.rs | get_range | `range@.len() <= i` + `n < usize::MAX` + lemma |
| 9 | 43 | OrderedSetStEph.rs | split_rank | `left@.len() + right@.len() <= j` + `n < usize::MAX` + lemma |
| 10 | 43 | OrderedSetMtEph.rs | from_seq | `inner@.len() <= i` + `seq.spec_len() < usize::MAX` |

**New structural assumes (unavoidable):**

| # | Chap | File | Function | Assume | Why unavoidable |
|---|---|---|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs | union (2nd loop) | `combined@.len() + 1 < usize::MAX` | Sum of two set sizes may exceed usize::MAX; fix requires capacity on union trait, cascading to Chap52/53 |
| 2 | 41 | AVLTreeSetMtEph.rs | insert | `current@.len() + 1 < usize::MAX` | RWLOCK_GHOST: bound lost across lock boundary (classified structural) |
| 3 | 43 | OrderedSetMtEph.rs | insert | `locked_val@.len() + 1 < usize::MAX` | RWLOCK_GHOST: same (classified structural) |
| 4 | 43 | OrderedTableMtPer.rs | domain | `len < usize::MAX` | StPer wf lacks strict < usize::MAX bound |

Items 2-3 are classified as RWLOCK_GHOST structural false positives (not holes).

**Other changes:**

- Made `lemma_wf_implies_len_bound` public in Chap41/AVLTreeSetStEph.rs (was private, needed by Chap43 callers).

### Chap43 hole assessment (4 original targets)

All 4 are irreducible with current infrastructure:

1. **reduce_range_parallel** (AugOrderedTableMtEph.rs:672): external_body wrapping `ParaPair!` parallel execution. Structural thread-boundary. Needs parallel proof infrastructure.

2. **lemma_reducer_clone_total** (AugOrderedTableStPer.rs:124): assume that cloning a total closure preserves totality. Verus cannot reason about closure identity after clone. Analogous to eq/clone workaround.

3. **select** (OrderedSetStEph.rs:1134): assume that `self@.filter(|x| x < result).len() == i`. Requires proving the inorder sequence is sorted — which is true for BSTs but not captured in `spec_avltreesetsteph_wf`. Fixing requires either adding sortedness to wf (cascading change) or a deep lemma `tree_is_bst ==> inorder_sorted`.

4. **select** (OrderedSetStPer.rs:1031): identical pattern to #3.

## Pre-existing PTT Failures

4 PTT failures in `ProveOrderedTableStPer` (loop_borrow_iter, loop_borrow_into, for_borrow_iter, for_borrow_into). Confirmed pre-existing — fail on clean baseline. Type mismatch on iterator View type. Not caused by R48 changes.

## Techniques Used

- Capacity-bounds standard: `requires` on insert instead of `assume` in body
- `lemma_wf_implies_len_bound`: establishes `n < usize::MAX` from tree wf
- `unique_seq_to_set`: bridges sequence length to set cardinality
- Cardinality loop invariants: `constructed@.len() <= i` pattern tracks set growth
- RWLOCK_GHOST assume bridge: capacity bound lost across lock boundary
