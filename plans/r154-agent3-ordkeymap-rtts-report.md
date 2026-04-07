# R154 Agent3 Report: OrdKeyMap RTTs + OrderedTable RTT Verification

## Task A: Expand OrdKeyMap RTTs

Added 19 new tests to `tests/Chap38/TestOrdKeyMap.rs`.

### New Tests

| # | Category | Test Name |
|---|----------|-----------|
| 1 | union | `test_ordkeymap_union_disjoint` |
| 2 | union | `test_ordkeymap_union_overlapping_other_wins` |
| 3 | union | `test_ordkeymap_union_with_empty` |
| 4 | union | `test_ordkeymap_union_identical` |
| 5 | union | `test_ordkeymap_union_size_correctness` |
| 6 | intersect | `test_ordkeymap_intersect_disjoint` |
| 7 | intersect | `test_ordkeymap_intersect_overlapping` |
| 8 | intersect | `test_ordkeymap_intersect_with_empty` |
| 9 | intersect | `test_ordkeymap_intersect_preserves_self_values` |
| 10 | difference | `test_ordkeymap_difference_disjoint` |
| 11 | difference | `test_ordkeymap_difference_overlapping` |
| 12 | difference | `test_ordkeymap_difference_from_empty` |
| 13 | difference | `test_ordkeymap_difference_identical` |
| 14 | split | `test_ordkeymap_split_at_min` |
| 15 | split | `test_ordkeymap_split_at_max` |
| 16 | split | `test_ordkeymap_split_partition_correctness` |
| 17 | split | `test_ordkeymap_split_union_roundtrip` |
| 18 | integration | `test_ordkeymap_large_map_operations` |
| 19 | integration | `test_ordkeymap_chain_insert_union_split` |

### What Each Category Covers

**union (5 tests)**
- Disjoint maps: all entries from both sides present.
- Overlapping maps: `other` wins on key collision.
- Empty maps: union with empty left/right = identity.
- Identical maps: `other`'s values win on all keys.
- Size correctness: |A ∪ B| = |A| + |B| - |A ∩ B|.

**intersect (4 tests)**
- Disjoint maps: result is empty.
- Overlapping maps: only shared keys, values from `self`.
- Empty intersection: result is empty.
- Value source: confirms `self`'s values, not `other`'s.

**difference (4 tests)**
- Disjoint maps: result equals `self`.
- Overlapping maps: shared keys removed.
- Subtract empty: `self` unchanged.
- Identical maps: result is empty.

**split (4 additional tests)**
- At min key: left is empty, right has all others.
- At max key: right is empty, left has all others.
- Partition correctness: no key appears in both left and right.
- Round-trip: `union(left, right).insert(k, v)` reconstructs original.

**integration (2 tests)**
- Large map (110 entries): insert, find, split correctness.
- Chain: insert-union-split-difference-intersect pipeline with 20-entry maps.

## Task B: Verify OrderedTable RTTs Still Pass

`scripts/rtt.sh` result:
```
Summary [13.994s] 3736 tests run: 3736 passed, 0 skipped
```

All 3736 RTTs pass. No failures or regressions.

## Validation

`scripts/validate.sh` result:
```
verification results:: 5757 verified, 0 errors
```

Zero verification errors. All previously-passing proofs intact.
