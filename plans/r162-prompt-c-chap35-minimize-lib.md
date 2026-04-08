# R162 Prompt C — Minimize Chap35 via veracity-minimize-lib. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
5. **NEVER run RTTs.** Skip `scripts/rtt.sh` entirely.
6. **NEVER delete `target/` or any subdirectory.**

## Your task

Run veracity-minimize-lib on Chap35, process the results, validate, and push.

## Files

- `src/Chap35/OrderStatSelectMtEph.rs` — 94 asserts
- `src/Chap35/OrderStatSelectMtPer.rs` — 93 asserts
- `src/Chap35/OrderStatSelectStPer.rs` — 65 asserts
- `src/Chap35/OrderStatSelectStEph.rs` — 65 asserts

Total: ~317 asserts. Isolate time: ~11s per run. Estimated minimize runtime: ~58 min.

## Step 1 — Confirm clean state

Your worktree must have no uncommitted changes before veracity will run.

```bash
git status
```

If anything is uncommitted, commit it now. Veracity exits if there are uncommitted changes.

## Step 2 — Run veracity-minimize-lib

Run from the veracity project directory, targeting YOUR worktree:

```bash
cd ~/projects/veracity
./target/release/veracity-minimize-lib \
  -c /home/milnes/projects/APAS-VERUS-agent2 \
  -l /home/milnes/projects/APAS-VERUS-agent2/src/vstdplus \
  --no-lib-min --project APAS --chapter Chap35 -a
```

This will run for ~58 minutes. Do NOT interrupt it. It will modify files in your
worktree directly, marking unneeded asserts as:

```
// Veracity: UNNEEDED assert    assert(original_code);
```

Wait for the Phase 10 summary line before proceeding.

## Step 3 — Process markers

After veracity completes, return to your worktree and delete all UNNEEDED lines:

```python
# Run this Python snippet:
import os
chap = "/home/milnes/projects/APAS-VERUS-agent2/src/Chap35"
for fname in os.listdir(chap):
    if not fname.endswith(".rs"):
        continue
    path = os.path.join(chap, fname)
    lines = open(path).readlines()
    kept = [l for l in lines if "// Veracity: UNNEEDED assert" not in l]
    if len(kept) != len(lines):
        open(path, "w").writelines(kept)
        print(f"{fname}: {len(lines)-len(kept)} removed")
```

## Step 4 — Validate

```bash
scripts/validate.sh isolate Chap35
```

Must show 0 errors. If any errors appear, read them carefully — a NEEDED assert was
incorrectly removed. Restore the specific assert from the veracity log (the log is in
`analyses/veracity-minimize-lib.YYYYMMDD-HHMMSS.log` in your worktree).

Then run full validation:

```bash
scripts/validate.sh
```

## Step 5 — Report and push

Write report to `plans/agent2-round162-report.md`. Include:

- Table of files: asserts before/after, lines before/after
- Isolate elapsed before/after
- Any asserts that had to be restored

Then RCP: `git add -A && git commit` with message `R162 Agent 2: minimize Chap35 (-N asserts)`, then `git push`.
