# R89 Agent 4 — Prove Chap62 Holes (StarPartition + StarContraction), STEP 20

## Objective

Remove external_body from 2 functions:
1. `parallel_star_partition` in `src/Chap62/StarPartitionMtEph.rs`
2. `route_edges_parallel` in `src/Chap62/StarContractionMtEph.rs`

## The 2 Holes

### 1. StarPartitionMtEph::parallel_star_partition

This is the big one. The function has 6 loops building a star partition:
1. Build vertex-to-index map
2. Flip coins for each vertex
3. Build tail-heads edges (tails adjacent to heads)
4. Pick center for each tail (first head neighbor)
5. Self-assign centers (heads are their own center)
6. Build final partition map + centers set

The proof body is already written inside the external_body — ~200 lines of loop
invariants and intermediate assertions. The R88 agent1 comment says: "proof
invariants (no_duplicates, to_seq postconditions) need to be threaded through 6
loops — significant proof work that was never completed."

**Strategy:** Remove the `#[verifier::external_body]` and see what fails. The
proof body is preserved — it may be close to working. Fix the failures. The
main challenge is threading `no_duplicates` and set-membership invariants through
all 6 loops, and proving the final `spec_valid_partition_map` postcondition.

### 2. StarContractionMtEph::route_edges_parallel

Recursive divide-and-conquer that routes edges through a partition map. Takes
`Arc<ArraySeqStEphS<Edge<V>>>` and `Arc<HashMapWithViewPlus<V, V>>`, splits at
midpoint, recurses, unions results.

The R88 agent1 comment says: "Ghost captures not Send; need Verus fix or
restructuring." The function was originally using `ParaPair!` for parallel
recursion but `Set<V::V>` ghost values aren't `Send`. It was changed to
sequential recursive calls.

**Strategy:** Since the recursive calls are already sequential (not ParaPair!),
the Send issue is gone. Remove external_body and see what remains. The recursive
structure with Arc cloning and ghost parameter passing should verify — the
difficult part is proving that the union of left/right results covers all edges
in [start, end).

## CRITICAL: Read the working StEph versions first

Both StEph versions are fully proved (0 holes). They are your primary reference
for the proof strategy, invariants, and spec functions.

- `src/Chap62/StarPartitionStEph.rs` — **0 holes, fully proved** — read this FIRST
- `src/Chap62/StarContractionStEph.rs` — **0 holes, fully proved** — read this FIRST
- `src/Chap62/StarPartitionMtEph.rs` — your file (read ALL of it, the proof body is inside external_body)
- `src/Chap62/StarContractionMtEph.rs` — your file

## Isolation

```bash
scripts/validate.sh isolate Chap64
```

(Chap64 pulls in Chap62 transitively.)

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify files outside Chap62.
- Do NOT add assume or accept.
- Prioritize `route_edges_parallel` first — it's likely easier (sequential
  recursion with Arc, no complex 6-loop invariant chain).
- For `parallel_star_partition`, start by removing external_body and reading
  the errors. The proof body is already there — it may just need a few more
  assertions or invariant clauses.
- If a function is genuinely too hard within 20 steps, leave it external_body
  and report exactly where the proof gets stuck.

## STEP 20

## Report

Write `plans/agent4-r89-chap62-report.md`.
