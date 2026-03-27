# R89 Agent 3 — Prove SpanTreeMtEph holes (Chap64), STEP 20

## Objective

Remove external_body from 2 functions in `src/Chap64/SpanTreeMtEph.rs`.

## The 2 Holes

### 1. `spanning_tree_star_contraction_mt` (line 56)

Calls `star_contract_mt(graph, seed, &base, &expand)` with two closures. The
external_body comment (from R88 agent1) says: "expand closure requires —
star_contract_mt demands forall|..| expand.requires(...) (universal)."

The `base` closure is trivial (returns empty set). The `expand` closure has
two loops collecting spanning edges. The challenge is that `star_contract_mt`
requires universal quantification over the closure's requires, and the expand
closure has implicit requires (wf, valid_key_type) that need to be explicitly
stated in its signature.

**Fix approach:**
1. Add explicit `requires` to the expand closure matching what star_contract_mt
   expects. Read `src/Chap62/StarContractionMtEph.rs` to see what
   `star_contract_mt` demands of `expand`.
2. Add explicit `ensures` on the expand closure — at minimum
   `result.spec_setsteph_wf()`.
3. The base closure needs `ensures result.spec_setsteph_wf()` too.

### 2. `verify_spanning_tree` (line 150)

Checks that tree_edges has exactly |V|-1 edges. The body just compares
`tree_edges.size() == expected_edges`. The ensures says `result ==>
tree_edges@.len() == |V|-1`.

**Fix approach:** This should be straightforward — `size()` ensures
`count == self@.len()`, so the comparison gives the postcondition directly.
May need an explicit assert bridging `size()` result to the ensures.

## Read first

- `src/Chap64/SpanTreeMtEph.rs` — your file (read all of it)
- `src/Chap64/SpanTreeStEph.rs` — working StEph version for reference
- `src/Chap62/StarContractionMtEph.rs` — `star_contract_mt` signature and
  what it requires of base/expand closures
- `src/standards/using_closures_standard.rs` — closure requires/ensures pattern

## Key: star_contract_mt's closure requirements

Read `star_contract_mt`'s signature carefully. It takes `&F` (base) and `&G`
(expand) with where-clause bounds. The closures need requires/ensures that
match those bounds. The universal quantification `forall|..| expand.requires(..)`
means every possible argument tuple must satisfy the closure's requires —
so the requires should only demand things that are always true (type-level
properties like `valid_key_type`, `obeys_key_model`, wf on the inputs).

## Isolation

```bash
scripts/validate.sh isolate Chap64
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify files outside Chap64.
- Do NOT add assume or accept.
- Prioritize verify_spanning_tree (easier) first, then spanning_tree.
- If the closure requires threading for spanning_tree is too complex,
  external_body it and report exactly what star_contract_mt demands vs
  what the closure can provide.

## STEP 20

## Report

Write `plans/agent3-r89-spantree-report.md`.
