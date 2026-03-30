# Relaunch Plan — 2026-03-30

## Current state (main at `94bc1e3c6`)

- **5433 verified, 0 errors, 3083 RTT, 157 PTT**
- **5 holes remaining** (was 7 at start of session)
- All committed and pushed to origin/main

## Running agents

| Agent | Round | Task | Branch | Status |
|-------|-------|------|--------|--------|
| 1 | R109 | Prove `union` in UnionFindStEph.rs (Chap65) | agent1/ready | Running, has unstaged changes |
| 2 | R110 | Write 22 iterator PTTs + running `ptt.sh` to fix bugs | agent2/ready | Running |
| 3 | R110 | Prove map_values assume in AVLTreeSeqMtPer.rs (Chap37) | agent3/ready | Ready, needs launch. Prompt: `plans/r110-agent3-avltreeseq-map-values-prompt.md` |
| 4 | — | Killed (Agent 3 did its work) | agent4/ready | Dead |

## 5 remaining holes

| # | Chap | File | Hole | Who |
|---|------|------|------|-----|
| 1 | 37 | AVLTreeSeqMtPer.rs:687 | assume — map_values+push | Agent 3 |
| 2 | 65 | UnionFindStEph.rs:1167 | external_body — union_merge | Agent 1 (proved on branch, not merged) |
| 3 | 65 | UnionFindStEph.rs:1404 | external_body — union | Agent 1 (working) |
| 4 | 65 | KruskalStEph.rs:40 | opaque — uf_wf_opaque | Blocked by Agent 1 |
| 5 | 65 | KruskalStEph.rs:49 | external_body | Blocked by Agent 1 |

## Veracity work pending

- `plans/veracity-wf-detection-improvements.md` — DONE by veracity agent
- `plans/veracity-compare-par-mut-generic-bounds-false-positives.md` — DONE by veracity agent
- `plans/veracity-compare-par-mut-return-type-false-positives.md` — DONE by veracity agent

## What to do on relaunch

1. Check Agent 1 and 2 status: `git log --oneline main..agent1/ready` and `agent2/ready`
2. Launch Agent 3 on map_values prompt if not already running
3. Merge any finished agents, validate, commit, push, rebase

## Key files

- Prompts: `plans/r109-agent1-union-prompt.md`, `plans/r110-agent2-iterator-ptts-prompt.md`, `plans/r110-agent3-avltreeseq-map-values-prompt.md`
- Standards: `src/standards/iterator_ptt_standard.rs` (new this session)
- Daily proof table: 5 holes, 43 clean chapters, 3 dirty (Chap37, Chap65 x2)

## Abbreviation added this session

- **LITL** = Look In The Log
