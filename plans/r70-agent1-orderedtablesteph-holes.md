# R70 Agent 1: OrderedTableStEph — Constructor Axioms + rank/select + wf

## Goal

Eliminate all 8 proof holes in `src/Chap43/OrderedTableStEph.rs`. Target: 8 → 0.

## Current Holes (8)

Run `scripts/holes.sh src/Chap43/OrderedTableStEph.rs` first to get exact current list.

| # | Line | Type | Content |
|---|------|------|---------|
| 1 | 1327 | assume | `obeys_cmp_spec::<Pair<K, V>>()` in tabulate |
| 2 | 1328 | assume | `view_ord_consistent::<Pair<K, V>>()` in tabulate |
| 3 | 1454 | assume | `spec_pair_key_determines_order::<K, V>()` in tabulate |
| 4 | 1455 | assume | `obeys_cmp_spec::<K>()` in tabulate |
| 5 | 1456 | assume | `view_ord_consistent::<K>()` in tabulate |
| 6 | 1457 | assume | `obeys_feq_fulls::<K, V>()` in tabulate |
| 7 | 3409 | external_body | `rank_key_iter` — has body, just needs proof |
| 8 | 3436 | external_body | `select_key` — has body, just needs proof |

Plus 1 non-hole style fix:
- Line 3753: `fn_missing_wf_ensures` — `from_sorted_entries` missing `result.spec_orderedtablesteph_wf()`.

## Strategy

### Constructor Axiom Assumes (holes 1-6)

The 6 axiom assumes are in `tabulate`. These are type-level laws that hold for all types
satisfying the trait bounds but can't be derived from the generic trait context.

**Fix**: Add the axiom predicates to the trait's `tabulate` requires:

```rust
fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
    requires
        // existing requires...
        vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        view_ord_consistent::<Pair<K, V>>(),
        spec_pair_key_determines_order::<K, V>(),
        vstd::laws_cmp::obeys_cmp_spec::<K>(),
        view_ord_consistent::<K>(),
        obeys_feq_fulls::<K, V>(),
    ensures ...;
```

Then delete the 6 assumes from the impl body — the requires provides them.

**Caller impact**: Search for callers of `tabulate` in Chap43. Methods that have
`self.spec_orderedtablesteph_wf()` in requires can satisfy the axiom predicates because
wf includes them. Check RTT and PTT tests too — concrete types (u64, etc.) satisfy
axioms trivially, but may need explicit `proof { }` blocks.

Also check whether `empty()` and `singleton()` in OrderedTableStEph already have axiom
requires or if they need the same treatment. Look at the trait signature for these
constructors.

### rank_key_iter (hole 7)

This function has a full body but is wrapped in `#[verifier::external_body]`. The comment
says "Agent 3 owns rank_key proof" — that was R69, which was not merged. Remove the
external_body and prove it.

The function iterates over sorted entries, counting keys less than `k`. You need:
- Loop invariants for `count` and `i`
- Prove `count` equals the number of keys in `self@.dom()` that are less than `k`
- Bridge between the sorted `ArraySeqStPerS` iteration and the abstract `rank` spec

Check the trait's `rank_key` ensures clause for the target postcondition.

### select_key (hole 8)

Similar pattern — has body, needs proof. Iterates over sorted entries finding the one
whose rank equals `i`. Depends on rank_key being proved first (it calls `self.rank_key()`
in the loop body).

### from_sorted_entries (style fix)

Add `result.spec_orderedtablesteph_wf()` to the ensures clause and prove it. The function
already has axiom predicates in its requires, so use those to establish wf.

## Steps

1. **Read** OrderedTableStEph.rs — understand the trait constructor signatures
2. **Search** callers of `tabulate`, `empty`, `singleton` across Chap43
3. **Add** axiom predicates to constructor trait requires
4. **Delete** axiom assumes from constructor impl bodies
5. **Fix** callers that now fail
6. **Remove** external_body from rank_key_iter, add loop invariants and proof
7. **Remove** external_body from select_key, add loop invariants and proof
8. **Add** wf ensures to from_sorted_entries
9. **Validate**, **rtt**, **ptt** — run sequentially, never in parallel

## Constraints

- Modify only `src/Chap43/OrderedTableStEph.rs` and callers within Chap43.
- Do NOT modify OrderedTableStPer.rs (Agent 2 owns that).
- Do NOT modify OrderedSetStEph.rs (Agent 4 owns that).
- Do NOT modify BSTTreapStEph.rs (Agent 3 owns that).
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially, never in parallel.
- Write report to `plans/agent1-round70-report.md` when done.
