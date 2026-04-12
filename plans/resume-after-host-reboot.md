# Resume Prompt — after host reboot (post-validate thrashing)

Drop this into a fresh Claude session in
`/home/milnes/projects/APAS-VERUS` to pick up where we stopped.

## Session context

Work pipeline at the moment of pause:

1. **R201 agent1 (iterator cleanup) and agent2 (iterator expansion)
   merged** into main by agent2-acting-as-orchestrator.
2. **Main pushed** to `origin/main` at `aecb25f76` (check with
   `git log --oneline -3`).
3. **Full `scripts/validate.sh` timed out** at its hardcoded
   300-second limit — the iterator work pushed the crate past the
   5-minute mark. **Peak RSS ~27.8 GB, min free 143 MB** — essentially
   at OOM. This is what "thrashed" the host.
4. Fell back to per-chapter isolate on the 4 src-changed chapters
   (37, 41, 42, 43). Progress at the moment of STOP:
   - `validate isolate Chap37` → **1,981 verified, 0 errors, 15 s** ✓
   - `validate isolate Chap41` — finished but result not inspected.
   - `validate isolate Chap42` — **NOT RUN**.
   - `validate isolate Chap43` — **NOT RUN**.
5. **RTT, PTT, holes report** — all pending after isolate passes.

Concurrent-validate incident root cause: **agent4 was accidentally
given an orchestrator prompt**; the user killed it. Agent2 earlier
also played orchestrator and kept auto-spawning validates. With all
of those silenced, the host can proceed on its own.

## R201 numbers (sources of truth so we don't re-derive)

From the individual agent reports (in their plans/):

| Agent | Report | validate | RTT | PTT |
|---|---|---:|---:|---:|
| agent1 | `plans/agent1-round201-report.md` | 5,728 | 4,209 | 265 |
| agent2 | `plans/agent2-round201-report.md` | 5,764 | — | 261 |

Expected post-merge numbers (both landed on main):

- validate: ~5,764 (agent2 added new src, agent1 added tests only)
- RTT: ~4,209 (agent1 added one BSTTreap delete test)
- PTT: **~289** (237 baseline + 28 from agent1 + 24 from agent2)

## Outstanding items

1. Finish isolate validate on Chap41/42/43 (Chap37 already done).
2. Run `scripts/rtt.sh` — should be 4,209.
3. Run `scripts/ptt.sh` — should be 289.
4. Run `scripts/all-holes-by-chap.sh` for the holes report.
5. APAS-AI quantitative snapshots (agent3 + agent4 worktrees) stay
   separate per user decision — do NOT merge into main.
6. Optional R202: 12 iterator files still uncovered after R201
   (Chap37 BST non-Mt variants, Chap43 AugOrderedTable variants,
   Chap52 AdjTableGraph/EdgeSetGraph variants). See
   `plans/r201-prompt-agent2-iterator-expansion.md` for the list.

## Memory posture

Full `scripts/validate.sh` OOMs on the current host (32 GB RAM).
**Use isolate mode** for iterative work. If you must run full
validate, either:

- Bump the 300s timeout in `scripts/validate.sh` (user's call — it's
  project infrastructure), OR
- Close the browser/other large processes first and ensure no other
  Claude sessions are mid-task.

Currently zero rust_verify / z3 / validate processes should be
running. Verify with:

```bash
ps aux | grep -E "rust_verify|z3 -smt|validate\.sh" | grep -v grep
```

If that's empty and `free -h` shows ≥ 20 GiB available, you're
clear to run isolate. For full validate, wait for `free -h` to
show ≥ 28 GiB.

## Other Claude sessions that may be active

At the moment of STOP, these had task-output directories:

- `APAS-VERUS-agent3` — APAS-AI snapshot done, idle
- `APAS-VERUS-agent4` — orchestrator prompt killed by user, idle
- `veracity` — separate project, don't touch
- `APAS-VERUS-agent1` / `-agent2` — R201 reports filed, idle

Before running anything heavy, confirm they are all quiescent:

```bash
for d in APAS-VERUS-agent1 APAS-VERUS-agent2 APAS-VERUS-agent3 APAS-VERUS-agent4; do
  echo "== $d =="
  cd /home/milnes/projects/$d 2>/dev/null && git status --short | head
done
cd /home/milnes/projects/APAS-VERUS
```

## What NOT to do

- Don't run full `scripts/validate.sh` until either the 300s
  timeout is bumped OR the user explicitly confirms they want to
  spend the RAM.
- Don't rebase agents 1/2/3/4 — all already rebased onto
  `aecb25f76`. Rebasing again is pointless.
- Don't merge APAS-AI data. User explicitly asked to keep that
  separate.
- Don't try to fold in R202 iterator expansion until the user
  decides whether they want it.

## Immediate next action when resumed

```bash
# 1. Confirm clean host
ps aux | grep -E "rust_verify|z3 -smt|validate\.sh" | grep -v grep
free -h | head -2

# 2. Pick up where we left off
scripts/validate.sh isolate Chap41   # check/rerun
scripts/validate.sh isolate Chap42
scripts/validate.sh isolate Chap43

# 3. Full tests
scripts/rtt.sh
scripts/ptt.sh

# 4. Holes report
scripts/all-holes-by-chap.sh
cat analyses/chapter-cleanliness-status.log | head -3
```

Then tell the user the numbers and ask whether to attempt full
validate (risky) or leave isolate-per-chapter as the coverage claim.
