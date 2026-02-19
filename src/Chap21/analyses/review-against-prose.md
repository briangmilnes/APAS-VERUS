<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 21 — Review Against Prose

**Date:** 2026-02-18
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap21.txt`
**Source directory:** `src/Chap21/`

---

## Phase 1: Inventory

### Function Table with Spec Strengths

| # | File | Function | Kind | V! | Spec Strength | Notes |
|---|---|---|---|:---:|---|---|
| 1 | Algorithm21_1.rs | `sum_inner_lens` | spec fn | Y | — | Helper spec for flatten length |
| 2 | Algorithm21_1.rs | `lemma_sum_inner_lens_mono` | proof fn | Y | strong | Monotonicity of sum_inner_lens |
| 3 | Algorithm21_1.rs | `lemma_sum_inner_lens_uniform` | proof fn | Y | strong | Uniform inner length → sum = k*m |
| 4 | Algorithm21_1.rs | `flatten_inner` | exec fn | Y | partial | Length correct, element content unspecified |
| 5 | Algorithm21_1.rs | `points2d_tab_flat` | exec fn | Y | partial | Length correct, no element content |
| 6 | Algorithm21_2.rs | `points3d_tab_flat` | exec fn | Y | partial | Length = pow(n,3), no element bounds |
| 7 | Algorithm21_5.rs | `primes_bf` | exec fn | Y | strong | Soundness + completeness: every element is prime, every prime in range appears |
| 8 | Algorithm21_6.rs | `lemma_product_not_prime` | proof fn | Y | strong | Product of two integers ≥ 2 is not prime |
| 9 | Algorithm21_6.rs | `prime_sieve` | exec fn | Y | partial | Element bounds (2 ≤ x ≤ n), no primality guarantee |
| 10 | Exercise21_5.rs | `lemma_inner_lens_sum_triangular` | proof fn | Y | strong | Descending-length pattern sums to n*(n+1)/2 |
| 11 | Exercise21_5.rs | `all_contiguous_subseqs` | exec fn | Y | partial | Length = n*(n+1)/2 (triangular), no element content |
| 12 | Exercise21_6.rs | — | docs only | — | — | Cost analysis documentation, no code |
| 13 | Exercise21_7.rs | `spec_is_even` | spec fn | Y | — | |
| 14 | Exercise21_7.rs | `is_even` | exec fn | Y | strong | r == spec_is_even |
| 15 | Exercise21_7.rs | `spec_is_vowel` | spec fn | Y | — | |
| 16 | Exercise21_7.rs | `is_vowel` | exec fn | Y | strong | r == spec_is_vowel |
| 17 | Exercise21_7.rs | `pair_even_with_vowels` | exec fn | Y | partial | Length ≤ |a|·|b|, no element content |
| 18 | Exercise21_8.rs | `spec_is_prime` | spec fn | Y | — | |
| 19 | Exercise21_8.rs | `spec_divisor_count` | spec fn | Y | — | |
| 20 | Exercise21_8.rs | `lemma_zero_count_means_no_divisors` | proof fn | Y | strong | Count 0 → no divisors |
| 21 | Exercise21_8.rs | `lemma_no_divisors_means_zero_count` | proof fn | Y | strong | No divisors → count 0 |
| 22 | Exercise21_8.rs | `lemma_divisor_count_nonneg` | proof fn | Y | strong | Non-negativity |
| 23 | Exercise21_8.rs | `is_divisible` | exec fn | Y | strong | divides == (n % i == 0) |
| 24 | Exercise21_8.rs | `is_prime` | exec fn | Y | strong | prime == spec_is_prime(n) — fully verified |
| 25 | Exercise21_8.rs | `lemma_filter_len_eq_divisor_count` | proof fn | Y | strong | Bridge: spec_filter_len over bool seq == spec_divisor_count |
| 26 | Exercise21_8.rs | `lemma_divisor_count_split_last` | proof fn | Y | strong | Splits last element off spec_divisor_count range |
| 27 | Exercise21_9.rs | `spec_is_composite` | spec fn | Y | — | m > 1 with a divisor d where 2 ≤ d < m |
| 28 | Exercise21_9.rs | `lemma_div_exact` | proof fn | Y | strong | d divides m → m == d * (m / d) |
| 29 | Exercise21_9.rs | `lemma_composite_has_small_divisor` | proof fn | Y | strong | Every composite has a divisor d with d² ≤ m |
| 30 | Exercise21_9.rs | `lemma_composites_covered_by_small_multiples` | proof fn | Y | strong | Composites in [2,n] are covered by multiples of i where i² ≤ n |
| 31 | Problem21_1.rs | `points2d` | exec fn | Y | strong | Length + element coordinate bounds |
| 32 | Problem21_3.rs | `points3d_loops` | exec fn | Y | strong | Length + element coordinate bounds |
| 33 | Problem21_4.rs | `cartesian_loops` | exec fn | Y | partial | Length only, no element content |
| 34 | Problem21_4.rs | `cartesian_tab_flat` | exec fn | Y | strong | Length + element containment (a.contains first, b.contains second) |

**Summary:** 12 source files, 27 locally-defined functions (15 exec, 12 proof), 6 spec fns, 1 documentation-only module. The `lemma_flatten_uniform_len` lemma, previously duplicated in Algorithm21_2.rs and Problem21_4.rs, is now imported from `vstdplus::seq::seq`.

---

## Phase 2: Prose Inventory

### Definitions

| # | Item | Prose Section | Description |
|---|---|---|---|
| 1 | Points in 2D | Problem 21.1 | Sequence of (x,y) with 0 ≤ x < n, 1 ≤ y < n |
| 2 | Points in 3D | Problem 21.3 | Sequence of (x,y,z) with 0 ≤ x < n, 1 ≤ y ≤ n, 2 ≤ z ≤ n+1 |
| 3 | Cartesian Product | Problem 21.4 | All pairs (a[i], b[j]) for sequences a, b |
| 4 | Contiguous subsequences | Exercise 21.5 | All subseq a[i..j] for 0 ≤ i ≤ j < |a| |
| 5 | isPrime | Algorithm 21.4 | n has exactly two distinct divisors (1 and itself) |
| 6 | Composite number | Section 2 | Not prime; has a divisor ≤ √n |

### Algorithms

| # | Algorithm | Prose Reference | Description |
|---|---|---|---|
| 1 | Algorithm 21.1 | 2D Points via tabulate + flatten | flatten(tabulate(λx. tabulate(λy. (x,y+1)) (n-1)) n) |
| 2 | Algorithm 21.2 | 3D Points via nested flatten | flatten(flatten nested tabulates) |
| 3 | Algorithm 21.3 | Cartesian Product | flatten(map(λx. map(λy. (x,y)) b) a) |
| 4 | Exercise 21.5 | All contiguous subseqs | flatten(tabulate(λi. tabulate(λj. a[i..i+j]) (|a|-i)) |a|) |
| 5 | Exercise 21.7 | Comprehension with conditionals | flatten ⟨⟨(x,y) : y∈b | isVowel y⟩ : x∈a | isEven x⟩ |
| 6 | Algorithm 21.4 | Brute force isPrime | |{n mod i : 1 ≤ i ≤ √n | n mod i == 0}| == 1 |
| 7 | Algorithm 21.5 | Brute force primesBF | filter isPrime over [2..n) |
| 8 | Algorithm 21.6 | Prime Sieve | Generate composites, use ninject for sieve, filter |

### Cost Specifications from Prose

| # | Algorithm | Work | Span |
|---|---|---|---|
| 1 | Algorithm 21.1 (2D Points) | Θ(n²) | Θ(lg n) |
| 2 | Algorithm 21.2 (3D Points) | Θ(n³) | Θ(lg n) |
| 3 | Algorithm 21.4 (isPrime) | Θ(√n) | Θ(lg n) |
| 4 | Algorithm 21.5 (primesBF) | Θ(n^{3/2}) | Θ(lg n) |
| 5 | Algorithm 21.6 (primeSieve) | Θ(n lg n) | Θ(lg n) |
| 6 | Exercise 21.5 (all subseqs) | Θ(n²) | Θ(lg n) |
| 7 | Exercise 21.6 (cost analysis) | Θ(n²) work, Θ(lg n) span | (analysis only) |

### Theorems/Exercises

| # | Item | Description | Implemented? |
|---|---|---|---|
| 1 | Exercise 21.2 | Cost analysis of 2D Points | No separate file (implicit in Algorithm21_1) |
| 2 | Exercise 21.9 | Prove composites ≤ √n suffice | **Yes** — 3 proof lemmas in Exercise21_9.rs |

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Function | APAS Cost | Claude-Opus-4.6 Cost | Match? | Notes |
|---|---|---|---|:---:|---|
| 1 | `flatten_inner` | W Θ(m), S Θ(lg k) | W Θ(m), S Θ(m) | Work ✓, Span ✗ | Sequential two-pass impl |
| 2 | `points2d_tab_flat` | W Θ(n²), S Θ(lg n) | W Θ(n²), S Θ(n²) | Work ✓, Span ✗ | Sequential StPer |
| 3 | `points3d_tab_flat` | W Θ(n³), S Θ(lg n) | W Θ(n³), S Θ(n³) | Work ✓, Span ✗ | Sequential StPer |
| 4 | `primes_bf` | W Θ(n^{3/2}), S Θ(lg n) | W Θ(n^{3/2}), S Θ(n^{3/2}) | Work ✓, Span ✗ | Sequential StPer |
| 5 | `prime_sieve` | W Θ(n lg n), S Θ(lg n) | W Θ(n lg n), S Θ(n lg n) | Work ✓, Span ✗ | Ninject-based boolean sieve; sequential StPer |
| 6 | `all_contiguous_subseqs` | W Θ(n²), S Θ(lg n) | W Θ(n³), S Θ(n³) | **Both ✗** | subseq_copy is O(k), not O(1) |
| 7 | `is_even` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | ✓ | |
| 8 | `is_vowel` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | ✓ | |
| 9 | `pair_even_with_vowels` | W Θ(|a|·|b|), S Θ(lg |a|) | W Θ(|a|·|b|), S Θ(|a|·|b|) | Work ✓, Span ✗ | Sequential StPer |
| 10 | `is_divisible` | W Θ(1), S Θ(1) | W Θ(1), S Θ(1) | ✓ | |
| 11 | `is_prime` | W Θ(√n), S Θ(lg n) | W Θ(√n), S Θ(√n) | Work ✓, Span ✗ | Sequential StEph |
| 12 | `points2d` | W Θ(n²), S Θ(n²) | W Θ(n²), S Θ(n²) | ✓ | Imperative, expected sequential |
| 13 | `points3d_loops` | W Θ(n³), S Θ(n³) | W Θ(n³), S Θ(n³) | ✓ | Imperative, expected sequential |
| 14 | `cartesian_loops` | W Θ(|a|·|b|), S Θ(|a|·|b|) | W Θ(|a|·|b|), S Θ(|a|·|b|) | ✓ | Imperative, expected sequential |
| 15 | `cartesian_tab_flat` | W Θ(|a|·|b|), S Θ(lg |a|) | W Θ(|a|·|b|), S Θ(|a|·|b|) | Work ✓, Span ✗ | Sequential StPer |

**Summary:** 7/15 fully match, 7/15 match on work but not span (expected for StPer/StEph), 1/15 diverges on work (`all_contiguous_subseqs`).

### 3b. Implementation Fidelity

| # | Algorithm | Fidelity | Notes |
|---|---|:---:|---|
| 1 | Algo 21.1 (2D Points, tabulate+flatten) | High | Matches prose: tabulate of tabulate then flatten |
| 2 | Algo 21.2 (3D Points, nested flatten) | High | Matches prose: nested flatten of tabulate |
| 3 | Algo 21.3 (Cartesian Product) | High | Both imperative and functional versions provided |
| 4 | Ex 21.5 (All contiguous subseqs) | High | Matches nested tabulate + flatten structure |
| 5 | Ex 21.7 (Comprehension with conditionals) | High | filter + tabulate + flatten matches prose |
| 6 | Algo 21.4 (isPrime) | High | tabulate + filter per textbook |
| 7 | Algo 21.5 (primesBF) | High | tabulate + filter(isPrime) matches prose |
| 8 | Algo 21.6 (primeSieve) | High | Ninject-based boolean sieve: generate composites, mark sieve positions false, collect primes |
| 9 | Ex 21.9 (Composite √n proof) | High | Three lemmas proving composites are covered by multiples of numbers ≤ √n |

**Implementation note:** Algorithm 21.6 (`prime_sieve`) now uses the ninject-based approach: generate composites via nested tabulate + flatten, iterate to set sieve[c]=false for each composite, then collect indices where sieve is true. Work matches APAS Θ(n lg n). The `lemma_product_not_prime` proof supports the soundness argument.

### 3c. Spec Fidelity

| # | Function | Issue |
|---|---|---|
| 1 | `points2d_tab_flat` | Ensures specifies length but not element content. The imperative `points2d` is stronger (includes coordinate bounds). |
| 2 | `points3d_tab_flat` | Same: length only vs. `points3d_loops` which includes coordinate bounds. |
| 3 | `prime_sieve` | Element bounds (2 ≤ x ≤ n) but no primality guarantee. Stronger than before (was length-only) but still missing the key property. |
| 4 | `cartesian_loops` | Length only. The functional `cartesian_tab_flat` is now stronger (includes element containment). |

**Resolved since last review:**
- `primes_bf` now has both soundness (every element is prime) and completeness (every prime in [2,n) appears in result).
- `all_contiguous_subseqs` now ensures the exact count: `result.len() * 2 == n * (n+1)` (triangular number).
- `pair_even_with_vowels` now ensures `result.len() <= |a| * |b|` (length upper bound).
- `cartesian_tab_flat` now ensures element containment: every pair's first is in a, second is in b.

---

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 21.** All implementations use StPer (single-threaded persistent) or StEph (single-threaded ephemeral). The APAS algorithms are inherently parallel (tabulate, flatten, filter), but no parallel variants have been implemented.

This is a notable gap: the APAS chapter emphasizes parallelism through comprehensions and the span analysis assumes parallel execution. Mt variants would be needed to achieve the prose's span bounds.

---

## Phase 5: Runtime Test Review

**No runtime tests exist.** No `tests/test_Chap21*` files found.

### Missing Test Coverage

| # | Function | Priority | Suggested Test |
|---|---|---|---|
| 1 | `points2d` / `points2d_tab_flat` | High | Verify output for n=0,1,3,5; check both produce same elements |
| 2 | `points3d_loops` / `points3d_tab_flat` | High | Verify for small n; check both produce same length |
| 3 | `cartesian_loops` / `cartesian_tab_flat` | High | Verify for known inputs; compare both implementations |
| 4 | `is_prime` | High | Test known primes and composites (2,3,4,5,10,97,100) |
| 5 | `primes_bf` | High | Compare against known prime lists for n=10,20,100 |
| 6 | `prime_sieve` | High | Compare against primes_bf for same n |
| 7 | `all_contiguous_subseqs` | Medium | Verify count = n(n+1)/2 for small sequences |
| 8 | `pair_even_with_vowels` | Medium | Test with known even/vowel inputs |

---

## Phase 6: PTT Review

Chapter 21 has no iterators and no verified loops that require proof-time tests. The while loops in `Problem21_1.rs`, `Problem21_3.rs`, and `Problem21_4.rs` use loop invariants verified directly by Verus (not through PTT infrastructure).

**No PTTs needed** for the current implementation state.

---

## Phase 7: Gap Analysis

### Prose Items with No Implementation

| # | Prose Item | Status |
|---|---|---|
| 1 | Exercise 21.2 (Cost analysis of 2D Points) | Not separately implemented; cost is documented in Algorithm21_1.rs comments |
| 2 | Remark on eliminating flatten | N/A — discussion only, no algorithm |
| 3 | Remark on comprehensions | N/A — discussion only |

### Code with No Direct Prose Counterpart

| # | Code Item | Notes |
|---|---|---|
| 1 | `Problem21_1.rs` (`points2d` imperative) | Prose gives functional form; imperative version is a bonus implementation |
| 2 | `Problem21_3.rs` (`points3d_loops` imperative) | Same: bonus imperative variant |
| 3 | `Problem21_4.rs` (`cartesian_loops` imperative) | Same: bonus imperative variant |
| 4 | `flatten_inner` (Algorithm21_1.rs) | Helper not in prose; needed because flatten is not available as a library operation |
| 5 | `lemma_sum_inner_lens_mono/uniform` | Verus proof scaffolding |
| 6 | `spec_divisor_count` and associated lemmas | Verus proof infrastructure for isPrime |
| 7 | `lemma_inner_lens_sum_triangular` (Exercise21_5.rs) | Proves descending-length sum = n*(n+1)/2; Verus proof scaffolding |
| 8 | `lemma_product_not_prime` (Algorithm21_6.rs) | Proves product of ≥ 2 is composite; Verus proof scaffolding for sieve |

### Resolved Issues

- **Exercise 21.9 implemented:** Was a placeholder; now contains `spec_is_composite`, `lemma_div_exact`, `lemma_composite_has_small_divisor`, and `lemma_composites_covered_by_small_multiples` — proving that composites in [2,n] are covered by multiples of i where i² ≤ n.
- **Duplication resolved:** `lemma_flatten_uniform_len` was previously duplicated in Algorithm21_2.rs and Problem21_4.rs. Both files now import from `crate::vstdplus::seq::seq::lemma_flatten_uniform_len`.

---

## Phase 8: TOC Review

### TOC Presence

| # | File | TOC Present? | Sections Used |
|---|---|:---:|---|
| 1 | Algorithm21_1.rs | Yes | module, imports, broadcast use, spec fns, proof fns, impls |
| 2 | Algorithm21_2.rs | Yes | module, imports, broadcast use, impls |
| 3 | Algorithm21_5.rs | Yes | module, imports, broadcast use, type definitions, impls |
| 4 | Algorithm21_6.rs | Yes | module, imports, broadcast use, proof fns, impls |
| 5 | Exercise21_5.rs | Yes | module, imports, broadcast use, proof fns, impls |
| 6 | Exercise21_6.rs | No | documentation only |
| 7 | Exercise21_7.rs | Yes | module, imports, broadcast use, spec fns, impls |
| 8 | Exercise21_8.rs | Yes | module, imports, broadcast use, spec fns, proof fns, impls |
| 9 | Exercise21_9.rs | Yes | module, spec fns, proof fns |
| 10 | Problem21_1.rs | Yes | module, imports, broadcast use, impls |
| 11 | Problem21_3.rs | Yes | module, imports, broadcast use, impls |
| 12 | Problem21_4.rs | Yes | module, broadcast use, impls |

**Finding:** 11 of 12 files have TOC headers per the project standard. Exercise21_6.rs is documentation-only and does not need one.

### In/Out Table

| # | File | Clone | PartialEq/Eq | Default | Drop | Iterator | Debug | Display | Macro | Other |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|---|
| 1 | Algorithm21_1.rs | - | - | - | - | - | - | - | - | - |
| 2 | Algorithm21_2.rs | - | - | - | - | - | - | - | - | - |
| 3 | Algorithm21_5.rs | - | - | - | - | - | - | - | - | - |
| 4 | Algorithm21_6.rs | - | - | - | - | - | - | - | - | - |
| 5 | Exercise21_5.rs | - | - | - | - | - | - | - | - | - |
| 6 | Exercise21_7.rs | - | - | - | - | - | - | - | - | - |
| 7 | Exercise21_8.rs | - | - | - | - | - | - | - | - | - |
| 8 | Problem21_1.rs | - | - | - | - | - | - | - | - | - |
| 9 | Problem21_3.rs | - | - | - | - | - | - | - | - | - |
| 10 | Problem21_4.rs | - | - | - | - | - | - | - | - | - |

Chapter 21 defines no new data types, so no derive impls are needed.

---

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap21/

✓ Algorithm21_1.rs   (2 clean proof functions)
✓ Algorithm21_2.rs
✓ Algorithm21_5.rs
✓ Algorithm21_6.rs   (1 clean proof function)
✓ Exercise21_5.rs    (1 clean proof function)
✓ Exercise21_6.rs
✓ Exercise21_7.rs
✓ Exercise21_8.rs    (5 clean proof functions)
✓ Exercise21_9.rs    (3 clean proof functions)
✓ Problem21_1.rs
✓ Problem21_3.rs
✓ Problem21_4.rs

SUMMARY: 12 clean modules, 0 holed modules
Proof functions: 12 total (12 clean, 0 holed)
Holes: 0 total

🎉 No proof holes or warnings found! All proofs are complete.
```

**All 12 proof functions across 5 modules are clean.** The two `assume()` holes that previously existed in Exercise21_8.rs (isqrt bound and filter-count bridge) were closed in a prior update by adding `lemma_filter_len_eq_divisor_count` and `lemma_divisor_count_split_last`.

---

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 20 |
| partial | 7 |
| weak | 0 |
| none | 0 |

| Strength | Functions |
|---|---|
| **strong** | `lemma_sum_inner_lens_mono`, `lemma_sum_inner_lens_uniform`, `is_even`, `is_vowel`, `lemma_zero_count_means_no_divisors`, `lemma_no_divisors_means_zero_count`, `lemma_divisor_count_nonneg`, `is_divisible`, `is_prime`, `lemma_filter_len_eq_divisor_count`, `lemma_divisor_count_split_last`, `points2d`, `points3d_loops`, `primes_bf`, `cartesian_tab_flat`, `lemma_product_not_prime`, `lemma_inner_lens_sum_triangular`, `lemma_div_exact`, `lemma_composite_has_small_divisor`, `lemma_composites_covered_by_small_multiples` |
| **partial** | `flatten_inner`, `points2d_tab_flat`, `points3d_tab_flat`, `prime_sieve`, `all_contiguous_subseqs`, `pair_even_with_vowels`, `cartesian_loops` |

**Changes since last review:** `primes_bf` upgraded from partial to strong (completeness added). `cartesian_tab_flat` upgraded from partial to strong (element containment added). `prime_sieve` upgraded from weak to partial (element bounds added). `all_contiguous_subseqs` upgraded from none to partial (triangular length property). `pair_even_with_vowels` upgraded from none to partial (length upper bound). Five new strong proof functions added (Exercise21_5, Algorithm21_6, Exercise21_9). Two `lemma_flatten_uniform_len` duplicates removed (moved to vstdplus).

---

## Overall Assessment

### Strengths

1. **Full coverage:** All 8 prose algorithms/exercises have implementations, plus 3 bonus imperative variants and Exercise 21.9's proof.
2. **Zero proof holes:** All 12 proof functions across 12 modules are clean.
3. **Strong proof infrastructure:** The isPrime proof in Exercise21_8.rs is sophisticated with 5 clean proof lemmas. Exercise21_9.rs proves that composites are covered by multiples of numbers ≤ √n. Algorithm21_6 has `lemma_product_not_prime` supporting the sieve.
4. **All code is inside verus!:** Every executable and proof function is properly inside `verus!` blocks.
5. **Imperative variants provide good spec contrast:** `points2d` and `points3d_loops` have strictly stronger specs than their functional counterparts.
6. **`primes_bf` is fully specified:** Both soundness (every element is prime) and completeness (every prime in range appears).
7. **No duplication:** `lemma_flatten_uniform_len` is now imported from vstdplus in both Algorithm21_2 and Problem21_4.
8. **TOC headers present:** 11 of 12 files have the project-standard TOC.

### Weaknesses

1. **No runtime tests at all:** Zero test coverage for the entire chapter.
2. **Functional variants have weaker specs than imperative variants:** `points2d_tab_flat` specifies only length while `points2d` also specifies element bounds. This pattern repeats for 3D points.
3. **`prime_sieve` lacks primality guarantee:** Has element bounds (2 ≤ x ≤ n) but doesn't prove returned elements are actually prime.
4. **No Mt (parallel) variants:** The chapter emphasizes parallelism but all implementations are single-threaded.
5. **`cartesian_loops` weaker than `cartesian_tab_flat`:** The imperative version has length-only spec while the functional version now has element containment.

### Priority Recommendations

| # | Priority | Action |
|---|---|---|
| 1 | High | Add runtime tests for all algorithms |
| 2 | Medium | Strengthen specs on functional variants (`points2d_tab_flat`, `points3d_tab_flat`) to include element bounds |
| 3 | Medium | Add primality guarantee to `prime_sieve` ensures |
| 4 | Medium | Strengthen `cartesian_loops` to match `cartesian_tab_flat`'s element containment spec |
| 5 | Low | Consider Mt variants for the tabulate+flatten algorithms |
