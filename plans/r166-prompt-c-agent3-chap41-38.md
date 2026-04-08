# R166 Prompt C — Agent 3: Lift repeated proof patterns in Chap41 OrdKeyMap + Chap38 BSTParaSt/Mt. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent3`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Chap41 OrdKeyMap has 20 functions over 50 proof lines (2641 lines total) — the
single densest module. R163 extracted 3 lemmas and saved 206 lines. There's
much more to extract: 20 functions doing BST set operations all prove
ordering, containment, and view-map equivalence after each modification.

Chap38 BSTParaStEph (964 lines in 4 fns) and BSTParaMtEph (943 in 4 fns)
are the parametric BST — same pattern, different type bounds.

## Your files

**Chap41:**
- `src/Chap41/OrdKeyMap.rs` — 20 fns ≥50 proof lines, 2641 total
- `src/Chap41/ArraySetStEph.rs` — related set operations
- `src/Chap41/ArraySetEnumMtEph.rs`
- `src/Chap41/AVLTreeSet{St,Mt}{Eph,Per}.rs`

**Chap38:**
- `src/Chap38/BSTParaStEph.rs` — 4 fns ≥50 (split, intersect, difference, union)
- `src/Chap38/BSTParaMtEph.rs` — 4 fns ≥50 (same operations, Mt)

## Approach

**Step 1 — Read all files and all standards.**

**Step 2 — OrdKeyMap deep analysis.** 20 functions. Look for:
- Post-modification BST invariant proofs (ordered, contains, view-map)
  repeated in union, intersect, split, difference, restrict, subtract,
  map_values, tabulate, select, rank, next, prev, find
- Loop invariant proof patterns shared across multiple operations
- The R163 lemmas (`lemma_view_gen_empty`, `lemma_freshness_from_sorted`,
  `lemma_map_dom_preserved_by_superset`) serve 6-8 sites each. Find MORE
  patterns at that frequency.

**Step 3 — BSTParaSt/Mt.** split, intersect, difference, union all traverse
BSTs. The St and Mt proofs should be structurally identical. Extract shared
parametric BST lemmas.

**Step 4 — Validate:**
```bash
scripts/validate.sh isolate Chap41
scripts/validate.sh isolate Chap38
```

## Report

Write `plans/agent3-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 3: lift proof patterns Chap41+38 (−N lines)"`, then `git push`.
