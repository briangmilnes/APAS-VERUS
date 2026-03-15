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
| 1 | Chap55 | CycleDetectStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 2 | Chap55 | CycleDetectStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 3 | Chap55 | DFSStEph | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 4 | Chap55 | DFSStPer | 1 | 1 | 0 | 1 | 2 | 0 | 2 | 0 | 0 |
| 5 | Chap55 | SCCStEph | 1 | 1 | 0 | 4 | 5 | 0 | 5 | 0 | 0 |
| 6 | Chap55 | SCCStPer | 1 | 1 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 7 | Chap55 | TopoSortStEph | 1 | 1 | 0 | 7 | 8 | 0 | 8 | 0 | 0 |
| 8 | Chap55 | TopoSortStPer | 1 | 1 | 0 | 3 | 4 | 0 | 4 | 0 | 0 |

## Function-by-Function Detail

### Chap55/CycleDetectStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 32&#8209;37 |
| 2 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 44&#8209;62 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;47 |
| 4 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 54&#8209;72 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;42 |
| 6 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 48&#8209;64 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 34&#8209;43 |
| 8 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 50&#8209;66 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `scc` | Y | Y |  |  | Y |  |  | unknown | 37&#8209;42 |
| 10 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 48&#8209;54 |
| 11 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 107&#8209;109 |
| 12 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 169&#8209;171 |
| 13 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 251&#8209;267 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `scc` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;44 |
| 15 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 50&#8209;74 |
| 16 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 116&#8209;122 |
| 17 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 186&#8209;188 |
| 18 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 246&#8209;248 |
| 19 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 291&#8209;307 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 118&#8209;124 |
| 21 | `lemma_set_true_num_false_eq` |  |  |  | Y | Y |  |  | unknown | 136&#8209;142 |
| 22 | `lemma_all_true_num_false_zero` |  |  |  | Y | Y |  |  | unknown | 154&#8209;157 |
| 23 | `lemma_all_false_num_false_eq_len` |  |  |  | Y | Y |  |  | unknown | 165&#8209;168 |
| 24 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;186 |
| 25 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 193&#8209;217 |
| 26 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 261&#8209;280 |
| 27 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 329&#8209;333 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;109 |
| 29 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 115&#8209;131 |
| 30 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 167&#8209;186 |
| 31 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 232&#8209;236 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
