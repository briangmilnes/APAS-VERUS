# Chap45 Spec Audit — Priority Queues (ADT 45.1)

Audited 2026-03-15 against APAS Chapter 45, Data Type 45.1.

## Reference: LeftistHeapPQ.rs (CORRECT)

View type: `Multiset<T>`. All ADT 45.1 specs correct.

| # | Function | Spec strength | Notes |
|---|----------|--------------|-------|
| 1 | empty | STRONG | `pq@ =~= Multiset::empty()` |
| 2 | singleton | STRONG | `pq@ =~= Multiset::empty().insert(element)` |
| 3 | find_min | STRONG | Minimality via `TotalOrder::le(*min, e)` for all `e` in multiset |
| 4 | insert | STRONG | `pq@ =~= self@.insert(element)` |
| 5 | delete_min | STRONG | Content preservation + minimality |
| 6 | meld | STRONG | `pq@ =~= self@.add(other@)` |
| 7 | from_seq | PARTIAL | Size only, no content preservation (size == seq@.len()) |
| 8 | extract_all_sorted | STRONG | Size + `spec_sorted(sorted@)` |
| 9 | to_sorted_vec | STRONG | Size + `spec_sorted(v@)` |

## BinaryHeapPQ.rs

View type: `Seq<T::V>`. Uses `to_multiset()` for content specs.

| # | Function | Before | After | Action taken |
|---|----------|--------|-------|-------------|
| 1 | empty | STRONG | STRONG | No change. `pq@.to_multiset() =~= Multiset::empty()` |
| 2 | singleton | STRONG | STRONG | No change. Multiset singleton ensures |
| 3 | find_min | WEAK | IMPROVED | Added `min_elem.unwrap()@ == self@[0]`. Verified without external_body |
| 4 | insert | STRONG | STRONG | No change. Multiset preservation |
| 5 | delete_min | STRONG | STRONG | No change. Multiset preservation + content decomposition |
| 6 | meld | STRONG | STRONG | No change. Multiset add |
| 7 | from_seq | WEAK | IMPROVED | Added `pq@.to_multiset() =~= seq@.to_multiset()`. Verified (heapify preserves multiset) |
| 8 | extract_all_sorted | PARTIAL | PARTIAL | Has `spec_sorted`. 1 assume for sortedness proof (heap property not in wf) |
| 9 | to_sorted_vec | STRONG | STRONG | No change. Size + spec_sorted |

### Remaining gaps

- **find_min TotalOrder minimality**: Requires connecting `spec_leq_view` (uninterp) to `TotalOrder::le`, and adding heap property to `spec_binaryheappq_wf`. Structural change.
- **from_seq wf**: The wf ensures is NOT present on from_seq (only len + multiset). Consider adding.

## BalancedTreePQ.rs

View type: `Seq<T::V>`. Backed by sorted AVL tree sequence.

| # | Function | Before | After | Action taken |
|---|----------|--------|-------|-------------|
| 1 | empty | OK | OK | No change |
| 2 | singleton | OK | OK | No change |
| 3 | find_min | WEAK | IMPROVED | Added `min_elem.unwrap()@ == self@[0]`. Verified |
| 4 | insert | OK | OK | Length + wf |
| 5 | delete_min | OK | OK | Length + wf + Some/None |
| 6 | meld | OK | OK | Length + wf |
| 7 | from_seq | WEAK | IMPROVED | Added `pq@.to_multiset() =~= seq@.to_multiset()`. Required external_body (+1 hole) |
| 8 | extract_all_sorted | WEAK | IMPROVED | Added `sorted@ =~= self@`. Verified (clone equality) |
| 9 | find_max | WEAK | IMPROVED | Added `max_elem.unwrap()@ == self@[self@.len() - 1]`. Verified |
| 10 | to_seq | OK | OK | `seq@ =~= self@` already present |

### Remaining gaps

- **find_min/find_max TotalOrder minimality/maximality**: The view is `Seq<T::V>` but `TotalOrder::le` operates on `T`. Needs either: (a) sorted invariant in wf, or (b) `spec_seq: Seq<T>` added to trait + TotalOrder ensures.
- **insert content preservation**: `insert` only ensures length + wf, not `pq@.to_multiset() =~= self@.to_multiset().insert(element@)`. Would enable from_seq proof.
- **from_seq external_body**: Blocked by insert lacking content preservation.

## Summary

| File | Before | After | Holes delta |
|------|--------|-------|------------|
| LeftistHeapPQ.rs | 0 holes | 0 holes | 0 |
| BinaryHeapPQ.rs | 1 hole | 1 hole | 0 |
| BalancedTreePQ.rs | 1 hole | 2 holes | +1 (from_seq external_body for stronger spec) |
