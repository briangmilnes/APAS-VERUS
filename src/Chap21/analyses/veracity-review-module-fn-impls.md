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
| 1 | `lemma_sum_inner_lens_mono` |  |  |  | Y | Y |  |  | unknown | 52&#8209;55 |
| 2 | `lemma_sum_inner_lens_uniform` |  |  |  | Y | Y |  |  | unknown | 65&#8209;71 |
| 3 | `flatten_inner` |  |  |  | Y | Y |  |  | unknown | 92&#8209;96 |
| 4 | `points2d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 152&#8209;158 |

### Chap21/Algorithm21_2.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `points3d_tab_flat` |  |  |  | Y | Y |  |  | unknown | 46&#8209;53 |

### Chap21/Algorithm21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `primes_bf` |  |  |  | Y | Y |  |  | unknown | 46&#8209;54 |

### Chap21/Algorithm21_6.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_product_not_prime` |  |  |  | Y | Y |  |  | unknown | 37&#8209;39 |
| 8 | `prime_sieve` |  |  |  | Y | Y |  |  | unknown | 62&#8209;68 |

### Chap21/Exercise21_5.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `lemma_inner_lens_sum_triangular` |  |  |  | Y | Y |  |  | unknown | 44&#8209;51 |
| 10 | `all_contiguous_subseqs` |  |  |  | Y | Y |  |  | unknown | 72&#8209;76 |

### Chap21/Exercise21_7.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `is_even` |  |  |  | Y | Y |  |  | unknown | 47&#8209;48 |
| 12 | `is_vowel` |  |  |  | Y | Y |  |  | unknown | 60&#8209;61 |
| 13 | `pair_even_with_vowels` |  |  |  | Y | Y |  |  | unknown | 75&#8209;84 |

### Chap21/Exercise21_8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_zero_count_means_no_divisors` |  |  |  | Y | Y |  |  | unknown | 55&#8209;62 |
| 15 | `lemma_no_divisors_means_zero_count` |  |  |  | Y | Y |  |  | unknown | 81&#8209;88 |
| 16 | `lemma_divisor_count_nonneg` |  |  |  | Y | Y |  |  | unknown | 99&#8209;101 |
| 17 | `lemma_filter_len_eq_divisor_count` |  |  |  | Y | Y |  |  | unknown | 109&#8209;115 |
| 18 | `lemma_divisor_count_split_last` |  |  |  | Y | Y |  |  | unknown | 135&#8209;140 |
| 19 | `is_divisible` |  |  |  | Y | Y |  |  | unknown | 164&#8209;166 |
| 20 | `is_prime` |  |  |  | Y | Y |  |  | unknown | 176&#8209;177 |

### Chap21/Exercise21_9.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 21 | `lemma_div_exact` |  |  |  | Y | Y |  |  | unknown | 28&#8209;30 |
| 22 | `lemma_composite_has_small_divisor` |  |  |  | Y | Y |  |  | unknown | 38&#8209;43 |
| 23 | `lemma_composites_covered_by_small_multiples` |  |  |  | Y | Y |  |  | unknown | 75&#8209;82 |

### Chap21/Problem21_1.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `points2d` |  |  |  | Y | Y |  |  | unknown | 34&#8209;43 |

### Chap21/Problem21_3.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `points3d_loops` |  |  |  | Y | Y |  |  | unknown | 37&#8209;48 |

### Chap21/Problem21_4.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `cartesian_loops` |  |  |  | Y | Y |  |  | unknown | 40&#8209;47 |
| 27 | `cartesian_tab_flat` |  |  |  | Y | Y |  |  | unknown | 92&#8209;103 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
