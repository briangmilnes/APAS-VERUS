# Chap06 Spec Audit — Graph Theory ADTs

Audited: 2026-03-15, Agent 4, Round 19.
Prose source: prompts/Chap06.txt (Definitions 6.1-6.17).

## Summary

All 20 files, 0 holes, all specs **strong**. No changes needed.

Chap06 implements four graph ADT families (Def 6.1-6.4, 6.17): DirGraph, UnDirGraph,
LabDirGraph, LabUnDirGraph, plus 12 WeightedDirGraph weight-type variants, and 4 MtEph
parallel wrappers. Every exec function has precise set-level ensures that directly encode
the textbook definitions.

## Per-File Classification

| # | File | Holes | Fns | Classification | Notes |
|---|------|:-----:|:---:|:--------------:|-------|
| 1 | DirGraphStEph.rs | 0 | 17 | Strong | Def 6.1/6.3/6.4/6.5/6.6 |
| 2 | UnDirGraphStEph.rs | 0 | 11 | Strong | Def 6.2/6.3/6.4/6.5/6.6 |
| 3 | LabDirGraphStEph.rs | 0 | 11 | Strong | Def 6.17 directed labeled |
| 4 | LabUnDirGraphStEph.rs | 0 | 10 | Strong | Def 6.17 undirected labeled |
| 5 | WeightedDirGraphStEphI64.rs | 0 | 9 | Strong | Def 6.17 weighted (i64) |
| 6 | WeightedDirGraphStEphI32.rs | 0 | 9 | Strong | Same pattern, i32 |
| 7 | WeightedDirGraphStEphI128.rs | 0 | 9 | Strong | Same pattern, i128 |
| 8 | WeightedDirGraphStEphI16.rs | 0 | 9 | Strong | Same pattern, i16 |
| 9 | WeightedDirGraphStEphI8.rs | 0 | 9 | Strong | Same pattern, i8 |
| 10 | WeightedDirGraphStEphIsize.rs | 0 | 9 | Strong | Same pattern, isize |
| 11 | WeightedDirGraphStEphU128.rs | 0 | 9 | Strong | Same pattern, u128 |
| 12 | WeightedDirGraphStEphU64.rs | 0 | 9 | Strong | Same pattern, u64 |
| 13 | WeightedDirGraphStEphU32.rs | 0 | 9 | Strong | Same pattern, u32 |
| 14 | WeightedDirGraphStEphU16.rs | 0 | 9 | Strong | Same pattern, u16 |
| 15 | WeightedDirGraphStEphU8.rs | 0 | 9 | Strong | Same pattern, u8 |
| 16 | WeightedDirGraphStEphUsize.rs | 0 | 9 | Strong | Same pattern, usize |
| 17 | DirGraphMtEph.rs | 0 | 24 | Strong | Parallel DirGraph wrapper |
| 18 | UnDirGraphMtEph.rs | 0 | 15 | Strong | Parallel UnDirGraph wrapper |
| 19 | LabDirGraphMtEph.rs | 0 | 14 | Strong | Parallel LabDirGraph wrapper |
| 20 | LabUnDirGraphMtEph.rs | 0 | 11 | Strong | Parallel LabUnDirGraph wrapper |

## Spec-vs-Prose Detail (DirGraphStEph — representative)

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `empty` | V = {}, A = {} | Def 6.1: G=(V,A) | Strong |
| 2 | `from_sets` | g@.V =~= V, g@.A =~= A | Def 6.1 | Strong |
| 3 | `vertices` | v@ == self@.V | Def 6.1: V | Strong |
| 4 | `arcs` | a@ =~= self@.A | Def 6.1: A | Strong |
| 5 | `sizeV` | n == self@.V.len() | Def 6.15: n=\|V\| | Strong |
| 6 | `sizeA` | n == self@.A.len() | Def 6.15: m=\|A\| | Strong |
| 7 | `neighbor` | b == A.contains((u,v)) | Def 6.3 | Strong |
| 8 | `n_plus` | result == spec_n_plus(v) | Def 6.4: N+(v) | Strong |
| 9 | `n_minus` | result == spec_n_minus(v) | Def 6.4: N-(v) | Strong |
| 10 | `ng` | result == spec_ng(v) | Def 6.4: N_G(v) | Strong |
| 11 | `n_plus_of_vertices` | result == spec_n_plus_of_vertices(U) | Def 6.4: N+(U) | Strong |
| 12 | `n_minus_of_vertices` | result == spec_n_minus_of_vertices(U) | Def 6.4: N-(U) | Strong |
| 13 | `ng_of_vertices` | result == spec_ng_of_vertices(U) | Def 6.4: N_G(U) | Strong |
| 14 | `incident` | b == (e.0 == v \|\| e.1 == v) | Def 6.5 | Strong |
| 15 | `degree` | n == spec_degree(v) = \|N_G(v)\| | Def 6.6 | Strong |
| 16 | `in_degree` | n == \|N-(v)\| | Def 6.6: d-(v) | Strong |
| 17 | `out_degree` | n == \|N+(v)\| | Def 6.6: d+(v) | Strong |

Spec functions encode definitions precisely:
- `spec_n_plus(v)` = `{w | (v,w) in A}` — exactly N+(v) per Def 6.4.
- `spec_n_minus(v)` = `{u | (u,v) in A}` — exactly N-(v) per Def 6.4.
- `spec_ng(v)` = N+(v) union N-(v) — Def 6.4.
- `spec_degree(v)` = |N_G(v)| — Def 6.6.

The UnDirGraph variant correctly handles both edge directions: `(v,w) || (w,v)`.
The Labeled variants use existential quantification over labels.
The Weighted variants add `total_weight`, `edges_above/below_weight` with precise
set-level specs.

## Verdict

No spec changes needed. All Chap06 specs faithfully encode APAS definitions 6.1-6.17.
