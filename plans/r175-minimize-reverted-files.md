# R175 — Minimize reverted files (fixed veracity). AFK.

## Assignment by agent

| Agent | Worktree | Chapter | Files | Testable |
|-------|----------|---------|-------|----------|
| 2 | APAS-VERUS-agent2 | Chap37 | BSTRBMtEph.rs | 60 |
| 3 | APAS-VERUS-agent3 | Chap43 | AugOrderedTableMtEph.rs, AugOrderedTableStEph.rs, OrderedTableMtEph.rs | 134 |
| 4 | APAS-VERUS-agent4 | Chap53 | PQMinStPer.rs | 17 |
| 5 | APAS-VERUS-agent5 | Chap57+59 | DijkstraStEph{F64,U64}.rs, Johnson{Mt,St}Eph{F64,I64}.rs | 34 |

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**

## What to do

Read all standards first. Then:

1. Determine your agent number from your worktree path (`pwd`).
2. Run `veracity-minimize-proofs` on your assigned files ONLY:

**Agent 2:**
```bash
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c ~/projects/APAS-VERUS-agent2 \
  --project APAS --chapter Chap37 \
  --file BSTRBMtEph.rs \
  --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
```

**Agent 3:**
```bash
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c ~/projects/APAS-VERUS-agent3 \
  --project APAS --chapter Chap43 \
  --file AugOrderedTableMtEph.rs --file AugOrderedTableStEph.rs --file OrderedTableMtEph.rs \
  --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
```

**Agent 4:**
```bash
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c ~/projects/APAS-VERUS-agent4 \
  --project APAS --chapter Chap53 \
  --file PQMinStPer.rs \
  --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
```

**Agent 5:**
```bash
~/projects/veracity/target/release/veracity-minimize-proofs \
  -c ~/projects/APAS-VERUS-agent5 \
  --project APAS --chapter Chap57,Chap59 \
  --file DijkstraStEphF64.rs --file DijkstraStEphU64.rs \
  --file JohnsonMtEphF64.rs --file JohnsonStEphF64.rs \
  --file JohnsonMtEphI64.rs --file JohnsonStEphI64.rs \
  --no-lib-min --fresh --danger \
  --max-incremental 0.00 --max-memory-increase 0.00
```

3. Wait for it to finish.
4. Commit the log and source changes.
5. Push.

## Context

These files were minimized in R170 but reverted because veracity's proof block
detector had a marker-shift bug that corrupted runtime behavior (30 RTT failures).
Veracity has been rebuilt with the fix. This run tests whether the fix works.

## Validation

After minimize completes, validate your chapter:
```bash
scripts/validate.sh isolate ChapNN
```

## RCP

```bash
git add -A && git commit -m "R175 Agent N: minimize reverted files ChapNN"
git push
```
