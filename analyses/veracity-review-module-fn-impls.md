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
| 1 | Chap66 | BoruvkaMtEph | 5 | 4 | 0 | 12 | 6 | 10 | 2 | 0 | 14 |
| 2 | Chap66 | BoruvkaStEph | 5 | 9 | 0 | 1 | 7 | 3 | 6 | 0 | 4 |

## Function-by-Function Detail

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `eq` |  | Y |  |  | Y |  |  | unknown | 48&#8209;49 |
| 2 | `vertex_bridges_mt` | Y |  |  | Y | Y |  | Y |  | 85&#8209;89 |
| 3 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  | Y |  | 93&#8209;98 |
| 4 | `boruvka_mst_mt` | Y |  |  | Y | Y |  | Y |  | 102&#8209;108 |
| 5 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  |  | unknown | 112&#8209;117 |
| 6 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 121&#8209;124 |
| 7 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 131&#8209;133 |
| 8 | `cmp` |  | Y |  |  |  | Y | Y |  | 137&#8209;142 |
| 9 | `hash` |  | Y |  |  |  | Y | Y |  | 146&#8209;151 |
| 10 | `hash_coin` |  |  |  | Y |  | Y | Y |  | 166&#8209;177 |
| 11 | `hash_coin_flips_mt` |  |  |  | Y |  | Y | Y |  | 179&#8209;212 |
| 12 | `compute_remaining_mt` |  |  |  | Y |  | Y | Y |  | 214&#8209;250 |
| 13 | `collect_mst_labels_mt` |  |  |  | Y |  | Y | Y |  | 252&#8209;287 |
| 14 | `build_partition_map_mt` |  |  |  | Y |  | Y | Y |  | 289&#8209;329 |
| 15 | `filter_tail_to_head_mt` |  |  |  | Y |  | Y | Y |  | 430&#8209;480 |
| 16 | `reroute_edges_mt` |  |  |  | Y |  | Y | Y |  | 545&#8209;587 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `eq` |  | Y |  |  | Y |  |  | unknown | 63&#8209;64 |
| 18 | `coin_flip` |  |  |  | Y | Y |  |  | unknown | 100&#8209;102 |
| 19 | `vertex_bridges` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;127 |
| 20 | `bridge_star_partition` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;142 |
| 21 | `boruvka_mst` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;158 |
| 22 | `boruvka_mst_with_seed` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;173 |
| 23 | `mst_weight` | Y | Y |  |  | Y |  | Y |  | 177&#8209;180 |
| 24 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 505&#8209;507 |
| 25 | `cmp` |  | Y |  |  |  | Y | Y |  | 511&#8209;516 |
| 26 | `hash` |  | Y |  |  |  | Y | Y |  | 520&#8209;525 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
