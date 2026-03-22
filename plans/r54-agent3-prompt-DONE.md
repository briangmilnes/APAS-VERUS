<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
# Agent 3 — Round 54: Chap26 ETSPMtEph + Chap38 clone bridge

## Goal

Close 3 holes across 3 files — all have clean dependencies.

## Priority 1: src/Chap26/ETSPMtEph.rs (1 hole)

**Hole**: `#[verifier::external_body]` on `point_distance` at line 612.

```rust
#[verifier::external_body]
fn point_distance(a: &Point, b: &Point) -> (d: f64)
    ensures d == spec_point_distance(*a, *b),
{
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}
```

**Problem**: f64 arithmetic (`-`, `*`, `.sqrt()`) is not verifiable in Verus. The function
computes Euclidean distance using floating-point operations.

**Fix approach**: This is a float arithmetic hole. Check `src/vstdplus/float.rs` for
available float axioms. The `FloatTotalOrder` trait provides ordering but NOT arithmetic.

Options:
1. If the `ensures` is the only spec obligation and the body is trivially correct, this
   may be an acceptable `external_body`. Document it as a float-boundary hole.
2. Check if Point coordinates can be integer (u64/i64) instead of f64. If so, rewrite
   with integer arithmetic which IS verifiable.
3. If the function is only called from other `external_body` functions, the hole may not
   matter for the proof chain.

Read the file to understand whether this hole actually blocks any other proof.

## Priority 2: src/Chap38/BSTParaStEph.rs (1 hole)

**Hole**: `assume(c == *x)` at line ~152 in `clone_elem`. Clone bridge pattern.

```rust
fn clone_elem<T: Clone>(x: &T) -> (c: T)
    ensures c == *x,
{
    let c = x.clone();
    proof { assume(c == *x); }
    c
}
```

**Problem**: Verus cannot prove that `T::clone()` returns a value equal to the original
for generic `T`. This is the standard clone bridge gap.

**Fix approach**: Read `src/standards/partial_eq_eq_clone_standard.rs` for the pattern.
The fix is to use the Clone workaround pattern:
- Add `ensures cloned(result, *x)` instead of `ensures c == *x`
- Or use `lemma_cloned_view_eq` to bridge from `cloned` to view equality
- Check if callers actually need `c == *x` or if `c@ == x@` (view equality) suffices

If view equality suffices for all callers, rewrite as:
```rust
fn clone_elem<T: StT + Clone>(x: &T) -> (c: T)
    ensures c@ == x@,
{
    let c = x.clone();
    proof { assume(c@ == x@); } // assume_eq_clone_workaround
    c
}
```

This downgrades from an algorithmic assume to a clone workaround (info-level, not error).

## Priority 3: src/Chap38/BSTParaMtEph.rs (1 hole)

Same `clone_elem` pattern as BSTParaStEph.rs. Apply the same fix.

## Rules

- Read `src/standards/partial_eq_eq_clone_standard.rs` before touching clone bridges.
- Do NOT add `accept()`. Prove or downgrade to clone workaround pattern.
- Do NOT modify files outside Chap26/Chap38.
- Validate after each file. Fix trigger warnings.
- Write `plans/agent3-round54-report.md` when done.
