# R128 Agent 3b — Parallelize Chap62 StarPartitionMtEph. AFK.

## Background

`src/Chap62/StarPartitionMtEph.rs` (737 lines) implements Algorithm 62.3: Parallel
Star Partition. Despite the "Mt" name, the current implementation uses 6 sequential
loops where APAS expects O(lg n) span via parallelism.

The sibling file `src/Chap62/StarContractionMtEph.rs` already uses `ParaPair!` for
parallel helpers — check it for patterns used in this chapter.

Also check `src/Chap66/BoruvkaMtEph.rs` which uses parallel D&C helpers
(`hash_coin_flips_mt`, `compute_remaining_mt`, `collect_mst_labels_mt`) via
`ParaPair!` for very similar operations (coin flips, vertex filtering, label
collection). Boruvka's helpers are the closest existing pattern.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs`
2. `src/standards/hfscheduler_standard.rs`
3. `src/standards/arc_usage_standard.rs` — Arc needed for sharing read-only data across join arms

## The algorithm's 6 loops

Read the function `parallel_star_partition` carefully. The 6 sequential loops are:

1. **Loop 1** (line ~113): Build vertex-to-index map. Sequential scan of vertices.
2. **Loop 2** (line ~182): Flip coins for each vertex. Sequential RNG.
3. **Loop 3** (line ~218): Build tail→heads edges from edge list + coin flips. Sequential scan.
4. **Loop 4** (line ~392): Initialize p_vec = vertices_vec (copy).
5. **Loop 5**: Update p_vec from th_edges (priority-based write).
6. **Loop 6**: Build centers set and partition map from p_vec + coin_flips.

## Which loops can be parallelized

| Loop | Parallelizable? | Pattern | Notes |
|------|----------------|---------|-------|
| 1 | Yes | Parallel tabulate | Map vertex → index. But HashMap construction is inherently sequential (each insert depends on prior state). Alternative: use parallel array + index. |
| 2 | Yes | Parallel tabulate with hash-based coins | See Boruvka's `hash_coin_flips_mt` — uses vertex hash instead of sequential RNG. This is the key insight: replace sequential RNG with deterministic hash-based coin flips that can be computed in parallel. |
| 3 | Yes | Parallel filter/map over edges | Each edge independently checks coin flips of endpoints. Read-only access to coin_flips. |
| 4 | Yes | Parallel copy / tabulate | Trivial parallel copy. |
| 5 | Maybe | Parallel write with conflict resolution | Each tail writes its head's index. Conflicts possible if multiple tails share a head. Needs atomic-style priority resolution or sequential fallback. |
| 6 | Yes | Parallel filter + map | Split vertices into centers (coin=heads) and non-centers, build partition map entries in parallel. |

## Approach

Focus on the loops that give the most span reduction with the least proof complexity:

1. **Loop 2 (coin flips)**: Replace sequential RNG with hash-based deterministic coins
   (like Boruvka). This is the highest-value change — it removes the sequential RNG
   dependency chain.

2. **Loop 3 (edge classification)**: Parallelize with D&C over the edge array. Each
   edge independently checks coin_flips (read-only). Wrap coin_flips in Arc for sharing
   across join arms.

3. **Loop 6 (center/partition construction)**: Parallelize with D&C over vertex array.
   Each vertex independently checks its coin flip.

4. **Loops 1, 4, 5**: Leave sequential if the parallel version would be complex for
   marginal span improvement. Document why in the DIFFERS annotation.

## Validation

Run `scripts/validate.sh isolate Chap62` after changes. Fix verification errors.
Then run `scripts/rtt.sh`.

## Rules

- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses.
- Preserve all existing RTTs.
- If a loop can't be parallelized, update the DIFFERS annotation with an accurate reason.
- Check `src/Chap66/BoruvkaMtEph.rs` for the hash-based coin flip pattern before writing your own.

## When done

Commit with `git add -A && git commit` and push.

## Report

Write `plans/agent3-r128b-report.md` with:
- Table of loops: # | Loop | Parallelized? | Old Span contribution | New Span | Reason if not
- Verification count
- Overall function span before/after
