<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap27 — Review Against Prose

**Date**: 2026-02-28
**Last mechanical audit:** 2026-02-28 — full review regeneration after external_body removal and trait-impl refactor.
**Reviewer**: Claude-Opus-4.6
**Verification**: 2898 verified, 0 errors
**Proof holes (Chap27)**: 0

---

## Phase 1: Inventory

31 function instances across 4 files (15 unique names; proof lemmas duplicated per standalone rule).
All classified **strong** spec strength. 0 holes, 0 IBI (bare impl), 0 outside verus!.

| # | Function | Chap | File | Kind | Spec Strength |
|---|----------|:----:|------|------|:------------:|
| 1 | `reduce_contract` | 27 | ReduceContractStEph.rs | exec (trait+impl) | strong |
| 2 | `lemma_fold_left_monoid` | 27 | ReduceContractStEph.rs | proof (ML) | strong |
| 3 | `lemma_fold_left_pair` | 27 | ReduceContractStEph.rs | proof (ML) | strong |
| 4 | `lemma_fold_left_singleton` | 27 | ReduceContractStEph.rs | proof (ML) | strong |
| 5 | `lemma_contraction_even` | 27 | ReduceContractStEph.rs | proof (ML) | strong |
| 6 | `reduce_contract_parallel` | 27 | ReduceContractMtEph.rs | exec (trait+impl) | strong |
| 7 | `contract_parallel` | 27 | ReduceContractMtEph.rs | exec (ML, shared) | strong |
| 8 | `lemma_fold_left_monoid` | 27 | ReduceContractMtEph.rs | proof (ML) | strong |
| 9 | `lemma_fold_left_pair` | 27 | ReduceContractMtEph.rs | proof (ML) | strong |
| 10 | `lemma_fold_left_singleton` | 27 | ReduceContractMtEph.rs | proof (ML) | strong |
| 11 | `lemma_contraction_even` | 27 | ReduceContractMtEph.rs | proof (ML) | strong |
| 12 | `scan_contract` | 27 | ScanContractStEph.rs | exec (trait+impl) | strong |
| 13 | `expand_scan` | 27 | ScanContractStEph.rs | exec (trait+impl) | strong |
| 14 | `lemma_fold_left_monoid` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 15 | `lemma_fold_left_pair` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 16 | `lemma_fold_left_singleton` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 17 | `lemma_contraction_even` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 18 | `lemma_prefix_contraction` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 19 | `lemma_expand_even` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 20 | `lemma_expand_odd` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 21 | `lemma_expand_odd_tail` | 27 | ScanContractStEph.rs | proof (ML) | strong |
| 22 | `scan_contract_parallel` | 27 | ScanContractMtEph.rs | exec (trait+impl) | strong |
| 23 | `expand_scan_parallel` | 27 | ScanContractMtEph.rs | exec (trait+impl) | strong |
| 24 | `lemma_fold_left_monoid` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 25 | `lemma_fold_left_pair` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 26 | `lemma_fold_left_singleton` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 27 | `lemma_contraction_even` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 28 | `lemma_prefix_contraction` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 29 | `lemma_expand_even` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 30 | `lemma_expand_odd` | 27 | ScanContractMtEph.rs | proof (ML) | strong |
| 31 | `lemma_expand_odd_tail` | 27 | ScanContractMtEph.rs | proof (ML) | strong |

Proof lemmas are duplicated across St/Mt files per the chapter standalone rule.

---

## Phase 2: Prose Inventory

Source: `prompts/Chap27.txt`

### Definitions
1. **Definition 27.1 (Contraction)** — Base case + inductive step (contract, solve, expand)

### Algorithms
1. **Example 27.1 (Maximal Element)** — Contraction-based max, W(n)=Θ(n), S(n)=Θ(lg n)
2. **Algorithm 27.2 (Reduce with Contraction)** — `reduceContract f id a`; W(n)=Θ(n), S(n)=Θ(log n)
3. **Algorithm 27.3 (Scan Using Contraction)** — `scan f id a`; W(n)=Θ(n), S(n)=Θ(log n)

### Cost specs
1. Reduce: W(n) = W(n/2) + n = O(n); S(n) = S(n/2) + 1 = O(log n)
2. Scan: W(n) = W(n/2) + n = O(n); S(n) = S(n/2) + 1 = O(log n)

### Theorems/Properties
- Correctness via induction (stated informally in prose, formally proved in Verus via `lemma_contraction_even` and `lemma_prefix_contraction`)
- Contraction preserves fold_left under monoid

### Exercises
- None in the prose excerpt

---

## Phase 3: Algorithmic Analysis

### 3a. Cost annotations

All exec functions have paired APAS/Claude-Opus-4.6 cost annotations. All proof functions annotated as N/A.

| # | Function | Chap | File | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|----------|:----:|------|-----------|---------------------|:---------:|
| 1 | `reduce_contract` | 27 | ReduceContractStEph.rs | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | ✗ |
| 2 | `reduce_contract_parallel` | 27 | ReduceContractMtEph.rs | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | ✗ |
| 3 | `contract_parallel` | 27 | ReduceContractMtEph.rs | N/A (helper) | W Θ(n), S Θ(n/2) | N/A |
| 4 | `scan_contract` | 27 | ScanContractStEph.rs | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | ✗ |
| 5 | `expand_scan` | 27 | ScanContractStEph.rs | N/A (sub-step) | W Θ(n), S Θ(n) | N/A |
| 6 | `scan_contract_parallel` | 27 | ScanContractMtEph.rs | W Θ(n), S Θ(log n) | W Θ(n), S Θ(n) | ✗ |
| 7 | `expand_scan_parallel` | 27 | ScanContractMtEph.rs | N/A (sub-step) | W Θ(n), S Θ(n) | N/A |

**Disagreements**: The StEph variants are sequential implementations with `while` loops. The APAS Span Θ(log n) assumes parallel tabulate; the actual sequential Span is Θ(n). The MtEph variants use one-level `join` for contraction but sequential expansion, giving Span Θ(n) rather than Θ(log n). The contraction step is parallel via `contract_parallel` (two halves via `join`), but the recursive structure and expansion loop remain sequential, so the overall span doesn't achieve the textbook's Θ(log n). This is documented in the cost annotations in each file.

### 3b. Implementation fidelity

| # | Function | Chap | File | Matches? | Notes |
|---|----------|:----:|------|:--------:|-------|
| 1 | `reduce_contract` | 27 | ReduceContractStEph.rs | ✓ | Algorithm 27.2: base cases, contract, recurse, expand for odd |
| 2 | `reduce_contract_parallel` | 27 | ReduceContractMtEph.rs | ✓ | Same with parallel contraction via `contract_parallel` |
| 3 | `contract_parallel` | 27 | ReduceContractMtEph.rs | N/A | Factored helper, not in prose |
| 4 | `scan_contract` | 27 | ScanContractStEph.rs | ✓ | Algorithm 27.3: contract, recurse, expand even/odd |
| 5 | `expand_scan` | 27 | ScanContractStEph.rs | ✓ | Expand sub-step of Algorithm 27.3 |
| 6 | `scan_contract_parallel` | 27 | ScanContractMtEph.rs | ✓ | Same with parallel contraction |
| 7 | `expand_scan_parallel` | 27 | ScanContractMtEph.rs | ✓ | Same expand logic using `call_f` for Arc |

No deviations from the prose algorithms. The Mt variants add `Arc<F>` wrapping, `call_f`, and `contract_parallel` for fork-join, but the algorithmic structure is identical to the prose.

### 3c. Spec fidelity

All exec functions have full `ensures` matching the prose:
- **Reduce**: `result == Seq::new(...).fold_left(id, spec_f)` — exactly captures "reduce a to a single value"
- **Scan**: `result.spec_index(i) == s.take(i).fold_left(id, spec_f)` — exactly captures "exclusive prefix scan"
- **Preconditions**: `spec_monoid(spec_f, id)` captures the prose requirement that f is associative with identity id
- **contract_parallel**: Ensures `b[j] == spec_f(a[2j], a[2j+1])` — exact contraction definition
- **expand_scan / expand_scan_parallel**: Full ensures on output: `result@[k] == s.take(k).fold_left(*id, spec_f)` for all k

No prose properties are missing from the specs.

---

## Phase 4: Parallelism Review

### 4a. Classification

| # | Function | Chap | File | Parallel? | Mechanism |
|---|----------|:----:|------|:---------:|-----------|
| 1 | `contract_parallel` | 27 | ReduceContractMtEph.rs | **Parallel** | `HFSchedulerMtEph::join` — splits contraction into left/right halves |
| 2 | `reduce_contract_parallel` | 27 | ReduceContractMtEph.rs | **Parallel** | Recursive; calls `contract_parallel` at each level |
| 3 | `scan_contract_parallel` | 27 | ScanContractMtEph.rs | **Parallel** | Recursive; calls `contract_parallel` for contraction |
| 4 | `expand_scan_parallel` | 27 | ScanContractMtEph.rs | **Sequential** | Expansion loop is sequential `while` with `call_f` |

### 4b. Span audit

| # | Function | APAS Span | Actual Span | Correct? |
|---|----------|-----------|-------------|:--------:|
| 1 | `reduce_contract_parallel` | Θ(log n) | Θ(n) | ✗ |
| 2 | `scan_contract_parallel` | Θ(log n) | Θ(n) | ✗ |

The contraction step is parallel (one-level join splitting the loop into two halves), but the recursive structure is sequential (single recursive call, not two independent subproblems). The expansion loop in scan is also sequential. The overall span is Θ(n), not the textbook's Θ(log n). This is documented in the cost annotations.

### 4c. Parallelism gap table

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|:---------:|-------|
| 1 | `contract_parallel` | — | Θ(n/2) | ✓ | Two halves via join |
| 2 | `reduce_contract_parallel` | Θ(log n) | Θ(n) | Partial | Contraction parallel, recursion sequential |
| 3 | `scan_contract_parallel` | Θ(log n) | Θ(n) | Partial | Contraction parallel, recursion + expansion sequential |
| 4 | `expand_scan_parallel` | Θ(1) | Θ(n) | ✗ | Sequential while loop; could parallelize with tabulate |

**Gap**: Mt functions are partially parallel. The contraction step uses `join` but the recursive call structure and expansion are sequential. Achieving Θ(log n) span would require parallel tabulate for both contraction and expansion at each level.

---

## Phase 5: Runtime Test Review

### 5a. Coverage

| # | Chap | Source File | RTT File | Tests | Coverage |
|---|:----:|------------|----------|:-----:|:--------:|
| 1 | 27 | ReduceContractStEph.rs | TestReduceContractStEph.rs | 10 | ✓ |
| 2 | 27 | ReduceContractMtEph.rs | TestReduceContractMtEph.rs | 10 | ✓ |
| 3 | 27 | ScanContractStEph.rs | TestScanContractStEph.rs | 9 | ✓ |
| 4 | 27 | ScanContractMtEph.rs | TestScanContractMtEph.rs | 9 | ✓ |

All 4 modules have corresponding test files with 38 total tests.

### 5b. Test quality

Each test file covers:
- ✓ Empty sequence (edge case)
- ✓ Single element (base case)
- ✓ Two-element sequences (minimal contraction step)
- ✓ Even-length sequences (no odd expansion)
- ✓ Odd-length sequences (exercises odd expansion path)
- ✓ Power-of-2 inputs (16, 32) matching prose assumption
- ✓ Large inputs (1000 for reduce, 100 for scan)
- ✓ Multiple operations (sum, product, max for reduce; sum, product for scan)
- ✓ Scan verifies prefix values match expected

### 5c. Missing tests

No gaps.

---

## Phase 6: PTT Review

Chap27 has **no iterators and no verified loops that need PTT coverage**. The `while` loops in the implementations are standard contraction/expansion loops covered by Verus verification directly.

**No PTTs needed.** The chapter has no iterators, no `IntoIterator`, no `GhostIterator`, no `ForLoopGhostIterator`.

---

## Phase 7: Gap Analysis

### Prose items with no implementation

| # | Prose Item | Status |
|---|-----------|--------|
| 1 | Example 27.1 (Maximal Element) | Subsumed — `reduce_contract` with max and 0 identity achieves the same; documented in trait doc comments on both StEph and MtEph |

### Code with no prose counterpart

| # | Function | Chap | File | Purpose |
|---|----------|:----:|------|---------|
| 1 | `contract_parallel` | 27 | ReduceContractMtEph.rs | Factored-out parallel contraction step for reuse by scan |
| 2 | `expand_scan` | 27 | ScanContractStEph.rs | Factored-out expand phase for Z3 tractability |
| 3 | `expand_scan_parallel` | 27 | ScanContractMtEph.rs | Same for Mt variant |
| 4 | `lemma_fold_left_monoid` | 27 | (all 4 files) | Proof scaffolding |
| 5 | `lemma_fold_left_pair` | 27 | (all 4 files) | Proof scaffolding |
| 6 | `lemma_fold_left_singleton` | 27 | (all 4 files) | Proof scaffolding |
| 7 | `lemma_contraction_even` | 27 | (all 4 files) | Core contraction correctness lemma |
| 8 | `lemma_prefix_contraction` | 27 | (both Scan files) | Core prefix contraction lemma |
| 9 | `lemma_expand_even` | 27 | (both Scan files) | Expand step: even index correctness |
| 10 | `lemma_expand_odd` | 27 | (both Scan files) | Expand step: odd index correctness |
| 11 | `lemma_expand_odd_tail` | 27 | (both Scan files) | Expand step: odd-length tail correctness |

All code-only items are proof scaffolding, factored helpers for Z3 tractability, or the Arc/fork-join pattern. Expected.

---

## Phase 8: TOC Review

### TOC presence and section ordering

| # | Chap | File | TOC? | Sections Present | Order Correct? | Notes |
|---|:----:|------|:----:|-----------------|:--------------:|-------|
| 1 | 27 | ReduceContractStEph.rs | ✓ | 1,2,3,7,8,9 | ✓ | |
| 2 | 27 | ReduceContractMtEph.rs | ✓ | 1,2,3,7,8,9 | ✗ | Missing `// 8. traits` section header before trait declaration |
| 3 | 27 | ScanContractStEph.rs | ✓ | 1,2,3,7,8,9 | ✓ | |
| 4 | 27 | ScanContractMtEph.rs | ✓ | 1,2,3,7,8,9 | ✓ | |

Note: ReduceContractMtEph.rs TOC lists section 8 but the `//	8. traits` comment is missing before the `pub trait ReduceContractMtEphTrait` declaration at line 147. The trait appears between sections 7 and 9 in the correct position but without its own section header.

### In/out table

| # | Chap | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|:----:|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | 27 | ReduceContractStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | 27 | ReduceContractMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | 27 | ScanContractStEph.rs | - | - | - | - | - | - | - | - | - |
| 4 | 27 | ScanContractMtEph.rs | - | - | - | - | - | - | - | - | - |

No derive impls in any Chap27 file — these are algorithm modules, not data type modules.

---

## Proof Holes Summary

```
✓ ReduceContractMtEph.rs — 4 clean proof functions
✓ ReduceContractStEph.rs — 4 clean proof functions
✓ ScanContractMtEph.rs — 8 clean proof functions
✓ ScanContractStEph.rs — 8 clean proof functions

Proof Functions: 24 clean, 0 holed
Holes: 0 total
Info: 0 total
```

No trust boundaries in chapter code. The only external infrastructure dependency is `assume_specification` for `Arc::clone` in `vstdplus/smart_ptrs.rs` (shared, not chapter-specific).

---

## Spec Strength Summary

| Classification | Count |
|---------------|:-----:|
| strong | 31 |
| partial | 0 |
| weak | 0 |
| none | 0 |

---

## Overall Assessment

Chap27 is **complete**:
- All prose algorithms (27.2, 27.3) implemented in both St and Mt variants
- Example 27.1 (Maximal Element) subsumed by generic reduce; documented in trait doc comments
- All 31 functions have strong specs — full ensures capturing fold_left / prefix fold_left semantics
- Zero proof holes — both scan implementations fully proven (previously had external_body)
- All exec functions follow the trait-impl pattern (including expand_scan/expand_scan_parallel)
- `contract_parallel` is a justified ML free function: shared helper imported by ScanContractMtEph
- Both Mt variants use `HFSchedulerMtEph::join` for contraction parallelism
- 38 runtime tests cover all edge cases
- No PTTs needed (no iterators)
- Minor: ReduceContractMtEph.rs missing `// 8. traits` section header
