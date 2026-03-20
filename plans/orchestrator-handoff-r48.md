# Orchestrator Handoff — R48

## State at Handoff

- **Main**: commit `13d060626`, 38 holes, 4419 verified (4429 with new standard), 36 clean chapters
- **New standard**: `capacity_bounds_standard.rs` written, validates, registered in lib.rs and CLAUDE.md. NOT YET COMMITTED.
- **watch-agents.sh**: New script in scripts/, also uncommitted.

## Uncommitted Changes on Main

1. `src/standards/capacity_bounds_standard.rs` — new standard (verified)
2. `src/lib.rs` — added `pub mod capacity_bounds_standard;`
3. `CLAUDE.md` — added standard #22 to index
4. `scripts/watch-agents.sh` — agent commit watcher script

**First action: commit these, push, do NOT rebase agents (they're running).**

## R48 Agents Running

| # | Agent | Target | Holes | Prompt |
|---|-------|--------|-------|--------|
| 1 | 1 | Chap41 (2 real) + Chap43 (4) | 6 | plans/r48-agent1-prompt.md |
| 2 | 2 | Chap59 (1) + Chap62 (2) + Chap65 (2) | 5 | plans/r48-agent2-prompt.md |
| 3 | 3 | Chap47 (4) | 4 | plans/r48-agent3-prompt.md |
| 4 | 4 | Chap26 (4) | 4 | plans/r48-agent4-prompt.md |

All rebased on `13d060626`, worktrees clean at launch.

## When Agents Finish

1. Run `scripts/watch-agents.sh` to detect commits (polls every 30 min)
2. Read each agent's report: `plans/agentN-round48-report.md`
3. Merge order: smallest diff first (usually by hole count)
4. Each merge: `scripts/merge-agent.sh agentN/ready`, resolve analysis conflicts with `scripts/resolve-analysis-merge.sh`, validate
5. After all merges: regenerate analyses, commit, push
6. Rebase agents only when user says go

## Hole Inventory (38 total)

| # | Chap | Holes | Agent | Notes |
|---|------|-------|-------|-------|
| 1 | 26 | 4 | 4 | ETSP float distance axioms |
| 2 | 38 | 7 | — | RwLock restructure needed (Arc antipattern) |
| 3 | 39 | 7 | — | Same as Chap38 |
| 4 | 41 | 6 | 1 | 2 real (usize::MAX assumes) + 4 Example (skip) |
| 5 | 43 | 4 | 1 | Blocked by Chap41; reduce_range + select |
| 6 | 45 | 1 | — | Example45_2, skip per CLAUDE.md |
| 7 | 47 | 4 | 3 | Double/quad probe + clone_elem + call_hash_fn |
| 8 | 59 | 1 | 2 | parallel_dijkstra_all |
| 9 | 62 | 2 | 2 | star_contract St + Mt |
| 10 | 65 | 2 | 2 | sort_edges_by_weight + prim_mst |

## Key Context

- **sort_edges_by_weight is NOT irreducible** — user corrected this. The external_body can have strong ensures (sorted + multiset preserved). Agent 2 should push on it.
- **Chap38/39 need RwLock restructure** — 14 holes from Arc<RwLock> antipattern. Not assigned this round. Big opportunity for R49.
- **capacity_bounds_standard.rs** — new standard says capacity bounds go in requires, not assumes. Directly addresses Chap41 holes.
- **User noted agents should read standards before coding**, especially arc_usage_standard.rs. All R48 prompts include REQUIRED READING.

## Daily Proof Table

| # | Round | Holes Start | Holes End | Delta | Cumulative | Clean | Verified |
|---|-------|-------------|-----------|-------|------------|-------|----------|
| 1 | R41 | 192 | 153 | -39 | -39 | 30 | 4320 |
| 2 | R42 | 153 | 139 | -14 | -53 | 30 | 4348 |
| 3 | R43 | 139 | 125 | -14 | -67 | 30 | 4366 |
| 4 | R44 | 125 | 99 | -26 | -93 | 30 | 4388 |
| 5 | R45 | 99 | 69 | -30 | -123 | 32 | 4396 |
| 6 | R46 | 69 | 43 | -26 | -149 | 36 | 4413 |
| 7 | R47 | 43 | 38 | -5 | -154 | 36 | 4419 |
| 8 | R48 | 38 | ? | ? | ? | ? | ? |
