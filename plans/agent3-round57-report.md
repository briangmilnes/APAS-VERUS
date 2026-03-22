<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 57 Report

## Summary

One actual fix landed (Task 3: `TableStPer.rs`). Tasks 1 and 2 (Chap05 Set files) hit
a Verus cycle that prevents the fix — the existing code is already correct per the project
standard and the veracity warnings are false positives.

## Verification

| State | Verified | Errors |
|---|:---:|:---:|
| Baseline (before) | 4485 | 0 |
| After fix | 4485 | 0 |

The `collect_by_key` ensures clause was already provable (the loop invariant
`result.spec_tablestper_wf()` was there); adding the ensures to the signature
added no new proof work.

## Holes Before / After

### Chap42 — Task 3 fixed

| # | Chap | File | Category | Before | After |
|---|:---:|---|---|:---:|:---:|
| 1 | 42 | TableStPer.rs | fn_missing_wf_ensures | 1 | 0 |
| 2 | 42 | TableMtEph.rs | assume_eq_clone_workaround | 2 | 2 |
| 3 | 42 | TableStEph.rs | assume_eq_clone_workaround | 1 | 1 |
| 4 | 42 | TableStPer.rs | assume_eq_clone_workaround | 2 | 2 |

**Net change Chap42: −1 fn_missing_wf_ensures.**

### Chap05 — Tasks 1 & 2: not fixed (false positives)

| # | Chap | File | Category | Before | After |
|---|:---:|---|---|:---:|:---:|
| 1 | 5 | SetStEph.rs | fn_missing_wf_requires | 5 | 5 |
| 2 | 5 | SetStEph.rs | fn_missing_wf_ensures | 2 | 2 |
| 3 | 5 | SetMtEph.rs | fn_missing_wf_requires | 5 | 5 |
| 4 | 5 | SetMtEph.rs | fn_missing_wf_ensures | 2 | 2 |
| 5 | 5 | SetStEph.rs | assume_eq_clone_workaround | 1 | 1 |
| 6 | 5 | SetMtEph.rs | assume_eq_clone_workaround | 2 | 2 |

**Net change Chap05: 0 (unchanged).**

## Task 3: What Was Fixed

**`src/Chap42/TableStPer.rs` — `collect_by_key` ensures**

Added `grouped.spec_tablestper_wf()` to the ensures clause of the free function
`collect_by_key`. The loop body already maintained `result.spec_tablestper_wf()` as a
loop invariant, so this required no proof changes — only the signature was incomplete.

```
// Before
ensures
    forall|k: K::V| grouped@.contains_key(k) ...,
    forall|k: K::V| ... grouped@[k] == ...,
    grouped@.dom().finite(),

// After
ensures
    grouped.spec_tablestper_wf(),      // added
    forall|k: K::V| grouped@.contains_key(k) ...,
    ...
```

## Tasks 1 & 2: Why Not Fixed

**Root cause**: Verus 3-node cycle when calling a trait method on a concrete non-Self type
inside a trait definition.

The five functions (`elt_cross_set`, `cartesian_product`, `all_nonempty`,
`partition_on_elt`, `partition`) take parameters of type `SetStEph<U>` or
`SetStEph<SetStEph<T>>` — non-Self concrete types. Calling `s2.spec_setsteph_wf()` in
the trait definition creates a cycle:

```
SetStEphTrait<T> definition
  → elt_cross_set requires s2.spec_setsteph_wf()
  → Verus resolves SetStEphTrait<U>::spec_setsteph_wf impl
  → impl body in same file as SetStEphTrait<T> definition  ← cycle
```

This is the exact scenario documented in `src/standards/spec_wf_standard.rs`
section 4 ("Non-Self concrete types need a free function (Verus cycle workaround)"):

> Calling a trait method on a concrete non-Self type inside a trait definition triggers
> a 3-node cycle in Verus. The workaround is a generic free spec fn at module level:
> `spec_setsteph_wf_generic(s)`.

The current code **correctly** uses `spec_setsteph_wf_generic(s2)` which is the
standard-mandated pattern. Veracity's `fn_missing_wf_requires` checker only recognizes
the method-call form `s2.spec_setsteph_wf()` and misses the semantically equivalent
free-function form.

**Attempted fix result**: Replacing `spec_setsteph_wf_generic(s2)` with
`s2.spec_setsteph_wf()` in the trait produced:
```
error: may be part of cycle (node 1 of 3 in cycle): function definition,
       whose body may have dependencies
```

**What is needed (user decision)**: Either:
1. A veracity fix to recognize `spec_<type>_wf_generic(param)` as satisfying the
   `fn_missing_wf` check (preferred — no code change needed).
2. A `// veracity: no_wf_required` or equivalent annotation added to these 10 trait
   function signatures (requires user approval per project rules).

## Techniques Used

- Verified that loop invariant `result.spec_tablestper_wf()` already proves the new
  ensures without any additional proof hints.
- Traced Verus cycle through trait resolution to confirm the false positive diagnosis.
- Read `src/standards/spec_wf_standard.rs` section 4 to confirm the generic-function
  pattern is the project-mandated solution.
