# Agent 1 Round 6 — Graph + Set Mt Lock-Boundary (Chap06 + Chap05)

## Mode: AFK — execute relentlessly

Read CLAUDE.md and `src/standards/*.rs` before starting. Do the proof work.
Run `scripts/validate.sh` after each file. Fix errors before moving on.
When done, commit all changes, push to `agent1/ready`, then stop.

## Assignment

**60 holes, all assume. Target: -40.**

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 06 | DirGraphMtEph.rs | 20 | assume (view bridging) |
| 2 | 06 | LabUnDirGraphMtEph.rs | 15 | assume (view bridging) |
| 3 | 06 | LabDirGraphMtEph.rs | 6 | assume (view bridging) |
| 4 | 06 | UnDirGraphMtEph.rs | 10 | assume (view bridging) |
| 5 | 05 | SetMtEph.rs | 9 | assume (RwLock inv) |

## Pattern

All 60 holes follow the lock-boundary pattern: `assume(inner@ == self@)` after
acquire_read, and `assume(result == self@.property)` from inner's ensures.

These are Mt wrappers around verified St implementations. Each method does:
1. `acquire_read` (or `acquire_write`)
2. Call the inner St method
3. Bridge the result from inner's ensures to the Mt wrapper's ensures
4. Release the lock

The assumes bridge the ghost shadow (or lock predicate) to the actual locked value.
The standard pattern is in `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`.

## Strategy

1. Read DirGraphMtEph.rs thoroughly. Understand the struct, View impl, lock predicate.
2. For each assume, determine if it's:
   - A reader accept (value read from lock matches ghost/view) — use `accept()`
   - A provable bridge (inner ensures flow to outer ensures) — prove it
   - A ghost sync (after write, ghost matches new state) — use `accept()`
3. Solve DirGraphMtEph first as template.
4. Replicate the pattern to LabUnDirGraphMtEph, LabDirGraphMtEph, UnDirGraphMtEph.
5. Then SetMtEph.rs (same pattern, different domain).

## Rules

- Do NOT convert `assume` to `accept` wholesale. Read each assume, understand what it
  asserts, try to prove it first. Only use `accept()` for genuine lock-boundary bridges
  that cannot be proved (ghost shadow disconnection).
- Do NOT add new assumes or external_body.
- Do NOT modify files outside your assignment (Chap05, Chap06 only).
- Run `scripts/validate.sh` after each file change.

## Prior Work

Agent 1 Round 4.1 report is in `plans/AGENT1.md`. Key context:
- Chap18/19/12 trivial spec_wf accepted.
- Chap55 conservation law proved.
- Chap52 EdgeSetGraphMtPer blocked on Chap41 filter spec.

## Baseline

3771 verified, 0 errors. Chap06: 51 holes. Chap05: 9 holes.

## Deliverable

When done: commit, push to `agent1/ready`, update `plans/AGENT1.md` with results table.
