# R112 Agent 2 — Chap37 compare-par-mut warning reduction. AFK. PBOGH.

## Objective

Reduce compare-par-mut warnings in Chap37 (145 warnings, second highest
after Chap43). The warnings mean MtEph variants have weaker specs than
their StEph counterparts.

## The files

| # | File | Warnings | Compare against |
|---|------|----------|-----------------|
| 1 | BSTPlainMtEph.rs | 30 | BSTPlainStEph.rs |
| 2 | BSTBBAlphaMtEph.rs | 30 | BSTBBAlphaStEph.rs |
| 3 | BSTSplayMtEph.rs | 29 | BSTSplayStEph.rs |
| 4 | BSTAVLMtEph.rs | 21 | BSTAVLStEph.rs |
| 5 | BSTRBMtEph.rs | 20 | BSTRBStEph.rs |
| 6 | AVLTreeSeqStEph.rs | 9 | AVLTreeSeqStPer.rs |
| 7 | AVLTreeSeqMtPer.rs | 6 | AVLTreeSeqStPer.rs |

## How to fix

The 5 BST*MtEph files share the same pattern — they wrap StEph trees in
RwLock. The fix is mechanical:

1. Read the StEph trait to see the real requires/ensures.
2. Copy them into the MtEph trait declaration.
3. The MtEph impl acquires the lock, calls the StEph method, and releases.
   The ensures should flow through from the StEph call.
4. Validate with `scripts/validate.sh isolate Chap37`.

The BST files are similar to each other. Once you fix one (start with
BSTPlainMtEph), the pattern applies to all five.

## Work order

1. BSTPlainMtEph.rs (30 warnings — template for the rest).
2. BSTBBAlphaMtEph.rs (30 warnings).
3. BSTAVLMtEph.rs (21 warnings).
4. BSTRBMtEph.rs (20 warnings).
5. BSTSplayMtEph.rs (29 warnings).
6. AVLTreeSeqStEph.rs (9 warnings) — different pattern, StEph vs StPer.
7. AVLTreeSeqMtPer.rs (6 warnings).
8. After each file or pair, `scripts/validate.sh isolate Chap37`.
9. Final full `scripts/validate.sh` once at the end.
10. Run `~/projects/veracity/target/release/veracity-compare-par-mut -c ~/projects/APAS-VERUS --chapter Chap37`
    and include the summary in your report.

## Rules

- Do NOT weaken ensures. Only strengthen.
- Do NOT add assume or accept.
- Skip "missing N fns" warnings — don't implement new functions.
- Run validates sequentially.
- No subagents.

## STEP 30

## Report

Write `plans/agent2-r112-chap37-warnings-report.md`. Include warnings
before/after per file.
