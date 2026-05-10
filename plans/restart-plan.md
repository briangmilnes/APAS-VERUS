// Copyright (c) 2026 All rights reserved. Brian G. Milnes

# Restart Plan — APAS-VERUS

**Generated:** 2026-05-04
**Last activity:** 2026-04-23 (validate/RTT/PTT logs); CLAUDE.md edits 2026-04-24.
**Last APAS-VERUS HEAD:** `aa31ddaa7` (CLAUDE.md: add Productivity Metrics rule)

## 1. State on disk

| # | Repo | Branch | HEAD | Status |
|---|---|---|---|---|
| 1 | `~/projects/APAS-VERUS` | main | `aa31ddaa7` | uncommitted: lectures/abstract.txt, lectures/build.sh; untracked veracity-count-* logs and slidesCMUCS.{pdf,pptx} |
| 2 | `~/projects/verus-lang-baseline` | `verita-apas-verus-config` | `db7a5ad8` (origin/main) | uncommitted: `tools/verita/run_configuration_all.toml` (apas-verus entry added; not yet committed/pushed) |
| 3 | `~/projects/verus` | rolling | tag `0.2026.04.20.8dcd677` | local mod `source/vstd/std_specs/borrow.rs` preserved |

## 2. Sanity-check before doing anything new

Run sequentially (CLAUDE.md rule: validate → RTT → PTT, never parallel, never tail). Logs are 11+ days old so reverify before believing them:

```bash
scripts/validate.sh
scripts/rtt.sh
scripts/ptt.sh
```

Expected (from R196 / 2026-04-11 cost.md): 5,674 verified, 3,776 RTT, 221 PTT, 0 errors. The seed-fix commit and any merges since may shift verified slightly.

If any step fails: read the log (`ls -t logs/<step>.*.log | head -1 | xargs cat`), do not re-run.

## 3. Outstanding items needing user approval

| # | Item | Where | Action needed |
|---|---|---|---|
| 1 | Commit Verita config addition | `verus-lang-baseline:verita-apas-verus-config` | `git add tools/verita/run_configuration_all.toml && git commit -m "verita: add apas-verus project" && git push -u origin verita-apas-verus-config` |
| 2 | Open PR to upstream | `briangmilnes/verus-lang:verita-apas-verus-config` → `verus-lang/verus:main` | `gh pr create` after step 1 |
| 3 | LOP0R snapshot | CLAUDE.md → Productivity Metrics, row 4 | User supplies a git commit hash (first complete draft of `src/`) OR a remembered raw count |
| 4 | Spend ($) for N and C | CLAUDE.md → Productivity Metrics, rows 5-9 | User supplies hourly rate OR direct dollar totals for APAS-AI and APAS-VERUS |
| 5 | Uncommitted lectures edits | `APAS-VERUS:main` | Decide: commit `lectures/abstract.txt`, `lectures/build.sh`, untracked PDFs/pptx, or revert |

## 4. Recently committed (FYI)

| # | Commit | Subject |
|---|---|---|
| 1 | `aa31ddaa7` | CLAUDE.md: add Productivity Metrics rule (N, C, R, LOP0R, LOP2R) |
| 2 | `50ce88089` | CLAUDE.md: add TIMESTAMP START / TIMESTAMP STOP rules |
| 3 | `fea6c074f` | TIMESTAMP START 2026-04-24T13:41:11Z (bundled prior session work: seed fix in `tests/Chap63/TestConnectivityMtEph.rs` 1111→789, time-by-gitlog scripts and analyses) |
| 4 | `3c6095e52` | lectures: add slidesMSR.pdf |
| 5 | `fa11c94d2` | rust 1.95.0 toolchain bump, Chap42 trigger fixes, `obeys_cmp_spec` rename |

## 5. Conventions in force (from memory + CLAUDE.md)

- TIMESTAMP / TIMESTAMP START / TIMESTAMP STOP: pre-authorized; format `<keyword> <ISO-8601 UTC>`. Use `date -u +%Y-%m-%dT%H:%M:%SZ`.
- "bridge" is a forbidden term in APAS-VERUS and trustd — use **trait specifications** for `#[verifier::external_trait_specification]` blocks.
- Authoritative line counts: `analyses/veracity-count-loc.log` (do not recount with `wc`).
- Authoritative timing: `analyses/time-by-gitlog-*-<TS>.log` and the merged `analyses/time-by-gitlog-merge-*-<TS>.log` files.
- Every TIMESTAMP commit captures a bundled diff — do not amend or squash.

## 6. Suggested order of operations

1. Sanity-check (§2). If clean → continue. If not → fix-forward, do not revert proof work without user approval.
2. Resolve item §3.5 (lectures edits) — quick, unblocks a clean working tree.
3. Resolve item §3.1 + §3.2 (Verita PR). Self-contained on `verus-lang-baseline`; doesn't touch APAS-VERUS.
4. Wait for user to supply LOP0R + spend (§3.3, §3.4) before computing N/C/R.
5. New round work begins from there.

## 7. Not in scope of restart

| # | Out of scope | Why |
|---|---|---|
| 1 | Rebasing agent worktrees | CLAUDE.md rule: only on user request |
| 2 | Running linters / formatters | CLAUDE.md rule |
| 3 | Re-running validate/RTT/PTT if §2 already passed | CLAUDE.md rule |
| 4 | Committing the Productivity Metrics table values | Inputs missing; would fabricate |
