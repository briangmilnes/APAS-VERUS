# Agent 3 Round 18 Report

## Mission

Strengthen specs for Chap45 PQ find_min/find_max/insert and Chap38 in_order.

## Changes

### BinaryHeapPQ (Chap45/BinaryHeapPQ.rs)

| # | Function | Change | Hole |
|---|----------|--------|------|
| 1 | find_min (trait) | Added TotalOrder minimality + multiset containment ensures | — |
| 2 | find_min (impl) | Added external_body (heap invariant not formally tracked in wf) | +1 external_body |

New ensures:
- `self@.to_multiset().count(min_elem.unwrap()@) > 0` (containment)
- `forall|i| TotalOrder::le(*min_elem.unwrap(), self.spec_seq()[i])` (minimality)

### BalancedTreePQ (Chap45/BalancedTreePQ.rs)

| # | Function | Change | Hole |
|---|----------|--------|------|
| 3 | find_min (trait+impl) | Added `min_elem.unwrap()@ == self@[0]` ensures | 0 (verified) |
| 4 | find_max (trait+impl) | Added `max_elem.unwrap()@ == self@[self@.len() - 1]` ensures | 0 (verified) |
| 5 | insert (trait) | Added `pq@.to_multiset() =~= self@.to_multiset().insert(element@)` | — |
| 6 | insert (impl) | Added external_body (multiset proof complex) | +1 external_body |

### BSTParaStEph (Chap38/BSTParaStEph.rs)

| # | Function | Change | Hole |
|---|----------|--------|------|
| 7 | in_order (trait) | Added `forall|v| self@.contains(v) <==> seq@.contains(v)` content ensures | — |
| 8 | in_order (impl) | Added external_body (content proof requires propagation through collect_in_order) | +1 external_body |

## Hole Delta

| File | Before | After | Delta | Notes |
|------|--------|-------|-------|-------|
| BinaryHeapPQ.rs | 1 assume | 1 assume + 1 external_body | +1 | find_min now has TotalOrder minimality |
| BalancedTreePQ.rs | 1 external + 0 external_body | 1 external + 1 external_body | +1 | insert has multiset preservation; find_min/find_max verified with stronger spec |
| BSTParaStEph.rs | 4 assume + 2 external_body | 4 assume + 3 external_body | +1 | in_order now ensures content preservation |

Net new holes: +3 external_body (all with stronger specs — strong spec + external_body > weak spec)

## Verification

- Validate: 4134 verified, 0 errors
- RTT: 2600 passed, 0 failed
- PTT: 147 passed, 0 failed

## Design Notes

- BalancedTreePQ find_min/find_max strengthened WITHOUT external_body — the `nth` ensures
  from AVLTreeSeqStPerS directly proves element-view equality.
- Full TotalOrder minimality for BalancedTreePQ not expressible because AVLTreeSeqStPerS
  only exposes `Seq<T::V>` at spec level (no `Seq<T>` available). BinaryHeapPQ can express
  it because ArraySeqStPerS has `pub seq: Vec<T>`.
- BSTParaStEph in_order content ensures uses T::V-level set/seq containment equivalence.
  Sortedness not added because ParamBST has `T: StT + Ord` (no TotalOrder bound).

## Commit

To be committed on agent3/ready.
