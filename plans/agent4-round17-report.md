# Agent 4 — Round 17 Report: Spec Audit (Chap45 + Chap47)

## Mission

Audit and strengthen weak/missing `requires`/`ensures` specs against APAS textbook
definitions ADT 45.1 (Meldable Priority Queues) and ADT 47.1 (Hash Tables).

## Holes Before/After

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 45 | LeftistHeapPQ.rs | 0 | 0 | 0 | Reference — no changes |
| 2 | 45 | BinaryHeapPQ.rs | 1 | 1 | 0 | +2 ensures, no new holes |
| 3 | 45 | BalancedTreePQ.rs | 1 | 2 | +1 | +4 ensures, +1 external_body (from_seq) |
| 4 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 | Structural only — no View |
| 5 | 47 | FlatHashTable.rs | 0 | 0 | 0 | +2 ensures on insert_with_probe |
| 6 | 47 | LinProbFlatHashTableStEph.rs | 0 | 0 | 0 | No changes needed |
| 7 | 47 | QuadProbFlatHashTableStEph.rs | 0 | 0 | 0 | No changes needed |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | 0 | 0 | 0 | No changes needed |
| 9 | 47 | ChainedHashTable.rs | 0 | 0 | 0 | lookup_chained `ensures true` — needs View |
| 10 | 47 | LinkedListChainedHashTableStEph.rs | 0 | 0 | 0 | No changes needed |
| 11 | 47 | VecChainedHashTableStEph.rs | 0 | 0 | 0 | No changes needed |
| 12 | 47 | StructChainedHashTable.rs | 0 | 0 | 0 | Best View candidate — not changed |
| | | **Total** | **4** | **5** | **+1** | |

## Chapters Closed

None. This round was spec strengthening, not hole removal.

## Verification

- 4149 verified, 0 errors
- RTT: not re-run (no behavioral changes)

## Specs Strengthened

### Chap45 — BinaryHeapPQ.rs (0 new holes)

1. **find_min**: Added `self@.len() > 0 ==> min_elem.unwrap()@ == self@[0]`.
   Verified via nth(0) ensures bridge.
2. **from_seq**: Added `pq@.to_multiset() =~= seq@.to_multiset()`.
   Verified — heapify already preserves multiset.

### Chap45 — BalancedTreePQ.rs (+1 hole)

1. **find_min**: Added `self@.len() > 0 ==> min_elem.unwrap()@ == self@[0]`. Verified.
2. **find_max**: Added `self@.len() > 0 ==> max_elem.unwrap()@ == self@[self@.len() as int - 1]`. Verified.
3. **from_seq**: Added `pq@.to_multiset() =~= seq@.to_multiset()`.
   Required `#[verifier::external_body]` (+1 hole) because insert's ensures lacks
   multiset content preservation — the loop can't prove the invariant.
4. **extract_all_sorted**: Added `sorted@ =~= self@`. Verified (clone equality).

### Chap47 — FlatHashTable.rs (0 new holes)

1. **insert_with_probe**: Added `table.table@.len() == table.current_size as int`
   and `table.current_size == old(table).current_size`. Verified.

## Techniques Used

- **View-position ensures**: Express minimality as `min_elem.unwrap()@ == self@[0]`
  when full TotalOrder minimality is blocked by T vs T::V type gap.
- **Multiset content preservation**: `pq@.to_multiset() =~= seq@.to_multiset()`
  for from_seq correctness.
- **external_body for correct-but-unprovable specs**: BalancedTreePQ from_seq gets
  the correct spec with external_body rather than a weak spec without it.

## Remaining Holes — What Blocks Them

### Chap45 (4 holes)

| # | File | Hole | Blocker |
|---|------|------|---------|
| 1 | BinaryHeapPQ.rs | extract_all_sorted assume | spec_leq_view uninterpreted, disconnected from TotalOrder::le |
| 2 | BalancedTreePQ.rs | from_seq external_body | insert lacks multiset content preservation ensures |
| 3 | BalancedTreePQ.rs | external (eq bridge) | Standard eq/clone workaround |
| 4 | Chap45 | external (call_hash or eq) | Infrastructure |

### Chap47 (2 holes)

| # | File | Hole | Blocker |
|---|------|------|---------|
| 1 | ParaHashTableStEph.rs | call_hash_fn external_body | Opaque Fn trait — can't verify hash function call |
| 2 | ParaHashTableStEph.rs | compute_second_hash external_body | std::hash — external dependency |

### Structural Blockers (not holes, but spec gaps)

- **Chap47 View**: No `Map<Key::V, Value::V>` ghost state on HashTable.
  All insert/lookup/delete/resize specs are structural (table.len, current_size).
  ADT 47.1 functional specs require a View impl. StructChainedHashTable has
  `spec_chain_to_map` — best starting point for a whole-table View.
- **TotalOrder minimality**: BinaryHeapPQ and BalancedTreePQ can't express
  `TotalOrder::le(*min, e) for all e` because View is `Seq<T::V>` and
  TotalOrder::le operates on T. Would need sorted invariant in wf or
  a `spec_seq: Seq<T>` in the trait.

## Deliverables

- [x] `src/Chap45/analyses/spec-audit.md`
- [x] `src/Chap47/analyses/spec-audit.md`
- [x] `plans/agent4-round17-report.md`
- [ ] Git commit + push to agent4/ready

## Commit Hash

f34e323b
