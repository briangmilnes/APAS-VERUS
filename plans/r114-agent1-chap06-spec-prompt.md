# R114 Agent 1 — Chap06 spec strengthening (StEph → MtEph). AFK. PBOGH.

## Objective

Strengthen MtEph specs in Chap06 to match their StEph counterparts. The
compare-par-mut tool reports 63 warnings here — MtEph graph variants have
weaker requires/ensures than StEph.

## The files

Read each StEph file, then strengthen the corresponding MtEph:

| # | StEph (reference) | MtEph (fix) |
|---|-------------------|-------------|
| 1 | DirGraphStEph.rs | DirGraphMtEph.rs |
| 2 | LabDirGraphStEph.rs | LabDirGraphMtEph.rs |
| 3 | LabUnDirGraphStEph.rs | LabUnDirGraphMtEph.rs |
| 4 | UnDirGraphStEph.rs | UnDirGraphMtEph.rs |

## How to fix

1. Read the StEph trait. Note every requires and ensures clause per function.
2. Read the MtEph trait. Find functions missing requires or ensures.
3. Copy the StEph clauses into the MtEph trait declaration.
4. The MtEph impl acquires a lock, calls the inner method, releases.
   The ensures should flow through. If not, add proof assertions.
5. Validate with `scripts/validate.sh isolate Chap06`.

## Work order

1. DirGraphMtEph.rs (likely most warnings — most functions).
2. UnDirGraphMtEph.rs.
3. LabDirGraphMtEph.rs.
4. LabUnDirGraphMtEph.rs.
5. After each file: `scripts/validate.sh isolate Chap06`.
6. Final: `scripts/validate.sh` once for full crate.
7. Run `scripts/rtt.sh`.
8. Commit.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Skip "missing N fns" warnings — don't implement new functions.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent1-r114-chap06-spec-report.md`. Include warnings
before/after per file.
