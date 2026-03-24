# R73 — Prove select_key in OrderedTableStEph.rs

## Objective

Remove the `external_body` from `select_key` at line 3436 in
`src/Chap43/OrderedTableStEph.rs` and prove its postcondition.

## The function

```rust
fn select_key(&self, i: usize) -> (selected: Option<K>)
    where K: TotalOrder
    requires
        self.spec_orderedtablesteph_wf(),
        obeys_view_eq::<K>(),
    ensures
        self@.dom().finite(),
        i >= self@.dom().len() ==> selected matches None,
        selected matches Some(k) ==> self@.dom().contains(k@),
        selected matches Some(v) ==> self@.dom().filter(
            |x: K::V| exists|t: K| #![trigger t@] t@ == x
                && TotalOrder::le(t, v) && t@ != v@
        ).len() == i as int;
```

## The body (already written, just needs proof)

The implementation:
1. Returns `None` if `i >= self.size()`.
2. Otherwise iterates `self.tree.in_order()`, calls `rank_key` on each
   candidate, and returns the first key whose rank equals `i`.

## Dependencies — rank_key_iter must be proved first

`select_key` calls `self.rank_key(&candidate_key)` which delegates to
`rank_key_iter`. That function must be proved first (see
`plans/r73-rank-key-iter-prompt.md`). If rank_key_iter still has
external_body when you start, prove it first.

## Proof strategy

1. Remove `#[verifier::external_body]` and the agent-ownership comment.
2. For the `i >= self.size()` branch: prove `self.size()` equals
   `self@.dom().len()` (should follow from existing specs).
3. For the loop branch, add invariants:
   - The sorted sequence from `in_order()` contains all domain elements.
   - If no result found yet in `sorted[0..j]`, no element in that prefix
     has rank `i`.
   - When a match is found, `rank_key` returns `i`, which by rank_key's
     ensures means the element has exactly `i` predecessors in the domain.
4. After the loop, connect `rank_key`'s postcondition (count of strictly-
   less-than elements) to select_key's postcondition (filter length equals
   `i`). These are the same predicate — just stated differently.

## Validation

Run `scripts/validate.sh` after removing the external_body. If the proof
is hard, add intermediate assertions. Do not re-add external_body — leave
the corpse if stuck and report what you tried.

## Rules

- Read `CLAUDE.md` on startup.
- Read the full OrderedTableStEph.rs file (or at least sections 6-9) to
  understand the spec functions, lemmas, and proof patterns already in use.
- Do NOT weaken the ensures. The spec is from APAS.
- Do NOT add assume or accept.
- Prove rank_key_iter first if it still has external_body.
