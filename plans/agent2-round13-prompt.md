# Agent 2 — Round 13 Prompt

## Mission

Prove 20 of the 53 StEph/StPer external_body holes in Chap43. These are
deterministic AVL tree wrappers — no threading, no concurrency, no excuses.

## Your Files

- `OrderedTableStEph.rs` — 14 external_body
- `OrderedSetStEph.rs` — 14 external_body, 1 assume
- `OrderedSetStPer.rs` — 12 external_body
- `OrderedTableStPer.rs` — 10 external_body
- `AugOrderedTableStEph.rs` — 3 external_body
- `AugOrderedTableStPer.rs` — 2 assume

## The Work

Each external_body wraps an inner AVLTreeSeq operation. The inner operations
(Chap37) have verified specs. Your job:

1. Remove `#[verifier::external_body]`
2. Write the body — usually a single call to the inner AVLTreeSeq method
3. Map the ensures: inner ensures `result.spec_seq()...` → outer ensures
   `result@...`
4. Add view-bridging assertions if needed
5. Validate

## Triage Order (easiest first)

**Batch 1 — Direct delegations (target: all)**:
find, singleton, size, is_empty, first, last, contains

**Batch 2 — One-call with view mapping**:
delete, insert, filter, get_range

**Batch 3 — Two-call compositions**:
split, split_rank, union, intersection, difference

**Batch 4 — Complex (stretch)**:
from_sorted_elements, rank, select, previous, next

Agent 1 proved singleton, delete, iter in R11. Read those proofs for the pattern.

## Also: Chap47 (10 holes, bonus)

If you finish Chap43 StEph/StPer work early, attack Chap47. The probe functions
are modular arithmetic: `(hash + attempt) % size`. Remove external_body, write
the body, assert `result < size`.

## DO NOT TOUCH

- Chap43 Mt files — Agent 1
- Chap41 — Agents 3 and 4
- Chap42 TableMtEph — Agent 4
- Chap38, Chap39 — Agents 3 and 4

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent2/ready`. Write `plans/agent2-round13-report.md`.
- **Prove or move on.** Don't spend more than 10 minutes on any single hole.

## Target: Chap43 St/StPer 53 → ≤ 33 (-20). Chap47 10 → ≤ 8 (-2).
