# R201 Prompt — Agent 3: APAS-AI quantitative snapshot for lecture comparison. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent3`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER modify `~/projects/APAS-AI/`.** Read-only target.
4. **You MAY and MUST build `~/projects/rusticate/`** (see Step 1).
   No source modifications there either — just `cargo build --release`.
5. **NEVER run `rm -rf` on any directory.**
6. **NEVER run `git clean`.**
7. **NEVER add `assume`, `admit`, `accept`, or `external_body`** —
   this round only reads and writes report files.

## Read all standards first.

(Even though this round only touches `lectures/quantitatives/`,
read the standards so the formatting / presentation follows project
conventions — tables with `#` index column, `Chap` column for
per-chapter rows, etc.)

## Context

The lecture being prepared for UW CSE / MSR / CMU will compare
APAS-VERUS (the current formally-verified project, 5+ months of
work) against **APAS-AI** (the earlier prototype, roughly 3 months
of work, no formal verification). Quantitative data on APAS-AI is
needed alongside the existing APAS-VERUS quantitatives at
`lectures/quantitatives/`.

APAS-AI lives at `~/projects/APAS-AI`. Core library at
`~/projects/APAS-AI/apas-ai/` (42 chapter directories, ~238 src
files, 175 bench files, 252 test files, ~820 total .rs files).

The LOC counter `rusticate-count-loc` lives in
`~/projects/rusticate`. Cargo release binary is not currently built
— step 1 rebuilds it.

The user notes that APAS-AI continued receiving stray fixes
(from APAS-VERUS work) after the formal APAS-VERUS project
started. So **last-commit date is not the same as "when the
APAS-AI project ended"**. The report must distinguish active
development vs. stray fixes.

## Goal

Produce `lectures/quantitatives/apas-ai-snapshot.md` — a parallel
to the existing APAS-VERUS `scale.md` / `cost.md` / etc. files —
capturing everything quantitative about APAS-AI that an AI or
human can extract without reading every source file.

## Plan

### Step 1: Build rusticate

```bash
cd ~/projects/rusticate
cargo build --release 2>&1 | tail -20
```

Verify the `rusticate-count-loc` binary is present:

```bash
ls -la ~/projects/rusticate/target/release/rusticate-count-loc
```

If the build fails (it hasn't been built in a while; dependencies
may have shifted), report the failure and stop here — this is a
dev-env issue that needs user attention, not agent repair.

### Step 2: Run rusticate-count-loc on APAS-AI

```bash
cd ~/projects/APAS-AI
~/projects/rusticate/target/release/rusticate-count-loc -h 2>&1 | head -30
```

Find the invocation that produces per-file and per-project LOC
tables (check `src/bin/count_loc.rs` source or `--help`). Then run
it against the `apas-ai/` library subdirectory and capture output.

Put the raw output at
`lectures/quantitatives/raw/rusticate-count-loc-apas-ai.log`.

### Step 3: Run veracity-count-loc on APAS-AI

```bash
cd ~/projects/APAS-AI
veracity-count-loc -c -a 2>&1 | tee /tmp/veracity-count-loc-apas-ai.log
```

Note: `-p APAS` is for APAS-VERUS; APAS-AI may or may not need it.
Try without first. If the output references Verus spec/proof/exec
separation, that's not applicable here (APAS-AI has no Verus) —
the totals (rust / comments / tests) are what matter.

Put the raw output at
`lectures/quantitatives/raw/veracity-count-loc-apas-ai.log`.

### Step 4: Git metadata extraction

```bash
cd ~/projects/APAS-AI
git log --reverse --format='%ad %h %s' --date=short | head -5
git log -1 --format='%ad %h %s' --date=short
git rev-list --count HEAD
```

Capture:
- First commit date + message + SHA
- Last commit date + message + SHA
- Total commit count
- **Active-development span heuristic**: find the date after which
  commit frequency drops sharply. Two approaches:
  - `git log --format='%ad' --date=short | sort | uniq -c`
    (commits per day — look for the last high-activity day)
  - Find the last commit that materially touched `apas-ai/src/` (as
    opposed to just docs or analyses): `git log --format='%H %ad %s'
    --date=short -- apas-ai/src/ | head -5`

Record all of these. The report should distinguish:
  - Project calendar span (first → last commit).
  - Active development span (first → last material src/ commit).
  - Tail of stray fixes after active development ended.

### Step 5: File / chapter inventory

```bash
cd ~/projects/APAS-AI
find apas-ai/src -type d -name 'Chap*' | wc -l         # chapters
find apas-ai/src -name '*.rs' | wc -l                  # src files
find . -path '*/benches/*.rs' | wc -l                  # bench files
find . -path '*/tests/*.rs' | wc -l                    # test files
find . -name '*.rs' | wc -l                            # all rs files
```

Also:
- Per-chapter file count (which chapter is largest?).
- Enumerate chapter numbers present vs. APAS-VERUS's 44 chapters;
  note differences.

### Step 6: Compile the report

Write `lectures/quantitatives/apas-ai-snapshot.md` with these
sections (follow the style of the existing `scale.md`):

1. **Project timeline** — first commit date, last commit date,
   active-development span (explaining the distinction), total
   commit count.
2. **File and chapter counts** — mirrors the APAS-VERUS `scale.md`
   Section 1 table.
3. **LOC totals** — from both rusticate-count-loc and
   veracity-count-loc (cross-reference). If they disagree, note by
   how much and why (different counting rules — rusticate is
   token-aware AST parsing, veracity is regex-ish).
4. **Per-chapter LOC table** — mirrors APAS-VERUS `scale.md` Section
   1 per-chapter breakdown.
5. **Comparison to APAS-VERUS** — side-by-side numbers. Small table
   at the end:
   ```
   | Metric | APAS-AI | APAS-VERUS | Ratio |
   | Total LOC | ... | 186,223 | ... |
   | Src files | ... | 262 | ... |
   | Chapters | 42 | 44 | ... |
   | Active span (months) | ... | 5.3 | ... |
   | Commits | 347 | 2,596 | ... |
   ```
   (Use current-main numbers for APAS-VERUS; ping against
   `lectures/quantitatives/scale.md`.)
6. **Caveats** — any counting-rule differences, any gaps, the
   "stray fixes after project end" distinction.

### Step 7: Raw artifacts

Keep raw output logs at `lectures/quantitatives/raw/` so the user
or a future agent can re-derive without re-running. Files:
- `rusticate-count-loc-apas-ai.log`
- `veracity-count-loc-apas-ai.log`
- (Optional) `git-log-apas-ai.txt` — `git log --format='%ad %h %s' --date=short`

### Step 8: Validate + commit

This round touches no `src/`, no `tests/`, no `rust_verify_test/`.
No `validate.sh`/`rtt.sh`/`ptt.sh` need running.

```bash
cd ~/projects/APAS-VERUS-agent3
git add lectures/quantitatives/
git status --short
```

Confirm only new files under `lectures/quantitatives/` are staged.

## Out of scope

- Modifying APAS-AI source (read-only).
- Modifying rusticate source (only rebuild).
- Running Verus or any verification on APAS-AI — it's not verified.
- Adding APAS-VERUS quantitative data or refreshing
  `lectures/quantitatives/scale.md` / `cost.md` / etc. — that
  belongs in its own round.
- Classifying APAS-AI chapter coverage against the APAS textbook
  (same deferred work as APAS-VERUS Section 2).

## Report

Write `plans/agent3-round201-report.md` with:

- Rusticate build outcome (clean / failed with reason).
- rusticate-count-loc: headline totals (spec/proof/exec/rust/
  comments, though "spec/proof" will be zero for APAS-AI).
- veracity-count-loc: headline totals (cross-reference; flag any
  disagreement).
- APAS-AI git metadata: first/last/active-span dates, commit
  counts.
- File/chapter totals.
- The key comparison row (APAS-AI vs APAS-VERUS LOC, files,
  chapters, duration, commits).
- Any caveats or blockers encountered.

## RCP

```
git add -A
git commit -m "R201 Agent 3: APAS-AI quantitative snapshot for lecture comparison"
git push
```
