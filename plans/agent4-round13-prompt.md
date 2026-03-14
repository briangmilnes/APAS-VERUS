# Agent 4 — Round 13 Prompt

## Mission

Wide coverage: prove holes across Chap41 Mt files, Chap42 TableMtEph, Chap39,
and Chap47. Hit the easiest hole in each file first, then go deeper.
Target: -15 holes.

## Your Files

**Chap41** (22 holes across 2 files):
- `AVLTreeSetMtEph.rs` — 6 external_body, 2 assume, 2 unsafe_impl (10 total)
- `AVLTreeSetMtPer.rs` — 7 external_body, 5 assume (12 total)

**Chap42** (11 holes):
- `TableMtEph.rs` — 11 external_body (parallel table ops)

**Chap39** (8 holes):
- `BSTTreapMtEph.rs` — 8 assume (lock/view consistency)

**Chap47** (10 holes across 5 files):
- `ParaHashTableStEph.rs` — 4 external_body
- `DoubleHashFlatHashTableStEph.rs` — 2 external_body
- `ChainedHashTable.rs` — 2 external_body
- `LinProbFlatHashTableStEph.rs` — 1 external_body
- `QuadProbFlatHashTableStEph.rs` — 1 external_body

## Priority Order

1. **Chap47** (10 holes) — Simplest. Probe functions are modular arithmetic:
   `(hash + attempt) % size`. Remove external_body, write body, assert
   `result < table_size`. ParaHashTable closure wrappers: call `f(key)`,
   ensures follows from closure ensures. DO THESE FIRST.

2. **Chap42 TableMtEph.rs** (11 holes) — Parallel table ops using join().
   Named closures with explicit ensures. Ghost view captures. Read
   `src/standards/using_closures_standard.rs` and Agent 3's broadcast work.

3. **Chap41 MtEph/MtPer** (22 holes) — Continue from R11. You have momentum.
   The wf chaining technique you used for Chap53 applies to more functions.

4. **Chap39 BSTTreapMtEph** (8 holes) — Lock/view consistency bridge. Try
   strengthening the RwLock invariant as you did for AVLTreeSetMtEph in R10.

## DO NOT TOUCH

- Chap43 — Agents 1 and 2
- Chap41 St files — Agent 3
- Chap38 — Agent 3

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent4/ready`. Write `plans/agent4-round13-report.md`.
- **Prove or move on.** Don't spend more than 10 minutes on any single hole.

## Target: Chap47 10 → 0 (close it). TableMtEph 11 → ≤ 8. Chap41 Mt 22 → ≤ 17. Chap39 8 → ≤ 5. Total -15.
