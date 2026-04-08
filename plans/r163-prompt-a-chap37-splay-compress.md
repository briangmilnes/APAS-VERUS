# R163 Prompt A — Compress BSTSplay proof functions. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER delete `target/` or any subdirectory.**
6. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
7. **NEVER weaken `ensures` clauses.**

## Goal

Reduce proof lines in the two splay functions — the largest proof functions
in the entire codebase. Extract repeated proof patterns into lemmas.

## Target files and functions

All functions over 100 proof lines in these files:

| # | Chap | File | Function | Kind | Proof Lines |
|---|------|------|----------|------|-------------|
| 1 | 37 | BSTSplayMtEph.rs | splay | exec | 718 |
| 2 | 37 | BSTSplayStEph.rs | splay | exec | 694 |

## Approach: Think deeply before coding

**Step 1 — Read both splay functions end to end.** Read all standards first.
These functions are 700+ lines of proof each. Before touching anything,
understand the proof structure.

**Step 2 — Identify repeated proof patterns.** Look for:

- **Identical or near-identical proof blocks** that appear in multiple
  match arms or branches. Splay has left-left, left-right, right-left,
  right-right cases — each case likely repeats the same BST invariant
  proofs (ordered, contains, size) with different variable names.
- **Shared lemma opportunities.** If the same 10-line `assert ... by { }`
  block appears 4 times with different variables, extract a lemma that
  takes those variables as parameters. One call replaces 10 lines × 4 = 40.
- **Proof steps that re-derive facts already in scope.** If `ensures`
  from a recursive call gives you `ordered(left)` and you re-prove it
  with 5 lines of asserts, delete the redundant proof.
- **Common patterns between St and Mt.** The two splay functions are
  St/Mt variants of the same algorithm. Proof patterns that appear in
  both files can share a lemma in the file or in a shared module.

**Step 3 — Write lemmas.** For each repeated pattern:

1. Extract the proof into a `proof fn lemma_...` with the minimal
   `requires` and `ensures`.
2. Replace every occurrence with a single call.
3. Validate after each extraction: `scripts/validate.sh isolate Chap37`.

**Step 4 — Look for proof steps Z3 can now handle alone.** After
minimization (R162), many asserts are marked `// Veracity: NEEDED assert`.
Some of these may become unnecessary once helper lemmas provide the
intermediate facts. Try commenting out (NOT deleting) assert blocks
that seem redundant after adding a lemma. If validate passes, the assert
was no longer needed.

## What NOT to do

- Do not restructure the algorithm. The exec code stays the same.
- Do not change function signatures, specs, or trait definitions.
- Do not split into separate functions unless the proof decomposition
  demands it (and then only as proof helpers).
- Do not add `admit` or `assume` to make things compile faster.

## Validation

```bash
scripts/validate.sh isolate Chap37
```

After each lemma extraction, validate. If errors appear, the lemma's
ensures are too weak or the call site is wrong. Fix forward — do not revert.

## Report

Write `plans/agent1-round163-report.md` with:

- Table of lemmas created: name, file, requires/ensures summary, lines saved
- Table of functions: proof lines before/after
- Total proof lines removed
- Any patterns you found but couldn't extract (and why)

## RCP

`git add -A && git commit -m "R163 Agent 1: compress BSTSplay proof (−N lines)"`, then `git push`.
