# r207 — Verus 0.2026.09.13 migration, round 1: name resolution and three reviews

Date: 2026-09-20. Scoping: `plans/r206-verus-upgrade-scoping.md`.

Four agents. Agents 1, 2, and 4 run concurrently. Agent 3 starts when agent 1
reports that the crate passes name resolution, because agent 3 edits
`src/standards/` and `rust_verify_test/`, which agent 1 also touches, and
agent 3 must validate its edits.

| # | Agent | Plan | Writes | Runs Verus |
|---|-------|------|--------|------------|
| 1 | name resolution | `plans/r207-agent1-name-resolution.md` | `src/**` renames; `docs/NameResolutionUpgrade.md` | yes, the only one |
| 2 | vstdplus review | `plans/r207-agent2-vstdplus-review.md` | `docs/VstdplusReview.md` only | no |
| 3 | standards upgrade | `plans/r207-agent3-standards-upgrade.md` | `src/standards/`, `rust_verify_test/tests/standards/`, `docs/StandardsUpgrade.md` | yes, after agent 1 |
| 4 | accept review Chap01–07 | `plans/r207-agent4-accept-review-chap01-07.md` | `docs/AcceptReviewChap01to07.md` only | no |

All four work in the main worktree `~/projects/APAS-VERUS` on branch `main`.
No agent commits. The orchestrator reviews the four docs, then decides whether
the volume of edits justifies a CST transformer built on `~/projects/CSTs`
before round 2.

## Rules common to every agent

1. Read `CLAUDE.md` in full, including the three imported ComputAItionalThinking
   files, and every file in `src/standards/` before editing or judging any code.
2. Write in the ComputAItionalThinking vocabulary: no metaphors, status as a
   measurement, "verified" only for a prover result.
3. Algorithms and their cost specifications must not change. Exec bodies are
   edited only where a plan names the exact substitution. `/// - Alg Analysis`
   lines are never edited.
4. Never add `assume`, `accept`, `// accept hole`, `admit`, `external_body`,
   `requires true`, or `// veracity: no_requires`. Never convert `assume` to
   `accept`. Never weaken an `ensures`.
5. No string rewriting of source with `sed`, `perl`, regex scripts, or Python.
   Edits are made with the Edit tool on exact identifier tokens, after reading
   the surrounding lines. `grep` is allowed for discovery.
6. Only agent 1, and later agent 3, run `scripts/validate.sh`. Agents 2 and 4
   never run `validate.sh`, `rtt.sh`, `ptt.sh`, or `profile.sh`. Read the
   newest log under `logs/` instead of rerunning.
7. Do not spawn subagents. Do not commit. Do not push. Do not run `git clean`,
   `git checkout --`, `git restore`, or `git reset`.
8. Every table in a doc carries a `#` column first; tables that name files carry
   a `Chap` column second; cells are at most 40 characters.
9. The deliverable is the named `docs/*.md` file plus, for agents 1 and 3, the
   source edits. Finish the file even if a step is blocked; state what is blocked
   and why.
