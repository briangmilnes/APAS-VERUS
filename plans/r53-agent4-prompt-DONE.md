<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent4 working in branch: ~/projects/APAS-VERUS-agent4/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read src/standards/partial_eq_eq_clone_standard.rs first.
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Round 53 — Chap38 clone bridge + Chap39 Send blocker.

Current state: 4476 verified, 15 holes, 38 clean chapters.

## Phase 1: Chap38 clone bridge (2 holes)

BSTParaStEph.rs:152 and BSTParaMtEph.rs:152 both have:
  assume(c == *x)  // Clone bridge

You successfully used ClonePreservesView in Chap39 last round to close the
identical hole. Apply the same technique here:

1. Read src/vstdplus/clone_view.rs (your creation).
2. Add `+ ClonePreservesView` to the T bounds on `clone_elem` in both files.
3. Replace `assume(c == *x)` with `x.clone_view()` call.
4. Propagate the ClonePreservesView bound to all callers.

IMPORTANT: Chap38 BSTParaStEph and BSTParaMtEph share the same pattern. Fix both.

## Phase 2: Chap39 Send blocker investigation (1 hole)

BSTParaTreapMtEph.rs:1699 has an assume in filter_parallel related to Ghost
spec types not implementing Send. You investigated this last round and found
it structural.

Second look: Can you factor the parallel section so the closure captures only
Send-compatible data? For example:
- Pre-compute the filter results into owned Vec<T> in the closure
- Pass the spec predicate through a different mechanism
- Use the HF Scheduler instead of ParaPair! (scheduler's join() may have
  different Send constraints)

Read src/Chap02/HFSchedulerMtEph.rs and src/standards/hfscheduler_standard.rs.
If the Send constraint is truly unavoidable, document exactly why in your report
and move on.

## Phase 3: Chap26 ETSPMtEph point_distance (1 hole, stretch)

If time remains, look at Chap26 ETSPMtEph.rs:612 — external_body on
`point_distance`. This uses f64::sqrt which has no Verus axiom.

Read src/vstdplus/float.rs. Could you write a tight ensures that doesn't
need sqrt axioms? For example, ensures on the squared distance:
  ensures result * result == (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)

This is a stretch goal. Only attempt if phases 1-2 are done.

Key rules:
- WARNING: Do NOT add accept() anywhere.
- Run scripts/validate.sh after changes. Show full output.
- DO NOT touch Chap47 (agent1's territory).
- Search vstd for lemmas before writing new ones.

Success criteria: Close Chap38 (0 holes). Investigate Chap39 Send.

REPORTING: Write plans/agent4-round53-report.md with holes before/after table.

Execute relentlessly. Propose a plan, then implement it.
