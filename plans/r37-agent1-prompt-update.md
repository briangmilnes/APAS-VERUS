# R37 Agent 1 Update: Remaining Work After MtEph Success

## What You Did

Proved 4 OrderedTableMtEph ordering ops (first_key, last_key, previous_key,
next_key). Good work. rank_key/select_key blocked on missing StEph template.

## New Targets (non-conflicting with running agents 2-4)

Do NOT touch: OrderedTableStEph/StPer (agent2), OrderedSetStEph/StPer or
AugOrderedTableMtEph (agent3), Chap47 or Chap57 (agent4).

### Target 1: BSTParaStEph.rs (Chap38) — 1 algorithmic assume

File: `src/Chap38/BSTParaStEph.rs`
Line 475: `assume(...)` — algorithmic assume in a parallel BST operation.

Read the function, understand what's being assumed, try to prove it from
the existing invariants or by adding an intermediate lemma.

### Target 2: JohnsonStEphI64.rs (Chap59) — 1 algorithmic assume

File: `src/Chap59/JohnsonStEphI64.rs`
Line 437: `assume(reweighted@.A.len() * 2 + 2 <= usize::MAX)`

This bounds the graph size for Dijkstra calls. Check if the graph's
well-formedness or the function's requires already bound `A.len()`.
If so, prove from existing constraints. If not, add a requires clause.

### Target 3: AVLTreeSetStEph.rs (Chap41) — 2 algorithmic assumes

File: `src/Chap41/AVLTreeSetStEph.rs`
Lines 1059, 1334: `assume(new_vec@.len() < usize::MAX)`

You assessed this earlier — fix cascades to 17 files. Try approach:
add `requires self@.len() + 1 < usize::MAX` to the insert functions.
Check if callers already satisfy this (most callers come from Mt modules
that check size). If cascade is manageable, do it. If it touches more
than 5 files, leave it and report.

### Target 4: OrderedSetMtEph.rs filter (Chap43) — 1 external_body

File: `src/Chap43/OrderedSetMtEph.rs`
Line 345: `external_body` on `filter`.

This needs acquire_write → call StEph::filter → release_write with
ghost spec_pred bridging. Read `src/standards/using_closures_standard.rs`
first. Check what OrderedSetStEph::filter looks like.

## Priority

BSTParaStEph (quick win if provable) → JohnsonStEphI64 (single assume) →
AVLTreeSetStEph (high impact if cascade works) → OrderedSetMtEph filter
(hardest, closure specs).

## Rules

- assume() only. NEVER accept().
- Do NOT touch files assigned to agents 2-4 (listed above).
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update your report at `plans/agent1-round37-report.md`.
- Commit, push to `agent1/ready`.
