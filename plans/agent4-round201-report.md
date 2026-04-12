# Agent 4 — Round 201 Report

Task: APAS-AI quantitative snapshot for lecture comparison.
Branch: agent4/ready
Output: `lectures/quantitatives/apas-ai-snapshot-agent4.md`

---

## Step outcomes

### Rusticate build

Clean. `cargo build --release` completed in 11.86s.
Binary at `~/projects/rusticate/target/release/rusticate-count-loc`.
One unrelated warning (`is_test` field unused in `analyze_modules_ast.rs`); no errors.

### rusticate-count-loc headline totals

Run from `~/projects/APAS-AI`:

| # | Scope | Files | LOC |
|---|-------|-------|-----|
| 1 | `apas-ai/src/` | 238 | 45,485 |
| 2 | `apas-ai/benches/` | 171 | 13,890 |
| 3 | `apas-ai/tests/` | 246 | 55,223 |
| 4 | `experiments/` | 21 | 1,400 |
| 5 | **Total** | **676** | **115,998** |

No spec/proof/exec split (APAS-AI is pure Rust, no Verus).

### veracity-count-loc headline totals

Run from `~/projects/APAS-AI` (`-c -a`; scans `apas-ai/` only, not `experiments/`):

| # | Metric | Value |
|---|--------|-------|
| 1 | Files | 655 |
| 2 | Lines with comments | 114,598 |
| 3 | Lines without comments | 86,469 |
| 4 | Comment lines | 28,129 (24.5%) |
| 5 | Spec/proof/exec | 0/0/0 (no Verus) |

**Cross-check**: rusticate `apas-ai/` only = 45,485+13,890+55,223 = **114,598**. ✓ Exact match.

### APAS-AI git metadata

| # | Metric | Value |
|---|--------|-------|
| 1 | First commit | 2025-08-31 (Initial commit) |
| 2 | Last commit | 2025-11-26 (ALGORITHMS.md) |
| 3 | Calendar span | 88 days (~2.9 months) |
| 4 | Active dev span | 2025-08-31 – 2025-10-29 (~59 days, ~2.0 months) |
| 5 | Stray-fix tail | Nov 2025, 8 commits |
| 6 | Total commits | 347 |

Active/stray boundary: commit frequency drops from 4–55/day during Oct to 1–5/day in Nov,
with a 3-day gap (Oct 30–31) before resuming with README cleanup.

### File and chapter totals

| # | Metric | Value |
|---|--------|-------|
| 1 | Chapters (src dirs) | 42 |
| 2 | Src files | 238 |
| 3 | Test files | 246 |
| 4 | Bench files | 171 |

Largest chapter by src LOC: Chap37 (5,706 — AVL trees, BST variants).
Smallest: Chap11 (34 — FibonacciMtPer, single file).

### Key comparison row (APAS-AI vs APAS-VERUS)

| # | Metric | APAS-AI | APAS-VERUS | Ratio |
|---|--------|---------|-----------|-------|
| 1 | Src algorithm LOC | 45,485 | 186,223 | 4.1× |
| 2 | Total project LOC | 115,998 | 275,014 | 2.4× |
| 3 | Src files | 238 | 262 | 1.1× |
| 4 | Chapters | 42 | 44 | ~1.05× |
| 5 | Active dev (months) | ~2.0 | ~5.3 | 2.7× |
| 6 | Commits | 347 | 2,596 | 7.5× |

## Caveats encountered

- No blockers. All tools ran cleanly.
- veracity-count-loc does not scan `experiments/` when run from `apas-ai/`; the 1,400 LOC
  difference vs rusticate's total is accounted for by this.
- The Verus spec/proof/exec split produces all-zeros for APAS-AI (expected: no Verus).
- Active development end date is a heuristic based on commit-frequency drop; two stray
  src/ commits in Nov 2025 mean a conservative estimate would say Nov 3 as the true end,
  but the bulk of development clearly ended Oct 29.

## Files written

| # | File | Purpose |
|---|------|---------|
| 1 | `lectures/quantitatives/apas-ai-snapshot-agent4.md` | Main report |
| 2 | `lectures/quantitatives/raw/apas-ai-rusticate-count-loc-agent4.log` | Raw rusticate output |
| 3 | `lectures/quantitatives/raw/apas-ai-veracity-count-loc-agent4.log` | Raw veracity output |
| 4 | `lectures/quantitatives/raw/apas-ai-git-log-agent4.txt` | Full git log |
