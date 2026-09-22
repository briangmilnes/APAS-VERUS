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
| 1 | Chap28 | MCSSSpec | 0 | 0 | 0 | 10 | 10 | 0 | 10 | 0 | 0 |
| 2 | Chap28 | MaxContigSubSumBruteStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap28 | MaxContigSubSumDivConMtEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 4 | Chap28 | MaxContigSubSumDivConOptMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 5 | Chap28 | MaxContigSubSumDivConOptStEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 6 | Chap28 | MaxContigSubSumDivConStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 7 | Chap28 | MaxContigSubSumIterStEph | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |
| 8 | Chap28 | MaxContigSubSumOptMtEph | 1 | 1 | 0 | 0 | 1 | 0 | 1 | 0 | 0 |
| 9 | Chap28 | MaxContigSubSumOptStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 10 | Chap28 | MaxContigSubSumReducedMcsseStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 11 | Chap28 | MaxContigSubSumReducedStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |

## Function-by-Function Detail

### Chap28/MCSSSpec.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_range_sum_snoc` |  |  |  | Y | Y |  |  | unknown | 154&#8209;161 |
| 2 | `lemma_range_sum_single` |  |  |  | Y | Y |  |  | unknown | 170&#8209;174 |
| 3 | `lemma_range_sum_empty` |  |  |  | Y | Y |  |  | unknown | 180&#8209;182 |
| 4 | `lemma_range_sum_split` |  |  |  | Y | Y |  |  | unknown | 187&#8209;192 |
| 5 | `lemma_range_sum_via_prefix` |  |  |  | Y | Y |  |  | unknown | 201&#8209;205 |
| 6 | `lemma_min_prefix_sum_is_min` |  |  |  | Y | Y |  |  | unknown | 211&#8209;216 |
| 7 | `lemma_min_prefix_sum_achieved` |  |  |  | Y | Y |  |  | unknown | 225&#8209;230 |
| 8 | `lemma_range_sum_subseq` |  |  |  | Y | Y |  |  | unknown | 248&#8209;256 |
| 9 | `lemma_crossing_decompose` |  |  |  | Y | Y |  |  | unknown | 267&#8209;271 |
| 10 | `lemma_sums_fit_subseq` |  |  |  | Y | Y |  |  | unknown | 277&#8209;284 |

### Chap28/MaxContigSubSumBruteStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `max_contig_sub_sum_brute` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;66 |
| 12 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 77&#8209;79 |

### Chap28/MaxContigSubSumDivConMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `max_contig_sub_sum_divcon_mt` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;63 |
| 14 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 72&#8209;73 |
| 15 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 86&#8209;88 |
| 16 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 144&#8209;146 |

### Chap28/MaxContigSubSumDivConOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `max_contig_sub_sum_divcon_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;69 |
| 18 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 78&#8209;79 |
| 19 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 92&#8209;100 |

### Chap28/MaxContigSubSumDivConOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_strength_combine` |  |  |  | Y | Y |  |  | unknown | 79&#8209;128 |
| 21 | `max_contig_sub_sum_divcon_opt` | Y | Y |  |  | Y |  |  | unknown | 275&#8209;281 |
| 22 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 292&#8209;293 |
| 23 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 306&#8209;316 |

### Chap28/MaxContigSubSumDivConStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `lemma_divcon_combine` |  |  |  | Y | Y |  |  | unknown | 71&#8209;98 |
| 25 | `max_contig_sub_sum_divcon` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;169 |
| 26 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 180&#8209;181 |
| 27 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 196&#8209;201 |
| 28 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 277&#8209;282 |

### Chap28/MaxContigSubSumIterStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `lemma_max_ending_at_is_max` |  |  |  | Y | Y |  |  | unknown | 83&#8209;89 |
| 30 | `lemma_max_ending_at_achieved` |  |  |  | Y | Y |  |  | unknown | 106&#8209;113 |
| 31 | `max_contig_sub_sum_iter` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;147 |
| 32 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 156&#8209;157 |

### Chap28/MaxContigSubSumOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `max_contig_sub_sum_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 40&#8209;47 |

### Chap28/MaxContigSubSumOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `lemma_prefix_opt_is_mcss` |  |  |  | Y | Y |  |  | unknown | 52&#8209;68 |
| 35 | `max_contig_sub_sum_opt` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;112 |

### Chap28/MaxContigSubSumReducedMcsseStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 36 | `max_contig_sub_sum_reduced_mcsse` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;65 |
| 37 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 76&#8209;77 |

### Chap28/MaxContigSubSumReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `max_contig_sub_sum_reduced` | Y | Y |  |  | Y |  |  | unknown | 56&#8209;62 |
| 39 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 73&#8209;74 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
