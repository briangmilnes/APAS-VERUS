# R39 Agent 4: Restructure OrderedTableMtEph.rs + OrderedTableMtPer.rs

## Baseline
- Main at `e6e3c688`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4337 verified, 175 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and these standards before starting:
- `src/standards/mod_standard.rs`
- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
- `src/standards/total_order_standard.rs`

## Context

Same restructure as Agents 1 and 2. The APAS textbook says ordered tables use balanced
BSTs. The current OrderedTable Mt variants wrap TableMtEph/TableMtPer (flat unsorted
arrays). Collect uses Vec::sort_by (unverified).

You are working on the **multi-threaded** variants. These use RwLock wrapping around
the single-threaded implementations. Your restructure follows the same pattern as
Agents 1 (StEph) and 2 (StPer), adapted for the Mt locking layer.

## Assignment

Restructure these two files:
1. `src/Chap43/OrderedTableMtEph.rs` (1336 lines) — primary target
2. `src/Chap43/OrderedTableMtPer.rs` (559 lines) — secondary target

### File 1: OrderedTableMtEph.rs

Read `src/Chap43/OrderedSetMtEph.rs` first — it shows how the Mt ordered SET wraps
a single-threaded OrderedSetStEph inside an RwLock. OrderedTableMtEph should follow
the same pattern, wrapping OrderedTableStEph inside an RwLock.

**Current structure:**
```rust
pub struct OrderedTableMtEph<K: MtKey, V: MtVal> {
    inner: Arc<RwLock<OrderedTableStEph<K, V>>>,
}
```

The struct likely already wraps OrderedTableStEph. If so, the restructure at this level
may be minimal — the inner OrderedTableStEph is being restructured by Agent 1 in parallel.
Your job is to make sure the Mt layer correctly delegates and its specs/ensures align.

**However**, if OrderedTableMtEph wraps TableMtEph instead of OrderedTableStEph, you need
to change it to wrap OrderedTableStEph (or the equivalent BST-backed structure).

Check the actual struct definition and delegation pattern before making changes.

**Key tasks:**
- Verify the struct wraps OrderedTableStEph (if not, restructure to do so)
- Fix the `collect` external_body (line 1065) — should delegate to inner's collect
  which no longer needs sort_by after Agent 1's restructure
- Fix `fn_missing_wf_ensures` in `from_sorted_entries` (line 1275)
- Ensure all Mt operations correctly acquire locks and delegate to StEph operations
- The trait interface MUST NOT CHANGE

**Important Mt patterns** (read `toplevel_coarse_rwlocks_for_mt_modules.rs`):
- `new_arc_rwlock` / `clone_arc_rwlock` from `vstdplus/arc_rwlock.rs`
- RwLock acquire → operate on inner → release
- Ghost tracking through `RwLockPredicate`

### File 2: OrderedTableMtPer.rs

Smaller file (559 lines). The persistent Mt variant. Similar pattern — wraps the
persistent StPer variant inside an RwLock. May need less restructuring if it already
delegates cleanly to OrderedTableStPer.

### Strategy

1. Read both files completely first
2. Check what they actually wrap (OrderedTableStEph? TableMtEph? Something else?)
3. If they already delegate to OrderedTableSt{Eph,Per}, the restructure may be minimal
   (just ensure collect delegation works without sort_by)
4. If they wrap Table directly, restructure to wrap the ordered table variants
5. Fix all external_body holes and warnings

### What to do about Agent 1/2 dependency

Agents 1 and 2 are restructuring OrderedTableStEph and OrderedTableStPer in parallel
with you. Since you work in your own worktree, your OrderedTableStEph/StPer will still
have the OLD implementation (wrapping TableStEph).

**Your approach**: restructure the Mt files to delegate to OrderedTableStEph/StPer as
they exist NOW. When Agent 1/2's work is merged, the Mt files will automatically benefit
because they delegate to the St layer. If the delegation is clean, the Mt files won't
need further changes.

If you find the Mt files directly access `base_table.entries` (bypassing the St layer),
change those accesses to use the St API instead.

### Expected Results

- Fix collect external_body in MtEph (-1 hole)
- Fix fn_missing_wf_ensures warning (-1 warning)
- Clean up any direct base_table access patterns
- Prepare Mt layer for seamless integration with Agent 1/2's restructure

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r39-report.md`.
