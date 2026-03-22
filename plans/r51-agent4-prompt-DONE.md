<!-- THESE ARE DONE. DO NOT WORK ON THESE TASKS. IGNORE THIS FILE. -->
You are agent4 working in branch: ~/projects/APAS-VERUS-agent4/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read senior-prose-engineer.mdc (from .cursor/rules).
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Read src/standards/* and apply them in your proposed work.
Fix trigger warnings as they occur using the rules.

Round 51 — Chap62 completion + Chap65 MST algorithms.

Current state: 4465 verified, 33 holes, 37 clean chapters.

You just proved quotient wf in Chap62 via ClonePreservesView last round.
Continue that momentum.

Phase 1 — Finish Chap62 (1 hole + warnings):

  Target: src/Chap62/StarContractionMtEph.rs
  1. Line 115: assume(spec_valid_partition_map) — prove that the partition
     loop maps every graph vertex to a center. You already understand the
     partition structure from R50.
  2. Add missing ensures on star_contract_mt_fuel (line 86) and
     star_contract_mt (line 142). These are fn_missing_ensures warnings.

  Also check src/Chap62/StarContractionStEph.rs for matching ensures.

  Goal: Close Chap62 entirely (0 holes, clean chapter).

Phase 2 — Chap65 MST algorithms (2 holes):

  Target 1: src/Chap65/KruskalStEph.rs (1 hole)
  - Line 58: sort_edges_by_weight is external_body. This sorts edges by
    f64 weight. Options: (a) delegate to a verified sort with a comparator,
    (b) prove the body using float axioms from vstdplus/float.rs.
    The function just needs to sort a Vec<LabEdge<V, WrappedF64>> by weight.

  Target 2: src/Chap65/PrimStEph.rs (1 hole)
  - Line 95: prim_mst is external_body — the main Prim algorithm. This is
    a bigger proof. Read the function body carefully. It uses a priority
    queue and greedy edge selection. The proof needs to show the result
    is a valid MST (or at minimum, a spanning tree).
  - Also fix fn_missing_requires on pq_entry_new (line 72) if it has a
    real precondition, or flag for user review if genuinely none.

  Read src/Chap65/UnionFindStEph.rs — it's clean and may provide useful
  spec functions for Kruskal's proof.

  Goal: Close at least KruskalStEph. Stretch: start PrimStEph proof.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Do the proof work: try to prove each assume, don't just label it.
- Run scripts/validate.sh after changes. Show full output.
- Run scripts/holes.sh src/Chap62/ and scripts/holes.sh src/Chap65/ to verify.
- DO NOT touch files outside Chap62 and Chap65.
- Search vstd for lemmas before writing new ones (veracity-search).
- Read vstdplus/float.rs for f64 axioms relevant to Chap65.

Success criteria: Close Chap62 (0 holes). Reduce Chap65 by at least 1.

REPORTING: Before committing, write plans/agent4-round51-report.md with:
  1. What you changed and why.
  2. Holes closed (file, line, hole type, how resolved).
  3. Blockers — anything you could not fix and why.
  4. Verified count, hole count for Chap62 and Chap65.

Execute relentlessly. Propose a plan, then implement it.
