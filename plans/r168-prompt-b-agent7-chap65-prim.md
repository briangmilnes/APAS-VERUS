# R168 Prompt B — Agent 7: Prove PrimStEph prim_mst. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent7`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**

## The hole

`src/Chap65/PrimStEph.rs:253` — `prim_mst` has `#[verifier::external_body]`.
The function is 280 lines with a full loop body, invariants, and proof blocks
already written. The `external_body` is hiding whatever verification error
the original author couldn't fix.

## Your task

1. **Remove `#[verifier::external_body]`** from `prim_mst`.
2. **Run `scripts/validate.sh isolate Chap65`** and read the errors.
3. **Fix the verification errors.** The proof body is already there — it
   likely needs stronger loop invariants, missing lemma calls, or trigger
   fixes. The function already has:
   - Loop invariants about `spec_labgraphview_wf`, `valid_key_type_LabEdge`
   - Ghost state: `remaining_budget`, `used_pairs`, `DA` directed adjacency
   - Proof blocks for finiteness and map bounds

## Attitude

Prove it or don't come back. No `external_body`. No `assume`. No `admit`.
No `accept`. No "I tried and it's hard." The proof body is 280 lines of
existing work — someone got close. Finish the job.

## Context

Read these files first:
- `src/Chap65/PrimStEph.rs` — the target (528 lines)
- `src/Chap06/LabUnDirGraphStEph.rs` — the graph type (ng, get_edge_label)
- `src/Chap45/BinaryHeapPQ.rs` — the priority queue (used in Prim's loop)
- `src/Chap05/SetStEph.rs` — the set type for mst_edges

The `ensures` is just `mst.spec_setsteph_wf()` — well-formedness of the result
set. This is a weak spec (no MST correctness). Your job is to get this weak
spec verified, not to strengthen it.

## Read all standards first.

## Validation

```bash
scripts/validate.sh isolate Chap65
```

## Report

Write `plans/agent7-round168-report.md`.

## RCP

`git add -A && git commit -m "R168 Agent 7: prove PrimStEph prim_mst (−1 hole)"`, then `git push`.
