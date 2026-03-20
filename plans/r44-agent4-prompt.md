# R44 Agent 4: Chap38 + Chap39 (48 holes)

## Assignment

Continue proving BSTParaMtEph.rs (Chap38, 21 holes remaining) and begin
BSTParaTreapMtEph.rs (Chap39, 27 holes). You proved 7 holes in Chap38 last
round using the ghost field migration pattern.

## Baseline

125 holes total. 4366 verified. Your chapters: Chap38 (21), Chap39 (27).

## Chap38 Target Holes (21)

Your R43 report identified Phase 2: adding ensures to helper functions to
unblock trait impl delegations (-6 to -7 estimated).

| # | File | Function | Line | Type | Notes |
|---|------|----------|------|------|-------|
| 1 | BSTParaMtEph.rs | find | 352 | assume | rwlock reader bridge |
| 2 | BSTParaMtEph.rs | join_pair (trait) | 363 | external_body | delegates to join_pair_inner |
| 3 | BSTParaMtEph.rs | union (trait) | 368 | external_body | delegates to union_inner |
| 4 | BSTParaMtEph.rs | intersect (trait) | 373 | external_body | delegates to intersect_inner |
| 5 | BSTParaMtEph.rs | difference (trait) | 378 | external_body | delegates to difference_inner |
| 6 | BSTParaMtEph.rs | filter (trait) | 383 | external_body | delegates to filter_inner |
| 7 | BSTParaMtEph.rs | reduce (trait) | 393 | external_body | delegates to reduce_inner |
| 8 | BSTParaMtEph.rs | in_order (trait) | 398 | external_body | delegates to collect_in_order |
| 9 | BSTParaMtEph.rs | expose_internal | 457 | external_body | rwlock acquire_read |
| 10 | BSTParaMtEph.rs | split_inner | 493 | external_body | recursive split |
| 11 | BSTParaMtEph.rs | min_key | 524 | external_body | tree traversal |
| 12 | BSTParaMtEph.rs | join_pair_inner | 535 | external_body | uses min_key + join_mid |
| 13 | BSTParaMtEph.rs | union_inner | 547 | external_body | parallel split + union + join |
| 14 | BSTParaMtEph.rs | intersect_inner | 561 | external_body | parallel split + intersect + join |
| 15 | BSTParaMtEph.rs | difference_inner | 578 | external_body | parallel split + difference + join |
| 16 | BSTParaMtEph.rs | filter_inner | 597 | external_body | recursive filter |
| 17 | BSTParaMtEph.rs | filter_parallel | 620 | external_body | parallel filter with join |
| 18 | BSTParaMtEph.rs | reduce_inner | 629 | external_body | recursive reduce |
| 19 | BSTParaMtEph.rs | reduce_parallel | 653 | external_body | parallel reduce with join |
| 20 | BSTParaMtEph.rs | collect_in_order | 663 | external_body | in-order traversal to Vec |
| 21 | BSTParaMtEph.rs | join_mid | 468 | fn_missing_requires | (warning) |
| 22 | BSTParaMtEph.rs | join_m | 518 | fn_missing_requires | (warning) |

### Chap38 Strategy

**Phase 2 (from your report):** Add ensures to helper functions, then prove trait
impl delegations.

Priority order:
1. Fix fn_missing_requires on `join_mid` and `join_m` (warnings, quick)
2. Add ensures to `expose_internal` — this is the rwlock acquire_read bridge.
   Even if the body stays external_body, adding ensures lets callers use results.
3. Add ensures to `split_inner`, `min_key`, `join_pair_inner` — enables delegation
4. Prove trait delegations: `join_pair`, `union`, `intersect`, `difference`, etc.
   These just call the _inner helpers. Once helpers have ensures, remove external_body
   from the trait impls.
5. `collect_in_order` — in-order traversal, may be provable with recursion
6. `filter_inner`, `reduce_inner` — recursive, harder

## Chap39 Target Holes (27)

BSTParaTreapMtEph.rs has the **same opaque View pattern** you fixed in Chap38.
Apply the ghost field migration:

1. Add `ghost_locked_root: Ghost<Set<T::V>>` to ParamTreap struct
2. Remove external_body from View::view, read from ghost field
3. Update `new()`, `expose_with_priority()`, `find()` to use ghost field
4. Add `unsafe impl Send/Sync` for the Ghost field

| # | File | Function | Line | Type |
|---|------|----------|------|------|
| 1 | BSTParaTreapMtEph.rs | view | 67 | external_body |
| 2 | BSTParaTreapMtEph.rs | new | 507 | assume |
| 3 | BSTParaTreapMtEph.rs | expose_with_priority | 537 | assume |
| 4 | BSTParaTreapMtEph.rs | find | 663 | assume |
| 5-27 | BSTParaTreapMtEph.rs | (23 more external_body) | various | external_body |

### Chap39 Strategy

Mirror the Chap38 ghost field work:
1. Ghost field migration (View, new, expose, find, clone) — estimated -5 to -7
2. Add ensures to helpers — estimated -3 to -5
3. Prove trait delegations — estimated -3 to -5

## What NOT to do:
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel implementations.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap38/ src/Chap39/`.
Write your report to `plans/agent4-round44-report.md`.
