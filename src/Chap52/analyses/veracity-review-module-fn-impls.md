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
| 1 | Chap52 | AdjMatrixGraphMtEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 2 | Chap52 | AdjMatrixGraphMtPer | 7 | 7 | 0 | 3 | 10 | 0 | 10 | 0 | 0 |
| 3 | Chap52 | AdjMatrixGraphStEph | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 4 | Chap52 | AdjMatrixGraphStPer | 9 | 9 | 0 | 3 | 12 | 0 | 12 | 0 | 0 |
| 5 | Chap52 | AdjSeqGraphMtEph | 7 | 7 | 0 | 2 | 9 | 0 | 9 | 0 | 0 |
| 6 | Chap52 | AdjSeqGraphMtPer | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 7 | Chap52 | AdjSeqGraphStEph | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 8 | Chap52 | AdjSeqGraphStPer | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 9 | Chap52 | AdjTableGraphMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 6 | 1 | 4 |
| 10 | Chap52 | AdjTableGraphStEph | 12 | 12 | 0 | 1 | 13 | 0 | 9 | 0 | 4 |
| 11 | Chap52 | AdjTableGraphStPer | 12 | 12 | 0 | 0 | 12 | 0 | 8 | 0 | 4 |
| 12 | Chap52 | EdgeSetGraphMtPer | 13 | 14 | 0 | 0 | 14 | 0 | 1 | 1 | 12 |
| 13 | Chap52 | EdgeSetGraphStEph | 13 | 13 | 0 | 0 | 13 | 0 | 2 | 0 | 11 |
| 14 | Chap52 | EdgeSetGraphStPer | 13 | 13 | 0 | 0 | 13 | 0 | 2 | 0 | 11 |

## Function-by-Function Detail

### Chap52/AdjMatrixGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 73&#8209;76 |
| 2 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 83&#8209;86 |
| 3 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 5 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;129 |
| 6 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 7 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;148 |
| 8 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 9 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;164 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;172 |
| 11 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;187 |
| 12 | `complement` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;197 |

### Chap52/AdjMatrixGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 74&#8209;77 |
| 14 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 84&#8209;87 |
| 15 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 94&#8209;97 |
| 16 | `new` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 17 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;123 |
| 18 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;137 |
| 19 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 20 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;158 |
| 21 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;168 |
| 22 | `complement` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;178 |

### Chap52/AdjMatrixGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 73&#8209;76 |
| 24 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 83&#8209;86 |
| 25 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 27 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;129 |
| 28 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 29 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;148 |
| 30 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 31 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;164 |
| 32 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;172 |
| 33 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;187 |
| 34 | `complement` | Y | Y |  |  | Y |  |  | unknown | 190&#8209;197 |

### Chap52/AdjMatrixGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 76&#8209;79 |
| 36 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 86&#8209;89 |
| 37 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 96&#8209;99 |
| 38 | `new` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;120 |
| 39 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;132 |
| 40 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;137 |
| 41 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;151 |
| 42 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;156 |
| 43 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;167 |
| 44 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;175 |
| 45 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 178&#8209;190 |
| 46 | `complement` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;200 |

### Chap52/AdjSeqGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 63&#8209;66 |
| 48 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 74&#8209;76 |
| 49 | `new` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;93 |
| 50 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 51 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;110 |
| 52 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 53 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;125 |
| 54 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;130 |
| 55 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;150 |

### Chap52/AdjSeqGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 59&#8209;62 |
| 57 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 69&#8209;71 |
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;88 |
| 59 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 60 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;105 |
| 61 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;112 |
| 62 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;120 |
| 63 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |

### Chap52/AdjSeqGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 64 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 61&#8209;64 |
| 65 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 72&#8209;74 |
| 66 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;91 |
| 67 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;101 |
| 68 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;105 |
| 69 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;118 |
| 70 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 71 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;133 |
| 72 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 73 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;153 |
| 74 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;173 |

### Chap52/AdjSeqGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 75 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 62&#8209;65 |
| 76 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 73&#8209;75 |
| 77 | `new` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;92 |
| 78 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;102 |
| 79 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 80 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;119 |
| 81 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;126 |
| 82 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;134 |
| 83 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 84 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;156 |
| 85 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;171 |

### Chap52/AdjTableGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 86 | `empty` | Y | Y |  |  | Y |  | Y |  | 66 |
| 87 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 68 |
| 88 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;72 |
| 89 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 90 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;80 |
| 91 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 82 |
| 92 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 93 | `delete_vertex` | Y | Y |  |  | Y |  |  | hole | 87&#8209;88 |
| 94 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 95 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 96 | `default` |  | Y |  |  | Y |  | Y |  | 279 |

### Chap52/AdjTableGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 97 | `lemma_sum_adj_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 75&#8209;77 |
| 98 | `empty` | Y | Y |  |  | Y |  | Y |  | 88 |
| 99 | `from_table` | Y | Y |  |  | Y |  | Y |  | 90 |
| 100 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 92 |
| 101 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 102 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 98 |
| 103 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 100 |
| 104 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 102 |
| 105 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 104 |
| 106 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 106 |
| 107 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 108 |
| 108 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 110 |
| 109 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 112 |

### Chap52/AdjTableGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 110 | `empty` | Y | Y |  |  | Y |  | Y |  | 56 |
| 111 | `from_table` | Y | Y |  |  | Y |  | Y |  | 58 |
| 112 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 60 |
| 113 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;64 |
| 114 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;67 |
| 115 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 116 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;75 |
| 117 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 77 |
| 118 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 119 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;83 |
| 120 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 121 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;94 |

### Chap52/EdgeSetGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 122 | `empty` | Y | Y |  |  | Y |  | Y |  | 49 |
| 123 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 51 |
| 124 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 53 |
| 125 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 55 |
| 126 | `vertices` | Y | Y |  |  | Y |  | Y |  | 57 |
| 127 | `edges` | Y | Y |  |  | Y |  | Y |  | 59 |
| 128 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 61 |
| 129 | `out_neighbors` | Y | Y |  |  | Y |  |  | hole | 63&#8209;64 |
| 130 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 66 |
| 131 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 68 |
| 132 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 133 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 73 |
| 134 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 75 |
| 135 | `default` |  | Y |  |  | Y |  | Y |  | 187 |

### Chap52/EdgeSetGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 136 | `empty` | Y | Y |  |  | Y |  | Y |  | 53 |
| 137 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 55 |
| 138 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 57 |
| 139 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 59 |
| 140 | `vertices` | Y | Y |  |  | Y |  | Y |  | 61 |
| 141 | `edges` | Y | Y |  |  | Y |  | Y |  | 63 |
| 142 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 65 |
| 143 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 144 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 70 |
| 145 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 72 |
| 146 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 147 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 77 |
| 148 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 79 |

### Chap52/EdgeSetGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 149 | `empty` | Y | Y |  |  | Y |  | Y |  | 55 |
| 150 | `from_vertices_and_edges` | Y | Y |  |  | Y |  | Y |  | 57 |
| 151 | `num_vertices` | Y | Y |  |  | Y |  | Y |  | 59 |
| 152 | `num_edges` | Y | Y |  |  | Y |  | Y |  | 61 |
| 153 | `vertices` | Y | Y |  |  | Y |  | Y |  | 63 |
| 154 | `edges` | Y | Y |  |  | Y |  | Y |  | 65 |
| 155 | `has_edge` | Y | Y |  |  | Y |  | Y |  | 67 |
| 156 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 69&#8209;70 |
| 157 | `out_degree` | Y | Y |  |  | Y |  | Y |  | 72 |
| 158 | `insert_vertex` | Y | Y |  |  | Y |  | Y |  | 74 |
| 159 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 160 | `insert_edge` | Y | Y |  |  | Y |  | Y |  | 79 |
| 161 | `delete_edge` | Y | Y |  |  | Y |  | Y |  | 81 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
