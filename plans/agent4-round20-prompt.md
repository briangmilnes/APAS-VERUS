# Agent 4 — Round 20: Tier 2 Spec Audit — Round C (13 chapters)

## Mission

Audit every exec fn's `requires`/`ensures` in the remaining clean algorithm chapters
against APAS textbook prose. These are the chapters NOT yet audited:

Chap03, 11, 17, 27, 28, 30, 35, 36, 40, 44, 49, 50, 51

R19 audited Chap05/06/18/19/21/23/26. This round covers everything else except
graph algorithms (Chap52-66, saved for Round D) and holed chapters (already audited
in Tier 1).

## Procedure

For each chapter:

1. **Read** `prompts/ChapNN.txt` — the APAS textbook prose defining the ADT/algorithm.
2. **Read** the StEph trait file(s) — the source-of-truth spec signatures.
3. **Read** `src/ChapNN/analyses/veracity-review-module-fn-impls.md` — function inventory.
4. **Compare** each trait fn's ensures against the prose definition.
5. **Classify** each fn spec as:
   - **strong**: ensures faithfully encodes the prose
   - **partial**: ensures present but missing key properties
   - **weak**: ensures only structural (wf, len, finite, true)
   - **missing**: no requires/ensures at all
6. **Write** `src/ChapNN/analyses/spec-audit.md` — per-function classification table.
7. **Fix** weak/missing/partial specs by writing correct requires/ensures in the trait.
8. Add `external_body` to impl fns that can't prove the strengthened spec.
9. `scripts/validate.sh` — 0 errors.

## Chapters to Audit

| # | Chapter | Topic | Files | Key Operations |
|---|---------|-------|-------|---------------|
| 1 | Chap03 | Sorting (InsertionSort) | 1 | insertion_sort |
| 2 | Chap11 | LinkedList | 5 | push, pop, nth, insert, delete |
| 3 | Chap17 | Randomized skip list | 1 | find, insert, delete |
| 4 | Chap27 | Merging/sorting variants | 4 | merge, sort |
| 5 | Chap28 | Contraction | 11 | contract, compact |
| 6 | Chap30 | Sorting networks | 1 | bitonic_sort, merge_network |
| 7 | Chap35 | BSTs (basic) | 4 | find, insert, delete, rotate |
| 8 | Chap36 | Treaps (basic) | 3 | find, insert, delete, split, join |
| 9 | Chap40 | Augmented BST | 3 | select, rank, augment |
| 10 | Chap44 | AugSet/AugTable | 2 | aug_reduce, aug_filter |
| 11 | Chap49 | (I)Tables | 8 | insert, lookup, delete |
| 12 | Chap50 | Unionfind | 8 | find, union, link |
| 13 | Chap51 | Hashing variants | 8 | insert, lookup, delete |

## What "Strong" Looks Like

```rust
// Sorting: result is a permutation AND sorted
ensures sorted.to_multiset() =~= old(a).to_multiset(),
        sorted.is_sorted(),

// LinkedList: push adds to front
ensures self@ == old(self)@.push_front(v@),

// BST: find returns membership
ensures found <==> self@.contains(key@),

// PQ: find_min has TotalOrder minimality
ensures min matches Some(v) ==> forall|t: T| self@.contains(t@) ==> TotalOrder::le(v, t),

// Sorting network: sorted + permutation
ensures result.to_multiset() =~= input.to_multiset(), result.is_sorted(),
```

## Expected Results

Based on R19's findings (Chap06/21/23/26 were all strong), many of these chapters
are likely already strong. Focus your effort on chapters where you find actual gaps:

- Chap03 (InsertionSort) — check for permutation + sorted ensures.
- Chap35/36 (BST/Treaps) — these were the Tier 1 audit targets' dependencies.
  Should have strong specs.
- Chap44 (AugSet/AugTable) — check augmentation function ensures.
- Chap49/50/51 — these are newer chapters, check carefully.

## Important

- If a chapter is already strong, say so. Don't change code unnecessarily.
- Only fix genuinely weak/missing specs.
- The prose is the source of truth, not the current code.
- Do NOT modify holed chapters (Chap05/19/37-43/45/47/52).
- Skip Example files (Example*.rs).
- Skip Chap52-66 (graph algorithms — saved for Round D).

## Deliverables

- `src/ChapNN/analyses/spec-audit.md` for each of the 13 chapters.
- Strengthened ensures where needed.
- `plans/agent4-round20-report.md` with per-chapter summary table.
- 0 errors on validate.
- Commit + push to `agent4/ready`.
