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
| 1 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 33&#8209;38 |
| 2 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 45&#8209;63 |

### Chap55/CycleDetectStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `has_cycle` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;47 |
| 4 | `dfs_check_cycle` |  |  |  | Y | Y |  |  | unknown | 54&#8209;72 |

### Chap55/DFSStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 34&#8209;43 |
| 6 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 49&#8209;65 |

### Chap55/DFSStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `dfs` | Y | Y |  |  | Y |  |  | unknown | 34&#8209;43 |
| 8 | `dfs_recursive` |  |  |  | Y | Y |  |  | unknown | 50&#8209;66 |

### Chap55/SCCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 9 | `scc` | Y | Y |  |  | Y |  |  | unknown | 38&#8209;43 |
| 10 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 49&#8209;55 |
| 11 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 108&#8209;110 |
| 12 | `check_wf_adj_list_eph` |  |  |  | Y | Y |  |  | unknown | 171&#8209;172 |
| 13 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 252&#8209;268 |

### Chap55/SCCStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 14 | `scc` | Y | Y |  |  | Y |  |  | unknown | 39&#8209;44 |
| 15 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 50&#8209;74 |
| 16 | `compute_finish_order` |  |  |  | Y | Y |  |  | unknown | 116&#8209;122 |
| 17 | `transpose_graph` |  |  |  | Y | Y |  |  | unknown | 186&#8209;188 |
| 18 | `check_wf_adj_list_per` |  |  |  | Y | Y |  |  | unknown | 247&#8209;248 |
| 19 | `dfs_reach` |  |  |  | Y | Y |  |  | unknown | 291&#8209;307 |

### Chap55/TopoSortStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 20 | `lemma_set_true_decreases_num_false` |  |  |  | Y | Y |  |  | unknown | 119&#8209;125 |
| 21 | `lemma_set_true_num_false_eq` |  |  |  | Y | Y |  |  | unknown | 137&#8209;143 |
| 22 | `lemma_all_true_num_false_zero` |  |  |  | Y | Y |  |  | unknown | 155&#8209;158 |
| 23 | `lemma_all_false_num_false_eq_len` |  |  |  | Y | Y |  |  | unknown | 166&#8209;169 |
| 24 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;188 |
| 25 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 195&#8209;219 |
| 26 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 263&#8209;282 |
| 27 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 331&#8209;335 |

### Chap55/TopoSortStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `topo_sort` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;110 |
| 29 | `dfs_finish_order` |  |  |  | Y | Y |  |  | unknown | 116&#8209;132 |
| 30 | `dfs_finish_order_cycle_detect` |  |  |  | Y | Y |  |  | unknown | 168&#8209;187 |
| 31 | `topological_sort_opt` |  |  |  | Y | Y |  |  | unknown | 233&#8209;237 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
