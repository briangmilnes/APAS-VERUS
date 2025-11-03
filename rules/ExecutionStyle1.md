### Tooling, Execution Protocol, and Transparency

## Execution Mode Selection

**Default Mode: Relentless Execution**

By default, execute tasks without stopping for confirmation. Only stop for:
1. **Data loss risk**: git push --force, rm -rf, destructive git operations
2. **User-specific info needed**: Credentials, API keys, personal preferences
3. **Three failed fix attempts**: After 3 attempts to fix an error, mark TODO failed and continue to next

**Careful Mode (Explicit Opt-In)**

User activates by saying "careful mode" or "ask first":
- Stop before large Python scripts modifying >10 files
- Stop before sweeps affecting >50 files
- Remind about git commits before destructive operations

**Key Principle**: Brief notifications replace approval requests. Say "Running cleanup on 5 files..." then proceed immediately.

---

### TODO Flow and Execution Discipline

#### Relentless TODO Flow (No-Pause Autopilot; 3-Attempt; Completion Guard)
- SEV levels:
  - SEV1: Critical breakage (data loss/destructive edits).
  - SEV2: Workflow breach (stopping/asking during execution; missed TODO completion; failure to continue relentlessly).
  - SEV3: Minor style/process deviations.
- Scope: applies to all execution unless "careful mode" specified.
- Execution: synchronous; clear terminal in a separate call; non-interactive flags; always `| cat`; one command per terminal invocation.
- Timing: assistant-text Start/End/Total from the system clock.
- Attempts: on any error, make up to 3 self-directed fix attempts; clear before each; keep edits minimal/local.
- No-pause Autopilot (hard rule): do not ask for confirmation or pause for review. Continue automatically unless one of the hard stops occurs (see Execution Mode Selection above).
  - Drive mode: shorthand for relentless execution—stay in motion without pausing for confirmation.
  - Violation is SEV2: Treat any stop/question/confirmation prompt during execution as a SEV2 bug. Immediately resume, log the violation in the status, and proceed relentlessly.
  - On hard stop: mark the TODO failed, print exact diagnostics, create a "Resume..." TODO, and immediately continue to the next independent TODO.
- Completion guard: after a successful step, immediately complete the active TODO; if another exists, set it `in_progress` and print its Start; otherwise print "All TODOs completed". No dangling TODOs (except explicit "[long-run]").
- Long-run tasks: only when tagged "[long-run]"; exclude from totals until stopped; don't mark complete until stop finishes.

#### Per-TODO Execution (hard gate)
- On setting a TODO `in_progress`: print Start via terminal:
  - `date +"Start: %a %b %d %T %Z %Y"`
- Execute exactly one synchronous action (or a clearly batched set): edits and/or 1 command, piped with `2>&1 | cat`. Reuse SAME Start for retries.
- On finishing: print End via terminal:
  - `date +"End: %a %b %d %T %Z %Y"`
- Provide a brief summary. Immediately complete or fail the TODO; if another exists, set it `in_progress` and print its Start.

#### TODO Completion Guard (Last Item)
- Same-message order:
  1) End and Total
  2) Brief summary
  3) Complete active TODO
  4) If another exists, set `in_progress` and print Start; else "All TODOs completed".
- Only `[long-run]` can remain active without completion.

#### Immediate TODO-completion guards (hard rules)
- Immediate completion on success (mandatory): immediately after any successful edit/command that fulfills the active TODO, call todo_write (merge=true) to mark it completed before any further tool calls or sending the message.
- No-call invariant: you may not start another tool call while any fulfilled TODO remains in_progress.
- No-send invariant: you may not send a message with a fulfilled TODO still in_progress (except explicit "[long-run]").
- Single-active: at most one TODO may be in_progress. To start the next task, first complete the current one; only mark the next TODO in_progress when its first tool call starts.
- Start-of-turn reconcile: before the first tool call each turn, if any prior in_progress TODO is already fulfilled, complete it via todo_write immediately.
- Batch completions: if one operation satisfies multiple TODOs, complete all of them in a single todo_write batch right after that operation.
- Escalation on miss (SEV2): if a fulfilled TODO is found in_progress at the start of a turn, immediately complete it and create a "Follow-up: missed TODO completion (SEV2)" TODO, then continue.

#### Clearing TO-DOS
- Overwrite the entire list via the todo tool with `merge=false` and `todos=[]`. State "All TODOs cleared." and stop.

#### Update todo status eagerly
- If a TODO is done, check it off immediately and set the next TODO to `in_progress`.

#### Never Stop Until All TODOs Complete (Absolute Completion Rule)
- **NEVER declare victory or completion until ALL TODO items are marked as "completed"**
- Always check the TODO list status before claiming a task is finished
- If any TODOs remain pending or in_progress, continue working relentlessly until every single item shows status "completed"
- Do not ask for permission to continue - just keep executing until 100% completion is achieved
- **NO EXCEPTIONS** for declaring success until the todo list shows 100% completion status
- This rule enhances all existing TODO Flow rules with an absolute completion requirement

#### Pause Question
- When asked if you are paused, answer explicitly: "Yes I am paused" or "No I am not"; explain why or what you're doing; continue with the work.

#### Reordering or changing the task list
- **CRITICAL**: Do not stop and ask to reorder a task list that the user has OK'd.
- In a sweep if you have a faster way to do the job you may propose it to the user.

---

### Verification and Quality

#### Mandatory Build Verification for Source Code Modifications
- **FINAL STEP REQUIREMENT**: The final steps of ANY source code modification MUST include:
  1. `cargo build` - MANDATORY for all `src/` changes
  2. `cargo nextest run` - MANDATORY if test code or functionality that affects tests was modified
- **NO EXCEPTIONS**: These verification steps are non-negotiable and must pass without warnings or errors
- **Failure Protocol**: If build/test fails, immediately fix the issues before declaring completion
- **TODO Completion Rule**: Mark TODOs as completed ONLY after successful build verification

#### Zero Warnings Policy
- All code changes must result in zero compiler warnings
- Fix warnings immediately as part of the implementation
- Never mark a TODO complete if warnings remain

#### Git State Verification Before Assumptions
- **MANDATORY**: Before assuming code state or declaring regressions, ALWAYS check:
  1. `git status` - to see current working tree state
  2. `git log --oneline -5` - to verify recent commits/reverts
  3. File timestamps if needed - to understand when changes occurred
  4. `git show --stat HEAD` - to see what the last commit actually changed
- **Never assume** the user has reverted code without verifying git state first
- **Common scenario**: User may commit structural changes but not commit Claude's fixes, leaving fixes uncommitted and lost
- **Rule violation**: Making assumptions about code regressions without git verification is a workflow error

---

### Tool Usage and Transparency

#### Tool Usage Transparency
- Announce every tool call before it runs (single line). Formats:
  - `Tool: read_file — path: /abs/path/file.rs`
  - `Tool: edit_file — file: /abs/path/file.rs`
  - `Tool: apply_patch — file: /abs/path/file.rs`
  - `Tool: codebase_search — query: "…"; target: ["…"]`
  - `Tool: grep — pattern: '…'; path: /abs/path; type/glob: rs`
  - `Tool: todo_write — merge: true; items: N`
  - `Tool: read_lints — paths: ["…"]`
  - `Tool: list_dir — target: /abs/dir`
  - `Tool: glob_file_search — target: /abs/dir; glob: "**/*.rs"`
  - `Tool: run_terminal_cmd — command: "…"`
- Redact secrets; truncate commands >160 chars. Stop announcements on STOP.
- Sweeps (hard rule, SEV2 on miss): During sweeps, echo EVERY tool call and batch related calls per step. Missing tool-echo lines or silent tool usage during a sweep is a SEV2 violation; resume immediately and continue relentlessly.

#### Tool Calls — Batching, Retries, Background
- Parallel: print a single `ToolCalls:` block, one line per tool in the batch.
- Retries: append `(attempt X/3)` to the tool line.
- Background: append `(background)` for background jobs.
- Clear policy: for shell commands, always use a clear-first in a separate invocation before running the command.

#### Terminal Clear & Output Visibility
- Clear-first, separate calls: if you need to clear the terminal, run that clear command in its own invocation, then run the actual command in a new invocation. Never chain clear + command with `&&`.
- One command per terminal window. Always pipe command output via `| cat`.
- Exception: if you explicitly say "no-clear", skip the clear step; still keep one command per terminal invocation.
- Present every command's captured output inside fenced code blocks so the terminal formatting box appears consistently.
- Echo the output immediately after the command finishes; do not defer or batch command logs.

#### Command Timestamping
- At the start of each work batch, print the current time via `date +"%d %H:%M:%S %Z"` before issuing the first shell command.
- For any `cargo` command, emit an additional timestamp immediately beforehand using the same format.
- Show each timestamp in the transcript (code block), then execute the command as a separate invocation.
- Print the timestamp right away in the response so the user sees it before any subsequent command output.
- Avoid Start/End markers unless the user explicitly requests them.
- Do not emit extra timing commands beyond these requirements.

#### Verification Trace (imports & bounds hoist)
- For each file verification, print:
  - `Verifying: <ABS-PATH>`
  - `Start: HH:MM:SS (system TZ)`
  - Checks (one line each):
    - imports: inside `src` use `crate::`; wildcard module imports; no `extern crate`
    - macros: exported at crate root; `$crate` paths; type-check helper present
    - bounds hoist: gather per-method bounds; hoist union to trait/impl; do not lift lifetimes; mirror impls
    - baseline: public APIs include `Clone + Sized + Display + Debug` at declaration sites
    - naming: CamlCase for multi-word items; files start with a capital
  - Tool calls announced via Tool Usage Transparency.
  - `End: HH:MM:SS (system TZ); Total: H:MM:SS`

#### High-Signal Live Tracing (mandatory)
- Heartbeat after every batch: After every tool call batch and every code edit, write a 1–2 line status update naming the active TODO, files touched, and the next action.
- No-quiet window: If 60 seconds elapse or 2 consecutive tool batches occur without user-visible output, send a status update anyway (e.g., "still building/searching…").
- Command echo: Before running a non-trivial shell command, print the exact command in one line; after it finishes, note success/failure.
- Build/test digest: After any build/test, output a 3–8 line digest including the first failing file:line with error code, total error count (approximate OK), and the concrete next fix.
- Edit diff signal: After any file edit, report the file path and a short change label (e.g., "hoisted T: StT; tuples→Pair").
- Search signal: For broad searches, report match counts and top 1–3 relevant paths (or "no matches").
- TODO-anchored tracing: Each status must reference the active TODO name and state, plus the immediate next step.
- Failure sentinel: On repeat of the same error, state the suspected root cause and the specific change being applied next.
- Blocked state: If input is required, state the exact question and mark the TODO as blocked until answered.

---

### Environment, Tools, and Misc

#### Git Terminology Consistency
- Refer to repository state with `git` phrases: say `git-untracked`, `git-committed`, and `git-pushed`. Avoid substitutes such as "shipped."

#### Cargo Nextest
- `cargo nextest` writes to stderr; use `2>&1 | cat` to capture output.
- When matching Claude's workflow, run `cargo nextest --no-fail-fast --no-capture --color never 2>&1 | cat` so the full stream is visible while preserving identical options.
- When touching benchmark code or configs, begin with `cargo bench --no-run 2>&1 | cat` to surface compile issues quickly before full runs.

#### GRIND - Comprehensive Build/Test/Bench Check
- **Full codebase check**: `scripts/grind_codebase.py`
  - Compile source: `cargo check --lib -j 10`
  - Compile tests: `cargo test --no-run -j 10`
  - Run tests: `cargo nextest run --no-fail-fast -j 10`
  - Compile benchmarks: `cargo bench --no-run -j 10`
  - Stops at first failure for fast feedback
  
- **Single module check**: `scripts/grind_module.py <ModuleName>`
  - Example: `scripts/grind_module.py AVLTreeSeq`
  - Compile source (lib only)
  - Find and compile ALL tests for that module (reads from Cargo.toml)
  - Run ALL tests for that module
  - Compile ALL benchmarks for that module
  - Handles multiple test/bench files per module (e.g., TestAVLTreeSeqStPer18, TestAVLTreeSeqStEphChap37)
  
- **Use when**: 
  - `grind_codebase`: Making changes that affect multiple parts of the codebase
  - `grind_module`: Testing a specific module after changes
- **Output**: Both stream incrementally, show clear step-by-step progress
- **Aliases**: User may say "grind" - means full codebase check

#### No Perl; awk acceptable
- Use `rg/grep`; `awk` only if needed; never Perl.

#### No heredocs
- **NEVER use heredocs** (bash `<<EOF` syntax) in terminal commands or scripts
- Heredocs make code unreadable and break copy-paste workflows
- Instead: write to files using proper tools (write tool, echo with >, Python scripts)
- If you need multi-line output, use Python or proper file writing

#### CPR rule (heartbeat lines)
- Always write brief heartbeat lines (e.g., "Thinking" or "Planning") so progress is visible—before a tool batch, after edits/commands, and at completion.

#### Minimal Constructor Surface
- Prefer inherent constructors/macros (`new`, `from_vec`, `Lit![]`) over struct literals in tests.

#### Claude and sed
- Claude is not allowed to run sed to attempt to fix bracket problems.

#### Tool parallelism
- Run cargo bench -j 10, too many rustcs are being forked.

#### emacs_sync
- Trigger: when you say you'll "fix/edit in Emacs," pause all write edits immediately.
- Pause behavior: read-only work only (reviews/plans), no file edits until "Emacs: done".
- Resync on done: propose refreshing repo state, then re-open changed files.
  - Suggested:
    ```bash
    git status -s | cat
    git diff --name-only | cat
    ```
- Reconcile: prefer your Emacs changes; never overwrite. If conflicts arise, summarize diffs and ask how to proceed.
- Style guarantees: keep existing indentation, whitespace, and CamelCase module naming intact.
- Scope hygiene: avoid global refactors while you're editing; target only files you didn't modify unless you approve.
- Follow-ups: after resync, run lint/diagnostics on changed files and resume the pending task.

#### Do not load from rules or prompts files
- Do not read files from the rules or prompts directory without explicit instruction; content may be out of date.

#### Project Organization
- **Analysis outputs**: Place all analysis results, reports, and summaries in `analyses/` directory
  - Organized by category: `analyses/benchmarks/`, `analyses/coverage/`, `analyses/code_quality/`, `analyses/implementation/`, `analyses/todos/`
  - Never leave analysis outputs at project root
  - **File versioning anti-pattern**: Never create `_updated`, `_new`, `_fixed`, `_v2`, etc. versions
    - Update files in place: `analysis.txt`, not `analysis_updated.txt`
    - If regenerating an analysis, overwrite the original file
    - Use git for versioning, not filename suffixes
- **Script output and logging**:
  - **Scripts MUST log by default**: All analysis/detection scripts must write output files automatically
    - Don't rely on shell redirection (`> file.txt`) - easy to forget
    - Scripts should take `--log_file` flag with default paths (e.g., `analyses/code_review/<script_name>.txt`)
    - Both stdout (for user) AND file (for analysis) should be produced
    - Example: Python script uses `tee` pattern - print to console and write to file
  - **Output naming**: `<script_name>.txt` (no dates - git provides timestamps)
    - Detection scripts: `analyses/code_review/detect_<pattern>.txt`
    - Fix scripts: `analyses/code_review/fix_<pattern>.txt`
    - Grind/test runs: `analyses/build_logs/grind_<module>.txt`
    - Update in place when re-running (git tracks changes)
  - **Analysis workflow**: run script → review output → grep/analyze → commit with summary
    - Large outputs: use grep/awk to extract key information
    - Add findings to commit message or separate summary file
    - Git checkout + log provides temporal context
- **Scripts organization**: Place all scripts in appropriate `scripts/` subdirectories
  - `scripts/APAS/src/` - APAS source code review and fixes
  - `scripts/APAS/tests/` - APAS test utilities
  - `scripts/APAS/benches/` - APAS benchmark utilities
  - `scripts/rust/src/` - General Rust compilation fixes
  - `scripts/rust/tests/` - General Rust test utilities
  - `scripts/rust/benches/` - General Rust benchmark utilities
  - `scripts/benches/` - Shared benchmark utilities
  - `scripts/counting/` - Counting and metrics utilities
  - `scripts/tmp/` - **Temporary one-time scripts only** (migrations, one-off fixes, reverts)
  - See `scripts/README.md` for complete organization
  - **Script location**: Never use `/tmp` for scripts - they damage future research
    - Temporary exploratory scripts go in `scripts/tmp/`
    - Reusable tools go in appropriate permanent locations
  - **Script naming**: No version suffixes (`_v2`, `_v3`, etc.)
    - Use base name for current version: `fix_trait_forwarding.py`
    - NOT: `fix_trait_forwarding_v3.py`
    - Use git for versioning, not filename suffixes
    - Exception: Legacy scripts may retain version numbers temporarily during transition
- **One-time scripts**: All temporary single-use scripts go in `scripts/tmp/`
  - Migration scripts (rename files, restructure)
  - One-time pattern fixes or reverts
  - Batch compilation error fixes from specific changes
  - **Never** put permanent/reusable tools in `tmp/`
  - Clean out periodically after scripts serve their purpose
