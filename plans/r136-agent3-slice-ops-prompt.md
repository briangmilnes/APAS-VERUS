# R136 Agent 3 — Add map, reduce, filter to ArraySeqMtEphSliceS. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- Standard 8 (`using_closures_standard.rs`) — named closures with ensures
- Standard 15 (`hfscheduler_standard.rs`) — join() patterns
- Standard 23 (`mt_type_bounds_standard.rs`) — use MtReduceFn, MtMapFn, MtPred

Report file: `plans/r136-agent3-slice-ops-report.md`

## Problem

`src/Chap19/ArraySeqMtEphSlice.rs` has O(1) slicing (`slice()` = Arc::clone +
window adjust) but no higher-order operations. It needs map, reduce, and filter
that exploit the O(1) split for D&C parallelism.

## Reference implementations

Read these files for the pattern — they implement the same operations on Vec-backed
sequences. Your versions use the same D&C + join() structure but with O(1) slice
split instead of O(n) subseq_copy:

- `src/Chap19/ArraySeqMtEph.rs` — map_dc (~line 1361), reduce (delegates to
  reduce_par ~line 910), filter_dc
- `src/Chap18/ArraySeqMtEph.rs` — map_dc, reduce_dc, filter_dc

## What to add to ArraySeqMtEphSlice.rs

### 1. reduce

```rust
fn reduce<F: MtReduceFn<u64>>(&self, f: &F, Ghost(spec_f): Ghost<spec_fn(u64, u64) -> u64>, id: u64) -> (reduced: u64)
```

D&C: if len <= 1, base case. Otherwise `slice(0, mid)` and `slice(mid, len-mid)`
(both O(1)), join(reduce_left, reduce_right), combine with f.

The ensures should match the Chap18/19 pattern: `reduced == spec_iterate(...)`.

### 2. map

```rust
fn map<U: StTInMtT, F: MtMapFn<u64, U>>(&self, f: &F) -> (mapped: ArraySeqMtEphSliceS<U>)
```

D&C: if len <= 1, base case. Otherwise slice split O(1), join(map_left, map_right),
combine results. The two halves each produce a Vec via a sequential inner map,
then wrap in from_vec. Combine by building a new Vec from both results.

Note: the output type is `ArraySeqMtEphSliceS<U>`, not `ArraySeqMtEphSliceS<u64>`.
The mapped type may differ from the input type.

Actually — consider whether map should be generic over T, not hardcoded to u64.
The slice type is `ArraySeqMtEphSliceS<T>`. The operations should be generic too.

### 3. filter

```rust
fn filter<F: MtPred<T>>(&self, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
```

D&C: slice split O(1), join(filter_left, filter_right), combine. Use the multiset
distribution lemma from `vstdplus/multiset.rs` if the ensures includes the multiset
postcondition.

## Key difference from Vec-backed D&C

The split is O(1): `self.slice(0, mid)` instead of `self.subseq_copy(0, mid)`.
Both return owned `ArraySeqMtEphSliceS` values (Arc::clone is O(1)). The join
arms own their slices. This is the whole point of the slice-backed type.

The rejoin is still O(n) (building a new Vec from results). That's the cost
analysis: split O(1), parallel work O(n)/O(lg n) span, rejoin O(n).
Reduce avoids the rejoin (scalar result) → true O(lg n) span.

## Use trait aliases

Use `MtReduceFn<T>`, `MtMapFn<T, U>`, `MtPred<T>` from Concurrency.rs. Do NOT
spell out `F: Fn(&T, &T) -> T + Clone + Send + Sync + 'static`. Standard 23.

## Clone closures

Use `clone_fn`, `clone_fn2`, `clone_pred` from `vstdplus::clone_plus` for cloning
closures into join arms. Standard 8.

## Add to the trait

Add the new methods to `ArraySeqMtEphSliceTrait<T>` (both declaration and impl).

## Update the experiment

After adding the operations, update `src/experiments/coarse_lock_parallel_tsm.rs`
to switch back to `ArraySeqMtEphSliceS` and call its new reduce/map directly.
This demonstrates the full architecture: coarse lock + TSM + O(1) slice split +
Mt type's own parallel operations.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.
Temporarily uncomment the experiment in lib.rs to validate it too, then comment
back out.

## Rules

- Named closures with explicit ensures for every join() call.
- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Use trait aliases (standard 23), not raw bounds.
- Add RTTs for the new operations in `tests/Chap19/TestArraySeqMtEphSlice.rs`
  (if the test file exists) or create one.

## When done

RCP.
