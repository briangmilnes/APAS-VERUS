# R139 Agent 4 — Implement Blelloch parallel prefix scan. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `prompts/Chap18.txt` — APAS describes scan as "same work and span of reduce."
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the slice-backed type with O(1) split.
Read `src/Chap19/ArraySeqMtEphSlice.rs` reduce_dc — the existing parallel reduce
as a reference for the D&C + join pattern.

Report file: `plans/r139-agent4-blelloch-scan-report.md`

## Problem

APAS CS 20.5: scan has Work O(|a|), Span O(lg |a|). Our implementations are
sequential O(n) loops. Parallel scan requires Blelloch's up-sweep/down-sweep
algorithm (parallel prefix sum).

## The algorithm

Blelloch parallel prefix scan on a sequence `[a0, a1, ..., an-1]` with
associative operator `f` and identity `id`:

**Phase 1 — Up-sweep (reduce):** Build a balanced binary tree of partial
reductions bottom-up. At each level, combine pairs. This is exactly reduce —
O(n) work, O(lg n) span.

**Phase 2 — Down-sweep:** Propagate prefixes top-down. At each node, the left
child gets the prefix from above, the right child gets `f(prefix_from_above,
left_child_reduction)`. O(n) work, O(lg n) span.

The result: `scan(a, f, id) = [id, f(id, a0), f(f(id, a0), a1), ...]`

For sequences, this means:
```
scan([a,b,c,d], +, 0) = ([0, a, a+b, a+b+c], a+b+c+d)
```

Returns a pair: (prefix sequence, total).

## Where to implement

Add `scan` to `ArraySeqMtEphSliceS<T>` in `src/Chap19/ArraySeqMtEphSlice.rs`.
This is the slice-backed type with O(1) split — ideal for Blelloch because both
phases split the sequence at the midpoint.

The trait already has an interface for scan (from the Vec-backed types). Match the
same ensures:

```rust
fn scan<F: MtReduceFn<T>>(&self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T)
    -> (scanned: (Self, T))
    requires
        spec_monoid(spec_f, id),
        ...
    ensures
        scanned.0.spec_len() == self.spec_len(),
        // scanned.0[i] == fold_left of first i elements
        scanned.1 == spec_iterate(self@, spec_f, id),  // total
```

## Implementation sketch

```
fn scan(&self, f, spec_f, id) -> (prefix_seq, total) {
    if len <= 1 {
        // base case
    } else {
        let mid = len / 2;
        let left = self.slice(0, mid);       // O(1)
        let right = self.slice(mid, len-mid); // O(1)

        // Up-sweep: reduce both halves in parallel
        let (left_prefixes, left_total) = join(
            || left.scan(f, spec_f, id),
            || right.scan(f, spec_f, id),
        );

        // The right half's prefixes need to be shifted by left_total
        // right_prefix[i] = f(left_total, right.scan_result[i])
        // This is a map over right_prefixes: prepend left_total
        ...

        // Combine
        let total = f(left_total, right_total);
        (combined_prefixes, total)
    }
}
```

Actually, the naive recursive scan above is O(n lg n) work because the "shift
right prefixes" step is O(n/2) at each level. Blelloch's algorithm avoids this
by separating up-sweep and down-sweep into two passes.

**Alternative (simpler, still O(lg n) span):** Use the contraction-based approach
from APAS. Reduce even/odd pairs, recursively scan the reduced sequence, then
expand back. This is equivalent to Blelloch but easier to implement recursively.

Read the APAS description carefully before choosing the approach.

## Proof requirements

- `spec_monoid(spec_f, id)` — associativity and identity
- The prefix at position i equals `fold_left` of the first i elements
- The total equals `fold_left` of all elements (same as reduce)
- Use `lemma_monoid_fold_left` (already exists in the reduce implementation)

## Also implement for ArraySeqMtEphS (Vec-backed)

After the slice version works, add the same scan to `src/Chap18/ArraySeqMtEph.rs`
and `src/Chap19/ArraySeqMtEph.rs` (using subseq_copy instead of slice — O(n) split,
so span is O(n) not O(lg n), but the algorithm is the same).

If time is short, do the slice version only.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.
Add RTTs for scan in the test file.

## Rules

- Named closures with explicit ensures for every join() call.
- Do NOT add assumes, accepts, or external_body.
- Use MtReduceFn from Concurrency.rs, not raw bounds.
- Use clone_fn2 for cloning the binary operator.
- The scan must be O(n) work, O(lg n) span with slice-backed sequences.

## When done

RCP.
