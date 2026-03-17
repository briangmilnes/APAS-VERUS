R32: Prove assumes in Chap39 and external_body in BSTTreapStEph.

TASK 1 — Chap39/BSTTreapMtEph.rs: Prove 6 reader-predicate assumes.

These are all the same pattern: acquire read lock on the root,
call the St helper, bridge the locked state to the module spec.

(a) `find` — prove result.is_some() <==> self@.contains(target@)
(b) `size` — prove result as nat == self@.len()
(c) `minimum` — prove result correctness against set containment
(d) `maximum` — prove result correctness against set containment
(e) `in_order` — prove ordered@.len() == self@.len()
(f) `pre_order` — prove preordered@.len() == self@.len()

The St counterpart BSTTreapStEph.rs has verified implementations
of these operations on the raw tree. The Mt wrapper:
1. Acquires read lock → gets &BSTTreapStEph
2. Calls the St method
3. Must bridge: RwLockPredicate inv connects locked state to self@

Read BSTTreapMtEph.rs to understand the RwLockPredicate and how
self@ (the View) relates to the locked BSTTreapStEph's view.

TASK 2 — Chap39/BSTTreapStEph.rs: Prove 2 external_body holes.

(a) `find` (line ~520) — BST search.
    Recursive search in treap. The treap is a BST by key, so search
    follows BST property. Prove result matches set containment spec.
    The BST invariant (spec_is_bst) should give you the comparison
    property needed for recursive descent.

(b) `insert_link` (line ~749) — Recursive insertion with rebalancing.
    Insert maintains BST ordering + heap priority ordering.
    Prove: result set == old set + {key}, result is well-formed.
    This is the hardest proof — may need rotation lemmas.
    If you can prove find but not insert_link, that's still progress.

Do NOT modify BSTParaTreapMtEph.rs (parallel ops — skip this round).
Do NOT add assume, accept, or external_body.
Every quantifier must have explicit #[trigger].
Run scripts/validate.sh after changes — 0 errors required.
See plans/orchestrator-r32-hole-reduction.md for context.
