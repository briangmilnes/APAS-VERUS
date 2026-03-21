You are agent4 working in branch: ~/projects/APAS-VERUS-agent4/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 52 — Chap39 remaining + Chap45 Example.

Current state: 4472 verified, 25 holes, 37 clean chapters.

Phase 1 — Chap39 BSTParaTreapMtEph (2 holes):

  Agent3 did a massive rewrite of this file last round. 2 holes remain:

  1. clone_elem assume(c == *x) — clone bridge. You created
     ClonePreservesView in vstdplus/clone_view.rs. Try applying it here
     to close this hole. Read the file to understand the current clone_elem
     pattern and see if ClonePreservesView can bridge it.

  2. filter_parallel pred send limit — the predicate closure is not Send.
     This may be a fundamental Verus limitation (closures don't implement
     Send). Investigate whether there's a workaround, or document as
     structural blocker.

  Goal: Close at least the clone bridge hole. If both close, Chap39 goes
  clean (new clean chapter).

Phase 2 — Chap45 Example45_2.rs (1 hole):

  This is an Example file. Per CLAUDE.md, Example files are low priority.
  Only work on this if Chap39 is done and you have time.

  Run scripts/holes.sh src/Chap45/ to see what the hole is.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap39/ to verify hole counts.
- DO NOT touch files outside Chap39 and Chap45.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Reduce Chap39 from 2 holes. If both close, Chap39 is
a new clean chapter. Minimum target: -1 hole.

REPORTING: Before committing, write plans/agent4-round52-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap39.

Execute relentlessly. Propose a plan, then implement it.
