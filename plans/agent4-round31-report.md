# Agent 4 — Round 31 Report

## Summary

Proved 3 of 4 assumes in Chap38/BSTParaStEph.rs (size arithmetic).
Analyzed 11 fn_missing_requires/requires_true across Chap43, 47, 57, 58 — all genuinely
have no real preconditions or adding requires would cascade.

## Verification State

- 4116 verified, 0 errors, 0 warnings
- 2613 RTT pass
- Net holes changed: -3 (BSTParaStEph 5 → 2)

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 38 | BSTParaStEph.rs | 5 | 2 | -3 | Proved 3 size arithmetic assumes |
| 2 | 47 | StructChainedHashTable.rs | 0 | 0 | 0 | fn_missing_requires: no real precondition |
| 3 | 47 | VecChainedHashTableStEph.rs | 0 | 0 | 0 | fn_missing_requires: no real precondition |
| 4 | 47 | LinkedListChainedHashTableStEph.rs | 0 | 0 | 0 | fn_missing_requires: no real precondition |
| 5 | 43 | OrderedSetStPer.rs | 0 | 0 | 0 | requires_true: no real precondition |
| 6 | 43 | OrderedSetStEph.rs | 0 | 0 | 0 | requires_true: no real precondition |
| 7 | 43 | AugOrderedTableStPer.rs | 1 | 1 | 0 | requires_true: cascade risk |
| 8 | 43 | AugOrderedTableMtEph.rs | 1 | 1 | 0 | requires_true + fn_missing_wf: cascade risk |
| 9 | 57 | DijkstraStEphI64.rs | 0 | 0 | 0 | requires_true: no real precondition |
| 10 | 58 | BellmanFordStEphI64.rs | 0 | 0 | 0 | requires_true: no real precondition |

## Techniques Used

### Chap38 — Size Arithmetic Proofs

Added real size-bound preconditions to BSTParaStEphTrait:
- `insert`: `old(self)@.len() < usize::MAX as nat`
- `delete`: `old(self)@.len() < usize::MAX as nat`
- `union`: `self@.len() + other@.len() <= usize::MAX as nat`

No cascade: BSTParaMtEph.rs does NOT import BSTParaStEph (independent implementation).
RTTs don't check Verus requires.

Proof pattern for insert/delete (after split into left/right):
```
vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
assert(left@.union(right@) =~= old_view.remove(key@));
assert(old_view.remove(key@).subset_of(old_view));
vstd::set_lib::lemma_len_subset(old_view.remove(key@), old_view);
```

Union proof required extensional subset reasoning to establish
`luv.union(ruv).insert(akv).subset_of(sv.union(other@))`, then:
```
vstd::set_lib::lemma_len_union(sv, other@);
vstd::set_lib::lemma_len_subset(luv_ruv.insert(akv), sv.union(other@));
```

### Chap47 — fn_missing_requires Analysis

All 5 functions (chain_insert, chain_lookup, chain_delete, clone_vec_pairs,
clone_linked_list_entry) are free functions operating on arbitrary inputs with no struct
or wf predicate. They genuinely have no preconditions. These are veracity style warnings,
not proof holes.

### Chap43 — requires_true Analysis

- `from_sorted_elements` (OrderedSetStPer, OrderedSetStEph): Delegates to from_vec
  (no requires) then from_seq (requires satisfied by from_vec ensures). No precondition.
- `calculate_reduction` (AugOrderedTableStPer): Adding closure requires would cascade
  to ~15+ call sites. Has existing assume for `reducer.requires(...)`.
- `recalculate_reduction` (AugOrderedTableMtEph): Called after mutations where wf may
  not hold. Adding wf requires would cascade.

### Chap57/58 — requires_true Analysis

- `pq_entry_new` (DijkstraStEphI64): Simple struct constructor `PQEntry { dist, vertex }`.
- `clamp_weight` (BellmanFordStEphI64): Pure function `if w > i64::MAX as i128 { ... }`.

Both genuinely have no preconditions.

## Remaining Holes in Assigned Files

| # | Chap | File | Holes | What Blocks |
|---|------|------|-------|-------------|
| 1 | 38 | BSTParaStEph.rs | 1 | assume: Clone bridge (L469) — needs Verus Clone ensures |
| 2 | 38 | BSTParaStEph.rs | 1 | external_body: Clone impl (L1577) — needs Verus Clone ensures |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | assume: reducer.requires() — closure infrastructure |
| 4 | 43 | AugOrderedTableMtEph.rs | 1 | external_body: calculate_reduction — closure infrastructure |
