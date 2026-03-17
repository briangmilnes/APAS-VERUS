R32: Prove assumes in Chap41 and fix fn_missing_ensures.

TASK 1 — Prove 4 assumes across 3 files.

(a) src/Chap41/AVLTreeSetStEph.rs — `insert` assume (line ~959).
    Assumes new_vec.len() < usize::MAX. Derive this from the
    existing well-formedness invariant or capacity bounds. The set
    is backed by an AVLTreeSeqStEph with bounded size — find where
    the size bound lives and propagate it to the insert proof.

(b) src/Chap41/AVLTreeSetMtEph.rs — `size` assume.
    Reader-predicate bridge: read lock gives &AVLTreeSetStEph,
    call size(), bridge locked state to self@.len(). Pattern:
    RwLockPredicate inv connects locked backing store to self@.

(c) src/Chap41/AVLTreeSetMtEph.rs — `find` assume.
    Same pattern as size: read lock, call StEph find, bridge
    result to self@.contains(target@).

(d) src/Chap41/AVLTreeSetMtPer.rs — `find` assume (line ~490).
    Binary search loop exit: prove element not found when loop
    terminates without match. The loop maintains sorted-array
    invariant — use it to prove absence.

TASK 2 — Fix 3 fn_missing_ensures.

(a) src/Chap41/AVLTreeSetMtEph.rs — `parallel_filter` (line ~313).
    Fork-join helper for filter. Add real ensures that describe
    what the parallel arms produce (filtered subset, well-formed).

(b) src/Chap41/AVLTreeSetMtEph.rs — `parallel_intersect` (line ~373).
    Fork-join helper for intersection. Add real ensures (intersection
    subset, well-formed).

(c) src/Chap41/AVLTreeSetMtPer.rs — `parallel_sort` (line ~232).
    Fork-join helper for sort. Add real ensures (sorted output,
    permutation of input).

Read the function bodies to understand what they actually produce.
The ensures must be REAL postconditions — not `ensures true`.

TASK 3 — Try one external_body proof.

src/Chap41/AVLTreeSetMtPer.rs — `cmp` (line ~575).
Lexicographic comparison for Ord impl. If the backing sorted
array is accessible, compare element-by-element. This may be
straightforward if the arrays support indexed access.

Do NOT modify Example41_3.rs (skip Example files per CLAUDE.md).
Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
See plans/orchestrator-r32-hole-reduction.md for context.
