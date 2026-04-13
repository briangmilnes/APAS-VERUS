# R202 Agent 1 Report — Verus 0.2026.04.10 Upgrade + Close 4 Send/Sync Holes

## Summary

Upgraded Verus from `ff454ab0f` (0.2026.03.24) to `release/rolling/0.2026.04.10.fc697a7`
(77 commits) and closed the final 4 proof holes in Chap41 that the upgrade unlocked.

**Total holes: 0** (down from 4). Full codebase clean.

## Metrics

| Metric         | Before R202 | After R202 | Delta   |
|----------------|-------------|------------|---------|
| Verified       | 5598        | 5765       | +167    |
| Holes          | 4           | 0          | -4      |
| RTT tests      | 4209 pass   | 4209 pass  | 0       |
| PTT tests      | 289 pass    | 289 pass   | 0       |
| Clean chapters | 46          | 46         | 0       |
| Holed chapters | 0           | 0          | 0       |

Verus commit: `ff454ab0f` → `fc697a7f4` (tag `release/rolling/0.2026.04.10.fc697a7`).

## Chap41 Holes Closed

Removed 4 `unsafe impl Send/Sync` blocks. These existed because `Ghost<T>` did not
implement `Send`/`Sync` in vstd; Verus PR #2287 (commit `16fa185f3`) added safe impls,
so the workaround is no longer needed.

| # | Chap | File                  | Line (before) | Kind                      |
|---|------|-----------------------|---------------|---------------------------|
| 1 | 41   | AVLTreeSetMtEph.rs    | 681           | `unsafe impl Send`        |
| 2 | 41   | AVLTreeSetMtEph.rs    | 682           | `unsafe impl Sync`        |
| 3 | 41   | AVLTreeSetMtPer.rs    | 739           | `unsafe impl Send`        |
| 4 | 41   | AVLTreeSetMtPer.rs    | 740           | `unsafe impl Sync`        |

Rust now auto-derives `Send`/`Sync` for `AVLTreeSetMtEph` and `AVLTreeSetMtPer` because
all their ghost fields (`Ghost<T>`) are Send+Sync.

## Upgrade Breakage + Fixes

The 77 commits introduced 5 regressions in the APAS-VERUS crate. All fixed forward
(no reverts, no holes added).

| # | Chap | File                        | Error                                      | Fix                                                   |
|---|------|-----------------------------|--------------------------------------------|-------------------------------------------------------|
| 1 | 43   | OrderedSetMtEph.rs          | `accept` not in scope (pre-existing R201)  | Added `use crate::vstdplus::accept::accept;`          |
| 2 | 39   | BSTParaTreapMtEph.rs:608    | assertion fail — `l@ != rkv` via cross-ord | Added explicit `assert(right@.contains(rkv))` + `assert(l.cmp_spec(&r_key) == Less)` inside by-block |
| 3 | 39   | BSTParaTreapMtEph.rs:646    | assertion fail — srv/rrv cross-ordering    | Added `assert(left@.contains(s@))` + `assert(right@.contains(o@))` inside by-block |
| 4 | 28   | MaxContigSubSumOptMtEph.rs  | rlimit — matching loop on spec_prefix_sum  | Tightened trigger from 1-term to 2-term multi-trigger (invariant lines 138-148) |
| 5 | 27   | ScanContractStEph.rs        | rlimit — matching loop on fold_left_split  | Added opaque `scan_prefix` wrapper + bridging lemma to shield loop invariant from auto-broadcast `vstd::seq_lib::lemma_fold_left_split` |

### Root cause for #4 and #5 (trigger matching loops)

The Verus upgrade rework of how `Copy`/`Fn`/`FnOnce`/`FnMut` are treated as first-class
traits (commit `3390e9af0`) changed how `spec_fn`-laden triggers fire. Two invariants
that previously verified now provoked quantifier matching loops:

- **Chap28** (198M cost): single-term trigger `spec_prefix_sum(a.seq@, hi)` in a `forall`
  matched against invariant `ap.spec_index(k) == spec_prefix_sum(a.seq@, k)`. Each
  instantiation at `k` produced a new `spec_prefix_sum(a.seq@, k)` term matching the
  forall's trigger at `hi = k`, looping.
- **Chap27** (198 BILLION cost, 183K instantiations): `lemma_fold_left_split` in
  `group_seq_lib_default` (auto-broadcast via `group_vstd_default`) has trigger
  `self.subrange(0, k).fold_left(b, f)`. Its ensures produces a new
  `self.subrange(0, k').fold_left(b, f)` term that matches the same trigger,
  self-looping. Before the upgrade, Z3's trigger selection apparently avoided this.

Fix strategies: Chap28 used multi-trigger (cheap, local). Chap27 required an opaque
spec-function wrapper (`scan_prefix`) with a bridging lemma (`lemma_scan_prefix_unfold`)
to shield the loop invariant from the auto-broadcast while still connecting to the
trait's fold_left-based requires/ensures.

## Files Changed (src)

```
src/Chap27/ScanContractStEph.rs       | 64 ++++++++++++++++++++++++++++++-----
src/Chap28/MaxContigSubSumOptMtEph.rs |  4 +--
src/Chap39/BSTParaTreapMtEph.rs       |  8 +++--
src/Chap41/AVLTreeSetMtEph.rs         |  4 ---
src/Chap41/AVLTreeSetMtPer.rs         |  4 ---
src/Chap43/OrderedSetMtEph.rs         |  1 +
6 files changed, 64 insertions(+), 21 deletions(-)
```

## Validation

- `scripts/validate.sh`: 5765 verified, 0 errors, 215s
- `scripts/rtt.sh`: 4209 tests, 0 failures, 42s
- `scripts/ptt.sh`: 289 tests, 0 failures, 284s
- `scripts/all-holes-by-chap.sh`: 46 clean chapters, 0 holed chapters, 0 holes

## Notable Observations for Future Rounds

1. The `lemma_fold_left_split` broadcast is in `group_seq_lib_default` → `group_vstd_default`,
   which has the attribute `broadcast_use_by_default_when_this_crate_is_imported`. This
   means ALL files auto-get it. Any fold_left invariant in a loop is at risk of a
   similar matching loop under the new Verus. The `scan_prefix` pattern from
   `src/Chap27/ScanContractStEph.rs` is the template.

2. Multi-trigger fix (Chap28) is cheap — it doesn't require helper spec fns. Try it
   first when a single-term trigger on a `forall` invariant provokes a matching loop.

3. The Chap39 assertion fixes illustrate a broader pattern under the new trait-axiom
   trigger handling: when a `forall`-quantified requires is expected to instantiate at
   a particular witness, explicitly `assert` the witness and the resulting fact inside
   the `by` block. Don't rely on Z3 to find the witness.
