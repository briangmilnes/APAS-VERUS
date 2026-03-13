# Agent 1 — Round 7: Chap37 AVLTreeSeq Critical Path

## Mission

Prove holes in Chap37 AVLTreeSeq files. This is the critical path — cleaning these unblocks
8 downstream chapters (41, 42, 43, 45, 52, 53, 55, 57, 65).

**Your success metric is holes eliminated.** Every hole you started with that still exists
when you finish is a failure. Do not write "deferred" or "hard" or "would require" — those
are words for quitters. Read the error, read vstd, write intermediate lemmas, decompose the
obligation, try a different approach. If you genuinely exhaust every idea on a specific hole,
say exactly what you tried and where you got stuck — but that should be the exception, not
the pattern. You were put on this task to prove things, not to catalog reasons you didn't.

## Your Files (ONLY touch these)

1. `src/Chap37/AVLTreeSeq.rs` — 3 holes (assume: nat_max)
2. `src/Chap37/AVLTreeSeqStEph.rs` — 8 holes (assume + ext_body)
3. `src/Chap37/AVLTreeSeqStPer.rs` — 14 holes (ext_body + assume)
4. `src/Chap37/AVLTreeSeqMtPer.rs` — 13 holes (ext_body)
5. `src/Chap37/BSTSplayStEph.rs` — 2 holes (trivial_wf + assume)

**DO NOT touch any BST*MtEph or BSTSet*MtEph files. Those are Agent 2's.**

## Strategy for AVLTreeSeq.rs nat_max Assumes

The assume at line 594 is:
```rust
assume(spec_avltreeseq_nat_max(
    spec_avltreeseq_cached_height(&n.left),
    spec_avltreeseq_cached_height(&n.right),
) + 2 < usize::MAX);
```

This is needed by `rebalance`'s requires. The current `insert_at_link` requires
`spec_avltreeseq_cached_size(&node) + 1 < usize::MAX` but `rebalance` also needs the
height bounded. The wf does NOT include a height-vs-size invariant, so you can't derive
the height bound from the size bound.

**DO NOT muck with `spec_avltreeseq_wf`.** Instead:

1. **Write `proof fn lemma_height_le_size`**: For any well-formed link, `height <= size`.
   This is a basic tree property — height is at most the number of nodes. Prove by
   structural induction on the link (`decreases link`). The wf gives you
   `height == 1 + max(left_height, right_height)` and `size == 1 + left_size + right_size`.
   Since `max(a,b) <= a + b`, you get `height <= 1 + left_size + right_size == size`.

2. **Add size bounds to trait requires**: Functions that grow the tree need
   `self.spec_avltreeseq_seq().len() + 1 < usize::MAX` (or equivalent on cached_size).
   The trait functions that need this added:
   - `push_back` (line 275) — currently has no size bound
   - `insert_value` (line 284) — currently has no size bound
   - `from_vec` (line 253) — already has `values@.len() < usize::MAX` (good)

3. **Use the lemma in `insert_at_link`**: After the recursive call, the child's wf holds
   (from ensures). Call `lemma_height_le_size` on both children. Now you know
   `max(left_height, right_height) <= max(left_size, right_size) <= left_size + right_size`.
   Combined with `size + 1 < usize::MAX`, the rebalance requires follows.

4. **Propagate size bounds through callers**: `push_back` and `insert_value` impls call
   `insert_at_link`. With the new trait requires, they can satisfy `insert_at_link`'s
   `cached_size + 1 < usize::MAX`. Update callers in StEph, StPer, and MtPer too.

The PartialEq::eq assumes (lines 1197-1201) are the standard eq/clone bridge pattern —
`assume(wf)` and `assume(size < MAX)` inside eq body. These are acceptable per the
eq/clone standard. Leave them.

## Execution Order

1. Read `src/standards/*.rs` (all 15 files).
2. Read each file above. Run `scripts/holes.sh src/Chap37/` to see current state.
3. **AVLTreeSeq.rs**: Write `lemma_height_le_size`. Add size bounds to `push_back` and
   `insert_value` trait requires. Use the lemma to eliminate the nat_max assume in
   `insert_at_link`. Validate.
4. **AVLTreeSeqStEph.rs** (8 holes): Propagate the new trait requires through the impl.
   Prove insert/delete/find assumes. Remove ext_body on algorithmic functions.
5. **AVLTreeSeqStPer.rs** (14 holes): Replicate StEph proofs. Similar structure, persistent view.
6. **BSTSplayStEph.rs** (2 holes): Write real `spec_bstsplaysteph_wf` body (not `{ true }`),
   prove remaining assume.
7. **AVLTreeSeqMtPer.rs** (13 holes): Lock-boundary ext_body — prove delegations through
   RwLock acquire/release. Use arc_rwlock standard pattern.
8. After each file: `scripts/validate.sh`. Fix all errors before moving on.
9. When done: `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh`.

## Rules

- Read CLAUDE.md fully.
- Never add assume(), accept(), admit(), or external_body.
- Never sequentialize Mt files.
- Search vstd before writing new lemmas.
- Commit to `agent1/ready` branch when done.

## Attitude

Prove big or go home. You are not here to assess difficulty — you are here to eliminate
holes. A round where you close 5 holes out of 40 is a failed round. Every assume is a proof
obligation you can discharge. Every ext_body on algorithmic logic is a wrapper you can remove.
The proofs are hard — that's why you exist. Do the work.
