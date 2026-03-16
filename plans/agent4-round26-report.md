# Agent 4 — Round 26 Report

## Summary

Closed 3 holes across 2 chapters. Attempted 4th hole (insert assume) but reverted
due to cascade across 5 chapters / 11 callers.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 41 | AVLTreeSetStEph.rs | 7 | 6 | -1 |
| 2 | 41 | AVLTreeSetStPer.rs | 6 | 5 | -1 |
| 3 | 45 | BinaryHeapPQ.rs | 2 | 1 | -1 |

## Verification State

- 4109 verified, 0 errors, 0 warnings
- 214 total holes (was 216, -2 net from this agent's work since R25 baseline)
- 208 clean modules, 49 holed
- 2613 RTT pass, 147 PTT pass

## Techniques Used

### AVLTreeSetStEph filter (-1 hole, Chap41)
Removed `#[verifier::external_body]` from filter. Three-invariant pattern:
1. Closure contract carried through loop isolation (`f.requires`, `f.ensures`)
2. Elements in filtered satisfy predicate (`filtered@.contains(v) ==> spec_pred(v)`)
3. Completeness for seen elements (`0 <= j < i && spec_pred(seq[j]) ==> filtered@.contains(seq[j])`)

Post-loop: completeness via `seq@.to_set().contains(v)` → `seq@.contains(v)` → witness j → invariant.

### AVLTreeSetStPer filter (-1 hole, Chap41)
Same pattern as StEph but with persistent insert (`filtered = filtered.insert(c)` returns new value).

### BinaryHeapPQ find_min (-1 hole, Chap45)
Removed `#[verifier::external_body]`. Added:
- `spec_exec_heap_inv_at(seq: Seq<T>, i)`: parent <= children using `TotalOrder::le`
- `spec_is_exec_heap(seq: Seq<T>)`: all positions satisfy exec heap invariant
- `lemma_heap_root_le_all(seq, i)`: root <= element i by induction on parent chain
  - Base (i=0): `T::reflexive(seq[0])`
  - Step (i>0): IH on parent p = (i-1)/2, heap inv gives `le(seq[p], seq[i])`,
    `T::transitive(seq[0], seq[p], seq[i])`
- Added `requires Self::spec_is_exec_heap(self.spec_seq())` to find_min trait
- No cascade: find_min has zero algorithm-file callers (tests only)

### Insert assume — reverted

Adding `old(self)@.len() + 1 < usize::MAX as int` to insert's requires cascades to:
- Internal: from_seq, filter, intersection, difference need `len <= i` invariants
- Union: needs `self@.len() + other@.len() < usize::MAX` requires (new trait constraint)
- External: OrderedSetStEph, OrderedSetMtEph, AVLTreeSetMtEph, GraphSearchStEph, PQMinStEph

11 errors across Chap41/43/53. Reverted. The insert assume stays as a known hole.
Root cause: AVLTreeSeqStEph's `from_vec` requires `len < usize::MAX`, and insert's
not-found path creates vec of size n+1. Tree wf gives n < usize::MAX but not n+1 < usize::MAX.

## Remaining Holes

### AVLTreeSetStEph.rs (Chap41, 6 holes)
- 1 assume: insert `new_vec@.len() < usize::MAX` (cascade blocker)
- 5 accept: size bridge, feq axioms, vec-len bounds, wf-through-clone

### AVLTreeSetStPer.rs (Chap41, 5 holes)
- 5 accept: feq axioms, insert found-case, filter subset, wf-through-clone

### BinaryHeapPQ.rs (Chap45, 1 hole)
- 1 assume: `extract_all_sorted` sortedness (needs heap invariant propagation through delete_min loop)
