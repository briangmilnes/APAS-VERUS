# R114 Agent 1 — Chap06 Spec Strengthening Report

## Objective

Strengthen locked MtEph trait specs to match StEph/inner MtEph counterparts.

## Changes Per File

| # | Chap | File | Functions Strengthened | Change |
|---|------|------|----------------------|--------|
| 1 | 06 | DirGraphMtEph.rs | ng, n_plus_of_vertices, n_minus_of_vertices, ng_of_vertices | Added 6 spec fns + functional ensures |
| 2 | 06 | UnDirGraphMtEph.rs | ng, ng_of_vertices | Added 2 spec fns + functional ensures |
| 3 | 06 | LabDirGraphMtEph.rs | n_plus, n_minus | Added 2 spec fns + functional ensures |
| 4 | 06 | LabUnDirGraphMtEph.rs | ng, add_labeled_edge | Added 1 spec fn + functional ensures |

## Detail

### DirGraphMtEph.rs — LockedDirGraphMtEphTrait

Added spec fns: `spec_n_plus`, `spec_n_minus`, `spec_ng`, `spec_n_plus_of_vertices`,
`spec_n_minus_of_vertices`, `spec_ng_of_vertices`.

Strengthened ensures:
- `ng()`: added `neighbors@ == self.spec_ng(v@)`
- `n_plus_of_vertices()`: added `out_neighbors@ == self.spec_n_plus_of_vertices(u_set@)`
- `n_minus_of_vertices()`: added `in_neighbors@ == self.spec_n_minus_of_vertices(u_set@)`
- `ng_of_vertices()`: added `neighbors@ == self.spec_ng_of_vertices(u_set@)`

### UnDirGraphMtEph.rs — LockedUnDirGraphMtEphTrait

Added spec fns: `spec_ng`, `spec_ng_of_vertices`.

Strengthened ensures:
- `ng()`: added `neighbors@ == self.spec_ng(v@)`
- `ng_of_vertices()`: added `neighbors@ == self.spec_ng_of_vertices(u_set@)`

### LabDirGraphMtEph.rs — LockedLabDirGraphMtEphTrait

Added spec fns: `spec_n_plus`, `spec_n_minus`.

Strengthened ensures:
- `n_plus()`: added `n_plus@ == self.spec_n_plus(v@)`
- `n_minus()`: added `n_minus@ == self.spec_n_minus(v@)`

### LabUnDirGraphMtEph.rs — LockedLabUnDirGraphMtEphTrait

Added spec fn: `spec_ng`.

Strengthened ensures:
- `ng()`: added `ng@ == self.spec_ng(v@)`
- `add_labeled_edge()`: added V/A update ensures matching StEph

## Verification

- Isolate Chap06: 1065 verified, 0 errors
- Full crate: 5388 verified, 0 errors
- RTT: 3529 passed
