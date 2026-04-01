# R129 Agent 1 — Fix 10 holes in Chap26/DivConReduceMtPer.rs. AFK. DOT.

## Problem

You introduced 10 holes (5 external_body + 5 Ghost::assume_new) in
`src/Chap26/DivConReduceMtPer.rs` by wrapping inline closures in external_body
bridge functions. This is wrong. Read `src/standards/using_closures_standard.rs`
NOW before doing anything else.

## The fix

Replace each external_body bridge with a named closure that has explicit `ensures`.
The pattern for ALL FIVE functions is identical:

BEFORE (WRONG — 2 holes per function):
```rust
#[verifier::external_body]
fn call_reduce_max(a: &ArraySeqMtPerS<usize>) -> (reduced: usize)
    ensures reduced == spec_iterate(
        Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_max_fn(), 0usize)
{
    ArraySeqMtPerS::reduce(a,
        &|x: &usize, y: &usize| if *x >= *y { *x } else { *y },
        Ghost::assume_new(), 0)
}
```

AFTER (CORRECT — 0 holes):
```rust
fn call_reduce_max(a: &ArraySeqMtPerS<usize>) -> (reduced: usize)
    ensures reduced == spec_iterate(
        Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_max_fn(), 0usize)
{
    let f = |x: &usize, y: &usize| -> (r: usize)
        ensures r == spec_max_fn()(*x, *y)
    { if *x >= *y { *x } else { *y } };

    ArraySeqMtPerS::reduce(a, &f, Ghost(spec_max_fn()), 0)
}
```

Key changes:
1. Remove `#[verifier::external_body]`
2. Bind closure to a named variable with explicit `ensures r == spec_xxx_fn()(*x, *y)`
3. Replace `Ghost::assume_new()` with `Ghost(spec_xxx_fn())`

## The five functions to fix

| # | Line | Function | spec_fn | Identity |
|---|------|----------|---------|----------|
| 1 | 227 | call_reduce_max | spec_max_fn() | 0usize |
| 2 | 237 | call_reduce_sum | spec_sum_fn() | 0usize |
| 3 | 247 | call_reduce_product | spec_product_fn() | 1usize |
| 4 | 257 | call_reduce_or | spec_or_fn() | false |
| 5 | 267 | call_reduce_and | spec_and_fn() | true |

## Validation

Run `scripts/validate.sh isolate Chap26`. Confirm 0 errors.
Then run `scripts/holes.sh src/Chap26/DivConReduceMtPer.rs` and confirm 0 holes.

## Rules

- Do NOT use `external_body` on these functions.
- Do NOT use `Ghost::assume_new()`.
- Do NOT use `assume()` or `accept()`.
- Named closures with explicit `ensures` — this is standard 8.

## When done

Commit with `git add -A && git commit` and push.
