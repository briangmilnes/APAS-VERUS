# R85 Agent 2 — Prove kruskal_greedy_phase with lean invariant, STEP 20

## Objective

Remove `external_body` from `kruskal_greedy_phase` in `KruskalStEph.rs`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent2/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

Read the file fully. Understand what the greedy loop does and what invariants it
actually needs. Do NOT put `uf.spec_unionfindsteph_wf()` in the loop invariant —
that's 13 quantifiers and will OOM Z3.

### Why the previous attempt failed

R84 agent3 put `uf.spec_unionfindsteph_wf()` in the greedy loop invariant.
Z3 had to maintain 13 quantified conjuncts across `equals` + `union` + `insert`
calls. Z3 peaked at 19+ GB and crashed.

### The fix: lean loop invariant

The greedy loop calls:
- `uf.equals(&u, &v)` — requires wf, ensures wf + roots preserved
- `mst_edges.insert(edge)` — requires setsteph_wf, ensures setsteph_wf
- `uf.union(&u, &v)` — requires wf, ensures wf + domain preserved + roots merged

Each call's REQUIRES needs `uf.spec_unionfindsteph_wf()`. But the loop invariant
doesn't need to CARRY the full wf — it just needs facts sufficient to prove
the requires at each call site.

**Key insight**: `equals` and `union` both ensure `self.spec_unionfindsteph_wf()`.
So after each call, wf is re-established by the call's ensures. The loop invariant
only needs to carry:
- `uf.spec_unionfindsteph_wf()` — BUT as a single opaque predicate that Z3 doesn't
  unfold. If the wf is `closed` (agent1 may close it), Z3 won't unfold it.
  If it's still `open`, use `assert(uf.spec_unionfindsteph_wf())` BEFORE each call
  but don't require Z3 to maintain all 13 conjuncts through the loop body.

**Alternative approach if wf is still open**: carry only these in the invariant:
- `uf@.parent.dom() =~= initial_dom` (domain = all vertices)
- `mst_edges.spec_setsteph_wf()`
- `forall|v| graph@.V.contains(v) ==> uf@.parent.contains_key(v)` (vertices in UF)

Then at each call site, prove `uf.spec_unionfindsteph_wf()` from the fact that
the previous call (`equals` or `union`) ensured it. Use ghost snapshots:
```rust
let ghost wf_proof = uf.spec_unionfindsteph_wf(); // capture from previous ensures
```

### What kruskal_greedy_phase does

Iterates over sorted edges. For each edge (u, v, weight):
1. Check if u and v are in different components: `uf.equals(&u, &v)`
2. If different: add edge to MST, union the components

The ensures: `mst_edges.spec_setsteph_wf()`.

### Edge endpoint proof

The proof block needs to show `uf@.parent.contains_key(u@)` and
`contains_key(v@)` for each edge. R84 agent3 wrote `lemma_sorted_edge_in_uf`
for this — it chains: sort provenance → edge_seq → graph@.A → graph wf → UF domain.
This lemma is already in the file. Use it.

## Important

- Do NOT modify UnionFindStEph.rs — another agent is refactoring it.
- Do NOT modify PrimStEph.rs.
- Do NOT add `assume` or `accept`.
- Do NOT put full `spec_unionfindsteph_wf()` in the loop invariant if it's still open.

## STEP 20

## Report

Write `plans/agent2-round85-report.md`.
