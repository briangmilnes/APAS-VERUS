# R79 Agent 3 — BSTSplayMtEph remaining 3 holes (Chap37)

## Objective

Prove or narrow the 3 remaining external_body holes in BSTSplayMtEph.rs.

## Baseline

- 4905 verified, 0 errors, 0 warnings
- BSTSplayMtEph.rs: 3 holes

## Holes

| # | Chap | File | Line | Function | Type | Blocker |
|---|------|------|------|----------|------|---------|
| 1 | 37 | BSTSplayMtEph.rs | ~1507 | build_balanced | external_body | to_vec specs, closure |
| 2 | 37 | BSTSplayMtEph.rs | ~1534 | filter_parallel | external_body | closure + Arc + parallelism |
| 3 | 37 | BSTSplayMtEph.rs | ~1567 | reduce_parallel | external_body | closure + Arc + parallelism |

## Context

Agent 2 R78 solved the clone root cause (manual `clone_link`) and replaced `node.clone()`
calls with `clone_link()` in all 3 functions. Clone is no longer the blocker.

Agent 3 R78 proved BSTRBMtEph's `filter_parallel` and `reduce_parallel` by:
1. Removing unnecessary `Arc::clone` (sequential recursion, not actual thread spawning)
2. Adding closure requires propagation (`forall|t: &T| predicate.requires((t,))`)
3. Explicit Arc deref: `(**predicate)(&node.key)` instead of `predicate(&node.key)`
4. `reveal_with_fuel(link_spec_size, 2)` for recursive requires

**Apply the same pattern to BSTSplayMtEph.** Read `src/Chap37/BSTRBMtEph.rs` to see
the R78 proven versions of filter_parallel/reduce_parallel. Then apply to Splay.

For `build_balanced`: check if `to_vec()` can be replaced with slice operations that
have Verus specs. Or if the function can be restructured to avoid `to_vec()`.

## Key resources

- `src/Chap37/BSTSplayMtEph.rs` — Read fully
- `src/Chap37/BSTRBMtEph.rs` — R78 proven filter_parallel/reduce_parallel pattern

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent3/ready`.

## Report

Write `plans/agent3-round79-report.md` with holes before/after (table with Chap column).
