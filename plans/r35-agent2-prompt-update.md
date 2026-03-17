# R35 Agent 2 Update: Ordering Operations via TotalOrder::cmp

## What you did

Good work on map and split_rank_key (-3 holes). The from_sorted_entries
and collect strengthening was solid infrastructure.

## What blocked you

You reported TotalOrder bridging blocks first_key, last_key, previous_key,
next_key, rank_key, select_key. Agent 1 solved this same problem in
OrderedSetStEph.rs. Here's the technique.

## TotalOrder bridging pattern (from OrderedSetStEph.rs)

Agent1 proved `first` (and last/previous/next) by:

1. Iterating entries with a loop, tracking the min/max
2. Using `TotalOrder::cmp(elem_ref, &best)` to get `core::cmp::Ordering`
3. Matching on `Ordering::Less / Equal / Greater`
4. Using `T::reflexive(x)` and `T::transitive(a, b, c)` in proof blocks
5. Loop invariant: `forall|j: int| 0 <= j < i ==> TotalOrder::le(best, vals[j])`
6. Post-loop: proving `forall|t: T| self@.contains(t@) ==> TotalOrder::le(best, t)`
   by connecting `self@.contains(t@)` → `elements@.to_set().contains(t@)` →
   `elements@.contains(t@)` → index in sequence → covered by loop invariant

The key insight: `TotalOrder::cmp` returns exec-level Ordering that Verus
can reason about. Match on the result to get the spec-level TotalOrder::le
facts. Use `T::reflexive` and `T::transitive` (spec lemmas from the
TotalOrder trait) to maintain the loop invariant.

Read `src/Chap43/OrderedSetStEph.rs` lines 369-460 for the full `first`
implementation. Then adapt for OrderedTableStEph's `first_key`.

### Adaptation for OrderedTable

OrderedTable stores `Pair<K, V>` entries. You iterate entries, extract
`.0` (the key), and compare keys with `TotalOrder::cmp`. The rest is
the same pattern.

```rust
fn first_key(&self) -> (first: Option<K>)
    where K: TotalOrder
{
    let entries = self.collect();
    let size = entries.length();
    if size == 0 { None }
    else {
        let mut best = entries.nth(0).0.clone();
        let mut i: usize = 1;
        while i < size
            invariant
                1 <= i <= size,
                // ... TotalOrder::le invariant over keys
            decreases size - i,
        {
            let k = entries.nth(i).0.clone();
            let c = TotalOrder::cmp(&k, &best);
            match c {
                core::cmp::Ordering::Less => {
                    proof { /* T::transitive for all previous */ }
                    best = k;
                },
                _ => {},
            }
            i += 1;
        }
        proof { /* connect loop invariant to self@.dom().contains */ }
        Some(best)
    }
}
```

## Also: Reference PartialOrd issue (get_key_range)

You reported `&pair.0 >= k1` fails because Verus doesn't support
`impl PartialOrd for &T`. Fix: dereference or clone. Use
`TotalOrder::cmp(&pair.0, k1)` instead of `&pair.0 >= k1`.

## Priority

1. first_key, last_key (min/max — simplest TotalOrder pattern)
2. previous_key, next_key (filtered min/max)
3. rank_key, select_key (counting/indexing)
4. get_key_range (fix &T comparison)
5. split_key (find split point)
6. filter (closure + key comparison)

Do StEph first, then mirror to StPer.

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read OrderedSetStEph.rs lines 369-460 FIRST for the TotalOrder pattern.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent2-round35-report.md`.
- Commit, push to `agent2/ready`.
