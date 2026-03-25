# R78 Agent 3 — BSTRBMtEph filter_parallel + reduce_parallel (Chap37, 2 holes)

## Objective

Prove or narrow 2 external_body holes in BSTRBMtEph.rs.

## Baseline

- 4898 verified, 0 errors, 0 warnings
- BSTRBMtEph.rs: 2 holes

## Holes

| # | Chap | File | Line | Function | Type |
|---|------|------|------|----------|------|
| 1 | 37 | BSTRBMtEph.rs | 799 | filter_parallel | external_body |
| 2 | 37 | BSTRBMtEph.rs | 824 | reduce_parallel | external_body |

## Strategy

These are fork-join functions using `Arc<F>` closures and thread spawning for parallel
tree traversal. Agent 3 R77 successfully proved the BSTSet-level filter/reduce by:
1. Removing external_body
2. Adding `forall|t: &T| #[trigger] predicate.requires((t,))` to requires + loop invariant
3. Using FnMut requires propagation

The BSTRBMtEph versions are different — they do **recursive parallel traversal** of the
tree structure with `Arc::clone` on the closure and thread spawning per subtree. The
pattern is:
```
fn filter_parallel(link: &Link<T>, predicate: &Arc<F>) -> Vec<T> {
    // recurse left subtree (spawned thread)
    // recurse right subtree (current thread)
    // merge results
}
```

**Approach**: Narrow external_body to just the thread spawn boundary. Factor the
algorithmic logic (traverse, filter, merge) into a verified helper. The thread spawn
just calls the helper on each subtree.

Read `src/standards/using_closures_standard.rs` for the closure/join pattern. Read
how `ParaPair!` macro works. Check if named closures with explicit ensures can replace
the `Arc<F>` thread spawn.

If narrowing isn't possible (Arc::clone on closures inside verus! may not compile),
document what blocks it.

## Key resources

- `src/Chap37/BSTRBMtEph.rs` — Read the filter_parallel and reduce_parallel functions
- `src/Chap37/BSTSetRBMtEph.rs` — Agent 3 R77's filter/reduce proof (BSTSet level)
- `src/standards/using_closures_standard.rs`

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent3/ready`.

## Report

Write `plans/agent3-round78-report.md` with holes before/after (table with Chap column).
