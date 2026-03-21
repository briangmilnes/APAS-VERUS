# Agent 2 — Round 54: Chap41 AVLTreeSetStEph + Chap43 critical path

## Goal

Close the Chap41 hole that blocks 4 downstream chapters (Chap43, 52, 53, 55).
Then attack Chap43 holes.

## Priority 1: src/Chap41/AVLTreeSetStEph.rs (1 hole)

**Hole**: `assume(combined@.len() + 1 < usize::MAX as nat)` at line 722, inside `union()`.

**Problem**: `self` and `other` each satisfy `spec_avltreesetsteph_wf()` which bounds
their individual lengths at `< usize::MAX`. But the union loop inserts elements from both
into `combined`, so `combined@.len()` could approach `2 * usize::MAX` which overflows.

**Fix approach**: Add a `requires` clause to `union` bounding the combined size:
```
requires self@.len() + other@.len() <= usize::MAX
```
or equivalently:
```
requires self.elements.length() + other.elements.length() <= usize::MAX
```

This propagates the capacity bound from callers. Then the assume becomes provable from
the loop invariant (combined grows by at most 1 per iteration, and starts from self which
is already bounded).

**Check callers**: After adding the requires, verify that all callers of `union` can
satisfy it. Search for `.union(` in Chap41 and downstream files (Chap43 OrderedSet,
Chap53 PQMinStEph). If any caller can't prove the bound, you may need to add the same
requires there.

**Important**: The trait definition in the same file must also get the requires clause
updated (specs live in the trait, not the impl).

## Priority 2: src/Chap43/OrderedTableMtPer.rs (1 hole)

**Hole**: `assume(len < usize::MAX)` at line 321. Capacity bound in `insert`.

Same pattern as above — add a requires clause bounding size.

## Priority 3: src/Chap43/OrderedSetStPer.rs (1 hole)

**Hole**: `assume(self@.filter(...))` at line 1031. This is about proving a filter property
on ordered sets.

Read the assume carefully. It may need a lemma connecting `TotalOrder::le` to set filtering.

## Priority 4: src/Chap43/AugOrderedTableStPer.rs (1 hole)

**Hole**: `assume(forall|v1: &V, v2: &V| cloned.requires(...))` at line 124 in
`lemma_reducer_clone_total`. This is a closure-requires hole — read
`src/standards/using_closures_standard.rs` before attempting.

## Rules

- Read `src/standards/capacity_bounds_standard.rs` for the capacity bound pattern.
- Read `src/standards/using_closures_standard.rs` before touching AugOrderedTableStPer.
- Do NOT add `accept()` or weaken ensures. Prove or leave the hole.
- Do NOT modify files outside Chap41/Chap43.
- Validate after each file. Fix trigger warnings.
- Write `plans/agent2-round54-report.md` when done.
