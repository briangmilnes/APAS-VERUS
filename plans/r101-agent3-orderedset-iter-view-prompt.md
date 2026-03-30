# R101 Agent 3 (continued) — Fix OrderedSet iterator views, STEP 15

## Objective

Check OrderedSetStPer and OrderedSetMtEph for the same iterator view issue:
ghost iterator views using `Seq<T>` or `T` instead of `Seq<T::V>` or `T::V`.

Also check OrderedSetStEph for consistency.

## Method

For each file in `src/Chap43/OrderedSet{StEph,StPer,MtEph}.rs`:

1. Check main struct View — should be `Set<T::V>` (or correct type)
2. Check iterator ghost views — should use `T::V` not `T`
3. Check `ForLoopGhostIterator::Item` — should be `T::V`
4. Check `next()` ensures — `element@` not `element`
5. Fix any mismatches, same pattern as R100 AVLTreeSeqMtPer fix

If all three files are already consistent, report that and move on to
checking `src/Chap43/OrderedTable{StPer,MtPer}.rs` for the same issue.

## Read first

- `src/Chap43/OrderedSetStPer.rs`
- `src/Chap43/OrderedSetMtEph.rs`
- `src/Chap43/OrderedSetStEph.rs`
- `plans/agent3-r100-avltreeseq-view-report.md` — the fix pattern

## Isolation

```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 15

## Report

Append to `plans/agent3-r101-orderedset-view-report.md`.
