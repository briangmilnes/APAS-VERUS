# R116 Agent 1 — Strengthen BSTParaMtEph specs to match StEph. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 11 warnings on BSTParaMtEph.rs. The MtEph
variant has weaker specs and missing functions compared to StEph. Most are
fixable by adding requires/ensures clauses that the existing implementations
already satisfy.

## Warnings (11)

### Missing functions (3 fns, 1 warning)

StEph has these, MtEph does not:
- `join_m(left, key, right) -> Self` — join with explicit key
- `min_key(&self) -> Option<T>` — minimum element
- `collect_in_order(&self, out: &mut Vec<T>)` — helper for in_order

These need to be implemented in MtEph. They are internal helpers and
the tree-manipulation logic can be copied from StEph (Mt standalone rule).

### Missing requires (6 warnings)

1. **`insert`** (count 3 vs 4): MtEph is missing `old(self).spec_bstpara_wf()`.
   Note: StEph uses the short name `spec_bstpara_wf` but that's actually
   `spec_bstparasteph_wf`. MtEph should already require
   `old(self).spec_bstparamteph_wf()` — check if it's there under the
   MtEph name. If the wf is genuinely missing, add it.

2. **`delete`** (count 2 vs 4): MtEph missing `old(self)@.len() < usize::MAX`
   and `spec_bstpara_wf`. StEph has both. Add the `len < MAX` requires if
   the delete impl needs it. Add wf if missing.

3. **`join_pair`** (count 3 vs 7): MtEph missing:
   - `self@.disjoint(other@)`
   - `forall|s, o| self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less`

   StEph requires these because `join_pair` is a sorted-merge of two disjoint
   BSTs. The MtEph version has `join_pair_inner` with these requires but the
   outer `join_pair` drops them. Check whether the MtEph `join_pair` impl
   calls `split` first (which doesn't need disjointness) or goes directly to
   `join_pair_inner` (which does). Add the requires that the impl actually needs.

### Missing ensures (4 warnings)

1. **`join_pair`**: Tool says StEph has `joined@ =~= self@.union(other@)` with
   no match, BUT MtEph has `joined@ == self@.union(other@)`. The `==` is
   strictly stronger than `=~=`. This is a **false positive** — no fix needed.

2. **`reduce`**: StEph has `ensures self@.len() == 0 ==> reduced@ == base@`.
   MtEph has no ensures at all. Add the ensures if the impl can prove it.

3. **`in_order`** (count 1 vs 2): MtEph has `seq@.len() == self@.len()` but
   missing `forall|v| self@.contains(v) <==> seq@.contains(v)`. Add it if
   the impl proves it.

## Work order

1. Read `src/Chap38/BSTParaMtEph.rs` — full file.
2. Read `src/Chap38/BSTParaStEph.rs` — the StEph trait and impls for reference.
3. Fix the missing requires on `insert`, `delete`, `join_pair`.
4. Fix the missing ensures on `reduce`, `in_order`.
5. Implement `join_m`, `min_key`, `collect_in_order` in MtEph.
6. Validate with `scripts/validate.sh isolate Chap38`.
7. Run RTTs: `scripts/rtt.sh Chap38`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from StEph. Duplicate what you need.
- No subagents.
- If a requires/ensures can't be proved, leave it out and report why.

## STEP 30

## Report

Write `plans/agent1-r116-chap38-specs-report.md`.
