<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent4 Round 52 Report

**Date**: 2026-03-20  
**Branch**: `~/projects/APAS-VERUS-agent4`  
**Starting state**: 4472 verified, 25 holes, 37 clean chapters

---

## Summary

Round 52 targeted two phases: (1) closing the `clone_elem` assume hole in Chap39 BSTParaTreapMtEph and investigating the `filter_parallel` Send blocker; (2) investigating the Chap45 Example45_2 hole.

**Result**: Closed the `clone_elem` hole (–1 hole). The `filter_parallel` hole is a structural blocker. The Chap45 Example45_2 hole is also a structural blocker.

---

## Holes Closed

| # | Chap | File | Line | Type | How Resolved |
|---|:----:|---|---:|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | 128 | `assume() [clone bridge]` | Used `ClonePreservesView::clone_view()` |

### Detail: clone_elem fix

`clone_elem` previously used `assume(c@ == x@)` to bridge the gap between Rust's `Clone` trait and Verus's view semantics. The fix:

1. Added `+ ClonePreservesView` to the `T: MtKey` bound on `clone_elem` and propagated it to all callers: `filter_inner`, `filter_parallel`, `expose_internal`, `expose_with_priority_internal`, `join_with_priority`, `split_inner`, `join_pair_inner`, `union_inner`, `intersect_inner`, `difference_inner`, `reduce_inner`, `reduce_parallel`, `collect_in_order`, and `ParamTreapTrait`.
2. Also propagated to `impl Clone for NodeInner<T>`, `impl Clone for Exposed<T>`, `impl fmt::Debug for ParamTreap<T>`, `impl fmt::Display for ParamTreap<T>` in `BSTParaTreapMtEph.rs`.
3. Updated `BSTSetTreapMtEph.rs`: `impl BSTSetTreapMtEphTrait`, `impl fmt::Debug`, and `impl fmt::Display` — all got `+ ClonePreservesView + 'static`.
4. Replaced the function body: `x.clone_view()` (no assume needed).

---

## Blockers

| # | Chap | File | Line | Hole | Reason |
|---|:----:|---|---:|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | 1699 | `filter_parallel Send limit` | T::V not Send |
| 2 | 45 | Example45_2.rs | 43 | `#[verifier::external]` impl | Calls non-verus fns |

### Blocker 1: filter_parallel (Send limitation)

`filter_parallel` uses `ParaPair!` (fork-join via threads). The predicate's ghost specification `Ghost<spec_fn(T::V) -> bool>` cannot be captured by `Send` closures because `T::V` is a Verus spec type (e.g., `int`, `Set<V>`) with no runtime representation and no `std::marker::Send` impl. Attempted to pass `Ghost(spec_pred)` through the closures; compiler rejected with `E0277`. The `assume` at line 1699 remains — it's a genuine Verus/Rust boundary limitation.

### Blocker 2: Example45_2.rs structural hole

The `impl Example45_2Trait for Example45_2` block delegates 8 methods to free functions declared outside `verus!` (in `HeapsortExample.rs`). Verus refuses to call non-verus functions from within a verified `verus!` impl. Attempted removing `#[verifier::external]` — 8 errors confirming the constraint. Restored the annotation. Fix would require moving all called functions into `verus!`, which cascades into a full `HeapsortExample.rs` refactor. Low priority (Example file per CLAUDE.md).

---

## Chap39 Hole Status

| # | File | Line | Type | Status |
|---|---|---:|---|---|
| 1 | BSTParaTreapMtEph.rs | 1699 | `assume() [algorithmic]` | Remains (Send blocker) |
| 2 | BSTParaTreapMtEph.rs | 206 | `fn_missing_requires` | Pre-existing warning |
| 3 | BSTParaTreapMtEph.rs | 419 | `fn_missing_requires` | Pre-existing warning |
| 4 | BSTParaTreapMtEph.rs | 1708 | `fn_missing_ensures` | Pre-existing warning |
| 5 | BSTParaTreapMtEph.rs | 1741 | `fn_missing_ensures` | Pre-existing warning |
| 6 | BSTTreapMtEph.rs | 388 | `fn_missing_requires` | Pre-existing warning |

Items 2–6 are veracity-level warnings (not algorithmic assumes), pre-existing from prior rounds.

---

## Verification Count

| Metric | Count |
|---|---|
| Verified | 4472 |
| Errors | 0 |
| Holes closed this round | 1 |
| Total holes (project) | 24 → 25 (net –1) |
| Chap39 algorithmic assumes | 1 remains |

---

## Files Changed

| # | Chap | File | Change |
|---|:----:|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | `clone_elem` → `clone_view()`; propagate `ClonePreservesView` |
| 2 | 39 | BSTSetTreapMtEph.rs | Propagate `ClonePreservesView + 'static` to 3 impl blocks |
