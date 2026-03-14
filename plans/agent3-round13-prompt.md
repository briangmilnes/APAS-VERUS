# Agent 3 — Round 13 (RESTART)

## You were the star last run (-16). Keep going.

ArraySetStEph is CLEAN. AVLTreeSetStEph 14→6. AVLTreeSetStPer 10→5. Excellent.
Now finish what's left and hit BSTParaStEph.

## Remaining holes you own

**Chap41** (11 holes across 2 files):
- `AVLTreeSetStEph.rs` — 6 assume remaining:
  - `size`: seq.len vs set.len needs `no_duplicates` invariant in wf
  - `find` not-found: needs sorted invariant in wf
  - `delete` postcondition (2): `from_vec` to `to_set()` gap
  - `insert` postcondition (2): same gap
- `AVLTreeSetStPer.rs` — 5 assume remaining:
  - `size`: same gap
  - `filter`: closure requires needs trait spec precondition
  - `find` not-found: same sorted invariant gap

**Chap38** (15 holes):
- `BSTParaStEph.rs` — 15 assume (you got 0 delta last run, all T::V witness)

## Strategy

For the Chap41 remaining 11:
- **size**: Add `no_duplicates` to `spec_avltreesetsteph_wf`. The inner
  AVLTreeSeq ensures no duplicates — thread that through.
- **find not-found**: Add sorted invariant to wf. The inner tree is sorted.
- **delete/insert postconditions**: The `from_vec` → `to_set()` gap needs
  a lemma connecting `Seq::to_set` after insert/delete to `Set::insert/remove`.

For BSTParaStEph (15):
- You said "T::V witness gap — cannot bridge view-level set membership to
  value-level cmp_spec." Try: can you add a spec function relating view
  membership to value ordering? Or restructure the set to use value-level?

## DO NOT

- Touch Chap43 (Agents 1 and 2)
- Touch Chap42, Chap47 (Agent 4)
- Touch Chap41 Mt files (Agent 4)

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent3/ready`. Write `plans/agent3-round13-report.md`.

## Target: AVLTreeSetStEph 6 → ≤ 2. AVLTreeSetStPer 5 → ≤ 2. BSTParaStEph 15 → ≤ 10. Total -12.
