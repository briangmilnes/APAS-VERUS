<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# The Biconditional Bridge Problem

## Overview

When a higher-order function like `filter` takes an exec closure and needs to
reason about its behavior in spec-level postconditions, we face the
**biconditional bridge** problem: connecting `pred.ensures` (an opaque
spec-level relation describing the closure's behavior) to a `spec_fn`
predicate that the postconditions can reference.

## The Pattern

The `filter` trait requires a `Ghost(spec_pred)` parameter and a biconditional
bridge in its `requires` clause:

```rust
fn filter<F: Fn(&T) -> bool>(
    a: &Self, pred: &F,
    Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
) -> (filtered: Self)
    requires
        forall|v: T, keep: bool|
            pred.ensures((&v,), keep) <==> spec_pred(v) == keep,
    ensures
        filtered.spec_len() == spec_filter_len(..., spec_pred),
        filtered.to_multiset() =~= a.to_multiset().filter(spec_pred),
```

The `ensures` clauses use `spec_pred` — not `pred.ensures` — so the bridge is
the only connection between the exec closure and the spec-level reasoning.

## The Two Directions

The biconditional `<==>` has two directions:

| # | Direction | Statement | Meaning |
|---|-----------|-----------|---------|
| 1 | Forward (`==>`) | `pred.ensures((&v,), keep) ==> spec_pred(v) == keep` | "If the closure returned `keep` for `v`, then `spec_pred` agrees." |
| 2 | Backward (`<==`) | `spec_pred(v) == keep ==> pred.ensures((&v,), keep)` | "If `spec_pred` says `keep`, then the closure's ensures relation holds for that `(v, keep)` pair." |

### Forward direction — provable

When you define a closure:

```rust
let pred = |x: &N| -> (keep: bool)
    ensures keep == spec_is_prime(*x as int),
{ is_prime(*x) };
```

Verus verifies the closure body against its `ensures` clause. At the call site,
`pred.ensures((&v,), keep)` is definitionally equivalent to the closure's
postcondition: `keep == spec_is_prime(v as int)`. The forward direction
(`pred.ensures ==> spec_pred match`) follows directly.

### Backward direction — unprovable?

The backward direction says: "for any hypothetical `(v, keep)` satisfying
`spec_pred`, the ensures relation holds." In principle this is trivially true
(the ensures clause is symmetric in `==`), but Verus may not be able to
establish this because:

1. `pred.ensures` is an opaque specification-level relation at the generic
   function boundary (`F: Fn(&T) -> bool`).
2. While Verus knows the concrete closure at the call site, the connection
   between `pred.ensures` and the concrete `ensures` clause may be encoded
   one-directionally in the SMT encoding.

## Why Only the Forward Direction Is Needed

Inside the `filter` implementation:

1. **Loop-based (Chap18):** Each iteration calls `pred(&a[i])` and receives a
   concrete `keep` value. The forward direction connects this actual return
   value to `spec_pred`. The loop invariant already uses `==>`:
   ```rust
   forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
   ```

2. **Deflate+flatten (Chap19):** The deflate function calls `pred` on each
   element, producing inner sequences of length 0 or 1. The forward direction
   tells us: if `pred` returned true (length 1), then `spec_pred` is true; if
   `pred` returned false (length 0), then `spec_pred` is false. This
   establishes the biconditional `ss[i].len() == 1 <==> spec_pred(a[i])`
   without needing the backward direction.

Neither implementation ever reasons from `spec_pred` backward to `pred.ensures`.
They always start from the actual closure call result and map forward to
`spec_pred`.

## The Weakening Hack

**Weaken the `requires` from `<==>` to `==>`.**

```rust
// BEFORE (unprovable at call site):
requires
    forall|v: T, keep: bool|
        pred.ensures((&v,), keep) <==> spec_pred(v) == keep,

// AFTER (provable from closure's ensures clause):
requires
    forall|v: T, keep: bool|
        pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
```

This eliminates the need for `assume` at every call site while preserving all
proof obligations inside the implementation.

## Affected Files

### Filter trait definitions (requires clause: `<==>` → `==>`)

| # | File | Chapter |
|---|------|---------|
| 1 | `src/Chap18/ArraySeq.rs` | Gold standard |
| 2 | `src/Chap18/ArraySeqStPer.rs` | |
| 3 | `src/Chap18/ArraySeqStEph.rs` | |
| 4 | `src/Chap18/ArraySeqMtPer.rs` | |
| 5 | `src/Chap18/ArraySeqMtEph.rs` | |
| 6 | `src/Chap18/LinkedListStPer.rs` | |
| 7 | `src/Chap18/LinkedListStEph.rs` | |
| 8 | `src/Chap19/ArraySeqStPer.rs` | |
| 9 | `src/Chap19/ArraySeqStEph.rs` | |
| 10 | `src/Chap19/ArraySeqMtEph.rs` | |

### Call sites (remove `assume`)

| # | File | Lines | Bridge assume |
|---|------|-------|---------------|
| 1 | `src/Chap21/Algorithm21_5.rs` | 56–58 | `pred <==> spec_pred` |
| 2 | `src/Chap21/Algorithm21_6.rs` | 119–121 | `pred <==> spec_not_composite` |
| 3 | `src/Chap21/Exercise21_7.rs` | 69–72 | `pred_even <==> spec_even`, `pred_vowel <==> spec_vowel` |
| 4 | `src/Chap21/Exercise21_8.rs` | 126–128 | `pred <==> spec_pred` |

### Not affected

| # | File | Reason |
|---|------|--------|
| 1 | `src/Chap21/Exercise21_8.rs:156` | Different assume — `ones.seq@.len() == spec_divisor_count(...)` — unrelated to biconditional bridge |

## Remaining Proof Hole After Fix

After eliminating the 5 biconditional bridge assumes, Chap21 will have exactly
**1 proof hole**: the `spec_divisor_count` length assumption in Exercise21_8.rs
(line 156). This is a separate problem requiring a recursive spec that connects
`spec_filter_len` to `spec_divisor_count`.
