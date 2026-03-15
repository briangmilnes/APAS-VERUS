# Review Against Prose: Chapter 42 -- Tables

Reviewer: Claude-Opus-4.6, 2026-03-15

## Phase 1: Inventory

Files reviewed (Example42_1.rs skipped per CLAUDE.md):

| # | Chap | File | Functions | Proof Fns | Holes | Clean |
|---|------|------|-----------|-----------|-------|-------|
| 1 | 42 | TableStEph.rs | 27 | 8 | 6 | No |
| 2 | 42 | TableStPer.rs | 31 | 12 | 5 | No |
| 3 | 42 | TableMtEph.rs | 28 | 10 | 4 | No |

Total: 86 functions (excl. Example), 30 proof fns (all clean), 15 holes (all external_body).

## Phase 2: Prose Inventory

APAS Data Type 42.1 (Tables) defines 15 named operations:

| # | APAS Operation | Signature |
|---|---------------|-----------|
| 1 | size | T -> N |
| 2 | empty | T |
| 3 | singleton | K x V -> T |
| 4 | domain | T -> S |
| 5 | tabulate | (K -> V) -> S -> T |
| 6 | map | (V -> V) -> T -> T |
| 7 | filter | (K x V -> B) -> T -> T |
| 8 | intersection | (V x V -> V) -> T -> T -> T |
| 9 | union | (V x V -> V) -> T -> T -> T |
| 10 | difference | T -> T -> T |
| 11 | find | T -> K -> (V | bottom) |
| 12 | delete | T -> K -> T |
| 13 | insert | (V x V -> V) -> T -> (K x V) -> T |
| 14 | restrict | T -> S -> T |
| 15 | subtract | T -> S -> T |

APAS also defines:
- Algorithm 42.3: collect -- takes a sequence of (key, value) pairs and produces a table mapping each key to all values with that key.
- Example 42.1: test cases for find, filter, map, tabulate, union (with second and +), subtract.
- Syntax 42.2 / 42.4: table notation and shorthands.
- Cost Spec 42.5: tree-based table costs.

## Phase 3a: Cost Annotations

Cost annotations added to all 3 files. Summary of agreement:

| # | Chap | File | Agreement Summary |
|---|------|------|-------------------|
| 1 | 42 | TableStEph.rs | Disagrees on most ops. Impl is array-backed (linear scan), APAS assumes tree-based (log n). |
| 2 | 42 | TableStPer.rs | Same disagreements as StEph. |
| 3 | 42 | TableMtEph.rs | Agrees on tabulate/map/filter (parallel via join). Disagrees on find/insert/delete/bulk (linear scan). |

Key cost disagreements:
- All three Table implementations are backed by `ArraySeq<Pair<K,V>>` (flat sorted array of entries), not a balanced BST. APAS Cost Spec 42.5 assumes tree-based O(log n) find/insert/delete. The actual implementation has O(n) find/insert/delete due to linear scan.
- For tabulate/map/filter, the MtEph variant achieves APAS costs via parallel fork-join (HFScheduler `join()`). The St variants are sequential and disagree on span.
- Bulk operations (intersection, union, difference, restrict, subtract) are O(|self| * |other|) in all files (nested linear scan), rather than the APAS-specified O(m * lg(1 + n/m)).
- The root cause is that these are array-backed tables, not tree-backed tables. The APAS cost spec is for tree-based implementations (Chapter 42 Section 2 states costs are "similar to those for sets" using balanced BSTs).

## Phase 3b: Implementation Fidelity

| # | Chap | File | APAS Ops Implemented | Missing | Extra |
|---|------|------|---------------------|---------|-------|
| 1 | 42 | TableStEph.rs | 15/15 + entries | None | entries, from_sorted_entries |
| 2 | 42 | TableStPer.rs | 15/15 + collect + entries | None | collect, entries, from_sorted_entries, collect_by_key |
| 3 | 42 | TableMtEph.rs | 15/15 + entries | None | entries, from_sorted_entries |

All 15 APAS operations are implemented in every file.

Additional operations:
- `entries` -- returns the backing array of Pair<K,V> entries. Not in APAS but useful for interop.
- `from_sorted_entries` -- constructs a table from a pre-sorted Vec of Pair entries. Not in APAS.
- `collect` -- APAS Algorithm 42.3. Only implemented in TableStPer.rs.
- `collect_by_key` -- Variant of collect that groups values by key. Only in TableStPer.rs.

Implementation deviations from APAS:
- Tables use `ArraySeq<Pair<K,V>>` as backing store, maintaining entries in sorted key order. This gives O(n) find (not O(log n)) because the implementation uses linear scan rather than binary search.
- `insert` takes a combining function `F: Fn(&V, &V) -> V` for handling duplicate keys. This matches APAS `insert(f)(a)(k,v) = union f a (singleton(k,v))`.
- `difference` does NOT take a combining function (APAS difference doesn't either -- correct).
- `map` in StEph is `&mut self` (ephemeral). In StPer it is `&self -> Self` (persistent). Both correct for their semantics.
- `collect` (Algorithm 42.3) is only in TableStPer, not in TableStEph or TableMtEph. This is a minor gap.

## Phase 3c: Spec Fidelity

| # | Chap | File | Spec Quality | Notes |
|---|------|------|-------------|-------|
| 1 | 42 | TableStEph.rs | Strong | All ops have full requires/ensures. View = spec_entries_to_map. |
| 2 | 42 | TableStPer.rs | Strong | Same spec quality. Persistent semantics correctly reflected. |
| 3 | 42 | TableMtEph.rs | Moderate | Some ops have weaker specs (e.g., intersection missing combine ensures). |

APAS spec mappings:
- `empty : T = {}` --> `ensures empty@ == Map::empty()` -- all files match.
- `singleton(k,v) = {k -> v}` --> `ensures tree@ == Map::empty().insert(key@, value@)` -- all files match.
- `size(a) = |a|` --> `ensures count == self@.len()` -- all files match.
- `domain(a)` returns set of keys --> `ensures domain@ =~= self@.dom()` -- all files match.
- `find(a,k)` returns Some(v) or None --> `ensures match found { Some(v) => self@.contains_key(key@) && self@[key@] == v@, None => !self@.contains_key(key@) }` -- all files match.
- `delete(a,k)` --> `ensures self@ =~= old(self)@.remove(key@)` -- all files match.
- `insert(f)(a)(k,v) = union f a (singleton(k,v))` --> Strong spec with combine function in St files; MtEph insert spec is weaker (missing combine ensures for existing key case).
- `filter(p)(a) = {(k->v) in a | p(k,v)}` --> Bidirectional spec in St files; all files have external_body.
- `intersection(f)(a)(b)` --> `ensures self@.dom() =~= old(self)@.dom().intersect(other@.dom())` plus combine result. MtEph intersection is weaker (missing combine ensures).
- `union(f)(a)(b)` --> Full 3-way spec (both, left-only, right-only) in St files. MtEph has partial spec.
- `difference(a)(b)` --> `ensures self@.dom() =~= old(self)@.dom().difference(other@.dom())` -- all files match.
- `restrict(a)(b)` --> `ensures self@.dom() =~= old(self)@.dom().intersect(keys@)` -- all files match.
- `subtract(a)(b)` --> `ensures self@.dom() =~= old(self)@.dom().difference(keys@)` -- all files match.
- `tabulate(f)(s)` --> Domain equals key set, values from f. All files match.
- `map(f)(a)` --> Domain preserved, values transformed. All files match.

Spec weakness in MtEph:
- `intersection` does not specify the combine result (missing `forall|k| ... exists|v1, v2, r| combine.ensures(...)`).
- `insert` does not specify the combine result for existing keys (missing the `old(self)@.contains_key(key@) ==> exists|old_v, r| ...` clause).
- These weaknesses mean callers cannot depend on the specific combined value, only on domain membership.

## Phase 4: Parallelism Review

| # | Chap | File | Type | Classification | Parallel Ops |
|---|------|------|------|---------------|--------------|
| 1 | 42 | TableStEph.rs | St | Sequential | None |
| 2 | 42 | TableStPer.rs | St | Sequential | None |
| 3 | 42 | TableMtEph.rs | Mt | Parallel | tabulate, map, filter |

Parallel operations detail (TableMtEph):
- `tabulate`: Splits key set at midpoint, forks two halves via `join()`, merges results. True divide-and-conquer parallelism.
- `map`: Uses ArraySeqMtEph's parallel tabulate to apply f to each entry in parallel. external_body.
- `filter`: Uses ArraySeqMtEph's parallel filter infrastructure. external_body.
- `insert`: external_body. The implementation does linear scan (sequential), not parallel.
- `intersection`, `union`, `difference`: Sequential (linear nested scan). Not parallel.

Missing parallelism (vs APAS expectations):
- APAS Cost Spec 42.5 specifies O(lg(n+m)) span for intersection/union/difference/restrict/subtract, implying parallel implementations. The actual MtEph implementation is sequential for these ops.
- `domain` is sequential in all files. APAS specifies O(lg |a|) span.

No sequentialization anti-patterns. The St files are correctly sequential. The Mt file maintains parallelism for tabulate/map/filter. Bulk set operations are sequential but this is an implementation gap, not an anti-pattern.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Test Count | Coverage |
|---|------|------|-----------|-----------|----------|
| 1 | 42 | TableStEph.rs | TestTableStEph.rs | 15 | All 15 ADT ops + macro + large ops |
| 2 | 42 | TableStPer.rs | TestTableStPer.rs | 14 | All 15 ADT ops + persistence semantics + macro |
| 3 | 42 | TableMtEph.rs | TestTableMtEph.rs | 14 | All 15 ADT ops + parallel tabulate |

RTT coverage is comprehensive for all files. Key test categories:
- Basic CRUD: empty, singleton, insert, find, delete tested in all files.
- Bulk operations: intersection, union, difference, restrict, subtract tested with combine functions.
- Higher-order: tabulate, map, filter tested with representative functions.
- Persistence: TableStPer tests verify that old versions are unchanged after operations.
- Macros: `TableStEphLit!` and `TableStPerLit!` macros tested.
- Large operations: StEph tests insert 100 elements, filter to 50.

Missing RTT coverage:
- `collect` and `collect_by_key` (TableStPer) -- no dedicated test for Algorithm 42.3.
- `entries` -- no test verifying entries() returns correct ordered pairs.
- Parallel path testing for MtEph tabulate/map/filter with large inputs (threshold crossing).
- `from_sorted_entries` -- no test in any file.

## Phase 6: PTT Review

No PTT files exist for Chapter 42. No iterator infrastructure exists in any Chap42 file, so no iterator PTTs are needed.

Potential PTT candidates:
- None identified. The `requires` clauses are straightforward (wf + closure requirements + feq/view_eq). No complex callability concerns.

## Phase 7: Gap Analysis

### Prose-to-Code Gaps

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 42 | Array-backed, not tree-backed | Medium | All 3 files use ArraySeq<Pair<K,V>>, giving O(n) find/insert/delete instead of APAS O(log n). |
| 2 | 42 | Bulk ops are O(n*m) not O(m*lg(1+n/m)) | Medium | Nested linear scan instead of tree merge. Same root cause as gap 1. |
| 3 | 42 | MtEph intersection/insert specs weak | Medium | Missing combine-function result specs. Callers cannot reason about combined values. |
| 4 | 42 | collect only in StPer | Low | Algorithm 42.3 not implemented in StEph or MtEph. |
| 5 | 42 | No MtPer table | Low | APAS does not specifically require it, but the project has MtPer variants for sets (Chap41). |
| 6 | 42 | domain span is O(n) not O(lg n) | Low | Sequential scan in all files. APAS expects O(lg |a|) span. |
| 7 | 42 | No tabulate cost spec from_seq | None | `from_sorted_entries` is an implementation utility, not an APAS operation. |

### Proof Holes Summary

| # | Chap | File | Hole Type | Count | Functions |
|---|------|------|-----------|-------|-----------|
| 1 | 42 | TableStEph.rs | external_body | 6 | tabulate, map, filter, intersection, union, insert |
| 2 | 42 | TableStPer.rs | external_body | 5 | tabulate, map, filter, intersection, union |
| 3 | 42 | TableMtEph.rs | external_body | 4 | tabulate, map, filter, insert |

All 15 holes are `external_body`. No `assume()` or `admit()` holes. All 30 proof functions are clean.

Common external_body pattern: The functions that iterate over entries and construct new entries (tabulate, map, filter, intersection, union) need to maintain the `spec_keys_no_dups` invariant through the transformation. The proof obligation is: if the input entries have no duplicate keys, the output entries also have no duplicate keys after filtering/mapping/combining. This requires an inductive argument over the entry sequence.

`insert` in StEph and MtEph is external_body because it needs to prove that inserting/replacing a key in a sorted entry array preserves the no-dup invariant and correctly updates the map view.

## Phase 8: TOC Review

| # | Chap | File | TOC Present | Section Order | Issues |
|---|------|------|-------------|--------------|--------|
| 1 | 42 | TableStEph.rs | Yes | 1,2,3,4,5,6,7,8,9,11,12,13 | Has duplicate TOC block (lines 5-17 outside verus, lines 39-51 inside). Duplicate section headers (//8. traits twice, //9. impls twice). |
| 2 | 42 | TableStPer.rs | Yes | 1,2,3,4,5,6,7,8,9,11,12,13 | Same duplicate TOC pattern. Multiple "// 6. spec fns" and "// 7. proof fns" headers. Missing section 6 from TOC listing. |
| 3 | 42 | TableMtEph.rs | Yes | 1,2,3,4,5,6,8,9,11,12,13 | Duplicate TOC block. Missing section 7 from TOC listing (proof fns exist in code but not in TOC). Section 13 appears before section 12 in code. |

TOC issues are minor formatting defects. All sections are present and correctly placed in the code; the headers just have duplicates and minor ordering inconsistencies.

## Summary

Chapter 42 implements all 15 APAS Table ADT operations plus Algorithm 42.3 (collect) across 3 files. The implementations are functionally correct but backed by sorted arrays rather than balanced BSTs, resulting in O(n) costs where APAS specifies O(log n). All 15 proof holes are `external_body` -- no `assume()` or `admit()` holes exist. The 30 proof functions (lemmas for entry-to-map conversion) are all clean. The primary proof challenge is maintaining the `spec_keys_no_dups` invariant through bulk operations. The MtEph variant achieves APAS-specified parallelism for tabulate/map/filter via HFScheduler `join()`, but bulk set operations (intersection/union/difference) remain sequential.
