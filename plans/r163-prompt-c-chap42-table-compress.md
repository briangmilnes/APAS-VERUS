# R163 Prompt C — Compress Table proof functions. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER delete `target/` or any subdirectory.**
6. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
7. **NEVER weaken `ensures` clauses.**

## Goal

Reduce proof lines across the three Table files. These are St/Mt/Per
variants of the same hash table — their proof patterns are heavily
duplicated. Extract shared proofs into lemmas that serve all three files.

## Target files and functions

All functions over 100 proof lines in these files:

**`src/Chap42/TableMtEph.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 1 | union | exec | 332 |
| 2 | lemma_entries_to_map_subseq_value | proof | 200 |
| 3 | insert | exec | 156 |
| 4 | intersection | exec | 112 |

**`src/Chap42/TableStEph.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 5 | union | exec | 229 |
| 6 | insert_wf | exec | 199 |
| 7 | insert | exec | 131 |
| 8 | delete_wf | exec | 114 |
| 9 | intersection | exec | 100 |

**`src/Chap42/TableStPer.rs`**

| # | Function | Kind | Proof Lines |
|---|----------|------|-------------|
| 10 | insert_wf | exec | 265 |
| 11 | insert | exec | 182 |
| 12 | union | exec | 164 |
| 13 | delete_wf | exec | 154 |
| 14 | intersection | exec | 114 |
| 15 | delete | exec | 108 |
| 16 | filter | exec | 100 |

Total: ~2,660 proof lines across 16 functions in 3 files.

## Approach: Think deeply before coding

**Step 1 — Read all three files.** Read all standards first. Understand
how TableStEph, TableStPer, and TableMtEph relate. They implement the
same trait with the same operations — their proofs should be structurally
identical.

**Step 2 — Cross-file pattern analysis.** This is the key insight:

- **`union` appears in all 3 files** (332 + 229 + 164 = 725 lines).
  Compare them side by side. The proof structure is likely identical
  with different variable types. Can you write one lemma that all three
  call?
- **`insert` / `insert_wf`** appear in all 3 files (156 + 199+131 + 265+182
  = 933 lines). Same question.
- **`intersection`** in all 3 (112 + 100 + 114 = 326 lines).
- **`delete_wf` / `delete`** in StEph and StPer (114 + 154+108 = 376).

If the same proof pattern appears in St and Mt variants, consider:

1. A shared proof lemma in one file that the others import, OR
2. A proof lemma in each file (if the types differ too much for sharing),
   but with the same structure.

**Step 3 — Look for `lemma_entries_to_map_subseq_value` (200 lines).**
This is a pure proof function. Can it be decomposed into smaller lemmas?
Does it prove a property that could be stated more directly?

**Step 4 — Write lemmas and compress.** For each extracted pattern:

1. Write the lemma, validate.
2. Replace all occurrences across all 3 files.
3. Validate after each file change.

## Validation

```bash
scripts/validate.sh isolate Chap42
```

## Report

Write `plans/agent3-round163-report.md` with:

- Table of lemmas created: name, file, callers across files, lines saved
- Table of 16 functions: proof lines before/after (grouped by file)
- Cross-file sharing: which lemmas serve multiple files
- Total proof lines removed

## RCP

`git add -A && git commit -m "R163 Agent 3: compress Table proof (−N lines)"`, then `git push`.
