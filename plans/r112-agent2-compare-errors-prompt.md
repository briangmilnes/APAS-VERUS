# R112 Agent 2 — Fix 5 compare-par-mut errors + Chap50 spec strengthening. AFK. PBOGH.

## Objective

Fix all 5 errors reported by `veracity-compare-par-mut`. These are MtEph
functions with `ensures wf` where StEph has real postconditions. Copy the
StEph ensures into the MtEph trait and prove them.

## The 5 errors

| # | Chap | File | Function | Issue |
|---|------|------|----------|-------|
| 1 | 43 | AugOrderedTableMtEph.rs:147 | `singleton` | MtEph only ensures wf |
| 2 | 50 | MatrixChainMtEph.rs:175 | `set_dimension` | MtEph only ensures wf |
| 3 | 50 | MatrixChainMtEph.rs:179 | `update_dimension` | MtEph only ensures wf |
| 4 | 50 | OptBinSearchTreeMtEph.rs:118 | `set_key_prob` | MtEph only ensures wf |
| 5 | 50 | OptBinSearchTreeMtEph.rs:122 | `update_prob` | MtEph only ensures wf |

## How to fix each one

1. Read the StEph variant's trait to see the real ensures.
2. Read the MtEph variant's trait to see the weak ensures.
3. Copy the StEph ensures into the MtEph trait declaration.
4. Update the MtEph impl body to satisfy the stronger ensures.
   - For RwLock-wrapped Mt modules, the proof typically goes through
     the lock invariant. Read the existing Mt proof pattern in the file.
5. Validate with `scripts/validate.sh isolate ChapNN`.

## Work order

1. Start with Chap43 AugOrderedTableMtEph (1 error, likely quick).
2. Then Chap50 MatrixChainMtEph (2 errors, related functions).
3. Then Chap50 OptBinSearchTreeMtEph (2 errors, related functions).
4. After all 5 fixed, run `scripts/validate.sh` once for full crate.
5. Run `scripts/rtt.sh`.
6. Run `~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS`
   and confirm 0 errors. Include the summary line in your report.
7. Commit.

## Rules

- Do NOT weaken StEph ensures to match MtEph. Strengthen MtEph to match StEph.
- Do NOT add assume or accept.
- Run isolate validates sequentially, not in parallel.
- No subagents.

## STEP 20

## Report

Write `plans/agent2-r112-compare-errors-report.md`.
