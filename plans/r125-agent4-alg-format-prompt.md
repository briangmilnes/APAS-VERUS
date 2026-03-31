# R125 Agent 4 — Standardize alg analysis annotations (Chap43 + Chap52-66). AFK. DOT.

## Task

Same as Agent 1 but for: Chap43, Chap52, Chap53, Chap54, Chap55, Chap56, Chap57, Chap58, Chap59, Chap61, Chap62, Chap63, Chap64, Chap65, Chap66

Note: Chap43 has two prose files (Chap43part2.txt). Read both.
Note: Chap54 has two prose files (Chap54part2.txt). Read both.

See `plans/r125-agent1-alg-format-prompt.md` for the full format specification.

In short:
- `/// - APAS: Work Θ(x), Span Θ(y)` → `/// - Alg Analysis: APAS (ChNN ref): Work O(x), Span O(y)`
- `/// - APAS: Algorithm NN.NN — desc` → `/// - Alg Analysis: APAS (ChNN Alg NN.NN): desc`
- Add `Code review (Claude Opus 4.6):` line where missing.
- Read `prompts/ChapNN.txt` for textbook references.
- Do NOT run `veracity-annotate-alg-analysis-from-toml`.
- Do NOT modify code — annotations only.

## No step limit — finish all chapters.

## Report

Write `plans/agent4-r125-alg-format-report.md`.
