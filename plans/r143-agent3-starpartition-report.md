# R143 Agent 3 — StarPartition Loops 1, 5 Parallelization

## Summary

Parallelized loops 1 and 5 in `src/Chap62/StarPartitionMtEph.rs`, eliminating the two
sequential bottlenecks that caused the DIFFERS annotation. All 6 loops are now parallel D&C.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarPartitionMtEph.rs | Parallelized Loop 1 (vertex_to_index) and Loop 5 (inject) |

### New D&C functions

| # | Function | Replaces | Work | Span |
|---|----------|----------|------|------|
| 1 | `build_vertex_to_index_mt` | Loop 1 (sequential while) | O(n lg n) | O(lg n) |
| 2 | `build_satellite_map_mt` | Part of Loop 5 — builds satellite→center map from th_edges | O(m lg m) | O(lg m) |
| 3 | `build_p_vec_with_inject_mt` | Loops 4+5 — builds p_vec with satellite injection in one pass | O(n) | O(lg n) |

### Arc clone helpers

Added 5 type-specific `external_body` Arc clone helpers with `ensures cloned@ == arc@`.
Required because vstd has no ensures on `Arc::clone`, preventing callers from connecting
function ensures (which reference Arc parameter views) back to local variable views.

### Conjunction flakiness fix

Split `build_partition_map_mt` ensures and `f_pm` closure ensures from:
```
forall|j| ... ==> contains_key(k) && map[k]@ == expected
```
to:
```
forall|j| ... ==> contains_key(k),
forall|j| ... ==> contains_key(k) ==> map[k]@ == expected,
```
Both sub-assertions proved individually but the conjunction failed under full-validation
Z3 load. Classic conjunction flakiness pattern.

## Cost analysis update

| Annotation | Before | After |
|---|---|---|
| Trait | Work O(n+m), Span O(n+lg m) — DIFFERS | Work O((n+m) lg(n+m)), Span O(lg(n+m)) |
| Impl | Same | Same |

The DIFFERS was about span (sequential loops gave O(n+m) span). Span is now O(lg(n+m)).
Work increased from O(n+m) to O((n+m) lg(n+m)) due to D&C HashMap merge overhead — same
trade-off as the existing parallel functions in this file.

## DIFFERS status

The 2 annotation sites (lines 55, 845 in original) both said "DIFFERS: Loops 1, 5
sequential". Both are now updated — all loops parallel. The DIFFERS is resolved: span
matches APAS O(lg n) (modulo the lg(n+m) from D&C merges). Work is O((n+m) lg(n+m))
vs APAS O(n+m), which is the standard trade-off for parallel HashMap construction.

## Verification

- Full validation: 5688 verified, 0 errors (was 5684)
- RTT: 3690 passed
- PTT: 221 passed
