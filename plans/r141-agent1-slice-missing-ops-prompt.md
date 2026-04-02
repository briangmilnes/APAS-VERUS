# R141 Agent 1 — Add missing operations to ArraySeqMtEphSlice. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the slice-backed type.
Read `src/Chap19/ArraySeqMtEph.rs` — the Vec-backed reference implementations.

Report file: `plans/r141-agent1-slice-missing-ops-report.md`

## Problem

`ArraySeqMtEphSliceS<T>` is missing 7 operations that the Vec-backed version has.
These need to be added to match APAS's sequence interface.

## Trivial operations (add to trait + impl)

These are O(1) and straightforward:

1. `is_empty(&self) -> bool` — `self.len == 0`
2. `is_singleton(&self) -> bool` — `self.len == 1`
3. `set(&self, index: usize, item: T) -> Self` — clone backing Vec, set one
   element, return new slice. O(n) work (clone), O(1) for the actual set.

Match the ensures from the Vec-backed version in `src/Chap19/ArraySeqMtEph.rs`.

## append

`append(a: &Self, b: &Self) -> Self` — concatenate two sequences.

For slices: allocate new Vec of size a.len + b.len, copy both halves, wrap in
from_vec. O(n+m) work. Cannot be O(1) because the two slices may have different
backing Arcs.

Match ensures from Vec-backed version.

## update

`update(a: &Self, index: usize, item: T) -> Self` — return new sequence with
one element changed.

For slices: clone the slice window into a new Vec, set the element, wrap in
from_vec. O(n) work.

Match ensures from Vec-backed version.

## inject

`inject(a: &Self, updates: &Vec<(usize, T)>) -> Self` — apply multiple updates.

For slices: clone the slice window into a new Vec, apply each update. O(n + m)
work where m = updates.len().

Match ensures from Vec-backed version.

## ninject

`ninject(a: &Self, updates: &Vec<(usize, T)>) -> Self` — inject with
non-overlapping guarantee.

Delegates to inject. Match ensures from Vec-backed version.

## Also add RTTs

Add runtime tests for all 7 new operations.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Match the ensures from the Vec-backed versions exactly.
- Add operations to both the trait and the impl.

## When done

RCP.
