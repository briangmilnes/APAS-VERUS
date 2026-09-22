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
| 1 | Chap65 | KruskalStEph | 3 | 0 | 0 | 7 | 7 | 0 | 7 | 0 | 0 |
| 2 | Chap65 | PrimStEph | 2 | 6 | 0 | 3 | 8 | 1 | 3 | 0 | 6 |
| 3 | Chap65 | UnionFindArrayStEph | 5 | 5 | 0 | 12 | 17 | 0 | 17 | 0 | 0 |
| 4 | Chap65 | UnionFindNoPCStEph | 6 | 6 | 0 | 7 | 13 | 0 | 13 | 0 | 0 |
| 5 | Chap65 | UnionFindPCStEph | 7 | 7 | 0 | 20 | 27 | 0 | 27 | 0 | 0 |

## Function-by-Function Detail

### Chap65/KruskalStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_sorted_edge_in_graph_v` |  |  |  | Y | Y |  |  | unknown | 63&#8209;83 |
| 2 | `kruskal_mst` | Y |  |  | Y | Y |  |  | unknown | 110&#8209;113 |
| 3 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 117&#8209;119 |
| 4 | `verify_mst_size` | Y |  |  | Y | Y |  |  | unknown | 123&#8209;127 |
| 5 | `kruskal_process_edge` |  |  |  | Y | Y |  |  | unknown | 133&#8209;146 |
| 6 | `kruskal_greedy_phase` |  |  |  | Y | Y |  |  | unknown | 157&#8209;184 |
| 7 | `sort_edges_by_weight` |  |  |  | Y | Y |  |  | unknown | 223&#8209;230 |

### Chap65/PrimStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 8 | `prim_mst` | Y |  |  | Y | Y |  |  | unknown | 92&#8209;97 |
| 9 | `mst_weight` | Y |  |  | Y | Y |  |  | unknown | 102&#8209;104 |
| 10 | `reflexive` |  | Y |  |  | Y |  | Y |  | 154 |
| 11 | `transitive` |  | Y |  |  | Y |  | Y |  | 162 |
| 12 | `antisymmetric` |  | Y |  |  | Y |  | Y |  | 180 |
| 13 | `total` |  | Y |  |  | Y |  | Y |  | 190 |
| 14 | `cmp` x2 |  | Y |  |  | Y |  | Y |  | 203 |
| 15 | `pq_entry_new` |  |  |  | Y | Y |  |  | unknown | 251&#8209;252 |
| 16 | `partial_cmp` |  | Y |  |  |  | Y | Y |  | 526&#8209;527 |

### Chap65/UnionFindArrayStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 17 | `lemma_count_above_mono` |  |  |  | Y | Y |  |  | unknown | 115&#8209;118 |
| 18 | `lemma_count_above_strict` |  |  |  | Y | Y |  |  | unknown | 121&#8209;124 |
| 19 | `lemma_pure_find_in_bounds` |  |  |  | Y | Y |  |  | unknown | 130&#8209;138 |
| 20 | `lemma_pure_find_is_root` |  |  |  | Y | Y |  |  | unknown | 146&#8209;155 |
| 21 | `lemma_find_after_link` |  |  |  | Y | Y |  |  | unknown | 163&#8209;186 |
| 22 | `lemma_map_update` |  |  |  | Y | Y |  |  | unknown | 201&#8209;204 |
| 23 | `lemma_root_counted_at` |  |  |  | Y | Y |  |  | unknown | 211&#8209;222 |
| 24 | `lemma_count_disjoint` |  |  |  | Y | Y |  |  | unknown | 225&#8209;236 |
| 25 | `lemma_count_additive` |  |  |  | Y | Y |  |  | unknown | 240&#8209;259 |
| 26 | `lemma_count_other` |  |  |  | Y | Y |  |  | unknown | 268&#8209;287 |
| 27 | `lemma_rank_lt_n_minus_1` |  |  |  | Y | Y |  |  | unknown | 296&#8209;308 |
| 28 | `lemma_link_preserves_wf` |  |  |  | Y | Y |  |  | unknown | 326&#8209;347 |
| 29 | `new` | Y | Y |  |  | Y |  |  | unknown | 405&#8209;408 |
| 30 | `find` | Y | Y |  |  | Y |  |  | unknown | 410&#8209;415 |
| 31 | `union` | Y | Y |  |  | Y |  |  | unknown | 417&#8209;428 |
| 32 | `num_sets` | Y | Y |  |  | Y |  |  | unknown | 430 |
| 33 | `size` | Y | Y |  |  | Y |  |  | unknown | 431&#8209;432 |

### Chap65/UnionFindNoPCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 34 | `lemma_find_in_dom` |  |  |  | Y | Y |  |  | unknown | 131&#8209;144 |
| 35 | `lemma_find_is_root` |  |  |  | Y | Y |  |  | unknown | 155&#8209;168 |
| 36 | `lemma_find_after_link` |  |  |  | Y | Y |  |  | unknown | 180&#8209;212 |
| 37 | `lemma_link_size_rank_inv` |  |  |  | Y | Y |  |  | unknown | 255&#8209;284 |
| 38 | `lemma_link_preserves_inv` |  |  |  | Y | Y |  |  | unknown | 347&#8209;380 |
| 39 | `lemma_find_insert_unchanged` |  |  |  | Y | Y |  |  | unknown | 443&#8209;467 |
| 40 | `lemma_rank_lt_n_minus_1` |  |  |  | Y | Y |  |  | unknown | 487&#8209;498 |
| 41 | `new` | Y | Y |  |  | Y |  |  | unknown | 545&#8209;547 |
| 42 | `insert` | Y | Y |  |  | Y |  |  | unknown | 549&#8209;551 |
| 43 | `find` | Y | Y |  |  | Y |  |  | unknown | 553&#8209;556 |
| 44 | `union_sets` | Y | Y |  |  | Y |  |  | unknown | 558&#8209;560 |
| 45 | `equals` | Y | Y |  |  | Y |  |  | unknown | 562&#8209;564 |
| 46 | `size` | Y | Y |  |  | Y |  |  | unknown | 566 |

### Chap65/UnionFindPCStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `lemma_compose_find_preserved` |  |  |  | Y | Y |  |  | unknown | 151&#8209;164 |
| 48 | `lemma_find_in_dom` |  |  |  | Y | Y |  |  | unknown | 172&#8209;185 |
| 49 | `lemma_find_is_root` |  |  |  | Y | Y |  |  | unknown | 196&#8209;209 |
| 50 | `lemma_rank_lt_find` |  |  |  | Y | Y |  |  | unknown | 220&#8209;235 |
| 51 | `lemma_compress_preserves_find` |  |  |  | Y | Y |  |  | unknown | 262&#8209;292 |
| 52 | `lemma_compress_preserves_find_all` |  |  |  | Y | Y |  |  | unknown | 329&#8209;357 |
| 53 | `lemma_compress_basic` |  |  |  | Y | Y |  |  | unknown | 370&#8209;389 |
| 54 | `lemma_compress_parent_in_dom` |  |  |  | Y | Y |  |  | unknown | 404&#8209;417 |
| 55 | `lemma_compress_rank_inv` |  |  |  | Y | Y |  |  | unknown | 425&#8209;439 |
| 56 | `lemma_compress_iter` |  |  |  | Y | Y |  |  | unknown | 451&#8209;479 |
| 57 | `lemma_build_final_wf` |  |  |  | Y | Y |  |  | unknown | 509&#8209;527 |
| 58 | `lemma_rank_lt_n_from_light_wf` |  |  |  | Y | Y |  |  | unknown | 560&#8209;564 |
| 59 | `lemma_root_find_self` |  |  |  | Y | Y |  |  | unknown | 571&#8209;576 |
| 60 | `lemma_non_root_next` |  |  |  | Y | Y |  |  | unknown | 582&#8209;590 |
| 61 | `lemma_compress_step_find` |  |  |  | Y | Y |  |  | unknown | 597&#8209;626 |
| 62 | `lemma_find_after_link` |  |  |  | Y | Y |  |  | unknown | 642&#8209;672 |
| 63 | `lemma_link_size_rank_inv` |  |  |  | Y | Y |  |  | unknown | 708&#8209;737 |
| 64 | `lemma_link_preserves_inv` |  |  |  | Y | Y |  |  | unknown | 794&#8209;817 |
| 65 | `lemma_find_insert_unchanged` |  |  |  | Y | Y |  |  | unknown | 881&#8209;903 |
| 66 | `lemma_rank_lt_n_minus_1` |  |  |  | Y | Y |  |  | unknown | 919&#8209;930 |
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 965&#8209;967 |
| 68 | `insert` | Y | Y |  |  | Y |  |  | unknown | 969&#8209;976 |
| 69 | `find_root` | Y | Y |  |  | Y |  |  | unknown | 978&#8209;981 |
| 70 | `find` | Y | Y |  |  | Y |  |  | unknown | 983&#8209;993 |
| 71 | `union` | Y | Y |  |  | Y |  |  | unknown | 995&#8209;1001 |
| 72 | `equals` | Y | Y |  |  | Y |  |  | unknown | 1003&#8209;1009 |
| 73 | `size` | Y | Y |  |  | Y |  |  | unknown | 1011 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
