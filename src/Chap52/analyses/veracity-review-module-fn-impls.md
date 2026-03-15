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
| 4 | Chap52 | AdjMatrixGraphStPer | 9 | 10 | 0 | 3 | 13 | 0 | 13 | 0 | 0 |
| 5 | Chap52 | AdjSeqGraphMtEph | 7 | 7 | 0 | 2 | 9 | 0 | 9 | 0 | 0 |
| 6 | Chap52 | AdjSeqGraphMtPer | 6 | 6 | 0 | 2 | 8 | 0 | 8 | 0 | 0 |
| 7 | Chap52 | AdjSeqGraphStEph | 9 | 9 | 0 | 2 | 11 | 0 | 11 | 0 | 0 |
| 8 | Chap52 | AdjSeqGraphStPer | 9 | 10 | 0 | 2 | 12 | 0 | 12 | 0 | 0 |
| 9 | Chap52 | AdjTableGraphMtPer | 10 | 11 | 0 | 0 | 11 | 0 | 10 | 0 | 1 |
| 10 | Chap52 | AdjTableGraphStEph | 12 | 12 | 0 | 1 | 13 | 0 | 13 | 0 | 0 |
| 11 | Chap52 | AdjTableGraphStPer | 12 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 12 | Chap52 | EdgeSetGraphMtPer | 13 | 14 | 0 | 0 | 14 | 0 | 13 | 0 | 1 |
| 13 | Chap52 | EdgeSetGraphStEph | 13 | 13 | 0 | 0 | 13 | 0 | 13 | 0 | 0 |
| 14 | Chap52 | EdgeSetGraphStPer | 13 | 13 | 0 | 0 | 13 | 0 | 13 | 0 | 0 |

## Function-by-Function Detail

### Chap52/AdjMatrixGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 2 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 3 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 4 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 5 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;128 |
| 6 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 7 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;147 |
| 8 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 9 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;163 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 11 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;186 |
| 12 | `complement` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;196 |

### Chap52/AdjMatrixGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 13 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 73&#8209;76 |
| 14 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 83&#8209;86 |
| 15 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 16 | `new` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 17 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 18 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;136 |
| 19 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;143 |
| 20 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;157 |
| 21 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;167 |
| 22 | `complement` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;177 |

### Chap52/AdjMatrixGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 23 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 72&#8209;75 |
| 24 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 82&#8209;85 |
| 25 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 26 | `new` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;116 |
| 27 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;128 |
| 28 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;133 |
| 29 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;147 |
| 30 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 31 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;163 |
| 32 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 33 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;186 |
| 34 | `complement` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;196 |

### Chap52/AdjMatrixGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 35 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 78&#8209;81 |
| 36 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 88&#8209;91 |
| 37 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 98&#8209;101 |
| 38 | `new` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 39 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;134 |
| 40 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 41 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;153 |
| 42 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 43 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;169 |
| 44 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 172&#8209;177 |
| 45 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;192 |
| 46 | `complement` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;202 |
| 47 | `eq` |  | Y |  |  | Y |  |  | unknown | 474&#8209;475 |

### Chap52/AdjSeqGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 48 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 49 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 50 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 51 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 52 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;111 |
| 53 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 54 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;126 |
| 55 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 56 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;153 |

### Chap52/AdjSeqGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 57 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 58&#8209;61 |
| 58 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 68&#8209;70 |
| 59 | `new` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 60 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;94 |
| 61 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;108 |
| 62 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |
| 63 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;123 |
| 64 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |

### Chap52/AdjSeqGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 65 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 60&#8209;63 |
| 66 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 71&#8209;73 |
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 68 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;108 |
| 69 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;113 |
| 70 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;127 |
| 71 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 72 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;142 |
| 73 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 74 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;167 |
| 75 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;189 |

### Chap52/AdjSeqGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 76 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 64&#8209;67 |
| 77 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 75&#8209;77 |
| 78 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 79 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;112 |
| 80 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 81 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;131 |
| 82 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 83 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;146 |
| 84 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;151 |
| 85 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;170 |
| 86 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;186 |
| 87 | `eq` |  | Y |  |  | Y |  |  | unknown | 510&#8209;511 |

### Chap52/AdjTableGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 88 | `empty` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;79 |
| 89 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;82 |
| 90 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;86 |
| 91 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;90 |
| 92 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 93 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;99 |
| 94 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 95 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;107 |
| 96 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;115 |
| 97 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;122 |
| 98 | `default` |  | Y |  |  | Y |  | Y |  | 262 |

### Chap52/AdjTableGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 99 | `lemma_sum_adj_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 76&#8209;78 |
| 100 | `empty` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 101 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;99 |
| 102 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;102 |
| 103 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 104 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 105 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;112 |
| 106 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 107 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;118 |
| 108 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 109 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 110 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;134 |
| 111 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;141 |

### Chap52/AdjTableGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 112 | `empty` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;60 |
| 113 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;68 |
| 114 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 115 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;75 |
| 116 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;79 |
| 117 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 81&#8209;83 |
| 118 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 119 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 120 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 121 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 122 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 123 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |

### Chap52/EdgeSetGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `empty` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;55 |
| 125 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;62 |
| 126 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 64&#8209;65 |
| 127 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 128 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 129 | `edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 130 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 131 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;81 |
| 132 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;84 |
| 133 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 134 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;92 |
| 135 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;96 |
| 136 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;100 |
| 137 | `default` |  | Y |  |  | Y |  | Y |  | 207 |

### Chap52/EdgeSetGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 138 | `empty` | Y | Y |  |  | Y |  |  | unknown | 55&#8209;56 |
| 139 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;63 |
| 140 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;66 |
| 141 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 68&#8209;69 |
| 142 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;72 |
| 143 | `edges` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 144 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;78 |
| 145 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;82 |
| 146 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 84&#8209;85 |
| 147 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;89 |
| 148 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;93 |
| 149 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;97 |
| 150 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;101 |

### Chap52/EdgeSetGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 151 | `empty` | Y | Y |  |  | Y |  |  | unknown | 57&#8209;58 |
| 152 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 60&#8209;65 |
| 153 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;68 |
| 154 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;71 |
| 155 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;74 |
| 156 | `edges` | Y | Y |  |  | Y |  |  | unknown | 76&#8209;77 |
| 157 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;80 |
| 158 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;84 |
| 159 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;87 |
| 160 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;91 |
| 161 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 162 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;99 |
| 163 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
