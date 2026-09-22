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
| 1 | Chap06 | DirGraphMtEph | 23 | 24 | 0 | 0 | 24 | 0 | 24 | 0 | 0 |
| 2 | Chap06 | DirGraphStEph | 17 | 18 | 2 | 0 | 20 | 0 | 20 | 0 | 0 |
| 3 | Chap06 | LabDirGraphMtEph | 14 | 14 | 0 | 0 | 14 | 0 | 12 | 2 | 0 |
| 4 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 5 | Chap06 | LabUnDirGraphMtEph | 11 | 11 | 0 | 0 | 11 | 0 | 9 | 2 | 0 |
| 6 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 7 | Chap06 | UnDirGraphMtEph | 14 | 15 | 0 | 0 | 15 | 0 | 15 | 0 | 0 |
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
| 1 | `empty` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;125 |
| 2 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 129&#8209;138 |
| 3 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 836&#8209;838 |
| 4 | `arcs` x3 | Y | Y |  |  | Y |  |  | unknown | 841&#8209;843 |
| 5 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 846&#8209;848 |
| 6 | `sizeA` x3 | Y | Y |  |  | Y |  |  | unknown | 851&#8209;853 |
| 7 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 856&#8209;861 |
| 8 | `incident` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 9 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 864&#8209;871 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 206&#8209;211 |
| 11 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 874&#8209;881 |
| 12 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 241&#8209;246 |
| 13 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 884&#8209;891 |
| 14 | `degree` | Y | Y |  |  | Y |  |  | unknown | 274&#8209;279 |
| 15 | `n_plus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 894&#8209;901 |
| 16 | `n_minus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 904&#8209;911 |
| 17 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 914&#8209;921 |
| 18 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 355&#8209;365 |
| 19 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 369&#8209;379 |
| 20 | `n_plus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 383&#8209;393 |
| 21 | `n_minus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 397&#8209;407 |
| 22 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 411&#8209;421 |
| 23 | `new` | Y | Y |  |  | Y |  |  | unknown | 825&#8209;833 |
| 24 | `eq` |  | Y |  |  | Y |  |  | unknown | 1116&#8209;1117 |

### Chap06/DirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 25 | `empty` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;129 |
| 26 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;142 |
| 27 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 146&#8209;147 |
| 28 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 151&#8209;152 |
| 29 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 30 | `sizeA` | Y | Y |  |  | Y |  |  | unknown | 162&#8209;164 |
| 31 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 168&#8209;170 |
| 32 | `ng` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |
| 33 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |
| 34 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 186&#8209;188 |
| 35 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 192&#8209;194 |
| 36 | `n_plus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 198&#8209;200 |
| 37 | `n_minus_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 38 | `incident` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;212 |
| 39 | `degree` | Y | Y |  |  | Y |  |  | unknown | 216&#8209;218 |
| 40 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 222&#8209;224 |
| 41 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;230 |
| 42 | `iter_vertices` |  |  | Y |  | Y |  |  | unknown | 239&#8209;246 |
| 43 | `iter_arcs` |  |  | Y |  | Y |  |  | unknown | 251&#8209;258 |
| 44 | `eq` |  | Y |  |  | Y |  |  | unknown | 709&#8209;710 |

### Chap06/LabDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `empty` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;128 |
| 46 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 132&#8209;141 |
| 47 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 145&#8209;146 |
| 48 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;151 |
| 49 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;157 |
| 50 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | hole | 760&#8209;767 |
| 51 | `add_labeled_arc` x3 | Y | Y |  |  | Y |  |  | hole | 770&#8209;778 |
| 52 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 176&#8209;180 |
| 53 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 184&#8209;186 |
| 54 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 781&#8209;788 |
| 55 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 791&#8209;798 |
| 56 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 244&#8209;254 |
| 57 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 258&#8209;268 |
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 749&#8209;757 |

### Chap06/LabDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 59 | `empty` | Y | Y |  |  | Y |  |  | unknown | 105&#8209;111 |
| 60 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 115&#8209;124 |
| 61 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 62 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;134 |
| 63 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 138&#8209;140 |
| 64 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;146 |
| 65 | `add_labeled_arc` | Y | Y |  |  | Y |  |  | unknown | 150&#8209;154 |
| 66 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 158&#8209;162 |
| 67 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 166&#8209;168 |
| 68 | `n_plus` | Y | Y |  |  | Y |  |  | unknown | 173&#8209;175 |
| 69 | `n_minus` | Y | Y |  |  | Y |  |  | unknown | 180&#8209;182 |

### Chap06/LabUnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 708&#8209;713 |
| 71 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 143&#8209;152 |
| 72 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 716&#8209;717 |
| 73 | `labeled_edges` x3 | Y | Y |  |  | Y |  |  | unknown | 720&#8209;721 |
| 74 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 724&#8209;727 |
| 75 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | hole | 745&#8209;749 |
| 76 | `add_labeled_edge` x3 | Y | Y |  |  | Y |  |  | hole | 752&#8209;757 |
| 77 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 189&#8209;195 |
| 78 | `has_edge` x3 | Y | Y |  |  | Y |  |  | unknown | 730&#8209;734 |
| 79 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 737&#8209;742 |
| 80 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 229&#8209;240 |

### Chap06/LabUnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 81 | `empty` | Y | Y |  |  | Y |  |  | unknown | 102&#8209;108 |
| 82 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 112&#8209;121 |
| 83 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 125&#8209;126 |
| 84 | `labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;131 |
| 85 | `edges` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;138 |
| 86 | `add_vertex` | Y | Y |  |  | Y |  |  | unknown | 142&#8209;144 |
| 87 | `add_labeled_edge` | Y | Y |  |  | Y |  |  | unknown | 148&#8209;153 |
| 88 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 157&#8209;163 |
| 89 | `has_edge` | Y | Y |  |  | Y |  |  | unknown | 167&#8209;170 |
| 90 | `ng` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;176 |

### Chap06/UnDirGraphMtEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 91 | `empty` | Y | Y |  |  | Y |  |  | unknown | 137&#8209;143 |
| 92 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;156 |
| 93 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 528&#8209;530 |
| 94 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 533&#8209;535 |
| 95 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 538&#8209;540 |
| 96 | `sizeE` x3 | Y | Y |  |  | Y |  |  | unknown | 543&#8209;545 |
| 97 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 548&#8209;553 |
| 98 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 556&#8209;563 |
| 99 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 566&#8209;573 |
| 100 | `incident` | Y | Y |  |  | Y |  |  | unknown | 228&#8209;230 |
| 101 | `degree` | Y | Y |  |  | Y |  |  | unknown | 234&#8209;239 |
| 102 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 243&#8209;253 |
| 103 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 257&#8209;267 |
| 104 | `new` | Y | Y |  |  | Y |  |  | unknown | 517&#8209;525 |
| 105 | `eq` |  | Y |  |  | Y |  |  | unknown | 721&#8209;722 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `empty` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;115 |
| 107 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;127 |
| 108 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 131&#8209;132 |
| 109 | `edges` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;137 |
| 110 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 141&#8209;143 |
| 111 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 147&#8209;149 |
| 112 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 153&#8209;159 |
| 113 | `ng` | Y | Y |  |  | Y |  |  | unknown | 163&#8209;170 |
| 114 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 174&#8209;181 |
| 115 | `incident` | Y | Y |  |  | Y |  |  | unknown | 185&#8209;187 |
| 116 | `degree` | Y | Y |  |  | Y |  |  | unknown | 191&#8209;196 |
| 117 | `eq` |  | Y |  |  | Y |  |  | unknown | 437&#8209;438 |

### Chap06/WeightedDirGraphStEphF64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 118 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 70&#8209;76 |
| 119 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 80&#8209;84 |
| 120 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 88&#8209;92 |
| 121 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;99 |
| 122 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;107 |
| 123 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 111&#8209;115 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 124 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 125 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 126 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 127 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 128 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 129 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 130 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 131 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 132 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 133 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 134 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 135 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 136 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 137 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 138 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 139 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 140 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 141 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 142 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 143 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 144 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 145 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 146 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 147 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 148 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 149 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 150 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 151 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 152 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 153 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 154 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 155 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 156 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 157 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 158 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 159 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 160 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 161 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 162 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 163 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 164 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 165 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 166 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 167 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 168 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 169 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 170 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 171 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 172 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 173 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 174 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 175 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 176 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 177 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 178 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 179 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 180 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 181 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 182 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 183 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 184 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 185 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 186 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 187 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 188 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 189 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 190 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 191 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 192 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 193 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 194 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 195 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 196 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 197 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 198 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 199 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 200 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 201 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 202 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 203 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 204 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 205 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 206 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 207 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 208 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 209 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 210 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 211 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 212 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 213 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 214 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 215 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 216 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 217 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 218 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 219 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 220 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 221 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 222 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 223 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 73&#8209;79 |
| 224 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 83&#8209;87 |
| 225 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 91&#8209;95 |
| 226 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 99&#8209;102 |
| 227 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 106&#8209;110 |
| 228 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;118 |
| 229 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;124 |
| 230 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;132 |
| 231 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 136&#8209;140 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
