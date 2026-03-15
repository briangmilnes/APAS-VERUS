# Chapter 30 — Review Against Prose

**Reviewer**: Claude-Opus-4.6 (Agent 2, Round 20)
**Date**: 2026-03-15

## Phase 1: Inventory

| # | Chap | File | Type | Functions | Holes | Status |
|---|------|------|------|-----------|-------|--------|
| 1 | 30 | Probability.rs | Infrastructure | 14 exec + 3 spec | 0 proof holes (11 accepted external_body) | Clean |

Single file chapter. No St/Mt variants. No Example/Exercise/Problem files.

**Consumers**: Chap50 OptBinSearchTree (StEph, StPer, MtEph, MtPer) imports
`Probability` and `ProbabilityTrait`.

## Phase 2: Prose Mapping

**No prose file exists** (`prompts/Chap30.txt` not found). Chapter 30 in the APAS textbook
covers probability theory concepts. The `Probability.rs` file is an infrastructure wrapper
around `f64` that provides a verified type shell for probability values. It does not implement
textbook algorithms; it provides a numeric type used by later chapters (specifically Chap50's
optimal binary search trees).

Without prose, no function-to-definition mapping or completeness assessment is possible.
The file's purpose is pragmatic: give Verus a type it can reason about for probability
values, with external_body on all f64 operations until Verus gains better float support.

## Phase 3: Cost Annotations

All 16 exec functions now have cost annotations in the source file. Since no APAS prose
exists for this chapter, all annotations use the "(no cost stated)" APAS line. All operations
are O(1) constant-time f64 operations or delegations.

| # | Chap | File | Function | APAS Cost | Claude-Opus-4.6 Cost |
|---|------|------|----------|-----------|---------------------|
| 1 | 30 | Probability.rs | new | (no cost stated) | Work O(1), Span O(1) |
| 2 | 30 | Probability.rs | value | (no cost stated) | Work O(1), Span O(1) |
| 3 | 30 | Probability.rs | infinity | (no cost stated) | Work O(1), Span O(1) |
| 4 | 30 | Probability.rs | zero | (no cost stated) | Work O(1), Span O(1) |
| 5 | 30 | Probability.rs | default | (no cost stated) | Work O(1), Span O(1) |
| 6 | 30 | Probability.rs | eq | (no cost stated) | Work O(1), Span O(1) |
| 7 | 30 | Probability.rs | partial_cmp | (no cost stated) | Work O(1), Span O(1) |
| 8 | 30 | Probability.rs | cmp | (no cost stated) | Work O(1), Span O(1) |
| 9 | 30 | Probability.rs | hash | (no cost stated) | Work O(1), Span O(1) |
| 10 | 30 | Probability.rs | from (f64) | (no cost stated) | Work O(1), Span O(1) |
| 11 | 30 | Probability.rs | from (Prob) | (no cost stated) | Work O(1), Span O(1) |
| 12 | 30 | Probability.rs | add | (no cost stated) | Work O(1), Span O(1) |
| 13 | 30 | Probability.rs | sub | (no cost stated) | Work O(1), Span O(1) |
| 14 | 30 | Probability.rs | mul | (no cost stated) | Work O(1), Span O(1) |
| 15 | 30 | Probability.rs | div | (no cost stated) | Work O(1), Span O(1) |
| 16 | 30 | Probability.rs | fmt (Debug) | (no cost stated) | Work O(1), Span O(1) |
| 17 | 30 | Probability.rs | fmt (Display) | (no cost stated) | Work O(1), Span O(1) |

## Phase 4: Parallelism Audit

N/A. No Mt modules in Chapter 30. `Probability` is a simple value type with no parallel
algorithms.

## Phase 5: RTT Review

**Gap**: No runtime tests exist for Chapter 30. Directory `tests/Chap30/` does not exist.

Basic RTT coverage should test:
- Construction via `new`, `zero`, `infinity`, `From<f64>`.
- Arithmetic: `add`, `sub`, `mul`, `div` produce expected f64 results.
- Ordering: `cmp` and `partial_cmp` agree, NaN handling is correct.
- Equality: `eq` uses bit-level comparison (NaN != NaN, +0 != -0).
- Round-trip: `From<f64>` then `From<Probability>` preserves value.
- `prob!` macro produces the expected `Probability` value.

Priority: low. This is infrastructure code with trivial implementations. The external_body
functions delegate directly to f64 operations, so the risk of logical error is minimal.

## Phase 6: PTT Review

No PTTs exist. No iterators in this module. No complex `requires` clauses. PTTs are not
warranted for this chapter.

## Phase 7: Gap Analysis

### Spec Gaps

The `ProbabilityTrait` functions (`new`, `value`, `infinity`, `zero`) have no
`requires`/`ensures` clauses. The style checker flags this (warning [12]). Since Probability
is an f64 wrapper with no invariants to maintain, the missing specs are low-priority but
could be strengthened:

- `new(p)` could ensure `result.value() == p` (if we had a spec for value).
- `value(&self)` could ensure the return matches the inner f64.
- `zero()` could ensure `result.value() == 0.0`.
- `infinity()` is external_body due to `f64::INFINITY` not being available in Verus ghost
  code.

These are all blocked on Verus's limited f64 support. The `AddSpecImpl` already acknowledges
this: `obeys_add_spec()` returns `false` and `add_spec` returns `arbitrary()`.

### Structural Observations

1. **Section ordering**: The style checker reports 7 warning [18] items for impls that
   appear after the derive-impls section. The `From`, `Add`, `Sub`, `Mul`, `Div`, and
   `AddSpecImpl` impls are in section 11 (derive impls) but should be in section 9 (impls).
   This is a style issue, not a correctness issue.

2. **No View impl**: `Probability` has no `View` type. This is intentional -- f64 has no
   meaningful ghost representation in Verus. The `AddSpecImpl` uses `arbitrary()` as a
   placeholder.

3. **external_body coverage**: All 11 external_body annotations are accepted holes. They
   wrap f64 operations that Verus cannot verify. This is the expected state until Verus
   gains float verification support.

### Dependency on Verus Float Support

The entire module is a thin wrapper waiting for Verus to support f64 operations in ghost
code. When that happens:
- `external_body` annotations can be removed from arithmetic ops.
- Real specs can be added to `ProbabilityTrait` functions.
- `AddSpecImpl` can provide a real `add_spec` instead of `arbitrary()`.

This is not a proof gap -- it is a tooling limitation.

## Phase 8: TOC Review

The file follows the standard TOC ordering with minor deviations:

| # | Section | Present | Notes |
|---|---------|---------|-------|
| 1 | module | Yes | `pub mod Probability` |
| 2 | imports | Yes | std, vstd, crate imports in order |
| 3 | broadcast use | No | Not needed |
| 4 | type definitions | Yes | `Probability` struct |
| 5 | view impls | No | No View (f64 limitation) |
| 6 | spec fns | No | No standalone spec fns |
| 7 | proof fns | No | No proof fns |
| 8 | traits | Yes | `ProbabilityTrait` |
| 9 | impls | Yes | `ProbabilityTrait for Prob` |
| 10 | iterators | No | Not applicable |
| 11 | derive impls in verus! | Yes | Default, PartialEq, Eq, Ord, Hash, From, Add/Sub/Mul/Div |
| 12 | macros | Yes | `prob!` macro (outside verus!) |
| 13 | derive impls outside verus! | Yes | Debug, Display |

Style warning: sections 12 and 13 appear in reversed order in the file (Debug/Display at
line 173, macro at line 187). The standard says macros come before derive-impls-outside.
Low priority.

## Summary

Chapter 30 is an infrastructure module, not an algorithm chapter. It wraps f64 in a
Verus-compatible type for use by Chap50 (optimal binary search trees). The module is clean
(0 proof holes), with 11 accepted external_body holes that reflect Verus's current lack of
f64 verification support. No prose file exists for comparison. Cost annotations have been
added to all exec functions. The main gap is the absence of runtime tests; the structural
and spec gaps are all blocked on Verus float support and are low priority.
