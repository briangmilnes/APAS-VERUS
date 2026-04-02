# R135 Agent 3 — Fix coarse lock experiment: use Mt type's own operations

## Summary

Rewrote `src/experiments/coarse_lock_parallel_tsm.rs` to call the inner Mt type's
own parallel operations instead of re-implementing them as standalone sequential loops.

## What changed

| # | Change | Detail |
|---|--------|--------|
| 1 | Removed `seq_reduce_u64` | Standalone sequential reduce loop — redundant |
| 2 | Removed `seq_map_u64` | Standalone sequential map loop — redundant |
| 3 | Switched inner type | `ArraySeqMtEphSliceS<u64>` → `ArraySeqMtEphS<u64>` (has reduce/map) |
| 4 | `mt_parallel_reduce` | Now calls `ArraySeqMtEphTrait::reduce` (parallel D&C internally) |
| 5 | `mt_parallel_map` | Now calls `ArraySeqMtEphTrait::map` (parallel D&C internally) |
| 6 | Removed imports | `HFSchedulerMtEph::join`, `clone_fn`, `clone_fn2` no longer needed |
| 7 | Added imports | `monoid::spec_monoid` (required by reduce's spec) |

## Why ArraySeqMtEphS instead of ArraySeqMtEphSliceS

`ArraySeqMtEphSliceS` (Chap19, slice-backed) provides construction, access, and O(1)
slicing — but no reduce or map operations. `ArraySeqMtEphS` (Chap19, Vec-backed) has
the full trait including parallel `reduce` (D&C via `reduce_par` + `join`) and `map`
(D&C via `map_dc` + `join`).

The experiment now demonstrates the architectural pattern correctly: after acquiring
the lock, call the inner Mt type's own unlocked operations. The Mt type handles
parallelism internally — the outer module doesn't split, loop, or join.

## Architectural pattern demonstrated

```
acquire_write → own interior
  → call interior.seq's own parallel map (D&C + join internally)
  → TSM token unchanged (map preserves length)
  → release_write
```

```
acquire_read → borrow interior
  → call interior.seq's own parallel reduce (D&C + join internally)
  → release_read → return scalar
```

## Validation

- Verification: 5479 verified (with experiment), 5472 (without) — 0 errors
- RTT: 3584 passed
- PTT: 221 passed
- Zero assumes, zero accepts, zero external_body in experiment code
