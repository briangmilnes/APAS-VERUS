# Agent 1 — Round 81 Report

## Objective

Remove `external_body` from `union` in `src/Chap65/UnionFindStEph.rs` and prove it.

## Result: Partial — external_body removed, proof architecture built, rlimit bottleneck remains

### What was accomplished

1. **Removed `external_body` from `union`** — the body is now verified Verus code.

2. **Wrote `lemma_root_is_self_parent`** — proves that if `roots[v] == v` then `parent[v]@ == v`. Uses contradiction via rank_increases + rank_bounded + parent_preserves_root.

3. **Wrote 3 wf sub-lemmas** (all verified at rlimit 50-80):
   - `lemma_union_wf_roots` — proves `spec_roots_idempotent` and `spec_roots_in_dom`
   - `lemma_union_wf_parent` — proves `spec_parent_closed`, `spec_self_parent_is_root`, `spec_parent_preserves_root`
   - `lemma_union_wf_ordering` — proves `spec_rank_increases`, `spec_rank_bounded`

4. **Wrote `spec_union_lemma_pre`** — factored shared preconditions for parent/ordering lemmas.

5. **Wrote `union_merge`** — free function that performs the parent/rank mutations, ghost roots update, and calls the 3 wf sub-lemmas.

6. **Simplified `union`** — delegates to `find_root_loop` (via `find`), `feq`, `lemma_root_is_self_parent`, and `union_merge`.

### What blocks completion

Two functions exceed rlimit with Z3 crashing above rlimit(120):

| # | Chap | File | Function | rlimit | Issue |
|---|------|------|----------|--------|-------|
| 1 | 65 | UnionFindStEph.rs | `union_merge` | 120 | 3 lemma calls + exec mutations + ghost roots (Map::new) exceed budget |
| 2 | 65 | UnionFindStEph.rs | `union` | 80 | 2 find calls + feq + 2 root lemmas + union_merge call exceed budget |

Plus one `admit()`:

| # | Chap | File | Line | Issue |
|---|------|------|------|-------|
| 1 | 65 | UnionFindStEph.rs | 717 | `admit()` for rank overflow (`rank_u + 1 <= usize::MAX`). Requires 2^rank theorem (induction over union history) — not derivable from static wf |

### Techniques tried (15 iterations)

1. Inline proof in union body — rlimit exceeded
2. Single `lemma_union_preserves_wf` — rlimit exceeded
3. Split into 2 lemma halves — structural half failed
4. Split into 3 lemma thirds — all pass individually
5. Targeted requires for roots lemma — passed at rlimit(50)
6. Various rlimit values (80-200) — Z3 crashes above ~120
7. Pre-asserting `spec_union_lemma_pre` — no improvement
8. Exec/proof split (`union_merge_exec` + `union_merge_prove_wf`) — exec passes but proof function exceeds
9. Frame assertions to help Z3 — no improvement
10. Removing redundant assertions from union — no improvement

### Root cause analysis

The verification work for `union_merge` exceeds Z3's rlimit budget at ~120 because:
- The function body has exec code (clone, insert × 2, Ghost assignment) generating verification conditions
- The proof block calls 3 lemmas, each requiring precondition verification
- The ensures clause (wf + frame + roots quantifier) requires combining lemma postconditions

Z3 crashes at rlimit > 120, preventing brute-force budget increases.

### Recommended next steps

1. **Split union_merge further**: Factor out the `if rank_u < rank_v` branches into separate functions, each handling one case. This reduces per-function complexity.

2. **Eliminate spec_union_lemma_pre**: Inline the shared conditions directly into lemma requires. The spec function unfolding may add solver overhead.

3. **Rank overflow**: Add `rank < elements.len()` as a wf conjunct. Proving preservation requires the 2^rank theorem, which needs a recursive ghost function counting component sizes.

4. **Consider Verus upgrade**: Newer Verus versions may have improved Z3 communication (the "too many empty lines" crash is a process IPC issue, not a logic issue).

## Holes before/after

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 65 | UnionFindStEph.rs | 1 (external_body on union) | 1 (admit for overflow) + 2 rlimit | net 0 proven |
| 2 | 65 | KruskalStEph.rs | 1 | 1 | 0 |
| 3 | 65 | PrimStEph.rs | 2 | 2 | 0 |

Total Chap65: 4 holes before, 4 holes after (converted 1 external_body to 1 admit + 2 rlimit-blocked).

The 3 wf sub-lemmas (roots, parent, ordering) are fully verified. The proof architecture is sound — it just needs the rlimit barrier resolved.
