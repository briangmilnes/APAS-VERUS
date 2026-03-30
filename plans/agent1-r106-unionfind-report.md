# R106 Agent 1 — UnionFind union_merge Report

## Result: union_merge PROVEN, union still external_body

| # | Chap | File | Hole | Status |
|---|------|------|------|--------|
| 1 | 65 | UnionFindStEph.rs | union_merge external_body | **CLOSED** — rlimit 30, 1.4GB Z3 |
| 2 | 65 | UnionFindStEph.rs | union external_body | OPEN — blocked by 2 issues |

Chap65 holes: 3 → 2 (1 external_body on `union`, 1 external_body + 1 opaque on KruskalStEph).

Verification: 2418 verified, 0 errors (isolate Chap65).

## Root Cause: Map =~= Matching Loop

The matching loop that previously blocked `union_merge` had two independent sources:

1. **Primary (R105 fix)**: `spec_elements_distinct` × `obeys_feq_view_injective` feedback
   loop — fixed in R105 by making feq `closed`.

2. **Secondary (this round)**: The `Map =~= old.parent.insert(loser, winner_val)` from
   `union_merge_exec`'s ensures created a universal quantifier over the full parent map.
   When this coexisted with roots/rank quantifiers in the same Z3 context via &mut encoding,
   the matching loop consumed 9GB+ of Z3 memory.

## Solution: Inlined Exec + Opaque Predicates

### 1. Inlined exec body (no function call overhead)

Instead of calling `union_merge_exec(uf, root_u, root_v)`, the mutations are inlined
directly into `union_merge`. This eliminates the exec's quantified ensures from the Z3
context — the Map insert axioms are local to the mutation site, not globally quantified.

### 2. Pointwise parent facts in lemma_union_merge_wf

Changed `lemma_union_merge_wf`'s requires from `uf.parent@ =~= mid.parent@.insert(...)` to:
- `uf.parent@.dom() =~= mid.parent@.dom()`
- `uf.parent@[loser_view] == winner_val`
- `forall|k| ... ==> uf.parent@[k] == mid.parent@[k]`

The lemma body reconstructs the Map =~= internally for sub-lemmas.

### 3. Opaque roots change predicates

Created two closed spec functions:
- `spec_roots_changed_by_merge` — the union_merge-style roots quantifier (trigger: `roots@[x]`)
- `spec_union_result` — the union-style roots quantifier (trigger: `roots.contains_key(x)`)

Union_merge's ensures use `spec_roots_changed_by_merge` (opaque boolean). Z3 never sees
the quantifier body in the &mut context, preventing the matching loop.

### 4. Combined pre-call helper

`lemma_union_merge_exec_pre` derives all exec prerequisites in a single proof call.

### 5. Elements == instead of =~=

Changed elements ensures from `=~=` to `==` to avoid per-element quantifier expansion.

## What Blocks union

### A. Rank bounds not in wf

`union_merge` requires `rank < elements.len()` for overflow prevention. This is a true
invariant of union-by-rank (rank < log₂(n) < n), but not derivable from the current
14-predicate wf. Proving it requires a component-size invariant (2^rank ≤ component_size).

### B. Roots quantifier matching loop

To prove `spec_union_result` from `spec_roots_changed_by_merge`, any proof context must
reveal BOTH closed predicates AND have `roots.dom =~=` (Set extensional equality). The
Set =~= expansion creates `roots.contains_key(x)` terms that trigger the revealed roots
quantifier, creating a matching loop. This affects any context — even a standalone proof
function — that combines the reveal with dom equality.

### C. Infrastructure ready but blocked

All the infrastructure for `union` is written and verified:
- `lemma_prove_union_result` (rlimit 30) — proves spec_union_result from merge predicate
- `lemma_union_roots_bridge` — translates guard from old→new dom
- `lemma_union_ensures_bridge` — existing quantifier transformation

The only issue: calling these from union's &mut context adds their output (opaque boolean +
dom Set =~=) to the context, and Z3 exceeds rlimit 30. At rlimit 50, Z3 uses 6GB+ with
linear growth — a slow matching loop.

## New Infrastructure

| Function | Purpose | rlimit |
|----------|---------|--------|
| `spec_union_result` | Opaque union-style roots predicate | — |
| `spec_roots_changed_by_merge` | Opaque merge-style roots predicate | — |
| `lemma_wf_type_axioms` | Extract type axioms from wf | default |
| `lemma_union_merge_exec_pre` | Combined exec prerequisites | default |
| `lemma_prove_roots_changed` | Prove merge predicate from Map::new | default |
| `lemma_prove_union_result` | Prove union result from merge predicate | 30 |
| `lemma_union_roots_bridge` | Bridge: old guard → new guard | default |
| `lemma_union_merge_ensures_bridge` | Forward bridge (not currently used) | default |
