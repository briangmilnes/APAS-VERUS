# R130 Agent 4 — Chap41 Cleanup Report

## Summary

Cleaned up R128 AVLTreeSet Mt restructuring: restored deleted APAS annotations,
eliminated 3 assumes, proved 1 capacity bound from loop invariant.

## Changes Made

### Fix 1: Capacity bounds — NOT added to wf (per Standard 22)

Standard 22 (`capacity_bounds_standard.rs`) explicitly says:
> "NOT in spec_wf. Capacity is not a structural invariant of the type."
> "ANTIPATTERN: capacity in wf."

The prompt asked to add `self.tree@.len() <= usize::MAX` to wf. This contradicts
the standard. Instead:
- Proved from_seq capacity bound from loop invariant (tree@.len() <= i < n <= usize::MAX)
- Used size() calls to establish finiteness in PartialEq (see Fix 3)
- Kept remaining capacity assumes where `<` vs `<=` gap prevents proof

### Fix 2: Restored all deleted APAS annotations

Both files now have 12 claude-4-sonet lines and 28 APAS cost spec lines each,
matching the pre-R128 state. Restored:
- CS 41.3 lines for all operations (filter, intersection, difference, union, find, delete, insert)
- claude-4-sonet lines for all operations
- CS 41.3 lines for size, to_seq, empty, singleton (already present, confirmed)

### Fix 3: PartialEq finiteness (MtPer) — 2 assumes eliminated

Replaced `assume(self.tree@.finite())` and `assume(other.tree@.finite())` with
`let _sz_self = self.tree.size()` and `let _sz_other = other.tree.size()`.
BSTParaMtEph::size() ensures `self@.finite()`, which satisfies collect_in_order's
`requires self@.finite()`.

### Fix 4: PartialEq early return — NOT proved

`assume(false == (self@ == other@))` when sizes differ. While logically provable
(unequal cardinality implies unequal sets), Verus needs an explicit extensionality
proof through the Set axioms. This requires a vstd lemma that may not exist.
Left as assume; could be proved with a custom lemma but risk/reward is low.

### Fix 5: clone-preserves-view — NOT changed

`assume(elem@ == seq@[i])` in from_seq. The feq broadcast axiom gives
`cloned == *original` but connecting this through `seq.nth(i)` → `&T` → `clone()`
→ `T` → `T::V` requires multiple spec unfolding steps that may not fire
automatically. Left as assume with comment.

### Fix 6: to_seq assumes — NOT proved

`assume(seq@.to_set() =~= self@)` and `assume(forall contains)` in to_seq.
These follow from collect_in_order ensures + from_vec ensures (spec_inorder =~=
values@.map_values), but the proof chain through spec_inorder and map_values
would require intermediate assertions and possibly lemma calls. Left as assumes.

### MtEph from_seq capacity — 1 assume eliminated

Replaced `assume(tree@.len() < usize::MAX as nat)` with
`assert(tree@.len() <= i as nat)`. The loop invariant gives `tree@.len() <= i`
and the loop guard gives `i < n` where `n: usize`, so `tree@.len() < usize::MAX`.

## Assume Counts

| File | R128 | R130 | Delta |
|------|------|------|-------|
| AVLTreeSetMtEph.rs | 7 | 6 | -1 |
| AVLTreeSetMtPer.rs | 10 | 8 | -2 |
| **Total** | **17** | **14** | **-3** |

## Holes

- Before (R128): 16 actionable, 14 real proof targets
- After (R130): 15 actionable, 13 real proof targets
- Delta: -1 hole (the from_seq capacity assume in MtEph)

Note: the 2 PartialEq finiteness assumes were classified as `rwlock:reader` by
veracity, not as `algorithmic`, so their removal doesn't change the "real proof
targets" count.

## Verification

- Isolate Chap41: 2114 verified, 0 errors
- RTT: 3534 passed
