# Orchestrator Handoff — R49 (restart due to HeapHelper threads)

## State at Handoff

- **Main**: commit `57a1ce7d8`, 34 holes, 4432 verified, 37 clean chapters
- **Agent 1 R48**: COMMITTED but NOT YET MERGED (2 commits on agent1/ready)
  - `6e72ca27e` R48 capacity-bounds refactor Chap41+43
  - `957881e11` R49 prompt (agent wrote its own next-round prompt)
  - Net: 38 → 38 holes (0 change), but quality improved — 2 algorithmic assumes replaced
  - Ready to merge immediately
- **Agents 2, 3**: Running R49 work (dirty files seen at last poll)
- **Agent 4**: Running R49 but idle at last poll

## First Actions

1. **Merge Agent 1 R48**: `scripts/merge-agent.sh agent1/ready`, resolve analysis conflicts, validate
2. **Do NOT rebase agents 2/3/4** — they're running R49
3. Restart watcher: `scripts/watch-agents.sh 300` (5-min polls)

## R49 Agents Running

| # | Agent | Target | Holes | Prompt |
|---|-------|--------|-------|--------|
| 1 | 1 | Self-assigned: Chap41+43 select (sortedness) | ~2 | plans/r49-agent1-prompt (in agent1 worktree) |
| 2 | 2 | Chap38 BST parallel | 7 | plans/r49-agent2-prompt.md |
| 3 | 3 | Chap39 Treap parallel | 7 | plans/r49-agent3-prompt.md |
| 4 | 4 | Chap62 StarContraction | 2 | plans/r49-agent4-prompt.md |

All agents rebased on `57a1ce7d8` at R49 start.

## Verus Upgrade Pending

Local verus: `76e69b81` (2026-03-16)
Upstream: `922ef8cd` (2026-03-20) — **21 commits behind**

Key new features:
- **IEEE floating point SMT theory** (`2c293264`, `5e1842f1`) — Z3 FP theory support!
  - `assume_ieee_float.rs` added
  - Float `as` cast specs
  - Could help Chap26 ETSP and Chap65 Kruskal/Prim float holes
- **Rust 1.94.0 support** (`a96bad0a`)
- **new-mut-ref fix** (`f04abf70`) — two-phase borrow issue

User noted Veria is validating the nightly. Rolling tags exist:
- `release/rolling/0.2026.03.20.922ef8c` (today)

**Do NOT upgrade while agents are running.** After R49 agents finish:
1. Clone to `~/projects/verus-rolling`
2. Build: `cd source && vargo build --release`
3. Validate APAS-VERUS against it
4. If clean, upgrade real `~/projects/verus`

## IronKV Discussion

User asked about verified-ironkv's hashtable at `~/projects/VerusCodebases/verified-ironkv/`.
Key finding: IronKV wraps std::HashMap with external_body — much weaker than APAS-VERUS's
explicit `spec_fn(Key) -> nat` approach. IronKV won't help with QuadProb quadratic residue
or call_hash_fn holes. User has Google Deep Research running on hash table number theory.

## Hole Inventory (34 total, after Agent 1 merge will stay 34)

| # | Chap | Holes | Agent | Notes |
|---|------|-------|-------|-------|
| 1 | 26 | 2 | — | ETSP float, skip until verus upgrade |
| 2 | 38 | 7 | 2 | BST parallel expose_internal chain |
| 3 | 39 | 7 | 3 | Treap parallel expose_internal chain |
| 4 | 41 | 5 | 1 | 1 real (union) + 4 Example (skip) |
| 5 | 43 | 5 | 1 | reduce_range, lemma_reducer_clone, select x2, domain |
| 6 | 45 | 1 | — | Example, skip per CLAUDE.md |
| 7 | 47 | 3 | — | clone_elem + call_hash_fn (irreducible) + QuadProb (needs number theory) |
| 8 | 62 | 2 | 4 | StarContraction St + Mt |
| 9 | 65 | 2 | — | Kruskal sort + Prim MST, float-adjacent |

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
| 8 | R48 | 38 | 34 | -4 | -158 | 37 | 4432 |
| 9 | R49 | 34 | ? | ? | ? | ? | ? |
