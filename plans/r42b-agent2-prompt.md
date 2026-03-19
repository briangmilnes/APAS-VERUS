# R42b Agent 2: OrderedTableStEph — Prove What Agent 1 Can't

## Baseline
- Main at `c010cf2a` (your R42 merge), 4333 verified, 146 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**
**DO NOT move code outside `verus!{}` or add `#[cfg(not(verus_keep_ghost))]` to
dodge verification.** All algorithm implementations belong inside `verus!{}`.
If you can't prove it, leave the `external_body` and report what you tried.

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

You just proved 7 methods in OrderedTableStPer.rs (domain, collect, difference,
first_key, last_key, previous_key, next_key) and added wf requires to both
OrderedTableStPer and AugOrderedTableStPer traits. Excellent work.

OrderedTableStEph.rs has 11 remaining external_body holes. Agent 1 was assigned
this file but has not committed anything. Agent 1 has uncommitted work on 4
methods (filter, intersection, split_key, get_key_range) — **avoid those 4**.

You are the best positioned agent for this because you already wrote the StPer
proofs that the StEph versions mirror.

## Assignment

Prove these OrderedTableStEph methods (the ones Agent 1 is NOT working on):

| # | Method | Lines | Notes |
|---|--------|-------|-------|
| 1 | insert | 533 | usize::MAX edge — new key → len+1. May need `requires self@.dom().len() < usize::MAX - 1` added to trait. |
| 2 | domain | 663 | feq_clone vs feq_full — you solved this for StPer. Adapt. |
| 3 | tabulate | 677 | usize::MAX edge like insert. |
| 4 | union | 777 | Closure value-correctness existential. You proved this for StPer using two-phase loop. Adapt. |
| 5 | rank_key | 1616 | TotalOrder counting. Hard — same as StPer rank_key you deferred. |
| 6 | select_key | 1669 | TotalOrder indexed access. Hard — same as StPer select_key you deferred. |
| 7 | avl_seq_length | 395 | Thin wrapper over base_seq.length(). Probably needs wf in requires. |
| 8 | avl_seq_nth | 403 | Thin wrapper over base_seq.nth(). Probably needs wf in requires. |

### Key Differences: StEph vs StPer

StEph uses `&mut self` (ephemeral — modifies in place). StPer uses `&self -> Self`
(persistent — returns new copy). The proof logic is the same but:
- StEph mutates `self.base_seq` in place
- StEph's `base_seq` is `AVLTreeSeqStEphS<Pair<K, V>>` (ephemeral sequence)
- StPer's `base_set` is `AVLTreeSetStPer<Pair<K, V>>` (persistent set)

Read the StEph file first. Understand the backing store differences.

### Strategy

1. **avl_seq_length, avl_seq_nth** (lines 395, 403) — likely trivial wrappers. Start here.
2. **domain** (line 663) — you already know the pattern from StPer.
3. **union** (line 777) — adapt your StPer two-phase loop proof.
4. **insert** (line 533) — may need trait requires change for usize::MAX bound.
5. **tabulate** (line 677) — same usize::MAX pattern as insert.
6. **rank_key, select_key** (lines 1616, 1669) — hard, attempt if time.

### Also: Add wf requires to StEph trait if needed

Just as you added `requires self.spec_orderedtablestper_wf()` to StPer trait
methods, the StEph trait may need `requires self.spec_orderedtablesteph_wf()`.
Check which methods need it and add to both trait and impl.

### Expected Results

Conservative: 4-5 methods proved (avl_seq_*, domain, union).
Optimistic: 6-8 methods proved (all except rank/select).

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent2-r42b-report.md`.

## Continue

Commit early, commit often. Push after each successful validation. Do not
stop until you run out of methods or context.
