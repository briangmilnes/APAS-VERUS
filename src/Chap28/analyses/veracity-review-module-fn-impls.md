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
| 1 | `lemma_range_sum_snoc` |  |  |  | Y | Y |  |  | unknown | 144&#8209;151 |
| 2 | `lemma_range_sum_single` |  |  |  | Y | Y |  |  | unknown | 160&#8209;164 |
| 3 | `lemma_range_sum_empty` |  |  |  | Y | Y |  |  | unknown | 170&#8209;172 |
| 4 | `lemma_range_sum_split` |  |  |  | Y | Y |  |  | unknown | 177&#8209;182 |
| 5 | `lemma_range_sum_via_prefix` |  |  |  | Y | Y |  |  | unknown | 191&#8209;195 |
| 6 | `lemma_min_prefix_sum_is_min` |  |  |  | Y | Y |  |  | unknown | 201&#8209;206 |
| 7 | `lemma_min_prefix_sum_achieved` |  |  |  | Y | Y |  |  | unknown | 215&#8209;220 |
| 8 | `lemma_range_sum_subseq` |  |  |  | Y | Y |  |  | unknown | 237&#8209;245 |
| 9 | `lemma_crossing_decompose` |  |  |  | Y | Y |  |  | unknown | 256&#8209;260 |
| 10 | `lemma_sums_fit_subseq` |  |  |  | Y | Y |  |  | unknown | 266&#8209;273 |

### Chap28/MaxContigSubSumBruteStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `max_contig_sub_sum_brute` | Y | Y |  |  | Y |  |  | unknown | 53&#8209;59 |
| 12 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 69&#8209;71 |

### Chap28/MaxContigSubSumDivConMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `max_contig_sub_sum_divcon_mt` | Y | Y |  |  | Y |  |  | unknown | 48&#8209;55 |
| 14 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 61&#8209;62 |
| 15 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 72&#8209;74 |
| 16 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 122&#8209;124 |

### Chap28/MaxContigSubSumDivConOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `max_contig_sub_sum_divcon_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;61 |
| 18 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 19 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 78&#8209;86 |

### Chap28/MaxContigSubSumDivConOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_strength_combine` |  |  |  | Y | Y |  |  | unknown | 69&#8209;118 |
| 21 | `max_contig_sub_sum_divcon_opt` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;269 |
| 22 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 277&#8209;278 |
| 23 | `max_contig_sub_sum_aux` |  |  |  | Y | Y |  |  | unknown | 291&#8209;301 |

### Chap28/MaxContigSubSumDivConStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `lemma_divcon_combine` |  |  |  | Y | Y |  |  | unknown | 61&#8209;88 |
| 25 | `max_contig_sub_sum_divcon` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;160 |
| 26 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 168&#8209;169 |
| 27 | `max_suffix_sum` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 28 | `max_prefix_sum` |  |  |  | Y | Y |  |  | unknown | 260&#8209;265 |

### Chap28/MaxContigSubSumIterStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `lemma_max_ending_at_is_max` |  |  |  | Y | Y |  |  | unknown | 74&#8209;80 |
| 30 | `lemma_max_ending_at_achieved` |  |  |  | Y | Y |  |  | unknown | 97&#8209;104 |
| 31 | `max_contig_sub_sum_iter` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;139 |
| 32 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 147&#8209;148 |

### Chap28/MaxContigSubSumOptMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `max_contig_sub_sum_opt_mt` | Y | Y |  |  | Y |  |  | unknown | 22&#8209;29 |

### Chap28/MaxContigSubSumOptStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `lemma_prefix_opt_is_mcss` |  |  |  | Y | Y |  |  | unknown | 30&#8209;46 |
| 35 | `max_contig_sub_sum_opt` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;86 |

### Chap28/MaxContigSubSumReducedMcsseStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 36 | `max_contig_sub_sum_reduced_mcsse` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;58 |
| 37 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |

### Chap28/MaxContigSubSumReducedStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `max_contig_sub_sum_reduced` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;55 |
| 39 | `max_with_neginf` |  |  |  | Y | Y |  |  | unknown | 65&#8209;66 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
