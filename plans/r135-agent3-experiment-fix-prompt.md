# R135 Agent 3 — Fix coarse lock experiment: use Mt type's own operations. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/architecture-coarse-lock-parallel-mt.md` — especially section 2 (locked/unlocked traits) and section 3 (Mt inside Mt).

Report file: `plans/r135-agent3-experiment-fix-report.md`

## Problem

Your `src/experiments/coarse_lock_parallel_tsm.rs` introduces standalone helper
functions `seq_map_u64` and `seq_reduce_u64` that re-implement map and reduce
as sequential loops over Vecs. This misses the entire point of the architecture.

The architecture says: store Mt types inside the lock, then call the Mt type's
OWN parallel operations on the owned data after acquire. The Mt type already has
map, reduce, filter with internal parallelism. Don't rewrite them.

## What to fix

1. **Remove `seq_map_u64` and `seq_reduce_u64`.** These are redundant — the
   `ArraySeqMtEphSliceS` type already has operations for these.

2. **For `mt_parallel_reduce`**: after creating slices from the borrowed interior,
   call the slice type's own reduce (or reduce_par / reduce_inner if available).
   If `ArraySeqMtEphSliceS` doesn't have a reduce method, use the Chap18/19
   `ArraySeqMtEph` reduce_par pattern on the slices — but call it, don't rewrite it.

3. **For `mt_parallel_map`**: after acquiring write and splitting, call the slice
   type's own map operation on each half inside the join arms. The join arms should
   call the Mt type's method, not a hand-rolled loop.

4. If `ArraySeqMtEphSliceS` doesn't yet have map/reduce methods, then the experiment
   should demonstrate the pattern using a type that DOES have them. Read what methods
   `ArraySeqMtEphSliceS` actually provides (check its trait). If it only has
   construction/access/slicing, you may need to either:
   - Use `ArraySeqMtEphS` (Chap18) instead of slice-backed, OR
   - Add map/reduce to the slice type's trait (which is real work, not experiment-only), OR
   - Document that the slice type needs these operations and keep the helpers as a
     temporary stand-in, clearly marked as such

   The point is: the experiment should show the ARCHITECTURAL pattern of calling
   the inner Mt type's own operations, even if the current slice type is incomplete.

5. Keep everything else: the TSM, the RwLock, the zero assumes, the test.

## Key principle from the architecture doc

> When M1 stores M2 inside its lock, M1's locked trait calls M2's **unlocked**
> trait directly on the owned M2 data.

The experiment should demonstrate this — not work around it with standalone helpers.

## Validation

Temporarily uncomment in lib.rs to validate. Comment back out before committing.

## When done

RCP.
