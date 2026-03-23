# R59 Restart Plan — Post-Reboot Orchestration

## State at Shutdown

- **Main**: `8dff182ec` — pushed, 4484 verified, 1 pre-existing error (Chap47 loop invariant, Agent 1's target)
- **All 4 agents**: rebased on main, clean worktrees, force-pushed
- **R59 prompts**: committed in `plans/r59-agent{1-4}-prompt.md`
- **RTT**: 2610 passed (3 Chap47 test fixes committed)
- **PTT**: 147 passed

## Agent Assignments

| Agent | Worktree | Prompt | Summary |
|-------|----------|--------|---------|
| 1 | APAS-VERUS-agent1 | `plans/r59-agent1-prompt.md` | Chap47 triangular proofs (2 holes) + Chap26 ETSPMtEph (1 hole) |
| 2 | APAS-VERUS-agent2 | `plans/r59-agent2-prompt.md` | Chap53 capacity cascade (12 holes across GraphSearch + PQMin) |
| 3 | APAS-VERUS-agent3 | `plans/r59-agent3-prompt.md` | Chap43 mixed: RWLOCK capacity (2), reducer clone (2), select rank (2) |
| 4 | APAS-VERUS-agent4 | `plans/r59-agent4-prompt.md` | Stabilize 3 flaky Z3 proofs (StarContraction + OrderedSetStPer) |

## Launch Sequence

After reboot:

1. **Verify state** (optional sanity check):
   ```bash
   cd ~/projects/APAS-VERUS
   scripts/survey-agents.sh
   ```
   All 4 should show "clean" and "pushed".

2. **Launch agents** (one terminal per agent):
   ```bash
   # Agent 1
   cd ~/projects/APAS-VERUS-agent1
   claude -p "$(cat plans/r59-agent1-prompt.md)" --allowedTools 'Bash(*)'

   # Agent 2
   cd ~/projects/APAS-VERUS-agent2
   claude -p "$(cat plans/r59-agent2-prompt.md)" --allowedTools 'Bash(*)'

   # Agent 3
   cd ~/projects/APAS-VERUS-agent3
   claude -p "$(cat plans/r59-agent3-prompt.md)" --allowedTools 'Bash(*)'

   # Agent 4
   cd ~/projects/APAS-VERUS-agent4
   claude -p "$(cat plans/r59-agent4-prompt.md)" --allowedTools 'Bash(*)'
   ```

3. **Wait for agents to finish and push**. Each agent writes its report to
   `plans/agent{N}-round59-report.md` and pushes to `agent{N}/ready`.

4. **Merge on main** (orchestrator):
   ```bash
   cd ~/projects/APAS-VERUS
   scripts/survey-agents.sh          # Check who pushed
   scripts/merge-agent.sh agent4/ready   # Merge stability fixes first
   scripts/validate.sh               # Verify stability
   scripts/merge-agent.sh agent1/ready   # Then Chap47/Chap26
   scripts/validate.sh
   scripts/merge-agent.sh agent2/ready   # Then Chap53 capacity
   scripts/validate.sh
   scripts/merge-agent.sh agent3/ready   # Then Chap43 mixed
   scripts/validate.sh
   scripts/rtt.sh
   scripts/ptt.sh
   ```

5. **Post-merge**: Regenerate analyses, update daily proof table, write R59 summary.

## Merge Order Rationale

- **Agent 4 first**: stability fixes are zero-hole-impact but eliminate the flaky 1-error.
  All subsequent validates become deterministic.
- **Agent 1 second**: Chap47 proofs are self-contained, no cascades expected.
- **Agent 2 third**: Chap53 capacity cascade may add requires to callers in Chap57-59.
- **Agent 3 last**: Chap43 RWLOCK changes may cascade to other Mt files.

## Known Issues

- The 1 pre-existing error in Chap47:467 (loop invariant) is Agent 1's R59 target.
  If Agent 1 doesn't fix it, it remains flaky/stable.
- Agent 4's flaky proofs (StarContraction, OrderedSetStPer) may need multiple validate
  runs to confirm stability. The prompt requires 3 clean runs.
- Agent 3's "select rank" holes (OrderedSetStEph/StPer) are hard — may remain open.

## Current Hole Baseline

24 holes across 5 chapters (41 clean, 5 holed, 258 modules, 4484 verified).
