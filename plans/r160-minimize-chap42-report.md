# R160 Agent 3 Report — Minimize Chap42 Table Union Proofs

## Summary

Minimized the post-loop proof blocks in the two `union` functions in Chap42.
Both functions verify; full codebase clean; RTTs pass.

## Per-Function Results

| # | Chap | File | Function | Asserts Before | Asserts After | Delta | Lines Before | Lines After | Line Delta |
|---|------|------|----------|---------------|--------------|-------|-------------|------------|------------|
| 1 | 42 | TableStEph.rs | `union` post-loop proof | 89 | 74 | -15 | ~167 | ~148 | -19 |
| 2 | 42 | TableMtEph.rs | `union` post-loop proof | 111 | 93 | -18 | ~236 | ~210 | -26 |
| — | — | — | **Total** | **200** | **167** | **-33** | — | — | — |

## Techniques Used

### Asserts successfully removed

- **`phase1_len == old_self_view.len()`** — Z3 derives this from loop invariants automatically.
- **Key-equality tautologies from `choose`** — when `si` is chosen such that `old_self_view[si].0 == k`, asserting that equality again is redundant.
- **Duplicate bounds assertions** — `0 <= si < phase1_len` established earlier in the block.
- **Phase 2 coverage found-case asserts** — the `if found { assert(phase2_sources[k] == oj); }` branch needed no explicit assertion when the lemma call already established membership.
- **Intermediate equality chains in "Value combined"** — Z3 could chain through `spec_entries_to_map_get` lemmas without intermediate steps.
- **No-dups phase2+phase2 ordering assert** — `assert(phase2_sources[ka] < phase2_sources[kb])` was derivable from the choose condition on `phase2_sources`.
- **MtEph combine ensures: 8 asserts removed** — bounds, key-equality duplicates, `ci` bounds, `other.entries@[ci].0` key equality, `kept@[si] == phase1_kept[si]`, suffix `contains_key`.

### Asserts that must remain

- **`assert(self.entries.spec_index(idx) == kept@[idx])`** — load-bearing bridge between exec array and ghost `kept@` after `from_vec`. Z3 cannot connect `self.entries@[idx]` to `kept@[idx]` without it. Must be UNCONDITIONAL (before any if/else branching) in the no-dups block so all branches see it.
- **`lemma_view_index` calls (StEph only)** — `ArraySeqStEphS::lemma_view_index` bridges `spec_index` to the view; `ArraySeqMtEphS` has no equivalent so spec_index asserts are used directly.
- **All `lemma_entries_to_map_*` calls** — these are load-bearing; Z3 cannot chain through the `spec_entries_to_map` spec function without them.
- **`assert(old_self_view[a].0 != old_self_view[b].0)` in no-dups phase1+phase1** — Z3 needs the explicit contradiction bridge from old self no-dups invariant.

## Key Discovery

The `spec_index` asserts in the no-dups block must be placed **unconditionally before** the
if/else branching. Placing them inside branches makes them unavailable to other branches,
causing Z3 failures. This is because Z3 processes each branch independently and does not
hoist facts from sibling branches.

`ArraySeqMtEphS` has no `lemma_view_index` method (unlike `ArraySeqStEphS`). All view-to-kept
bridging in MtEph must use `spec_index` asserts directly.

## Validation

- Isolate validation (`scripts/validate.sh isolate Chap42`): ~94s, 0 errors
- Full validation (`scripts/validate.sh`): **5748 verified, 0 errors** (113s)
- RTTs (`scripts/rtt.sh`): **3776 passed, 0 skipped** (30s)
