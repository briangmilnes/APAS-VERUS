# R115 Agent 2 — Explore new-mut-ref for UnionFind. AFK. PBOGH.

## Context

Verus has a new `&mut` encoding opt-in via `-V new-mut-ref`. Migration:
`*x` in ensures becomes `*final(x)`. The new encoding may produce a
different Z3 context that avoids the matching loops blocking union.

## Objective

Try proving `union` in UnionFindStEph.rs using the new-mut-ref encoding.
This is an EXPERIMENT — we want to know if the new encoding helps, not
necessarily to ship the result.

## Steps

1. Uncomment Chap65 in lib.rs.
2. Read `src/Chap65/UnionFindStEph.rs` — understand the current infrastructure.
3. Read `plans/agent1-r106-unionfind-report.md` — understand the blockers.
4. Create a branch of UnionFindStEph.rs that uses new-mut-ref patterns:
   - `*self` in ensures → `*final(self)` where appropriate
   - Test with `scripts/validate.sh isolate Chap65` (add `-V new-mut-ref`
     flag — check how validate.sh passes extra flags, or modify the script
     temporarily)
5. See if the union proof goes through with the different encoding.
6. Document what changed, what worked, what didn't.

## How to enable new-mut-ref

Check `scripts/validate.sh` for how it invokes verus. You may need to add
`-V new-mut-ref` to the verus invocation. Look for the command line in the
script.

Alternatively, check if there's a `VERUS_EXTRA_ARGS` env var or similar.

## Read about new-mut-ref

The memory note says:
- new-mut-ref is opt-in via `-V new-mut-ref`
- Migration: `*x` in ensures → `*final(x)`

Search the Verus guide and `~/projects/verus/source/rust_verify_test/tests/`
for examples of `final(` to understand the pattern.

## Rules

- This is exploratory. If it doesn't work after 15 steps, write up what
  you learned and stop.
- Do NOT modify any files outside Chap65.
- Do NOT add assume or accept.
- Use `scripts/validate.sh isolate Chap65` only.
- No subagents.

## STEP 20

## Report

Write `plans/agent2-r115-new-mut-ref-report.md`. Include:
- Whether new-mut-ref changes the Z3 behavior on union
- Any matching loop differences observed
- Whether the encoding helps or not
- Code snippets showing the key differences
