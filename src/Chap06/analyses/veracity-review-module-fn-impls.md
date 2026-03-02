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
| 1 | Chap06 | DirGraphMtEph | 17 | 18 | 0 | 5 | 23 | 0 | 23 | 0 | 0 |
| 2 | Chap06 | DirGraphStEph | 17 | 18 | 2 | 0 | 20 | 0 | 20 | 0 | 0 |
| 3 | Chap06 | LabDirGraphMtEph | 11 | 11 | 0 | 2 | 13 | 0 | 13 | 0 | 0 |
| 4 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 5 | Chap06 | LabUnDirGraphMtEph | 10 | 10 | 0 | 1 | 11 | 0 | 11 | 0 | 0 |
| 6 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 7 | Chap06 | UnDirGraphMtEph | 11 | 12 | 0 | 2 | 14 | 0 | 14 | 0 | 0 |
| 8 | Chap06 | UnDirGraphStEph | 11 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 9 | Chap06 | WeightedDirGraphStEphI128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 10 | Chap06 | WeightedDirGraphStEphI16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 11 | Chap06 | WeightedDirGraphStEphI32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 12 | Chap06 | WeightedDirGraphStEphI64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 13 | Chap06 | WeightedDirGraphStEphI8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 14 | Chap06 | WeightedDirGraphStEphIsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 15 | Chap06 | WeightedDirGraphStEphU128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 16 | Chap06 | WeightedDirGraphStEphU16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 17 | Chap06 | WeightedDirGraphStEphU32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 18 | Chap06 | WeightedDirGraphStEphU64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 19 | Chap06 | WeightedDirGraphStEphU8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 20 | Chap06 | WeightedDirGraphStEphUsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |

## Function-by-Function Detail

### Chap06/DirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `empty` | Y | Y |  |  | Y |  |  | unknown | 98&#8209;103 |
| 2 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;117 |
| 3 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 4 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;127 |
| 5 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 6 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 7 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;147 |
| 8 | `incident` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 9 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 171&#8209;178 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 182&#8209;187 |
| 11 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;212 |
| 12 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;221 |
| 13 | `ng` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;238 |
| 14 | `degree` | Y | Y |  |  | Y |  |  | unknown | 248&#8209;253 |
| 15 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 269&#8209;276 |
| 16 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 292&#8209;299 |
| 17 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 315&#8209;322 |
| 18 | `n_plus_par` |  |  |  | Y | Y |  |  | unknown | 329&#8209;339 |
| 19 | `n_minus_par` |  |  |  | Y | Y |  |  | unknown | 376&#8209;386 |
| 20 | `n_plus_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 423&#8209;435 |
| 21 | `n_minus_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 490&#8209;502 |
| 22 | `ng_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 557&#8209;569 |
| 23 | `eq` |  | Y |  |  | Y |  |  | unknown | 697&#8209;698 |

### Chap06/DirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 24 | `empty` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 25 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;131 |
| 26 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 27 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 28 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 29 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 30 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 31 | `ng` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 32 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 33 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 34 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 35 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 36 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 37 | `incident` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 38 | `degree` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 39 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 40 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 41 | `iter_vertices` |  |  | Y |  | Y |  |  | unknown | 227&#8209;228 |
| 42 | `iter_arcs` |  |  | Y |  | Y |  |  | unknown | 232&#8209;233 |
| 43 | `eq` |  | Y |  |  | Y |  |  | unknown | 592&#8209;593 |

### Chap06/LabDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 44 | `empty` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;102 |
| 45 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;116 |
| 46 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 120&#8209;121 |
| 47 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 48 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;133 |
| 49 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;139 |
| 50 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;148 |
| 51 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 152&#8209;157 |
| 52 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 161&#8209;163 |
| 53 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 196&#8209;203 |
| 54 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 208&#8209;215 |
| 55 | `n_plus_par` |  |  |  | Y | Y |  |  | unknown | 222&#8209;235 |
| 56 | `n_minus_par` |  |  |  | Y | Y |  |  | unknown | 337&#8209;350 |

### Chap06/LabDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 57 | `empty` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 58 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;87 |
| 59 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 60 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 61 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 62 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 63 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 64 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 65 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 66 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 67 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |

### Chap06/LabUnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 68 | `empty` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;110 |
| 69 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;124 |
| 70 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 71 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 72 | `edges` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;141 |
| 73 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 74 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 75 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;162 |
| 76 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;171 |
| 77 | `ng` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;188 |
| 78 | `ng_par` |  |  |  | Y | Y |  |  | unknown | 195&#8209;208 |

### Chap06/LabUnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 79 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;71 |
| 80 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;83 |
| 81 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 82 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 83 | `edges` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 84 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 85 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |
| 86 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 87 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 88 | `ng` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |

### Chap06/UnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 89 | `empty` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |
| 90 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;129 |
| 91 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 92 | `edges` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;139 |
| 93 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;144 |
| 94 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;149 |
| 95 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 96 | `ng` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;176 |
| 97 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;193 |
| 98 | `incident` | Y | Y |  |  | Y |  |  | unknown | 197&#8209;199 |
| 99 | `degree` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;208 |
| 100 | `ng_par` |  |  |  | Y | Y |  |  | unknown | 215&#8209;225 |
| 101 | `ng_of_vertices_par` |  |  |  | Y | Y |  |  | unknown | 312&#8209;324 |
| 102 | `eq` |  | Y |  |  | Y |  |  | unknown | 480&#8209;481 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 103 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 104 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;110 |
| 105 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 106 | `edges` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 107 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 108 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 109 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 110 | `ng` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;151 |
| 111 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;161 |
| 112 | `incident` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 113 | `degree` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;174 |
| 114 | `eq` |  | Y |  |  | Y |  |  | unknown | 358&#8209;359 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 115 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 116 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 117 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 118 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 119 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 120 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 121 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 122 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 123 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 125 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 126 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 127 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 128 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 129 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 130 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 131 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 132 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 134 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 135 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 136 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 137 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 138 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 139 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 140 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 141 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 142 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 143 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 144 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 145 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 146 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 147 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 148 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 149 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 150 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 151 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 152 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 153 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 154 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 155 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 156 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 157 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 158 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 159 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 160 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 161 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 162 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 163 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 164 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 165 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 166 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 167 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 168 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 169 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 170 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 171 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 172 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 173 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 174 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 175 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 176 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 177 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 178 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 179 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 180 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 181 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 182 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 183 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 184 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 185 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 186 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 187 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 188 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 189 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 190 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 191 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 192 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 193 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 194 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 195 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 196 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 197 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 198 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 199 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 200 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 201 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 202 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 203 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 204 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 205 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 206 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 207 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 208 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 209 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 210 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 211 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 212 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 213 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 214 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;48 |
| 215 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 216 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 58&#8209;62 |
| 217 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 65&#8209;68 |
| 218 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 71&#8209;75 |
| 219 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 78&#8209;82 |
| 220 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;87 |
| 221 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 89&#8209;93 |
| 222 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;99 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
