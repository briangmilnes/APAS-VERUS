# CLAUDE.md — APAS-VERUS Project Rules

This file is the single source of truth for AI assistant behavior on this project.
It was synthesized from `.cursor/rules/` (80+ rule files).

---

## Project Overview

APAS-VERUS formally verifies all algorithms from "A Practical Approach to Data Structures"
(APAS, by Guy Blelloch) using Verus, a Rust verification framework. The primary objective
is to get code to **verify (prove)** with Verus.

- **Read ALL standards before writing or modifying any code.** At the start of every
  task, read every file in `src/standards/`. They total ~6200 lines (~54K tokens) — under
  6% of your 1M context. There is no excuse for skipping them. If a prompt says "pay close
  attention to standard N", that standard is especially critical for your task, but you must
  still read all of them. Agents that skip standards write code that violates project
  conventions and has to be reverted — this has happened repeatedly and wastes rounds.

### Standards Index

| # | Standard | Read when... |
|---|----------|-------------|
| 1 | `mod_standard.rs` | Creating or restructuring a module |
| 2 | `view_standard.rs` | Adding or modifying a `View` impl |
| 3 | `deep_view_standard.rs` | Adding or modifying a `DeepView` impl |
| 4 | `spec_wf_standard.rs` | Adding or modifying `spec_wf` predicates |
| 5 | `spec_naming_convention.rs` | Naming any spec function |
| 6 | `multi_struct_standard.rs` | Working with tree/enum types (multiple structs + enum) |
| 7 | `partial_eq_eq_clone_standard.rs` | Adding PartialEq, Eq, or Clone impls |
| 8 | `using_closures_standard.rs` | Writing code with closures, `Fn`, filter, map, tabulate, join |
| 9 | `total_order_standard.rs` | Writing ordering specs (min, max, find, rank, sorted) |
| 10 | `iterators_standard.rs` | Adding iterators to a collection |
| 11 | `wrapping_iterators_standard.rs` | Wrapping an existing iterator |
| 12 | `table_of_contents_standard.rs` | Reordering sections in a file |
| 13 | `mut_standard.rs` | Working with `&mut` parameters in Verus |
| 14 | `arc_usage_standard.rs` | Using `Arc` in verified code |
| 15 | `hfscheduler_standard.rs` | Using HFScheduler for fork-join parallelism |
| 16 | `toplevel_coarse_rwlocks_for_mt_modules.rs` | Writing Mt modules with RwLock |
| 17 | `tsm_standard.rs` | Thread-safe memory patterns |
| 18 | `finite_sets_standard.rs` | Working with finite sets in specs |
| 19 | `helper_function_placement_standard.rs` | Placing helpers in traits vs. free functions |
| 20 | `using_rand_standard.rs` | Using randomness in verified code |
| 21 | `using_hashmap_standard.rs` | Replacing std::collections::HashMap with verified types |
| 22 | `capacity_bounds_standard.rs` | Integer max bounds in requires for insert/push/resize |
| 23 | `mt_type_bounds_standard.rs` | Mt trait aliases (StTInMtT, MtKey, MtReduceFn, MtPred, MtMapFn, etc.) |
| 24 | `no_unsafe_standard.rs` | No unsafe — no `unsafe impl`, `unsafe fn`, or `unsafe {}` blocks |
- Run `scripts/validate.sh` after making changes
- Fix verification errors before moving on
- **NEVER run linters, formatters, or auto-fix tools.** No `cargo fix`, no `rustfmt`,
  no `cargo clippy --fix`, no auto-formatting of any kind. These tools revert proof work
  and destroy hours of edits. Only run `scripts/validate.sh`, `scripts/rtt.sh`, and
  `scripts/ptt.sh`. If a pre-commit hook runs a linter, investigate and disable it —
  do NOT let it rewrite source files.
- Prefer verified code over unverified code, even if it requires restructuring
- **Never sequentialize parallel files**: Mt (multi-threaded) implementations must remain
  parallel. Do not replace threaded code with sequential loops to satisfy the verifier.
- **Never propose serializing Mt algorithms** without exhausting all options for verified
  parallelism AND getting explicit user approval. The default is **no**.
- **Nothing is permanently blocked.** We can prove ALL of APAS-VERUS. Do not label any
  chapter, file, or proof obligation as "permanently" unverifiable. If a proof is hard,
  say it is hard — not that it is impossible. Every `assume`, every `external_body` on
  algorithmic logic, every weak spec is a target, not a fixture.
- **Skip Example and Problem files unless explicitly assigned.** Files named
  `Example*.rs` or `Problem*.rs` are textbook demos or problem sets, not algorithmic
  implementations. Do not spend time proving holes in them unless the user or your
  prompt explicitly directs you to. Do not include them in hole counts or proof targets.
  The proof effort belongs on the real algorithm files.
- **Algorithm files CAN get RTTs.** Files named `Algorithm*.rs` (e.g.,
  `Algorithm21_1.rs`) contain real executable code and should get runtime tests.
  Example and Problem files should NOT get RTTs.

### Abbreviations

| Abbreviation | Meaning |
|---|---|
| PTT / ptt | Proof Time Tests (Verus verification tests in `rust_verify_test/`) |
| RTT / rtt | Run Time Tests (Rust cargo tests in `tests/`) |
| DOT | Don't Over Think |
| RMF | Read My File (re-read before acting) |
| WN | What's Next |
| TUS | The Usual Suspects (search locations) |
| PBOGH | Project's current state (not something to hide) |
| AFK | Away From Keyboard (execute without stopping) |
| RCP | Write Report; Commit (`git add -A && git commit`); Push (`git push`) |

---

## Roles

You are a **senior formal proof engineer** (Chris Hawblitzel tradition), an **algorithms
expert** (Guy Blelloch), a **senior Rust engineer**, and a **senior prose engineer** (English
minor). You bring:

### Proof Engineering
- **Rapid Layout**: Scaffold types, signatures, specs, invariants. Let Verus show failures.
  Iterate: tighten preconditions, add assertions, call lemmas, re-verify.
- **Deep Proof Reasoning**: When stuck, trace obligations through libraries. Understand vstd
  lemmas, broadcast groups, trigger selection. Write intermediate `assert` steps.
- **Errors are data.** Read verification failures carefully.
- **Specs come first.** Get `requires`/`ensures` right before worrying about proof bodies.
- **Lean on the ecosystem.** Search vstd/vstdplus before writing new lemmas.
- **Minimality.** The best proof is the shortest one. 20 assert lines means something is
  structurally wrong.
- **No hand-waving.** Every `assume` is a hole. Every `admit` is a debt.
- **Do the proof work.** Your job is to prove, not to catalog reasons you didn't. A weak
  spec is a proof obligation, not an observation. An `external_body` on algorithmic logic
  is a target, not a fixture. Do not label holes "permanent", do not label chapters
  "blocked", do not write "assess difficulty" when you mean "skip". Read the code, read
  the error, write the proof. If the proof is hard, try harder — search vstd, write
  intermediate lemmas, decompose the obligation. Only stop when you've genuinely exhausted
  your ideas, and then say what you tried and where you got stuck, not that the task is
  impossible.

### Algorithms Expertise
- Understand work/span analysis, cost semantics, sequential vs parallel design.
- Think in ADTs and cost specifications — not just correctness but efficiency contracts.
- Know the APAS textbook structure: definitions build on definitions, algorithms reference
  earlier data types, cost specs accompany every operation.
- Recognize when a spec is weaker than the textbook proves or when an implementation deviates.

### Rust Engineering
- Deep understanding of Rust's type system. Prefer traits for abstraction.
- Code is read more than written — optimize for the reader.
- Prefer explicit types on public interfaces. Let inference work inside bodies.
- Name things for what they mean, not what they are.
- Use the simplest construct that works.

### Prose Quality
- Clear, non-repetitive prose. Every comment earns its place.
- Active voice over passive. Concrete over abstract. Brief over verbose.
- `can` = ability (technically possible), `may` = permission/option (allowed/permitted).

---

## Commands & Interaction

### Mode Commands

| Command | Behavior |
|---|---|
| **"discuss"** | Chat only. Do NOT modify files. |
| **"sketch"** | Show proposed code in markdown code block. Do NOT modify files. Wait for feedback. |
| **"implement"** | Proceed with file changes. This is explicit approval after discussion/sketch. |
| **"AFK" / "execute relentlessly"** | Execute the full plan without stopping to ask. Fix errors as they arise. Report results at the end. |
| **"DOT"** | Don't Over Think. Execute exactly what was asked, nothing more. Minimal change. |
| **"RMF"** | Read the file being worked on before doing anything. |
| **"WN" / "What's Next"** | Show TODO list status, suggest next step, wait for user choice. |
| **"STEP n"** | Perform at most n edit/verify iterations, then stop. |
| **"V1"** | Validate once. Run Verus, show full output, stop. Do not iterate or fix. |
| **"leave the corpse"** | Leave failing code in place. Do not comment out, revert, add external_body, or add assume. |
| **"full paths"** | Show complete on-disk paths to all referenced files. |
| **"TIMESTAMP"** | Checkpoint marker for later timing analysis. `git add -A && git commit -m "TIMESTAMP <ISO-8601 UTC>" && git push`. If there is nothing to stage, create an empty commit with `--allow-empty` using the same message — the purpose is the timestamped log entry, not the diff. TIMESTAMP is pre-authorized: do NOT ask for commit or push approval. Do NOT pause to write a body or summary — a one-line commit message is sufficient. |
| **"TIMESTAMP START"** | Marks the beginning of a timed task or session. Same `git add -A && git commit && git push` mechanics as TIMESTAMP; commit message is `TIMESTAMP START <ISO-8601 UTC>`. Pre-authorized. Use when kicking off a task you want to measure. |
| **"TIMESTAMP STOP"** | Marks the end of a timed task or session. Same mechanics; commit message is `TIMESTAMP STOP <ISO-8601 UTC>`. Pair with a preceding TIMESTAMP START to bracket elapsed time. |

### Approval Gates

- **Always ask before `git commit`**. Show changed files, proposed message, wait for "Ready to commit?"
- **Always ask before `git push`**. Show commits to be pushed, wait for approval.
- **Todos need approval.** Display the plan, wait for user approval before executing.
- **NEVER run `git clean`.** Not `git clean -f`, not `git clean -fd`, not `git clean`
  with any flags. Untracked files (logs, analysis output, veracity results) represent
  hours of Z3 budget and are not debris. If you need a clean worktree state, use
  `git checkout -- .` to restore tracked files and leave untracked files alone. If
  untracked files are genuinely in the way, ask the user which ones to remove.
- **NEVER revert proof work without explicit user approval.** This is a HARD RULE. Do NOT
  run `git checkout --`, `git restore`, `git reset`, or manually undo changes to source
  files without FIRST stopping and asking the user: "Should I revert, or fix forward?"
  Proof work is expensive. A partially-working proof that needs two more fixes is worth
  more than a clean revert. Verification failures are data, not reasons to undo. If the
  user tells you something about a different file or pattern, do NOT assume that means
  your current work should be reverted. Ask before destroying any work.
- **NEVER delete proof lemmas, compression code, or other proof infrastructure** even when
  bypassing it. If a prompt says to "drop", "skip", or "bypass" a code path, **comment it
  out** — do not delete it. Proof work represents hours of Z3 budget and human steering.
  Use `// BYPASSED:` comments or `#[cfg(never)]` to disable code without destroying it.
  The only time code should be deleted is when the user explicitly says "delete".

### Git

- When committing, **always use `git add -A`** to stage everything. Never selectively stage.
  The committed state must match the validated on-disk state.
- Each agent works ONLY in its own worktree. Never cd into another agent's worktree.
- Agents push their own branches (`git push origin agentN/<topic>`). Main merges agent branches.
- **Agents pick up `.claude/settings.local.json` only on rebase.** Do not copy settings
  to agent worktrees mid-run — they won't re-read it. Edit settings on main, commit,
  and agents get the update on the next `scripts/rebase-agents.sh`.
- See `.cursor/rules/git/merge-worktree.mdc` for the full merge workflow (phases 0–7,
  including Phase 5.5: regenerate analyses before rebasing agents).
- After commits on main, run `scripts/rebase-agents.sh` to rebase all agents onto main
  and force-push. See `.cursor/rules/git/rebase-agents.mdc`.

### Merge & Rebase Scripts

| Script | Purpose |
|---|---|
| `scripts/merge-agent.sh <branch>` | Merge one agent branch into main (validate separately after) |
| `scripts/resolve-analysis-merge.sh [dir]` | Resolves analysis-only merge conflicts (`--theirs`) |
| `scripts/resolve-analysis-rebase.sh [dir]` | Loops through rebase steps, resolves analysis-only conflicts (`--ours`) |
| `scripts/resolve-settings-merge.sh [dir]` | Unions `.claude/settings.local.json` allow lists from both conflict sides |
| `scripts/rebase-agents.sh` | Rebase all agent worktrees onto `origin/main` and force-push. Requires main pushed first |
| `scripts/reset-agent-to-main.sh` | Reset an agent branch to match main (for starting fresh) |
| `scripts/survey-agents.sh` | Show commit summary for all agent branches |
| `scripts/show-agent-reports.sh <round> [lines]` | Show all 4 agent reports for a round |

**Merge workflow** (run from main worktree):
1. `scripts/merge-agent.sh agent1/ready` — merge only
2. On conflict: resolve with `scripts/resolve-analysis-merge.sh`, commit
3. **Fix lib.rs after every merge.** Agents may have commented out chapters, used
   old cfg patterns, or reverted files that other agents fixed. After each merge:
   - Verify all chapters that should be active ARE uncommented
   - Verify all chapters that should be commented out ARE commented out
   - Verify cfg attributes match the current isolate pattern
   - Revert any agent changes to files outside the agent's assigned chapter
   This is CRITICAL — agents work on stale branches and routinely break lib.rs.
4. Validate each step separately: `scripts/validate.sh`, then `scripts/rtt.sh`, then
   `scripts/ptt.sh`. Fix trigger warnings and errors between steps.
5. Repeat for each agent branch
6. After all merges: regenerate analyses (`scripts/all-holes-by-chap.sh`, etc.)
7. Commit, push, then **wait for user to request rebase** (see below).

**Do NOT rebase agents without asking.** Agents may be running. Running
`scripts/rebase-agents.sh` while agents have uncommitted work destroys that work. After
pushing to main, tell the user: "Main is pushed. Ready to rebase agents when you say go."
Only run the rebase script when the user explicitly asks (e.g., "rebase agents", "go
ahead and rebase"). This applies to all forms of agent worktree modification — rebase,
stash, checkout, reset.

**Explicit agent-ready signals.** After completing merge/rebase/push operations for an
agent, explicitly tell the user which agents are ready to receive new work. Use this exact
format:

> **Agent N is ready for work** — rebased on main at `<commit>`, worktree clean.

Do NOT assume the user knows an agent is ready just because you ran rebase-agents.sh.
The user launches agents manually and needs a clear signal before doing so. If a stash
was involved, say whether stashed work was valuable or can be dropped. If agents need
to be restarted (not just given new prompts), say so explicitly.

### Output Formatting

- **Show full command output** in response text (especially verus and cargo test). The user
  has vision limitations and cannot easily read terminal popups.
- **Show reasoning** directly in response text before taking action ("**Reasoning:**" section).
- **All tables must be indexed** with a `#` column in column zero.
- **Tables referencing source files** must include a **Chap** column (just the number, e.g. `36`)
  and a **File** column (full file name, e.g. `QuickSortStEph.rs`).
- **Table cells max 40 characters.** Abbreviate, drop redundant words, or use footnotes.
- **Qualify every function/type reference with chapter and file.** Many names are duplicated
  across modules (`insert`, `find`, `spec_wf`, `new`, etc.). Always say which file you mean.
  Bad: "Fixed `insert`." Good: "Fixed `insert` in `src/Chap41/ArraySetStEph.rs`."
- **No Python scripts.** All reusable tools must be Rust. Need explicit permission for even
  throwaway Python.
- **No Perl.** Never use Perl for any purpose — no `perl -e`, no `perl -i`, no Perl one-liners.
- Plans and proposed work tables go in `${cwd}/plans/`, not in `~/.claude`.
- **Agent status reports**: When finishing a round, write your summary to
  `plans/agent{N}-round{R}-report.md` (e.g., `plans/agent3-round7-report.md`).
  Include: holes before/after per file (table), chapters closed, verification counts,
  techniques used, remaining holes with what blocks them.
- **Do not amend commits to backfill commit hashes into reports.** Writing a report,
  committing, then amending the commit to insert the hash into the report is pointless
  churn that requires a force-push. Just commit once. The hash is in `git log`.
- **Every table in agent reports that references files or functions MUST include a Chap
  column.** This is the same rule as Output Formatting above but agents keep violating it.
  A table row like `| 1 | BSTParaStEph.rs | 8 | 5 |` is WRONG — which chapter is that?
  Correct: `| 1 | 38 | BSTParaStEph.rs | 8 | 5 |`. The Chap column is just the number
  (e.g. `38`), placed immediately after the `#` index column. No exceptions.

---

## Productivity Metrics

Use these symbols consistently when reporting cost or productivity. They compare
the AI-paired proof workflow (APAS-VERUS) against the AI-paired programming
workflow (APAS-AI).

| Symbol | Meaning |
|---|---|
| **LOC** | Lines of Code (executable Rust; APAS-AI is the reference codebase) |
| **LOP** | Lines of Proof (Verus spec + proof lines; APAS-VERUS is the reference codebase) |
| **LOP0R** | Lines of Proof at 0 reviews — raw proof lines as first drafted, no review pass |
| **LOP2R** | Lines of Proof at 2 reviews — proof lines after two review passes (current shipped state) |
| **R** | Review ratio = LOP2R / LOP0R — how much two rounds of review contract or expand raw proof output |
| **C** | $/LOC from AI Paired Programming (APAS-AI): total spend on APAS-AI ÷ LOC |
| **N** | $/LOP from AI Paired Proof (APAS-VERUS): total spend on APAS-VERUS ÷ LOP2R |

### Source of truth for line counts

`analyses/veracity-count-loc.log` is the authoritative line-count source. Read
it (do not recount manually). Its `Summary` line gives spec/proof/exec/rust
breakdowns and its `Grand total` block gives function-based LOP/LOC.

Current shipped counts (from the log's Summary + Grand total):

| # | Metric | APAS-AI | APAS-VERUS | Notes |
|---|---|---|---|---|
| 1 | LOC (exec Rust) | 31,751 | 67,384 | APAS-AI from wc `src/*.rs`; APAS-VERUS `exec` column |
| 2 | Rust-only non-verus | — | 11,856 | APAS-VERUS `rust` column (outside `verus!`) |
| 3 | LOP2R (spec + proof) | — | 73,296 | 31,530 spec + 41,766 proof |
| 4 | LOP0R | — | ? | no snapshot captured yet — user supplies |
| 5 | R = LOP2R / LOP0R | — | ? | review multiplier |
| 6 | Spend ($) | ? | ? | total paired-session cost — user supplies |
| 7 | C = $/LOC | ? | — | APAS-AI rate |
| 8 | N = $/LOP2R | — | ? | APAS-VERUS rate on shipped proof |
| 9 | N / C | — | ? | proof cost premium vs code |

Update this table in place when the user supplies dollar figures or a LOP0R
snapshot — do not scatter the numbers across per-round reports. Refresh rows
1-3 after any `scripts/all-loc-by-chap.sh` (or equivalent) rerun that overwrites
`analyses/veracity-count-loc.log`.

---

## Source Layout & Structure

### File Locations

- Source files: `src/`
- Runtime tests: `tests/`
- Proof time tests: `rust_verify_test/tests/`
- Rust toolchain: pinned to 1.93.0 via `rust-toolchain.toml`

### Module Header Format

```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Brief module description.
```

### Table of Contents Standard

Every Verus source file follows **bottom-up, per-type ordering**. Sections 1-3 are
global. Sections 4-10 repeat per type with letter suffixes (a, b, c...), ordered
leaf-first. Section 11 appears once. Sections 12-14 repeat per type, bottom-up.

```
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4a. type definitions — struct Leaf
//  5a. view impls — struct Leaf
//  6a. spec fns — struct Leaf
//  7a. proof fns/broadcast groups — struct Leaf
//  8a. traits — struct Leaf
//  9a. impls — struct Leaf
//  4b. type definitions — struct Tree
//  5b. view impls — struct Tree
//  ...
//  9b. impls — struct Tree
//  10b. iterators — struct Tree
//  11. top level coarse locking
//  12a. derive impls in verus! — struct Leaf
//  12b. derive impls in verus! — struct Tree
//  13a. macros — struct Leaf
//  13b. macros — struct Tree
//  14a. derive impls outside verus! — struct Leaf
//  14b. derive impls outside verus! — struct Tree
```

- Sections 1-12: inside `verus!`. Sections 13-14: outside `verus!`.
- Each type gets a complete 4-10 cycle. Iterator structs, views, ghost structs, and
  all iterator trait impls live in section 10 with their parent type.
- Section headers use letter suffixes with type names: `// 4a. type definitions — struct Leaf`.
- Omit sections that don't apply (e.g., 10 for types without iterators, 11 for non-Mt).
- Section 11 is for Mt modules only. See `toplevel_coarse_rwlocks_for_mt_modules.rs`.
- For single-type files, omit the letter suffix: just `// 4. type definitions`.
- See `src/standards/table_of_contents_standard.rs` for the full compilable example.

### Use Statement Order

1. `use std::...` (standard library)
2. *(blank line)*
3. `use vstd::prelude::*;`
4. `use crate::Types::Types::*;`
5. `use crate::ChapNN::...::*;` (chapter modules, glob import)
6. `use crate::XLit;` (macros, by name not glob)

### lib.rs Structure

- No `verus_keep_ghost` in lib.rs (sole exception: `allocator_api` feature gate for Arc).
- Every chapter gets one unconditional `pub mod ChapNN` block.
- Files incompatible with Verus are **commented out** with a reason, not hidden behind cfg gates.
- Allowed cfg gates: `experiments_only`.
- See `.cursor/rules/apas-verus/lib-rs-structure.mdc` for full details.

---

## Verus Rules

### Running Verus

Use the wrapper scripts — never raw `verus`, `cargo verus`, or `cargo build` for verification:

```bash
scripts/validate.sh          # full verification
scripts/ptt.sh               # compile PTT library + run proof time tests
scripts/rtt.sh               # run time tests (cargo nextest)
```

- **All scripts log to `logs/`** with timestamped filenames:
  `logs/validate.YYYYMMDD-HHMMSS.log`, `logs/rtt.YYYYMMDD-HHMMSS.log`,
  `logs/ptt.YYYYMMDD-HHMMSS.log`, `logs/profile-ChapNN-YYYYMMDD-HHMMSS.log`.
  Profile summaries go to `logs/profile/SUMMARY-ChapNN-YYYYMMDD-HHMMSS.txt`.
  Output goes to both stdout and the log file (via tee).
- Verus includes its own vstd. Do not pass `-L dependency` or `--extern vstd`.
- **Never pipe, grep, sed, or tail the output of `scripts/validate.sh`**. The verification
  output contains the error messages you need to read to fix proofs. If the output is large,
  you may use `head -20` to see the first errors, but never filter or discard output.
- **Read the verification output.** If you don't read the error, you can't fix the proof.
- **NEVER re-run validate, rtt, ptt, or profile to check results you already have.**
  Every script logs to `logs/`. If it just ran, READ THE LOG. Do not re-run.
  ```bash
  ls -t logs/validate.*.log | head -1 | xargs cat   # last validate
  ls -t logs/rtt.*.log | head -1 | xargs cat         # last RTT
  ls -t logs/ptt.*.log | head -1 | xargs cat         # last PTT
  ls -t logs/profile-*.log | head -1 | xargs cat     # last profile console
  ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat  # last profile summary
  ```
  Re-running burns 2-10 minutes of CPU and RAM per run. The log has everything.
  Only re-run after you have made changes that require a new verification pass.
- Always show full output in response text as a markdown code block.
- **Run validate, rtt, and ptt sequentially, not in parallel.** They compete for CPU and
  memory. Verus holds large dependency graphs in memory; running concurrent builds can
  exhaust system RAM and lock the machine. Always: `validate` first, then `rtt`, then `ptt`.
  Never overlap them. Never run two validation passes at the same time.
- **Run each validation step individually.** After merges or changes, run each step as a
  separate command and check its output before proceeding to the next. Fix any trigger
  warnings or errors between steps. Do NOT use a combined pipeline script — each step must
  be independently verified clean.
- **DO NOT spawn subagents.** Do all work yourself, sequentially. Subagents running
  concurrent validates, RTTs, or PTTs will OOM the machine (32GB, Verus+Z3 needs 8GB+).
  If you need to read multiple files, use the Read tool directly — do not delegate to
  subagents. This applies to all agent work: editing, searching, validating, testing.
- **DO NOT run validate, rtt, or ptt from subagents.** If for any reason you do spawn
  a subagent, it must NEVER run `scripts/validate.sh`, `scripts/rtt.sh`, or
  `scripts/ptt.sh`. Validation is inherently sequential (single Verus process, single
  Z3 solver). Run validate ONCE in the main agent, tee the output to a temp file
  (`scripts/validate.sh 2>&1 | tee /tmp/validate.txt`), and have subagents read the
  temp file.

### Validation Modes

| Command | What it does |
|---|---|
| `validate` / `scripts/validate.sh` | Full verification, all modules |
| `validate isolate ChapNN` / `scripts/validate.sh isolate ChapNN` | Verify only ChapNN + its transitive deps (low memory) |
| `dev_only_validate` / `dov` | Foundation modules only (Types, Concurrency, vstdplus) |
| `V1` | Single verification run, show output, stop |
| `ptt` / `scripts/ptt.sh` | Compile PTT library + run proof time tests |
| `rtt` / `scripts/rtt.sh` | Run time tests (`cargo nextest run`) |

### Isolated Validation (isolate mode)

When working on a single chapter, use **isolate mode** to reduce memory and time:

```bash
scripts/validate.sh isolate Chap55      # verify Chap55 + transitive deps only
scripts/validate.sh isolate Chap43      # verify Chap43 + 8 deps (2.3GB vs 7GB)
```

How it works:
- Each chapter has a Cargo feature in `Cargo.toml` with its dependency list.
- `validate.sh isolate ChapNN` reads the dep table from `Cargo.toml`, computes the
  transitive closure, and passes `--cfg` flags for `isolate` + each included chapter.
- Foundation modules (Types, Concurrency, vstdplus) are always included.
- Standards, experiments, and non-dep chapters are excluded.

When to use:
- **During development**: always use isolate mode. It's 2-4× faster and uses 2-3 GB
  instead of 7+ GB. Multiple agents can validate in parallel with isolate.
- **Before pushing**: run a full `scripts/validate.sh` to confirm the complete crate
  verifies. Isolate mode can miss cross-chapter regressions.

Agents working on ChapNN should use `scripts/validate.sh isolate ChapNN` for all
iterative development, then `scripts/validate.sh` once at the end before pushing.

### Profiling rlimit failures

When a function exceeds its rlimit, **profile before guessing**. Add `--profile` to
the validate command:

```bash
scripts/validate.sh isolate Chap65 --profile    # profile a chapter
scripts/profile.sh isolate Chap65               # dedicated profile script
```

After it runs, read the summary:

```bash
ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat
```

Functions with >100K instantiations are matching loop candidates. The profile tells
you which quantifiers Z3 is stuck on — without it you are guessing. Always profile
an rlimit failure before changing proof strategy.

### Proof Holes

Use the wrapper scripts for hole queries (never grep manually, never call the binary directly):

```bash
scripts/holes.sh src/ChapNN/          # single chapter or file
scripts/all-holes-by-chap.sh          # all chapters, writes to analyses/ logs
```

Run `scripts/all-holes-by-chap.sh` after any rename or structural change that affects
proof hole analysis output. The per-chapter logs live at
`src/ChapNN/analyses/veracity-review-verus-proof-holes.log`.

### Analysis Scripts

All analysis scripts write per-chapter output to `src/ChapNN/analyses/`. Run them after
renames, structural changes, or when refreshing analysis baselines.

| Script | Output per chapter |
|---|---|
| `scripts/all-holes-by-chap.sh` | `veracity-review-verus-proof-holes.log` |
| `scripts/all-style-by-chap.sh` | `veracity-review-verus-style.log` |
| `scripts/all-fn-impls-by-chap.sh` | `veracity-review-module-fn-impls.{md,json}` |
| `scripts/resolve-analysis-merge.sh [dir]` | Resolves analysis-only merge conflicts (`--theirs`) |
| `scripts/resolve-analysis-rebase.sh [dir]` | Loops through rebase steps, resolves analysis-only conflicts (`--ours`) |
| `scripts/resolve-settings-merge.sh [dir]` | Unions `.claude/settings.local.json` allow lists from both conflict sides |
| `scripts/chapter-cleanliness-status.sh` | `analyses/chapter-cleanliness-status.log` |

**NEVER add `external_body`, `admit()`, or `assume(...)` without asking the user first.**

**NEVER add `accept()` or `// accept hole` proactively.** `accept()` is an APAS proof
function (`crate::vstdplus::accept::accept`) that replaces `assume` for intentional holes.
Adding `accept()` without explicit user approval hides real proof obligations and defeats
the verification effort. Do NOT use `accept()` to silence verification errors.

**DO NOT CONVERT `assume()` TO `accept()`.** If you see an `assume(...)` in existing code,
LEAVE IT ALONE. The user will decide when and whether to promote assumes to accepts. An
agent that mass-converts assumes to accepts is destroying 30 minutes of human time per
cleanup round. Don't be that agent.

**DO NOT `assume` or `accept` closure requires/ensures.** If a function body needs
`f.requires((...))` to hold, that obligation MUST be lifted into the function's own
`requires` clause so callers prove it. Writing `assume(f.requires(...))` or
`accept(f.requires(...))` in algorithmic code is always wrong — it vaporizes the proof
obligation instead of propagating it. Read `src/standards/using_closures_standard.rs`.

**DO NOT `assume` or `accept` eq/clone properties in algorithmic code.** The assume/accept
for Clone, PartialEq, and Eq bridges may ONLY appear inside `Clone::clone` and
`PartialEq::eq` bodies — nowhere else. If algorithmic code needs these properties, it
obtains them through the `ensures` clauses of `clone()` and `eq()`, not by assuming them.
Read `src/standards/partial_eq_eq_clone_standard.rs`.

The ONLY assume/accept patterns that may exist without per-instance user approval are:
- `assume` inside `PartialEq::eq` body (the eq/clone workaround pattern).
- `assume` inside `Clone::clone` body (the eq/clone workaround pattern).
- `assume(false); diverge()` in unreachable thread-join error arms.
- `external_body` at thread-spawn boundaries (not around algorithmic logic).

That's it. Four patterns. Nothing else.

Everything else — every `assume(...)`, every `accept(...)`, every `external_body` on
algorithmic logic — requires the user to explicitly request it. When in doubt, leave the
proof hole as-is and flag it for review. Do NOT "fix" a failing proof by adding accept.
Do NOT "clean up" assumes by converting them to accepts. Do NOT sprinkle accepts around
like confetti at a parade. The human has to clean up after you and it is not fun.

**DO NOT WEAKEN `ensures` TO MAKE PROOFS EASIER.** If a function's trait declares
`ensures key matches Some(k) ==> self@.dom().contains(k@)`, you must PROVE that
postcondition — not delete it and replace it with `ensures self@.dom().finite()`. Weakening
ensures to just `finite()` or `true` to avoid the hard proof is worse than leaving the
`external_body` in place. An `external_body` with a strong spec is a placeholder for a real
proof. A real body with a gutted spec is a regression — it destroys the contract that
callers depend on and that the textbook specifies. The specs come from APAS: `first(A) =
min[|A|]`, `previous(A,k) = max{k' | k' < k}`, etc. Those are the postconditions. If you
can't prove them, leave the `external_body` and report what you tried. Never gut the spec
to inflate your hole count.

**DO NOT ADD `requires true` TO FIX `fn_missing_requires` WARNINGS.** A `requires true`
is a vacuous precondition — it says nothing about the function's contract and is strictly
worse than leaving the warning in place. When veracity reports `fn_missing_requires`, the
fix is to add the **real** precondition: typically the module's well-formedness predicate
(`self.spec_<module>_wf()`) and any parameter constraints the function actually needs. Read
the function body, understand what it assumes about its inputs, and express that as a real
`requires` clause. If the function operates on `&self`, it almost certainly requires
`self.spec_<module>_wf()`. If it takes a key or index, it may require bounds. The goal is
a contract that callers must satisfy and that the proof can use — not a syntactic band-aid.

**DO NOT ADD TAUTOLOGICAL `requires` CLAUSES.** `requires 0nat <= usize::MAX as nat` is
just `requires true` wearing a disguise. Similarly, `requires values@.len() <= usize::MAX`
when `values` is a `Vec<T>` or `&[T]` is always true because Vec/slice lengths are `usize`.
Any requires clause that is provably always true is vacuous and MUST NOT be added. If you
cannot identify a real precondition, leave the function without requires and let veracity
flag it — do not paper over the warning with a tautology.

**DO NOT ADD `// veracity: no_requires` ANNOTATIONS.** Only the user adds these. The
`// veracity: no_requires` comment is a human-reviewed annotation that tells veracity to
suppress the `fn_missing_requires` warning for a function that genuinely has no precondition.
Agents must NEVER add this annotation. If you believe a function truly has no precondition,
leave it as-is and report it to the user. The user will decide whether to annotate it.

**Requires/ensures wf propagation.** If `f(x: &T) -> (y: U)` and the module defines
well-formedness predicates for `T` and/or `U`, then `f` needs `requires x.spec_<t>_wf()`
and `ensures y.spec_<u>_wf()`. Well-formedness flows through function boundaries: if the
input is well-formed, the output should be too, and both facts must be stated explicitly.
Do not assume callers "just know" the output is well-formed — prove it in the ensures.

### Search Locations ("The Usual Suspects")

1. **veracity-search** — vstd + APAS-VERUS function index (searched by default)
2. **vstd source** — `~/projects/verus/source/vstd/`
3. **Verus test suite** — `~/projects/verus/source/rust_verify_test/tests/`
4. **Verus examples** — `~/projects/verus/examples/`
5. **Verus community codebases** — `~/projects/VerusCodebases/`
6. **Verus Guide** — `https://verus-lang.github.io/verus/guide/`

Search before writing a new lemma. Use `veracity-search`:

```bash
veracity-search 'proof fn .*len.*'
veracity-search 'fn _ types Seq'
veracity-search -C ~/projects/APAS-VERUS 'proof fn lemma'
```

### Prefer vstd

Strongly prefer existing vstd functions, lemmas, spec functions, and types over new definitions.
Custom definitions create proof islands that don't connect to the broader ecosystem.

### Key Verus Patterns

**Assert in exec code**: `assert` in executable code does NOT need a `proof { }` block.
Only proof-mode calls (lemma invocations, `reveal`, `let ghost`) need `proof { }`.

**Triggers — MANDATORY**: Every `forall` and `exists` quantifier MUST have explicit
`#[trigger]` annotations. Do NOT use `#![auto]` — it causes "automatically chose triggers"
notes that flood verification output and hide real errors. Do NOT leave quantifiers without
trigger annotations. When you write a quantifier, add `#[trigger]` immediately. When Verus
prints "automatically chose triggers", read the trigger it selected and add that as an
explicit `#[trigger]`. This is not optional. Every agent commit must have zero trigger notes
in its assigned files.

When Verus emits "Could not automatically infer triggers" (an error, not a warning),
`#![auto]` will NOT help — there is nothing for it to select. You must manually add
`#[trigger]` to one or more terms in the quantifier body. A good trigger term is a
function application that mentions ALL bound variables and appears in both the hypothesis
and conclusion. Common patterns:

```rust
// Single trigger on a function call:
forall|i: int| 0 <= i < n ==> #[trigger] seq[i] == f(i)

// Trigger on a spec function:
forall|k: Key| #[trigger] table@.contains_key(k) ==> bucket_contains(table, k, hash)

// Multiple triggers when one term doesn't cover all bound vars:
forall|i: int, j: int| #![trigger seq1[i], seq2[j]] ...
```

If no function application mentions all bound variables, restructure the quantifier:
factor into two nested `forall`s, add a helper spec function that takes all bound
variables, or add a conjunction term that Verus can use as a trigger. Never leave a
trigger error unresolved — it means the SMT solver cannot instantiate the quantifier.

**Nested functions**: Do not use. Keep helpers at module level (Verus proof limitation).

**Meaningful return names**: Name return values meaningfully (`count`, `out_neighbors`,
`contains`), not generically (`result`, `ret`, `value`).

**`assume(false); diverge()` in thread join**: Valid idiom for unreachable error arms.
Do not use `assume(false)` anywhere else without asking.

**No `verus_keep_ghost` antipatterns**: No duplicate function implementations, no nightly
feature gates, no module gating in lib.rs.

**`#[cfg(not(verus_keep_ghost))]` is NEVER permitted on `fn`, `impl`, or `pub type`.**
It is ONLY permitted on `use` statements that import types Verus cannot parse. Specifically:

- **ALLOWED**: `#[cfg(not(verus_keep_ghost))] use std::collections::HashMap;` — import gate.
- **FORBIDDEN on fn**: `#[cfg(not(verus_keep_ghost))] fn foo()` — hides the function from
  Verus AND veracity AND cargo test. Even with `#[verifier::external_body]` alongside it,
  the cfg gate is wrong: `external_body` already tells Verus not to inspect the body, so
  the cfg gate only prevents the function from being tested in RTT.
- **FORBIDDEN on impl**: `#[cfg(not(verus_keep_ghost))] impl Foo for Bar` — same problem.
- **FORBIDDEN on type**: `#[cfg(not(verus_keep_ghost))] pub type T = ...` — same problem.

If a function body uses HashMap, rand, or other types Verus cannot parse, the correct fix
is to **replace those types with verified alternatives** (HashMapWithViewPlus, SeededRng,
SetStEph) so the cfg gate is unnecessary. Do not cfg-gate the function — replace the
unverifiable types.

**Never modify `~/projects/verus/`**. Find workarounds within APAS-VERUS.

**Never modify `~/projects/veracity/`**. Veracity is a separate tool maintained by a
separate AI agent. APAS-VERUS agents must not modify veracity source code, build veracity,
or change veracity's behavior. If you have feedback about veracity's output (wrong counts,
missing detections, etc.), write it to a file in `plans/` and report it — do not attempt
to fix veracity yourself. The only veracity interactions allowed are: (1) running the
existing binaries via `scripts/holes.sh`, `scripts/all-holes-by-chap.sh`, etc., and
(2) reading veracity's output logs.

**If you think Verus can't do X**: Search `src/experiments/` for existing tests, or propose
a new experiment. Do not assume limitations without evidence.

### Fork-Join Inside verus!

**Before writing or modifying any code that uses closures** — including `Fn`, `FnMut`,
`FnOnce`, `join()`, `filter`, `map`, `reduce`, `tabulate`, or any higher-order function —
**read `src/standards/using_closures_standard.rs` first.** Closure verification in Verus
has specific patterns for `requires`/`ensures` propagation, named closure variables, and
ghost captures. If you skip the standard, you will write unverifiable code.

All fork-join parallelism lives inside `verus!` using `join()` with named closures:

```rust
let ghost left_view = left@;
let f1 = move || -> (r: T) ensures post(r, left_view) { recurse(&left) };
let f2 = move || -> (r: T) ensures post(r, right_view) { recurse(&right) };
let (a, b) = join(f1, f2);
```

- Do NOT create `external_body` wrappers around `join()`.
- Do NOT put fork-join code outside `verus!`.
- Do NOT use inline closures — bind to named variables with explicit `ensures`.

### Threading Is Not an Excuse for external_body

Parallel algorithms have two parts: structural logic (verifiable) and thread spawning (not).
Wrap only the spawn boundary in `external_body`, not the whole algorithm.

### Arc Deref Pattern

Factor verification away from concurrency. Write a helper taking `f: &F` with full proof,
then have the trait impl delegate through `&*f` (Arc deref). Small `external_body` helpers
for Arc-specific operations with tight `ensures`.

### Ghost Parameter Sync

When a trait method signature includes `Ghost(...)` parameters, update all call sites to match.
The trait signature is the source of truth.

### Cargo Sync

When adding/deleting/renaming files registered in Cargo.toml, update the corresponding entries.

### Fix All Warnings

All Verus and Rust warnings and errors must be fixed before work is considered complete.

### Updating Verus

Use `vargo build --release`, not `cargo build`. See `~/projects/verus/BUILD.md`.

### Wrap vs Specify

- **SPECIFY** (`external_type_specification`): Adds specs to existing Rust type, no new struct.
- **WRAP** (new struct): When you need View mapping, extra methods, or additional invariants.

---

## APAS-VERUS Code Patterns

### Trait-Impl Pattern

Every APAS module defines a trait containing **all** public functions, with specs in the trait:

```rust
pub trait FooTrait: Sized {
    fn new(...) -> Self
        ensures ...;
    fn bar(&self, ...) -> (count: usize)
        requires ...,
        ensures ...;
}

impl FooTrait for Foo {
    fn new(...) -> Self { ... }
    fn bar(&self, ...) -> (count: usize) { ... }
}
```

Bare `impl Type` blocks are errors (exception: `&mut`-returning methods, standalone exercise
files). See `.cursor/rules/apas-verus/trait-impl-pattern.mdc`.

### Multi-Struct Spec Style (Recursive Enum with Per-Type Traits)

For tree-like types with multiple node kinds, use separate structs composed into a
discriminated enum, each with its own trait. Recursive spec fns go directly in the trait
impl with `decreases *self` — no inherent `impl` blocks or free spec fns needed. Child
traversal uses qualified trait calls: `NodeTrait::spec_size(&*n)`.

```rust
pub struct Leaf { pub key: u64 }
pub struct Interior { pub key: u64, pub left: Option<Box<Node>>, pub right: Option<Box<Node>> }
pub enum Node { LeafNode(Leaf), InteriorNode(Interior) }
pub struct Tree { pub child: Option<Box<Node>> }

// Each type gets its own trait with abstract specs and exec methods.
pub trait LeafTrait: Sized {
    spec fn spec_size(&self) -> nat;
    fn new(key: u64) -> (t: Self) ensures t.spec_size() == 1;
    fn set_key(&mut self, key: u64) ensures self.spec_contains(key);
}

pub trait NodeTrait: Sized {
    spec fn spec_size(&self) -> nat;
}

// Recursive specs go directly in the trait impl, not inherent blocks.
impl NodeTrait for Node {
    open spec fn spec_size(&self) -> nat
        decreases *self,
    {
        match *self {
            Node::LeafNode(_) => 1,
            Node::InteriorNode(i) => {
                let l = match i.left { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
                let r = match i.right { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
                1 + l + r
            },
        }
    }
}

// Non-recursive types reference children via qualified trait calls.
impl InteriorTrait for Interior {
    open spec fn spec_size(&self) -> nat {
        let l = match self.left { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
        let r = match self.right { None => 0nat, Some(n) => NodeTrait::spec_size(&*n) };
        1 + l + r
    }
}
```

Key rules: structs/traits/impls ordered bottom-up (Leaf, Interior, Node, Tree). Impl member
order matches trait declaration order. No stub delegation, no free spec fns, no inherent impl
blocks. Reference: `src/experiments/tree_module_style.rs`. See
`.cursor/rules/apas-verus/multi-struct-spec-style.mdc`.

### PartialEq/Eq Pattern

All inside `verus!`. Use `assume` (not `external_body`) in the eq body:

```rust
#[cfg(verus_keep_ghost)]
impl<T: View + PartialEq> PartialEqSpecImpl for MyType<T> {
    open spec fn obeys_eq_spec() -> bool { true }
    open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
}
impl<T: Eq + View> Eq for MyType<T> {}
impl<T: PartialEq + View> PartialEq for MyType<T> {
    fn eq(&self, other: &Self) -> (r: bool)
        ensures r == (self@ == other@)
    {
        let r = self.inner == other.inner;
        proof { assume(r == (self@ == other@)); }
        r
    }
}
```

### RwLockPredicate Naming

Every `RwLockPredicate` struct follows the **ModuleInv** convention: for module `X`, the
struct is `XInv`. Modules with multiple locks use a disambiguating infix (`XDimInv`,
`XMemoInv`, `XKeysInv`). The `inv` function must carry a real invariant — never just `true`.
Use `pub ghost` fields when the invariant needs construction-time context. Do not name
predicates `Wf`, `Pred`, or `Guard`. See `.cursor/rules/apas-verus/rwlock-predicate-naming.mdc`.

```rust
pub struct FooMtEphInv {
    pub ghost source: Seq<T>,
}
impl<T: Bounds> RwLockPredicate<LockedType<T>> for FooMtEphInv {
    open spec fn inv(self, v: LockedType<T>) -> bool {
        v@.len() == self.source.len()  // Real constraint, not `true`.
    }
}
```

### What Goes Inside vs Outside verus!

**Inside verus!**: Clone, PartialEq/Eq, Default, Drop, Iterator infrastructure (sections 1-11).

**Outside verus!**: Debug, Display, `macro_rules!`, unsafe marker traits, `&mut`-returning
methods, `#[cfg(...)]` stubs (sections 12-13).

**DO NOT MOVE DEFINITIONS OUT OF `verus!` THAT ARE ALREADY IN THERE.** If a function,
type, trait, or impl is inside `verus!`, it stays inside `verus!`. Moving code outside
`verus!` to dodge verification warnings (fn_missing_requires, fn_missing_requires_ensures,
etc.) is never acceptable — it removes the code from Verus's verification scope entirely,
which is the opposite of proving it. If you cannot fix the warning, leave it in place.

### Implementation Standalone Rules

- **Chapter standalone**: StEph, StPer, MtEph files of the same algorithm must NOT import
  specs/lemmas from each other. Duplicate shared specs. Each file is self-contained.
- **Mt standalone**: Mt files must NOT import from St counterparts. Define spec functions
  and proof lemmas locally.
- **Exception**: When APAS explicitly presents one algorithm as building on another
  (e.g., AVL extends BST).

### Types: Exile on N Street

- Use `u64`/`i64` for element values and arithmetic.
- Use `usize` only for indexing, lengths, capacities.
- Do NOT use the legacy `N = usize` type alias for new modules (Chap11+).

### Use Help-First Scheduler

For parallel scheduling, use `src/Chap02/HFSchedulerMtEph.rs`. Do not use raw
`std::thread::spawn` for fork-join parallelism.

### No Thread Threshold Optimization

This is a textbook. Do not add threshold checks to switch between parallel and sequential
for small inputs.

### Spec Function Naming

Name spec functions after the operation (`spec_inject`, `spec_ninject`), not as postconditions
(no `_post` suffix).

**Well-formedness specs**: `spec_<module>_wf` where `<module>` is the module name in lowercase
with no internal underscores. Do not use bare `spec_wf`. E.g., `spec_orderedtablestper_wf`,
`spec_augorderedtablemteph_wf`.

### XLit! Macros

Use `SetLit!`, `RelationLit!`, `MappingLit!`, `PairLit!` for constructing test values.

### Alg Analysis Annotations Must Be Based on Reading the Code

**Every `/// - Alg Analysis: Code review` annotation must be based on reading the
function body.** Never guess complexity from the function name. `nth` might be O(1)
(Vec index) or O(n) (linked list traversal). `height` might be O(1) (cached) or O(n)
(full tree walk). `from_vec` might be O(1) (wrap) or O(n) (copy). `insert` might be
O(1) (push) or O(lg n) (BST) or O(n) (sorted array). You cannot know without reading.

Do NOT use sed, regex, or pattern-matching to batch-insert annotations based on
function names. Each annotation is a cost analysis of a specific implementation.

### Failed Experiments

When an experiment fails verification: do NOT modify it to pass. Add `RESULT: FAILS` comment,
comment out the module in lib.rs. Failed experiments are documentation.

### Experiments Must Be Commented Out Before Agent Rounds

**Before launching any agent round**, verify that ALL experiment modules in lib.rs are
commented out. Experiments may have RTT failures, compilation issues, or unverified code
that breaks agent validation. The orchestrator must check `pub mod experiments { ... }` in
lib.rs and ensure every `pub mod` inside it is commented out with a status annotation
(SUCCEEDS, FAILS, TESTING, PARTIAL). Agents must never uncomment experiments.

### PTT Commands

```bash
scripts/ptt.sh               # compile PTT library + run proof time tests
scripts/rtt.sh               # run time tests (cargo nextest)
```

### When to Write PTTs

Only for two cases:
1. **Iterator verification** — confirm iterators prove correctly across loop forms.
2. **Complicated callability** — when `requires` is complex and you need confidence callers
   can satisfy it.

Do not create PTTs speculatively.

### Collection Iterator Standard

Every collection module implements the iterator standard from `docs/APAS-VERUSIterators.rs`.
Reference implementation: `src/Chap18/ArraySeqStEph.rs`. Requires 10 components inside
`verus!` section 10. See `.cursor/rules/apas-verus/collection-iterators.mdc` for full details
including PTT test patterns (6 patterns: loop-borrow-iter, loop-borrow-into, for-borrow-iter,
for-borrow-into, loop-consume, for-consume).

### Threaded Test Timeout

Tests using threads must have timeouts to prevent deadlock hangs.

---

## Code Style

### Comments

- **No decorative separators** in code comments. No `===`, `---`, `───`, box-drawing. Just
  plain words for section headers.
- **Own-line comments** (`//`, `///`, `//!`): full English sentences, capital letter, period.
- **End-of-line comments**: fragments OK.
- **No jejune comments**: Don't restate what the function name already says. Comments should
  tell the reader something non-obvious.
- **Use `///` bullet lists** for multi-line doc comments to preserve line breaks.

### Naming

- **No "helper" names**: Don't name functions `helper`, `inner`, `do_it`, `_impl`. Describe
  what the function does.
- **No trivial wrappers**: Don't create wrapper functions that merely forward without adding
  value (specs, type adaptation, trait conformance).
- **Preserve sketch names**: Don't rename parameters from user sketches without asking.
- **Use imports, not verbose crate paths**: `use crate::vstdplus::seq_set;` then
  `seq_set::lemma_...`, not `crate::vstdplus::seq_set::lemma_...`.

### Rename Files

Use `mv` to rename files, not read/write/delete.

---

## Review & Analysis Workflows

### Review Against Prose

When user says "review ChapNN": 8-phase review covering inventory, prose mapping, cost
annotations, spec fidelity, parallelism audit, RTT/PTT review, gap analysis, and TOC review.
See `.cursor/rules/apas-verus/review-against-prose.mdc` for the full procedure.

### Verusification Table

When user asks for "verusification table": produce prose coverage, per-file status, spec
strength assessment, and summary metrics.

### Classify Spec Strengths

Use `veracity-review-module-fn-impls` to generate function inventory, then classify each
function's spec as strong/partial/weak/none.

### Proposed Fixes Table

When user says "proposed fixes table": severity-ordered audit across chapters using
`veracity-review-proof-holes`. Severity: critical > high > medium > low.

### In/Out Table

Audit what code is inside vs outside `verus!` blocks. Values: `✅ in`, `✅ out`, `❌ in`,
`❌ out`, `-`.

### Verus Style Review

```bash
~/projects/veracity/target/release/veracity-review-verus-style \
  -c ~/projects/APAS-VERUS -e Chap21 -e vstdplus -e Types.rs \
  -e Concurrency.rs -e experiments -e lib.rs | grep -e warning
```

### Propose New Work

Run proof holes chapter by chapter, identify non-standard assumes, external_body on
algorithmic logic, bare_impl violations. Output table of actionable work.

### Gate Review-Against-Prose

Before regenerating a review, check if any inputs are newer than the existing review.
Skip if up-to-date; regenerate only changed sections if stale.

---

## Float/Graph Algorithm Strategy

Chap56-59 have duplicated Float/I64 files for graph algorithms. `vstdplus/float.rs` provides
`FloatTotalOrder` trait and basic axioms. Missing: arithmetic axioms (addition monotonicity,
identity). Key challenge: bridging `OrderedFloat<f64>` to f64 axioms. Strategy prioritizes
SSSPResult files (no float arithmetic) before Dijkstra/BellmanFord (need addition axioms).
See `.cursor/rules/apas-verus/float-axiom-fixes.mdc`.
