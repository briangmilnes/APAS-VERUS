# Chapter 27 — Review Against Prose

**Reviewer**: Claude-Opus-4.6 (Agent 2)
**Date**: 2026-03-15

## Phase 2: Prose Inventory

Source: `prompts/Chap27.txt` (Chapter 27 — Contraction)

### Definitions

| # | Item | Type | Description |
|---|------|------|-------------|
| 1 | Definition 27.1 | Technique | Contraction algorithm structure: base case, contract, solve, expand |

### Algorithms

| # | Item | Description |
|---|------|-------------|
| 1 | Example 27.1 | Maximal element via contraction |
| 2 | Algorithm 27.2 | Reduce with contraction (reduceContract) |
| 3 | Algorithm 27.3 | Scan with contraction (scan using contraction, powers of 2) |
| 4 | (inline) | Sequential scan via iterate (linear work, fully sequential) |

### Cost Specs

| # | Algorithm | Work | Span | Source |
|---|-----------|------|------|--------|
| 1 | Example 27.1 (max) | W(n) = Theta(n) | S(n) = Theta(log n) | p. 177 |
| 2 | Algorithm 27.2 (reduce) | W(n) = O(n) | S(n) = O(log n) | p. 178-179 |
| 3 | Algorithm 27.3 (scan) | W(n) = O(n) | S(n) = O(log n) | p. 181 |

### Theorems / Properties

| # | Property | Description |
|---|----------|-------------|
| 1 | Associativity requirement | f must be associative for contraction correctness |
| 2 | Monoid identity | id is the left identity for f |
| 3 | Contraction preserves result | Contracting pairs under associative f preserves fold |
| 4 | Even-position correctness | In scan, even-indexed results from recursive call are correct |
| 5 | Odd-position expansion | Odd-indexed results computed by f(r[i/2], a[i-1]) |

### Exercises / Problems

None listed in the prose extract.

## Phase 3: Algorithmic Analysis

### Phase 3a: Cost Annotations

All exec functions now carry paired APAS/Claude-Opus-4.6 cost annotations.
All proof functions carry N/A annotations.

| # | Chap | File | Function | APAS Cost | Claude-Opus-4.6 Cost | Agreement |
|---|------|------|----------|-----------|-----------------------|-----------|
| 1 | 27 | ReduceContractStEph.rs | reduce_contract | W Theta(n), S Theta(log n) | W Theta(n), S Theta(n) | DISAGREE |
| 2 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | W Theta(n), S Theta(log n) | W Theta(n), S Theta(n) | DISAGREE |
| 3 | 27 | ReduceContractMtEph.rs | contract_parallel | N/A (helper) | W Theta(n), S Theta(n/2) | N/A |
| 4 | 27 | ScanContractStEph.rs | scan_contract | W Theta(n), S Theta(log n) | W Theta(n), S Theta(n) | DISAGREE |
| 5 | 27 | ScanContractStEph.rs | expand_scan | W Theta(n), S Theta(1) | W Theta(n), S Theta(n) | DISAGREE |
| 6 | 27 | ScanContractMtEph.rs | scan_contract_parallel | W Theta(n), S Theta(log n) | W Theta(n), S Theta(n) | DISAGREE |
| 7 | 27 | ScanContractMtEph.rs | expand_scan_parallel | W Theta(n), S Theta(1) | W Theta(n), S Theta(n) | DISAGREE |

**Disagreement reasons:**

1. **ReduceContractStEph::reduce_contract** — Sequential implementation with a `while` loop
   for contraction and sequential recursion. No parallelism, so span equals work: Theta(n).
   APAS states Theta(log n) span assuming parallel tabulate in contraction.

2. **ReduceContractMtEph::reduce_contract_parallel** — Uses `contract_parallel` which splits
   the contraction into two halves via `join`, but each half iterates sequentially. The
   recursion is sequential (not spawned in parallel). Contraction achieves one level of
   fork-join per recursion level, but each half still loops through n/4 elements. Total span
   across all log(n) levels is still Theta(n).

3. **contract_parallel** — One `join` splits work into two sequential halves. Each half
   iterates n/4 elements, so span per level is Theta(n/2), not Theta(1) as APAS assumes
   with a parallel tabulate.

4. **ScanContractStEph::scan_contract** — Sequential `while` loop for contraction and
   sequential expand. No parallelism. Span equals work.

5. **ScanContractStEph::expand_scan** — Sequential `while` loop iterating through all n/2
   pairs. APAS assumes a parallel tabulate for Theta(1) span.

6. **ScanContractMtEph::scan_contract_parallel** — Contraction uses `contract_parallel` (one
   level of join), but expansion (`expand_scan_parallel`) is a sequential loop. Recursion
   is sequential. Overall span Theta(n).

7. **ScanContractMtEph::expand_scan_parallel** — Despite the name, uses a sequential `while`
   loop. No `join` or thread spawning in the expand phase. Span equals work.

### Phase 3b: Implementation Fidelity

| # | Chap | File | Function | APAS Algorithm | Fidelity | Notes |
|---|------|------|----------|----------------|----------|-------|
| 1 | 27 | ReduceContractStEph.rs | reduce_contract | Alg 27.2 | Faithful | Handles odd-length (APAS assumes power-of-2) |
| 2 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | Alg 27.2 | Faithful | Same logic, contraction parallelized one level |
| 3 | 27 | ScanContractStEph.rs | scan_contract | Alg 27.3 | Faithful | Handles odd-length; exclusive scan (returns prefixes, not (prefixes, total)) |
| 4 | 27 | ScanContractMtEph.rs | scan_contract_parallel | Alg 27.3 | Faithful | Same logic, contraction reuses contract_parallel |

**Deviations from APAS:**

1. **Odd-length handling**: APAS assumes power-of-2 inputs. All implementations handle
   arbitrary-length inputs by processing a trailing element in the expand phase.

2. **Exclusive scan only**: APAS returns `(prefixes, total)`. The implementation returns
   only the prefix sequence (exclusive scan). The total is not returned separately.
   This is a minor interface difference; the total can be obtained by reducing the sequence.

3. **No parallel tabulate**: APAS uses parallel tabulate (Theta(1) span per level) for
   both contraction and expansion. The implementations use sequential loops or one-level
   `join`. To achieve the APAS span, `contract_parallel` would need a recursive parallel
   tabulate (or `tabulate` from ArraySeqMtEph), and expand would need parallel tabulate.

### Phase 3c: Spec Fidelity

| # | Chap | File | Function | Spec Strength | Fidelity | Notes |
|---|------|------|----------|---------------|----------|-------|
| 1 | 27 | ReduceContractStEph.rs | reduce_contract | Strong | Faithful | ensures == fold_left(id, spec_f), matches APAS reduce semantics |
| 2 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | Strong | Faithful | Same ensures as StEph variant |
| 3 | 27 | ScanContractStEph.rs | scan_contract | Strong | Faithful | ensures each prefix == fold_left(input[0..i], id, spec_f) |
| 4 | 27 | ScanContractStEph.rs | expand_scan | Strong | Faithful | ensures expansion produces complete prefix scan |
| 5 | 27 | ScanContractMtEph.rs | scan_contract_parallel | Strong | Faithful | Same ensures as StEph variant |
| 6 | 27 | ScanContractMtEph.rs | expand_scan_parallel | Strong | Faithful | Same ensures as StEph expand |

All specs faithfully encode APAS correctness properties:
- Reduce ensures result equals monoid fold over the input sequence.
- Scan ensures each output element equals the exclusive prefix fold.
- The `spec_monoid` precondition captures associativity and identity requirements.

No prose properties are missing from the specs.

## Phase 4: Parallelism Review

### Phase 4a: Mt Function Classification

| # | Chap | File | Function | Classification | Evidence |
|---|------|------|----------|----------------|----------|
| 1 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | Parallel | Calls contract_parallel (which uses join); but recursion is sequential |
| 2 | 27 | ReduceContractMtEph.rs | contract_parallel | Parallel | Uses HFSchedulerMtEph::join to split contraction into two halves |
| 3 | 27 | ScanContractMtEph.rs | scan_contract_parallel | Parallel | Contraction via contract_parallel; expansion is sequential |
| 4 | 27 | ScanContractMtEph.rs | expand_scan_parallel | Sequential | Sequential while loop, no join/spawn despite "_parallel" suffix |

### Phase 4b: Span Audit

| # | Chap | File | Function | APAS Span | Actual Span | Achieved? |
|---|------|------|----------|-----------|-------------|-----------|
| 1 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | Theta(log n) | Theta(n) | No |
| 2 | 27 | ReduceContractMtEph.rs | contract_parallel | N/A | Theta(n/2) | N/A |
| 3 | 27 | ScanContractMtEph.rs | scan_contract_parallel | Theta(log n) | Theta(n) | No |
| 4 | 27 | ScanContractMtEph.rs | expand_scan_parallel | Theta(1) | Theta(n) | No |

### Phase 4c: Parallelism Gap Table

| # | Function | APAS Span | Actual | Parallel? | Notes |
|---|----------|-----------|--------|-----------|-------|
| 1 | reduce_contract_parallel | Theta(log n) | Theta(n) | Partial | One join in contraction; needs parallel tabulate for Theta(1) contraction span |
| 2 | contract_parallel | N/A | Theta(n/2) | Yes | Splits into 2 halves via join; each half loops n/4 elements |
| 3 | scan_contract_parallel | Theta(log n) | Theta(n) | Partial | Contraction parallel; expansion sequential |
| 4 | expand_scan_parallel | Theta(1) | Theta(n) | No | Sequential loop despite name; needs parallel tabulate |

**Summary**: To achieve APAS span bounds, the contraction step needs a fully parallel
tabulate (recursive join down to base case), and the expansion step in scan needs parallel
tabulate as well. Currently, `contract_parallel` uses a single-level `join` that splits the
loop in half, which reduces span per level to n/2 but not to Theta(1). The expansion
(`expand_scan_parallel`) is entirely sequential.

## Phase 5: RTT Coverage

### Phase 5a: Coverage Check

| # | Chap | File | Exec Function | Test File | Covered? |
|---|------|------|---------------|-----------|----------|
| 1 | 27 | ReduceContractStEph.rs | reduce_contract | TestReduceContractStEph.rs | Yes |
| 2 | 27 | ReduceContractMtEph.rs | reduce_contract_parallel | TestReduceContractMtEph.rs | Yes |
| 3 | 27 | ReduceContractMtEph.rs | contract_parallel | TestReduceContractMtEph.rs | Indirect |
| 4 | 27 | ScanContractStEph.rs | scan_contract | TestScanContractStEph.rs | Yes |
| 5 | 27 | ScanContractStEph.rs | expand_scan | TestScanContractStEph.rs | Indirect |
| 6 | 27 | ScanContractMtEph.rs | scan_contract_parallel | TestScanContractMtEph.rs | Yes |
| 7 | 27 | ScanContractMtEph.rs | expand_scan_parallel | TestScanContractMtEph.rs | Indirect |

### Phase 5b: Test Quality

| # | Test File | Happy Path | Edge Cases | Spec-Relevant | Notes |
|---|-----------|:----------:|:----------:|:--------------:|-------|
| 1 | TestReduceContractStEph.rs | Yes | Yes (empty, single, 2-elem) | Yes (sum, product, max) | 10 tests; covers odd/even/power-of-2/large |
| 2 | TestReduceContractMtEph.rs | Yes | Yes (empty, single, 2-elem) | Yes (sum, product, max) | 10 tests; mirrors St tests |
| 3 | TestScanContractStEph.rs | Yes | Yes (empty, single, 2-elem) | Yes (prefix sums checked element-by-element) | 10 tests |
| 4 | TestScanContractMtEph.rs | Yes | Yes (empty, single, 2-elem) | Yes (prefix sums checked element-by-element) | 10 tests |

Test quality is strong across all four modules. Each test file covers:
- Empty sequence (returns identity or empty)
- Single element
- Two elements (minimal contraction)
- Odd and even lengths (exercises the odd-tail expand path)
- Power-of-2 sizes (16, 32)
- Large inputs (100 or 1000 elements)
- Multiple operations (sum, product, max for reduce; sum, product for scan)

### Phase 5c: Missing Tests

No missing tests identified. Coverage is comprehensive.

## Phase 6: PTT Review

No PTTs exist for Chapter 27. No PTTs are needed:
- No iterator implementations (no `iter()`, `IntoIterator`, `GhostIterator`)
- The verified loops use standard `while` patterns with straightforward invariants
- No "complicated callability" situations

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Prose Item | Status | Notes |
|---|-----------|--------|-------|
| 1 | Example 27.1 (maximal element) | Subsumed | reduce_contract with max function and 0 identity; noted in doc comment |
| 2 | Sequential scan via iterate (p. 179) | Not implemented | APAS shows this as a negative example of a sequential-only approach; not needed |
| 3 | Scan return type `(S_alpha, alpha)` | Partial | APAS returns (prefixes, total); code returns only prefixes |

**Note on gap 3**: The APAS scan signature returns a pair of the prefix sequence and the
total reduction. The implementation returns only the prefix sequence. The total can be
obtained by calling `reduce_contract` separately, so this is a minor interface simplification,
not a correctness gap.

### Code With No Prose Counterpart

| # | Chap | File | Item | Purpose |
|---|------|------|------|---------|
| 1 | 27 | ReduceContractStEph.rs | lemma_fold_left_monoid | Verus proof scaffolding for monoid fold properties |
| 2 | 27 | ReduceContractStEph.rs | lemma_fold_left_pair | Verus proof helper for 2-element fold |
| 3 | 27 | ReduceContractStEph.rs | lemma_fold_left_singleton | Verus proof helper for 1-element fold |
| 4 | 27 | ReduceContractStEph.rs | lemma_contraction_even | Core contraction correctness lemma |
| 5 | 27 | ScanContractStEph.rs | lemma_prefix_contraction | Prefix fold vs contracted fold |
| 6 | 27 | ScanContractStEph.rs | lemma_expand_even | Expand step correctness for even indices |
| 7 | 27 | ScanContractStEph.rs | lemma_expand_odd | Expand step correctness for odd indices |
| 8 | 27 | ScanContractStEph.rs | lemma_expand_odd_tail | Odd-length tail handling |
| 9 | 27 | ReduceContractMtEph.rs | contract_parallel | Factored-out parallel contraction step |
| 10 | 27 | Various | Duplicated proof lemmas | Same 4-8 lemmas duplicated across St/Mt and reduce/scan modules |

Items 1-8 are proof scaffolding required by Verus that have no direct prose counterpart.
They formalize the mathematical reasoning that APAS presents informally (e.g., "by
associativity, the sum is preserved"). Item 9 is a code factoring decision (sharing
contraction between reduce and scan Mt modules). Item 10 is a consequence of the
chapter-standalone rule: each module duplicates proof lemmas rather than importing them.

## Phase 8: TOC and In/Out Table

### TOC Review

| # | Chap | File | TOC Present | Order Correct | Notes |
|---|------|------|:-----------:|:-------------:|-------|
| 1 | 27 | ReduceContractStEph.rs | Yes | Yes | Sections 1, 2, 3, 7, 8, 9 |
| 2 | 27 | ReduceContractMtEph.rs | Yes | Yes | Sections 1, 2, 3, 7, 8, 9 |
| 3 | 27 | ScanContractStEph.rs | Yes | Yes | Sections 1, 2, 3, 7, 8, 9 |
| 4 | 27 | ScanContractMtEph.rs | Yes | Yes | Sections 1, 2, 3, 7, 8, 9 |

All files follow standard section ordering. No sections 4-6, 10-13 (no type definitions,
view impls, spec fns, iterators, locking, macros, or derive impls in these modules).

### In/Out Table

| # | Chap | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|------|------|:-----:|:------------:|:-------:|:----:|:--------:|:-----:|:-------:|:-----:|-------|
| 1 | 27 | ReduceContractStEph.rs | - | - | - | - | - | - | - | - | - |
| 2 | 27 | ReduceContractMtEph.rs | - | - | - | - | - | - | - | - | - |
| 3 | 27 | ScanContractStEph.rs | - | - | - | - | - | - | - | - | - |
| 4 | 27 | ScanContractMtEph.rs | - | - | - | - | - | - | - | - | - |

No derive impls needed. All code is inside `verus!`. No code outside `verus!` except the
module declaration and closing braces.

## Proof Holes Summary

| # | Chap | File | Holes | Proof Fns | Status |
|---|------|------|:-----:|:---------:|--------|
| 1 | 27 | ReduceContractStEph.rs | 0 | 4 clean | Clean |
| 2 | 27 | ReduceContractMtEph.rs | 0 | 4 clean | Clean |
| 3 | 27 | ScanContractStEph.rs | 0 | 8 clean | Clean |
| 4 | 27 | ScanContractMtEph.rs | 0 | 8 clean | Clean |

**Total: 0 holes across 4 modules, 24 clean proof functions, 13 exec functions with complete specs.**

## Style Warnings

25 style warnings across 4 files, all rule [23]: free proof functions have unbounded type
parameter `T` while the module trait bounds it to `StT` or `StTInMtT`. These are benign
because the proof lemmas are generic by design (they work for any `T`, not just the
trait-constrained `T`). Tightening the bounds would be cosmetic only.

## Summary

Chapter 27 is a clean, well-verified implementation of contraction-based reduce and scan.
All 4 modules verify with 0 holes. Specs are strong and faithful to the APAS prose. RTT
coverage is comprehensive with 40 tests across 4 test files.

The main gap is **parallelism depth**: both Mt modules achieve only partial parallelism.
The contraction step uses one level of fork-join (`join` splitting the loop into two halves),
but APAS assumes a fully parallel tabulate (Theta(1) span per level). The expansion step
in scan is entirely sequential. To achieve the APAS Theta(log n) span, both contraction
and expansion would need recursive parallel tabulate or direct use of
`ArraySeqMtEph::tabulate`.
