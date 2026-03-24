# R69 Agent 5: Accept Standard Pattern Holes + Iterator Assumes

## Goal

Convert remaining standard-pattern holes to `accept()` where appropriate, and investigate
whether the iterator `assume(iter_invariant(self))` pattern can be proved.

## Task 1: Accept Iterator Assumes (if unprovable)

Two iterator assumes remain:
- `src/Chap43/OrderedTableStEph.rs` line ~3728: `assume(iter_invariant(self))`
- `src/Chap43/OrderedTableStPer.rs` line ~3305: `assume(iter_invariant(self))`

`iter_invariant` typically asserts that `self@.0` (position) and `self@.1` (elements)
are consistent with the underlying collection. In `Iterator::next`, after advancing
the iterator, the invariant should still hold.

**First**: Try to PROVE it. Read the `iter_invariant` spec function and the `next`
implementation. The invariant likely says something like:
```
0 <= pos <= elements.len() && elements == original_sequence
```
After `next` increments pos and returns `elements[old_pos]`, the invariant should
hold by simple arithmetic.

If provable: replace `assume` with the proof (assertions). This is -2 holes.
If not provable (e.g., the invariant references internal state that's opaque): convert
to `accept(iter_invariant(self))` using `crate::vstdplus::accept::accept`.

## Task 2: Survey All eq/clone Workaround Assumes

Scan `src/Chap43/` for any remaining eq/clone workaround assumes. For each one:
- If inside `Clone::clone` or `PartialEq::eq` body: convert `assume` to `accept`
- If in algorithmic code: flag as misplaced (Agent 4 handles those)

Check these files:
- OrderedTableStPer.rs — Agent 3 report mentioned 2 eq/clone
- OrderedTableMtEph.rs — 3 warnings
- OrderedTableMtPer.rs — 3 warnings
- AugOrderedTableStEph.rs — 1 warning

For warnings (already in Clone/Eq bodies), convert `assume` → `accept`.

## Task 3: fn_missing_wf_ensures

If Agent 1 or Agent 2 haven't already fixed these:
- OrderedTableStEph `from_sorted_entries` — add `ensures result.spec_orderedtablesteph_wf()`
- OrderedTableStPer `from_sorted_entries` — add `ensures result.spec_orderedtablestper_wf()`

Only fix if the proof goes through. `from_sorted_entries` builds a tree from sorted
pairs using `BSTParaStEph::from_vec`. The wf follows from `from_vec`'s ensures +
the axiom predicates. You may need to add the axiom predicates to `from_sorted_entries`'
requires first.

## Steps

1. **Read** the iterator implementations in OrderedTableStEph.rs and StPer.rs
2. **Read** `iter_invariant` spec fn — understand what it asserts
3. **Try to prove** iter_invariant in `next`. If it works, great. If not, accept.
4. **Scan** Chap43 for eq/clone assumes in Clone/Eq bodies, convert to accept
5. **Fix** from_sorted_entries wf ensures if not done by other agents
6. **Validate**, **rtt**, **ptt**

## Constraints

- Modify files in `src/Chap43/` only.
- Do NOT modify OrderedSetStEph.rs (Agent 4 owns that).
- Do NOT modify BSTTreapStEph.rs (Agent 4 owns that).
- Converting standard-pattern assumes to `accept` is explicitly authorized.
- Do NOT add new `assume` or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially.
