# R44 Agent 3: Chap66 + Chap65 (12 holes)

## Assignment

Prove external_body functions in Chap66 (Boruvka parallel MST) and Chap65
(Kruskal + Prim sequential MST). Also fix fn_missing_requires/ensures warnings.

## Baseline

125 holes total. 4366 verified. Your chapters: Chap66 (10), Chap65 (2).

## Target Holes

| # | Chap | File | Function | Line | Type |
|---|------|------|----------|------|------|
| 1 | 66 | BoruvkaMtEph.rs | hash_coin_flips_mt | 140 | external_body |
| 2 | 66 | BoruvkaMtEph.rs | compute_remaining_mt | 175 | external_body |
| 3 | 66 | BoruvkaMtEph.rs | collect_mst_labels_mt | 213 | external_body |
| 4 | 66 | BoruvkaMtEph.rs | build_partition_map_mt | 251 | external_body |
| 5 | 66 | BoruvkaMtEph.rs | vertex_bridges_mt | 296 | external_body |
| 6 | 66 | BoruvkaMtEph.rs | bridge_star_partition_mt | 352 | external_body |
| 7 | 66 | BoruvkaMtEph.rs | filter_tail_to_head_mt | 391 | external_body |
| 8 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt | 447 | external_body |
| 9 | 66 | BoruvkaMtEph.rs | reroute_edges_mt | 506 | external_body |
| 10 | 66 | BoruvkaMtEph.rs | boruvka_mst_mt_with_seed | 552 | external_body |
| 11 | 65 | KruskalStEph.rs | kruskal_mst | 62 | external_body |
| 12 | 65 | PrimStEph.rs | prim_mst | 101 | external_body |

Also fix these warnings:
- Chap66/BoruvkaMtEph.rs: `mst_weight` fn_missing_ensures (line 566)
- Chap65/KruskalStEph.rs: `mst_weight` fn_missing_ensures (line 103)
- Chap65/KruskalStEph.rs: `verify_mst_size` fn_missing_ensures (line 128)
- Chap65/PrimStEph.rs: `pq_entry_new` fn_missing_requires (line 78)
- Chap65/PrimStEph.rs: `mst_weight` fn_missing_ensures (line 157)

## Strategy

### Chap65 — Start here (simpler)

**KruskalStEph.rs**: `kruskal_mst` uses `UnionFindStEph` for union-find and iterates
over sorted edges. The implementation is complete — remove external_body and add loop
invariants. The function uses `SetLit![]`, `HashMapWithViewPlus`, and `UnionFindStEph`
— all Verus-compatible.

**PrimStEph.rs**: `prim_mst` uses `BinaryHeapPQ` and `HashSetWithViewPlus`. More complex
— priority queue operations in a while loop. Try removing external_body; if the PQ
operations don't have strong enough ensures, you may need to keep external_body.

### Chap66 — Boruvka (hardest)

BoruvkaMtEph.rs has 10 external_body functions implementing parallel Boruvka's MST.
Many use `ParaPair!` for fork-join parallelism and `hash_coin` for deterministic
random coin flips.

**Triage by difficulty:**

- **Easy** (loop-based helpers): `compute_remaining_mt`, `collect_mst_labels_mt`,
  `build_partition_map_mt`, `filter_tail_to_head_mt` — sequential loops that can use
  the iterator proof pattern.
- **Medium** (parallel fork-join): `hash_coin_flips_mt`, `vertex_bridges_mt`,
  `reroute_edges_mt` — use `ParaPair!` for divide-and-conquer. The spawn boundary
  needs external_body but the logic around it may be provable.
- **Hard** (main algorithm): `boruvka_mst_mt`, `boruvka_mst_mt_with_seed`,
  `bridge_star_partition_mt` — complex recursive/iterative algorithms.

Start with the easy helpers, then medium, then hard.

### Proof patterns:

1. **Iterator loops**: `for x in collection.iter()` with invariant.
2. **clone_plus()**: `use crate::vstdplus::clone_plus::clone_plus::*;`
3. **SetStEph::empty()** / **SetLit![]**: For initial empty sets.
4. **ParaPair! spawn boundary**: Keep external_body on functions that use `ParaPair!`
   for actual thread spawning. Factor verifiable logic out of the spawn.
5. **hash_coin**: This function is in BoruvkaMtEph itself — it's the deterministic
   coin flip for parallel code. Leave it as-is (it's not a hole).

### What NOT to do:
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel (Mt) implementations — keep ParaPair! calls.
- Do NOT replace HashMapWithViewPlus with std::collections::HashMap.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes. Run `scripts/holes.sh src/Chap66/ src/Chap65/`.
Write your report to `plans/agent3-round44-report.md`.
