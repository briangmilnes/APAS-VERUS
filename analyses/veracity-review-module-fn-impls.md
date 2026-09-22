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
| 1 | Chap66 | BoruvkaMtEph | 5 | 4 | 0 | 12 | 13 | 3 | 12 | 1 | 3 |
| 2 | Chap66 | BoruvkaStEph | 5 | 9 | 0 | 1 | 7 | 3 | 6 | 1 | 3 |

## Function-by-Function Detail

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `vertex_bridges_mt` | Y |  |  | Y | Y |  |  | unknown | 81&#8209;91 |
| 2 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  |  | unknown | 96&#8209;105 |
| 3 | `boruvka_mst_mt` | Y |  |  | Y | Y |  |  | unknown | 111&#8209;123 |
| 4 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  |  | unknown | 129&#8209;141 |
| 5 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 146&#8209;152 |
| 6 | `hash_coin` |  |  |  | Y | Y |  |  | hole | 192 |
| 7 | `hash_coin_flips_mt` |  |  |  | Y | Y |  |  | unknown | 206&#8209;218 |
| 8 | `compute_remaining_mt` |  |  |  | Y | Y |  |  | unknown | 268&#8209;276 |
| 9 | `collect_mst_labels_mt` |  |  |  | Y | Y |  |  | unknown | 336&#8209;344 |
| 10 | `build_partition_map_mt` |  |  |  | Y | Y |  |  | unknown | 404&#8209;415 |
| 11 | `filter_tail_to_head_mt` |  |  |  | Y | Y |  |  | unknown | 641&#8209;653 |
| 12 | `reroute_edges_mt` |  |  |  | Y | Y |  |  | unknown | 835&#8209;847 |
| 13 | `eq` |  | Y |  |  | Y |  |  | unknown | 1013&#8209;1014 |
| 14 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 1044&#8209;1046 |
| 15 | `cmp` |  | Y |  |  |  | Y | Y |  | 1049&#8209;1054 |
| 16 | `hash` |  | Y |  |  |  | Y | Y |  | 1057&#8209;1062 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `coin_flip` |  |  |  | Y | Y |  |  | unknown | 101&#8209;102 |
| 18 | `vertex_bridges` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;133 |
| 19 | `bridge_star_partition` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;147 |
| 20 | `boruvka_mst` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;168 |
| 21 | `boruvka_mst_with_seed` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;189 |
| 22 | `mst_weight` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;200 |
| 23 | `eq` |  | Y |  |  | Y |  |  | hole | 490&#8209;491 |
| 24 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 511&#8209;513 |
| 25 | `cmp` |  | Y |  |  |  | Y | Y |  | 516&#8209;521 |
| 26 | `hash` |  | Y |  |  |  | Y | Y |  | 524&#8209;529 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
