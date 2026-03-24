# R69 Agent 2: OrderedTableStPer Constructor Axiom Elimination

## Goal

Eliminate constructor axiom assumes in `src/Chap43/OrderedTableStPer.rs`. Target: 21 → ~3.

## Current Holes (21)

- 7 axiom assumes in `empty()` (line ~843)
- 7 axiom assumes in `singleton()` (line ~862) — wait, check current state
- 2 axiom assumes in `tabulate()` pre-loop (line ~1026)
- 4 axiom assumes in `tabulate()` body (line ~1144)
- 1 `assume(iter_invariant(self))` in iterator — standard, leave
- 1 `fn_missing_wf_ensures` on `from_sorted_entries` — add ensures wf

Run `scripts/holes.sh src/Chap43/OrderedTableStPer.rs` first to get the exact current list.

## Strategy: Lift Axioms to Constructor Requires

The axiom predicates (`obeys_cmp_spec::<K>()`, `view_ord_consistent::<K>()`, etc.) are
type-level laws. Methods get them from `self.spec_orderedtablestper_wf()`. Constructors
have no `self`.

**Fix**: Add the axiom predicates to the trait's constructor `requires`:

```rust
fn empty() -> (table: Self)
    requires
        vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        view_ord_consistent::<Pair<K, V>>(),
        spec_pair_key_determines_order::<K, V>(),
        vstd::laws_cmp::obeys_cmp_spec::<K>(),
        view_ord_consistent::<K>(),
        obeys_feq_fulls::<K, V>(),
        obeys_feq_full::<Pair<K, V>>(),
    ensures
        table@ == Map::empty(),
        table.spec_orderedtablestper_wf();
```

Same for `singleton` and `tabulate`.

Then in the impl body, delete the assumes — the requires provides them.

### Caller Impact

Check who calls `empty()`, `singleton()`, `tabulate()`:
- Other Chap43 methods that have wf in requires — they can satisfy the axiom requires
  because wf includes those predicates
- RTT tests — they use concrete types (u64, etc.) where the axioms hold trivially
- PTT tests — same
- AugOrderedTable — check if it calls constructors

If callers already have wf in scope, they satisfy the new requires automatically. If
a caller creates a table without prior wf context, they'll need their own axiom requires
(cascade).

### from_sorted_entries

This is a free function, not a trait method. Add axiom requires + ensures wf:
```rust
fn from_sorted_entries(...) -> (result: OrderedTableStPer<K, V>)
    requires
        ...,  // existing requires
        vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        view_ord_consistent::<Pair<K, V>>(),
    ensures
        ...,  // existing ensures
        result.spec_orderedtablestper_wf(),
```

## Steps

1. **Read** OrderedTableStPer.rs — understand trait constructor signatures
2. **Search** for callers of `empty()`, `singleton()`, `tabulate()`, `from_sorted_entries()`
3. **Add** axiom predicates to constructor requires in the trait
4. **Delete** axiom assumes from constructor impl bodies
5. **Fix** any callers that now fail (add axiom requires to them too, or extract from wf)
6. **Add** `ensures wf` to `from_sorted_entries`
7. **Validate**, **rtt**, **ptt**

## Constraints

- Modify only `src/Chap43/OrderedTableStPer.rs` and possibly its callers in Chap43.
- Do NOT modify files outside Chap43.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- The iterator assume stays (standard pattern).
- Run validate, rtt, ptt sequentially.
