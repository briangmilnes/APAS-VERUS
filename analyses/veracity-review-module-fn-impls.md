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
| 1 | Chap58 | BellmanFordStEphFloat | 1 | 0 | 0 | 2 | 0 | 2 | 0 | 0 | 2 |
| 2 | Chap58 | BellmanFordStEphInt | 1 | 0 | 0 | 2 | 1 | 1 | 0 | 0 | 2 |
| 3 | Chap59 | JohnsonMtEphFloat | 1 | 0 | 0 | 5 | 0 | 5 | 0 | 0 | 5 |
| 4 | Chap59 | JohnsonMtEphInt | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 5 | Chap59 | JohnsonStEphFloat | 1 | 0 | 0 | 4 | 0 | 4 | 0 | 0 | 4 |
| 6 | Chap59 | JohnsonStEphInt | 1 | 0 | 0 | 4 | 1 | 3 | 0 | 0 | 4 |
| 7 | Chap61 | EdgeContractionMtEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 8 | Chap61 | EdgeContractionStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 9 | Chap61 | VertexMatchingMtEph | 1 | 0 | 0 | 5 | 1 | 4 | 0 | 0 | 5 |
| 10 | Chap61 | VertexMatchingStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 11 | Chap62 | StarContractionMtEph | 2 | 0 | 0 | 4 | 2 | 2 | 0 | 0 | 4 |
| 12 | Chap62 | StarContractionStEph | 2 | 0 | 0 | 3 | 2 | 1 | 0 | 0 | 3 |
| 13 | Chap62 | StarPartitionMtEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 14 | Chap62 | StarPartitionStEph | 1 | 0 | 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 15 | Chap63 | ConnectivityMtEph | 4 | 0 | 0 | 7 | 4 | 3 | 0 | 0 | 7 |
| 16 | Chap63 | ConnectivityStEph | 4 | 0 | 0 | 5 | 4 | 1 | 0 | 0 | 5 |
| 17 | Chap64 | SpanTreeMtEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 18 | Chap64 | SpanTreeStEph | 2 | 0 | 0 | 2 | 2 | 0 | 0 | 0 | 2 |
| 19 | Chap64 | TSPApproxStEph | 4 | 0 | 0 | 7 | 0 | 7 | 0 | 0 | 7 |
| 20 | Chap65 | KruskalStEph | 3 | 0 | 0 | 3 | 3 | 0 | 0 | 0 | 3 |
| 21 | Chap65 | PrimStEph | 2 | 2 | 0 | 5 | 2 | 5 | 0 | 0 | 7 |
| 22 | Chap65 | UnionFindStEph | 6 | 7 | 0 | 0 | 6 | 1 | 0 | 0 | 7 |
| 23 | Chap66 | BoruvkaMtEph | 5 | 0 | 0 | 7 | 5 | 2 | 0 | 0 | 7 |
| 24 | Chap66 | BoruvkaStEph | 5 | 0 | 0 | 5 | 5 | 0 | 0 | 0 | 5 |

## Function-by-Function Detail

### Chap58/BellmanFordStEphFloat.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `bellman_ford` | Y |  |  | Y |  | Y | Y |  | 24&#8209;27 |
| 2 | `reconstruct_predecessors` |  |  |  | Y |  | Y | Y |  | 130&#8209;164 |

### Chap58/BellmanFordStEphInt.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 3 | `bellman_ford` | Y |  |  | Y | Y |  | Y |  | 26&#8209;27 |
| 4 | `reconstruct_predecessors` |  |  |  | Y |  | Y | Y |  | 71&#8209;92 |

### Chap59/JohnsonMtEphFloat.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 5 | `johnson_apsp` | Y |  |  | Y |  | Y | Y |  | 31&#8209;33 |
| 6 | `parallel_dijkstra_all` |  |  |  | Y |  | Y | Y |  | 76&#8209;139 |
| 7 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 141&#8209;174 |
| 8 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 176&#8209;202 |
| 9 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 204&#8209;219 |

### Chap59/JohnsonMtEphInt.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 36 |
| 11 | `parallel_dijkstra_all` |  |  |  | Y |  | Y | Y |  | 78&#8209;137 |
| 12 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 139&#8209;163 |
| 13 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 165&#8209;189 |
| 14 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 191&#8209;204 |

### Chap59/JohnsonStEphFloat.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `johnson_apsp` | Y |  |  | Y |  | Y | Y |  | 28&#8209;30 |
| 16 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 87&#8209;111 |
| 17 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 113&#8209;134 |
| 18 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 136&#8209;148 |

### Chap59/JohnsonStEphInt.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `johnson_apsp` | Y |  |  | Y | Y |  | Y |  | 33 |
| 20 | `add_dummy_source` |  |  |  | Y |  | Y | Y |  | 99&#8209;125 |
| 21 | `reweight_graph` |  |  |  | Y |  | Y | Y |  | 127&#8209;151 |
| 22 | `create_negative_cycle_result` |  |  |  | Y |  | Y | Y |  | 153&#8209;166 |

### Chap61/EdgeContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `edge_contract_mt` | Y |  |  | Y | Y |  | Y |  | 32&#8209;35 |
| 24 | `contract_round_mt` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 25 | `build_edges_parallel` |  |  |  | Y |  | Y | Y |  | 109&#8209;162 |

### Chap61/EdgeContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `edge_contract` | Y |  |  | Y | Y |  | Y |  | 30&#8209;33 |
| 27 | `contract_round` | Y |  |  | Y | Y |  | Y |  | 37 |

### Chap61/VertexMatchingMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `parallel_matching_mt` | Y |  |  | Y | Y |  | Y |  | 30 |
| 29 | `flip_coins_parallel` |  |  |  | Y |  | Y | Y |  | 78&#8209;100 |
| 30 | `select_edges_parallel` |  |  |  | Y |  | Y | Y |  | 102&#8209;133 |
| 31 | `select_edges_recursive` |  |  |  | Y |  | Y | Y |  | 135&#8209;178 |
| 32 | `should_select_edge` |  |  |  | Y |  | Y | Y |  | 180&#8209;211 |

### Chap61/VertexMatchingStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 33 | `greedy_matching` | Y |  |  | Y | Y |  | Y |  | 26 |
| 34 | `parallel_matching_st` | Y |  |  | Y | Y |  | Y |  | 30 |

### Chap62/StarContractionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `star_contract_mt` | Y |  |  | Y | Y |  | Y |  | 32&#8209;37 |
| 36 | `contract_to_vertices_mt` | Y |  |  | Y | Y |  | Y |  | 41 |
| 37 | `build_quotient_graph_parallel` |  |  |  | Y |  | Y | Y |  | 85&#8209;106 |
| 38 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 108&#8209;158 |

### Chap62/StarContractionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 39 | `star_contract` | Y |  |  | Y | Y |  | Y |  | 27&#8209;31 |
| 40 | `contract_to_vertices` | Y |  |  | Y | Y |  | Y |  | 35 |
| 41 | `build_quotient_graph` |  |  |  | Y |  | Y | Y |  | 78&#8209;109 |

### Chap62/StarPartitionMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 42 | `parallel_star_partition` | Y |  |  | Y | Y |  | Y |  | 27&#8209;29 |

### Chap62/StarPartitionStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 43 | `sequential_star_partition` | Y |  |  | Y | Y |  | Y |  | 26 |

### Chap63/ConnectivityMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 44 | `count_components_mt` | Y |  |  | Y | Y |  | Y |  | 37 |
| 45 | `connected_components_mt` | Y |  |  | Y | Y |  | Y |  | 41&#8209;43 |
| 46 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 47 |
| 47 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 51&#8209;53 |
| 48 | `build_quotient_edges_parallel` |  |  |  | Y |  | Y | Y |  | 126&#8209;142 |
| 49 | `route_edges_parallel` |  |  |  | Y |  | Y | Y |  | 144&#8209;194 |
| 50 | `compose_maps_parallel` |  |  |  | Y |  | Y | Y |  | 196&#8209;211 |

### Chap63/ConnectivityStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 51 | `count_components` | Y |  |  | Y | Y |  | Y |  | 32 |
| 52 | `connected_components` | Y |  |  | Y | Y |  | Y |  | 36 |
| 53 | `count_components_hof` | Y |  |  | Y | Y |  | Y |  | 40 |
| 54 | `connected_components_hof` | Y |  |  | Y | Y |  | Y |  | 44 |
| 55 | `build_quotient_edges` |  |  |  | Y |  | Y | Y |  | 119&#8209;147 |

### Chap64/SpanTreeMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `spanning_tree_star_contraction_mt` | Y |  |  | Y | Y |  | Y |  | 33&#8209;35 |
| 57 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 39 |

### Chap64/SpanTreeStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `spanning_tree_star_contraction` | Y |  |  | Y | Y |  | Y |  | 27 |
| 59 | `verify_spanning_tree` | Y |  |  | Y | Y |  | Y |  | 31 |

### Chap64/TSPApproxStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 60 | `euler_tour` | Y |  |  | Y |  | Y | Y |  | 34&#8209;36 |
| 61 | `shortcut_tour` | Y |  |  | Y |  | Y | Y |  | 38&#8209;40 |
| 62 | `tour_weight` | Y |  |  | Y |  | Y | Y |  | 42&#8209;47 |
| 63 | `approx_metric_tsp` | Y |  |  | Y |  | Y | Y |  | 49&#8209;54 |
| 64 | `euler_tour_dfs` |  |  |  | Y |  | Y | Y |  | 86&#8209;137 |
| 65 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 202&#8209;216 |
| 66 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 218&#8209;233 |

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `kruskal_mst` | Y |  |  | Y | Y |  | Y |  | 26&#8209;28 |
| 68 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 32 |
| 69 | `verify_mst_size` | Y |  |  | Y | Y |  | Y |  | 36&#8209;39 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `prim_mst` | Y |  |  | Y | Y |  | Y |  | 43&#8209;46 |
| 71 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 50 |
| 72 | `pq_entry_new` |  |  |  | Y |  | Y | Y |  | 59&#8209;69 |
| 73 | `cmp` |  | Y |  |  |  | Y | Y |  | 73&#8209;75 |
| 74 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 80&#8209;82 |
| 75 | `get_neighbors` |  |  |  | Y |  | Y | Y |  | 155&#8209;169 |
| 76 | `get_edge_weight` |  |  |  | Y |  | Y | Y |  | 171&#8209;186 |

### Chap65/UnionFindStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 77 | `new` | Y | Y |  |  | Y |  | Y |  | 20 |
| 78 | `insert` | Y | Y |  |  | Y |  | Y |  | 24 |
| 79 | `find` | Y | Y |  |  | Y |  | Y |  | 28 |
| 80 | `union` | Y | Y |  |  | Y |  | Y |  | 32 |
| 81 | `equals` | Y | Y |  |  | Y |  | Y |  | 36 |
| 82 | `num_sets` | Y | Y |  |  | Y |  | Y |  | 40 |
| 83 | `default` |  | Y |  |  |  | Y | Y |  | 126&#8209;128 |

### Chap66/BoruvkaMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 84 | `vertex_bridges_mt` | Y |  |  | Y | Y |  | Y |  | 36&#8209;38 |
| 85 | `bridge_star_partition_mt` | Y |  |  | Y | Y |  | Y |  | 42&#8209;45 |
| 86 | `boruvka_mst_mt` | Y |  |  | Y | Y |  | Y |  | 49&#8209;51 |
| 87 | `boruvka_mst_mt_with_seed` | Y |  |  | Y | Y |  | Y |  | 55&#8209;58 |
| 88 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 62 |
| 89 | `filter_tail_to_head_mt` |  |  |  | Y |  | Y | Y |  | 171&#8209;222 |
| 90 | `reroute_edges_mt` |  |  |  | Y |  | Y | Y |  | 275&#8209;318 |

### Chap66/BoruvkaStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 91 | `vertex_bridges` | Y |  |  | Y | Y |  | Y |  | 35 |
| 92 | `bridge_star_partition` | Y |  |  | Y | Y |  | Y |  | 39&#8209;42 |
| 93 | `boruvka_mst` | Y |  |  | Y | Y |  | Y |  | 46 |
| 94 | `boruvka_mst_with_seed` | Y |  |  | Y | Y |  | Y |  | 50&#8209;53 |
| 95 | `mst_weight` | Y |  |  | Y | Y |  | Y |  | 57 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
