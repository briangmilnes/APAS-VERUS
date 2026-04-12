# R200 Prompt — Agent 1: Fix veracity-mislabeled exec code. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
   If you want to report a veracity bug, write to
   `plans/veracity-bugs/<filename>.md` — do not edit veracity source.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **Only uncomment lines whose comment header is
   `// Veracity: UNNEEDED proof block` AND whose content is
   demonstrably executable code, not a `proof { ... }` block or an
   `assert(...)` statement.** If you can't tell, leave it commented
   and report it for human review.

## Read all standards first.

**Especially read:**
- `src/standards/mut_standard.rs` — how Mt modules acquire/release
  locks.
- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the
  acquire/release pairing contract.

## Context

Agent1 R199 fixed a `filter` deadlock in `OrderedSetMtEph` by
uncommenting one line:

```
-// Veracity: UNNEEDED proof block             write_handle.release_write(locked_val);
+            write_handle.release_write(locked_val);
```

The veracity tool's "UNNEEDED proof block" pass mislabeled an
executable lock-release as a removable proof-mode comment. Grep
revealed this is **not** a one-off: at least five more cases of the
same pattern are visible on the current main.

## Known targets

### Category A — lock-operation mislabels (resource leaks → deadlocks
under writer contention)

| # | File | Line | Code commented out |
|---|------|------|--------------------|
| 1 | `src/Chap43/OrderedSetMtEph.rs` | 417 | `other_read.release_read();` (in `intersection`) |
| 2 | `src/Chap43/OrderedSetMtEph.rs` | 434 | `other_read.release_read();` |
| 3 | `src/Chap43/OrderedTableMtPer.rs` | 617 | `read_handle.release_read();` |
| 4 | `src/Chap43/OrderedTableMtPer.rs` | 697 | `read_handle.release_read();` |

Each is paired with a prior `acquire_read()` and must be un-commented
to balance the acquire/release contract.

### Category B — commented-out algorithmic step (correctness bug)

| # | File | Line | Code commented out |
|---|------|------|--------------------|
| 5 | `src/Chap39/BSTTreapStEph.rs` | 1336 | `rotated.left = Self::delete_link(rotated.left.take(), target);` |

This is a **recursive descent** into the left subtree of a treap
delete. Leaving it commented silently breaks delete in that branch.

## Plan

### Step 1 — uncomment the five known mislabels

For each of the 5 lines in Category A and B:

1. Open the file, confirm the commented line at the given line number
   matches the description above.
2. Remove the `// Veracity: UNNEEDED proof block ` prefix and the
   leading whitespace re-alignment, leaving just the executable code
   with the right indentation.
3. Save.

If any line doesn't match (e.g., line numbers shifted since this
prompt was written), find the equivalent line by grep on the
function name + exact code pattern, and fix.

### Step 2 — broader grep audit

Grep for every `^// Veracity: UNNEEDED proof block ` comment where
the content appears to be executable (not an `assert(...)` or
`proof { ... }` block). Use:

```bash
grep -rnE "^// Veracity: UNNEEDED proof block\s+[^/]" src/ \
  | grep -vE "assert\(|proof \{|assume\(|ghost " \
  | head -40
```

For each non-obvious hit, read 5 lines of context (before and after)
and decide:

- **Exec-like**: a method call (`.release_*`, `.take()`, `.push()`,
  `.insert()`, `.iter_mut()`, any `Self::*` recursion, any `=`
  assignment to a non-ghost lvalue). → **uncomment**.
- **Proof-like**: an `assert`, a `proof {}` block body, a lemma
  call, a `reveal(...)`, a `use_type_invariant(...)`. → **leave
  commented**.
- **Ghost operation**: `Ghost(...)` assignment, `let ghost ...`. →
  usually leave commented, but note in report.

Build a table in `plans/r200-veracity-mislabel-audit.md` of every
candidate and the action taken.

### Step 3 — validate

Run:

```bash
scripts/validate.sh isolate Chap43
scripts/validate.sh isolate Chap39
```

Both must be clean. The uncommented lines should verify — they were
commented by the tool claiming they were "UNNEEDED", which suggests
the proof still closes without them. Putting them back should **also**
verify (the tool's claim is about removability, not required-absence).

If a reactivated line breaks verification, that's a veracity-tool
bug beyond "it commented out exec code" — it also means the tool
mislearned what's proved. STOP and report that specific case.

After isolate passes:

```bash
scripts/validate.sh
scripts/rtt.sh
```

Both must remain clean. RTT count should be unchanged or up (5690
verified, 4162 RTT expected per R199 baseline).

### Step 4 — PTT

Per project rule, PTT is expensive (~4 min). Run **once** at the end:

```bash
scripts/ptt.sh
```

Should be 225 passed (unchanged from R199).

### Step 5 — file a veracity bug report

Create `plans/veracity-bugs/UNNEEDED-proof-block-mislabels-exec-code.md`
describing the pattern:

- The veracity tool's "UNNEEDED proof block" pass labels
  executable-mode code as removable proof scaffolding.
- Root cause hypothesis: the tool matches structurally by proximity
  to `proof { }` blocks or `// Veracity: ...` markers and doesn't
  distinguish exec-mode statements from proof-mode ones.
- Observed effect: silent resource leaks (lock-release removed) and
  silent correctness bugs (recursive algorithm step removed).
- Evidence: list every (file, line) fixed in this round.

The veracity agent will read this. **Do not edit the veracity source
code.**

## Out of scope

- Writing new iterators (agent2's R199 scope).
- Rewriting tests or src/ logic.
- Any proof-mode `// Veracity: UNNEEDED proof block` (those are fine
  — the tool got them right; the labels on *proof code* are correct).
- PTT iteration. Run once at the end only.

## Report

Write `plans/agent1-round200-report.md` with:

- Per-file fix table (file, line number, what was uncommented,
  one-sentence explanation of why it was exec-not-proof).
- Category A (lock ops) + Category B (algorithmic) counts.
- Audit result: total candidates surfaced by grep, classification
  (exec / proof / ghost), action for each.
- Validation numbers (validate / rtt / ptt counts vs R199 baseline).
- Pointer to the veracity bug report file.

## RCP

```
git add -A
git commit -m "R200 Agent 1: uncomment 5 veracity-mislabeled exec lines (4 lock releases + 1 delete recursion); audit report; veracity bug report"
git push
```
