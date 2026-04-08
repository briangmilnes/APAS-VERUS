# R166 Prompt D — Agent 4: Lift repeated proof patterns in Chap42 Table + Chap40. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent4`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER weaken `ensures` clauses.**

## Goal

Chap42 Table has 3 files with 38 combined functions ≥50 proof lines (4367 total).
R165 already extracted `TableSpecsAndLemmas.rs` with shared specs and 12 lemmas.
Now find the within-function repeated assert patterns that can become MORE lemmas
in `TableSpecsAndLemmas.rs`.

Chap40 BST{Size,Reduced,KeyValue}StEph has 3 files, ~53K tokens — smaller but
still has within-module duplication.

## Your files

**Chap42:**
- `src/Chap42/TableSpecsAndLemmas.rs` — ADD lemmas here
- `src/Chap42/TableMtEph.rs` — 14 fns ≥50 lines (1568 total)
- `src/Chap42/TableStEph.rs` — 11 fns ≥50 lines (1242 total)
- `src/Chap42/TableStPer.rs` — 13 fns ≥50 lines (1557 total)

**Chap40:**
- `src/Chap40/BSTSizeStEph.rs`
- `src/Chap40/BSTReducedStEph.rs`
- `src/Chap40/BSTKeyValueStEph.rs`

## Approach

**Step 1 — Read all files and all standards.**

**Step 2 — Cross-function patterns in Table.** union, insert, insert_wf,
delete, delete_wf, intersection, difference, filter, restrict, subtract
all operate on entries sequences. They all prove:
- Result entries map to the correct Map
- Keys are unique after operation
- Result is well-formed

Find the SPECIFIC assert blocks that repeat and extract them as lemmas
into `TableSpecsAndLemmas.rs`.

**Step 3 — Cross-variant patterns.** The 3 Table files implement the same
operations. If TableMtEph's `union` proof has the same structure as
TableStEph's `union` proof, the shared part is a lemma.

**Step 4 — Chap40.** Size, Reduced, and KeyValue BSTs share the same
underlying BST structure with different augmentation. Their insert/delete
proofs likely share ordering and containment sub-proofs.

**Step 5 — Validate:**
```bash
scripts/validate.sh isolate Chap42
scripts/validate.sh isolate Chap40
```

## Report

Write `plans/agent4-round166-report.md`.

## RCP

`git add -A && git commit -m "R166 Agent 4: lift proof patterns Chap42+40 (−N lines)"`, then `git push`.
