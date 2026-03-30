# R112 Agent 3 — Chap43 compare-par-mut warning reduction. AFK. PBOGH.

## Objective

Reduce compare-par-mut warnings in Chap43. This chapter has 395 warnings —
over half the codebase total. The warnings mean MtEph/MtPer/StEph variants
have weaker specs than StPer.

## The files

| # | File | Warnings | Compare against |
|---|------|----------|-----------------|
| 1 | OrderedTableMtEph.rs | 84 | OrderedTableStEph.rs |
| 2 | AugOrderedTableMtEph.rs | 79 | AugOrderedTableStEph.rs |
| 3 | OrderedTableStEph.rs | 63 | OrderedTableStPer.rs |
| 4 | OrderedTableMtPer.rs | 56 | OrderedTableStPer.rs |
| 5 | OrderedSetMtEph.rs | 49 | OrderedSetStEph.rs |
| 6 | AugOrderedTableStEph.rs | 37 | AugOrderedTableStPer.rs |
| 7 | OrderedSetStEph.rs | 27 | OrderedSetStPer.rs |

## How to fix

The pattern is mechanical for most warnings:

1. Read the reference variant's trait (e.g., OrderedTableStEph trait).
2. Find the function with missing requires/ensures in the weaker variant.
3. Copy the requires/ensures clauses from the reference into the weaker trait.
4. The impl body may already satisfy the spec (the logic is there, just the
   trait declaration is missing the contract). If not, add proof assertions.
5. Validate with `scripts/validate.sh isolate Chap43`.

## Important distinctions

- **"has requires but X does not"**: The weaker variant has no requires at all.
  Add the requires from the reference. Usually `self.spec_wf()`.
- **"has ensures clause X with no match"**: The weaker variant is missing a
  specific ensures clause. Add it.
- **"missing N fns"**: The weaker variant doesn't have the function at all.
  These are harder — may need to implement the function. Skip these and
  note them in your report.
- **Clause count mismatches**: Usually the weaker variant has fewer clauses.
  Add the missing ones from the reference.

## Work order

1. Start with OrderedSetStEph.rs (27 warnings, smallest file).
2. OrderedSetMtEph.rs (49 warnings).
3. AugOrderedTableStEph.rs (37 warnings).
4. AugOrderedTableMtEph.rs (79 warnings).
5. OrderedTableStEph.rs (63 warnings).
6. OrderedTableMtPer.rs (56 warnings).
7. OrderedTableMtEph.rs (84 warnings).
8. After each file, `scripts/validate.sh isolate Chap43` to confirm clean.
9. Final full `scripts/validate.sh` once at the end.
10. Run `~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS --chapter Chap43`
    and include the summary in your report.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Skip "missing N fns" warnings — don't implement new functions.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent3-r112-chap43-warnings-report.md`. Include warnings
before/after per file.
