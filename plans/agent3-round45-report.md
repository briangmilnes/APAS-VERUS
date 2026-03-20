# Agent 3 — Round 45 Report

## Summary

Proved all 9 Boruvka parallel MST holes in Chap66/BoruvkaMtEph.rs. Chap65 holes
(sort_edges_by_weight, prim_mst) remain — both require Verus infrastructure that
doesn't exist (exec-level Vec::sort_by spec, complex PQ loop invariants).

**Holes: 11 before, 2 after. Net: -9.**

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 66 | BoruvkaMtEph.rs | 9 | 0 | -9 |
| 2 | 65 | KruskalStEph.rs | 1 | 1 | 0 |
| 3 | 65 | PrimStEph.rs | 1 | 1 | 0 |
| | | **Total** | **11** | **2** | **-9** |

## Chapters Closed

- **Chap66**: 0 holes (was 9). Both BoruvkaMtEph.rs and BoruvkaStEph.rs are clean.

## What Was Proved (Chap66)

All 9 external_body functions in BoruvkaMtEph.rs were verified:

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 66 | BoruvkaMtEph.rs | hash_coin_flips_mt | ParaPair named closures + HashMap merge loop |
| 2 | 66 | BoruvkaMtEph.rs | compute_remaining_mt | ParaPair named closures + Vec merge loop |
| 3 | 66 | BoruvkaMtEph.rs | collect_mst_labels_mt | ParaPair named closures + Vec merge loop |
| 4 | 66 | BoruvkaMtEph.rs | build_partition_map_mt | ParaPair named closures + HashMap merge loop |
| 5 | 66 | BoruvkaMtEph.rs | vertex_bridges_mt | ParaPair named closures + min-weight HashMap merge |
| 6 | 66 | BoruvkaMtEph.rs | bridge_star_partition_mt | Orchestrator with verified helper calls |
| 7 | 66 | BoruvkaMtEph.rs | filter_tail_to_head_mt | ParaPair named closures + HashMap merge loop |
| 8 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt | Main algorithm with verified iterator loops |
| 9 | 66 | BoruvkaMtEph.rs | reroute_edges_mt | ParaPair named closures + Vec merge loop |

Also fixed:
- `boruvka_mst_mt_with_seed` fn_missing_ensures warning (added `ensures mst.spec_setsteph_wf()`)
- `bridge_star_partition_mt` fn_missing_ensures warning (added `ensures result.0.spec_setsteph_wf()`)
- Added requires to trait methods (obeys_key_model, bounds, wf predicates)

## Techniques Used

### ParaPair Named Closure Pattern
The core technique. For each recursive divide-and-conquer function:

1. Removed `#[verifier::external_body]`
2. Added `#[verifier::exec_allows_no_decreases_clause]` (recursion through closures)
3. Added `requires start <= end, end <= collection@.len()` for index bounds
4. Added `obeys_key_model::<V>()` where HashMapWithViewPlus::new() is called
5. Replaced inline ParaPair! calls with named closures:
   ```rust
   let f1 = move || -> (r: ReturnType)
       requires start <= mid, mid <= v1@.len(), obeys_key_model::<V>(), ...
   { recursive_call(v1, p1, start, mid) };
   let Pair(left, right) = crate::ParaPair!(f1, f2);
   ```
6. Closure `requires` proved at creation site; `ensures` propagated through ParaPair!

### Verus-Compatible Merge Operations
All non-Verus operations were replaced:
- `vec![x]` -> `Vec::new()` + `push()`
- `left.extend(pair.1)` -> index-based while loop with `push()`
- `pair.1.inner.into_iter()` -> HashMapWithViewPlus `iter()` + `insert()` loop
- `left.append(&mut right)` -> index-based while loop
- `partition.inner.keys().cloned().collect()` -> manual iterator loop
- `remaining.iter().cloned().collect::<Vec<V>>()` -> manual iterator loop
- `Arc::try_unwrap(...).unwrap_or_else(...)` -> iterator reconstruction loop
- `SetLit![]` -> `SetStEph::empty()`

### Arc View Pattern
`Arc<Vec<V>>` has `View` in vstd (delegates to inner type's view). So `vertices@`
on `Arc<Vec<V>>` gives `Seq<V>`. Arc::clone ensures `res == *a`, so `v1@.len() ==
vertices@.len()` follows automatically.

## Remaining Holes (2)

### Chap65/KruskalStEph.rs: sort_edges_by_weight (1 hole)
**Blocked by**: No exec-level Verus spec for `Vec::sort_by`. vstd has `Seq::sort_by`
at spec level but no external_fn_specification for the standard library's sort_by.
The function's ensures (length preservation, element containment) are reasonable and
trusted via external_body.

### Chap65/PrimStEph.rs: prim_mst (1 hole)
**Blocked by**: Complex algorithmic invariants:
- PQ size bounds: `pq@.len() * 2 <= usize::MAX` required by `delete_min`, but PQ
  grows via `insert()` on each neighbor. Needs invariant relating PQ size to graph size.
- Loop termination: PQ can grow (no obvious decreases clause). Needs
  `exec_allows_no_decreases_clause` plus careful invariant management.
- `for v in neighbors.iter()` inner loop: requires rewriting as explicit `loop` with
  `next()` calls and nested loop invariants.
- `continue` control flow: complicates outer loop invariant (must hold at continue point).
- Heap property maintenance: must track `spec_is_exec_heap(pq.spec_seq())` through
  insert/delete operations.

Not attempted — requires significant restructuring and deep algorithmic invariants.

## Warnings Remaining

| # | Chap | File | Warning | Notes |
|---|------|------|---------|-------|
| 1 | 65 | PrimStEph.rs | fn_missing_requires on pq_entry_new | Pure constructor, genuinely no precondition |
| 2 | 66 | BoruvkaMtEph.rs | assume_eq_clone_workaround | Standard PartialEq pattern |
| 3 | 66 | BoruvkaStEph.rs | assume_eq_clone_workaround | Standard PartialEq pattern |

## Verification Counts

- Verified: 4388
- Errors: 0
- RTT: 2613 passed, 0 skipped
