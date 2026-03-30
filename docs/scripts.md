# Scripts Reference

All scripts live in `scripts/` and are executable. Most log output to `logs/`.

## Verification

| Script | Usage | Description |
|--------|-------|-------------|
| `validate.sh` | `validate.sh [mode] [--time] [--profile]` | Verus verification. Modes: `full` (default), `dev_only`, `exp`, `isolate ChapNN`. Logs to `logs/validate.*.log`. |
| `ptt.sh` | `ptt.sh [filter]` | Compile PTT library and run proof time tests. Filter is case-insensitive substring (e.g. `ptt.sh Chap05`). |
| `rtt.sh` | `rtt.sh [filter]` | Runtime tests via cargo nextest. Filter is case-insensitive substring (e.g. `rtt.sh bst`). |
| `profile.sh` | `profile.sh [isolate ChapNN]` | Profile Z3 instantiation counts. Logs to `logs/profile-*.log`. |
| `profile-table.sh` | `profile-table.sh` | Generate summary tables from profile SUMMARY files. |

### Isolated validation

`validate.sh isolate ChapNN` verifies only ChapNN and its transitive dependencies.
Uses 2-3 GB instead of 7+ GB. Use during development; run full validation before pushing.

### Environment variables

- `VERUS_EXTRA_ARGS` — extra flags passed to the Verus invocation (e.g. `VERUS_EXTRA_ARGS="-V new-mut-ref"`).

## Analysis

| Script | Usage | Description |
|--------|-------|-------------|
| `holes.sh` | `holes.sh [dir-or-file]` | Run veracity proof hole analysis. Defaults to `src/`. |
| `all-holes-by-chap.sh` | `all-holes-by-chap.sh` | Proof holes for every chapter. Per-chapter logs to `src/ChapNN/analyses/`. |
| `all-style-by-chap.sh` | `all-style-by-chap.sh` | Verus style review for every chapter. Logs to `src/ChapNN/analyses/`. |
| `all-fn-impls-by-chap.sh` | `all-fn-impls-by-chap.sh` | Function/impl inventory per chapter. Outputs `.md` and `.json`. |
| `chapter-cleanliness-status.sh` | `chapter-cleanliness-status.sh` | Clean/holed chapter summary with dependency blockers. Run `all-holes-by-chap.sh` first. |
| `hole-priorities.sh` | `hole-priorities.sh` | Prioritized proof hole summary across all chapters. |
| `blockers.sh` | `blockers.sh` | Per-file blocking report with dependency graph and proof status. |
| `check-stale-reviews.sh` | `check-stale-reviews.sh` | Check if analysis files are older than source. |
| `proof-velocity.sh` | `proof-velocity.sh [days]` | Proof state and weekly velocity (default: 7 days). |

## Agent Workflow

| Script | Usage | Description |
|--------|-------|-------------|
| `merge-agent.sh` | `merge-agent.sh <branch>` | Merge an agent branch into main. |
| `rebase-agents.sh` | `rebase-agents.sh` | Rebase all agent worktrees onto `origin/main` and force-push. Run after main is pushed. |
| `reset-agent-to-main.sh` | `reset-agent-to-main.sh` | Reset current agent branch to `origin/main` and force-push. Run in agent worktree. |
| `survey-agents.sh` | `survey-agents.sh` | Table of all agent worktrees: commit, uncommitted changes, unpushed status. |
| `show-agent-reports.sh` | `show-agent-reports.sh <round> [lines]` | Display agent reports for a given round. |
| `watch-agents.sh` | `watch-agents.sh` | Poll agent worktrees for new commits every 30 minutes. |
| `resume-claude.sh` | `resume-claude.sh` | Resume a suspended (Ctrl+Z) Claude Code process. |

## Merge Conflict Resolution

| Script | Usage | Description |
|--------|-------|-------------|
| `resolve-analysis-merge.sh` | `resolve-analysis-merge.sh [dir]` | Resolve analysis-only merge conflicts with `--theirs`. |
| `resolve-analysis-rebase.sh` | `resolve-analysis-rebase.sh [dir]` | Resolve analysis-only rebase conflicts with `--ours`. Loops through rebase steps. |
| `resolve-settings-merge.sh` | `resolve-settings-merge.sh [dir]` | Union `.claude/settings.local.json` allow lists from both conflict sides. |

## Utility

| Script | Usage | Description |
|--------|-------|-------------|
| `verusdoc.sh` | `verusdoc.sh` | Generate Verus documentation with specifications. |
| `tags.sh` | `tags.sh` | Generate tags file for the project. |
| `clear-lsp-cache.sh` | `clear-lsp-cache.sh` | Clear LSP and Verus analyzer caches. |
| `consolidate-rules.sh` | `consolidate-rules.sh` | Consolidate `.cursor/rules` into `.github/copilot-instructions.md`. |

## Test Fixture

| Script | Usage | Description |
|--------|-------|-------------|
| `validate-path.sh` | `validate-path.sh [mode] [--time]` | Verify the `path/` parallel build. Run from fixture root. |
| `regenerate-path.sh` | `regenerate-path.sh` | Regenerate `path/` from `src`. Run from fixture root. |
