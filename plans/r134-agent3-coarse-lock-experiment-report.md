# R134 Agent 3 — Coarse Lock + Parallel Inside Experiment

## Summary

Wrote `src/experiments/coarse_lock_parallel_tsm.rs` — a complete experiment
demonstrating all three layers of the target Mt architecture:

1. **Layer 1 (Thread Safety)**: One coarse `RwLock<CollectionInterior, CollectionInv>`
   wrapping a sequence + TSM token.
2. **Layer 2 (Zero Assumes)**: TSM token inside the lock; `RwLockPredicate` ties
   `token.count == seq.length` and `seq.wf()`. No assumes, no accepts.
3. **Layer 3 (Parallel Inside)**: `join()` on owned O(1) slices from
   `ArraySeqMtEphSliceS<u64>`, both for read (parallel reduce) and write
   (parallel map) operations.

## Architecture

```
CollectionMt {
    lock: RwLock<CollectionInterior, CollectionInv>,
    inst: Tracked<CollectionSM::Instance>,
}

CollectionInterior {
    seq: ArraySeqMtEphSliceS<u64>,    // O(1) slice via Arc<Vec>
    token: Tracked<CollectionSM::count>,
}

CollectionInv ties: token.value() == seq.length && seq.wf()
```

## Operations Implemented

| # | Operation | Lock | Parallel | Notes |
|---|-----------|------|----------|-------|
| 1 | `new_empty()` | — | — | Init TSM with count=0 |
| 2 | `from_vec(v)` | — | — | Init TSM with count=v.len() |
| 3 | `mt_size()` | read | no | acquire_read, length, release |
| 4 | `mt_parallel_reduce(f, id)` | read | **yes** | O(1) slice split, join, combine |
| 5 | `mt_parallel_map(f)` | write | **yes** | O(1) slice split, join, rebuild seq |

## Key Patterns Demonstrated

- **O(1) slicing**: `ArraySeqMtEphSliceS::slice()` does Arc::clone + window adjust.
  Both halves share the same backing `Vec<u64>`. Cost: O(1).
- **Owned slices for join**: Slices hold their own `Arc<Vec<u64>>`, so they're
  `Send + 'static` and can move into `join()` closures independently of the lock.
- **clone_fn/clone_fn2**: Closure cloning with spec preservation for join arms.
  `clone_fn` for `Fn(&u64) -> u64` (map), `clone_fn2` for `Fn(&u64, &u64) -> u64` (reduce).
- **Named closures with ensures**: Each join arm has explicit `requires` and `ensures`
  for spec propagation through `join()`.
- **TSM token unchanged for map**: Map preserves sequence length, so the TSM token
  doesn't need stepping. The predicate `token.count == seq.length` holds because
  both sides equal the original length.
- **Release-before-join for reads**: `mt_parallel_reduce` releases the read lock
  before `join()` — the slices are independent (own their Arc).
- **Hold-through-join for writes**: `mt_parallel_map` holds the write lock through
  the entire operation to ensure exclusive access for the interior mutation.

## Proof Highlights

- Zero assumes, zero accepts, zero external_body in the experiment code itself.
  (HFScheduler's `join()` and `clone_fn`/`clone_fn2` are external_body but those
  are pre-existing infrastructure, not experiment code.)
- TSM predicate proved on every `release_write` via ghost variable chain:
  `orig_token_val == len == new_seq.spec_len()`.
- `seq_map_u64` ensures `result@.len() == seq.spec_len()` — length preservation
  carries through the join and combine loop to prove the predicate on release.
- `obeys_feq_clone::<u64>()` required for `nth_cloned` access; provided by
  `group_feq_axioms` broadcast group.

## Verification Results

- **Verified**: 5484 verified, 0 errors (full validate)
- **RTT**: 3584 passed, 0 failed (test_coarse_lock_parallel_tsm passes)
- **Status**: SUCCEEDS — commented out in lib.rs per experiment rules

## Test Coverage

The test exercises:
- Empty collection (size=0, reduce returns identity)
- 8-element collection from vec
- Parallel reduce (sum = 36)
- Parallel map (double → sum = 72)
- Chained map + reduce (add 1 → sum = 80)
- Single element (size=1, reduce, map)

## What This Proves for the Architecture

The experiment validates that:
1. RwLock + TSM gives zero-assume thread safety
2. `ArraySeqMtEphSliceS::slice()` produces owned data suitable for `join()`
3. `join()` works while holding a write handle (it's just a function call)
4. TSM token correctly tracks invariants through parallel operations
5. `clone_fn`/`clone_fn2` successfully clone closures for join arms
6. The three layers compose without conflicts

This is the foundation for migrating real Mt modules to the coarse lock +
parallel inside pattern described in `docs/architecture-coarse-lock-parallel-mt.md`.
