<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 2 — Round 58 Report

## Summary

Round 58 assigned: fix 14 `fn_missing_wf_requires` / `fn_missing_wf_ensures`
warnings in `src/Chap05/SetStEph.rs` and `src/Chap05/SetMtEph.rs`.

**Outcome: 0 of 14 warnings fixed.** This round is blocked by a veracity
tool limitation. The warnings are false positives — the wf predicates are
correctly present in free-function form, but veracity's checker only
recognizes the trait-method form. Using the trait-method form causes a
confirmed 3-node Verus cycle error.

---

## Warning Table (before R58)

| # | Chap | File | Line | Type | Function | Parameter |
|---|:----:|------|------|------|----------|-----------|
| 1 | 5 | SetMtEph.rs | 228 | fn_missing_wf_requires | `elt_cross_set` | `s2: &SetMtEph<U>` |
| 2 | 5 | SetMtEph.rs | 228 | fn_missing_wf_ensures | `elt_cross_set` | `product: SetMtEph<...>` |
| 3 | 5 | SetMtEph.rs | 239 | fn_missing_wf_requires | `cartesian_product` | `s2: &SetMtEph<U>` |
| 4 | 5 | SetMtEph.rs | 239 | fn_missing_wf_ensures | `cartesian_product` | `product: SetMtEph<...>` |
| 5 | 5 | SetMtEph.rs | 251 | fn_missing_wf_requires | `all_nonempty` | `parts: &SetMtEph<SetMtEph<T>>` |
| 6 | 5 | SetMtEph.rs | 260 | fn_missing_wf_requires | `partition_on_elt` | `parts: &SetMtEph<SetMtEph<T>>` |
| 7 | 5 | SetMtEph.rs | 275 | fn_missing_wf_requires | `partition` | `parts: &SetMtEph<SetMtEph<T>>` |
| 8 | 5 | SetStEph.rs | 216 | fn_missing_wf_requires | `elt_cross_set` | `s2: &SetStEph<U>` |
| 9 | 5 | SetStEph.rs | 216 | fn_missing_wf_ensures | `elt_cross_set` | `product: SetStEph<...>` |
| 10 | 5 | SetStEph.rs | 227 | fn_missing_wf_requires | `cartesian_product` | `s2: &SetStEph<U>` |
| 11 | 5 | SetStEph.rs | 227 | fn_missing_wf_ensures | `cartesian_product` | `product: SetStEph<...>` |
| 12 | 5 | SetStEph.rs | 238 | fn_missing_wf_requires | `all_nonempty` | `parts: &SetStEph<SetStEph<T>>` |
| 13 | 5 | SetStEph.rs | 247 | fn_missing_wf_requires | `partition_on_elt` | `parts: &SetStEph<SetStEph<T>>` |
| 14 | 5 | SetStEph.rs | 262 | fn_missing_wf_requires | `partition` | `parts: &SetStEph<SetStEph<T>>` |

**Warning table after R58: identical — 0 fixed.**

---

## Investigation

### What the code has (correct)

All 5 functions in both files already have wf predicates using the
**free-function form**:

```rust
// In SetStEph.rs trait declaration — elt_cross_set
fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
    requires
      Self::spec_valid_key_type(),
      spec_setsteph_wf_generic(s2),          // ← wf IS present
      valid_key_type::<Pair<T, U>>(),
    ensures
       spec_setsteph_wf_generic(&product),   // ← wf IS present
       ...
```

The free-function form is semantically equivalent to the trait-method form
(`s2.spec_setsteph_wf()`) — both expand to `s2@.finite() && valid_key_type::<U>()`.

### Why the free-function form is used

The `src/experiments/generic_specs_to_prevent_cycles.rs` experiment
documents a 3-node Verus cycle that occurs when a trait method uses another
concrete instantiation of that same trait in its `requires` or `ensures`:

- **Pattern 4a** (free function): PASSES — no trait dispatch, no cycle.
- **Pattern 4b** (trait method on concrete type): CYCLES — impl resolution.

Conclusion from the experiment: "only a free function (4a) avoids the cycle."

### Cycle confirmed by testing

Attempt: replaced all `spec_setsteph_wf_generic(s2)` calls with
`s2.spec_setsteph_wf()` in both files.

Result from `scripts/validate.sh`:

```
error: found a cyclic self-reference in a definition, which may result in nontermination
   --> src/Chap05/SetStEph.rs:216:9
    |
    | fn elt_cross_set<U: StT + Hash + Clone>(...)
    |    ^                               ^ may be part of cycle (node 1 of 3)
    | open spec fn spec_setsteph_wf(&self)  may be part of cycle (node 2 of 3)
    | pub trait SetStEphTrait<T: StT + Hash>  may be part of cycle (node 3 of 3)
```

Reverted all changes immediately.

### Why veracity flags the correct code

Veracity's `check_wf_flow` function (in `review_verus_proof_holes.rs`)
collects wf calls from requires/ensures using `collect_method_calls_expr`,
which only captures `Expr::MethodCall` nodes (i.e., `recv.method()` form).

A free-function call `spec_setsteph_wf_generic(s2)` is an `Expr::Call`
(not `Expr::MethodCall`), so it is never extracted into `req_calls`.
The check `req_calls.iter().any(|(recv, m)| recv == "s2" && m == "spec_setsteph_wf")`
then finds nothing and reports `fn_missing_wf_requires`.

---

## Root Cause

This is a **veracity tool limitation**: veracity's `fn_missing_wf` checker
only recognizes the trait-method form `param.spec_TYPE_wf()` and does not
recognize the semantically equivalent free-function form
`spec_TYPE_wf_generic(param)`.

---

## Resolution Required (veracity change)

Veracity's `check_wf_flow` needs to also accept the free-function form as
valid wf coverage. The pattern to match:

```
spec_TYPE_wf_generic(param)   →  counts as wf coverage for param
```

Where `spec_TYPE_wf_generic` is a free spec function whose name:
- starts with `spec_`
- ends with `_wf_generic`
- takes the same type as `param`

Alternatively, the tool could check if the requires contains a call to
any free function that ends with `_wf_generic` passing `param` as first arg,
and recognize that as equivalent to `param.spec_TYPE_wf()`.

**This requires a veracity source code change in `review_verus_proof_holes.rs`,
specifically in `check_wf_flow` and `collect_spec_wf_predicates`.**

---

## Verification Status

| Metric | Before R58 | After R58 |
|--------|:----------:|:---------:|
| fn_missing_wf_requires | 10 | 10 |
| fn_missing_wf_ensures | 4 | 4 |
| Verus verified count | 4484 | 4484 |
| Verus errors (Chap05) | 0 | 0 |
| Verus errors (pre-existing, Chap62) | 1 | 1 |

No source files were modified in this round. The pre-existing Chap62
error (`invariant not satisfied before loop` in `StarContractionStEph.rs`)
is outside Agent 2's scope.

---

## Techniques Tried

1. Replaced free-function wf with trait-method wf in both files.
2. Ran `scripts/validate.sh dev_only` — passed (Chap05 not in dev_only scope).
3. Ran `scripts/validate.sh` (full) — 3-node cycle error confirmed.
4. Reverted all changes.
5. Read veracity source to understand the limitation.
6. No further approach available without veracity change.
