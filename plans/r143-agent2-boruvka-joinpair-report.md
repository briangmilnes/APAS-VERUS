# R143 Agent 2 — Boruvka DIFFERS + BST join_pair Report

## Summary

Investigated 5 DIFFERS annotations across Chap66 (Boruvka) and Chap38 (BST join_pair).
4 Boruvka DIFFERS were annotation errors — the code implements Algorithm 66.3 (star
contraction, O(lg^2 n) span) but the annotations referenced Algorithm 66.1 (tree
contraction, O(lg^3 n) span). Fixed all 4. The BST join_pair DIFFERS is genuine —
documented with clearer explanation.

## Problem 1: Boruvka (4 DIFFERS resolved)

### Finding

The annotations referenced APAS Algorithm 66.1 (basic Boruvka with tree contraction)
which has O(lg^3 n) span. But the code implements Algorithm 66.3 (bridge star partition)
which has O(lg^2 n) span. The code review correctly computed O(lg^2 n) span, then
incorrectly flagged it as DIFFERS because it was comparing against the wrong algorithm.

The "sequential O(lg n) loop" noted in the annotation is NOT a deviation — Boruvka's
algorithm IS a sequential loop of O(lg n) rounds, each doing parallel work. This is
exactly what APAS describes. O(lg n) rounds x O(lg n) per round = O(lg^2 n) span.

### Changes

| # | Chap | File | Line | Change |
|---|------|------|------|--------|
| 1 | 66 | BoruvkaMtEph.rs | 134 | Trait boruvka_mst_mt: Alg 66.1 → Alg 66.3, DIFFERS → matches APAS |
| 2 | 66 | BoruvkaMtEph.rs | 153 | Trait boruvka_mst_mt_with_seed: same fix |
| 3 | 66 | BoruvkaMtEph.rs | 762 | Impl boruvka_mst_mt: same fix |
| 4 | 66 | BoruvkaMtEph.rs | 994 | Impl boruvka_mst_mt_with_seed: same fix |

## Problem 2: BST join_pair (1 DIFFERS — genuine, documented)

### Finding

APAS Algorithm 38.4 (joinPair) assumes T1 < T2 (disjoint, ordered): find min key of
T2, split T2, joinM. Cost: O(lg n) work, O(lg n) span.

Our `join_pair` has ensures `self@.union(other@)` — a general union without the disjoint
precondition. It delegates to `union_inner` (parallel D&C union) giving O(m·lg(n/m))
work, O(lg n) span.

The DIFFERS is correct — our function is strictly more general than APAS joinPair.
The disjoint case is handled by `join_pair_inner` which has the proper preconditions
and O(lg n) cost.

### Changes

| # | Chap | File | Line | Change |
|---|------|------|------|--------|
| 5 | 38 | BSTParaMtEph.rs | 348 | Trait join_pair: clarified DIFFERS reason (general union vs APAS disjoint join) |
| 6 | 38 | BSTParaMtEph.rs | 654 | Impl join_pair: updated annotation to match trait, consistent work/span |

## Validation

- `scripts/validate.sh isolate Chap66`: 860 verified, 0 errors
- `scripts/validate.sh isolate Chap38`: 1156 verified, 0 errors
- `scripts/rtt.sh`: 3690 tests passed
