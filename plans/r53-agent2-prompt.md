You are agent2 working in branch: ~/projects/APAS-VERUS-agent2/.
Your worktree has been updated.
Read senior-proof-engineer.mdc (from .cursor/rules).
Read src/standards/using_closures_standard.rs BEFORE writing any closure code.
REREAD ANY FILES BEFORE MODIFYING THEM as I modify files in emacs.

Round 53 — Chap41 union capacity + Chap43 provable holes.

Current state: 4476 verified, 15 holes, 38 clean chapters.

## Phase 1: Chap41 AVLTreeSetStEph union capacity (1 hole)

The last Chap41 hole is at AVLTreeSetStEph.rs:722:
  assume(combined@.len() + 1 < usize::MAX as nat)

This is in the `union` loop body. The issue: we know self@.len() < usize::MAX and
other@.len() < usize::MAX from wf, but their union could be up to 2 * usize::MAX.

Approach: Add `requires self@.len() + other@.len() < usize::MAX as nat` to the
`union` method in the trait. Then propagate to callers. The callers are:
- Chap43 OrderedSetStEph, OrderedSetStPer, OrderedSetMtEph (union methods)
- Chap53 PQMinStEph (if it calls union)

Read the trait definition, add the requires, fix callers. You MAY touch Chap43
and Chap53 files to propagate the requires — this is a cross-chapter cascade.

If Chap41 closes to 0 holes, that unblocks Chap43 and downstream (Chap52/53/55).

## Phase 2: Chap43 provable holes (up to 5 holes)

After fixing the union cascade, attack Chap43's remaining holes:

1. OrderedSetStEph.rs:1134 — assume in `from_sorted_seq`. This assumes the filter
   result equals the set. Read the function, understand the spec, try to prove it.
2. OrderedSetStPer.rs:1031 — same pattern as above.
3. OrderedSetStPer.rs:1157 — fn_missing_requires on `from_sorted_elements`. Add
   the real requires (likely wf on input).
4. OrderedTableMtPer.rs:321 — assume(len < usize::MAX). Capacity bound, same
   pattern as Chap41.
5. AugOrderedTableStPer.rs:117+124 — assume in lemma_reducer_clone_total. This
   is a closure-requires assume. Read using_closures_standard.rs first. Try to
   lift the obligation into the function's requires clause.

Run scripts/holes.sh src/Chap41/ and scripts/holes.sh src/Chap43/ first.

Key rules:
- WARNING: Do NOT add accept() anywhere. Do NOT convert assume to accept.
- Read using_closures_standard.rs BEFORE touching any closure assume.
- Run scripts/validate.sh after changes. Show full output.
- You MAY touch Chap41, Chap43, Chap52, Chap53, Chap55 for cascade fixes.
- Search vstd for lemmas before writing new ones (veracity-search).

Success criteria: Close Chap41 (0 holes). Reduce Chap43 by at least 2.

REPORTING: Write plans/agent2-round53-report.md with holes before/after table.

Execute relentlessly. Propose a plan, then implement it.
