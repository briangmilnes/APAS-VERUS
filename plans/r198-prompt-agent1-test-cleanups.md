# R198 Prompt — Agent 1: Cargo.toml test-entry cleanup + stale comment audit. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER modify `src/` to make a test pass.** If a test fails after
   reactivation, either fix the test, leave it commented out with the
   real reason, or STOP and report. Do not edit `src/`.
7. **NEVER touch `Example*.rs` or `Problem*.rs` test files.**

## Read all standards first.

## Context

`Cargo.toml` has **57 commented-out `[[test]]` entries**, many with
reasons like "module commented out in lib.rs", "API changed", or "uses
ordered_float (removed crate)". Agent1 in R197 already reactivated 6
Chap52 and 12 Chap28 entries whose disabling reasons were stale.
There's more to audit — and at least one known-stale comment:

**Known stale**: `TestKruskalStEph` and `TestPrimStEph` are both
commented with `# [[test]]  # uses ordered_float (removed crate)`.
Grep of the actual test files shows **no `ordered_float` or
`OrderedFloat` references**. The comment is wrong; these tests may
just need reactivation (or fixing for the current Kruskal/Prim API
after R196's rewiring).

## Goal

1. Walk every commented `[[test]]` entry in `Cargo.toml`.
2. Verify the stated reason against current repo state (`src/lib.rs`,
   imports in the test file, referenced modules).
3. For each entry, take one of three actions:
   - **Reactivate**: uncomment and run the test. If it passes, keep
     reactivated. If it fails, fix the test (in tests/ only, never
     src/) and retry.
   - **Update comment**: reason is actually valid but phrased wrong.
     Replace the stale reason with the real one.
   - **Leave commented**: reason is correct. Move on.
4. Document all decisions in a report table.

## Plan

### Step 1: Enumerate the 57 entries

```bash
grep -B0 -A2 "^# \[\[test\]\]" Cargo.toml
```

Build a table in `plans/r198-test-entry-audit.md`:

| # | Line | Name | Stated reason | Real state | Action |
|---|------|------|---------------|-------------|--------|

### Step 2: Verification loop per entry

For each entry, run these checks in order:

1. **Does the test file exist?**
   `ls tests/ChapNN/TestXYZ.rs` — if not, note and leave commented
   with "file does not exist".

2. **Does the source module referenced by the test exist in `src/`?**
   Read the first 20 lines of the test file (imports) and check each
   `use apas_verus::ChapNN::ModuleName::...;` against `src/lib.rs` and
   `src/ChapNN/ModuleName.rs`.

3. **Is the stated reason still true?**
   - "module commented out in lib.rs" → check `src/lib.rs`, grep for
     the `pub mod` entry; if it's active, the comment is stale.
   - "uses ordered_float (removed crate)" → grep the test file for
     `ordered_float`; if absent, stale.
   - "API changed: X renamed to Y" → grep the test for `X`; if absent,
     stale.
   - "uses apas_ai crate, no source module exists" → check
     `Cargo.toml` for an `apas_ai` dependency; if absent, the reason
     stands (legitimate skip).
   - "depends on Y (commented out)" → check Y's state; if Y is now
     active, this is also stale.
   - "experiments removed" → these are listed explicitly; leave them.

4. **If stale**, uncomment the entry and try the test:
   ```bash
   cargo test --test TestXYZ 2>&1 | tail -20
   ```
   Record the result:
   - **PASS** → reactivation successful.
   - **COMPILE FAIL** → the test uses an old API. Fix the imports /
     method calls in the test file. Do not fix in src/. Retry. If it
     still fails, either fix or re-comment with updated reason.
   - **RUNTIME FAIL** → This is a **bug found**. STOP, document it in
     a prominent "Bugs Found" section, and move on. Do not attempt to
     fix src/.

### Step 3: Specific known-stale targets

Agent1 R197 already showed these *were* stale — reconfirm and fix:

| # | Chap | Test | Stated reason | Probably actually... |
|---|------|------|---------------|---------------------|
| 1 | 65 | TestKruskalStEph | uses ordered_float (removed crate) | grep shows no such reference — fix comment or reactivate |
| 2 | 65 | TestPrimStEph | uses ordered_float (removed crate) | same |

**Note on Kruskal**: R196 rewired Kruskal to use
`UnionFindPCStEph`. The old `TestKruskalStEph.rs` may reference
`UnionFindStEph` (the retired module, now `UnionFindNoPCStEph`) or
older APIs. If reactivation requires rewriting the test to the new
Kruskal API, do so — this is a test-side fix, not a src fix. If the
test exercises behavior that the new API doesn't support, re-comment
with an updated reason.

### Step 4: Categorize remaining uncommentable entries

For entries where reactivation genuinely fails (src genuinely missing,
or API change makes the test unrepairable), **rewrite the comment** to
match reality. Example transformations:

- `# [[test]]  # module commented out in lib.rs` where the module IS
  commented out → leave, but the entry should ideally point to the
  specific cfg gate (if any).
- `# [[test]]  # API changed: out_neighbors renamed to n_plus` →
  apply the rename in the test file and reactivate.

### Step 5: Validation

After all reactivations:

```bash
scripts/rtt.sh
```

All tests must pass. If anything newly fails, back out the last
change. The net result should be more tests running, same pass rate
(100%).

```bash
scripts/validate.sh
scripts/ptt.sh
```

Both should be unchanged — this round doesn't touch src/ or
rust_verify_test/.

## Out of scope

- **Do not** modify `src/`.
- **Do not** reactivate tests for modules that genuinely don't exist
  (they'd fail to compile). Document them as "source gap" instead.
- **Do not** touch the experiments/ entries — those have a separate
  disposition (use verus-proof-time-testing project per Cargo.toml).
- **Do not** touch bench entries (`[[bench]]`) — agent2's R197 work
  already curated those.
- **Do not** overlap with agent3's R197 work on
  `TestEdgeSetGraphMtEph.rs` — leave the placeholder for agent3 to
  fill.

## Report

Write `plans/agent1-round198-report.md` with:

- Audit table (every commented entry + action taken).
- Count: total commented-out entries at start, reactivated, updated-comment, left-commented.
- New RTT count and wall time vs baseline (3838 after R197).
- Any **bugs found** (prominent section).
- Any tests that needed in-test fixes to reactivate (what was broken, what you fixed).
- Any source gaps surfaced (e.g., test says module Y is missing — Y is genuinely not in src/).

## RCP

```
git add -A
git commit -m "R198 Agent 1: Cargo.toml test-entry audit — N reactivated, M comments updated"
git push
```
