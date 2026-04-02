# R143 Agent 1 — Parallel partition for QuickSortMtEphSlice. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap36/QuickSortMtEphSlice.rs` — the slice-backed quicksort.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — filter_dc_vec pattern (parallel filter).
Read `prompts/Chap36.txt` — APAS cost specs for quicksort.

Report file: `plans/r143-agent1-parallel-partition-report.md`

## Problem

6 DIFFERS in Chap36 quicksort (3 in Vec-backed, 3 in slice-backed). All caused
by sequential O(n) partition dominating span. APAS says partition is two parallel
filters: elements < pivot and elements >= pivot.

Fix the slice-backed version. The Vec-backed version gets ACCEPTED DIFFERENCE
(same as other Vec-backed sequences).

## The algorithm

APAS parallel partition: given sequence `s` and pivot `p`:
```
left  = filter(s, |x| x < p)
right = filter(s, |x| x >= p)
```

Two independent filters — run them in parallel via `join()`. Each filter is
itself parallel (D&C via filter_dc_vec). Combined: O(n) work, O(lg n) span.

The current sequential partition scans left-to-right building two Vecs.
Replace it with `join(filter(<pivot), filter(>=pivot))`.

## Where to change

`QuickSortMtEphSlice.rs` has a `partition` helper (or inline partition code)
inside each sort variant. Find it — it's the sequential loop that splits
elements around the pivot.

Replace the sequential partition with:
```rust
let pivot_clone = pivot.clone();
let f_less = |x: &T| -> bool { TotalOrder::cmp(x, &pivot) == Ordering::Less };
let f_geq  = |x: &T| -> bool { TotalOrder::cmp(x, &pivot) != Ordering::Less };
let (left, right) = join(
    || s.filter(&f_less, Ghost(spec_less)),
    || s.filter(&f_geq, Ghost(spec_geq)),
);
```

The filter function on ArraySeqMtEphSliceS is already parallel (D&C internally).
You need to define the predicates with proper spec fns and use clone_pred for
the join arms.

## Proof requirements

- The union of left and right elements equals the original (as multisets)
- left contains only elements < pivot
- right contains only elements >= pivot
- These follow from filter's ensures

## Also annotate Vec-backed as ACCEPTED DIFFERENCE

In `QuickSortMtEph.rs`, change the 3 DIFFERS annotations to:
```
— ACCEPTED DIFFERENCE: Vec-backed; see QuickSortMtEphSlice for parallel partition
```

## Validation

Run `scripts/validate.sh isolate Chap36`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Named closures with explicit ensures for join() calls.
- Use clone_pred for cloning filter predicates.
- The partition must be parallel — two filters via join().

## When done

RCP.
