# Agent 1 — Round 8: Chap37 AVLTreeSeq* — The Root Blocker

## Mission

Prove AVLTreeSeq and its variants. This is the single most impactful work in the entire
project right now because unblocking these files cascades to Chap41 → Chap42 → Chap43 →
Chap45 → Chap52/53 (6 downstream chapters, 300+ holes).

**Your success metric is holes eliminated.** Every hole you started with that still exists
when you finish is a failure. Do not write "deferred" or "hard" or "would require" — those
are words for quitters. Read the error, read vstd, write intermediate lemmas, decompose the
obligation, try a different approach. If you genuinely exhaust every idea on a specific hole,
say exactly what you tried and where you got stuck — but that should be the exception, not
the pattern. You were put on this task to prove things, not to catalog reasons you didn't.

## Your Files (ONLY touch these)

1. `src/Chap37/AVLTreeSeq.rs` — 2 external_body + eq/clone assumes
2. `src/Chap37/AVLTreeSeqStEph.rs` — 1 assume, 4 external_body + clone assumes
3. `src/Chap37/AVLTreeSeqStPer.rs` — 9 assume, 6 external_body
4. `src/Chap37/AVLTreeSeqMtPer.rs` — 10 assume, 4 external_body, 1 missing spec
5. `src/Chap37/BSTSplayStEph.rs` — 1 external_body

**DO NOT touch any BST*MtEph, BSTSet*MtEph, BST*StEph (except BSTSplayStEph) files.
Those are Agent 2's.**

## Hole Breakdown

**AVLTreeSeq.rs** (2 holes):
- Line 1117: external_body (iterator-related)
- Line 1195: external_body (iterator-related)
- Lines 1215-1216: eq/clone workaround assumes (standard pattern, acceptable)

**AVLTreeSeqStEph.rs** (5+clone holes):
- Line 863: assume(self.next_key < usize::MAX) — needs size bound propagation
- Lines 851, 1017, 1038, 1052: 4 external_body (iterator/clone/from_vec)
- Line 668, 1073-1075: clone workaround assumes (standard pattern)

**AVLTreeSeqMtPer.rs** (14 holes):
- 8 assumes of form `assume(n_val@ == n.value@)` — Arc value-linking after clone
- 2 assumes: from_vec spec + wf
- 4 external_body: rec, from_vec, iterator stubs
- 1 fn missing requires/ensures (rec at line 510)

**AVLTreeSeqStPer.rs** (15 holes):
- 9 assumes (value-linking after clone, similar pattern to MtPer)
- 6 external_body (iterator/clone/from_vec stubs)

**BSTSplayStEph.rs** (1 hole):
- 1 external_body

## Strategy

### Value-linking assumes (`assume(n_val@ == n.value@)`)
These appear in MtPer and StPer rotation/rebalance code. The pattern is: clone a node's
value into a local variable, then need to prove the clone preserved the view. Read
`src/standards/partial_eq_eq_clone_standard.rs` — the `axiom_cloned_implies_eq_owned`
lemma from vstdplus should handle this. Call it after each clone to establish the link.

### external_body on iterator/from_vec/clone
Read `src/standards/collection_iterators_standard.rs` for the iterator pattern.
Read clean iterator implementations in `src/Chap18/ArraySeqStEph.rs` as reference.
For from_vec, look at how other modules build from Vec (e.g., Chap41 ArraySetStEph).

### next_key < usize::MAX assume
In Round 7, Agent 1 added `lemma_height_le_size` and strengthened wf with size bounds
(`next_key < usize::MAX && cached_size + 1 < usize::MAX`). Verify these are working
and propagate the size bound through trait requires for growth functions (push_back,
insert_value, append, set_ith).

## Standards to Read First

1. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone workaround
2. `src/standards/collection_iterators_standard.rs` — iterator pattern
3. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — MtPer pattern
4. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridges

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap37/ # track hole reduction
```

## Target

**Holes**: 38 → ≤ 15. Close BSTSplayStEph. Get AVLTreeSeq and AVLTreeSeqStEph
to eq/clone-only holes. Eliminate all value-linking assumes in MtPer/StPer.

## When Done

Push to `agent1/ready`. Write `plans/agent1-round8-report.md` with:
- Holes before/after per file (table)
- Verification counts
- Techniques used
- Remaining holes with what blocks them
- Commit hash
