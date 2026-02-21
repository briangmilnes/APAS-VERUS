<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Scripts

All scripts live in `scripts/`, auto-detect the worktree root via `BASH_SOURCE`, and work from any of the four worktrees (main, agent1, agent2, agent3). Output is ANSI-stripped for Emacs `M-x compile`.

## Summary

| # | Script | Usage | Purpose |
|---|--------|-------|---------|
| 1 | `validate.sh` | `validate.sh [full\|dev\|exp] [--time]` | Verus verification |
| 2 | `check.sh` | `check.sh` | `cargo check --lib` |
| 3 | `rtt.sh` | `rtt.sh [filter]` | Runtime tests |
| 4 | `ptt.sh` | `ptt.sh [filter]` | Compile PTT lib + proof time tests |
| 5 | `holes.sh` | `holes.sh [dir-or-file]` | Proof hole detection |
| 6 | `validate-check-rtt-ptt.sh` | `validate-check-rtt-ptt.sh` | Full pipeline |
| 7 | `merge-agent.sh` | `merge-agent.sh <branch>` | Merge agent branch into main |
| 8 | `reset-agent-to-main.sh` | `reset-agent-to-main.sh` | Reset agent to `origin/main` |
| 9 | `tags.sh` | `tags.sh` | Generate Emacs TAGS |
| 10 | `verusdoc.sh` | `verusdoc.sh` | Generate Verus API docs |
| 11 | `clear-lsp-cache.sh` | `clear-lsp-cache.sh` | Clear LSP and Verus analyzer caches |

---

## 1. validate.sh

Runs Verus verification on the crate.

**Usage:** `scripts/validate.sh [full|dev|exp] [--time]`

| Argument | Description |
|----------|-------------|
| `full` (default) | All modules including `#[cfg(feature = "full_verify")]` |
| `dev` | Skip cfg-gated modules (faster iteration) |
| `exp` | Experiments only (`#[cfg(feature = "experiments_only")]`) |
| `--time` | Pass `--time` to Verus for timing breakdown |

**Examples:**

```
scripts/validate.sh dev
scripts/validate.sh full --time
scripts/validate.sh exp
```

---

## 2. check.sh

Runs `cargo check --lib` to catch Rust compilation errors without running Verus.

**Usage:** `scripts/check.sh`

No arguments.

**Example:**

```
scripts/check.sh
```

---

## 3. rtt.sh

Runs runtime tests (RTTs) via `cargo nextest run` with `-j 6` parallelism and a 120-second timeout.

**Usage:** `scripts/rtt.sh [filter]`

| Argument | Description |
|----------|-------------|
| `filter` (optional) | Case-insensitive substring match on test function names |

**Examples:**

```
scripts/rtt.sh           # all tests
scripts/rtt.sh bst       # BST tests only
scripts/rtt.sh quick_sort
```

---

## 4. ptt.sh

Compiles the PTT library (`.rlib` and `.vir`) with `verus --compile`, then runs proof time tests from `rust_verify_test/` via `cargo nextest run -j 6`.

**Usage:** `scripts/ptt.sh [filter]`

| Argument | Description |
|----------|-------------|
| `filter` (optional) | Case-insensitive substring match on test function names |

The compile step always runs all modules. The filter applies only to which PTTs execute.

**Examples:**

```
scripts/ptt.sh           # compile + all PTTs
scripts/ptt.sh Chap05    # compile + Chap05 PTTs only
```

---

## 5. holes.sh

Runs `veracity-review-proof-holes` to detect proof holes (`assume`, `admit`, `external_body`, etc.). Auto-detects whether the argument is a file or directory.

**Usage:** `scripts/holes.sh [dir-or-file]`

| Argument | Description |
|----------|-------------|
| (none) | Scan all of `src/` |
| directory | Scan that directory (e.g. `src/Chap05/`) |
| file | Scan that single file (e.g. `src/Chap05/SetStEph.rs`) |

**Examples:**

```
scripts/holes.sh
scripts/holes.sh src/Chap05/
scripts/holes.sh src/Chap05/SetStEph.rs
```

---

## 6. validate-check-rtt-ptt.sh

Runs the full validation pipeline in order. Stops on first failure (`set -euo pipefail`).

**Usage:** `scripts/validate-check-rtt-ptt.sh [full|dev|exp] [--time] [filter]`

| Argument | Description |
|----------|-------------|
| `full\|dev\|exp` (optional) | Verus mode, passed to `validate.sh` (defaults to `dev`) |
| `--time` (optional) | Passed to `validate.sh` for timing breakdown |
| `filter` (optional) | Case-insensitive test name filter, passed to `rtt.sh` and `ptt.sh` |

Calls in order:

1. `validate.sh <mode> [--time]`
2. `check.sh`
3. `rtt.sh [filter]`
4. `ptt.sh [filter]`

**Examples:**

```
scripts/validate-check-rtt-ptt.sh                  # dev, all tests
scripts/validate-check-rtt-ptt.sh full --time       # full verify with timing, all tests
scripts/validate-check-rtt-ptt.sh dev bst           # dev verify, BST tests only
scripts/validate-check-rtt-ptt.sh full --time bst   # full with timing, BST tests only
```

---

## 7. merge-agent.sh

Merges an agent branch into the current worktree (should be main). On success, runs the full validation pipeline. On conflict, prints the conflicted files and exits for manual resolution.

**Usage:** `scripts/merge-agent.sh <branch>`

| Argument | Description |
|----------|-------------|
| `branch` (required) | Branch to merge (e.g. `agent1/ready`) |

**Example:**

```
cd ~/projects/APAS-VERUS
scripts/merge-agent.sh agent1/ready
```

See [WorkingWithMultipleAgentsInWorktrees.md](WorkingWithMultipleAgentsInWorktrees.md) for the full merge procedure and conflict resolution strategy.

---

## 8. reset-agent-to-main.sh

Resets the current agent branch to `origin/main` and force-pushes. Run in an agent worktree **after** main has merged and pushed the agent's work. Refuses to run on `main`.

**Usage:** `scripts/reset-agent-to-main.sh`

No arguments. Detects the current branch automatically.

**Example:**

```
cd ~/projects/APAS-VERUS-agent1
scripts/reset-agent-to-main.sh
```

---

## 9. tags.sh

Generates an Emacs `TAGS` file covering `src/`, `vstd`, and `builtin` using `verus-etags`.

**Usage:** `scripts/tags.sh`

No arguments. Requires `ctags` (universal-ctags) and `~/projects/verus-etags/target/release/verus-etags`.

**Example:**

```
scripts/tags.sh
```

---

## 10. verusdoc.sh

Generates API documentation with Verus specifications (requires/ensures) using `rustdoc` and the `verusdoc` post-processor. Builds `verusdoc` and `vstd` (debug) if not already built.

**Usage:** `scripts/verusdoc.sh`

No arguments. Output goes to `target/verusdoc/apas_verus/index.html`.

**Example:**

```
scripts/verusdoc.sh
```

---

## 11. clear-lsp-cache.sh

Clears Verus analyzer and rust-analyzer caches (`.verus-log`, `target/.fingerprint`, `.rust-analyzer`). Useful when the LSP gets into a bad state.

**Usage:** `scripts/clear-lsp-cache.sh`

No arguments.

**Example:**

```
scripts/clear-lsp-cache.sh
```
