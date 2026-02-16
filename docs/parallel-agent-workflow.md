<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Parallel Agent Workflow

Two AI agents work simultaneously on APAS-VERUS using git worktrees, each in its own Cursor window.

## Layout

| Agent | Role | Branch | Directory | Writes To |
|-------|------|--------|-----------|-----------|
| Implementer | Writes code, runs Verus, fixes proofs | `main` | `~/projects/APAS-VERUS/` | `src/`, `tests/`, `rust_verify_test/`, `lib.rs` |
| Reviewer | Reviews code against textbook prose, runs veracity tools, produces analysis | `review/prose` | `~/projects/APAS-VERUS-review/` | `analyses/`, `docs/` |

Both agents share the same `.git` database. Commits on either branch are immediately visible to the other via `git merge`.

## Setup (one-time)

From `~/projects/APAS-VERUS` on `main`:

```bash
git worktree add ../APAS-VERUS-review review/prose
```

Then open `~/projects/APAS-VERUS-review` in a second Cursor window.

## Syncing Changes

### Reviewer picks up implementer's new code

```bash
cd ~/projects/APAS-VERUS-review
git merge main
```

### Implementer picks up reviewer's analysis

```bash
cd ~/projects/APAS-VERUS
git merge review/prose
```

Merges should be clean because the agents write to non-overlapping file sets. If a conflict does occur on a shared file (e.g., something in `analyses/`), resolve by taking the later version:

```bash
# From main, prefer reviewer's version of conflicting files
git merge review/prose -X theirs

# From review/prose, prefer implementer's version of conflicting files
git merge main -X theirs
```

## Rules for Each Agent

### Implementer (on `main`)

- Stay on `main`. Never checkout or modify `review/prose`.
- Commit frequently so the reviewer can merge your changes.
- Write code in `src/`, tests in `tests/` and `rust_verify_test/`.
- Running veracity tools (e.g., `veracity-review-proof-holes`) is fine — their logs in `analyses/` and `src/ChapNN/analyses/` are ephemeral and get overwritten each run.
- Do not write curated analysis or documentation to `analyses/` or `docs/` — that's the reviewer's domain.

### Reviewer (on `review/prose`)

- Stay on `review/prose`. Never checkout or modify `main`.
- Commit frequently so the implementer can merge your analyses.
- Write curated analysis to `analyses/` and documentation to `docs/`.
- Running veracity tools is fine — if both agents run the same tool, the later output wins on merge.
- You may read `src/`, `tests/`, and `prompts/` but do not write to them.
- Run `git merge main` before starting a review session to pick up the latest code.

### Shared: Veracity Tool Logs

Both agents may run veracity tools (`veracity-review-proof-holes`, `veracity-review-module-fn-impls`, etc.). These tools write ephemeral logs to `analyses/` and `src/ChapNN/analyses/`. The logs are disposable and regenerated in seconds, so on merge conflicts the later version wins. Use `git merge -X theirs` to resolve automatically.

## Pushing to Origin

Push is for backup, not for coordination. Push whenever you want a remote checkpoint:

```bash
# Implementer
cd ~/projects/APAS-VERUS
git push origin main

# Reviewer
cd ~/projects/APAS-VERUS-review
git push origin review/prose
```

## Teardown

When parallel work is done and everything is merged back to `main`:

```bash
cd ~/projects/APAS-VERUS
git merge review/prose          # final merge of any remaining review work
git worktree remove ../APAS-VERUS-review
git branch -d review/prose      # optional: delete the branch
```
