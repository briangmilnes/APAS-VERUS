# R72 Agent 4 Report: Chap43 Iterator Standard Fix + PTTs

## Scope

Two objectives from R72 assignment:
1. Fix Chap43 OrderedTable iterator standard (IntoIter wrapping + ghost protocol)
2. Create iterator PTTs for Chap43 modules

## Changes

| # | Chap | File | Change | Result |
|---|------|------|--------|--------|
| 1 | 43 | OrderedTableStEph.rs | Rewrote section 10: IntoIter wrapping + ghost protocol | -1 hole (assume in next removed) |
| 2 | 43 | OrderedTableStPer.rs | Rewrote section 10: IntoIter wrapping + ghost protocol | -1 hole (assume in next removed) |
| 3 | 43 | OrderedTableStEph.rs | Added `spec_orderedtablesteph_wf()` to singleton ensures | +7 holes (type axiom assumes) |
| 4 | 43 | OrderedSetStEph.rs | Rewrote section 10: full 10-component iterator standard | 0 hole change |
| 5 | 43 | ProveOrderedTableStPer.rs | Updated existing PTT: 4 tests match new View type | 4 pass |
| 6 | 43 | ProveOrderedTableStEph.rs | New PTT: 4 loop/for patterns | 4 pass |
| 7 | 43 | ProveOrderedSetStEph.rs | New PTT: 4 loop/for patterns | import fixed, needs ptt rerun |
| 8 | — | Cargo.toml | Added ProveOrderedSetStEph + ProveOrderedTableStEph entries | — |

## Part 1: Iterator Standard Rewrite (3 files)

### OrderedTableStEph.rs + OrderedTableStPer.rs

Replaced manual `ArraySeqStPerS`-backed iterator with `IntoIter<Pair<K, V>>` wrapping
(pattern from BalBinTreeStEph.rs). Changes per file:

1. **Struct**: `{sorted, pos, len}` → `{inner: IntoIter<Pair<K, V>>}`
2. **View**: `(int, Seq<(K::V, V::V)>)` → `(int, Seq<Pair<K, V>>)` (matches IntoIter's
   raw exec-type view)
3. **iter_invariant**: Simplified to `0 <= it@.0 <= it@.1.len()`
4. **next()**: Delegates to `self.inner.next()` — NO assume needed (was
   `assume(iter_invariant(self))`)
5. **Ghost iterator**: Full ForLoopGhostIterator protocol (6 spec fns)
6. **IntoIterator for &Self**: Added for StEph (was missing); updated for StPer

### OrderedSetStEph.rs

Rewrote iterator section with full 10-component standard wrapping `IntoIter<T>`:
custom iter struct, View, iter_invariant, Iterator::next (no assume), ghost iterator
struct, ForLoopGhostIteratorNew, ForLoopGhostIterator (6 spec fns), View for ghost
iterator, iter() method, IntoIterator for `&Self`.

## Part 2: Singleton wf Fix (OrderedTableStEph.rs)

**Why source was touched**: PTT chain `singleton → insert → iter` failed because
`insert()` requires `self.spec_orderedtablesteph_wf()` but `singleton()` didn't
ensure it. Without wf in singleton's ensures, the PTT test couldn't call insert on
the freshly-created table.

**Fix**: Added `tree.spec_orderedtablesteph_wf()` to singleton ensures (trait + impl).
Added 7 type axiom assumes in the proof body — identical to the pattern already in
OrderedTableStPer.rs singleton (lines 834-849):

```
assume(obeys_feq_fulls::<K, V>());
assume(obeys_feq_full::<Pair<K, V>>());
assume(vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>());
assume(view_ord_consistent::<Pair<K, V>>());
assume(spec_pair_key_determines_order::<K, V>());
assume(vstd::laws_cmp::obeys_cmp_spec::<K>());
assume(view_ord_consistent::<K>());
```

These 7 assumes are the same type-class axioms that every Chap43 ordered module needs.
StPer already had them. Net hole change: +7 (but these are structural type axioms, not
algorithmic holes).

## Part 3: PTT Results

| # | Chap | PTT File | Patterns | Status |
|---|------|----------|----------|--------|
| 1 | 43 | ProveOrderedTableStPer.rs | 4 (2 loop + 2 for) | 4 pass |
| 2 | 43 | ProveOrderedTableStEph.rs | 4 (2 loop + 2 for) | 4 pass |
| 3 | 43 | ProveOrderedSetStEph.rs | 4 (2 loop + 2 for) | import fixed, needs rerun |

### Skipped PTTs

- **OrderedSetStPer**: Module commented out in lib.rs (77 field-rename errors from
  AVLTreeSetStPer `elements` → `tree` rename). PTT file created then removed.
- **OrderedTableMtEph**: Iterator `next()` is outside `verus!` (line 808-820) with no
  ensures — not verifiable in PTT framework.

## Verification

- 4437 verified, 0 errors
- 2528 RTT pass
- PTT: 151 pass (ProveOrderedSetStEph needs rerun after import fix)

## Net Hole Change

- -2 holes: removed `assume(iter_invariant(self))` from next() in OrderedTableStEph + StPer
- +7 holes: type axiom assumes in OrderedTableStEph singleton (matches StPer pattern)
- Net: +5 holes in OrderedTableStEph (structural type axioms, not algorithmic)

## Remaining Work

- Run `ptt.sh` to confirm ProveOrderedSetStEph passes after import fix
- OrderedSetStPer needs field rename fix before it can be uncommented + tested
