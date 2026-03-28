# R97 Agent 4 — Full validate + RTT + PTT clean run, STEP 5

## Objective

Run full validation, RTT, and PTT. Report results. Fix any regressions.

## Steps

1. `scripts/validate.sh` — must be 0 errors
2. `scripts/rtt.sh` — must be 0 failures
3. `scripts/ptt.sh` — must be 0 failures
4. If anything fails, investigate and fix

## Also

Check for any `(result:` returns that agent4 R96 missed:
```bash
grep -rn "(result:" src/Chap*/ --include="*.rs" | grep -v Chap43 | grep -v Chap52
```

If any remain in chapters not being worked by other agents, rename them.

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 5

## Report

Write `plans/agent4-r97-validate-report.md`.
