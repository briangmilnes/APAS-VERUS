# R125 Agent 3 — Standardize alg analysis annotations (Chap40-51 except Chap43). AFK. DOT.

## Task

Same as Agent 1 but for: Chap40, Chap41, Chap42, Chap44, Chap45, Chap47, Chap49, Chap50, Chap51

Note: Chap43 is handled by Agent 4. Do NOT touch Chap43.
Note: Chap47 has two prose files (Chap47part2.txt). Read both.

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

Write `plans/agent3-r125-alg-format-report.md`.
