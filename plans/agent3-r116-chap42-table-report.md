# Agent 3 R116 — Chap42 Table Spec Strengthening Report

## Summary

Strengthened requires/ensures on TableStEph and TableMtEph to close the gap
with TableStPer, reducing veracity-compare-par-mut warnings from 13 to 6.
All changes verify clean (2173 verified, 0 Chap42 errors). 3529 RTTs pass.
Zero proof holes across all 3 Chap42 files.

## Changes Made

### TableStEph.rs — requires additions (9 functions)

| # | Chap | Function | Change |
|---|------|----------|--------|
| 1 | 42 | `singleton` | Added `tree.spec_tablesteph_wf()` ensures |
| 2 | 42 | `map` | Added `self.spec_tablesteph_wf()` ensures |
| 3 | 42 | `filter` | Added `obeys_feq_full::<Pair<K,V>>()` requires, `self.spec_tablesteph_wf()` ensures |
| 4 | 42 | `intersection` | Added `self.spec_tablesteph_wf()` ensures |
| 5 | 42 | `union` | Added `self.spec_tablesteph_wf()` ensures |
| 6 | 42 | `difference` | Added `obeys_feq_full::<Pair<K,V>>()` requires, `self.spec_tablesteph_wf()` ensures |
| 7 | 42 | `find` | Added `obeys_feq_full::<V>()` requires |
| 8 | 42 | `delete` | Added `obeys_feq_clone::<Pair<K,V>>()`, `obeys_feq_full::<Pair<K,V>>()` requires |
| 9 | 42 | `insert` | Added `obeys_feq_full::<Pair<K,V>>()` requires |
| 10 | 42 | `insert_wf` | Added `obeys_feq_full::<Pair<K,V>>()` requires |
| 11 | 42 | `delete_wf` | Added `obeys_feq_full::<Pair<K,V>>()` requires |
| 12 | 42 | `restrict` | Added `obeys_feq_full::<Pair<K,V>>()` requires, `self.spec_tablesteph_wf()` ensures |
| 13 | 42 | `subtract` | Added `obeys_feq_full::<Pair<K,V>>()` requires, `self.spec_tablesteph_wf()` ensures |

### TableMtEph.rs — spec + requires + ensures additions

| # | Chap | Function | Change |
|---|------|----------|--------|
| 1 | 42 | `spec_stored_value` | Added spec fn to trait and impl |
| 2 | 42 | `singleton` | Added `obeys_feq_clone::<Pair<K,V>>()` requires |
| 3 | 42 | `domain` | Added `obeys_feq_clone::<K>()` requires |
| 4 | 42 | `map` | Added `self.spec_tablemteph_wf()` ensures |
| 5 | 42 | `filter` | Added `self.spec_tablemteph_wf()` ensures |
| 6 | 42 | `intersection` | Added `self.spec_tablemteph_wf()` ensures |
| 7 | 42 | `union` | Added `self.spec_tablemteph_wf()` ensures |
| 8 | 42 | `difference` | Added `self.spec_tablemteph_wf()` ensures + no_dups proof |
| 9 | 42 | `restrict` | Added `self.spec_tablemteph_wf()` ensures + no_dups proof |
| 10 | 42 | `subtract` | Added `self.spec_tablemteph_wf()` ensures + no_dups proof |

### Proof work in MtEph impl bodies

For `difference`, `restrict`, and `subtract`: added `spec_keys_no_dups`
proofs via subsequence argument (monotone source indices → inherited
no-dup from parent). The feq components of wf are spec-only predicates
carried from `old(self).spec_tablemteph_wf()` in requires.

## Warning Count

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap42 compare-par-mut warnings (Phase 2) | 2 | 2 | 0 |
| Chap42 compare-par-mut warnings (Phase 4) | 11 | 4 | -7 |
| **Total Chap42 warnings** | **13** | **6** | **-7** |
| Chap42 proof holes | 0 | 0 | 0 |

## Remaining 6 Warnings (assessed)

| # | Chap | File | Warning | Assessment |
|---|------|------|---------|------------|
| 1 | 42 | TableStEph.rs | Missing `collect` fn from StPer | StEph has `entries()` which is equivalent. Naming difference, not missing functionality. |
| 2 | 42 | TableMtEph.rs | Missing `find_ref` from StEph | Returns `&V` — incompatible with RwLock-guarded Mt access. Architectural limitation. |
| 3 | 42 | TableMtEph.rs | Missing `insert_wf` from StEph | Requires `ClonePreservesView`/`ClonePreservesWf` where clauses. Implementable but needs real proof work. |
| 4 | 42 | TableMtEph.rs | Missing `delete_wf` from StEph | Same as insert_wf. |
| 5 | 42 | TableMtEph.rs | `find` missing `obeys_feq_full::<V>()` | Subsumed by `spec_tablemteph_wf()` in requires (wf includes `obeys_feq_fulls::<K,V>()`). False positive. |
| 6 | 42 | TableMtEph.rs | `delete` missing `obeys_feq_clone::<Pair<K,V>>()` | Subsumed by `spec_tablemteph_wf()` (wf includes `obeys_feq_full::<Pair<K,V>>()`). False positive. |

## Verification

- `scripts/validate.sh isolate Chap42`: 2173 verified, 0 Chap42 errors
  (1 pre-existing Chap37 rlimit flakiness)
- `scripts/rtt.sh`: 3529 passed
- `scripts/holes.sh src/Chap42/`: 0 holes, 3 clean files
