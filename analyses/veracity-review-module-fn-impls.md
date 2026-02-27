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
| 1 | Chap35 | OrderStatSelectMtEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 2 | Chap35 | OrderStatSelectMtPer | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 3 | Chap35 | OrderStatSelectStEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 4 | Chap35 | OrderStatSelectStPer | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |

## Function-by-Function Detail

### Chap35/OrderStatSelectMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |
| 2 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 93&#8209;98 |
| 3 | `select` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 4 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 124&#8209;146 |
| 5 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 399&#8209;408 |

### Chap35/OrderStatSelectMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 6 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 68&#8209;69 |
| 7 | `lemma_const_seq_multiset` |  |  |  | Y | Y |  |  | unknown | 93&#8209;98 |
| 8 | `select` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 9 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | unknown | 124&#8209;146 |
| 10 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 399&#8209;408 |

### Chap35/OrderStatSelectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 11 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 60&#8209;61 |
| 12 | `select` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;95 |
| 13 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 113&#8209;120 |

### Chap35/OrderStatSelectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 59&#8209;60 |
| 15 | `select` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;94 |
| 16 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 112&#8209;119 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
