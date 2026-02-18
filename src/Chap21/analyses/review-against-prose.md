<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 21 — Review Against Prose

**Date:** 2026-02-13
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
| 6 | Algorithm21_2.rs | `lemma_flatten_uniform_len` | proof fn | Y | strong | Seq::flatten uniform length |
| 7 | Algorithm21_2.rs | `points3d_tab_flat` | exec fn | Y | partial | Length = pow(n,3), no element bounds |
| 8 | Algorithm21_5.rs | `primes_bf` | exec fn | Y | partial | Soundness only (every element is prime), no completeness |
| 9 | Algorithm21_6.rs | `prime_sieve` | exec fn | Y | weak | Length bounds only, no primality guarantee |
| 10 | Exercise21_5.rs | `all_contiguous_subseqs` | exec fn | Y | none | No ensures clause |
| 11 | Exercise21_6.rs | — | docs only | — | — | Cost analysis documentation, no code |
| 12 | Exercise21_7.rs | `spec_is_even` | spec fn | Y | — | |
| 13 | Exercise21_7.rs | `is_even` | exec fn | Y | strong | r == spec_is_even |
| 14 | Exercise21_7.rs | `spec_is_vowel` | spec fn | Y | — | |
| 15 | Exercise21_7.rs | `is_vowel` | exec fn | Y | strong | r == spec_is_vowel |
| 16 | Exercise21_7.rs | `pair_even_with_vowels` | exec fn | Y | none | No ensures clause |
| 17 | Exercise21_8.rs | `spec_is_prime` | spec fn | Y | — | |
| 18 | Exercise21_8.rs | `spec_divisor_count` | spec fn | Y | — | |
| 19 | Exercise21_8.rs | `lemma_zero_count_means_no_divisors` | proof fn | Y | strong | Count 0 → no divisors |
| 20 | Exercise21_8.rs | `lemma_no_divisors_means_zero_count` | proof fn | Y | strong | No divisors → count 0 |
| 21 | Exercise21_8.rs | `lemma_divisor_count_nonneg` | proof fn | Y | strong | Non-negativity |
| 22 | Exercise21_8.rs | `is_divisible` | exec fn | Y | strong | divides == (n % i == 0) |
| 23 | Exercise21_8.rs | `is_prime` | exec fn | Y | strong | prime == spec_is_prime(n) (2 proof holes) |
| 24 | Exercise21_9.rs | — | placeholder | — | — | Proof-only exercise, no Verus code |
| 25 | Problem21_1.rs | `points2d` | exec fn | Y | strong | Length + element coordinate bounds |
| 26 | Problem21_3.rs | `points3d_loops` | exec fn | Y | strong | Length + element coordinate bounds |
| 27 | Problem21_4.rs | `cartesian_loops` | exec fn | Y | partial | Length only, no element content |
| 28 | Problem21_4.rs | `lemma_flatten_uniform_len` | proof fn | Y | strong | Duplicate of Algorithm21_2's lemma |
| 29 | Problem21_4.rs | `cartesian_tab_flat` | exec fn | Y | partial | Length only, no element content |

**Summary:** 12 source files, 22 veracity-tracked functions (15 exec, 7 proof), 4 spec fns, 2 placeholder modules.

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
| 2 | Exercise 21.9 | Prove composites ≤ √n suffice | Placeholder only (Exercise21_9.rs) |

---

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

| # | Function | APAS Cost | Claude-Opus-4.6 Cost | Match? | Notes |
|---|---|---|---|:---:|---|
| 1 | `flatten_inner` | W Θ(m), S Θ(lg k) | W Θ(m), S Θ(m) | Work ✓, Span ✗ | Sequential two-pass impl |
| 2 | `points2d_tab_flat` | W Θ(n²), S Θ(lg n) | W Θ(n²), S Θ(n²) | Work ✓, Span ✗ | Sequential StPer |
| 3 | `points3d_tab_flat` | W Θ(n³), S Θ(lg n) | W Θ(n³), S Θ(n³) | Work ✓, Span ✗ | Sequential StPer |
| 4 | `primes_bf` | W Θ(n^{3/2}), S Θ(lg n) | W Θ(n^{3/2}), S Θ(n^{3/2}) | Work ✓, Span ✗ | Sequential StPer |
| 5 | `prime_sieve` | W Θ(n lg n), S Θ(lg n) | W Θ(n² lg n), S Θ(n² lg n) | **Both ✗** | Linear membership scan instead of ninject sieve |
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

**Summary:** 6/15 fully match, 7/15 match on work but not span (expected for StPer/StEph), 2/15 diverge on work (`prime_sieve`, `all_contiguous_subseqs`).

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
| 8 | Algo 21.6 (primeSieve) | **Low** | Uses linear membership test instead of ninject sieve; asymptotically worse |

**Key deviation:** Algorithm 21.6 (`prime_sieve`) does not use `ninject` to build a boolean sieve array. Instead, it generates the list of composites and uses a linear scan filter to check membership. This changes the work from Θ(n lg n) to Θ(n² lg n), a significant algorithmic divergence.

### 3c. Spec Fidelity

| # | Function | Issue |
|---|---|---|
| 1 | `points2d_tab_flat` | Ensures specifies length but not element content. The imperative `points2d` is stronger (includes coordinate bounds). |
| 2 | `points3d_tab_flat` | Same: length only vs. `points3d_loops` which includes coordinate bounds. |
| 3 | `primes_bf` | Soundness only: every returned element is prime. Missing completeness: all primes < n are present. |
| 4 | `prime_sieve` | Only upper-bounds the result length. No primality guarantee at all. |
| 5 | `all_contiguous_subseqs` | No ensures clause. |
| 6 | `pair_even_with_vowels` | No ensures clause. |
| 7 | `cartesian_loops` / `cartesian_tab_flat` | Length only. No ensures that elements are actual pairs from a and b. |

---

## Phase 4: Parallelism Review

**No Mt (multi-threaded) modules exist in Chapter 21.** All implementations use StPer (single-threaded persistent) or StEph (single-threaded ephemeral). The APAS algorithms are inherently parallel (tabulate, flatten, filter), but no parallel variants have been implemented.

This is a notable gap: the APAS chapter emphasizes parallelism through comprehensions and the span analysis assumes parallel execution. Mt variants would be needed to achieve the prose's span bounds.

---

## Phase 5: Runtime Test Review

**No runtime tests exist.** The directories `tests/Chap21/` and `rust_verify_test/tests/Chap21/` do not exist.

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
| 2 | Exercise 21.9 (Prove composites ≤ √n suffice) | **Placeholder only** — Exercise21_9.rs is empty |
| 3 | Remark on eliminating flatten | N/A — discussion only, no algorithm |
| 4 | Remark on comprehensions | N/A — discussion only |

### Code with No Direct Prose Counterpart

| # | Code Item | Notes |
|---|---|---|
| 1 | `Problem21_1.rs` (`points2d` imperative) | Prose gives functional form; imperative version is a bonus implementation |
| 2 | `Problem21_3.rs` (`points3d_loops` imperative) | Same: bonus imperative variant |
| 3 | `Problem21_4.rs` (`cartesian_loops` imperative) | Same: bonus imperative variant |
| 4 | `flatten_inner` (Algorithm21_1.rs) | Helper not in prose; needed because flatten is not available as a library operation |
| 5 | `lemma_sum_inner_lens_mono/uniform` | Verus proof scaffolding |
| 6 | `lemma_flatten_uniform_len` (duplicated in Algorithm21_2.rs and Problem21_4.rs) | Should be deduplicated into a shared module |
| 7 | `spec_divisor_count` and associated lemmas | Verus proof infrastructure for isPrime |

### Duplication

`lemma_flatten_uniform_len` appears in both `Algorithm21_2.rs` and `Problem21_4.rs` with identical implementations. This should be refactored into a shared module (e.g., `vstdplus` or a Chap21 utils module).

---

## Phase 8: TOC Review

### TOC Presence

| # | File | TOC Present? | Sections Used |
|---|---|:---:|---|
| 1 | Algorithm21_1.rs | No | imports, broadcast use, spec fns, proof fns, exec fns |
| 2 | Algorithm21_2.rs | No | imports, broadcast use, proof fns, exec fns |
| 3 | Algorithm21_5.rs | No | imports, broadcast use, type alias, exec fns |
| 4 | Algorithm21_6.rs | No | imports, broadcast use, exec fns |
| 5 | Exercise21_5.rs | No | imports, broadcast use, exec fns |
| 6 | Exercise21_6.rs | No | documentation only |
| 7 | Exercise21_7.rs | No | imports, broadcast use, spec fns, exec fns |
| 8 | Exercise21_8.rs | No | imports, broadcast use, spec fns, proof fns, exec fns |
| 9 | Exercise21_9.rs | No | placeholder |
| 10 | Problem21_1.rs | No | imports, broadcast use, exec fns |
| 11 | Problem21_3.rs | No | imports, broadcast use, exec fns |
| 12 | Problem21_4.rs | No | imports, broadcast use, exec fns, proof fns |

**Finding:** No files in Chapter 21 have TOC headers per the project standard.

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

✓ Algorithm21_1.rs  (2 clean proof functions)
✓ Algorithm21_2.rs  (1 clean proof function)
✓ Algorithm21_5.rs
✓ Algorithm21_6.rs
✓ Exercise21_5.rs
✓ Exercise21_6.rs
✓ Exercise21_7.rs
❌ Exercise21_8.rs   (2 × assume())
✓ Exercise21_9.rs
✓ Problem21_1.rs
✓ Problem21_3.rs
✓ Problem21_4.rs     (1 clean proof function)

SUMMARY: 11 clean modules, 1 holed module
Holes: 2 total (2 × assume())
Proof functions: 7 total (7 clean, 0 holed)
```

### Hole Details

| # | File | Line | Hole | Purpose |
|---|---|---|---|---|
| 1 | Exercise21_8.rs | 142 | `assume(1 < k as int + 1)` | Bridge: isqrt(n) >= 1 for n >= 2. Could be proved with isqrt properties. |
| 2 | Exercise21_8.rs | 153 | `assume(ones.seq@.len() == spec_divisor_count(...))` | Bridge: filter count equals divisor count. Requires stronger filter spec (multiset equality). |

---

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 13 |
| partial | 6 |
| weak | 1 |
| none | 2 |

| Strength | Functions |
|---|---|
| **strong** | `lemma_sum_inner_lens_mono`, `lemma_sum_inner_lens_uniform`, `lemma_flatten_uniform_len` (×2), `is_even`, `is_vowel`, `lemma_zero_count_means_no_divisors`, `lemma_no_divisors_means_zero_count`, `lemma_divisor_count_nonneg`, `is_divisible`, `is_prime`, `points2d`, `points3d_loops` |
| **partial** | `flatten_inner`, `points2d_tab_flat`, `points3d_tab_flat`, `primes_bf`, `cartesian_loops`, `cartesian_tab_flat` |
| **weak** | `prime_sieve` |
| **none** | `all_contiguous_subseqs`, `pair_even_with_vowels` |

---

## Overall Assessment

### Strengths

1. **Good coverage:** 8 of 8 prose algorithms/exercises have implementations (plus 3 bonus imperative variants).
2. **Strong proof infrastructure:** The isPrime proof in Exercise21_8.rs is sophisticated, with 3 clean proof lemmas establishing the connection between divisor count and primality.
3. **All code is inside verus!:** Every executable and proof function is properly inside `verus!` blocks.
4. **Imperative variants provide good spec contrast:** `points2d` and `points3d_loops` have strictly stronger specs than their functional counterparts, demonstrating what full verification looks like.

### Weaknesses

1. **Algorithm 21.6 (prime_sieve) has significant algorithmic divergence:** It uses linear membership testing instead of the ninject-based boolean sieve from the prose. This changes work from Θ(n lg n) to Θ(n² lg n). This is the most important fidelity gap in the chapter.
2. **Missing ensures on 2 functions:** `all_contiguous_subseqs` and `pair_even_with_vowels` have no ensures clauses.
3. **Functional variants have weaker specs than imperative variants:** `points2d_tab_flat` specifies only length while `points2d` also specifies element bounds. This pattern repeats for 3D points.
4. **No runtime tests at all:** Zero test coverage for the entire chapter.
5. **No TOC headers:** None of the 12 source files have the project-standard TOC.
6. **Proof holes in isPrime:** 2 `assume()` calls remain. The first could likely be closed with isqrt properties; the second requires a stronger filter specification.
7. **Duplicate lemma:** `lemma_flatten_uniform_len` is copy-pasted in two files.
8. **Exercise 21.9 is empty:** Just a placeholder with no Verus proof code.
9. **No Mt (parallel) variants:** The chapter emphasizes parallelism but all implementations are single-threaded.

### Priority Recommendations

| # | Priority | Action |
|---|---|---|
| 1 | High | Fix `prime_sieve` to use ninject-based boolean sieve per the prose algorithm |
| 2 | High | Add ensures to `all_contiguous_subseqs` and `pair_even_with_vowels` |
| 3 | High | Add runtime tests for all algorithms |
| 4 | Medium | Strengthen specs on functional variants (`points2d_tab_flat`, etc.) to include element bounds |
| 5 | Medium | Close proof hole 1 in isPrime (isqrt ≥ 1) |
| 6 | Medium | Deduplicate `lemma_flatten_uniform_len` into shared module |
| 7 | Low | Add TOC headers to all 12 files |
| 8 | Low | Implement Exercise 21.9 as a Verus proof |
| 9 | Low | Add `primes_bf` completeness to ensures (all primes < n are returned) |
| 10 | Low | Consider Mt variants for the tabulate+flatten algorithms |
