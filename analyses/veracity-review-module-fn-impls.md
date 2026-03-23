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
| 1 | Chap66 | BoruvkaMtEph | 5 | 4 | 0 | 12 | 13 | 3 | 11 | 2 | 3 |
| 2 | Chap66 | BoruvkaStEph | 5 | 9 | 0 | 1 | 7 | 3 | 5 | 1 | 4 |

## Function-by-Function Detail

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `eq` |  | Y |  |  | Y |  |  | hole | 47&#8209;48 |
| 2 | `vertex_bridges_mt` | Y |  |  | Y | Y |  |  | unknown | 84&#8209;92 |
| 3 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  |  | unknown | 96&#8209;105 |
| 4 | `boruvka_mst_mt` | Y |  |  | Y | Y |  |  | unknown | 109&#8209;120 |
| 5 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  |  | unknown | 124&#8209;135 |
| 6 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 139&#8209;142 |
| 7 | `hash_coin` |  |  |  | Y | Y |  |  | hole | 150 |
| 8 | `hash_coin_flips_mt` |  |  |  | Y | Y |  |  | unknown | 164&#8209;174 |
| 9 | `compute_remaining_mt` |  |  |  | Y | Y |  |  | unknown | 232&#8209;238 |
| 10 | `collect_mst_labels_mt` |  |  |  | Y | Y |  |  | unknown | 291&#8209;297 |
| 11 | `build_partition_map_mt` |  |  |  | Y | Y |  |  | unknown | 350&#8209;359 |
| 12 | `filter_tail_to_head_mt` |  |  |  | Y | Y |  |  | unknown | 578&#8209;588 |
| 13 | `reroute_edges_mt` |  |  |  | Y | Y |  |  | unknown | 792&#8209;801 |
| 14 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 965&#8209;967 |
| 15 | `cmp` |  | Y |  |  |  | Y | Y |  | 971&#8209;976 |
| 16 | `hash` |  | Y |  |  |  | Y | Y |  | 980&#8209;985 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `eq` |  | Y |  |  | Y |  |  | hole | 62&#8209;63 |
| 18 | `coin_flip` |  |  |  | Y | Y |  |  | unknown | 100&#8209;101 |
| 19 | `vertex_bridges` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;126 |
| 20 | `bridge_star_partition` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;141 |
| 21 | `boruvka_mst` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;157 |
| 22 | `boruvka_mst_with_seed` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;172 |
| 23 | `mst_weight` | Y | Y |  |  | Y |  | Y |  | 176&#8209;179 |
| 24 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 504&#8209;506 |
| 25 | `cmp` |  | Y |  |  |  | Y | Y |  | 510&#8209;515 |
| 26 | `hash` |  | Y |  |  |  | Y | Y |  | 519&#8209;524 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
