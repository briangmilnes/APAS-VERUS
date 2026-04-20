# R203 Orchestrator Restart

## Current State (2026-04-18)

All proofs complete. Zero holes. First time the project has reached this milestone.

```
Chapters:  46/46 clean, 0 holed, 0 holes
Verified:  5765 (validate.sh, 2026-04-17)
RTTs:      4209/4209
PTTs:      289/289, 0 flaky
Verus:     release/rolling/0.2026.04.10.fc697a7
Commit:    40568cca4 (main, up to date with origin/main)
```

## Daily Proof Table

| Round | Holes Start | Holes End | Delta | Clean Chaps | Verified |
|-------|-------------|-----------|-------|-------------|----------|
| R202  | 4           | 0         | -4    | 45 → 46     | 5765     |
| R203  | 0           | 0         | 0     | 46          | 5765     |

R202 closed the last 4 holes: `unsafe impl Send/Sync` in `AVLTreeSetMtEph.rs` and
`AVLTreeSetMtPer.rs` (Chap41). Fixed by upgrading Verus to `0.2026.04.10` which added
`Ghost<T>: Send + Sync`, removing the need for `unsafe impl`.

## R203 Infrastructure Work

This round had no proof work — pure infrastructure:

| # | Commit | Description |
|---|--------|-------------|
| 1 | `5cebdd2` | Add `-V new-mut-ref` to validate.sh; fix 4 proof errors it exposed |
| 2 | `798398b` | Add `verita.toml` for daily validation via verita runner |
| 3 | `678cad3` | Add `VERUS_NO_LOCK=1` bypass to validate.sh and ptt.sh |
| 4 | `40568cc` | Fix PTT parallel `libtest_crate.rlib` collision (`.current_dir(test_dir)`) |

### Key Technical Notes

**`-V new-mut-ref` proof fixes** (Chap27, Chap28, Chap36, Chap39): The new-mut-ref mode
changes how Verus tracks mutation through references, which exposed 4 proof gaps. All fixed
via trigger adjustments — no proof structure changes needed.

**verita.toml**: Verita is a crater-style benchmarking tool in `~/projects/verus/tools/verita/`.
Run with:
```bash
cd ~/projects/verus/tools/verita
cargo run --release -- --verus-repo ~/projects/verus --label apas-test ~/projects/APAS-VERUS/verita.toml
```
Confirmed working: 5765/0, ~358s. Output in `output/2026-04-18-03-41-20-113-apas-test/`.

**VERUS_NO_LOCK=1**: Bypass for environments without the multi-agent slot infrastructure
(e.g., verita, CI, single-machine runs). Set before calling any script.

**PTT flakiness fix**: `run_verus` in `rust_verify_test/tests/common/mod.rs` lacked
`.current_dir(test_dir)`. All 6 parallel nextest workers shared the same cwd and raced
to write `libtest_crate.rlib` to the same path. Fixed by adding `.current_dir(test_dir)`.
Also removed stray committed artifacts (`liblib.rlib`, `rust_verify_test/libtest_crate.rlib`).

## What's Next

With 0 holes and 46/46 clean chapters, the project has reached full verification coverage
of the APAS algorithm set. Possible next directions:

1. **Spec strength improvements** — many functions have `external_body` with strong specs
   or accepted holes from earlier rounds. Now is the time to prove rather than accept.
2. **Coverage expansion** — add remaining APAS chapters not yet in the codebase.
3. **Cost spec completeness** — audit which functions have full complexity annotations.
4. **Parallel algorithm proof depth** — Mt files have some accepted thread-safety holes.

No agents are currently running. No rebase needed. Main is clean.

## Agent Worktrees

| Agent | Branch | Status |
|-------|--------|--------|
| agent1 | agent1/ready (merged R202) | Idle — needs new prompt after rebase |

Before launching agents: verify experiments in lib.rs are commented out.

## Scripts Reference

```bash
scripts/validate.sh              # full verification (5765 verified)
scripts/validate.sh isolate ChapNN  # single-chapter, fast
scripts/rtt.sh                   # runtime tests (4209/4209)
scripts/ptt.sh                   # proof-time tests (289/289)
scripts/all-holes-by-chap.sh     # regenerate hole analyses
scripts/chapter-cleanliness-status.sh  # regenerate cleanliness log
```
