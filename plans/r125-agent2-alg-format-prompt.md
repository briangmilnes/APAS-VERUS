# R125 Agent 2 — Standardize alg analysis annotations (Chap26-39). AFK. DOT.

## Task

Same as Agent 1 but for: Chap26, Chap27, Chap28, Chap30, Chap35, Chap36, Chap37, Chap38, Chap39

See `plans/r125-agent1-alg-format-prompt.md` for the full format specification.

In short:
- `/// - APAS: Work Θ(x), Span Θ(y)` → `/// - Alg Analysis: APAS (ChNN ref): Work O(x), Span O(y)`
- `/// - APAS: Algorithm NN.NN — desc` → `/// - Alg Analysis: APAS (ChNN Alg NN.NN): desc`
- Add `Code review (Claude Opus 4.6):` line where missing.
- Read `prompts/ChapNN.txt` for textbook references.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT modify code — annotations only.

## No step limit — finish all chapters.

## When done

Commit all changes with `git add -A && git commit` and push.

## Report

Write `plans/agent2-r125-alg-format-report.md`.
