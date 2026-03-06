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
| 1 | Chap66 | BoruvkaMtEph | 5 | 4 | 0 | 12 | 6 | 10 | 1 | 0 | 15 |
| 2 | Chap66 | BoruvkaStEph | 5 | 9 | 0 | 0 | 6 | 3 | 2 | 3 | 4 |

## Function-by-Function Detail

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `eq` |  | Y |  |  | Y |  |  | unknown | 42&#8209;43 |
| 2 | `vertex_bridges_mt` | Y |  |  | Y | Y |  | Y |  | 56&#8209;60 |
| 3 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  | Y |  | 64&#8209;69 |
| 4 | `boruvka_mst_mt` | Y |  |  | Y | Y |  | Y |  | 73&#8209;79 |
| 5 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  | Y |  | 83&#8209;87 |
| 6 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 91&#8209;94 |
| 7 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 105&#8209;107 |
| 8 | `cmp` |  | Y |  |  |  | Y | Y |  | 111&#8209;116 |
| 9 | `hash` |  | Y |  |  |  | Y | Y |  | 120&#8209;125 |
| 10 | `hash_coin` |  |  |  | Y |  | Y | Y |  | 140&#8209;151 |
| 11 | `hash_coin_flips_mt` |  |  |  | Y |  | Y | Y |  | 153&#8209;186 |
| 12 | `compute_remaining_mt` |  |  |  | Y |  | Y | Y |  | 188&#8209;224 |
| 13 | `collect_mst_labels_mt` |  |  |  | Y |  | Y | Y |  | 226&#8209;261 |
| 14 | `build_partition_map_mt` |  |  |  | Y |  | Y | Y |  | 263&#8209;303 |
| 15 | `filter_tail_to_head_mt` |  |  |  | Y |  | Y | Y |  | 404&#8209;454 |
| 16 | `reroute_edges_mt` |  |  |  | Y |  | Y | Y |  | 519&#8209;561 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `eq` |  | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 18 | `vertex_bridges` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;114 |
| 19 | `bridge_star_partition` | Y | Y |  |  | Y |  |  | hole | 121&#8209;125 |
| 20 | `boruvka_mst` | Y | Y |  |  | Y |  |  | hole | 129&#8209;134 |
| 21 | `boruvka_mst_with_seed` | Y | Y |  |  | Y |  |  | hole | 138&#8209;142 |
| 22 | `mst_weight` | Y | Y |  |  | Y |  | Y |  | 146&#8209;149 |
| 23 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 369&#8209;371 |
| 24 | `cmp` |  | Y |  |  |  | Y | Y |  | 375&#8209;380 |
| 25 | `hash` |  | Y |  |  |  | Y | Y |  | 384&#8209;389 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
