<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent3 working in branch: ~/projects/APAS-VERUS-agent3/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 52 — Chap62 completion + Chap26 ETSP + Chap65 Prim.

Current state: 4472 verified, 25 holes, 37 clean chapters.

Your R50 Chap39 rewrite was excellent — type_invariant + real RwLockPredicate.
Now apply your skills to close out smaller chapters.

Phase 1 — Chap62 StarContraction (2 holes):

  Target: src/Chap62/StarContractionMtEph.rs (1 hole)
  - Line ~115: assume(spec_valid_partition_map) — prove the partition loop
    maps every graph vertex to a center.
  - Add missing ensures on star_contract_mt_fuel and star_contract_mt.

  Target: src/Chap62/StarContractionStEph.rs (1 hole)
  - Check for matching ensures/hole.

  Goal: Close Chap62 entirely (0 holes, new clean chapter).

Phase 2 — Chap26 ETSPMtEph (2 holes):

  Target: src/Chap26/ETSPMtEph.rs
  - 2 external_body holes: point_distance and find_best_swap_par.
  - point_distance does f64 arithmetic (sqrt of sum of squares). Read
    vstdplus/float.rs for f64 axioms.
  - find_best_swap_par is the parallel swap-finder. May need fork-join
    proof infrastructure.

  Goal: Close at least point_distance. Stretch: both holes.

Phase 3 — Chap65 PrimStEph (1 hole, if time):

  Target: src/Chap65/PrimStEph.rs
  - prim_mst is external_body with a complex loop body.
  - Read src/Chap65/UnionFindStEph.rs for MST-related specs.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh on your chapters to verify.
- DO NOT touch files outside Chap62, Chap26, and Chap65.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Close Chap62 (0 holes). Reduce Chap26. Minimum: -2 holes.

REPORTING: Before committing, write plans/agent3-round52-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole counts for Chap62, Chap26, Chap65.

Execute relentlessly. Propose a plan, then implement it.
