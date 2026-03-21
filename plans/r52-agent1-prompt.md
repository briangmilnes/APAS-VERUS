You are agent1 working in branch: ~/projects/APAS-VERUS-agent1/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 52 — Chap47 ParaHashTableStEph remaining holes.

Current state: 4472 verified, 23 holes, 37 clean chapters.

You worked on Chap47 in R50 (added feq broadcast, obeys_feq_clone requires).
Continue that work. 3 holes remain.

Run scripts/holes.sh src/Chap47/ to see exactly what the 3 holes are.
Read src/Chap47/ParaHashTableStEph.rs carefully before making changes.

From your R50 report, the remaining holes are:
  1. clone_elem assume(c == *x) — clone bridge. Agent4 created
     ClonePreservesView in vstdplus/clone_view.rs. Read it and try
     applying it to close this hole.
  2. call_hash_fn external_body — Rust Hash trait not verifiable via vstd.
     Investigate whether you can write a tight ensures and prove the body
     partially, or whether this is truly structural.
  3. A third hole (possibly QuadProbFlatHashTableStEph assume(false)).
     Run holes to confirm.

IMPORTANT: Do NOT rename spec_impl_wf or restructure the wf predicates.
The user owns that refactor. Work within the existing naming.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap47/ to verify hole counts.
- DO NOT touch files outside Chap47.
- DO NOT rename wf predicates or restructure the spec_impl_wf pattern.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Reduce Chap47 from 3 holes. Minimum target: -1 hole.

REPORTING: Before committing, write plans/agent1-round52-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap47.

Execute relentlessly. Propose a plan, then implement it.
