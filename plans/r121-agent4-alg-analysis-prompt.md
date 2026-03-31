# R121 Agent 4 — Review and rewrite algorithmic analysis comments (Chap52-66). AFK. DOT.

## Task

For each chapter in your range, read the APAS prose from `prompts/ChapNN.txt`,
then read every source file in `src/ChapNN/`, and review/rewrite the algorithmic
analysis comments (`/// - Alg Analysis:` and `/// - APAS:` lines) to accurately
reflect the textbook's cost specifications.

**You are editing comments only. Do NOT modify any code, specs, requires,
ensures, function signatures, or proof bodies.**

## Your chapters

Chap52, Chap53, Chap54 (+ Chap54part2.txt), Chap55, Chap56, Chap57,
Chap58, Chap59, Chap61, Chap62, Chap63, Chap64, Chap65, Chap66

Note: Chap54 has two prose files. Read both parts.

## For each chapter

1. Read `prompts/ChapNN.txt` (and `prompts/ChapNNpart2.txt` if it exists)
   — the APAS textbook prose for that chapter.
2. Read each `.rs` file in `src/ChapNN/`.
3. The veracity tool has already placed `/// - Alg Analysis:` annotations on
   functions it could match. Your job:
   - **Verify** existing annotations are correct (right Work/Span, right
     chapter/theorem/definition reference).
   - **Fix** any wrong annotations (wrong cost, wrong reference, mismatched
     function).
   - **Add** missing annotations where the prose specifies a cost and the
     function exists but has no annotation.
   - **Add** `/// - APAS: N/A — implementation utility, not in prose.` for
     functions that are internal helpers with no textbook counterpart.
   - **Remove** incorrect annotations that reference the wrong function or
     wrong chapter.
4. Also run the annotation tool to ensure your chapters are up to date:
   ```bash
   ~/projects/veracity/target/release/veracity-annotate-alg-analysis-from-toml --codebase ~/projects/APAS-VERUS
   ```
   Run this ONCE at the start to ensure annotations are current, then do
   your manual review on top.

## Annotation format

```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
/// - Alg Analysis: APAS (Ch26 Thm 26.1): Work O(n), Span O(lg n)
/// - APAS: N/A — implementation utility, not in prose.
/// - APAS: no cost spec (semantics-only chapter).
```

- Reference format: `ChNN <type> NN.NN` where type is CS (Cost Spec),
  Alg (Algorithm), Thm (Theorem), Def (Definition), Ex (Exercise).
- Work/Span use standard asymptotic notation.
- Multiple annotations are fine if a function appears in multiple cost specs.

## Rules

- **DO NOT modify code.** Comments only. No changes to fn signatures, bodies,
  requires, ensures, specs, proofs, or any Rust code.
- Do NOT add annotations to Example*.rs or Problem*.rs files.
- Preserve existing non-analysis comments.
- No subagents.
- Validate after: `scripts/validate.sh isolate Chap52` to confirm you didn't
  accidentally break anything.

## STEP 40

## Report

Write `plans/agent4-r121-alg-analysis-report.md`. Per chapter: number of
annotations added/fixed/verified, any prose operations with no matching
source function.
