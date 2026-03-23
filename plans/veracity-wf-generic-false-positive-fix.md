# Fix: fn_missing_wf false positives on generic wf free functions

You are working on `~/projects/veracity`. The binary is
`veracity-review-proof-holes`. Build with `cargo build --release`.
Test against `~/projects/APAS-VERUS/src/`.

## The Problem

`fn_missing_wf_requires` and `fn_missing_wf_ensures` emit false positives
when a function's requires/ensures uses a **free function** form of the wf
predicate instead of the **method** form.

Example from `src/Chap05/SetStEph.rs:216`:

```rust
fn elt_cross_set<U: StT + Hash + Clone>(a: &T, s2: &SetStEph<U>) -> (product: SetStEph<Pair<T, U>>)
    requires
      Self::spec_valid_key_type(),
      spec_setsteph_wf_generic(s2),       // ← wf IS here
      valid_key_type::<Pair<T, U>>(),
    ensures
       spec_setsteph_wf_generic(&product), // ← wf IS here
       ...
```

Veracity reports:
```
error: fn_missing_wf_requires - fn elt_cross_set — requires should include s2.spec_setsteph_wf() for input type SetStEph
error: fn_missing_wf_ensures - fn elt_cross_set — ensures should include product.spec_setsteph_wf() for return type SetStEph
```

The wf predicate IS present — just as `spec_setsteph_wf_generic(s2)` (free
function) instead of `s2.spec_setsteph_wf()` (method call). The free
function form is necessary because `s2: &SetStEph<U>` has a different type
parameter than `Self = SetStEph<T>`, so the method `spec_setsteph_wf()`
(which is defined on `SetStEph<T>` for the trait's `T`) can't be called
on `s2`.

## Why the free function exists

```rust
// In SetStEph.rs, inside the module:
pub open spec fn spec_setsteph_wf_generic<V: StT + Hash>(s: &SetStEph<V>) -> bool {
    s@.finite() && valid_key_type::<V>()
}
```

This is the same predicate as `spec_setsteph_wf(&self)` but parameterized
over any `V`, not just the trait's `T`. It exists precisely for the case
where a function takes `SetStEph<U>` with `U != T`.

The same pattern exists in `SetMtEph.rs` with `spec_setmteph_wf_generic`.

## The Fix

When checking whether a function's requires/ensures includes a wf predicate
for a parameter of type `Foo<X>`, veracity currently looks for:
- `param.spec_foo_wf()` (method form)

It should ALSO recognize:
- `spec_foo_wf_generic(param)` (free function form)
- `spec_foo_wf_generic(&param)` (free function with reference)

More precisely: if a parameter has type `Foo<X>` and the requires/ensures
contains a call to a function matching `spec_*_wf_generic` where the
argument is the parameter (or `&parameter`), that satisfies the wf check.

The naming convention is: `spec_<module>_wf_generic`. The `_generic` suffix
signals it's the free-function form of the module's wf predicate.

## Affected warnings (14 total — all false positives)

| # | Chap | File | Line | Function | False positive |
|---|------|------|------|----------|----------------|
| 1 | 5 | SetStEph.rs | 216 | `elt_cross_set` | has `spec_setsteph_wf_generic(s2)` + `spec_setsteph_wf_generic(&product)` |
| 2 | 5 | SetStEph.rs | 227 | `cartesian_product` | has `spec_setsteph_wf_generic(s2)` + `spec_setsteph_wf_generic(&product)` |
| 3 | 5 | SetStEph.rs | 238 | `all_nonempty` | has `spec_setsteph_wf_generic(parts)` |
| 4 | 5 | SetStEph.rs | 247 | `partition_on_elt` | has `spec_setsteph_wf_generic(parts)` |
| 5 | 5 | SetStEph.rs | 262 | `partition` | has `spec_setsteph_wf_generic(parts)` |
| 6 | 5 | SetMtEph.rs | 228 | `elt_cross_set` | has `spec_setmteph_wf_generic(s2)` + `spec_setmteph_wf_generic(&product)` |
| 7 | 5 | SetMtEph.rs | 239 | `cartesian_product` | has `spec_setmteph_wf_generic(s2)` + `spec_setmteph_wf_generic(&product)` |
| 8 | 5 | SetMtEph.rs | 251 | `all_nonempty` | has `spec_setmteph_wf_generic(parts)` |
| 9 | 5 | SetMtEph.rs | 260 | `partition_on_elt` | has `spec_setmteph_wf_generic(parts)` |
| 10 | 5 | SetMtEph.rs | 275 | `partition` | has `spec_setmteph_wf_generic(parts)` |

## Validation

After the fix, run against `~/projects/APAS-VERUS/src/` and verify:

- The 14 Chap05 Set warnings above no longer appear
- Warning count drops from 178 to 164 (or lower if other FPs existed)
- No new false negatives: functions that genuinely lack wf should still warn
- Total hole count unchanged (this is a warning-only fix)
