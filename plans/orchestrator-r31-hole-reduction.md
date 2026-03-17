# R31 Plan: Hole Reduction (4 Agents)

## Goal

Close 7 chapters (35 → 42 clean) via mechanical fn_missing_requires fixes,
plus reduce holes in Chap37, Chap39, Chap41, Chap47.

Current: 251 actionable holes, 35 clean chapters.
Target: ~190 holes, 42 clean chapters.

## Agent Assignments

### Agent 1: Close 4 chapters + Chap37 mechanical (24 fixes)

Close these chapters (all purely fn_missing_requires):
- Chap03: InsertionSortStEph.rs (1)
- Chap23: BalBinTreeStEph.rs (1)
- Chap55: SCCStEph.rs (1), SCCStPer.rs (1)
- Chap66: BoruvkaStEph.rs (1)

Then fix Chap37 fn_missing_requires (19):
- BSTSplayStEph.rs (5), BSTSetSplayMtEph.rs (2), BSTSetPlainMtEph.rs (2),
  BSTSetBBAlphaMtEph.rs (2), BSTSetAVLMtEph.rs (2), AVLTreeSeqStEph.rs (2),
  BSTSplayMtEph.rs (1), BSTSetRBMtEph.rs (1), BSTRBMtEph.rs (1),
  AVLTreeSeqStPer.rs (1)

All mechanical: read each function, determine the real precondition
(typically spec_wf or structural), add it. DO NOT add `requires true`.
DO NOT add tautological requires. Read the CLAUDE.md rules on this.

**Expected: 4 new clean chapters, Chap37 drops from 22 to 3.**

### Agent 2: Close 3 chapters + Chap45/59 mechanical (18 fixes)

Close these chapters:
- Chap21: Exercise21_7.rs (2), Exercise21_8.rs (1)
- Chap28: 8 MaxContigSubSum files (1 each)
- Chap42: TableMtEph.rs (1), TableStEph.rs (1), TableStPer.rs (1)

Then fix remaining mechanical:
- Chap45: LeftistHeapPQ.rs (1), HeapsortExample.rs (1), BinaryHeapPQ.rs (1)
- Chap59: JohnsonStEphI64.rs (3)

**Expected: 3 new clean chapters, Chap45 drops from 5 to 2, Chap59 drops from 4 to 1.**

### Agent 3: Chap39 assumes + Chap41 mechanical + proof (17 holes)

Chap39 — prove the 6 assumes in BSTTreapMtEph.rs:
- contains, size, find_first, find_last, in_order, pre_order
- These are reader-predicate results read through RwLock handles.
  The St counterpart (BSTTreapStEph) has the proofs — adapt them.
- Also fix 5 fn_missing_requires in BSTTreapMtEph.rs.

Chap41 — fix 6 fn_missing_requires:
- AVLTreeSetMtEph.rs (4), AVLTreeSetMtPer.rs (2)

**Expected: Chap39 drops from 23 to ~12, Chap41 drops from 27 to ~19.**

### Agent 4: Chap47 mechanical + Chap38 assumes + Chap43 mechanical (17 holes)

Chap47 — fix 5 fn_missing_requires:
- StructChainedHashTable.rs (3), VecChainedHashTableStEph.rs (1),
  LinkedListChainedHashTableStEph.rs (1)

Chap38 — prove the 4 assumes in BSTParaStEph.rs:
- Size arithmetic assumes (left@.len() + right@.len() < usize::MAX).
- Use vstd overflow lemmas or add size bounds from wf predicates.

Chap43 — fix 4 fn_missing_requires:
- OrderedSetStPer.rs (1), OrderedSetStEph.rs (1),
  AugOrderedTableStPer.rs (1), AugOrderedTableMtEph.rs (1)

Chap57/58 — fix 2 fn_missing_requires:
- DijkstraStEphI64.rs (1), BellmanFordStEphI64.rs (1)

**Expected: Chap47 drops from 22 to 17, Chap38 drops from 13 to ~9.**

## No File Conflicts

Agent 1: Chap03, 23, 37, 55, 66.
Agent 2: Chap21, 28, 42, 45, 59.
Agent 3: Chap39, 41.
Agent 4: Chap38, 43, 47, 57, 58.

## Rules Emphasis

- DO NOT add `requires true`. Add REAL preconditions.
- DO NOT add tautological requires (e.g., `0nat <= usize::MAX`).
- DO NOT add `// veracity: no_requires`. Only the user adds these.
- Every quantifier MUST have explicit `#[trigger]`. No `#![auto]`.
- DO NOT add `assume`, `accept`, or `external_body` without user approval.
- Run `scripts/validate.sh` — 0 errors required.

## Expected Outcome

- Clean chapters: 35 → 42 (+7)
- Mechanical holes fixed: ~62
- Assumes proved: ~10
- Total actionable: 251 → ~185
