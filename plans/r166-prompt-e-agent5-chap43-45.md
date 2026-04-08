# R166 Prompt E — Agent 5: Lift repeated proof patterns in Chap43 + Chap45. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent5`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Chap43 OrderedTable has 5 files ≥50 proof lines (1324+941 in StEph+StPer alone).
R165 extracted `OrderedSpecsAndLemmas.rs`. Now find within-function patterns.

Chap45 priority queues: BinaryHeapPQ (1068 lines in 5 fns) and SortedListPQ
(1055 in 3 fns) are the densest.

## Your files

**Chap43:** (skip Example files)
- `src/Chap43/OrderedSpecsAndLemmas.rs` — ADD lemmas here
- `src/Chap43/OrderedTableStEph.rs` — 5 fns ≥50 (1324 total)
- `src/Chap43/OrderedTableStPer.rs` — 5 fns ≥50 (941 total)
- `src/Chap43/OrderedTableMtEph.rs`, `OrderedTableMtPer.rs`
- `src/Chap43/OrderedSetStEph.rs`, `OrderedSetStPer.rs`
- `src/Chap43/AugOrderedTable{St,Mt}Eph.rs`, `AugOrderedTableStPer.rs`

**Chap45:**
- `src/Chap45/BinaryHeapPQ.rs` — 5 fns ≥50 (delete_min 128, bubble_up 105, bubble_down 96, extract_all 67)
- `src/Chap45/SortedListPQ.rs` — meld (237), insert (138)
- `src/Chap45/LeftistHeapPQ.rs` — meld_nodes (54), lemma_heap_root_is_min (59)
- `src/Chap45/UnsortedListPQ.rs` — delete_min (70), extract_all (56)

## Approach

**Step 1 — Read all files and all standards.**

**Step 2 — Chap43 OrderedTable.** These are BST-backed sorted tables.
union, insert, delete, restrict, subtract, from_sorted_entries all prove
sortedness and key-uniqueness after modification. Find shared patterns,
add lemmas to `OrderedSpecsAndLemmas.rs`.

**Step 3 — Chap45 BinaryHeapPQ.** bubble_up and bubble_down both prove
the heap property is maintained. delete_min and extract_all both prove
elements are preserved. These are within-file extraction candidates.

**Step 4 — Chap45 SortedListPQ.** meld (237 lines) is the big one.
insert calls meld. If insert's proof is a special case of meld's proof,
extract the shared part.

**Step 5 — Validate:**
```bash
scripts/validate.sh isolate Chap43
scripts/validate.sh isolate Chap45
```

## Report

Write `plans/agent5-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 5: lift proof patterns Chap43+45 (−N lines)"`, then `git push`.
