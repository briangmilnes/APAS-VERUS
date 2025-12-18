# ParaPairDisjoint Usage

When splitting **arcs/edges**, parallel results are disjoint.
When splitting **vertices**, results may overlap.

## Can Use `ParaPairDisjoint!` + `disjoint_union`

| File | Function | Splits |
|------|----------|--------|
| `DirGraphMtEph.rs` | `parallel_nplus` | arcs |
| `DirGraphMtEph.rs` | `parallel_nminus` | arcs |
| `UnDirGraphMtEph.rs` | `parallel_ng` | edges |
| `LabDirGraphMtEph.rs` | `parallel_out` | arcs |
| `LabDirGraphMtEph.rs` | `parallel_in` | arcs |
| `LabUnDirGraphMtEph.rs` | `parallel_neighbors` | edges |

## Must Use `ParaPair!` + `union`

| File | Function | Splits | Why Not Disjoint |
|------|----------|--------|------------------|
| `DirGraphMtEph.rs` | `parallel_ng_of_vertices` | vertices | Same vertex can neighbor v₁ and v₂ |
| `DirGraphMtEph.rs` | `parallel_nplus_of_vertices` | vertices | Same vertex can be out-neighbor of multiple |
| `DirGraphMtEph.rs` | `parallel_nminus_of_vertices` | vertices | Same vertex can be in-neighbor of multiple |
| `UnDirGraphMtEph.rs` | `parallel_ng_of_vertices` | vertices | Same vertex can neighbor multiple |

