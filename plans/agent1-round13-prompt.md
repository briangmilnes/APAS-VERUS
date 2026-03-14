# Agent 1 — Round 13 (RESTART)

## You produced ZERO holes in 40 minutes.

You were assigned 50 RwLock ghost-state assumes and produced nothing. You
wandered off to "bonus files" instead of doing the assigned work. That is
unacceptable.

## Your ONE job

Prove the RwLock ghost-state assumes in these TWO files:

1. `Chap43/OrderedSetMtEph.rs` — 36 assume
2. `Chap43/OrderedTableMtPer.rs` — 14 assume

That's it. Do not touch bonus files. Do not touch StEph files. Do not explore.
Do not analyze. PROVE.

## The pattern (you should already know this)

1. Open OrderedSetMtEph.rs
2. Find the `OrderedSetMtEphInv` struct and its `inv` function
3. If `inv` returns `true` or doesn't link ghost to locked value:
   - Change `inv` to: `self.ghost_field@ == v@  && v@.finite()`
   - Update every constructor (new, empty, singleton) to pass the right pred
   - Update every mutator to preserve it
4. Every `assume(self.ghost_X@ == locked_val@)` becomes `assert(...)`
5. Every `assume(self@.finite())` becomes `assert(...)`
6. Run `scripts/validate.sh`
7. Repeat for OrderedTableMtPer.rs

Read `src/Chap41/AVLTreeSetMtEph.rs` — Agent 4 already did this exact pattern
there. Copy the approach.

## DO NOT

- Touch any StEph/StPer files (Agent 2)
- Touch Chap41 (Agent 3)
- Touch Chap42, Chap47 (Agent 4)
- Work on "bonus" files before finishing the primary assignment
- Write paragraphs about why something is hard instead of trying it

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent1/ready`. Write `plans/agent1-round13-report.md`.

## Target: -25 minimum. Prove or explain exactly what you tried.
