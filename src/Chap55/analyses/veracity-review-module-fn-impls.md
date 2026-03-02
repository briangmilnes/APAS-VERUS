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
| 1 | Chap55 | CycleDetectStEph | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 2 | Chap55 | CycleDetectStPer | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap55 | DFSStEph | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 4 | Chap55 | DFSStPer | 1 | 0 | 0 | 2 | 2 | 0 | 2 | 0 | 0 |
| 5 | Chap55 | SCCStEph | 1 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 6 | Chap55 | SCCStPer | 1 | 0 | 0 | 6 | 6 | 0 | 6 | 0 | 0 |
| 7 | Chap55 | TopoSortStEph | 1 | 0 | 0 | 5 | 5 | 0 | 5 | 0 | 0 |
| 8 | Chap55 | TopoSortStPer | 1 | 0 | 0 | 4 | 4 | 0 | 4 | 0 | 0 |

## Function-by-Function Detail

### Chap55/CycleDetectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `has_cycle` | Y |  |  | Y | Y |  |  | unknown | 28 |
| 2 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 35&#8209;53 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `has_cycle` | Y |  |  | Y | Y |  |  | unknown | 38 |
| 4 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 45&#8209;63 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `dfs` | Y |  |  | Y | Y |  |  | unknown | 21 |
| 6 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 25&#8209;41 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `dfs` | Y |  |  | Y | Y |  |  | unknown | 22 |
| 8 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 27&#8209;43 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `scc` | Y |  |  | Y | Y |  |  | unknown | 33&#8209;34 |
| 10 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 40&#8209;41 |
| 11 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 77&#8209;79 |
| 12 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 139&#8209;140 |
| 13 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 219&#8209;235 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `scc` | Y |  |  | Y | Y |  |  | unknown | 31&#8209;32 |
| 15 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 38&#8209;54 |
| 16 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 89&#8209;90 |
| 17 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 134&#8209;136 |
| 18 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 194&#8209;195 |
| 19 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 238&#8209;254 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 54&#8209;60 |
| 21 | `topo_sort` | Y |  |  | Y | Y |  |  | unknown | 76 |
| 22 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 83&#8209;99 |
| 23 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 136&#8209;155 |
| 24 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 204&#8209;205 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `topo_sort` | Y |  |  | Y | Y |  |  | unknown | 39 |
| 26 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 45&#8209;61 |
| 27 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 97&#8209;116 |
| 28 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 162&#8209;163 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
