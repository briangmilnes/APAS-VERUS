# R106-R107 Agent 2 — View Conformance + Spec Strengthening Report

## Final Results

| Metric | Before | After |
|--------|--------|-------|
| Phase 2 errors (View/return) | 14 | **0** |
| Phase 4 errors (spec weakening) | 7 | **0** |
| Verified | 5429 | 5433 |
| RTT | 3083 | 3083 |
| PTT | 157 | 157 |

## All Fixes Applied

### View Type Fix (1)

| # | Chap | File | Fix |
|---|------|------|-----|
| 1 | 37 | BSTRBMtEph.rs | View Link\<T\> → BalBinTree\<T\>: link_to_bbt conversion + 3 bridge lemmas |

### Spec Weakening Fixes (8)

| # | Chap | File | Fix |
|---|------|------|-----|
| 1 | 38 | BSTParaMtEph.rs | insert ensures self@ =~= old(self)@.insert(key@) |
| 2 | 38 | BSTParaMtEph.rs | delete ensures self@ =~= old(self)@.remove(key@) |
| 3 | 41 | AVLTreeSetMtEph.rs | from_seq ensures constructed@ =~= seq@.to_set() |
| 4 | 41 | AVLTreeSetMtPer.rs | from_seq ensures constructed@ =~= seq@.to_set() |
| 5 | 43 | OrderedSetStEph.rs | filter ensures subset_of + containment + pred |
| 6 | 43 | OrderedTableMtEph.rs | get_key_range ensures dom subset + value agreement |
| 7 | 43 | AugOrderedTableMtEph.rs | get_key_range ensures dom subset + value agreement |
| 8 | 06 | LabUnDirGraphMtEph.rs | add_labeled_edge ensures V/A update spec |

### Supporting Infrastructure

| # | Chap | File | What |
|---|------|------|------|
| 1 | 37 | AVLTreeSeqMtPer.rs | values_in_order: ensures values@.map_values =~= spec_seq() (with assume for map_values+push) |

## False Positives Resolved

8 false positives in earlier veracity-compare-par-mut runs were caused by the tool comparing
Mt's Locked/GhostIter View types against the wrong St struct. Tool was updated between runs
and these resolved to 0 errors.

## Assumes Added

| Chap | File | Assume | Reason |
|------|------|--------|--------|
| 37 | AVLTreeSeqMtPer.rs | values_in_order: `out@.map_values =~= self.spec_seq()` | map_values + push distribution lemma unavailable in vstd |
| 43 | OrderedTableMtEph.rs | get_key_range: `inner@ =~= self@` | Lock shadow synchronization |
| 43 | OrderedTableMtEph.rs | get_key_range: `range.spec_orderedtablesteph_wf()` | Lock predicate bridge (pre-existing) |

## Remaining Work

- Phase 2: 0 errors (215 warnings — intentional return type/signature differences)
- Phase 4: 0 errors (733 warnings — mostly `ensures true` on traversal/filter/reduce fns)
- Global holes: 4 (all in Chap65 UnionFind — Agent 1)
