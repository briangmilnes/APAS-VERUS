# R168 Orchestrator Briefing — Agent7 Findings

## 1. Chap65 Uncommented

Chap65 is now uncommented in lib.rs and compiling. Full validate: 5634 verified, 0 errors.
3776 RTT, 221 PTT. PrimStEph `prim_mst` proved. UnionFindStEph has 11 new external_body
holes (7 root cause, 4 downstream) — these are draft proofs that were never compiled,
not regressions. See `plans/agent7-round168-report.md` for details.

## 2. Minimize-Proofs Agents 1-6: DO NOT MERGE SOURCE CHANGES

All 6 agents finished. **Their source changes should not be merged.**

### Why

The minimize-proofs tool lost its timing regression guard at some point during R167
script rewrites. The tool was supposed to compare before/after elapsed time and reject
removals that increased verification time. That check was dropped when the scripts were
rewritten for 6-way parallel agents. Without it, the tool happily removed assertions
that were helping Z3, as long as verification still passed.

### Evidence — Per-Agent Per-Chapter Timing (first isolate → last isolate)

| # | Agent | Chap | First | Last | Delta | Status |
|---|-------|------|-------|------|-------|--------|
| 1 | 1 | 02 | 4s | 20s | **+16s** | clean |
| 2 | 1 | 11 | 4s | 5s | +1s | clean |
| 3 | 1 | 26 | 18s | 18s | 0s | clean |
| 4 | 1 | 45 | 19s | 18s | -1s | **ERRORS** |
| 5 | 2 | 44 | 26s | 37s | **+11s** | clean |
| 6 | 2 | 45 | 19s | 18s | -1s | **ERRORS** |
| 7 | 2 | 50 | 4s | 5s | +1s | clean |
| 8 | 2 | 54 | 11s | 11s | 0s | clean |
| 9 | 4 | 41 | 23s | 34s | **+11s** | clean |
| 10 | 4 | 43 | 29s | 31s | +2s | clean |
| 11 | 4 | 44 | 26s | 34s | **+8s** | clean |
| 12 | 4 | 45 | 19s | 26s | +7s | **ERRORS** |
| 13 | 5 | 41 | 23s | 19s | -4s | clean |
| 14 | 5 | 52 | 37s | 28s | -9s | clean |
| 15 | 5 | 53 | 27s | 19s | -8s | clean |
| 16 | 5 | 54 | 11s | 8s | -3s | clean |
| 17 | 6 | 45 | 19s | 31s | **+12s** | clean |
| 18 | 6 | 55 | 27s | 21s | -6s | clean |
| 19 | 6 | 56 | 7s | 12s | +5s | **ERRORS** |

Agent5 consistently improved. Agents 1, 2, 4, 6 have chapters that got significantly
worse. Agent4 and agent6 ended with outright verification failures (rlimit on
AVLTreeSeqMtPer.rs:389 rotate_right; postcondition failures in ArraySeqStEph.rs:162,170).

### Each agent ran ~10,000 isolate validations

| Agent | Runs | Chapters Completed | Status |
|-------|------|--------------------|--------|
| 1 | 10,114 | Chap26,30,61,63,64 | clean but Chap02 +16s |
| 2 | 10,126 | Chap44,50,54 | clean but Chap44 +11s |
| 3 | — | (revert/marking only) | pushed |
| 4 | 10,529 | Chap43,44 | **rlimit failure** |
| 5 | 10,903 | Chap52,53,54 | clean, all improved |
| 6 | 10,843 | Chap55,56,57 | **postcondition failure** |

### Root Cause

The minimize-proofs tool evolved through iterative prompting without a durable spec.
The timing regression guard existed in an earlier version but was not carried forward
when the scripts were rewritten for R167's 6-way agent split. No spec document said
"MUST compare before/after timing" so the requirement didn't survive refactoring.

## 3. Dirty Changes on Main

My commit `1f5aeaef2` accidentally included uncommitted changes from the main working
tree that belonged to agent2's minimize work: Chap44, Chap50, Chap54, and BFS files
(Chap54). These should be reverted from that commit since we're not taking minimize
source changes.

## 4. Uncommitted Script Edits on Main

I started editing `validate.sh`, `rtt.sh`, `ptt.sh` to add agent name detection in
log filenames (e.g., `validate.agent3.20260409-120000.log`). The pattern:

```bash
AGENT_TAG=""
if [[ "$PROJECT_ROOT" =~ -agent([0-9]+)$ ]]; then
    AGENT_TAG=".agent${BASH_REMATCH[1]}"
fi
LOGFILE="$LOGDIR/validate${AGENT_TAG}.$(date +%Y%m%d-%H%M%S).log"
```

These edits are uncommitted on main. They were done from agent7's context (wrong place).
The orchestrator should review and commit them, or redo them. `profile.sh` also needs
the same treatment.

## 5. Recommended Actions

1. **Revert** the Chap44/50/54 source changes from commit `1f5aeaef2` on main.
2. **Do not merge** agent 1-6 source branches. The minimize work is tainted by the
   missing timing guard.
3. **Copy only logs** from agents to main (with agent name in path):
   - `/tmp/r167-agent{N}*.log` → `logs/agent{N}/`
   - Agent-specific `analyses/veracity-minimize-proofs.APAS-VERUS-agent{N}.*` logs
   - Skip the ~10,000 ephemeral validate/rtt/ptt logs per agent
4. **Fix the timing guard** in veracity's minimize-proofs tool before the next run.
   Write a spec for the tool that includes: "reject any removal that increases
   elapsed time by more than N% or M seconds."
5. **Finish the agent-name-in-logs script fix** from the orchestrator (validate.sh,
   rtt.sh, ptt.sh, profile.sh).
6. **Agent5's changes might be salvageable** — it's the only agent where every chapter
   got faster. But verify independently before merging.
