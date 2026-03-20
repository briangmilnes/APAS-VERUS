# R45 Agent 4: Chap38 + Chap39 (30 holes)

## Assignment

Continue proving BSTParaMtEph.rs (Chap38, 14 holes) and BSTParaTreapMtEph.rs
(Chap39, 16 holes). You've closed 25 holes across R43+R44 using ghost field
migration and helper ensures. Now tackle the recursive _inner functions.

## Baseline

99 holes total. 4388 verified. Your chapters: Chap38 (14), Chap39 (16).

## Root-Cause Analysis

Read `plans/veracity-root-cause-vs-downstream-holes.md` for context.

In both chapters, `expose_internal` is the root cause. It does `acquire_read` on
`Arc<RwLock<...>>` — Verus can't verify through that. All other _inner functions
call `expose_internal` and are external_body because they need its result.

**Strategy: Add ensures to expose_internal even though its body stays external_body.**
If expose_internal has strong enough ensures (returns the locked BST node matching
the ghost field's state), then callers can use those ensures to prove their own logic.
The body stays external_body (RwLock boundary), but the spec becomes useful.

## Chap38 Target Holes (14)

| # | Chap | File | Function | Line | Type | Blocker |
|---|------|------|----------|------|------|---------|
| 1 | 38 | BSTParaMtEph.rs | find | 352 | assume | BST search correctness |
| 2 | 38 | BSTParaMtEph.rs | expose_internal | 450 | external_body | ROOT CAUSE: RwLock boundary |
| 3 | 38 | BSTParaMtEph.rs | split_inner | 486 | external_body | Calls expose_internal |
| 4 | 38 | BSTParaMtEph.rs | min_key | 517 | external_body | Calls expose_internal |
| 5 | 38 | BSTParaMtEph.rs | join_pair_inner | 528 | external_body | Calls min_key + join_mid |
| 6 | 38 | BSTParaMtEph.rs | union_inner | 542 | external_body | ParaPair + split + join |
| 7 | 38 | BSTParaMtEph.rs | intersect_inner | 558 | external_body | ParaPair + split + join |
| 8 | 38 | BSTParaMtEph.rs | difference_inner | 577 | external_body | ParaPair + split + join |
| 9 | 38 | BSTParaMtEph.rs | filter_inner | 598 | external_body | Recursive filter |
| 10 | 38 | BSTParaMtEph.rs | filter_parallel | 621 | external_body | ParaPair filter |
| 11 | 38 | BSTParaMtEph.rs | reduce_inner | 643 | external_body | Recursive reduce |
| 12 | 38 | BSTParaMtEph.rs | reduce_parallel | 667 | external_body | ParaPair reduce |
| 13 | 38 | BSTParaMtEph.rs | collect_in_order | 677 | external_body | Recursive collect |
| 14 | 38 | BSTParaStEph.rs | clone_elem | — | assume | Clone workaround |

Also fix warnings:
- BSTParaMtEph.rs: `join_mid` fn_missing_requires (line 461)
- BSTParaMtEph.rs: `join_m` fn_missing_requires (line 511)

### Chap38 Strategy

**Phase 3: Prove through the _inner functions.**

Priority order:
1. **Fix fn_missing_requires** on `join_mid` and `join_m` — add real requires
   (probably `self.spec_bstparamteph_wf()` or BST invariants on input nodes).

2. **Strengthen expose_internal ensures** — even with external_body, add ensures
   that describe what the returned node looks like relative to the ghost field.
   E.g., the returned BST node's view matches `self@`.

3. **Prove non-parallel _inner functions first**: `min_key`, `collect_in_order`,
   `filter_inner`, `reduce_inner`. These don't use ParaPair — just recursion over
   the exposed BST node. Once expose_internal has ensures, these may prove with
   recursion + the BST structure.

4. **Prove ParaPair _inner functions**: `union_inner`, `intersect_inner`,
   `difference_inner`, `filter_parallel`, `reduce_parallel`. These use ParaPair
   for fork-join. Use the named-closure pattern from plans/parapair-is-not-a-blocker.md.

5. **split_inner + join_pair_inner**: These are the BST split/join primitives.
   May be harder — depend on BST ordering invariants.

## Chap39 Target Holes (16)

| # | Chap | File | Function | Line | Type | Blocker |
|---|------|------|----------|------|------|---------|
| 1 | 39 | BSTParaTreapMtEph.rs | expose_internal | 158 | external_body | ROOT CAUSE: RwLock boundary |
| 2 | 39 | BSTParaTreapMtEph.rs | tree_priority | 201 | external_body | Calls expose_internal |
| 3 | 39 | BSTParaTreapMtEph.rs | tree_size | 211 | external_body | Calls expose_internal |
| 4 | 39 | BSTParaTreapMtEph.rs | make_node | 221 | external_body | Arc/RwLock construction |
| 5 | 39 | BSTParaTreapMtEph.rs | join_with_priority | 234 | external_body | Recursive + RwLock |
| 6 | 39 | BSTParaTreapMtEph.rs | split_inner | 258 | external_body | Recursive + RwLock |
| 7 | 39 | BSTParaTreapMtEph.rs | join_pair_inner | 288 | external_body | Recursive + RwLock |
| 8 | 39 | BSTParaTreapMtEph.rs | union_inner | 303 | external_body | ParaPair + split + join |
| 9 | 39 | BSTParaTreapMtEph.rs | intersect_inner | 318 | external_body | ParaPair + split + join |
| 10 | 39 | BSTParaTreapMtEph.rs | difference_inner | 337 | external_body | ParaPair + split + join |
| 11 | 39 | BSTParaTreapMtEph.rs | filter_inner | 356 | external_body | Recursive filter |
| 12 | 39 | BSTParaTreapMtEph.rs | filter_parallel | 374 | external_body | ParaPair filter |
| 13 | 39 | BSTParaTreapMtEph.rs | reduce_inner | 396 | external_body | Recursive reduce |
| 14 | 39 | BSTParaTreapMtEph.rs | reduce_parallel | 419 | external_body | ParaPair reduce |
| 15 | 39 | BSTParaTreapMtEph.rs | collect_in_order | 428 | external_body | Recursive collect |
| 16 | 39 | BSTParaTreapMtEph.rs | find | 703 | assume | BST search correctness |

### Chap39 Strategy

Mirror Chap38. Same root cause (expose_internal), same downstream pattern.

1. Strengthen expose_internal ensures (external_body stays)
2. Prove simple helpers: `tree_priority`, `tree_size` (just read through expose)
3. Prove `make_node` — Arc/RwLock construction with ghost field
4. Prove non-parallel _inner functions: `collect_in_order`, `filter_inner`, `reduce_inner`
5. Prove ParaPair functions: `union_inner`, `intersect_inner`, `difference_inner`
6. Hard: `split_inner`, `join_with_priority`, `join_pair_inner`

## ParaPair Pattern

Several _inner functions use `ParaPair!` for fork-join. This is fully verifiable.
Read `plans/parapair-is-not-a-blocker.md` for the pattern. Key points:
- Named closures with `let ghost` captures
- Explicit `ensures` on each closure arm
- `let Pair(a, b) = crate::ParaPair!(f1, f2);`
- Chap36 QuickSort and Chap06 Graph both verify through ParaPair with zero holes

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel implementations — keep ParaPair! calls.
- Do NOT claim RwLock or ParaPair makes functions "permanently blocked."

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap38/ src/Chap39/`.
Write your report to `plans/agent4-round45-report.md`.
