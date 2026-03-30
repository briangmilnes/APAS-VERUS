# R114 Agent 4 — Chap38 + Chap39 + Chap52 spec strengthening. AFK. PBOGH.

## Objective

Strengthen specs in Chap38 (29 warnings), Chap39 (32 warnings), and
Chap52 (34 warnings). 95 warnings total across three chapters.

## Chap38 files

| # | Reference | Fix |
|---|-----------|-----|
| 1 | BSTParaStEph.rs | BSTParaMtEph.rs |
| 2 | BSTParaStPer.rs | BSTParaMtPer.rs (if exists) |

## Chap39 files

| # | Reference | Fix |
|---|-----------|-----|
| 3 | BSTRankedParaStEph.rs | BSTRankedParaMtEph.rs |
| 4 | BSTRankedParaStPer.rs | BSTRankedParaMtPer.rs (if exists) |

## Chap52 files

| # | Reference | Fix |
|---|-----------|-----|
| 5 | EdgeSetGraphStEph.rs | EdgeSetGraphMtEph.rs |
| 6 | AdjTableGraphStEph.rs (from Chap51) | AdjTableGraphMtEph.rs |
| 7 | AdjSeqGraphStEph.rs (from Chap51) | AdjSeqGraphMtEph.rs |
| 8 | EdgeSetGraphStPer.rs | EdgeSetGraphMtPer.rs (if exists) |

Note: Chap52 may import graph types from Chap51. Read imports carefully.

## How to fix

1. Read the StEph/StPer trait. Note every requires and ensures clause.
2. Read the MtEph/MtPer trait. Find missing requires or ensures.
3. Copy the reference clauses into the weaker trait declaration.
4. Validate with isolate mode for each chapter.

## Work order

1. Chap52 first (34 warnings, graph modules).
2. Chap39 (32 warnings).
3. Chap38 (29 warnings).
4. After each chapter: `scripts/validate.sh isolate ChapNN`.
5. Final: `scripts/validate.sh` once for full crate.
6. Run `scripts/rtt.sh`.
7. Commit.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Skip "missing N fns" warnings — don't implement new functions.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent4-r114-chap38-39-52-spec-report.md`. Include warnings
before/after per file.
