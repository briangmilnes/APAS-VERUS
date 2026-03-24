# R70 Agent 2: OrderedTableStPer — Constructor Axiom Elimination

## Goal

Eliminate all 20 proof holes in `src/Chap43/OrderedTableStPer.rs`. Target: 20 → 0.

## Current Holes (20)

Run `scripts/holes.sh src/Chap43/OrderedTableStPer.rs` first to get exact current list.

| # | Line | Type | Content |
|---|------|------|---------|
| 1 | 823 | assume | `obeys_feq_fulls::<K, V>()` in empty |
| 2 | 824 | assume | `obeys_feq_full::<Pair<K, V>>()` in empty |
| 3 | 825 | assume | `obeys_cmp_spec::<Pair<K, V>>()` in empty |
| 4 | 826 | assume | `view_ord_consistent::<Pair<K, V>>()` in empty |
| 5 | 827 | assume | `spec_pair_key_determines_order::<K, V>()` in empty |
| 6 | 828 | assume | `obeys_cmp_spec::<K>()` in empty |
| 7 | 829 | assume | `view_ord_consistent::<K>()` in empty |
| 8 | 842 | assume | `obeys_feq_fulls::<K, V>()` in singleton |
| 9 | 843 | assume | `obeys_feq_full::<Pair<K, V>>()` in singleton |
| 10 | 844 | assume | `obeys_cmp_spec::<Pair<K, V>>()` in singleton |
| 11 | 845 | assume | `view_ord_consistent::<Pair<K, V>>()` in singleton |
| 12 | 846 | assume | `spec_pair_key_determines_order::<K, V>()` in singleton |
| 13 | 847 | assume | `obeys_cmp_spec::<K>()` in singleton |
| 14 | 848 | assume | `view_ord_consistent::<K>()` in singleton |
| 15 | 1006 | assume | `obeys_cmp_spec::<Pair<K, V>>()` in tabulate pre-loop |
| 16 | 1007 | assume | `view_ord_consistent::<Pair<K, V>>()` in tabulate pre-loop |
| 17 | 1124 | assume | `spec_pair_key_determines_order::<K, V>()` in tabulate body |
| 18 | 1125 | assume | `obeys_cmp_spec::<K>()` in tabulate body |
| 19 | 1126 | assume | `view_ord_consistent::<K>()` in tabulate body |
| 20 | 1127 | assume | `obeys_feq_fulls::<K, V>()` in tabulate body |

Plus 1 non-hole style fix:
- Line 3347: `fn_missing_wf_ensures` — `from_sorted_entries` missing `result.spec_orderedtablestper_wf()`.

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

Same pattern for `singleton` and `tabulate`.

Then delete the assumes from the impl bodies.

### Caller Impact

Search for callers of `empty()`, `singleton()`, `tabulate()` in Chap43:
- Other Chap43 methods with `self.spec_orderedtablestper_wf()` in requires can satisfy
  the axiom requires because wf includes those predicates.
- RTT tests use concrete types (u64, etc.) where axioms hold trivially, but may need
  explicit `proof { }` blocks.
- AugOrderedTableStPer, OrderedSetStPer, OrderedTableMtPer — check if they call constructors.

If callers already have wf in scope, they satisfy the new requires automatically. If a
caller creates a table without prior wf context, they need their own axiom requires
(cascade).

### from_sorted_entries

This is a free function. Add `ensures result.spec_orderedtablestper_wf()` and prove it.
It already has axiom predicates in its requires.

## Steps

1. **Read** OrderedTableStPer.rs — understand trait constructor signatures and spec_wf definition
2. **Search** callers of `empty()`, `singleton()`, `tabulate()`, `from_sorted_entries()` across Chap43
3. **Add** axiom predicates to constructor requires in the trait
4. **Delete** axiom assumes from constructor impl bodies
5. **Fix** callers that now fail (add axiom requires or extract from wf)
6. **Add** wf ensures to `from_sorted_entries` and prove it
7. **Validate**, **rtt**, **ptt** — run sequentially, never in parallel

## Constraints

- Modify only `src/Chap43/OrderedTableStPer.rs` and callers within Chap43.
- Do NOT modify OrderedTableStEph.rs (Agent 1 owns that).
- Do NOT modify OrderedSetStEph.rs (Agent 4 owns that).
- Do NOT modify BSTTreapStEph.rs (Agent 3 owns that).
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially, never in parallel.
- Write report to `plans/agent2-round70-report.md` when done.
