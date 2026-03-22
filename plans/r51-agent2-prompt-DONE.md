<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent2 working in branch: ~/projects/APAS-VERUS-agent2/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 51 — Chap39 BSTParaTreapMtEph Hole Reduction.

Current state: 4465 verified, 33 holes, 37 clean chapters.

Your target: src/Chap39/BSTParaTreapMtEph.rs (9 holes).
This is the single most-holed file in the project.

You just proved filter_inner in Chap38/BSTParaMtEph.rs last round. The Chap39
treap uses the same BST patterns (Ord consistency, set algebra, split/join).

The 9 holes break into categories:

  1. expose_internal (2 holes, lines 191+200):
     RWLOCK_GHOST — connect physical node state to ghost set.
     - assume(tree@.finite() && tree@.len() == 0) for empty node
     - assume(tree@.finite() && tree@.contains(key@) && ...) for non-empty
     Same lock-boundary pattern you know from Chap38.

  2. split_inner (3 holes, lines 328+339+348):
     Ord consistency proofs:
     - key < root_key implies key@ != root_key@ and !right@.contains(key@)
     - key > root_key implies key@ != root_key@ and !left@.contains(key@)
     - key == root_key implies key@ == root_key@
     You proved exactly this pattern in Chap38's split_inner.

  3. intersect_inner (2 holes, line 430+):
     Set intersection disjointness after split. Need to show
     a@.intersect(b@) relates to recursive results.

  4. union_inner / difference_inner (2 holes):
     Similar set-algebra proofs connecting recursive results to
     set operations on views.

Priority: split_inner (3 holes) first — direct reuse of Chap38 technique.
Then expose_internal (2 holes). Then set-algebra proofs (4 holes).

Read the standard: src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs
Read the standard: src/standards/using_closures_standard.rs

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap39/ to verify hole counts.
- DO NOT touch files outside Chap39.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Reduce Chap39 from 9 holes. Minimum target: -5 holes
(split_inner + expose_internal). Stretch: all 9.

REPORTING: Before committing, write plans/agent2-round51-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap39.

Execute relentlessly. Propose a plan, then implement it.
