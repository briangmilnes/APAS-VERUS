R32: Prove external_body holes in Chap37 and Chap38.

TASK 1 — Chap37: Close the chapter (3 external_body).

(a) src/Chap37/AVLTreeSeq.rs — `next` (line ~1117).
    Iterator next() for in-order traversal. Prove the exec body
    matches the View spec: returns elements in order, increments
    position, returns None when exhausted. The tree's `nth` method
    is already verified — connect it to the iterator contract.

(b) src/Chap37/AVLTreeSeqMtPer.rs — `build_balanced_from_slice` (line ~509).
    Parallel balanced tree construction from slice. Recursively splits
    at midpoint, spawns left/right via join(). Prove:
    - result is well-formed (spec_avltreeseqmtper_wf)
    - in-order traversal matches input slice
    Use named closures with explicit ensures per CLAUDE.md fork-join rules.

(c) src/Chap37/AVLTreeSeqMtPer.rs — `subseq_copy` (line ~630).
    Parallel subsequence extraction. Prove:
    - result is well-formed
    - result view equals input view subsequence

TASK 2 — Chap38: Prove 2 holes in BSTParaStEph.rs.

(a) `expose` assume (line ~469).
    The assume bridges cloned key equality. The Clone::clone ensures
    clause guarantees `result@ == self@`. Use that ensures to prove
    the cloned key satisfies cmp_spec ordering. If Clone ensures are
    insufficient, document what's missing.

(b) `clone` external_body (line ~1577).
    ParamBST Clone impl. Prove that cloning through RwLock recreation
    preserves view semantics (cloned@ == self@). The internal tree
    clone is already verified; bridge the lock state creation.

Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
See plans/orchestrator-r32-hole-reduction.md for context.
