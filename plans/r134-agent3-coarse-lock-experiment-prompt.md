# R134 Agent 3 — Write coarse lock + parallel inside experiment. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/architecture-coarse-lock-parallel-mt.md` — this is the design you're implementing.
Read `src/experiments/bst_plain_mt_tsm.rs` — the existing TSM experiment (simpler version).
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the slice-backed sequence type.

Report file: `plans/r134-agent3-coarse-lock-experiment-report.md`

## Task

Write `src/experiments/coarse_lock_parallel_tsm.rs` — an experiment demonstrating
all three layers of the target Mt architecture:

1. **Layer 1**: One coarse `RwLock` wrapping an interior that contains TWO different
   Mt data structures
2. **Layer 2**: TSM token inside the lock, predicate ties token to data, zero assumes
3. **Layer 3**: Parallel operations inside `acquire_write` via `join()` on owned data

## What to build

A module `CollectionMt` that stores:
- An `ArraySeqMtEphSliceS<u64>` (slice-backed sequence — O(1) split)
- A count tracked by a TSM token

Operations:
- `new_empty()` — construct with empty sequence
- `from_vec(v: Vec<u64>)` — construct from vec
- `mt_size(&self)` — read: acquire_read, get length, release
- `mt_sum(&self)` — read: acquire_read, sum elements, release
- `mt_parallel_reduce(&self, f, id)` — read: acquire_read, split sequence via O(1)
  slice, join(reduce_left, reduce_right), combine results, release
- `mt_parallel_map(&self, f)` — write: acquire_write, own the interior, split
  sequence via O(1) slice, join(map_left, map_right), build new sequence from
  results, step TSM, release_write

The key demonstration points:
- `slice()` is O(1) — both halves share the same `Arc<Vec<T>>`
- `join()` works while holding a WriteHandle (it's just a function call)
- TSM token is stepped after parallel work, before release
- No assumes anywhere
- No nested locks
- No unsafe

## Architecture (from the design doc)

```
CollectionMt {
    lock: RwLock<CollectionInterior, CollectionInv>,
    inst: Tracked<CollectionSM::Instance>,
}

CollectionInterior {
    seq: ArraySeqMtEphSliceS<u64>,
    token: Tracked<CollectionSM::count>,
}
```

The `RwLockPredicate` ties `token.value() == seq.length()`.

## For the parallel operations

Use `clone_fn` / `clone_fn2` from `vstdplus::clone_plus` for cloning closures
into join arms. Named closures with explicit `ensures` per standard 8.

For `mt_parallel_reduce`:
```
acquire_read → borrow interior
  let seq = &interior.seq;
  let mid = seq.length() / 2;
  let left = seq.slice(0, mid);        // O(1)
  let right = seq.slice(mid, len-mid); // O(1)
  let (l_result, r_result) = join(
      move || reduce_over(left, f1, id1),
      move || reduce_over(right, f2, id2),
  );
  combine(l_result, r_result)
release_read
```

Wait — `acquire_read` gives a borrow, not owned data. The slices need to be owned
to move into join closures. `slice()` returns a new `ArraySeqMtEphSliceS` (owned,
Arc::clone). So even with acquire_read, you can create owned slices from the
borrowed interior. Check that this works — the slice's Arc keeps the backing alive
even after release_read.

Actually: the slices hold `Arc<Vec<T>>` which keeps the data alive independently
of the lock. So you can create slices from borrowed data, release the lock, and
the slices remain valid. But for the experiment, keep it simple — hold the lock
through the join.

For `mt_parallel_map`:
```
acquire_write → own interior
  destructure interior
  split seq via slice
  join(map_left, map_right) → two new sequences
  append results (or build new from_vec)
  step TSM
  release_write(reassembled interior)
```

## Do NOT register in lib.rs

Per the rule: experiments stay commented out. Write the file but do NOT add it to
lib.rs. Leave a comment at the top noting it's an experiment.

## Validation

You can temporarily uncomment the experiment in lib.rs to validate, but comment it
back out before committing. Run `scripts/validate.sh` to verify, then
`scripts/rtt.sh` for the test.

Include a `#[test]` function that exercises all operations.

## Rules

- Zero assumes, zero accepts, zero external_body (except clone_fn which is vstdplus)
- Named closures with explicit ensures for every join() call
- TSM token inside the lock, predicate ties to data
- Use ArraySeqMtEphSliceS for O(1) slicing
- Read standard 8 (closures) and standard 15 (HFScheduler) carefully

## When done

RCP.
