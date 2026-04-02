# R140 Agent 2 — Fix 2 Chap19 style holes in ArraySeqMtEphSlice. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap19/ArraySeqMtEphSlice.rs` — the file with both holes.

Report file: `plans/r140-agent2-chap19-style-holes-report.md`

## Hole 1: bare_impl (line ~444)

Veracity reports: `impl<T: Eq + Clone> ArraySeqMtEphSliceS<T> { — impl without trait`

This bare impl block holds D&C helpers: `lemma_monoid_fold_left`, `reduce_dc`,
`map_dc_vec`, `filter_dc_vec`, `tabulate_dc_vec`. These are implementation helpers
called by the trait methods.

**Fix:** Move these functions into the trait. Add them to `ArraySeqMtEphSliceTrait`
with appropriate requires/ensures, then move the implementations into the
`impl ArraySeqMtEphSliceTrait for ArraySeqMtEphSliceS<T>` block.

Alternatively, if moving to the trait is too disruptive (these are internal helpers
not meant to be part of the public API), convert the bare impl to a second trait:

```rust
pub trait ArraySeqMtEphSliceDCHelpers<T: Eq + Clone>: Sized {
    fn reduce_dc<F: MtReduceFn<T>>(...) -> ...;
    fn map_dc_vec<U: ..., F: MtMapFn<T, U>>(...) -> Vec<U>;
    fn filter_dc_vec<F: MtPred<T>>(...) -> Vec<T>;
    fn tabulate_dc_vec<F: MtTabulateFn<T>>(...) -> Vec<T>;
}
```

**Prefer the simpler approach.** If you can just add these to the existing trait
without breaking callers, do that. If the type bounds are incompatible (the helpers
need `Send + Sync + 'static` which the trait doesn't require), use the second-trait
approach.

The `proof fn lemma_monoid_fold_left` can stay in a bare impl — proof functions
in bare impls are acceptable (veracity only flags exec functions in bare impls).
Actually check: if veracity flags it, move it too.

## Hole 2: fn_missing_ensures on flatten_dc_vec (line ~847)

Veracity reports: `fn flatten_dc_vec — exec fn should have ensures`

`flatten_dc_vec` is a free function (not in a trait or impl). It needs an `ensures`
clause. It currently has `requires` but no `ensures`.

**Fix:** Add ensures that match what `flatten` (the public function that calls it)
needs:
```rust
fn flatten_dc_vec<T: Eq + Clone + Send + Sync + 'static>(
    a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
) -> (result: Vec<T>)
    requires
        spec_nested_wf(a),
        obeys_feq_clone::<T>(),
    ensures
        // The result Vec can be wrapped into a wf slice sequence.
        // At minimum: result length is well-bounded.
        result@.len() <= usize::MAX,
```

Read the function body to determine what stronger ensures you can prove. At minimum
the length bound. If the function builds a Vec from inner sequences, you may be able
to prove the total length equals the sum of inner lengths.

Do NOT add ensures you can't prove — that creates verification errors worse than
the warning.

## Validation

Run `scripts/validate.sh isolate Chap19`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT break existing callers.
- Do NOT weaken existing ensures on other functions.

## When done

RCP.
