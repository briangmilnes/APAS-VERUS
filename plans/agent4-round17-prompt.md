# Agent 4 — Round 17: Spec Audit Chap45 + Chap47

## Project State

103 holes, 4150 verified, 38 clean chapters, 8 holed.

## Mission: Fix Weak/Missing requires/ensures Against APAS Prose

This round is about **spec correctness**, not hole closure. Audit every trait
function's `requires`/`ensures` against the textbook definitions in `prompts/`.
Where specs are weak or missing, write the correct spec. If the corrected spec
breaks an existing proof body, add `#[verifier::external_body]`.

**Read `prompts/Chap45.txt` and `prompts/Chap47.txt` FIRST.**

## Chap45: Priority Queues (ADT 45.1)

Files:
- `src/Chap45/BinaryHeapPQ.rs`
- `src/Chap45/BalancedTreePQ.rs`
- `src/Chap45/LeftistHeapPQ.rs`

### ADT 45.1 Definitions

| Fn | Prose |
|----|-------|
| `empty()` | Creates PQ with 0 elements |
| `singleton(e)` | Creates PQ with just element e |
| `findMin(Q)` | Returns minimum element or None |
| `insert(Q, e)` | Adds element e |
| `deleteMin(Q)` | Removes minimum, returns (rest, min_or_None) |
| `meld(Q1, Q2)` | Union of elements from both queues |
| `fromSeq(S)` | Builds PQ from sequence S |

### LeftistHeapPQ.rs — REFERENCE (correct specs)

This file has **correct specs** including TotalOrder minimality proofs:
```
find_min ensures: size > 0 ==> forall|e| pq@.contains(e) ==> min <= e
```

Use LeftistHeapPQ as the reference for fixing the other two files.

### BinaryHeapPQ.rs — 3 weak specs

| # | Function | Problem | Correct ensures (copy from LeftistHeapPQ pattern) |
|---|----------|---------|--------------------------------------------------|
| 1 | `find_min` | WEAK: returns Some if non-empty, but no minimality | Add: `Some(min) ==> forall|e| self@.to_multiset().contains(e) ==> TotalOrder::le(min@, e@)` |
| 2 | `from_seq` | WEAK: `pq@.len() == seq@.len()` only, no content | Add: `pq@.to_multiset() =~= seq@.to_multiset()` (multiset preservation) |
| 3 | `to_seq` | PARTIAL: multiset equiv but sequence order undefined | Acceptable as-is (multiset equiv is the right spec for unordered PQ content). Mark as "correct". |

### BalancedTreePQ.rs — 5 weak specs

| # | Function | Problem | Action |
|---|----------|---------|--------|
| 1 | `find_min` | WEAK: no minimality assertion | Add TotalOrder::le universal quantifier |
| 2 | `find_max` | WEAK: no maximality assertion | Add `forall|e| ... ==> TotalOrder::le(e@, max@)` |
| 3 | `from_seq` | WEAK: length only, no content | Add multiset preservation |
| 4 | `extract_all_sorted` | WEAK: length only, no sortedness | Add: `Self::spec_sorted(sorted.seq@)` (spec fn should already exist) |
| 5 | `to_seq` | PARTIAL: see above | Acceptable |

### What multiset preservation looks like

For `from_seq`:
```
ensures
    pq.spec_*_wf(),
    pq@.len() == seq@.len(),
    pq@.to_multiset() =~= seq@.to_multiset(),
```

Check if `@` gives a `Seq` (then use `.to_multiset()`) or a `Multiset` directly
(then use `=~=` directly). Look at how LeftistHeapPQ handles it.

## Chap47: Hash Tables (ADT 47.1)

Files:
- `src/Chap47/ParaHashTableStEph.rs` (base trait)
- `src/Chap47/LinProbFlatHashTableStEph.rs`
- `src/Chap47/QuadProbFlatHashTableStEph.rs`
- `src/Chap47/DoubleHashFlatHashTableStEph.rs`
- `src/Chap47/ChainedHashTableStEph.rs` (or LinkedListChainedHashTableStEph.rs)
- `src/Chap47/FlatHashTable.rs` (if exists — shared flat trait)

### The Problem: 86% Weak Specs

Hash table operations have almost no postconditions on key storage. `insert` doesn't
ensure the key is retrievable. `lookup` doesn't ensure it returns the stored value.
`delete` doesn't ensure the key is removed. `resize` doesn't ensure content preservation.

### ADT 47.1 Correct Specs

The challenge: hash tables in this project use a `Vec<Option<(K,V)>>` or chain-based
storage. The logical view should be a `Map<K,V>`. Check if the hash table types have
a `View` impl — if so, specs should reference `self@` (the Map view).

If no `View` impl exists, the first step is understanding what the abstract state is.
Look at `ParaHashTableStEph` struct fields — is there a ghost field or spec function
that gives the logical map?

**Minimum correct specs for hash table operations:**

| Fn | Correct ensures |
|----|-----------------|
| `insert(k, v)` | `self@.contains_key(k@)` and `self@[k@] == v@` |
| `lookup(k)` | `Some(v) ==> self@.contains_key(k@) && v@ == self@[k@]`, `None ==> !self@.contains_key(k@)` |
| `delete(k)` | `!self@.contains_key(k@)` and other keys preserved |
| `resize(m')` | `self@ == old(self)@` (logical map unchanged) |
| `createTable(m)` | `self@ == Map::empty()` |

### Approach for Chap47

1. **First**: Read ParaHashTableStEph struct and trait. Understand what `@` gives you.
   If there's no View impl, you may need to define one (a `Map<K,V>` view of the
   hash table's contents). This is a structural change — if needed, note it in the
   audit and propose it, but don't implement without validating the approach.

2. **If View exists**: Strengthen ensures on insert/lookup/delete/resize per above.
   Add `external_body` where proofs break.

3. **If no View exists**: The spec situation is more fundamental — the type can't
   express its abstract state. Note this in the audit as "structural: needs View impl"
   and propose the View definition. Fix `probe()` bounds (already correct) and
   `createTable`/`loadAndSize` (already correct). Leave insert/lookup/delete for a
   follow-up round after the View is established.

4. **FlatHashTable trait**: Same analysis — check if there's an abstract state.

### probe() Functions — Already Correct

All 3 flat hash table variants have correct `probe()` ensures (`slot < current_size`).
Leave these alone.

## Deliverables

1. **`src/Chap45/analyses/spec-audit.md`** — per-function table for all 3 PQ files
2. **`src/Chap47/analyses/spec-audit.md`** — per-function table for all hash table files
3. Corrected trait ensures where possible
4. `external_body` on impl fns that can't prove the strengthened spec
5. Clean validation (0 errors)

## DO NOT TOUCH

- Chap41, Chap42 (Agent 1)
- Chap43 (Agent 2)
- Chap37, Chap38, Chap39 (Agent 3)
- Any Example files
- Mt/MtPer wrapper files

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **The prose is the source of truth.** Not what's easy to prove.
- **NO accept().** NO assume→accept.
- **Add `external_body` if you can't prove the correct spec.** Never weaken ensures.
- **DO NOT delete existing ensures.** Only add to them.
- Push to `agent4/ready`. Write `plans/agent4-round17-report.md`.

## Target

Audit all trait fns in Chap45 (3 files, ~36 fns) + Chap47 (5-6 files, ~37 fns).
Fix ~8 weak PQ specs + assess ~32 weak hash table specs. Chap45 fixes should be
straightforward (copy LeftistHeapPQ patterns). Chap47 depends on whether View
exists — if not, this round produces the audit + proposal, next round implements.
