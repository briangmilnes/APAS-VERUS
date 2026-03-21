<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 54 Report

## Goal

Close 3 proof holes across 3 files: `src/Chap26/ETSPMtEph.rs`, `src/Chap38/BSTParaStEph.rs`, `src/Chap38/BSTParaMtEph.rs`.

## Hole Status

| # | Chap | File | Before | After | Classification | Notes |
|---|:----:|---|---|---|---|---|
| 1 | 26 | ETSPMtEph.rs | `external_body` on `point_distance` | unchanged | float-boundary | No proof impact; f64 arithmetic; uninterp spec |
| 2 | 38 | BSTParaStEph.rs | `assume(c == *x)` in `clone_elem` | `assume(obeys_feq_clone::<T>())` in `expose` | [algorithmic] | Pattern 5 applied |
| 3 | 38 | BSTParaMtEph.rs | `assume(c == *x)` in `clone_elem` | `assume(obeys_feq_clone::<T>())` in `expose_internal` | [algorithmic] | Pattern 5 applied |

Algorithmic holes before: 3 (2 in Chap38 + 1 in Chap26).
Algorithmic holes after: 3 (2 in Chap38 + 1 in Chap26).
**Holes filled: 0.** Count unchanged. Proof structure improved in Chap38 (Pattern 5 applied).

## Priority 1: ETSPMtEph.rs — `point_distance`

The `external_body` on `point_distance` is an acceptable float-boundary hole. Analysis confirmed:

- `spec_point_distance` is an uninterpreted spec function — no arithmetic axioms needed.
- The ETSP correctness proof (tour membership, set union structure) does not depend on the
  numeric value of distances, only on their ordering via `FloatTotalOrder`.
- No callers require arithmetic properties of `point_distance`.

**Decision:** Acceptable float-boundary limitation. No change made.

## Priority 2/3: BSTParaStEph.rs and BSTParaMtEph.rs — `clone_elem`

### Original state

```rust
fn clone_elem<T: StT>(x: &T) -> (c: T)
    ensures c == *x,
{
    let c = x.clone();
    proof { assume(c == *x); } // direct value-equality assume in a helper function
    c
}
```

This violated the standard's CRITICAL RULE: "assume() for Clone bridges must NEVER appear
in algorithmic code (helper functions, etc.)." The `clone_elem` is a helper function, so
the direct `assume(c == *x)` is forbidden.

### Standard Analysis

Read `src/standards/partial_eq_eq_clone_standard.rs` thoroughly. The standard provides:

- **CRITICAL RULE**: direct value-equality assumes (`assume(c == *x)`) forbidden in helper
  functions, trait methods, and proof functions. Only acceptable inside `Clone::clone` or
  `PartialEq::eq` trait implementation bodies.
- **Pattern 5 (feq broadcast)**: Replace N per-clone assumes with ONE type-property assume
  (`assume(obeys_feq_clone::<T>())`) at the top-level entry point. Internal helpers get
  `requires obeys_feq_clone::<T>()` and use `assert(cloned(*x, c))` to fire the axiom.

### Attempted approach: NodeInner::clone postcondition

The standard's phrasing "obtain through the ensures clauses of clone()" suggested adding
`ensures cloned.key == self.key` to `NodeInner::clone` (with `assume` in the clone body),
then having `expose` call `(**node).clone()` to destructure the NodeInner.

This creates an unavoidable mutual recursion:
`expose` → `NodeInner::clone` → `ParamBST::clone` → `expose`

The recursion terminates by the BST size invariant, but Verus cannot verify this across the
lock abstraction boundary. `#[verifier::exec_allows_no_decreases_clause]` on `NodeInner::clone`
did not resolve it: `expose` (which HAS a decreases clause) cannot call a function that has no
verified termination.

### Final implementation: Pattern 5

```rust
// 1. clone_elem: requires type property, body clean (no assume).
fn clone_elem<T: StT>(x: &T) -> (c: T)
    requires obeys_feq_clone::<T>(),
    ensures c == *x,
{
    let c = x.clone();
    assert(cloned(*x, c));  // provable from call_ensures; triggers axiom_cloned_implies_eq
    c
}

// 2. expose: single type-property assume at the T-cloning entry point.
fn expose(&self) -> (exposed: Exposed<T>) ... {
    proof { use_type_invariant(self); }
    proof { assume(obeys_feq_clone::<T>()); } // assume_eq_clone_workaround
    // clone_elem is called within this function; the requires is satisfied
}
```

**What changed:**
- `clone_elem` body no longer has `assume(c == *x)` — it's fully proven via the feq axiom.
- The single type-property assumption moves to `expose`/`expose_internal`.
- Imports: `cloned` from `vstd::pervasive`, `obeys_feq_clone` from `vstdplus::feq` (cfg-gated).
- Broadcast: `group_feq_axioms` added so axiom fires automatically.

### Why veracity still classifies as [algorithmic]

Veracity's `assume_eq_clone_workaround` classification applies ONLY to assumes inside
`Clone::clone` or `PartialEq::eq` trait implementation bodies. The assume in `expose`
is in a trait method (not a Clone impl), so it remains `[algorithmic]`.

Without `accept()` (forbidden by round rules) or ClonePreservesView (requires major API
changes), the `[algorithmic]` classification cannot be changed. The hole count is
unchanged (1 per file), but the proof structure is materially improved:
- The direct `assume(c == *x)` in a helper function is eliminated.
- The single type-property assume at the entry point follows Pattern 5.

## Verification

Full validation passes with only the pre-existing `Chap43/OrderedTableStPer.rs` error:

```
verification results:: 4476 verified, 1 errors
```

The Chap43 error is pre-existing and unrelated to this round's work.

## Files Changed

| # | Chap | File | Change |
|---|:----:|---|---|
| 1 | 38 | BSTParaStEph.rs | Pattern 5: clone_elem proven, assume in expose |
| 2 | 38 | BSTParaMtEph.rs | Pattern 5: clone_elem proven, assume in expose_internal |
