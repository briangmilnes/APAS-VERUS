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
| 1 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 3 | 0 | 2 | 0 | 1 |
| 2 | Chap65 | PrimStEph | 2 | 2 | 0 | 3 | 2 | 3 | 1 | 0 | 4 |
| 3 | Chap65 | UnionFindStEph | 6 | 6 | 0 | 0 | 6 | 0 | 6 | 0 | 0 |

## Function-by-Function Detail

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `kruskal_mst` | Y |  |  | Y | Y |  |  | unknown | 39&#8209;42 |
| 2 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 46 |
| 3 | `verify_mst_size` | Y |  |  | Y | Y |  |  | unknown | 50&#8209;54 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `prim_mst` | Y |  |  | Y | Y |  |  | unknown | 63&#8209;67 |
| 5 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 71 |
| 6 | `pq_entry_new` |  |  |  | Y |  | Y | Y |  | 76&#8209;86 |
| 7 | `cmp` |  | Y |  |  |  | Y | Y |  | 90&#8209;92 |
| 8 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 97&#8209;99 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `new` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;80 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;90 |
| 11 | `find` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 12 | `union` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;126 |
| 13 | `equals` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;147 |
| 14 | `num_sets` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;157 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
