# R93 Agent 4 — Tocify all source files, STEP 10

## Objective

Run `veracity-tocify fix` on the codebase to standardize all Table of Contents
headers. Then validate, and hand-check 5 random files.

## Steps

### 1. Run tocify

```bash
~/projects/veracity/target/release/veracity-tocify fix ~/projects/APAS-VERUS \
  -e experiments -e vstdplus -e Types.rs -e Concurrency.rs -e standards -e lib.rs
```

This fixes TOC headers in all algorithm source files, excluding infrastructure.

### 2. Validate

```bash
scripts/validate.sh
```

Must have the same verified count as before (±0). Tocify only changes comments,
not code. If the verified count changes, something went wrong.

### 3. Hand-check 5 random files

```bash
find src/Chap* -name "*.rs" | shuf | head -5
```

For each file, read the TOC and verify:
- Sections are numbered correctly (1-14 per standard)
- Section names match the standard (e.g., "4. type definitions")
- Letter suffixes used for multi-type files
- Bottom-up ordering (leaf types before composites)

### 4. Check the TOC standard itself

Read `src/standards/table_of_contents_standard.rs` and confirm the tocify
output matches the standard's prescribed format.

## Isolation

Full validate (tocify touches many files):
```bash
scripts/validate.sh
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT manually edit TOC headers — let tocify do it.
- If tocify breaks a file (parse error, wrong section), revert that file
  with `git checkout -- <file>` and report the problem.
- If tocify changes code (not just comments), revert immediately.
- The verified count must match baseline. Tocify is comment-only.

## STEP 10

## Report

Write `plans/agent4-r93-tocify-report.md`.
