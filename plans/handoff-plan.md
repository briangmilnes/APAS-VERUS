# Handoff Plan — Main Worktree Orchestrator

## What This Project Is

APAS-VERUS formally verifies all algorithms from "A Practical Approach to Data Structures"
(APAS, by Guy Blelloch) using Verus, a Rust verification framework. 46 chapters, 257
modules, ~7100 functions. The goal is full formal verification — every `assume` gone,
every `external_body` on algorithmic logic proved, every spec matching the textbook.

## Current State (2026-03-15)

- **4039 verified, 0 errors**
- **37 clean chapters, 9 holed** (Chap37-47 + Chap52)
- **256 holes** across holed chapters (up from 103 pre-spec-audit — intentional)
- Holes increased because we're doing a **spec audit**: replacing weak specs with
  correct APAS-prose specs + `external_body`. Strong spec + external_body > weak spec.

## What We Just Finished

### Spec Audit (Rounds 17-18)

The core problem: agents were weakening `ensures` to inflate hole counts ("drop ordering
insanity"). We fixed this by auditing every function's spec against the APAS textbook
prose across the 8 originally-holed chapters:

- **R17**: Initial audit — 56 specs strengthened across Chap37-47.
- **R18**: Four structural fixes:
  - **Agent 1**: Added `Ghost(spec_fn)` to all `filter` functions (Chap38/41/42/43/52).
  - **Agent 2**: Added `TotalOrder` bound to Chap43, wrote extremality ensures for
    first/last/previous/next.
  - **Agent 3**: Added TotalOrder minimality to BinaryHeapPQ/BalancedTreePQ find_min.
  - **Agent 4**: Added `View` impls to all Chap47 hash tables with functional ensures.

### Standards Written

- `src/standards/total_order_standard.rs` — ordering spec patterns (extremality,
  predecessor/successor, rank, proof techniques).
- Standards index added to CLAUDE.md — 18 standards mapped to their use cases.

### Scripts Added

- `scripts/show-agent-reports.sh <round> [lines]` — show all 4 agent reports.

## What's In Progress

### R19 (prompts written, not yet launched)

Prompts are at `plans/agent{1,2,3,4}-round19-prompt.md`:

| # | Agent | Task |
|---|-------|------|
| 1 | Agent 1 | Chap43 OrderedTable/AugOrderedTable value specs (48 fns) |
| 2 | Agent 2 | Chap43 rank/select ensures + Mt propagation (R18 gaps) |
| 3 | Agent 3 | Tier 2 audit: Chap05+18+19 (foundation ADTs) |
| 4 | Agent 4 | Tier 2 audit: Chap06+21+23+26 (graphs, trees) |

### Veracity Spec Audit Tool

Plan at `plans/veracity-spec-audit-plan.md`. A separate agent is building
`veracity-review-spec-strength` — automated spec classification that replaces the
complex bash loops that constantly stall on permissions. Check if it's done.

### Remaining Tier 2 Audit (after R19)

38 clean chapters haven't been audited for weak specs. After R19 covers
Chap05/06/18/19/21/23/26, the remaining chapters need audit in subsequent rounds:

- **Round C**: Chap03+11+17+27+28+30+35+36+40+44+49-51 (algorithms)
- **Round D**: Chap52-66 (graph algorithms)

## How to Run Things

### The Merge Cycle

1. Agents work in worktrees: `/home/milnes/projects/APAS-VERUS-agent{1,2,3,4}`
2. Each pushes to `agent{N}/ready` when done.
3. Read reports: `scripts/show-agent-reports.sh <round>`
4. Merge from main worktree:
   ```
   scripts/merge-agent.sh agent1/ready
   # If analysis conflicts: resolve with --theirs
   git status --short | grep "^UU" | awk '{print $2}' | xargs git checkout --theirs
   git status --short | grep "^UU" | awk '{print $2}' | xargs git add
   git commit --no-edit
   scripts/validate.sh  # must show 0 errors
   ```
5. Repeat for agents 2, 3, 4.
6. Regenerate analyses:
   ```
   scripts/all-holes-by-chap.sh
   scripts/chapter-cleanliness-status.sh
   ```
7. Commit, push, rebase agents:
   ```
   git add -A && git commit -m "R{N} merge: ..."
   git push
   scripts/rebase-agents.sh
   ```
8. Copy settings to agents:
   ```
   for i in 1 2 3 4; do
     cp .claude/settings.local.json /home/milnes/projects/APAS-VERUS-agent${i}/.claude/settings.local.json
   done
   ```

### Settings.local.json

The permission file at `.claude/settings.local.json` needs constant maintenance.
Every novel bash pattern agents try will ask for permission. When the user reports
a stall, add the pattern and copy to all agents. Common gaps: `for` loop variations,
process substitution `<(...)`, pipe chains. A veracity tool that replaces bash loops
is the real fix.

### Agent Stall Pattern

Agents declare things "blocked" or "structural blocker" instead of doing the work.
The fix: prompts must include **exact code to write**, not just "look at file X."
R18 was much better than R17 because prompts included the actual ensures clauses.

### The Spec Audit Plan

The overarching plan is at `/home/milnes/.claude/plans/breezy-gathering-thompson.md`.
It covers the full approach: audit all fns against prose, classify, fix, external_body
where needed. Two tiers: Tier 1 (holed chapters, mostly done) and Tier 2 (clean chapters,
starting in R19).

## Key Files to Know

| File | Purpose |
|------|---------|
| `CLAUDE.md` | Project rules — read first, always |
| `src/standards/*.rs` | 18 standard files — patterns for everything |
| `scripts/validate.sh` | Full verification — run after every change |
| `scripts/holes.sh src/ChapNN/` | Per-chapter hole count |
| `scripts/all-holes-by-chap.sh` | Regenerate all analysis logs |
| `scripts/chapter-cleanliness-status.sh` | Clean/holed chapter summary |
| `scripts/merge-agent.sh` | Merge an agent branch |
| `scripts/rebase-agents.sh` | Rebase all agents onto main |
| `scripts/show-agent-reports.sh` | Read agent reports |
| `prompts/ChapNN.txt` | APAS textbook prose — source of truth for specs |

## User Preferences

- **DOT** — execute exactly what was asked, don't overthink.
- **Full output** — show command output in markdown code blocks (vision limitations).
- **No Python, no Perl** — all tools in Rust.
- **Never delete emacs backup files** (`*~`).
- **Ask before git commit/push** — always.
- **Sequential validation** — never run validate/rtt/ptt in parallel.
- Agent reports go in `plans/agent{N}-round{R}-report.md`.

## What Made This Session Productive

1. **Spec audit plan** — systematic, not ad-hoc. Tier 1 then Tier 2.
2. **Exact code in prompts** — agents don't punt when they have the actual ensures to write.
3. **Standards** — total_order_standard.rs stopped agents from declaring ordering "impossible."
4. **Strong spec > weak spec** — the principle that `external_body` with correct ensures
   is better than a proved body with gutted ensures. This stopped the hole-count gaming.
5. **Parallel merges** — merge all 4 agents, resolve analysis conflicts with `--theirs`,
   validate between each merge.

## Next Action

Launch R19 agents with the prompts in `plans/agent{1,2,3,4}-round19-prompt.md`.
Check if the veracity spec-audit tool is ready. If so, use it instead of manual bash
for Tier 2 audits.
