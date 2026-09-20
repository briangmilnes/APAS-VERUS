# r206 — Verus upgrade restart plan

Verus is now the prebuilt release `0.2026.09.13.671956e` unpacked flat in
`~/projects/verus` (binary, `z3`, `vstd.vir`, rlibs). Rust 1.98.1 is installed.
All APAS-VERUS scripts and Cargo files already point at the new layout
(uncommitted). A standalone Verus program verifies and compiles.

## Steps

1. Whitelist the verus dir (one shell command, Claude cannot do it):
   `mkdir -p ~/projects/verus/.claude && cp plans/verus-dir-settings.local.json ~/projects/verus/.claude/settings.local.json`
2. `source ~/.bashrc`, restart Claude.
3. Commit the path changes as-is: `git add -A && git commit -m "Upgrade to verus 0.2026.09.13 prebuilt release"`.
4. `scripts/validate.sh isolate Chap05` — smoke test the wiring on a small chapter. Read the log.
5. `scripts/validate.sh` — full run. Expect breakage from a year of vstd changes. Read the log, do not filter it.
6. Triage errors by kind (renamed vstd lemmas, changed trigger rules, new syntax) and fix chapter by chapter, isolate mode.
7. `scripts/rtt.sh`, then `scripts/ptt.sh`. Fix in that order.
8. Regenerate analyses, commit, push.

## Rollback

Every change is on disk only. `git checkout -- .` restores the repo; the bashrc
backup is in the session scratchpad.
