<style>
  body { max-width: 98%; margin: auto; font-size: 16px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 4px 8px; }
</style>

# Module Function Implementations Review

## Specification Summary by Module

| Abbr | Meaning |
|------|---------|
| Tr | declared in a `trait` block |
| IT | in `impl Trait for Type` |
| IBI | in bare `impl Type` |
| ML | module-level free fn |
| V! | inside `verus!` macro |
| -V! | outside `verus!` macro |
| Unk | has requires/ensures (strength not assessed) |
| Hole | contains `assume()`, `admit()`, or `#[verifier::external_body]` |
| NoSpec | no spec |

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap21 | Algorithm21_1 | 0 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 2 | Chap21 | Algorithm21_2 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 3 | Chap21 | Algorithm21_5 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 4 | Chap21 | Algorithm21_6 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 5 | Chap21 | Exercise21_5 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 6 | Chap21 | Exercise21_7 | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 7 | Chap21 | Exercise21_8 | 0 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 8 | Chap21 | Exercise21_9 | 0 | 0 | 0 | 3 | 3 | 0 | 3 | 0 | 0 |
| 9 | Chap21 | Problem21_1 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 10 | Chap21 | Problem21_3 | 0 | 0 | 0 | 1 | 1 | 0 | 1 | 0 | 0 |
| 11 | Chap21 | Problem21_4 | 0 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |

## Function-by-Function Detail

### Chap21/Algorithm21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_sum_inner_lens_mono` |  |  |  | Y | Y |  |  | unknown | 58&#8209;61 |
| 2 | `lemma_sum_inner_lens_uniform` |  |  |  | Y | Y |  |  | unknown | 70&#8209;76 |
| 3 | `flatten_inner` |  |  |  | Y | Y |  |  | unknown | 98&#8209;102 |
| 4 | `points2d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 162&#8209;168 |

### Chap21/Algorithm21_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `points3d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 53&#8209;60 |

### Chap21/Algorithm21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `primes_bf` |  |  |  | Y | Y |  |  | unknown | 54&#8209;62 |

### Chap21/Algorithm21_6.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_product_not_prime` |  |  |  | Y | Y |  |  | unknown | 44&#8209;46 |
| 8 | `prime_sieve` |  |  |  | Y | Y |  |  | unknown | 77&#8209;83 |

### Chap21/Exercise21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `lemma_inner_lens_sum_triangular` |  |  |  | Y | Y |  |  | unknown | 50&#8209;57 |
| 10 | `all_contiguous_subseqs` |  |  |  | Y | Y |  |  | unknown | 82&#8209;86 |

### Chap21/Exercise21_7.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `is_even` |  |  |  | Y | Y |  |  | unknown | 62&#8209;63 |
| 12 | `is_vowel` |  |  |  | Y | Y |  |  | unknown | 70&#8209;71 |
| 13 | `pair_even_with_vowels` |  |  |  | Y | Y |  |  | unknown | 85&#8209;94 |

### Chap21/Exercise21_8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_zero_count_means_no_divisors` |  |  |  | Y | Y |  |  | unknown | 61&#8209;68 |
| 15 | `lemma_no_divisors_means_zero_count` |  |  |  | Y | Y |  |  | unknown | 84&#8209;91 |
| 16 | `lemma_divisor_count_nonneg` |  |  |  | Y | Y |  |  | unknown | 100&#8209;102 |
| 17 | `lemma_filter_len_eq_divisor_count` |  |  |  | Y | Y |  |  | unknown | 110&#8209;116 |
| 18 | `lemma_divisor_count_split_last` |  |  |  | Y | Y |  |  | unknown | 137&#8209;142 |
| 19 | `is_divisible` |  |  |  | Y | Y |  |  | unknown | 168&#8209;170 |
| 20 | `is_prime` |  |  |  | Y | Y |  |  | unknown | 181&#8209;182 |

### Chap21/Exercise21_9.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 21 | `lemma_div_exact` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 22 | `lemma_composite_has_small_divisor` |  |  |  | Y | Y |  |  | unknown | 46&#8209;51 |
| 23 | `lemma_composites_covered_by_small_multiples` |  |  |  | Y | Y |  |  | unknown | 89&#8209;96 |

### Chap21/Problem21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `points2d` |  |  |  | Y | Y |  |  | unknown | 35&#8209;44 |

### Chap21/Problem21_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `points3d_loops` |  |  |  | Y | Y |  |  | unknown | 39&#8209;50 |

### Chap21/Problem21_4.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `cartesian_loops` |  |  |  | Y | Y |  |  | unknown | 42&#8209;49 |
| 27 | `cartesian_tab_flat` |  |  |  | Y | Y |  |  | unknown | 98&#8209;109 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
