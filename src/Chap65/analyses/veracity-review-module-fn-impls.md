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
| 1 | Chap65 | KruskalStEph | 3 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |
| 2 | Chap65 | PrimStEph | 2 | 2 | 0 | 3 | 3 | 2 | 3 | 0 | 2 |
| 3 | Chap65 | UnionFindStEph | 6 | 6 | 0 | 0 | 6 | 0 | 6 | 0 | 0 |

## Function-by-Function Detail

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `kruskal_mst` | Y |  |  | Y | Y |  |  | unknown | 42&#8209;45 |
| 2 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 49&#8209;50 |
| 3 | `verify_mst_size` | Y |  |  | Y | Y |  |  | unknown | 54&#8209;58 |
| 4 | `sort_edges_by_weight` |  |  |  | Y | Y |  |  | unknown | 62&#8209;70 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `prim_mst` | Y |  |  | Y | Y |  |  | unknown | 60&#8209;65 |
| 6 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 69&#8209;70 |
| 7 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 8 | `cmp` |  | Y |  |  |  | Y | Y |  | 332&#8209;334 |
| 9 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 338&#8209;340 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;80 |
| 11 | `insert` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;90 |
| 12 | `find` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 13 | `union` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;126 |
| 14 | `equals` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;147 |
| 15 | `num_sets` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;157 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
