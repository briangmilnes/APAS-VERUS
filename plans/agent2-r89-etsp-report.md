# R89 Agent 2 Report: Fix ETSPStEph rlimit matching loop

## Objective

Uncomment `ETSPStEph.rs` (Chap26) and fix the rlimit exceeded error on
`lemma_combined_cycle` caused by a Z3 matching loop on modular sequence indexing.

## Result

**Fixed.** ETSPStEph.rs is uncommented and fully verified with 0 holes.

## Technique: Closed Spec Fn to Break Trigger Chain

The matching loop occurred because `spec_edges_form_cycle` had trigger `tour[i]`
with body containing `tour[((i + 1) % tour.len())]`. When Z3 instantiated the
forall for index `i`, it produced a new `tour[...]` term at `(i+1) % n`, which
triggered another instantiation, looping endlessly. This cascaded through three
foralls (combined, lt, rt) in `lemma_combined_cycle`.

**Fix:** Added a `closed spec fn spec_next_edge_from(tour, i)` that wraps
`tour[((i + 1) % n)].from`. Rewrote `spec_edges_form_cycle` to use it:

```rust
pub closed spec fn spec_next_edge_from(tour: Seq<Edge>, i: int) -> Point {
    tour[((i + 1) % (tour.len() as int))].from
}

pub open spec fn spec_edges_form_cycle(tour: Seq<Edge>) -> bool {
    tour.len() > 0 ==>
    forall|i: int| #![trigger tour[i]] 0 <= i < tour.len() ==>
        spec_point_eq(tour[i].to, spec_next_edge_from(tour, i))
}
```

Now when Z3 instantiates for index `i`, it gets `spec_next_edge_from(tour, i)` —
an opaque function call, NOT `tour[(i+1) % n]`. The chain is broken.

A helper lemma selectively reveals the closed fn for specific indices:

```rust
pub proof fn lemma_next_edge_from_eq(tour: Seq<Edge>, i: int)
    requires tour.len() > 0, 0 <= i < tour.len(),
    ensures spec_next_edge_from(tour, i) == tour[((i + 1) % (tour.len() as int))].from,
{ reveal(spec_next_edge_from); }
```

In `lemma_combined_cycle`, calls to `lemma_next_edge_from_eq(lt, li)` produce
one `lt[...]` term. The lt forall fires once, producing opaque
`spec_next_edge_from(lt, ...)` — no further chaining. The loop is limited to
exactly one step per explicit lemma call.

Base cases (n=2, n=3) use `reveal(spec_next_edge_from)` directly since small
tours cannot sustain a matching loop.

## Files Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 26 | ETSPStEph.rs | Added `spec_next_edge_from`, `lemma_next_edge_from_eq`; rewrote `spec_edges_form_cycle`; updated `lemma_combined_cycle` and base cases |
| 2 | — | lib.rs | Uncommented `pub mod ETSPStEph` |

## Verification

| Check | Result |
|-------|--------|
| `validate.sh isolate Chap26` | 1065 verified, 0 errors |
| `validate.sh` (full) | 5278 verified, 0 errors (pre-existing Chap52 compile error unrelated) |
| `holes.sh ETSPStEph.rs` | 0 holes, 4 clean proof functions |

## Steps Used

3 of 20 (1 edit cycle + 1 isolate validate + 1 full validate).
