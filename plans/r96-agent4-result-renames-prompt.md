# R96 Agent 4 — Rename (result:) returns to meaningful names, STEP 15

## Objective

CLAUDE.md says: "Name return values meaningfully (count, out_neighbors,
contains), not generically (result, ret, value)."

Rename all `(result: ...)` named returns to meaningful names in algorithm files.
Chapters 49, 53, and 55 are already done. Do the rest.

## Method

For each file with `(result:`:

1. Read the function — understand what it returns
2. Pick a meaningful name:
   - Constructors (`new`, `empty`, `from_*`): use the thing being constructed
   - Queries (`find`, `has_*`, `contains`): use `found`, `has_edge`, `contains`
   - Computations: use what's computed (`weight`, `distance`, `count`)
   - Clone: `cloned`
3. Rename in the signature AND all ensures/proof blocks that reference `result`
4. Do NOT rename `result` used as a local variable in function bodies
5. Validate after each chapter

## Chapters to fix (agents not touching these)

Skip Chap43 and Chap52 (other agents working there). Do these:

| Chap | Files |
|------|-------|
| 37 | 3 files |
| 38 | 2 files |
| 39 | 2 files |
| 41 | 3 files |
| 44 | 1 file |
| 47 | 1 file |
| 57 | 2 files |
| 58 | 1 file |
| 61 | 2 files |
| 62 | 4 files |
| 63 | 2 files |
| 64 | 3 files |
| 65 | 2 files |
| 66 | 1 file |

## How to find them

```bash
grep -rn "(result:" src/Chap*/  --include="*.rs" | grep -v "Chap43\|Chap52\|Chap49\|Chap53\|Chap55\|Chap59\|Chap26"
```

## Isolation

Validate per-chapter as you go:
```bash
scripts/validate.sh isolate ChapNN
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT touch Chap43, Chap52, Chap49, Chap53, Chap55, Chap59, Chap26.
- Do NOT rename `result` that appears as a local variable in function bodies.
- Every rename must update BOTH the signature AND all ensures/proof references.
- Validate after each batch of changes.
- If a rename causes an error, the ensures reference was missed — find and fix it.

## STEP 15

## Report

Write `plans/agent4-r96-result-renames-report.md`.
