# Agent 1 — Round 85 Report

## Objective

Close `spec_unionfindsteph_wf` and prove `union_merge` + `union` in
`src/Chap65/UnionFindStEph.rs`.

## Result

**Partial success.** Achieved the wf decomposition architecture and verified the
proof coordination lemma (`lemma_union_merge_wf`). Could not remove `external_body`
from the exec wrappers (`union_merge`, `union`) due to Z3 &mut encoding overhead.

## Verification Counts

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap65 isolate verified | 2403 | 2405 | +2 |
| UnionFindStEph holes | 3 | 3 | 0 |
| Chap65 total holes | 6 | 6 | 0 |

## Architecture Changes

### 1. Closed `spec_uf_wf` delegation pattern

Added `pub closed spec fn spec_uf_wf<V>` containing the conjunction of all 14
sub-predicates. The trait impl's `spec_unionfindsteph_wf` delegates:

```rust
open spec fn spec_unionfindsteph_wf(&self) -> bool {
    spec_uf_wf(self)
}
```

Effect: Z3 sees one opaque boolean in exec contexts. Proof functions use
`reveal(spec_uf_wf)` to access the sub-predicate conjunction, then
selectively `reveal` individual closed sub-predicates.

### 2. Functions refactored with selective reveals

| # | Chap | Function | Reveals |
|---|------|----------|---------|
| 1 | 65 | `lemma_insert_preserves_wf` | spec_uf_wf + all 10 closed sub-predicates |
| 2 | 65 | `lemma_root_is_self_parent` | spec_uf_wf + 4 sub-predicates |
| 3 | 65 | `lemma_non_root_rank_lt_root` | spec_uf_wf + 4 sub-predicates |
| 4 | 65 | `find_root_loop` | spec_uf_wf + 5 sub-predicates |
| 5 | 65 | `new()` | spec_uf_wf + all 10 sub-predicates |
| 6 | 65 | `num_sets` | spec_uf_wf + spec_elements_forward |

### 3. New: `lemma_union_merge_wf` (VERIFIED)

Proof function that coordinates all sub-lemmas to prove wf after union merge.
Takes pre-state wf + structural mutation facts as requires, ensures post-state wf.
Internally reveals `spec_uf_wf` and calls: `lemma_establish_union_pre`,
`lemma_union_wf_roots_closed`, `lemma_union_wf_parent`, `lemma_union_wf_ordering`,
`lemma_union_wf_frame`, `lemma_assemble_wf`.

### 4. Sub-lemmas refactored to by-ref

Changed `lemma_union_wf_roots_closed`, `lemma_union_wf_parent`,
`lemma_union_wf_ordering`, `lemma_union_wf_frame` from by-value to by-reference
parameters. Reduces Z3 struct equality overhead.

### 5. `new()` proves wf

Added `uf.spec_unionfindsteph_wf()` to `new()`'s ensures (trait + impl). Proves
wf for empty UF via reveals. Fixes Kruskal's loop invariant establishment.

### 6. `lemma_decompose_wf` / `lemma_assemble_wf` simplified

Bodies now just `reveal(spec_uf_wf)` instead of revealing all 10 sub-predicates.

## What blocked `union_merge` exec verification

The Z3 &mut encoding for `UnionFindStEph<V>` (4 fields: parent Map, rank Map,
elements Seq, roots Ghost Map) creates expensive array-theory terms. Even a
minimal exec wrapper (one `union_merge_exec` call + one proof lemma call) uses
4–15 GB Z3 RSS and exceeds rlimit at any setting. The proof itself is complete —
`lemma_union_merge_wf` verifies the wf transition. The issue is purely the
encoding overhead of combining &mut snapshot semantics with quantified map
operations in a single Z3 context.

Approaches tried (16 validate iterations):
1. Open wf with closed sub-predicates: 15 GB Z3 RSS
2. Separated proof lemma (verified!): exec wrapper still 11 GB
3. By-ref sub-lemmas: still 15 GB
4. Closed spec_uf_wf delegation: Z3 down to 4.4 GB but rlimit exceeded
5. Matched field access patterns: no improvement
6. Various rlimit settings (30–200): exponential blowup above 60

## Remaining Holes in UnionFindStEph.rs

| # | Chap | File | Line | Type | Notes |
|---|------|------|------|------|-------|
| 1 | 65 | UnionFindStEph.rs | 982 | admit() | Rank overflow bound (intentional) |
| 2 | 65 | UnionFindStEph.rs | 1078 | external_body | union_merge (Z3 &mut encoding) |
| 3 | 65 | UnionFindStEph.rs | 1308 | external_body | union (Z3 &mut encoding) |

## Techniques Used

- Closed spec function delegation (spec_uf_wf pattern)
- Selective `reveal` to limit Z3 quantifier exposure
- Proof/exec separation (lemma_union_merge_wf)
- By-reference proof function parameters (reduce Z3 struct copies)
- Ghost<UnionMergeInfo<V>> return type for exec→proof coordination
