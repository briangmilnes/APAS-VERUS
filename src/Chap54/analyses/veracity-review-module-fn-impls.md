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
| 1 | Chap54 | BFSMtEph | 4 | 4 | 0 | 11 | 15 | 0 | 15 | 0 | 0 |
| 2 | Chap54 | BFSMtPer | 4 | 4 | 0 | 11 | 15 | 0 | 15 | 0 | 0 |
| 3 | Chap54 | BFSStEph | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |
| 4 | Chap54 | BFSStPer | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap54/BFSMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 2 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 3 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 96&#8209;107 |
| 4 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 117&#8209;123 |
| 5 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 127&#8209;144 |
| 6 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 159&#8209;164 |
| 7 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 177&#8209;185 |
| 8 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 210&#8209;223 |
| 9 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 235&#8209;246 |
| 10 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 261&#8209;266 |
| 11 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;275 |
| 12 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 281&#8209;294 |
| 13 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 298&#8209;316 |
| 14 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 322&#8209;344 |
| 15 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 480&#8209;498 |

### Chap54/BFSMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 57&#8209;62 |
| 17 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 65&#8209;82 |
| 18 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 96&#8209;107 |
| 19 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 117&#8209;123 |
| 20 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 127&#8209;144 |
| 21 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 159&#8209;164 |
| 22 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 177&#8209;185 |
| 23 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 211&#8209;224 |
| 24 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 237&#8209;248 |
| 25 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 263&#8209;268 |
| 26 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;277 |
| 27 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 283&#8209;296 |
| 28 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 300&#8209;318 |
| 29 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 325&#8209;347 |
| 30 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 492&#8209;510 |

### Chap54/BFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 32 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 33 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 98&#8209;104 |
| 34 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 109&#8209;126 |
| 35 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;157 |
| 36 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;179 |
| 37 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;189 |
| 38 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;196 |

### Chap54/BFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 40 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 67&#8209;84 |
| 41 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 98&#8209;104 |
| 42 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 108&#8209;125 |
| 43 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;156 |
| 44 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;178 |
| 45 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;188 |
| 46 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;195 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
