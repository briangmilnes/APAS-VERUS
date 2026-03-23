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
| 1 | Chap06 | DirGraphMtEph | 23 | 24 | 0 | 0 | 24 | 0 | 13 | 11 | 0 |
| 2 | Chap06 | DirGraphStEph | 17 | 18 | 2 | 0 | 20 | 0 | 20 | 0 | 0 |
| 3 | Chap06 | LabDirGraphMtEph | 14 | 14 | 0 | 0 | 14 | 0 | 10 | 4 | 0 |
| 4 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 5 | Chap06 | LabUnDirGraphMtEph | 11 | 11 | 0 | 0 | 11 | 0 | 4 | 7 | 0 |
| 6 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 7 | Chap06 | UnDirGraphMtEph | 14 | 15 | 0 | 0 | 15 | 0 | 8 | 7 | 0 |
| 8 | Chap06 | UnDirGraphStEph | 11 | 12 | 0 | 0 | 12 | 0 | 12 | 0 | 0 |
| 9 | Chap06 | WeightedDirGraphStEphF64 | 6 | 6 | 0 | 0 | 6 | 0 | 6 | 0 | 0 |
| 10 | Chap06 | WeightedDirGraphStEphI128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 11 | Chap06 | WeightedDirGraphStEphI16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 12 | Chap06 | WeightedDirGraphStEphI32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 13 | Chap06 | WeightedDirGraphStEphI64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 14 | Chap06 | WeightedDirGraphStEphI8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 15 | Chap06 | WeightedDirGraphStEphIsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 16 | Chap06 | WeightedDirGraphStEphU128 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 17 | Chap06 | WeightedDirGraphStEphU16 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 18 | Chap06 | WeightedDirGraphStEphU32 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 19 | Chap06 | WeightedDirGraphStEphU64 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 20 | Chap06 | WeightedDirGraphStEphU8 | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |
| 21 | Chap06 | WeightedDirGraphStEphUsize | 9 | 9 | 0 | 0 | 9 | 0 | 9 | 0 | 0 |

## Function-by-Function Detail

### Chap06/DirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `empty` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 2 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;119 |
| 3 | `vertices` x3 | Y | Y |  |  | Y |  |  | hole | 741&#8209;743 |
| 4 | `arcs` x3 | Y | Y |  |  | Y |  |  | hole | 745&#8209;747 |
| 5 | `sizeV` x3 | Y | Y |  |  | Y |  |  | hole | 749&#8209;751 |
| 6 | `sizeA` x3 | Y | Y |  |  | Y |  |  | hole | 753&#8209;755 |
| 7 | `neighbor` x3 | Y | Y |  |  | Y |  |  | hole | 757&#8209;762 |
| 8 | `incident` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 9 | `n_plus` x3 | Y | Y |  |  | Y |  |  | hole | 764&#8209;771 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;192 |
| 11 | `n_minus` x3 | Y | Y |  |  | Y |  |  | hole | 773&#8209;780 |
| 12 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;227 |
| 13 | `ng` x3 | Y | Y |  |  | Y |  |  | hole | 782&#8209;788 |
| 14 | `degree` | Y | Y |  |  | Y |  |  | unknown | 255&#8209;260 |
| 15 | `n_plus_of_vertices` x3 | Y | Y |  |  | Y |  |  | hole | 790&#8209;796 |
| 16 | `n_minus_of_vertices` x3 | Y | Y |  |  | Y |  |  | hole | 798&#8209;804 |
| 17 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | hole | 806&#8209;812 |
| 18 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 335&#8209;345 |
| 19 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 348&#8209;358 |
| 20 | `n_plus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 361&#8209;371 |
| 21 | `n_minus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 374&#8209;384 |
| 22 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 387&#8209;397 |
| 23 | `new` | Y | Y |  |  | Y |  |  | unknown | 729&#8209;739 |
| 24 | `eq` |  | Y |  |  | Y |  |  | unknown | 942&#8209;943 |

### Chap06/DirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `empty` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;119 |
| 26 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;131 |
| 27 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;136 |
| 28 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 140&#8209;141 |
| 29 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;147 |
| 30 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;153 |
| 31 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;159 |
| 32 | `ng` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 33 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;171 |
| 34 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 175&#8209;177 |
| 35 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 181&#8209;183 |
| 36 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 187&#8209;189 |
| 37 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 193&#8209;195 |
| 38 | `incident` | Y | Y |  |  | Y |  |  | unknown | 199&#8209;201 |
| 39 | `degree` | Y | Y |  |  | Y |  |  | unknown | 205&#8209;207 |
| 40 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 211&#8209;213 |
| 41 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;219 |
| 42 | `iter_vertices` |  |  | Y |  | Y |  |  | unknown | 227&#8209;229 |
| 43 | `iter_arcs` |  |  | Y |  | Y |  |  | unknown | 233&#8209;235 |
| 44 | `eq` |  | Y |  |  | Y |  |  | unknown | 594&#8209;595 |

### Chap06/LabDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `empty` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;104 |
| 46 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 108&#8209;118 |
| 47 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;123 |
| 48 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 127&#8209;128 |
| 49 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;135 |
| 50 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | hole | 657&#8209;664 |
| 51 | `add_labeled_arc` x3 | Y | Y |  |  | Y |  |  | hole | 666&#8209;674 |
| 52 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;159 |
| 53 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;165 |
| 54 | `n_plus` x3 | Y | Y |  |  | Y |  |  | hole | 676&#8209;682 |
| 55 | `n_minus` x3 | Y | Y |  |  | Y |  |  | hole | 684&#8209;690 |
| 56 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;232 |
| 57 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 235&#8209;245 |
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 645&#8209;655 |

### Chap06/LabDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `empty` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;75 |
| 60 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;87 |
| 61 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;92 |
| 62 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;97 |
| 63 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;103 |
| 64 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;109 |
| 65 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;117 |
| 66 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;125 |
| 67 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;131 |
| 68 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |
| 69 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;145 |

### Chap06/LabUnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 602&#8209;607 |
| 71 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;126 |
| 72 | `vertices` x3 | Y | Y |  |  | Y |  |  | hole | 609&#8209;610 |
| 73 | `labeled_edges` x3 | Y | Y |  |  | Y |  |  | hole | 612&#8209;613 |
| 74 | `edges` x3 | Y | Y |  |  | Y |  |  | hole | 615&#8209;618 |
| 75 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | hole | 632&#8209;636 |
| 76 | `add_labeled_edge` x3 | Y | Y |  |  | Y |  |  | hole | 638&#8209;639 |
| 77 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;164 |
| 78 | `has_edge` x3 | Y | Y |  |  | Y |  |  | hole | 620&#8209;624 |
| 79 | `ng` x3 | Y | Y |  |  | Y |  |  | hole | 626&#8209;630 |
| 80 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 194&#8209;205 |

### Chap06/LabUnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 81 | `empty` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;71 |
| 82 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 75&#8209;83 |
| 83 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 87&#8209;88 |
| 84 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 92&#8209;93 |
| 85 | `edges` | Y | Y |  |  | Y |  |  | unknown | 97&#8209;100 |
| 86 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 104&#8209;106 |
| 87 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;115 |
| 88 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 89 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;132 |
| 90 | `ng` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;138 |

### Chap06/UnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 91 | `empty` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;117 |
| 92 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;131 |
| 93 | `vertices` x3 | Y | Y |  |  | Y |  |  | hole | 498&#8209;500 |
| 94 | `edges` x3 | Y | Y |  |  | Y |  |  | hole | 502&#8209;504 |
| 95 | `sizeV` x3 | Y | Y |  |  | Y |  |  | hole | 506&#8209;508 |
| 96 | `sizeE` x3 | Y | Y |  |  | Y |  |  | hole | 510&#8209;512 |
| 97 | `neighbor` x3 | Y | Y |  |  | Y |  |  | hole | 514&#8209;519 |
| 98 | `ng` x3 | Y | Y |  |  | Y |  |  | hole | 521&#8209;527 |
| 99 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | hole | 529&#8209;535 |
| 100 | `incident` | Y | Y |  |  | Y |  |  | unknown | 203&#8209;205 |
| 101 | `degree` | Y | Y |  |  | Y |  |  | unknown | 209&#8209;214 |
| 102 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 217&#8209;227 |
| 103 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 230&#8209;240 |
| 104 | `new` | Y | Y |  |  | Y |  |  | unknown | 486&#8209;496 |
| 105 | `eq` |  | Y |  |  | Y |  |  | unknown | 629&#8209;630 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `empty` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;101 |
| 107 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;112 |
| 108 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 116&#8209;117 |
| 109 | `edges` | Y | Y |  |  | Y |  |  | unknown | 121&#8209;122 |
| 110 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 126&#8209;128 |
| 111 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;134 |
| 112 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;144 |
| 113 | `ng` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;155 |
| 114 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 159&#8209;166 |
| 115 | `incident` | Y | Y |  |  | Y |  |  | unknown | 170&#8209;172 |
| 116 | `degree` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;181 |
| 117 | `eq` |  | Y |  |  | Y |  |  | unknown | 365&#8209;366 |

### Chap06/WeightedDirGraphStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 118 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 40&#8209;47 |
| 119 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 51&#8209;55 |
| 120 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 121 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 67&#8209;70 |
| 122 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 74&#8209;78 |
| 123 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 82&#8209;86 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 125 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 126 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 127 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 128 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 129 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 130 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 131 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 132 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 134 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 135 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 136 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 137 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 138 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 139 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 140 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 141 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 142 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 143 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 144 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 145 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 146 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 147 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 148 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 149 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 150 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 151 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 152 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 153 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 154 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 155 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 156 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 157 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 158 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 159 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 160 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 161 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 162 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 163 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 164 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 165 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 166 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 167 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 168 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 169 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 170 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 171 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 172 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 173 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 174 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 175 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 176 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 177 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 178 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 179 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 180 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 181 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 182 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 183 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 184 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 185 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 186 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 187 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 188 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 189 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 190 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 191 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 192 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 193 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 194 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 195 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 196 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 197 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 198 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 199 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 200 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 201 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 202 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 203 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 204 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 205 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 206 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 207 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 208 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 209 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 210 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 211 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 212 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 213 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 214 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 215 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 216 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 217 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 218 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 219 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 220 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 221 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 222 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 223 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 43&#8209;50 |
| 224 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 54&#8209;58 |
| 225 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 62&#8209;66 |
| 226 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;73 |
| 227 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 77&#8209;81 |
| 228 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 85&#8209;89 |
| 229 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 93&#8209;95 |
| 230 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;103 |
| 231 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 107&#8209;111 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
