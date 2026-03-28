# R98 Agent 4 — Full validate + RTT + PTT + daily proof table, STEP 5

## Objective

1. Run full validate, RTT, PTT — confirm all green
2. Generate the daily proof table for rounds R88-R98
3. Commit the table to `plans/daily-proof-table-r98.md`

## Daily proof table format

```
| Round | Holes Start | Holes End | Delta | Clean Chaps | Verified | Notes |
```

Get historical data:
```bash
# Current
scripts/all-holes-by-chap.sh
head -1 analyses/chapter-cleanliness-status.log

# For history, check git log for round commits and their hole counts
git log --oneline --grep="R88\|R89\|R90\|R91\|R92\|R93\|R94\|R95\|R96\|R97\|R98" | head -20
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## STEP 5

## Report

Write `plans/agent4-r98-validate-report.md`.
