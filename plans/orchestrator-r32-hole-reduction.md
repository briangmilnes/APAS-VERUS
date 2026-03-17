# R32 Plan: Real Proof Work (4 Agents)

## Goal

Reduce real proof holes (assume + external_body) in the 5 largest holed chapters.
Mechanical fixes are done (fn_missing_requires = 0). This round is proof engineering.

Current: 196 actionable holes, 35 clean chapters.
Target: ~160-170 holes (26-36 real proofs closed).

## Strategy

Focus on provable holes first — reader-predicate assumes, iterator proofs,
insert/find/delete with existing St counterpart proofs to adapt. Skip
parallel set operations (union/intersect/difference/filter) and View-through-Arc
stubs — those are structurally hard and low ROI this round.

## Agent Assignments (no file conflicts)

### Agent 1: Chap37 (3 holes) + Chap38 (2 provable)

**Chap37** — close the chapter (3 external_body):
- AVLTreeSeq.rs: `next` iterator — prove in-order traversal matches spec
- AVLTreeSeqMtPer.rs: `build_balanced_from_slice` — parallel construction
- AVLTreeSeqMtPer.rs: `subseq_copy` — parallel subsequence extraction

**Chap38** — prove 2 of 10:
- BSTParaStEph.rs: `expose` assume (clone key equality) — try Clone ensures
- BSTParaStEph.rs: `clone` external_body — RwLock recreation

Skip BSTParaMtEph.rs parallel ops (union/intersect/difference/filter/reduce)
this round — they need the Param framework verification first.

**Expected: Chap37 closes (new clean chapter). -3 to -5 holes.**

### Agent 2: Chap39 (8 provable of 20)

**BSTTreapMtEph.rs** — prove 6 reader-predicate assumes:
- `find`, `size`, `minimum`, `maximum`, `in_order`, `pre_order`
- Pattern: acquire read lock, call St helper on locked root, bridge
  locked state to spec via RwLockPredicate inv. The St counterpart
  BSTTreapStEph.rs has the real proofs — adapt the Mt wrapper.

**BSTTreapStEph.rs** — prove 2 external_body:
- `find`: BST search — prove correctness against set spec
- `insert_link`: recursive insertion with heap rebalancing

Skip BSTParaTreapMtEph.rs (10 parallel ops) — same structural pattern
as Chap38 parallel framework, defer.

**Expected: -6 to -8 holes.**

### Agent 3: Chap41 (5 provable of 20)

**AVLTreeSetStEph.rs** — prove 1 assume:
- `insert`: size bound assume (`new_vec.len() < usize::MAX`)
  Derive from existing capacity/wf invariants.

**AVLTreeSetMtEph.rs** — prove 2 assumes:
- `size`: reader-predicate bridge (read lock → spec len)
- `find`: reader-predicate bridge (read lock → spec contains)

**AVLTreeSetMtPer.rs** — prove 1 assume + 1 external_body:
- `find`: binary search loop exit → element not found
- `cmp`: lexicographic comparison (Ord impl)

Also fix remaining fn_missing_requires/ensures (3 total):
- AVLTreeSetMtEph.rs: `parallel_filter` fn_missing_ensures
- AVLTreeSetMtEph.rs: `parallel_intersect` fn_missing_ensures
- AVLTreeSetMtPer.rs: `parallel_sort` fn_missing_ensures

Skip external_body parallel ops (filter/union/intersection/difference/
from_seq/to_seq/delete/insert in MtPer) — defer to later round.

**Expected: -5 to -8 holes.**

### Agent 4: Chap45 (2 holes) + Chap47 (6 provable of 17)

**Chap45** — prove 2, potentially close chapter:
- BinaryHeapPQ.rs: `extract_all_sorted` assume — prove sortedness
  from heap extract loop invariant
- BalancedTreePQ.rs: `insert` external_body — prove tree insertion

**Chap47** — prove lookup assumes + some insert/delete:
- DoubleHashFlatHashTableStEph.rs: 2 lookup assumes (probe chain
  arithmetic, wrapping mod)
- QuadProbFlatHashTableStEph.rs: 1 lookup assume (quadratic probe mod)
- LinProbFlatHashTableStEph.rs: `insert` external_body (linear probe)
- DoubleHashFlatHashTableStEph.rs: `insert` external_body
- QuadProbFlatHashTableStEph.rs: `insert` external_body

Skip resize operations (5 files) — they rebuild the entire table and
are lower priority than insert/lookup/delete correctness.

**Expected: Chap45 closes (new clean chapter). -5 to -8 holes.**

## No File Conflicts

- Agent 1: Chap37, Chap38
- Agent 2: Chap39
- Agent 3: Chap41
- Agent 4: Chap45, Chap47

## Expected Outcome

- Holes: 196 → ~160-170 (26-36 reduction)
- Clean chapters: 35 → 37 (Chap37, Chap45 close)
- Chap39 assumes eliminated (unblocks internal holes)
- Chap41 assumes eliminated (progress toward unblocking Chap43)

## Verification

Each agent runs `scripts/validate.sh` after changes (0 errors required).
Merge order: any (chapter-disjoint assignments).
