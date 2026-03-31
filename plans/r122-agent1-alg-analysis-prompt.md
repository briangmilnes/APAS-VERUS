# R122 Agent 1 — Algorithmic analysis: verify APAS + independent code review (Chap02-23). AFK. DOT.

## Task

For each function in your chapters, do THREE things:

1. **Check the APAS annotation against the textbook.** Read `prompts/ChapNN.txt`
   for the textbook prose. Compare what the `/// - Alg Analysis: APAS (...)` line
   says against what the textbook actually says. Fix the APAS line if wrong.

2. **Write your own independent analysis.** Read the actual code. Determine the
   Work and Span from the implementation. Write a `/// - Alg Analysis: Code review:`
   line with your analysis.

3. **Flag differences.** If your code review matches APAS, write `— matches APAS`.
   If it differs, write `— DIFFERS:` with a brief reason.

## Your chapters

Chap02, Chap03, Chap05, Chap06, Chap11, Chap12, Chap17, Chap18, Chap19, Chap21, Chap23

## Existing placeholders

The veracity tool has already placed two lines per function:
```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
/// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
```

Your job is to **replace `NONE`** with your actual analysis and match/differ flag.
Change `Claude-Opus-4.6 (1M): NONE` → `Code review (Claude Opus 4.6): Work O(...), Span O(...) — matches APAS` (or DIFFERS).

## Output format

Every exec function in a trait should have up to two annotation lines:

```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(lg |a|)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg |a|) — matches APAS
```

Or when they differ:

```rust
/// - Alg Analysis: APAS (Ch26 Thm 26.1): Work O(n), Span O(lg n)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential loop, no parallel split
```

For functions not in the textbook:

```rust
/// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
```

For spec functions, proof functions, and type definitions: no annotations needed.

## How to analyze code

- **Work** = total operations across all threads (sequential cost)
- **Span** = longest path through parallel computation (parallel cost)
- A sequential loop over n items: Work O(n), Span O(n)
- A parallel join over two halves: Work O(n), Span O(lg n) (if balanced)
- A constant-time accessor: Work O(1), Span O(1)
- A tree traversal to depth h: Work O(h), Span O(h)
- If the function delegates to an inner function, trace through to the real work
- Mt (multi-threaded) variants may have different Span than St (sequential) variants

## Process per chapter

1. Read `prompts/ChapNN.txt` — the APAS textbook prose.
2. Run the annotation tool ONCE to ensure baseline annotations exist:
   ```bash
   ~/projects/veracity/target/release/veracity-annotate-alg-analysis-from-toml --codebase ~/projects/APAS-VERUS
   ```
3. Read each `.rs` file in `src/ChapNN/`.
4. For each exec function in a trait:
   a. Check existing APAS annotation against textbook. Fix if wrong.
   b. Read the code, determine Work/Span.
   c. Write `Code review (Claude Opus 4.6):` line with your analysis and match/differ flag.

## Rules

- **DO NOT modify code.** Annotations only (comment lines starting with `///`).
  No changes to fn signatures, bodies, requires, ensures, specs, or proofs.
- Do NOT annotate spec fns, proof fns, or type definitions.
- Do NOT annotate Example*.rs or Problem*.rs files.
- Do NOT remove existing APAS annotations — fix them if wrong.
- Preserve all existing non-analysis comments.
- No subagents.
- Validate after: `scripts/validate.sh` to confirm nothing broke.

## No step limit — finish all chapters.

## Report

Write `plans/agent1-r122-alg-analysis-report.md`. Per chapter: functions
annotated, matches, differs (with brief reason for each differ).
