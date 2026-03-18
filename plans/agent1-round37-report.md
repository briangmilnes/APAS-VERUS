# Agent 1 — Round 37 Report

## Summary

Proved 4 ordering operations in OrderedTableMtEph.rs by porting the TotalOrder
bridging proofs from OrderedTableStEph.rs. Made 2 TableMtEph lemmas public to
support the proofs. Investigated 4 additional targets from R37 Update — all
blocked by structural issues or cascade scope.

## Verification

- **4290 verified, 0 errors** (was 4282 in R36)
- **2613 RTT passed**, 147 PTT passed
- **71 actionable holes** (was 75)

## Changes

### Tier 1: OrderedTableMtEph Ordering Ops (−4 holes)

| # | Chap | File | Function | Before | After | Technique |
|---|------|------|----------|--------|-------|-----------|
| 1 | 43 | OrderedTableMtEph.rs | first_key | external_body | verified | TotalOrder min scan + clone_plus |
| 2 | 43 | OrderedTableMtEph.rs | last_key | external_body | verified | TotalOrder max scan + clone_plus |
| 3 | 43 | OrderedTableMtEph.rs | previous_key | external_body | verified | TotalOrder predecessor scan |
| 4 | 43 | OrderedTableMtEph.rs | next_key | external_body | verified | TotalOrder successor scan |

**Technique**: Ported the identical proof structure from OrderedTableStEph.rs
(proved by Agent2 in R36). Each function iterates over `self.base_table.entries`
using `TotalOrder::cmp` to find min/max/predecessor/successor, with loop
invariants tracking the best candidate via `spec_index`. Post-loop proof bridges
from entry-level facts to map-domain-level ensures using
`lemma_entries_to_map_contains_key` and `lemma_entries_to_map_key_in_seq`.

**Supporting changes**:
- Added `pub` to `lemma_entries_to_map_contains_key` and
  `lemma_entries_to_map_key_in_seq` in `src/Chap42/TableMtEph.rs` (were private,
  needed by OrderedTableMtEph proofs).
- Added `clone_plus` import and changed feq imports to glob in
  `src/Chap43/OrderedTableMtEph.rs`.

### Tier 1 Remaining: rank_key, select_key (kept external_body)

Both `rank_key` and `select_key` are also external_body in OrderedTableStEph.rs —
no proved template to copy. The ensures use `Set::filter` with existential
quantifiers over TotalOrder, which requires linking a concrete loop count to
abstract set filter cardinality. This is a hard proof obligation shared across
StEph and MtEph.

## R37 Update Targets — Investigation Results

### Target 1: BSTParaStEph.rs assume (Chap38, line 475) — STRUCTURAL

The assume bridges `k = node.key.clone()` to `k@ == node.key@` and
`t.cmp_spec(&k) == t.cmp_spec(&node.key)` inside `expose()`. The function has
no `requires` — `obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()` are
unavailable. These are needed by all the file's helper lemmas
(`lemma_cmp_antisymmetry`, `lemma_cmp_transitivity`, etc.). Adding requires to
`expose()` would cascade to `Clone::clone` (which can't have requires in Rust).
**Cannot fix without redesigning the expose/clone interface.**

### Target 2: JohnsonStEphI64.rs assume (Chap59, line 437) — CASCADE

The assume is `reweighted@.A.len() * 2 + 2 <= usize::MAX as int`, needed for
Dijkstra's edge-count bound. Fixing requires:
1. `from_weighed_edges` (Chap06) must ensure `g@.A.len()` relationship.
2. `reweight_graph` must ensure `result@.A.len()` preservation.
3. `johnson_apsp` requires must include edge-count bound.
Each change cascades through WeightedDirGraph specs and all callers.
**Too complex for a single-target fix.**

### Target 3: AVLTreeSetStEph.rs assumes (Chap41, lines 1059/1334) — CASCADE

Two `assume(new_vec@.len() < usize::MAX)` in insert/insert_sorted. Root cause:
wf bounds `< usize::MAX`, insert adds one element needing `+ 1 < usize::MAX`.
Fix requires `requires old(self)@.len() + 1 < usize::MAX as nat` on the trait's
`insert` method, cascading to 17 files across Chap41–55.
**Not attempted due to cascade scope.**

### Target 4: OrderedSetMtEph.rs to_seq (Chap43, line 345) — BLOCKED

The external_body wraps: acquire_read → inner.to_seq() → release_read → copy
elements → ArraySeqStPerS::from_vec. Removing external_body requires:
1. `assume(inner@ =~= self@)` — RWLOCK_GHOST bridge (standard).
2. `assume(st_seq.spec_avltreeseqstper_wf())` — OrderedSetStEph::to_seq trait
   ensures omit wf on the returned sequence (impl ensures it via from_vec, but
   trait contract doesn't propagate). Needed for nth/length calls.
3. clone_plus loop with element-wise view invariant for clone/view bridging.
The StEph version already uses `accept()` for the same clone/view gap.
**Fixable with 2 assumes + loop proof, but wf trait gap needs to be addressed
in OrderedSetStEph.rs (agent3 territory).**

## Files Modified

| # | Chap | File | Nature of change |
|---|------|------|-----------------|
| 1 | 42 | TableMtEph.rs | Made 2 lemmas pub |
| 2 | 43 | OrderedTableMtEph.rs | Proved 4 ordering ops, added imports |

## Remaining Holes in Assigned Files

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 43 | OrderedTableMtEph.rs | rank_key | external_body | Set::filter cardinality (blocked in StEph) |
| 2 | 43 | OrderedTableMtEph.rs | select_key | external_body | Set::filter cardinality (blocked in StEph) |
| 3 | 43 | OrderedSetMtEph.rs | to_seq | external_body | Trait wf gap + clone bridging |
| 4 | 38 | BSTParaStEph.rs | expose | assume | No requires on expose/Clone |
| 5 | 59 | JohnsonStEphI64.rs | johnson_apsp | assume | Edge-count spec cascade |
| 6 | 41 | AVLTreeSetStEph.rs | insert | assume | usize::MAX cascade to 17 files |
| 7 | 41 | AVLTreeSetStEph.rs | insert_sorted | assume | usize::MAX cascade to 17 files |
