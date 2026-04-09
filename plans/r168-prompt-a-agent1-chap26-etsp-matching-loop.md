# R168 Prompt A — Agent 1: Fix ETSP lemma_combined_cycle matching loop. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**

## The hole

`src/Chap26/ETSPMtEph.rs:206` — `lemma_combined_cycle` has `#[verifier::external_body]`.
The proof body is empty. The comment says:

```
BYPASSED: rlimit — Z3 matching loop on spec_edges_form_cycle modular indexing trigger.
Same issue as ETSPStEph.rs lemma_combined_cycle.
```

The StEph version (`src/Chap26/ETSPStEph.rs`) has the same lemma — check if it's
proven there or also bypassed. If proven, adapt the proof for the MtEph version.

## The problem

`spec_edges_form_cycle` uses modular indexing: `edges[(i+1) % edges.len()].from == edges[i].to`.
The `%` operator creates a Z3 matching loop — the trigger on `edges[k]` fires on
every `edges[(expr) % n]` which generates more `%` terms, which fire more triggers.

## Approaches to try (in order)

1. **Manual trigger control.** Add an explicit `#[trigger]` on a non-modular term.
   Use a helper spec fn like `spec_cycle_connected_at(edges, i)` that takes a plain
   `int` index and has `edges[i]` as trigger instead of `edges[(i+1) % n]`.

2. **Decompose into sub-lemmas.** Instead of proving `spec_edges_form_cycle(combined)`
   directly, prove:
   - `lemma_combined_consecutive`: for each pair of adjacent indices, the edge
     endpoints connect. No modular arithmetic needed except at the wraparound.
   - `lemma_combined_wraparound`: the last edge connects to the first. One modular
     step, bounded fuel.
   
3. **Use `assert_by` with bounded fuel.** Prove each index case separately with
   `reveal_with_fuel(spec_edges_form_cycle, N)` for small N, then combine.

4. **Profile first.** If you're not sure which quantifier is looping:
   ```bash
   scripts/validate.sh isolate Chap26 --profile
   ```
   Then read `ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat`.

## Also check ETSPStEph.rs

Read `src/Chap26/ETSPStEph.rs` — it has a `lemma_combined_cycle` too.
If it's proven, study its technique. If it's also `external_body`, you need
to solve the matching loop from scratch.

## Validation

```bash
scripts/validate.sh isolate Chap26
```

## Report

Write `plans/agent1-round168-report.md`.

## RCP

`git add -A && git commit -m "R168 Agent 1: prove ETSPMtEph lemma_combined_cycle (−1 hole)"`, then `git push`.
