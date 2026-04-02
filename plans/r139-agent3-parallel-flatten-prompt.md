# R139 Agent 3 — Implement parallel flatten for ArraySeqMtEphSlice. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap18.txt` — APAS Definition 18.12, flatten.
Read `prompts/Chap20.txt` — APAS CS 20.2, flatten cost spec.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the slice-backed type.
Read `src/Chap19/ArraySeqMtEph.rs` — the Vec-backed flatten (sequential reference).

Report file: `plans/r139-agent3-parallel-flatten-report.md`

## Problem

APAS: flatten has Work O(Sigma |a_i|), Span O(lg |a| + max |a_i|).

`ArraySeqMtEphSliceS<T>` does not have `flatten` yet. The Vec-backed version in
`src/Chap19/ArraySeqMtEph.rs` is nested sequential loops.

## The algorithm

`flatten` takes a sequence of sequences and concatenates them into one.

```
flatten([[1,2], [3], [4,5,6]]) = [1,2,3,4,5,6]
```

D&C approach:
```
fn flatten(a: &Self_of_Self) -> Self {
    if a.length() == 0 { return empty(); }
    if a.length() == 1 { return a.nth(0).clone(); }

    let mid = a.length() / 2;
    let left_half = a.slice(0, mid);      // O(1) slice
    let right_half = a.slice(mid, a.length() - mid);  // O(1) slice

    let (left_flat, right_flat) = join(
        || flatten(&left_half),
        || flatten(&right_half),
    );

    append(left_flat, right_flat)
}
```

This is the standard D&C flatten. The span is O(lg |a|) levels of recursion
plus the cost of append at each level. If append is O(n), total work is
O(sum |a_i| * lg |a|) which is too much.

For correct APAS span, we need either:
- O(1) append (slice-backed adjacent merge — only if both halves share backing)
- Pre-allocated output with parallel scatter

**Practical approach:** Use the same Vec-based D&C pattern as `map_dc_vec`.
Build Vec results, extend them. This gives:
- Work: O(sum |a_i|) — each element copied once at each level, but geometrically
  decreasing → O(sum |a_i|)
- Span: O(lg^2 |a| + max |a_i|) — lg |a| levels, each with O(lg) for the join

This is close to APAS. The extra lg factor comes from Vec concatenation.
Accept this for now (matches the map/filter pattern).

## Type signature

The tricky part is that flatten operates on `ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>`.
The inner type must be the same slice-backed type. Check how the Vec-backed version
handles this in `src/Chap19/ArraySeqMtEph.rs`.

```rust
fn flatten(a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>) -> (flattened: ArraySeqMtEphSliceS<T>)
    requires
        a.spec_arrayseqmtephslice_wf(),
        forall|i: int| 0 <= i < a.spec_len()
            ==> (#[trigger] a.spec_index(i)).spec_arrayseqmtephslice_wf(),
    ensures
        // flattened view equals the concatenation of all inner views
        flattened.spec_arrayseqmtephslice_wf(),
        flattened.spec_len() == spec_sum_lengths(a@),
        // element-wise: flatten@[k] = a[i][j] for appropriate i,j
```

Check the Vec-backed version's ensures and match them.

## Implementation

Follow `map_dc_vec` as template:
1. Base: length 0 → empty, length 1 → clone the single inner sequence
2. Recursive: slice the outer sequence at midpoint, join two recursive flattens
3. Combine: extend left Vec with right Vec, wrap in from_vec
4. Named closures, clone_fn for any captured closures (flatten itself is recursive,
   not a captured closure — but the outer sequence reference needs careful handling)

Since flatten is recursive and captures `&self` (the outer sequence), the join arms
capture slices of the outer sequence. These are `&` references, which are Send.

## Also implement append

If `ArraySeqMtEphSliceS` doesn't have `append` yet, you'll need it for flatten.
Append for slices: create a new Vec from both halves, wrap in from_vec. O(n+m) work.

Check if append exists first. If not, add it to both trait and impl.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.
Add RTTs for flatten (and append if added).

## Rules

- Named closures with explicit ensures for every join() call.
- Do NOT add assumes, accepts, or external_body.
- Use clone_fn/clone_fn2 for any closure cloning needed.
- Follow the map_dc_vec pattern for D&C structure.

## When done

RCP.
