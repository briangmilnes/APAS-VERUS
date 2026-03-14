# Agent 1 — Round 11 Prompt

## Mission

Start Chap43 (145 holes — the largest remaining chapter). Continue Chap37 (5 holes)
and Chap45 (3 holes) as secondary.

## Your Files

**Chap43** (145 holes across 11 files — PRIMARY):
- `OrderedTableStEph.rs` — 16 external_body
- `OrderedTableStPer.rs` — 10 external_body
- `OrderedTableMtEph.rs` — holes TBD (read the file)
- `OrderedTableMtPer.rs` — holes TBD
- `AugOrderedTableStEph.rs` — holes TBD
- `AugOrderedTableStPer.rs` — holes TBD
- `AugOrderedTableMtEph.rs` — holes TBD
- `OrderedSetStEph.rs` — holes TBD
- `OrderedSetStPer.rs` — holes TBD
- `OrderedSetMtEph.rs` — holes TBD
- `Example43_1.rs` — SKIP (Example file)

**Chap37** (5 real holes — SECONDARY):
- `AVLTreeSeq.rs` — 1 external_body (next iterator)
- `AVLTreeSeqStPer.rs` — 1 assume (build_balanced clone)
- `AVLTreeSeqMtPer.rs` — 2 external_body (thread boundaries) + 1 trivial_wf
- `BSTSplayStEph.rs` — 1 trivial_wf (you added spec_is_bst_link foundation)

**Chap45** (3 holes — SECONDARY):
- `BinaryHeapPQ.rs` — 1 assume (sorted)
- `BalancedTreePQ.rs` — 1 external (external impl)

## Priority Order

1. **Chap43** — Read ALL 10 algorithm files. Run `scripts/holes.sh src/Chap43/` to get
   per-file counts. Identify which functions are provable NOW vs blocked on deps.
2. Prove the easy Chap43 external_body stubs first (new, size, find, insert, delete —
   these are AVL tree operations that delegate to Chap37 AVLTreeSeq).
3. Continue Chap37/45 only if stuck on Chap43.

## Specific Guidance for Chap43

Chap43's OrderedTable is backed by AVLTreeSeq (Chap37). The ordered table operations
(collect, first_key, last_key, split, rank, select) are tree traversals.

**Start with the StEph file** — it has the most external_body (16). Read it, understand
the backing data structure, then prove functions one at a time.

**Dep warning**: Chap43 depends on Chap37 (AVLTreeSeqStEph, AVLTreeSeqStPer) and
Chap41 (ArraySetStEph, AVLTreeSetStEph, AVLTreeSetStPer). Some proofs may need specs
from these chapters. Agent 4 is working on Chap41. Prove what you can without waiting.

**The ordered set files** (OrderedSetStEph, StPer, MtEph) wrap OrderedTable with
key-only semantics. If OrderedTable is proved, OrderedSet should follow.

## Chap37/45 — What's Left

You said these are irreducible in Round 10:
- **AVLTreeSeq next()**: Iterator::next can't have requires. Try: add a `valid` field
  to the iterator struct that tracks whether wf holds. Check the iterator standard.
- **AVLTreeSeqStPer build_balanced clone**: Cascades feq to callers. This IS the work —
  do the cascade to Chap41/43/45 callers. Agent 4 owns Chap41, so coordinate: only
  update callers in YOUR files (Chap43, Chap45).
- **BinaryHeapPQ sorted**: Needs heap property invariant infrastructure. If you have time
  after Chap43 work, start building the `spec_is_heap` predicate.
- **Thread boundaries (MtPer)**: Acceptable external_body. Leave them.

## DO NOT TOUCH (other agents' files)

- Chap38, Chap39 — Agent 3
- Chap41, Chap53 — Agent 4

## Rules

- Read `src/standards/*.rs` before modifying code.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent1/ready`. Write `plans/agent1-round11-report.md`.
