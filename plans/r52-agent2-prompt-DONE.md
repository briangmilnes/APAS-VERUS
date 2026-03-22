<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent2 working in branch: ~/projects/APAS-VERUS-agent2/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 52 — Chap41 AVLTreeSetStEph Hole Reduction (key dependency blocker).

Current state: 4472 verified, 25 holes, 37 clean chapters.

Chap41 is the MOST IMPORTANT chapter to fix: it blocks Chap43 (5 holes),
and transitively blocks Chap52, Chap53, and Chap55. Closing Chap41 unblocks
4 downstream chapters.

Your target: Chap41 (5 holes, 7 files, only AVLTreeSetStEph.rs has holes).

Run scripts/holes.sh src/Chap41/ to see exactly what the 5 holes are.
Read src/Chap41/AVLTreeSetStEph.rs carefully before making changes.

The 5 holes are likely a mix of:
- assume() on set-semantics (insert/delete/filter preserve set view)
- external_body on algorithmic functions
- Possibly Ord consistency axioms similar to Chap38/39

Strategy:
1. Run holes, read the file, understand each hole.
2. Search vstd for relevant lemmas (veracity-search).
3. Agent3 just used type_invariant + real RwLockPredicate to great effect
   on Chap39. Check if similar infrastructure helps here.
4. Agent4 created ClonePreservesView (vstdplus/clone_view.rs). Use it
   if clone bridge holes appear.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap41/ to verify hole counts.
- DO NOT touch files outside Chap41.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Reduce Chap41 from 5 holes. If all close, Chap41 goes
clean, which unblocks Chap43+52+53+55. Minimum target: -3 holes.

REPORTING: Before committing, write plans/agent2-round52-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap41.

Execute relentlessly. Propose a plan, then implement it.
