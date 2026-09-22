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
| 1 | Chap35 | OrderStatSelectMtEph | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |
| 2 | Chap35 | OrderStatSelectMtPer | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |
| 3 | Chap35 | OrderStatSelectStEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 4 | Chap35 | OrderStatSelectStPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap35/OrderStatSelectMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 86&#8209;87 |
| 2 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 120&#8209;125 |
| 3 | `lemma_all_equal_multiset` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 4 | `select` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;165 |
| 5 | `append_vec` |  |  |  | Y | Y |  |  | unknown | 172&#8209;173 |
| 6 | `partition_three_dc` |  |  |  | Y | Y |  |  | unknown | 218&#8209;235 |
| 7 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 374&#8209;397 |
| 8 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 493&#8209;503 |

### Chap35/OrderStatSelectMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 86&#8209;87 |
| 10 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 120&#8209;125 |
| 11 | `lemma_all_equal_multiset` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 12 | `select` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;165 |
| 13 | `append_vec` |  |  |  | Y | Y |  |  | unknown | 172&#8209;173 |
| 14 | `partition_three_dc` |  |  |  | Y | Y |  |  | unknown | 218&#8209;235 |
| 15 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 374&#8209;397 |
| 16 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 491&#8209;501 |

### Chap35/OrderStatSelectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 18 | `select` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 19 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 134&#8209;141 |

### Chap35/OrderStatSelectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 67&#8209;68 |
| 21 | `select` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;112 |
| 22 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 134&#8209;141 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
