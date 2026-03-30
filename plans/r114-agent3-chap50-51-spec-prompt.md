# R114 Agent 3 — Chap50 + Chap51 spec strengthening. AFK. PBOGH.

## Objective

Strengthen MtEph/MtPer specs in Chap50 (61 warnings) and Chap51 (20 warnings)
to match their StEph/StPer counterparts. 81 warnings total.

## Chap50 files

| # | Reference | Fix |
|---|-----------|-----|
| 1 | TopDownDPStEph.rs | TopDownDPMtEph.rs |
| 2 | BottomUpDPStEph.rs | BottomUpDPMtEph.rs |
| 3 | MatrixChainStEph.rs | MatrixChainMtEph.rs |
| 4 | OptBinSearchTreeStEph.rs | OptBinSearchTreeMtEph.rs |
| 5 | TopDownDPStPer.rs | TopDownDPMtPer.rs (if exists) |
| 6 | BottomUpDPStPer.rs | BottomUpDPMtPer.rs (if exists) |

## Chap51 files

| # | Reference | Fix |
|---|-----------|-----|
| 7 | AdjTableGraphStEph.rs | AdjTableGraphMtEph.rs |
| 8 | AdjSeqGraphStEph.rs | AdjSeqGraphMtEph.rs |
| 9 | AdjMatrixGraphStEph.rs | AdjMatrixGraphMtEph.rs |
| 10 | AdjTableGraphStPer.rs | AdjTableGraphMtPer.rs (if exists) |

## How to fix

1. Read the StEph/StPer trait. Note every requires and ensures clause.
2. Read the MtEph/MtPer trait. Find missing requires or ensures.
3. Copy the reference clauses into the weaker trait declaration.
4. Validate with `scripts/validate.sh isolate Chap50` then `isolate Chap51`.

## Work order

1. Chap50 files first (61 warnings, bigger impact).
2. Then Chap51 files (20 warnings).
3. After each file: isolate validate.
4. Final: `scripts/validate.sh` once for full crate.
5. Run `scripts/rtt.sh`.
6. Commit.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Skip "missing N fns" warnings — don't implement new functions.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent3-r114-chap50-51-spec-report.md`. Include warnings
before/after per file.
