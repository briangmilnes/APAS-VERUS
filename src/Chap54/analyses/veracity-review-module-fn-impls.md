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
| 1 | Chap54 | BFSMtEph | 4 | 2 | 0 | 13 | 15 | 0 | 15 | 0 | 0 |
| 2 | Chap54 | BFSMtPer | 4 | 2 | 0 | 13 | 15 | 0 | 15 | 0 | 0 |
| 3 | Chap54 | BFSStEph | 4 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |
| 4 | Chap54 | BFSStPer | 4 | 2 | 0 | 6 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap54/BFSMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 55&#8209;60 |
| 2 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 63&#8209;80 |
| 3 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 94&#8209;105 |
| 4 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 5 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;142 |
| 6 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 7 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 175&#8209;183 |
| 8 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 208&#8209;221 |
| 9 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 233&#8209;244 |
| 10 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 259&#8209;264 |
| 11 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 267&#8209;273 |
| 12 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 277&#8209;287 |
| 13 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 291&#8209;305 |
| 14 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 311&#8209;333 |
| 15 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 595&#8209;613 |

### Chap54/BFSMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 55&#8209;60 |
| 17 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 63&#8209;80 |
| 18 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 94&#8209;105 |
| 19 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 115&#8209;121 |
| 20 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 125&#8209;142 |
| 21 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 157&#8209;162 |
| 22 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 175&#8209;183 |
| 23 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 209&#8209;222 |
| 24 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 235&#8209;246 |
| 25 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 26 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 27 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 279&#8209;289 |
| 28 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 293&#8209;307 |
| 29 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 314&#8209;336 |
| 30 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 612&#8209;630 |

### Chap54/BFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 32 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 33 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 99&#8209;109 |
| 34 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 113&#8209;127 |
| 35 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;137 |
| 36 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 37 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 149&#8209;155 |
| 38 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 160&#8209;177 |

### Chap54/BFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 40 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 41 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 96&#8209;102 |
| 42 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 106&#8209;123 |
| 43 | `bfs` | Y |  |  | Y | Y |  |  | unknown | 140&#8209;150 |
| 44 | `bfs_tree` | Y |  |  | Y | Y |  |  | unknown | 154&#8209;168 |
| 45 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;178 |
| 46 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;185 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
