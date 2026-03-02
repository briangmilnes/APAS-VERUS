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
| 1 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 3 | 0 | 0 | 0 | 3 |
| 2 | Chap65 | PrimStEph | 2 | 2 | 0 | 3 | 2 | 3 | 0 | 0 | 5 |
| 3 | Chap65 | UnionFindStEph | 6 | 6 | 0 | 0 | 6 | 0 | 1 | 5 | 0 |

## Function-by-Function Detail

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `kruskal_mst` | Y |  |  | Y | Y |  | Y |  | 26&#8209;28 |
| 2 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 32 |
| 3 | `verify_mst_size` | Y |  |  | Y | Y |  | Y |  | 36&#8209;39 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 4 | `prim_mst` | Y |  |  | Y | Y |  | Y |  | 43&#8209;46 |
| 5 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 50 |
| 6 | `pq_entry_new` |  |  |  | Y |  | Y | Y |  | 59&#8209;69 |
| 7 | `cmp` |  | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 8 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 80&#8209;82 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `new` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;111 |
| 10 | `insert` | Y | Y |  |  | Y |  |  | hole | 115&#8209;121 |
| 11 | `find` | Y | Y |  |  | Y |  |  | hole | 130&#8209;143 |
| 12 | `union` | Y | Y |  |  | Y |  |  | hole | 147&#8209;157 |
| 13 | `equals` | Y | Y |  |  | Y |  |  | hole | 169&#8209;178 |
| 14 | `num_sets` | Y | Y |  |  | Y |  |  | hole | 182&#8209;188 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
