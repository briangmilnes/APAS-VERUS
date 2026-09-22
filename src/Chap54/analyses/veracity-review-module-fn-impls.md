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
| 3 | Chap54 | BFSSpecsAndLemmas | 0 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 4 | Chap54 | BFSStEph | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |
| 5 | Chap54 | BFSStPer | 4 | 4 | 0 | 4 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap54/BFSMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 270&#8209;275 |
| 2 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 279&#8209;285 |
| 3 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 112&#8209;117 |
| 4 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 124&#8209;141 |
| 5 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 150&#8209;161 |
| 6 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 170&#8209;176 |
| 7 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 183&#8209;200 |
| 8 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 209&#8209;222 |
| 9 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 242&#8209;253 |
| 10 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;305 |
| 11 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 310&#8209;328 |
| 12 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 336&#8209;341 |
| 13 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 355&#8209;363 |
| 14 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 391&#8209;413 |
| 15 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 554&#8209;572 |

### Chap54/BFSMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 16 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 271&#8209;276 |
| 17 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;286 |
| 18 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 111&#8209;116 |
| 19 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 123&#8209;140 |
| 20 | `lemma_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 149&#8209;160 |
| 21 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 169&#8209;175 |
| 22 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 182&#8209;199 |
| 23 | `lemma_copy_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 209&#8209;222 |
| 24 | `lemma_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 243&#8209;254 |
| 25 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 293&#8209;306 |
| 26 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 311&#8209;329 |
| 27 | `copy_distances` |  |  |  | Y | Y |  |  | unknown | 337&#8209;342 |
| 28 | `copy_graph` |  |  |  | Y | Y |  |  | unknown | 356&#8209;364 |
| 29 | `process_frontier_parallel` |  |  |  | Y | Y |  |  | unknown | 393&#8209;415 |
| 30 | `process_frontier_tree_parallel` |  |  |  | Y | Y |  |  | unknown | 566&#8209;584 |

### Chap54/BFSSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 31 | `lemma_bfs_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 58&#8209;63 |
| 32 | `lemma_bfs_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 67&#8209;72 |
| 33 | `lemma_bfs_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 76&#8209;93 |
| 34 | `lemma_bfs_copy_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 105&#8209;116 |
| 35 | `lemma_bfs_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 126&#8209;143 |
| 36 | `lemma_bfs_copy_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 155&#8209;166 |

### Chap54/BFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 37 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 38 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 39 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 40 | `lemma_set_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 148&#8209;165 |
| 41 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 174&#8209;180 |
| 42 | `lemma_set_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 188&#8209;205 |
| 43 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 221&#8209;234 |
| 44 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 239&#8209;257 |

### Chap54/BFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `top_down_order` | Y | Y |  |  | Y |  |  | unknown | 63&#8209;67 |
| 46 | `bottom_up_order` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 47 | `lemma_tabulate_all_no_parent` |  |  |  | Y | Y |  |  | unknown | 136&#8209;141 |
| 48 | `lemma_update_preserves_parents_bounded` |  |  |  | Y | Y |  |  | unknown | 148&#8209;165 |
| 49 | `lemma_tabulate_all_unreachable` |  |  |  | Y | Y |  |  | unknown | 174&#8209;180 |
| 50 | `lemma_update_preserves_bounded` |  |  |  | Y | Y |  |  | unknown | 187&#8209;204 |
| 51 | `bfs` | Y | Y |  |  | Y |  |  | unknown | 220&#8209;233 |
| 52 | `bfs_tree` | Y | Y |  |  | Y |  |  | unknown | 238&#8209;257 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
