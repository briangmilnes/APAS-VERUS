# R148 Agent 4 — Traitify AVLTreeSeq in Chap37. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap37/BSTPlainStEph.rs` — the completed example. See how
`BSTPlainNodeFns` trait was created and 7 free functions became trait methods.

Report file: `plans/r148-agent4-chap37-avltreeseq-trait-report.md`

## Problem

Top-level free functions that operate on the AVLTreeSeq types should be
trait methods.

## Files and functions

### AVLTreeSeq.rs (11 fns)

This is the base definition file. Check which functions operate on the
AVLTreeSeq type as first arg. Rotations, rebalance, insert, delete, nth,
update, split, join — all candidates.

### AVLTreeSeqStEph.rs (12 fns)

StEph variant. Same pattern.

### AVLTreeSeqStPer.rs (12 fns)

StPer variant. Same pattern.

### AVLTreeSeqMtPer.rs (11 fns)

MtPer variant. Standalone — duplicates from StPer per standalone rule.

## Pattern

Follow BSTPlainStEph.rs exactly:
1. Create appropriate traits per file
2. Move free functions into trait impls
3. First param → `self` or `&self`
4. Use `let ghost node = self;` for consuming methods
5. Update all call sites

AVLTreeSeq types may use a different tree structure than BalBinTree. Read
the type definitions first. The trait goes on whatever type the free functions
take as first parameter.

## Note on AVLTreeSeq.rs

This is the base file that other variants may import from. If StEph/StPer/MtPer
import functions from AVLTreeSeq.rs, those shared functions should become trait
methods in AVLTreeSeq.rs, and the importing files call them as methods.

However — the standalone rule says Mt files must not import from St counterparts.
Check whether AVLTreeSeqMtPer.rs duplicates functions or imports them. If it
duplicates, make separate traits per file.

## Validation

Run `scripts/validate.sh isolate Chap37`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT change function logic or ensures/requires (except node → self).
- All existing RTTs must pass.

## When done

RCP.
