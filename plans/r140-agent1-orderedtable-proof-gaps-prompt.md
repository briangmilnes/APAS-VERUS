# R140 Agent 1 — Finish 5 proof gaps in OrderedTable BST helpers. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `plans/r138-agent1-orderedtable-remaining-report.md` — your R138 report.
Read `src/Chap43/OrderedTableStEph.rs` — the file you modified in R138.

Report file: `plans/r140-agent1-orderedtable-proof-gaps-report.md`

## Problem

In R138 you wrote 5 O(lg n) BST helper functions for OrderedTable. Two verify
(`bst_next_by_key`, `bst_prev_by_key`). Three have proof gaps:

1. `bst_split_by_key` — 2 errors: found-value postcondition, size bound
2. `bst_rank_by_key` — 3 errors: equal arm filter, greater arm disjoint-union
3. `bst_select_by_rank` — 2 errors: filter equivalence in both arms

## What to do

Fix the 7 verification errors in these 3 functions. Your R138 report describes
the specific failures. Read the Verus errors carefully — they tell you exactly
which postconditions fail.

Key tools available:
- `reveal_param_bst_backings` — exposes BST type invariant for view witnesses
- `TotalOrder::cmp_spec_less_implies_le` / `cmp_spec_greater_implies_le` — bridges
- `spec_set_pair_view_generated` in wf — gives concrete Pair preimages

For the filter/domain proofs in rank and select: the issue is connecting
`tree.view().dom().filter(pred)` to the left/right subtree domains. You need
to instantiate the filter quantifier with concrete Pair witnesses from the BST
backing. The `reveal_param_bst_backings` + `use_type_invariant` pattern gives you
the witnesses. Then assert extensional equality (`=~=`) on the filtered sets.

## Also: OrderedTableStPer

If time permits after fixing StEph, apply the same 7 functions to
`src/Chap43/OrderedTableStPer.rs`. Copy the helpers, adjust types.
Also fix `first_key_iter` and `last_key_iter` in StPer (agent4 fixed these
in StEph in R137 but not in StPer).

If time is short, do StEph only.

## Validation

Run `scripts/validate.sh isolate Chap43`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Every fix must maintain O(lg n).
- Use the infrastructure from R137/R138 (TotalOrder bridges, reveal_param_bst_backings).

## When done

RCP.
