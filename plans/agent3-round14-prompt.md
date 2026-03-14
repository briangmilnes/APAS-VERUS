# Agent 3 — Round 14

## You got -16 in R13 session 1, then 0 on the restart. Time for the hard ones.

The feq broadcast and set operation loop invariants were your best work. Now apply
similar reasoning to the remaining Chap41 St holes and take a crack at Chap38.

## Your files

**Chap41 — AVLTreeSetStEph.rs (6 assume):**

| # | Line | Hole | What's needed |
|---|------|------|---------------|
| 1 | 175 | size: `assume(r == self@.len())` | Seq.len == Set.len when no_duplicates. Add `no_duplicates` to wf, prove from sorted invariant |
| 2 | 534 | find not-found: `assume(!self@.contains(x@))` | Need sorted invariant in wf. Binary search exhausted all positions → not in set |
| 3 | 557 | delete: `assume(result_vec@.len() < usize::MAX)` | Subset of original, original fits in usize |
| 4 | 561 | delete: `assume(self@ == old(self)@.remove(x@))` | Loop built result_vec without x. Need lemma: seq_without_elem.to_set == set.remove |
| 5 | 611 | insert: `assume(new_vec@.len() < usize::MAX)` | Original + 1, original < usize::MAX - 1 |
| 6 | 616 | insert: `assume(self@ == old(self)@.insert(x_view))` | Loop inserted x at sorted position. Need lemma: seq_with_inserted.to_set == set.insert |

Strategy:
- **size**: The elements are stored in an AVLTreeSeqStEph which maintains sorted order.
  Sorted order implies no_duplicates. `seq_to_set` of a no-dup seq has same len. Use
  `vstd::seq_lib::lemma_seq_to_set_is_no_dup_has_same_len` or similar.
- **find not-found**: The loop scans all elements and none matched. After exhausting
  the seq, assert `!self.elements@.contains(x@)` then use seq_to_set containment.
- **delete/insert postconditions**: Build the connection between the Vec you constructed
  and `to_set()`. You may need a custom lemma: "if seq' = seq.filter(|e| e != x),
  then seq'.to_set() == seq.to_set().remove(x)".
- **Vec length bounds**: Original fits in usize. Delete produces smaller. Insert adds 1,
  assert `old(self).elements@.len() < usize::MAX - 1` (or add to requires).

**Chap41 — AVLTreeSetStPer.rs (5 assume):**

| # | Line | Hole | What's needed |
|---|------|------|---------------|
| 1 | 158 | size: same as StEph | Same fix |
| 2 | 247 | filter: `assume(f.requires((&*elem,)))` | Pred trait missing requires. Add to fn requires or use existing trait bound |
| 3 | 522 | find not-found: same as StEph | Same fix |
| 4 | 546 | delete postcondition | Same pattern as StEph |
| 5 | 612 | insert postcondition | Same pattern as StEph |

**Chap38 — BSTParaStEph.rs (15 assume) — stretch goal:**

These are the T::V witness gap holes. All are about proving set disjointness and
containment after recursive split+union/intersect/difference. The gap is: set elements
are `T::V` but ordering is `T::cmp_spec`. You need to bridge from "x is in the set of
views" to "there exists a T with that view, and it compares correctly."

Try: add a spec function to the BST invariant relating set membership to ordering:
```
forall|v: T::V| self@.contains(v) ==> exists|t: T| t@ == v && t.cmp_spec(&key) == ...
```
This is a stretch — if it doesn't work after 20 minutes, report what you tried and move on.

## DO NOT

- Touch Chap43 (Agents 1 and 2)
- Touch Chap41 Mt files (Agent 4)
- Touch Chap39, Chap42, Chap47 (Agent 4)
- Touch Example files

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent3/ready`. Write `plans/agent3-round14-report.md`.
- Start with size (easiest), then find, then delete/insert.

## Target: AVLTreeSetStEph 6 → ≤ 2. AVLTreeSetStPer 5 → ≤ 2. BSTParaStEph -3 stretch. Total -8.
