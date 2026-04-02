# R138 Agent 3 — Validate + RTT + PTT after multi-agent merges. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r138-agent3-validate-report.md`

## Task

Three agents just merged significant changes into Chap38, Chap43, Chap52, plus
the TotalOrderBridge ripple across 10+ files. Run the full validation pipeline
and fix any issues.

## Steps

1. `scripts/validate.sh` — full validate. Report the verified count and any errors.
2. Fix any verification errors. Common issues after multi-agent merges:
   - Trigger warnings from new quantifiers
   - rlimit exceeded from changed proof structure
   - Type mismatches from signature changes (TotalOrderBridge)
3. `scripts/rtt.sh` — all runtime tests. Report pass count.
4. `scripts/ptt.sh` — all proof time tests. Report pass count.
5. Fix any test failures.

Run each step sequentially. Fix issues between steps.

## If everything passes

Report the counts and you're done. If there are issues, fix them and re-run.

## Rules

- Do NOT run validate, rtt, ptt in parallel — they compete for memory.
- Show full output of any errors.
- If a fix requires changing code, explain what and why.

## When done

RCP.
