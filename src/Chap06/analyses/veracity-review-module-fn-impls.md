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
| 3 | Chap06 | LabDirGraphMtEph | 14 | 14 | 0 | 0 | 14 | 0 | 14 | 0 | 0 |
| 4 | Chap06 | LabDirGraphStEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 5 | Chap06 | LabUnDirGraphMtEph | 11 | 11 | 0 | 0 | 11 | 0 | 11 | 0 | 0 |
| 6 | Chap06 | LabUnDirGraphStEph | 10 | 10 | 0 | 0 | 10 | 0 | 10 | 0 | 0 |
| 7 | Chap06 | UnDirGraphMtEph | 14 | 15 | 0 | 0 | 15 | 0 | 15 | 0 | 0 |
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
| 1 | `empty` | Y | Y |  |  | Y |  |  | unknown | 101&#8209;106 |
| 2 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 110&#8209;120 |
| 3 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 742&#8209;744 |
| 4 | `arcs` x3 | Y | Y |  |  | Y |  |  | unknown | 746&#8209;748 |
| 5 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 750&#8209;752 |
| 6 | `sizeA` x3 | Y | Y |  |  | Y |  |  | unknown | 754&#8209;756 |
| 7 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 758&#8209;763 |
| 8 | `incident` | Y | Y |  |  | Y |  |  | unknown | 156&#8209;158 |
| 9 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 765&#8209;772 |
| 10 | `out_degree` | Y | Y |  |  | Y |  |  | unknown | 188&#8209;193 |
| 11 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 774&#8209;781 |
| 12 | `in_degree` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;228 |
| 13 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 783&#8209;789 |
| 14 | `degree` | Y | Y |  |  | Y |  |  | unknown | 256&#8209;261 |
| 15 | `n_plus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 791&#8209;797 |
| 16 | `n_minus_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 799&#8209;805 |
| 17 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 807&#8209;813 |
| 18 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 336&#8209;346 |
| 19 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 349&#8209;359 |
| 20 | `n_plus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 362&#8209;372 |
| 21 | `n_minus_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 375&#8209;385 |
| 22 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 388&#8209;398 |
| 23 | `new` | Y | Y |  |  | Y |  |  | unknown | 730&#8209;740 |
| 24 | `eq` |  | Y |  |  | Y |  |  | unknown | 943&#8209;944 |

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
| 45 | `empty` | Y | Y |  |  | Y |  |  | unknown | 100&#8209;105 |
| 46 | `from_vertices_and_labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 109&#8209;119 |
| 47 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 123&#8209;124 |
| 48 | `labeled_arcs` | Y | Y |  |  | Y |  |  | unknown | 128&#8209;129 |
| 49 | `arcs` | Y | Y |  |  | Y |  |  | unknown | 133&#8209;136 |
| 50 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | unknown | 658&#8209;665 |
| 51 | `add_labeled_arc` x3 | Y | Y |  |  | Y |  |  | unknown | 667&#8209;675 |
| 52 | `get_arc_label` | Y | Y |  |  | Y |  |  | unknown | 155&#8209;160 |
| 53 | `has_arc` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 54 | `n_plus` x3 | Y | Y |  |  | Y |  |  | unknown | 677&#8209;683 |
| 55 | `n_minus` x3 | Y | Y |  |  | Y |  |  | unknown | 685&#8209;691 |
| 56 | `n_plus_par` | Y | Y |  |  | Y |  |  | unknown | 223&#8209;233 |
| 57 | `n_minus_par` | Y | Y |  |  | Y |  |  | unknown | 236&#8209;246 |
| 58 | `new` | Y | Y |  |  | Y |  |  | unknown | 646&#8209;656 |

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
| 70 | `empty` x3 | Y | Y |  |  | Y |  |  | unknown | 603&#8209;608 |
| 71 | `from_vertices_and_labeled_edges` | Y | Y |  |  | Y |  |  | unknown | 117&#8209;127 |
| 72 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 610&#8209;611 |
| 73 | `labeled_edges` x3 | Y | Y |  |  | Y |  |  | unknown | 613&#8209;614 |
| 74 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 616&#8209;619 |
| 75 | `add_vertex` x3 | Y | Y |  |  | Y |  |  | unknown | 633&#8209;637 |
| 76 | `add_labeled_edge` x3 | Y | Y |  |  | Y |  |  | unknown | 639&#8209;640 |
| 77 | `get_edge_label` | Y | Y |  |  | Y |  |  | unknown | 160&#8209;165 |
| 78 | `has_edge` x3 | Y | Y |  |  | Y |  |  | unknown | 621&#8209;625 |
| 79 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 627&#8209;631 |
| 80 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 195&#8209;206 |

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
| 91 | `empty` | Y | Y |  |  | Y |  |  | unknown | 113&#8209;118 |
| 92 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 122&#8209;132 |
| 93 | `vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 499&#8209;501 |
| 94 | `edges` x3 | Y | Y |  |  | Y |  |  | unknown | 503&#8209;505 |
| 95 | `sizeV` x3 | Y | Y |  |  | Y |  |  | unknown | 507&#8209;509 |
| 96 | `sizeE` x3 | Y | Y |  |  | Y |  |  | unknown | 511&#8209;513 |
| 97 | `neighbor` x3 | Y | Y |  |  | Y |  |  | unknown | 515&#8209;520 |
| 98 | `ng` x3 | Y | Y |  |  | Y |  |  | unknown | 522&#8209;528 |
| 99 | `ng_of_vertices` x3 | Y | Y |  |  | Y |  |  | unknown | 530&#8209;536 |
| 100 | `incident` | Y | Y |  |  | Y |  |  | unknown | 204&#8209;206 |
| 101 | `degree` | Y | Y |  |  | Y |  |  | unknown | 210&#8209;215 |
| 102 | `ng_par` | Y | Y |  |  | Y |  |  | unknown | 218&#8209;228 |
| 103 | `ng_of_vertices_par` | Y | Y |  |  | Y |  |  | unknown | 231&#8209;241 |
| 104 | `new` | Y | Y |  |  | Y |  |  | unknown | 487&#8209;497 |
| 105 | `eq` |  | Y |  |  | Y |  |  | unknown | 630&#8209;631 |

### Chap06/UnDirGraphStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 106 | `empty` | Y | Y |  |  | Y |  |  | unknown | 95&#8209;100 |
| 107 | `from_sets` | Y | Y |  |  | Y |  |  | unknown | 103&#8209;110 |
| 108 | `vertices` | Y | Y |  |  | Y |  |  | unknown | 114&#8209;115 |
| 109 | `edges` | Y | Y |  |  | Y |  |  | unknown | 119&#8209;120 |
| 110 | `sizeV` | Y | Y |  |  | Y |  |  | unknown | 124&#8209;126 |
| 111 | `sizeE` | Y | Y |  |  | Y |  |  | unknown | 130&#8209;132 |
| 112 | `neighbor` | Y | Y |  |  | Y |  |  | unknown | 135&#8209;141 |
| 113 | `ng` | Y | Y |  |  | Y |  |  | unknown | 144&#8209;151 |
| 114 | `ng_of_vertices` | Y | Y |  |  | Y |  |  | unknown | 154&#8209;161 |
| 115 | `incident` | Y | Y |  |  | Y |  |  | unknown | 164&#8209;166 |
| 116 | `degree` | Y | Y |  |  | Y |  |  | unknown | 169&#8209;174 |
| 117 | `eq` |  | Y |  |  | Y |  |  | unknown | 358&#8209;359 |

### Chap06/WeightedDirGraphStEphI128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 118 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 119 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 120 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 121 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 122 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 123 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 124 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 125 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 126 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 127 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 128 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 129 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 130 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 131 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 132 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 133 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 134 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 135 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 136 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 137 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 138 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 139 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 140 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 141 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 142 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 143 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 144 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 145 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 146 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 147 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 148 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 149 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 150 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 151 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 152 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 153 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphI8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 154 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 155 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 156 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 157 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 158 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 159 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 160 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 161 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 162 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphIsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 163 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 164 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 165 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 166 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 167 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 168 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 169 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 170 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 171 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU128.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 172 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 173 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 174 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 175 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 176 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 177 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 178 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 179 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 180 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU16.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 181 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 182 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 183 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 184 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 185 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 186 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 187 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 188 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 189 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU32.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 190 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 191 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 192 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 193 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 194 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 195 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 196 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 197 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 198 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU64.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 199 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 200 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 201 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 202 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 203 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 204 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 205 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 206 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 207 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphU8.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 208 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 209 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 210 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 211 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 212 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 213 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 214 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 215 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 216 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |

### Chap06/WeightedDirGraphStEphUsize.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 217 | `from_weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 42&#8209;49 |
| 218 | `add_weighed_edge` | Y | Y |  |  | Y |  |  | unknown | 52&#8209;56 |
| 219 | `get_edge_weight` | Y | Y |  |  | Y |  |  | unknown | 59&#8209;63 |
| 220 | `weighed_edges` | Y | Y |  |  | Y |  |  | unknown | 66&#8209;69 |
| 221 | `out_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 72&#8209;76 |
| 222 | `in_neighbors_weighed` | Y | Y |  |  | Y |  |  | unknown | 79&#8209;83 |
| 223 | `total_weight` | Y | Y |  |  | Y |  |  | unknown | 86&#8209;88 |
| 224 | `edges_above_weight` | Y | Y |  |  | Y |  |  | unknown | 90&#8209;94 |
| 225 | `edges_below_weight` | Y | Y |  |  | Y |  |  | unknown | 96&#8209;100 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
