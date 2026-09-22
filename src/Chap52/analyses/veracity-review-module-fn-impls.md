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
| 1 | Chap52 | AdjMatrixGraphMtEph | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 2 | Chap52 | AdjMatrixGraphMtPer | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 3 | Chap52 | AdjMatrixGraphStEph | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 4 | Chap52 | AdjMatrixGraphStPer | 9 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 5 | Chap52 | AdjSeqGraphMtEph | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 6 | Chap52 | AdjSeqGraphMtPer | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 7 | Chap52 | AdjSeqGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 8 | Chap52 | AdjSeqGraphStPer | 9 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 9 | Chap52 | AdjTableGraphMtPer | 10 | 10 | 0 | 1 | 11 | 0 | 11 | 0 | 0 |
| 10 | Chap52 | AdjTableGraphSpecsAndLemmas | 0 | 0 | 0 | 18 | 18 | 0 | 18 | 0 | 0 |
| 11 | Chap52 | AdjTableGraphStEph | 12 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 12 | Chap52 | AdjTableGraphStPer | 12 | 12 | 0 | 3 | 15 | 0 | 15 | 0 | 0 |
| 13 | Chap52 | EdgeSetGraphMtEph | 13 | 14 | 0 | 1 | 15 | 0 | 14 | 0 | 1 |
| 14 | Chap52 | EdgeSetGraphMtPer | 13 | 13 | 0 | 1 | 14 | 0 | 14 | 0 | 0 |
| 15 | Chap52 | EdgeSetGraphStEph | 13 | 13 | 0 | 1 | 14 | 0 | 14 | 0 | 0 |
| 16 | Chap52 | EdgeSetGraphStPer | 13 | 14 | 0 | 1 | 15 | 0 | 15 | 0 | 0 |

## Function-by-Function Detail

### Chap52/AdjMatrixGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;93 |
| 2 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;109 |
| 3 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 4 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;127 |
| 5 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;136 |
| 6 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;149 |
| 7 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;161 |
| 8 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;179 |
| 9 | `complement` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;193 |

### Chap52/AdjMatrixGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 10 | `new` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;98 |
| 11 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;115 |
| 12 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;122 |
| 13 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;133 |
| 14 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;142 |
| 15 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;158 |
| 16 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;170 |
| 17 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;188 |
| 18 | `complement` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;202 |

### Chap52/AdjMatrixGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `new` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;90 |
| 20 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;106 |
| 21 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;112 |
| 22 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;122 |
| 23 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 24 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;140 |
| 25 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;149 |
| 26 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;166 |
| 27 | `complement` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;179 |

### Chap52/AdjMatrixGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 28 | `new` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 29 | `from_matrix` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;118 |
| 30 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;125 |
| 31 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;136 |
| 32 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 33 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;156 |
| 34 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;166 |
| 35 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;184 |
| 36 | `complement` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;198 |
| 37 | `eq` |  | Y |  |  | Y |  |  | unknown | 662&#8209;663 |

### Chap52/AdjSeqGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 38 | `new` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 39 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;110 |
| 40 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;117 |
| 41 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;129 |
| 42 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 43 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;148 |
| 44 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 45 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;180 |
| 46 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;205 |

### Chap52/AdjSeqGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 47 | `new` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 48 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;111 |
| 49 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;118 |
| 50 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 51 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 134&#8209;138 |
| 52 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;148 |
| 53 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;155 |
| 54 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;176 |
| 55 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;193 |

### Chap52/AdjSeqGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 56 | `new` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;91 |
| 57 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;110 |
| 58 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;116 |
| 59 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;127 |
| 60 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;135 |
| 61 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 139&#8209;144 |
| 62 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;150 |
| 63 | `set_neighbors` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;176 |
| 64 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;198 |
| 65 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;220 |
| 66 | `set_edge` | Y | Y |  |  | Y |  |  | unknown | 224&#8209;244 |

### Chap52/AdjSeqGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 67 | `new` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;96 |
| 68 | `from_seq` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;117 |
| 69 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 70 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;135 |
| 71 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;144 |
| 72 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 149&#8209;154 |
| 73 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 74 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;182 |
| 75 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;199 |
| 76 | `eq` |  | Y |  |  | Y |  |  | unknown | 644&#8209;645 |

### Chap52/AdjTableGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 77 | `empty` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;101 |
| 78 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;106 |
| 79 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;113 |
| 80 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;119 |
| 81 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;128 |
| 82 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 83 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;144 |
| 84 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;154 |
| 85 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;168 |
| 86 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;180 |
| 87 | `count_table_edges` |  |  |  | Y | Y |  |  | unknown | 188&#8209;199 |

### Chap52/AdjTableGraphSpecsAndLemmas.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 88 | `lemma_count_true_monotone` |  |  |  | Y | Y |  |  | unknown | 89&#8209;92 |
| 89 | `lemma_count_true_bound` |  |  |  | Y | Y |  |  | unknown | 99&#8209;102 |
| 90 | `lemma_count_true_all_false` |  |  |  | Y | Y |  |  | unknown | 110&#8209;113 |
| 91 | `lemma_count_true_ext` |  |  |  | Y | Y |  |  | unknown | 122&#8209;125 |
| 92 | `lemma_count_true_set_true` |  |  |  | Y | Y |  |  | unknown | 134&#8209;140 |
| 93 | `lemma_count_true_set_false` |  |  |  | Y | Y |  |  | unknown | 154&#8209;160 |
| 94 | `lemma_count_true_at_least_one` |  |  |  | Y | Y |  |  | unknown | 174&#8209;177 |
| 95 | `lemma_sum_of_monotone` |  |  |  | Y | Y |  |  | unknown | 187&#8209;190 |
| 96 | `lemma_sum_of_unfold` |  |  |  | Y | Y |  |  | unknown | 198&#8209;200 |
| 97 | `lemma_sum_of_all_zero` |  |  |  | Y | Y |  |  | unknown | 205&#8209;208 |
| 98 | `lemma_sum_of_ext` |  |  |  | Y | Y |  |  | unknown | 217&#8209;220 |
| 99 | `lemma_sum_of_change_one` |  |  |  | Y | Y |  |  | unknown | 230&#8209;236 |
| 100 | `lemma_sum_of_lower_bound` |  |  |  | Y | Y |  |  | unknown | 250&#8209;253 |
| 101 | `lemma_sum_of_bounded` |  |  |  | Y | Y |  |  | unknown | 262&#8209;267 |
| 102 | `lemma_sum_adj_remove` |  |  |  | Y | Y |  |  | unknown | 283&#8209;286 |
| 103 | `lemma_sum_adj_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 302&#8209;308 |
| 104 | `lemma_sum_entry_sizes_eq` |  |  |  | Y | Y |  |  | unknown | 331&#8209;338 |
| 105 | `lemma_sum_entry_sizes_monotone` |  |  |  | Y | Y |  |  | unknown | 397&#8209;400 |

### Chap52/AdjTableGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `empty` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;90 |
| 107 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 94&#8209;105 |
| 108 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;109 |
| 109 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;114 |
| 110 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 118&#8209;122 |
| 111 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;127 |
| 112 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;134 |
| 113 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 114 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 115 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;150 |
| 116 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;162 |
| 117 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |

### Chap52/AdjTableGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 118 | `lemma_entries_to_map_eq` |  |  |  | Y | Y |  |  | unknown | 83&#8209;85 |
| 119 | `lemma_keys_no_dups_eq` |  |  |  | Y | Y |  |  | unknown | 93&#8209;96 |
| 120 | `lemma_sum_entry_sizes_eq_stper` |  |  |  | Y | Y |  |  | unknown | 100&#8209;106 |
| 121 | `empty` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;127 |
| 122 | `from_table` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;141 |
| 123 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 124 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;152 |
| 125 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;159 |
| 126 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 127 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;173 |
| 128 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 177&#8209;178 |
| 129 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 130 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 131 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;199 |
| 132 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 202&#8209;207 |

### Chap52/EdgeSetGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `lemma_eq_spec_iff_view_eq` |  |  |  | Y | Y |  |  | unknown | 59&#8209;64 |
| 134 | `empty` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;104 |
| 135 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;118 |
| 136 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 137 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 138 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;130 |
| 139 | `edges` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 140 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 141 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;147 |
| 142 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;153 |
| 143 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;160 |
| 144 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;168 |
| 145 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;176 |
| 146 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 179&#8209;183 |
| 147 | `default` |  | Y |  |  | Y |  | Y |  | 424 |

### Chap52/EdgeSetGraphMtPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 148 | `lemma_eq_spec_iff_view_eq` |  |  |  | Y | Y |  |  | unknown | 73&#8209;78 |
| 149 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;101 |
| 150 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;115 |
| 151 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 152 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;125 |
| 153 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 154 | `edges` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 155 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 156 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;149 |
| 157 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;156 |
| 158 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;163 |
| 159 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 160 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;179 |
| 161 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;186 |

### Chap52/EdgeSetGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 162 | `lemma_eq_spec_iff_view_eq` |  |  |  | Y | Y |  |  | unknown | 55&#8209;60 |
| 163 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;101 |
| 164 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;116 |
| 165 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 166 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 167 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 168 | `edges` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 169 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;138 |
| 170 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 171 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 172 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;155 |
| 173 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;161 |
| 174 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 165&#8209;170 |
| 175 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |

### Chap52/EdgeSetGraphStPer.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 176 | `lemma_eq_spec_iff_view_eq` |  |  |  | Y | Y |  |  | unknown | 60&#8209;65 |
| 177 | `empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;105 |
| 178 | `from_vertices_and_edges` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;119 |
| 179 | `num_vertices` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 180 | `num_edges` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 181 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;133 |
| 182 | `edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 183 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;142 |
| 184 | `out_neighbors` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;148 |
| 185 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;153 |
| 186 | `insert_vertex` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;160 |
| 187 | `delete_vertex` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 188 | `insert_edge` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;173 |
| 189 | `delete_edge` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;178 |
| 190 | `eq` |  | Y |  |  | Y |  |  | unknown | 471&#8209;472 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
