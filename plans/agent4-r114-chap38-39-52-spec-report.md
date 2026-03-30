# R114 Agent 4 — Chap38 + Chap39 + Chap52 Spec Strengthening Report

## Summary

Strengthened trait ensures in Chap38 and Chap39 Mt files to match their St
reference counterparts. Chap52 Mt files already had specs matching their St
references — no changes needed.

## Results

- **Verified**: 5388 (was 5388 — no regression)
- **RTT**: 3529 passed
- **PTT**: 221 passed

## Changes by File

| # | Chap | File | Change | Impact |
|---|------|------|--------|--------|
| 1 | 38 | BSTParaMtEph.rs | `singleton` ensures: added `tree.spec_bstparamteph_wf()` | Callers now know singleton produces wf tree |
| 2 | 38 | BSTParaMtEph.rs | `expose` ensures: added full Node case (union, finite, disjoint, ordering) | Matches StEph expose contract |
| 3 | 38 | BSTParaMtEph.rs | `split` ensures: added `union =~= remove`, disjoint, not-contains, ordering quantifiers | Matches StEph split contract |
| 4 | 38 | BSTParaMtEph.rs | `join_pair` ensures: added `joined@ == self@.union(other@)` | Was only `finite()`, now exposes union semantics |
| 5 | 39 | BSTSetTreapMtEph.rs | `filter` ensures: added `subset_of`, contains iff spec_pred | Was only `finite()`, now exposes filter semantics |
| 6 | 39 | BSTTreapMtEph.rs | `insert` ensures: `self@.contains(value@)` → `self@ =~= old(self)@.insert(value@)` | Strictly stronger: callers get full set semantics |
| 7 | 39 | BSTTreapMtEph.rs | `delete` ensures: `!self@.contains(target@)` → `self@ =~= old(self)@.remove(target@)` | Strictly stronger: callers get full set semantics |

## Chap52 Analysis

All Mt files already match their St references:

| # | Chap | Mt File | St Reference | Status |
|---|------|---------|-------------|--------|
| 1 | 52 | AdjMatrixGraphMtEph.rs | AdjMatrixGraphStEph.rs | Specs match |
| 2 | 52 | AdjMatrixGraphMtPer.rs | AdjMatrixGraphStPer.rs | Specs match (different architecture: handles OOB gracefully) |
| 3 | 52 | AdjSeqGraphMtEph.rs | AdjSeqGraphStEph.rs | Specs match |
| 4 | 52 | AdjSeqGraphMtPer.rs | AdjSeqGraphStPer.rs | Specs match |
| 5 | 52 | AdjTableGraphMtPer.rs | AdjTableGraphStPer.rs | Specs match |
| 6 | 52 | EdgeSetGraphMtPer.rs | EdgeSetGraphStPer.rs | Specs match |

Note: EdgeSetGraphMtEph.rs and AdjTableGraphMtEph.rs do not exist.

## Techniques

- Compared trait ensures between St/Mt pairs, identified weaker Mt specs
- Checked free function ensures (e.g. `split_inner`, `expose_internal`, `union_inner`)
  to confirm impl already proves stronger postconditions
- Added proof block in `split` impl to derive `self@.remove(key@) =~= left@.union(right@)`
  from `split_inner`'s conditional union ensures
- Added `obeys_feq_full_trigger` assertion in `singleton` for wf proof
