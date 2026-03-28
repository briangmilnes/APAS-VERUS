# R101 Agent 2 (continued) — Fix AVLTreeSetMtEph iterator views Seq<T> → Seq<T::V>, STEP 15

## Objective

The main struct View for AVLTreeSetMtEph is already `Set<T::V>` (correct).
But the iterator ghost views still use `Seq<T>` instead of `Seq<T::V>`.
The compare-par-mut tool flags this.

## What to fix

In `src/Chap41/AVLTreeSetMtEph.rs`:

1. **Line 96**: `AVLTreeSetMtEphGhostIter` — `type V = Seq<T>` → `Seq<T::V>`
2. **Line 89**: `AVLTreeSetMtEphIter` — `type V = (int, Seq<T>)` → `(int, Seq<T::V>)`
3. Any `ForLoopGhostIterator::Item` that returns `T` instead of `T::V`
4. Any `next()` ensures referencing `element` instead of `element@`
5. Any `elements` field typed `Seq<T>` instead of `Seq<T::V>`

This is the exact same pattern agent3 R100 fixed in AVLTreeSeqMtPer. Read
that report for the technique: `plans/agent3-r100-avltreeseq-view-report.md`.

## Read first

- `src/Chap41/AVLTreeSetMtEph.rs` — your file
- `plans/agent3-r100-avltreeseq-view-report.md` — exact same fix pattern
- `src/Chap37/AVLTreeSeqMtPer.rs` — the fixed file for reference

## Isolation

```bash
scripts/validate.sh isolate Chap41
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 15

## Report

Append to `plans/agent2-r101-avltreeset-view-report.md`.
