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
| 1 | Chap55 | CycleDetectStEph | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 2 | Chap55 | CycleDetectStPer | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 3 | Chap55 | DFSSpecsAndLemmas | 0 | 0 | 0 | 10 | 10 | 0 | 10 | 0 | 0 |
| 4 | Chap55 | DFSStEph | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 5 | Chap55 | DFSStPer | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 6 | Chap55 | SCCStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 7 | Chap55 | SCCStPer | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 8 | Chap55 | TopoSortStEph | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |
| 9 | Chap55 | TopoSortStPer | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |

## Function-by-Function Detail

### Chap55/CycleDetectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_cycle_not_dag` |  |  |  | Y | Y |  |  | unknown | 100&#8209;116 |
| 2 | `lemma_path_ord_decreases` |  |  |  | Y | Y |  |  | unknown | 158&#8209;171 |
| 3 | `lemma_extract_ord` |  |  |  | Y | Y |  |  | unknown | 210&#8209;229 |
| 4 | `lemma_acyclic_ord_implies_dag` |  |  |  | Y | Y |  |  | unknown | 247&#8209;256 |
| 5 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 280&#8209;285 |
| 6 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 295&#8209;339 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_cycle_not_dag_per` |  |  |  | Y | Y |  |  | unknown | 101&#8209;115 |
| 8 | `lemma_extract_ord_per` |  |  |  | Y | Y |  |  | unknown | 152&#8209;171 |
| 9 | `lemma_path_ord_decreases_per` |  |  |  | Y | Y |  |  | unknown | 189&#8209;202 |
| 10 | `lemma_acyclic_ord_implies_dag_per` |  |  |  | Y | Y |  |  | unknown | 242&#8209;251 |
| 11 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;279 |
| 12 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 288&#8209;329 |

### Chap55/DFSSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 44&#8209;50 |
| 14 | `lemma_set_true_num_false_eq` |  |  |  | Y | Y |  |  | unknown | 76&#8209;82 |
| 15 | `lemma_all_true_num_false_zero` |  |  |  | Y | Y |  |  | unknown | 105&#8209;108 |
| 16 | `lemma_all_false_num_false_eq_len` |  |  |  | Y | Y |  |  | unknown | 116&#8209;119 |
| 17 | `lemma_bool_view_eq_spec_index` |  |  |  | Y | Y |  |  | unknown | 127&#8209;128 |
| 18 | `lemma_bool_array_set_view` |  |  |  | Y | Y |  |  | unknown | 136&#8209;149 |
| 19 | `lemma_usize_view_eq_spec_index` |  |  |  | Y | Y |  |  | unknown | 160&#8209;161 |
| 20 | `lemma_graph_view_bridge` |  |  |  | Y | Y |  |  | unknown | 168&#8209;177 |
| 21 | `lemma_usize_per_view_eq_spec_index` |  |  |  | Y | Y |  |  | unknown | 182&#8209;183 |
| 22 | `lemma_graph_per_view_bridge` |  |  |  | Y | Y |  |  | unknown | 189&#8209;198 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `lemma_reachable_self` |  |  |  | Y | Y |  |  | unknown | 55&#8209;57 |
| 24 | `lemma_reachable_step` |  |  |  | Y | Y |  |  | unknown | 71&#8209;80 |
| 25 | `lemma_neighbor_closed_path` |  |  |  | Y | Y |  |  | unknown | 119&#8209;135 |
| 26 | `lemma_neighbor_closed_implies_reachable` |  |  |  | Y | Y |  |  | unknown | 178&#8209;195 |
| 27 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;221 |
| 28 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 232&#8209;282 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 29 | `lemma_reachable_self_per` |  |  |  | Y | Y |  |  | unknown | 55&#8209;57 |
| 30 | `lemma_reachable_step_per` |  |  |  | Y | Y |  |  | unknown | 72&#8209;81 |
| 31 | `lemma_neighbor_closed_path_per` |  |  |  | Y | Y |  |  | unknown | 120&#8209;136 |
| 32 | `lemma_neighbor_closed_implies_reachable_per` |  |  |  | Y | Y |  |  | unknown | 180&#8209;197 |
| 33 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 214&#8209;224 |
| 34 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 234&#8209;284 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `scc` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;77 |
| 36 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 85&#8209;93 |
| 37 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 191&#8209;195 |
| 38 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 337&#8209;338 |
| 39 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 394&#8209;416 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 40 | `scc` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;74 |
| 41 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 82&#8209;106 |
| 42 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 187&#8209;195 |
| 43 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 282&#8209;286 |
| 44 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 427&#8209;428 |
| 45 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 488&#8209;509 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 46 | `lemma_edge_implies_reachable` |  |  |  | Y | Y |  |  | unknown | 168&#8209;177 |
| 47 | `lemma_self_reachable` |  |  |  | Y | Y |  |  | unknown | 200&#8209;205 |
| 48 | `lemma_reachable_via_edge` |  |  |  | Y | Y |  |  | unknown | 219&#8209;230 |
| 49 | `lemma_reachable_edge_contradicts_dag` |  |  |  | Y | Y |  |  | unknown | 269&#8209;279 |
| 50 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 331&#8209;338 |
| 51 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 347&#8209;410 |
| 52 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 691&#8209;713 |
| 53 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 791&#8209;797 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 54 | `lemma_edge_implies_reachable_per` |  |  |  | Y | Y |  |  | unknown | 159&#8209;164 |
| 55 | `lemma_self_reachable_per` |  |  |  | Y | Y |  |  | unknown | 184&#8209;189 |
| 56 | `lemma_reachable_via_edge_per` |  |  |  | Y | Y |  |  | unknown | 203&#8209;211 |
| 57 | `lemma_reachable_edge_contradicts_dag_per` |  |  |  | Y | Y |  |  | unknown | 242&#8209;250 |
| 58 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 290&#8209;297 |
| 59 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 305&#8209;368 |
| 60 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 641&#8209;662 |
| 61 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 738&#8209;745 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
