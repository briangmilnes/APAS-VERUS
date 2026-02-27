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
| 1 | Chap35 | OrderStatSelectMtEph | 1 | 1 | 0 | 2 | 3 | 0 | 2 | 1 | 0 |
| 2 | Chap35 | OrderStatSelectMtPer | 1 | 1 | 0 | 2 | 3 | 0 | 2 | 1 | 0 |
| 3 | Chap35 | OrderStatSelectStEph | 1 | 1 | 0 | 2 | 3 | 0 | 3 | 0 | 0 |
| 4 | Chap35 | OrderStatSelectStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |

## Function-by-Function Detail

### Chap35/OrderStatSelectMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `select` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;57 |
| 2 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | hole | 65&#8209;87 |
| 3 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 128&#8209;137 |

### Chap35/OrderStatSelectMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `select` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;57 |
| 5 | `parallel_three_way_partition` |  |  |  | Y | Y |  |  | hole | 63&#8209;85 |
| 6 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 126&#8209;135 |

### Chap35/OrderStatSelectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_total_ordering` |  |  |  | Y | Y |  |  | unknown | 60&#8209;61 |
| 8 | `select` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;95 |
| 9 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 113&#8209;120 |

### Chap35/OrderStatSelectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `select` | Y | Y |  |  | Y |  |  | unknown | 49&#8209;54 |
| 11 | `select_inner` |  |  |  | Y | Y |  |  | unknown | 72&#8209;79 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
