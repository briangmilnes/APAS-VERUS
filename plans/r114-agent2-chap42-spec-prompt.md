# R114 Agent 2 — Chap42 spec strengthening (StEph → MtEph). AFK. PBOGH.

## Objective

Strengthen MtEph specs in Chap42 to match their StEph counterparts.
55 compare-par-mut warnings — Table and ArraySet MtEph variants have
weaker requires/ensures than StEph.

## The files

| # | StEph (reference) | MtEph (fix) |
|---|-------------------|-------------|
| 1 | TableStEph.rs | TableMtEph.rs |
| 2 | ArraySetStEph.rs | ArraySetMtEph.rs |

Also check StPer → MtPer if those exist:

| # | StPer (reference) | MtPer (fix) |
|---|-------------------|-------------|
| 3 | TableStPer.rs | TableMtPer.rs (if exists) |

## How to fix

1. Read the StEph trait. Note every requires and ensures clause per function.
2. Read the MtEph trait. Find functions missing requires or ensures.
3. Copy the StEph clauses into the MtEph trait declaration.
4. The MtEph impl acquires a lock, calls the inner method, releases.
   The ensures should flow through. If not, add proof assertions.
5. Validate with `scripts/validate.sh isolate Chap42`.

## Work order

1. TableMtEph.rs (larger module, more warnings).
2. ArraySetMtEph.rs.
3. TableMtPer.rs if it exists.
4. After each file: `scripts/validate.sh isolate Chap42`.
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

Write `plans/agent2-r114-chap42-spec-report.md`. Include warnings
before/after per file.
