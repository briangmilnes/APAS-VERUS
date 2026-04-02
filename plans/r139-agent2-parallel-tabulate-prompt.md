# R139 Agent 2 — Implement parallel tabulate for ArraySeqMtEphSlice. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap18.txt` — APAS Definition 18.6, tabulate.
Read `prompts/Chap20.txt` — APAS CS 20.2, tabulate cost spec.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the slice-backed type.
Read `src/Chap19/ArraySeqMtEph.rs` — the Vec-backed tabulate (sequential reference).
Read `src/Concurrency.rs` — MtTabulateFn trait alias.

Report file: `plans/r139-agent2-parallel-tabulate-report.md`

## Problem

APAS CS 20.2: tabulate has Work O(n * W(f)), Span O(lg n + S(f)).

`ArraySeqMtEphSliceS<T>` (slice-backed, O(1) split) does not have `tabulate` yet.
The Vec-backed version in `src/Chap19/ArraySeqMtEph.rs` is a sequential loop.

## The algorithm

Tabulate is embarrassingly parallel. Each position i just calls f(i).

D&C approach (matches reduce/map pattern already in the file):
```
fn tabulate(f, length) -> Self {
    if length <= 1 {
        // base: empty or singleton
    } else {
        let mid = length / 2;
        let (left, right) = join(
            || tabulate(f, 0..mid),        // tabulate first half
            || tabulate(f, mid..length),   // tabulate second half
        );
        append(left, right)  // combine
    }
}
```

The function f needs an offset parameter so the right half calls f(mid + i).
Use the same clone_fn pattern as reduce_dc/map_dc_vec in the file.

Actually, since tabulate creates a NEW sequence (not splitting an existing one),
the D&C builds two halves and appends. This gives:
- Work: O(n * W(f)) — same total work as sequential
- Span: O(lg n * (S(f) + W(append))) — lg n levels, each with f + append

For append to be O(1) on slices, both halves need to be adjacent in the same
backing Vec. The simplest correct approach:

**Option A (simple, O(n) rejoin):** Build each half as a Vec, then concatenate.
This is O(n) work per level, O(n lg n) total — too much.

**Option B (pre-allocate):** Pre-allocate the full Vec, then fill ranges in parallel.
This gives O(n) work, O(lg n) span — but requires mutable disjoint writes.

**Option C (Vec-based D&C, same as map_dc_vec):** Follow the exact pattern of
`map_dc_vec` already in the file. Build Vec results in each half, concatenate with
`extend`. This is O(n) work per join, but the span is O(lg n) because joins are
at lg n levels with max O(n/2^k) work at level k — total work O(n).

Use **Option C** — it matches the existing `map_dc_vec` pattern and is proven in the file.

## Where to implement

Add to `ArraySeqMtEphSliceS<T>` impl in `src/Chap19/ArraySeqMtEphSlice.rs`.

Add to the trait first (interface), then the impl.

The trait signature:
```rust
fn tabulate<F: MtTabulateFn<T>>(
    f: &F,
    Ghost(spec_f): Ghost<spec_fn(usize) -> T>,
    length: usize,
) -> (tab: Self)
    requires
        forall|i: usize| #[trigger] spec_f(i) == f.requires((i,)),
        // f produces valid values for all indices
    ensures
        tab.spec_len() == length as nat,
        forall|i: int| 0 <= i < length ==> #[trigger] tab@[i] == spec_f(i as usize)@,
```

Check the Vec-backed version's exact ensures and match them.

## Implementation

Follow `map_dc_vec` as your template:
1. Base case: length 0 → empty, length 1 → singleton(f(0))
2. Recursive: split length at midpoint, join two recursive calls
3. The right half needs f called with offset: `f(mid + i)`. Capture `mid` in the
   closure. The ghost spec should reflect this offset.
4. Combine results with Vec extend (or the from_vec + append pattern)
5. Use `clone_fn` from `src/vstdplus/clone_plus.rs` to clone f for both arms
6. Named closures with explicit ensures for both join arms

## Also add to the trait

If `tabulate` is not in the trait declaration yet, add it. Check the trait in the
same file.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.
Add RTTs for tabulate.

## Rules

- Named closures with explicit ensures for every join() call.
- Do NOT add assumes, accepts, or external_body.
- Use MtTabulateFn from Concurrency.rs, not raw bounds.
- Use clone_fn for cloning the tabulate function.
- Follow the map_dc_vec pattern exactly for the D&C structure.

## When done

RCP.
