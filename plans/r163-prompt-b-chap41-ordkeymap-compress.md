# R163 Prompt B — Compress OrdKeyMap proof functions. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER delete `target/` or any subdirectory.**
6. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
7. **NEVER weaken `ensures` clauses.**

## Goal

Reduce proof lines in OrdKeyMap.rs — a single file with 11 functions over
100 proof lines. Many of these functions are set operations (union, intersect,
split, select) that traverse the same BST structure and prove the same
invariants. Extract shared proof patterns into lemmas.

## Target file and functions

`src/Chap41/OrdKeyMap.rs` — all functions over 100 proof lines:

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 1 | ordkeymap_select | exec | 281 |
| 2 | union_with | exec | 237 |
| 3 | ordkeymap_prev | exec | 205 |
| 4 | ordkeymap_next | exec | 201 |
| 5 | ordkeymap_rank | exec | 195 |
| 6 | union | exec | 183 |
| 7 | intersect_with | exec | 166 |
| 8 | split_rank_key | exec | 124 |
| 9 | intersect | exec | 119 |
| 10 | ordkeymap_split | exec | 116 |
| 11 | tabulate | exec | 114 |

Total: ~1,941 proof lines across 11 functions in one file.

## Approach: Think deeply before coding

**Step 1 — Read OrdKeyMap.rs end to end.** Read all standards first.
This is a big file. Understand the data structure (augmented BST mapping
keys to values) and how the spec functions work.

**Step 2 — Identify repeated proof patterns across all 11 functions.**

These functions all operate on the same BST structure. Look for:

- **BST invariant proofs** that appear in every recursive function:
  ordered, size correct, keys match, view correct. If 8 out of 11
  functions prove `ordered(result)` with the same 15-line proof block,
  that's a lemma.
- **Split/join invariant chains.** `union`, `intersect`, `split`,
  `difference` all split a tree and rejoin. The proof that the result
  is ordered after join is likely duplicated across all of them.
- **Rank/select symmetry.** `ordkeymap_prev` and `ordkeymap_next` are
  mirror operations. `ordkeymap_select` and `ordkeymap_rank` are inverses.
  Their proofs likely share structure — one lemma could serve both.
- **Map view equivalences.** Proofs that `result@ == expected_map` after
  a set operation likely follow the same pattern: decompose into left/right
  subtree views, apply the recursive ensures, reassemble.

**Step 3 — Write lemmas.** For each repeated pattern:

1. Extract into `proof fn lemma_...` with minimal requires/ensures.
2. Place lemmas in the same file, in the proof fns section (section 7).
3. Replace every occurrence with a single call.
4. Validate: `scripts/validate.sh isolate Chap41`.

**Step 4 — Check for redundant asserts.** After lemma extraction, some
`// Veracity: NEEDED assert` lines may become unnecessary because the
lemma provides the fact directly. Try commenting out (NOT deleting) and
validate. If it passes, the assert was redundant.

## Validation

```bash
scripts/validate.sh isolate Chap41
```

## Report

Write `plans/agent2-round163-report.md` with:

- Table of lemmas created: name, requires/ensures summary, callers, lines saved
- Table of 11 functions: proof lines before/after
- Total proof lines removed
- Patterns found but not extracted (and why)

## RCP

`git add -A && git commit -m "R163 Agent 2: compress OrdKeyMap proof (−N lines)"`, then `git push`.
