You are agent1 working in branch: ~/projects/APAS-VERUS-agent1/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 51 — Chap38 BSTParaMtEph + BSTParaStEph Hole Reduction.

Current state: 4465 verified, 31 holes, 37 clean chapters.

Your target: Chap38 (4 holes across 2 files).

File 1: src/Chap38/BSTParaMtEph.rs (3 holes)

  1. Line 152: assume(c == *x) — clone bridge in clone_elem.
     T::clone preserves view. Agent4 created ClonePreservesView in
     vstdplus/clone_view.rs last round. Check if you can use it to close
     this hole. Read src/vstdplus/clone_view.rs and
     src/standards/partial_eq_eq_clone_standard.rs.

  2. Lines 229-230: assume(obeys_cmp_spec) + assume(view_ord_consistent)
     in lemma_cmp_order_axioms. These are Ord consistency axioms for
     generic T: MtKey. Search vstd for existing lemmas about
     obeys_cmp_spec and view_ord_consistent. Check if there's a broadcast
     group or lemma that establishes these for types satisfying MtKey bounds.
     Use veracity-search: `veracity-search 'fn _ ensures .*obeys_cmp'`

File 2: src/Chap38/BSTParaStEph.rs (1 hole)

  3. assume(false) — find the assume(false) and determine what it guards.
     If it's an unreachable branch, prove unreachability. If it's a
     thread-join error arm, it's a standard pattern (leave it).

Priority: clone bridge (#1) first — try ClonePreservesView. Then Ord
axioms (#2). Then assume(false) (#3).

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap38/ to verify hole counts.
- DO NOT touch files outside Chap38.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Reduce Chap38 from 4 holes. If all 4 close, Chap38
becomes a clean chapter. Minimum target: -2 holes.

REPORTING: Before committing, write plans/agent1-round51-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap38.

Execute relentlessly. Propose a plan, then implement it.
