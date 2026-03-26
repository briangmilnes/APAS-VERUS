# R83 Agent 4 — Prove union_merge + union + kruskal_mst (Chap65, 3 holes), STEP 20

## Objective

Remove `external_body` from:
1. `UnionFindStEph.rs:661` — `union_merge` (rlimit exceeded)
2. `UnionFindStEph.rs:945` — `union` (blocked by union_merge)
3. `KruskalStEph.rs:178` — `kruskal_mst`

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh` — it uses 8+ GB and will OOM.
Do NOT run `scripts/rtt.sh` or `scripts/ptt.sh` — orchestrator runs those after merge.
Push to `agent4/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## union_merge — the rlimit problem

R81 agent1 built the proof architecture:
- 3 verified sub-lemmas: `lemma_union_wf_roots`, `lemma_union_wf_parent`, `lemma_union_wf_ordering`
- `spec_union_lemma_pre` — shared precondition spec
- `union_merge` — calls all 3 lemmas + does exec mutations + ghost roots update

The function exceeds rlimit(80) because it handles 3 rank cases (lt, gt, eq) each
calling 3 lemmas in one function body. Z3 crashes above rlimit ~120.

### Fix: split by branch

Factor `union_merge` into 3 functions, one per rank case:
- `union_merge_lt(uf, root_u, root_v)` — rank_u < rank_v: root_u points to root_v
- `union_merge_gt(uf, root_u, root_v)` — rank_u > rank_v: root_v points to root_u
- `union_merge_eq(uf, root_u, root_v)` — rank_u == rank_v: root_v points to root_u, bump rank

Each function does ONE parent.insert + the ghost roots update + calls 3 sub-lemmas.
The eq case also does rank.insert. Each should fit well under rlimit(80).

Then `union_merge` becomes a thin dispatcher: read ranks, branch, call the appropriate
case function.

### The admit for rank overflow

Line 718 has `admit()` for `rank_u + 1 <= usize::MAX` in the equal-rank case.
This requires proving `rank < log2(component_size) < 64`. That needs a ghost
`component_sizes` field or an inductive argument over union history. If you can't
prove it, leave the admit and move on to union and kruskal.

## union

Once `union_merge` verifies, `union` should follow — it calls `find` (proved),
`feq` (proved), `lemma_root_is_self_parent` (proved), and `union_merge`. The
rlimit(50) may need adjustment.

## kruskal_mst

The body is already written with loop invariants and proof blocks (lines 179-293).
All called functions have full specs (union with external_body but correct ensures).
Remove `external_body`, fix loop invariants as needed. See R81 agent2 prompt for
detailed context.

### Priority

1. `union_merge` — split by branch, this is the core fix
2. `union` — should follow once merge works
3. `kruskal_mst` — independent, uses union's spec

## STEP 20

## Report

Write `plans/agent4-round83-report.md`.
