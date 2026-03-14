# Agent 4 — Round 10 Report

## Summary

Reduced Chap41 from 54 to 44 holes (-10). Chap53 remains at 1 hole (external_body replaced
with verified code + 1 assume for seq wf — net 0 change but structural proof now verified).

**Verification:** 3934 verified, 0 errors. **RTT:** 2600 pass.

## Per-File Changes

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 41 | AVLTreeSetMtEph.rs | 19 | 10 | -9 | Ghost field for set view |
| 2 | 41 | AVLTreeSetStEph.rs | 8 | 7 | -1 | Filter closure requires |
| 3 | 53 | GraphSearchMtPer.rs | 1 | 1 | 0 | Iterative rewrite |
| 4 | 41 | ArraySetStEph.rs | 3 | 3 | 0 | Type axioms, left as-is |
| 5 | 41 | AVLTreeSetStPer.rs | 6 | 6 | 0 | Blocked by same issues |
| 6 | 41 | AVLTreeSetMtPer.rs | 12 | 12 | 0 | Not attempted |
| 7 | 41 | Example41_3.rs | 4 | 4 | 0 | User said ignore |
| 8 | 41 | ArraySetEnumMtEph.rs | 1 | 1 | 0 | Bit macro filter |

## Techniques Used

### Ghost Field for RwLock View (AVLTreeSetMtEph.rs: 19 → 10)

Added `ghost_set_view: Ghost<Set<V>>` to AVLTreeSetMtEph struct (matching BSTTreapMtEph
pattern). This field tracks the set view independently of the Arc<RwLock>.

- **spec_set_view**: Returns `self.ghost_set_view@` instead of external_body placeholder.
  Removed external_body (-1).
- **wf**: Changed from trivial `true` to `self.ghost_set_view@.finite()`.
  Removed trivial_wf (-1).
- **empty/singleton/from_seq**: Set ghost field from StEph's view at construction.
  Proves view and finite from StEph ensures. Removed 4 assumes (-4).
- **insert/delete**: Update ghost field after write-lock release.
  `self.ghost_set_view = Ghost(old_view.insert(x_view))` proves view ensures.
  Removed 4 assumes (-4).
- **size**: `self@.finite()` now follows from wf. Removed 1 assume (-1).
- **Clone**: Copies ghost field. Removed accept (-1 accept, not counted as hole but cleaner).
- **unsafe Send/Sync**: Required because `Ghost<Set<V>>` isn't auto-Sync.
  Added 2 unsafe impls (+2 holes).
- **into_iter**: Added `requires self.spec_avltreesetmteph_wf()` to match iter().

Net: -11 proves + 2 unsafe impls = -9 holes.

### Filter Closure Requires (AVLTreeSetStEph.rs: 8 → 7)

Added `forall|t: &T| #[trigger] f.requires((t,))` to filter's trait requires and loop
invariant. Standard closure requires propagation per using_closures_standard.rs.
Cascaded to OrderedSetStEph.rs filter trait requires.

### Iterative Graph Search (GraphSearchMtPer.rs: 1 → 1)

Replaced recursive external_body `graph_search_explore` with verified iterative while loop
matching GraphSearchStEph pattern. Added closure requires to trait and standalone functions.
1 assume for `frontier.elements.spec_avltreeseqmtper_wf()` (seq wf for nth/length access).
Net 0 change in holes but structural proof is now fully verified.

## Remaining Holes

### AVLTreeSetMtEph.rs (10 holes)
- 2 assume: size/find view bridge (inner@ vs ghost_set_view@)
- 6 external_body: to_seq, filter, intersection, difference, union, iter
  (contain nested parallel functions or complex runtime operations)
- 2 unsafe impl: Send/Sync for Ghost<Set<V>> field

### Blockers (structural)
- **ArraySetStEph.rs** (3): `assume(obeys_feq_full::<T>())` — type axioms, cascade
  through entire table/graph hierarchy if changed to requires.
- **AVLTreeSetStEph.rs** (7): 6 blocked by clone view bridge (need feq for
  `elem.clone()@ == elem@`), 1 blocked by lack of sorted invariant in
  AVLTreeSeqStEph backing store.
- **AVLTreeSetStPer.rs** (6): Same structural blockers as StEph.
- **AVLTreeSetMtPer.rs** (12): No RwLock (plain struct wrapping seq), so ghost
  field approach doesn't apply. Parallel operations remain external_body.

## Totals

| Chapter | Before | After | Delta |
|---------|--------|-------|-------|
| Chap41 | 53 | 43 | -10 |
| Chap53 | 1 | 1 | 0 |
| **Total** | **54** | **44** | **-10** |
