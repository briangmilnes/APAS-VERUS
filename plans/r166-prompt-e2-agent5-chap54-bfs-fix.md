# R166 Prompt E2 — Agent 5: Fix BFS spec_index→@ notation and complete BFSSpecsAndLemmas extraction. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent5`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Problem

Chap54 BFS files use `spec_index` notation in their spec fns:
```rust
pub open spec fn spec_bfssteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> bool
pub open spec fn spec_distances_bounded(distances: &ArraySeqStEphS<usize>, n: int) -> bool
pub open spec fn spec_parents_bounded(parents: &ArraySeqStEphS<usize>, n: int) -> bool
```

Meanwhile `BFSSpecsAndLemmas.rs` has equivalent specs using `@` (Seq) notation:
```rust
pub open spec fn spec_bfs_graph_wf(graph: Seq<Seq<usize>>) -> bool
pub open spec fn spec_bfs_distances_bounded(distances: Seq<usize>, n: int) -> bool
pub open spec fn spec_bfs_parents_bounded(parents: Seq<usize>, n: int) -> bool
```

The BFS files can't use the shared specs because of this notation mismatch.
Agent4 punted and left bridge wrappers. Your job: fix it properly.

## Goal

1. Migrate the 4 BFS files from `spec_index`-based spec fns to `@`-based
   spec fns that call the shared versions in `BFSSpecsAndLemmas.rs`.
2. Remove the local duplicate spec fns.
3. Remove the bridge wrapper lemmas that agent4 added.
4. Ensure the shared specs in `BFSSpecsAndLemmas.rs` are actually used.

## Files

- `src/Chap54/BFSSpecsAndLemmas.rs` — shared specs (already exists, may need adjustment)
- `src/Chap54/BFSStEph.rs` — migrate spec_index → @
- `src/Chap54/BFSStPer.rs` — migrate spec_index → @
- `src/Chap54/BFSMtEph.rs` — migrate spec_index → @
- `src/Chap54/BFSMtPer.rs` — migrate spec_index → @

## Approach

**Step 1 — Read all 5 files and all standards.**

**Step 2 — Understand the notation gap.** `spec_index` on ArraySeqStEphS
returns the same value as `@` indexing — it's just a different surface syntax.
The local spec fns like `spec_bfssteph_wf(graph)` can be rewritten as:
```rust
pub open spec fn spec_bfssteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> bool {
    spec_bfs_graph_wf(graph@)  // delegate to shared version
}
```
Or better: remove the local spec fn entirely and replace all call sites
with `spec_bfs_graph_wf(graph@)`.

**Step 3 — Migrate one file at a time.** Start with BFSStEph.rs:
- Replace local `spec_bfssteph_wf` with calls to `spec_bfs_graph_wf(graph@)`
- Replace local `spec_distances_bounded` with `spec_bfs_distances_bounded(distances@, n)`
- Replace local `spec_parents_bounded` with `spec_bfs_parents_bounded(parents@, n)`
- Update all requires/ensures clauses that reference these
- Remove the local spec fn definitions
- Remove any bridge wrapper lemmas
- Validate: `scripts/validate.sh isolate Chap54`

**Step 4 — Repeat for BFSStPer, BFSMtEph, BFSMtPer.**

**Step 5 — Clean up BFSSpecsAndLemmas.rs.** Remove anything that's no longer
needed (bridge lemmas, redundant specs). The file should contain only the
canonical `@`-based specs and any shared proof lemmas.

## Validation

```bash
scripts/validate.sh isolate Chap54
```

## Report

Write `plans/agent5-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 5: fix BFS spec_index→@ notation, complete SpecsAndLemmas extraction (−N lines)"`, then `git push`.
