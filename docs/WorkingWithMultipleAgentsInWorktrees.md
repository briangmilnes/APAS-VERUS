<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Working With Multiple Agents In Worktrees

Procedure for merging agent branches into `main` and syncing all worktrees.

## Layout

| # | Worktree | Directory | Branch |
|---|----------|-----------|--------|
| 1 | main | `~/projects/APAS-VERUS` | `main` |
| 2 | agent1 | `~/projects/APAS-VERUS-agent1` | `agent1/ready` (or `agent1/<topic>`) |
| 3 | agent2 | `~/projects/APAS-VERUS-agent2` | `agent2/ready` (or `agent2/<topic>`) |
| 4 | agent3 | `~/projects/APAS-VERUS-agent3` | `agent3/ready` (or `agent3/<topic>`) |

## Scripts

All scripts live in `scripts/` and auto-detect the worktree root. They strip ANSI for Emacs.

| # | Script | Usage | Purpose |
|---|--------|-------|---------|
| 1 | `scripts/validate.sh` | `validate.sh [full\|dev\|exp] [--time]` | Verus verification |
| 2 | `scripts/check.sh` | `check.sh` | `cargo check --lib` |
| 3 | `scripts/rtt.sh` | `rtt.sh [filter]` | Runtime tests (`-j 6`, 120s timeout) |
| 4 | `scripts/ptt.sh` | `ptt.sh [filter]` | Compile PTT lib + proof time tests (`-j 6`) |
| 5 | `scripts/holes.sh` | `holes.sh [dir-or-file]` | Proof hole detection |
| 6 | `scripts/validate-check-rtt-ptt.sh` | `validate-check-rtt-ptt.sh` | Full pipeline: validate + check + RTT + PTT |
| 7 | `scripts/merge-agent.sh` | `merge-agent.sh <branch>` | Merge an agent branch + validate |
| 8 | `scripts/reset-agent-to-main.sh` | `reset-agent-to-main.sh` | Reset agent branch to `origin/main` + force push |

## Critical Rules

1. **Merge one agent at a time.** Complete the full verify cycle before starting the next.
2. **Parallelism limit: `-j 6`** on all `cargo nextest` commands. Unbounded parallelism locks the machine.
3. **Never `git checkout main`** in an agent worktree. Git forbids the same branch in two worktrees.
4. **PTTs require a compile step first.** Never run PTTs directly from the workspace root.
5. **Validate before every commit.** No worktree commits without passing validation, RTTs, and PTTs.

## Conflict Resolution Strategy

| # | File(s) | Take from | Reason |
|---|---------|-----------|--------|
| 1 | `src/lib.rs` | **main** (ours) | Main controls module visibility and cfg gates |
| 2 | `analyses/*` | **agent** (theirs) | Agents produce the latest analyses |
| 3 | `src/ChapNN/analyses/*` | **agent** (theirs) | Agents produce the latest chapter reviews |
| 4 | `src/ChapNN/*.rs` source files | **manual merge** | All four worktrees modify source — inspect the diff and decide |

## Phase 1: Main Validates, Commits, and Pushes

### Step 1: Validate main

> **Script:** `cd ~/projects/APAS-VERUS && scripts/validate-check-rtt-ptt.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS

# Verus verification
~/projects/verus/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Cargo check
cargo check --lib

# RTTs — runtime tests
cargo nextest run -j 6 --no-fail-fast

# Compile PTT library
~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir

# PTTs — proof time tests (from rust_verify_test/, NOT workspace root)
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run -j 6 --no-fail-fast
```

### Step 2: Commit and push main

```bash
cd ~/projects/APAS-VERUS
git add -A
git commit -m "Pre-merge: main work"
git push origin main
```

## Phase 2: Agent1 Validates, Commits, and Pushes

### Step 3: Validate agent1

> **Script:** `cd ~/projects/APAS-VERUS-agent1 && scripts/validate-check-rtt-ptt.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS-agent1

# Verus verification
~/projects/verus/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Cargo check
cargo check --lib

# RTTs — runtime tests
cargo nextest run -j 6 --no-fail-fast

# Compile PTT library
~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS-agent1/target/verus/apas_verus.vir

# PTTs — proof time tests (from rust_verify_test/, NOT workspace root)
cd ~/projects/APAS-VERUS-agent1/rust_verify_test && cargo nextest run -j 6 --no-fail-fast
```

### Step 4: Commit and push agent1

```bash
cd ~/projects/APAS-VERUS-agent1
git add -A
git commit -m "Pre-merge: agent1 work"
git push origin agent1/ready
```

## Phase 3: Agent2 Validates, Commits, and Pushes

### Step 5: Validate agent2

> **Script:** `cd ~/projects/APAS-VERUS-agent2 && scripts/validate-check-rtt-ptt.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS-agent2

# Verus verification
~/projects/verus/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Cargo check
cargo check --lib

# RTTs — runtime tests
cargo nextest run -j 6 --no-fail-fast

# Compile PTT library
~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS-agent2/target/verus/apas_verus.vir

# PTTs — proof time tests (from rust_verify_test/, NOT workspace root)
cd ~/projects/APAS-VERUS-agent2/rust_verify_test && cargo nextest run -j 6 --no-fail-fast
```

### Step 6: Commit and push agent2

```bash
cd ~/projects/APAS-VERUS-agent2
git add -A
git commit -m "Pre-merge: agent2 work"
git push origin agent2/ready
```

## Phase 3.5: Agent3 Validates, Commits, and Pushes

### Step 6a: Validate agent3

> **Script:** `cd ~/projects/APAS-VERUS-agent3 && scripts/validate-check-rtt-ptt.sh`

### Step 6b: Commit and push agent3

```bash
cd ~/projects/APAS-VERUS-agent3
git add -A
git commit -m "Pre-merge: agent3 work"
git push origin agent3/ready
```

## Phase 4: Merge Agent1 Into Main

### Step 7: Fetch and merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/merge-agent.sh agent1/ready`
>
> (Auto-validates after merge. If conflicts, stops for manual resolution.)

Full commands:

```bash
cd ~/projects/APAS-VERUS
git fetch origin
git merge agent1/ready --no-edit
```

### Step 8: Resolve conflicts (if any)

```bash
cd ~/projects/APAS-VERUS

# List conflicted files
git diff --name-only --diff-filter=U

# Take ours for lib.rs
git checkout --ours src/lib.rs
git add src/lib.rs

# Take theirs for analyses and reviews
git checkout --theirs analyses/veracity-review-module-fn-impls.json \
  analyses/veracity-review-module-fn-impls.md \
  src/ChapNN/analyses/review-against-prose.md
git add analyses/ src/ChapNN/analyses/

# For conflicted .rs source files: inspect the diff and decide per file
git diff src/ChapNN/SomeFile.rs   # review the conflict markers
# Then: git checkout --ours, --theirs, or hand-edit as needed
git add src/ChapNN/SomeFile.rs

# Commit the merge
git commit --no-edit
```

### Step 9: Validate after agent1 merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/validate-check-rtt-ptt.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS

# Verus verification
~/projects/verus/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Cargo check
cargo check --lib

# RTTs — runtime tests
cargo nextest run -j 6 --no-fail-fast

# Compile PTT library
~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir

# PTTs — proof time tests (from rust_verify_test/, NOT workspace root)
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run -j 6 --no-fail-fast
```

### Step 10: Commit and push main

```bash
cd ~/projects/APAS-VERUS
git add -A
git commit -m "Merge agent1/ready into main"
git push origin main
```

## Phase 5: Merge Agent2 Into Main

### Step 11: Fetch and merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/merge-agent.sh agent2/ready`

Full commands:

```bash
cd ~/projects/APAS-VERUS
git fetch origin
git merge agent2/ready --no-edit
```

### Step 12: Resolve conflicts (if any)

```bash
cd ~/projects/APAS-VERUS

# List conflicted files
git diff --name-only --diff-filter=U

# Take ours for lib.rs
git checkout --ours src/lib.rs
git add src/lib.rs

# Take theirs for analyses and reviews
git checkout --theirs analyses/veracity-review-module-fn-impls.json \
  analyses/veracity-review-module-fn-impls.md \
  src/ChapNN/analyses/review-against-prose.md
git add analyses/ src/ChapNN/analyses/

# For conflicted .rs source files: inspect the diff and decide per file
git diff src/ChapNN/SomeFile.rs   # review the conflict markers
# Then: git checkout --ours, --theirs, or hand-edit as needed
git add src/ChapNN/SomeFile.rs

# Commit the merge
git commit --no-edit
```

### Step 13: Validate after agent2 merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/validate-check-rtt-ptt.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS

# Verus verification
~/projects/verus/source/target-verus/release/verus \
  --crate-type=lib src/lib.rs --multiple-errors 20 --expand-errors

# Cargo check
cargo check --lib

# RTTs — runtime tests
cargo nextest run -j 6 --no-fail-fast

# Compile PTT library
~/projects/verus/source/target-verus/release/verus \
  --compile --crate-type=lib --crate-name apas_verus src/lib.rs \
  -o target/verus/libapas_verus.rlib \
  --export /home/milnes/projects/APAS-VERUS/target/verus/apas_verus.vir

# PTTs — proof time tests (from rust_verify_test/, NOT workspace root)
cd ~/projects/APAS-VERUS/rust_verify_test && cargo nextest run -j 6 --no-fail-fast
```

### Step 14: Commit and push main

```bash
cd ~/projects/APAS-VERUS
git add -A
git commit -m "Merge agent2/ready into main"
git push origin main
```

## Phase 5.5: Merge Agent3 Into Main

### Step 14a: Fetch and merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/merge-agent.sh agent3/ready`

### Step 14b: Resolve conflicts (if any)

Same pattern as Step 8 or 12 — take ours for lib.rs, theirs for analyses, manual for source files.

### Step 14c: Validate after agent3 merge

> **Script:** `cd ~/projects/APAS-VERUS && scripts/validate-check-rtt-ptt.sh`

### Step 14d: Commit and push main

```bash
cd ~/projects/APAS-VERUS
git add -A
git commit -m "Merge agent3/ready into main"
git push origin main
```

## Phase 6: Reset Agent1 to Main

**Do NOT `git checkout main`** — it's already checked out in the primary worktree.
Reset the agent branch to point at main's HEAD. No merge commit needed — agent1's
work is already in main.

### Step 15: Reset agent1 and push

> **Script:** `cd ~/projects/APAS-VERUS-agent1 && scripts/reset-agent-to-main.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS-agent1
git fetch origin
git reset --hard origin/main
git push origin agent1/ready --force
```

## Phase 7: Reset Agent2 to Main

### Step 16: Reset agent2 and push

> **Script:** `cd ~/projects/APAS-VERUS-agent2 && scripts/reset-agent-to-main.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS-agent2
git fetch origin
git reset --hard origin/main
git push origin agent2/ready --force
```

## Phase 7.5: Reset Agent3 to Main

### Step 16a: Reset agent3 and push

> **Script:** `cd ~/projects/APAS-VERUS-agent3 && scripts/reset-agent-to-main.sh`

Full commands:

```bash
cd ~/projects/APAS-VERUS-agent3
git fetch origin
git reset --hard origin/main
git push origin agent3/ready --force
```

## Phase 8: Verify All Worktrees Are In Sync

### Step 17: Check commit hashes

```bash
cd ~/projects/APAS-VERUS
git worktree list
```

All four should show the same commit hash.

## Common Mistakes

| # | Mistake | What happens | Fix |
|---|---------|-------------|-----|
| 1 | `git checkout main` in agent worktree | `fatal: 'main' is already used by worktree` | Use `git reset --hard origin/main` instead |
| 2 | `cargo nextest run` without `-j 6` | Machine locks up — unbounded parallelism | Always pass `-j 6` |
| 3 | Running PTTs from workspace root | Compiles `rust_verify_test_macros` with stable Rust, fails on `#![feature]` | Always `cd rust_verify_test` first |
| 4 | Running PTTs without compile step | Tests use stale `.rlib`/`.vir`, mysterious failures | Always compile PTT library before running PTTs |
| 5 | Merging agents simultaneously | Conflict hell, unclear resolution | One at a time, full verify between each |
| 6 | Forgetting `cargo check` after merge | Spec-only imports (`tree_contains`, `avl_balanced`) invisible to Verus but break cargo | Always run cargo check |
| 7 | Committing without validation | Broken code gets pushed, cascades into merge conflicts | Always validate, RTT, and PTT before every commit |
