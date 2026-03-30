# R106 Agent 2 — View Conformance Report

## Summary

- **Phase 2 errors (View/return type mismatches):** 14 → 14 (unchanged — all require large refactors)
- **Phase 4 errors (spec weakening):** 7 → 2 (5 fixed)
- **Total errors:** 21 → 16
- **Verified count:** 5429 verified, 0 errors (full validate clean)

## Fixes Applied

| # | Chap | File | Fix | Verified |
|---|------|------|-----|----------|
| 1 | 38 | BSTParaMtEph.rs | `insert` ensures `self@ =~= old(self)@.insert(key@)` | Yes |
| 2 | 38 | BSTParaMtEph.rs | `delete` ensures `self@ =~= old(self)@.remove(key@)` | Yes |
| 3 | 41 | AVLTreeSetMtEph.rs | `from_seq` ensures `constructed@ =~= seq@.to_set()` | Yes |
| 4 | 43 | OrderedSetStEph.rs | `filter` ensures subset_of + containment + pred | Yes |
| 5 | 06 | LabUnDirGraphMtEph.rs | `add_labeled_edge` ensures V/A update spec | Yes |

## Triage Table — All 21 Errors

| # | Chap | File | Error Type | Classification | Status |
|---|------|------|-----------|---------------|--------|
| 1 | 05 | SetMtEph.rs | View Set vs StEph Seq | **St wrong** (Set→Set) | Cascade: SetStEph uses Seq as backing View, 100+ refs |
| 2 | 06 | DirGraphMtEph.rs | View GraphView vs StEph Seq | **St wrong** | Cascade: Chap52-66 all use DirGraph |
| 3 | 06 | LabDirGraphMtEph.rs | View LabGraphView vs StEph Seq | **St wrong** | Cascade: Chap52-66 |
| 4 | 06 | LabUnDirGraphMtEph.rs | View LabGraphView vs StEph Seq | **St wrong** | Cascade: Chap52-66 |
| 5 | 06 | UnDirGraphMtEph.rs | View GraphView vs StEph Seq | **St wrong** | Cascade: Chap52-66 |
| 6 | 37 | AVLTreeSeqMtPer.rs | iter return type | **Priority 3** (intentional — different iter types) | Report only |
| 7 | 37 | BSTRBMtEph.rs | View Link vs StEph BalBinTree | **Mt wrong** | 176 Link-based spec refs, large rewrite |
| 8 | 37 | BSTRBMtEph.rs | supertrait mismatch | Follows #7 | Follows #7 |
| 9 | 37 | BSTSplayMtEph.rs | insert returns Result vs () | **Priority 3** (Mt uses Result for lock errors) | Report only |
| 10 | 41 | AVLTreeSetMtEph.rs | View Seq vs StPer Set | **False positive** — line 96 is GhostIter, not main struct | No fix needed |
| 11 | 43 | OrderedSetMtEph.rs | View Set vs StPer Seq | **St wrong** (OrderedSet→Set) | StPer has 141 self@ refs using Seq ops |
| 12 | 43 | OrderedSetMtEph.rs | to_seq return type | **Priority 3** (different backing seq types) | Report only |
| 13 | 43 | OrderedTableMtPer.rs | View Map vs StPer Seq | **St wrong** (Table→Map) | StPer has Seq<Pair> view, large rewrite |
| 14 | 43 | OrderedTableMtPer.rs | domain return type | **Priority 3** (different set types) | Report only |
| 15 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge weak | Spec weakening | **Fixed** |
| 16 | 38 | BSTParaMtEph.rs | insert weak | Spec weakening | **Fixed** |
| 17 | 38 | BSTParaMtEph.rs | delete weak | Spec weakening | **Fixed** |
| 18 | 41 | AVLTreeSetMtPer.rs | from_seq weak | Spec weakening | **Blocked** — values_in_order ensures true |
| 19 | 41 | AVLTreeSetMtEph.rs | from_seq weak | Spec weakening | **Fixed** |
| 20 | 43 | AugOrderedTableMtEph.rs | get_key_range weak | Spec weakening | **Blocked** — OrderedTableMtEph::get_key_range is also weak + View mismatch |
| 21 | 43 | OrderedSetStEph.rs | filter weak | Spec weakening | **Fixed** |

## Blocked Items

### AVLTreeSetMtPer from_seq (#18)
- MtPer from_seq builds the set by calling `seq.values_in_order()` then inserting each element
- `AVLTreeSeqMtPerS::values_in_order()` ensures `true` — no relationship between output and seq@
- Need to strengthen values_in_order in Chap37/AVLTreeSeqMtPer.rs first

### AugOrderedTableMtEph get_key_range (#20)
- Delegates to `OrderedTableMtEph::get_key_range` which only ensures `wf + finite`
- OrderedTableMtEph and OrderedTableStPer have mismatched View types (Map vs Seq<Pair>)
- The impl already contains `proof { assume(range.spec_orderedtablesteph_wf()); }` — existing hole
- Blocked by the OrderedTable View mismatch resolution

## View Mismatch Analysis

All 10 View mismatches (errors 1-5, 7-8, 10-11, 13) fall into two categories:

### Category A: St variant uses backing store as View (wrong)
- Chap05 SetStEph: View = Seq<T> (hash set backing) vs correct Set<T::V>
- Chap06 *StEph (4 files): View = Seq<V> (adjacency list) vs correct GraphView/LabGraphView
- Chap43 OrderedSetStPer: View = Seq<T> (sorted array) vs correct Set<T::V>
- Chap43 OrderedTableStPer: View = Seq<Pair<K,V>> (sorted array) vs correct Map<K::V,V::V>

These require rewriting all specs from Seq operations (index, subrange, etc.) to abstract operations (contains, subset, etc.). Cascade scope:
- Chap05: SetStEph internal only (~100 refs)
- Chap06: 4 St graph files + downstream Chap52-66 (massive)
- Chap43: OrderedSetStPer (~141 refs) + OrderedTableStPer + downstream

### Category B: Mt variant uses concrete type (wrong)
- Chap37 BSTRBMtEph: View = Link<T> (pointer) vs correct BalBinTree<T>
- Requires rewriting 176 Link-based spec function references

### Recommendation
These are R107+ tasks, not STEP 20 items. Prioritize:
1. **Chap05 SetStEph** — self-contained, no downstream cascade
2. **Chap37 BSTRBMtEph** — self-contained Mt file
3. **Chap43 OrderedSetStPer** — minimal external cascade (only Example43_1)
4. **Chap43 OrderedTableStPer** — moderate cascade
5. **Chap06 graph files** — largest cascade (save for last)

## Steps Used

5 edit/verify iterations out of STEP 20 budget.
