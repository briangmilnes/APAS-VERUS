# Review Against Prose: Chapter 41 -- Sets

Reviewer: Claude-Opus-4.6, 2026-03-15

## Phase 1: Inventory

Files reviewed (Example41_3.rs skipped per CLAUDE.md):

| # | Chap | File | Functions | Proof Fns | Holes | Clean |
|---|------|------|-----------|-----------|-------|-------|
| 1 | 41 | ArraySetStEph.rs | 21 | 7 | 1 | No |
| 2 | 41 | ArraySetEnumMtEph.rs | 21 | 7 | 0 | Yes |
| 3 | 41 | AVLTreeSetStEph.rs | 15 | 1 | 2 | No |
| 4 | 41 | AVLTreeSetStPer.rs | 14 | 0 | 1 | No |
| 5 | 41 | AVLTreeSetMtEph.rs | 15 | 0 | 9 | No |
| 6 | 41 | AVLTreeSetMtPer.rs | 16 | 0 | 9 | No |

Total: 102 functions (excl. Example), 15 proof fns, 22 holes.

## Phase 2: Prose Inventory

APAS Data Type 41.1 (Sets) defines 12 named operations:

| # | APAS Operation | Signature |
|---|---------------|-----------|
| 1 | size | S -> N |
| 2 | toSeq | S -> Seq |
| 3 | empty | S |
| 4 | singleton | U -> S |
| 5 | fromSeq | Seq -> S |
| 6 | filter | (U -> B) -> S -> S |
| 7 | intersection | S -> S -> S |
| 8 | difference | S -> S -> S |
| 9 | union | S -> S -> S |
| 10 | find | S -> U -> B |
| 11 | delete | S -> U -> S |
| 12 | insert | S -> U -> S |

APAS also defines:
- iterate / reduce in terms of toSeq (not directly implemented; available through iterators).
- Example 41.1: test cases for size, filter, find, union, toSeq, fromSeq.
- Example 41.3: fromSeq via iterate and via reduce.
- Remark: no map function for sets (maps can collapse distinct elements).

Cost Specifications:
- Cost Spec 41.3: Arrays for Enumerable Sets (universe = {0..u-1}, boolean array).
- Cost Spec 41.4: Tree Sets (balanced BST, comparison-based).

## Phase 3a: Cost Annotations

Cost annotations added to all files. Summary of agreement:

| # | Chap | File | Annotations | Agreement |
|---|------|------|-------------|-----------|
| 1 | 41 | ArraySetStEph.rs | Added (no APAS cost spec) | N/A -- unordered array set not in APAS |
| 2 | 41 | ArraySetEnumMtEph.rs | APAS 41.3 + Claude-Opus-4.6 | Agrees on all ops |
| 3 | 41 | AVLTreeSetStEph.rs | APAS 41.4 + claude-4-sonet | Agrees on work; span disagrees on filter, bulk ops |
| 4 | 41 | AVLTreeSetStPer.rs | APAS 41.4 + claude-4-sonet | Same as StEph |
| 5 | 41 | AVLTreeSetMtEph.rs | APAS 41.4 + claude-4-sonet | Bulk ops disagree (impl is Theta(m+n) not m*lg(1+n/m)) |
| 6 | 41 | AVLTreeSetMtPer.rs | APAS 41.4 + claude-4-sonet | Same as MtEph |

Key cost disagreements:
- The AVLTreeSet implementations back onto AVLTreeSeq (index-ordered AVL tree), not a value-ordered BST. This means insert/delete are O(n) (filter + rebuild), not O(log n). find uses binary search on the sorted inorder sequence, achieving O(log n). Bulk operations (intersection, difference, union) iterate over one set and binary-search in the other, giving O(min * log(max)) work but O(n) span (not O(log n) as APAS specifies).
- ArraySetStEph is an unordered seq-backed set with linear-scan membership. APAS does not define a cost spec for this representation -- it is a simple utility set, not a performance implementation.
- ArraySetEnumMtEph matches APAS Cost Spec 41.3 closely. The implementation uses bit arrays (Vec<u64>) and achieves the O(u/w) word-level parallelism for intersection/difference/union. filter is the only parallel operation (fork per element via HFScheduler join).

## Phase 3b: Implementation Fidelity

| # | Chap | File | APAS Op | Impl Fidelity | Notes |
|---|------|------|---------|---------------|-------|
| 1 | 41 | ArraySetStEph.rs | all 12 | Correct | Linear scan; no cost spec from APAS |
| 2 | 41 | ArraySetEnumMtEph.rs | all 12 | Correct | Bit array for enumerable universe |
| 3 | 41 | AVLTreeSetStEph.rs | all 12 | Correct | Backed by AVLTreeSeqStEph |
| 4 | 41 | AVLTreeSetStPer.rs | all 12 | Correct | Persistent variant |
| 5 | 41 | AVLTreeSetMtEph.rs | all 12 | Correct | Parallel filter/intersection/union/difference |
| 6 | 41 | AVLTreeSetMtPer.rs | all 12 | Correct | Persistent + parallel |

All 12 APAS operations are implemented in every file. No operations are missing.

Additional operations beyond APAS:
- `from_seq` -- APAS calls this `fromSeq`. Implemented in all files.
- `to_seq` -- APAS calls this `toSeq`. Implemented in all files.
- `iter` -- Iterator support (MtEph only). APAS defines iterate/reduce in terms of toSeq.
- `default` -- Rust Default trait. Returns empty set.
- `eq` / `partial_cmp` / `cmp` -- Rust comparison traits (MtPer includes Ord for ordered table use).

Implementation deviations:
- AVLTreeSet files back onto AVLTreeSeq (an index-ordered tree, not a key-ordered BST). This means the actual costs differ from APAS Cost Spec 41.4 for insert/delete/bulk ops. The textbook assumes a key-ordered BST; the implementation uses filter+rebuild for insert/delete (O(n) not O(log n)).
- ArraySetEnumMtEph takes a universe size `u` parameter in `new/empty/singleton/from_seq`. APAS Cost Spec 41.3 implicitly assumes a fixed universe. The implementation is faithful to the cost model.
- ArraySetEnumMtEph uses `PredVal<usize>` (value-based predicate) for filter, not `PredMt`. This is because the values are plain `usize`, not Mt-bounded types.

## Phase 3c: Spec Fidelity

| # | Chap | File | Spec Quality | Notes |
|---|------|------|-------------|-------|
| 1 | 41 | ArraySetStEph.rs | Strong | All ADT ops have correct requires/ensures matching APAS |
| 2 | 41 | ArraySetEnumMtEph.rs | Strong | Universe-bounded specs. filter spec is weaker (subset_of only, no bidirectional) |
| 3 | 41 | AVLTreeSetStEph.rs | Strong | Full bidirectional filter spec. wf includes no_duplicates + finite |
| 4 | 41 | AVLTreeSetStPer.rs | Strong | Same quality as StEph |
| 5 | 41 | AVLTreeSetMtEph.rs | Partial | Bulk ops (intersection/union/difference) missing ensures on correctness (only have subset/superset, not equality) |
| 6 | 41 | AVLTreeSetMtPer.rs | Partial | Same weakness as MtEph; from_seq missing from_seq ensures |

APAS spec mappings:
- `empty : S = {}` --> `ensures empty@ == Set::empty()` -- all files match.
- `singleton(x) : S = {x}` --> `ensures tree@ == Set::empty().insert(x@)` -- all files match.
- `find(a,x) : B = x in a` --> `ensures found == self@.contains(x@)` -- all files match.
- `insert(a,x) : S = a union {x}` --> `ensures self@ == old(self)@.insert(x@)` -- all files match.
- `delete(a,x) : S = a \ {x}` --> `ensures self@ == old(self)@.remove(x@)` -- all files match.
- `intersection(a,b) = a intersect b` --> `ensures common@ == self@.intersect(other@)` -- St files match; Mt files have weaker specs.
- `difference(a,b) = a \ b` --> `ensures remaining@ == self@.difference(other@)` -- St files match; Mt files weaker.
- `union(a,b) = a union b` --> `ensures combined@ == self@.union(other@)` -- St files match; Mt files weaker.
- `filter(f,a) = {x in a | f(x)}` --> bidirectional spec in St files; external_body in most.
- `fromSeq(a) = range(a)` --> `ensures constructed@ =~= seq@.to_set()` -- all files match.
- `size(a) = |a|` --> `ensures count == self@.len()` -- all files match.
- `toSeq(a)` --> `ensures seq@.to_set() =~= self@` -- all files match.

## Phase 4: Parallelism Review

| # | Chap | File | Type | Classification | Notes |
|---|------|------|------|---------------|-------|
| 1 | 41 | ArraySetStEph.rs | St | Sequential | All ops sequential. Expected. |
| 2 | 41 | ArraySetEnumMtEph.rs | Mt | Parallel | filter uses join() for parallelism |
| 3 | 41 | AVLTreeSetStEph.rs | St | Sequential | All ops sequential. Expected. |
| 4 | 41 | AVLTreeSetStPer.rs | St | Sequential | All ops sequential. Expected. |
| 5 | 41 | AVLTreeSetMtEph.rs | Mt | Parallel | filter/intersection/union/difference use ParaPair! |
| 6 | 41 | AVLTreeSetMtPer.rs | Mt | Parallel | Same parallel ops as MtEph |

Parallel operations detail:
- ArraySetEnumMtEph: Only `filter` is parallel (forks per element evaluation). intersection/union/difference are sequential (bitwise word ops, O(u/w) -- fast enough without threads).
- AVLTreeSetMtEph: `filter`, `intersection`, `union`, `difference` use `ParaPair!` macro for divide-and-conquer parallelism. `find`, `delete`, `insert` are sequential (O(log n) tree traversal). All parallel ops are currently `external_body`.
- AVLTreeSetMtPer: Same parallel structure as MtEph. Has `SEQUENTIAL_CUTOFF = 1` -- no threshold optimization (correct per CLAUDE.md).

No sequentialization anti-patterns found. Mt modules maintain parallelism.

## Phase 5: Runtime Test Review

| # | Chap | File | Test File | Test Count | Coverage |
|---|------|------|-----------|-----------|----------|
| 1 | 41 | ArraySetStEph.rs | TestArraySetStEph.rs | 7 | size, empty, singleton, insert, find, delete, from_seq, filter, intersection, union, difference, to_seq, macro |
| 2 | 41 | ArraySetEnumMtEph.rs | TestArraySetEnumMtEph.rs | 9 | new, size, insert, find, delete, filter, intersection, union, difference, thread safety, universe bounds |
| 3 | 41 | AVLTreeSetStEph.rs | TestAVLTreeSetStEph.rs | 25+ | Comprehensive: all ops, strings, large sets, edge cases |
| 4 | 41 | AVLTreeSetStPer.rs | TestAVLTreeSetStPer.rs | 10 | Persistence semantics, all ADT ops |
| 5 | 41 | AVLTreeSetMtEph.rs | TestAVLTreeSetMtEph.rs | 30+ | Parallel path tests for filter/union/intersection with large sets |
| 6 | 41 | AVLTreeSetMtPer.rs | TestAVLTreeSetMtPer.rs | Many | Several DISABLED due to thread explosion; gated behind `all_chapters` |

RTT coverage is comprehensive for all files. MtPer tests have thread explosion issues from recursive ParaPair! calls -- several tests are disabled.

Missing RTT coverage:
- ArraySetStEph: No test for `to_seq` round-trip ordering guarantee.
- ArraySetEnumMtEph: No test for `to_seq` or `from_seq` round-trip.
- AVLTreeSetMtPer: Most parallel bulk operation tests are disabled.

## Phase 6: PTT Review

| # | Chap | File | PTT File | Tests | Coverage |
|---|------|------|----------|-------|----------|
| 1 | 41 | AVLTreeSetMtEph.rs | ProveAVLTreeSetMtEph.rs | 3 | Iterator verification: loop-borrow-iter, for-borrow-iter, for-borrow-into |

Only AVLTreeSetMtEph has PTTs, and only for iterator verification. No other files have PTTs.

Missing PTTs:
- No PTT for complex requires (e.g., filter's closure requirements, intersection of two sets with wf preconditions). These would be useful for confidence in callability.

## Phase 7: Gap Analysis

### Prose-to-Code Gaps

| # | Chap | Gap | Severity | Notes |
|---|------|-----|----------|-------|
| 1 | 41 | ArraySetStEph cost asymptotic gap | Low | APAS has no cost spec for unordered array sets. Implementation is O(n) for all ops. Not a bug -- just a simple utility set. |
| 2 | 41 | AVLTreeSet insert/delete are O(n) not O(log n) | Medium | Backed by index-ordered AVLTreeSeq, not a value-ordered BST. APAS Cost Spec 41.4 assumes O(log n). |
| 3 | 41 | AVLTreeSet bulk ops are O(m+n) not O(m*lg(1+n/m)) | Medium | Same root cause as gap 2. The tree is not split by value. |
| 4 | 41 | MtEph/MtPer bulk op specs are weaker than APAS | Medium | intersection/union/difference specs don't prove `a intersect b` etc. -- only subset/superset bounds. |
| 5 | 41 | filter is external_body in most files | Medium | ArraySetStEph, AVLTreeSetStEph, AVLTreeSetStPer, AVLTreeSetMtEph, AVLTreeSetMtPer all have external_body on filter. Only ArraySetEnumMtEph has a proved filter. |
| 6 | 41 | MtPer thread explosion | Low | Recursive ParaPair! creates exponential threads. Tests disabled. Runtime issue, not a verification gap. |
| 7 | 41 | ArraySetEnumMtEph filter spec is weak | Low | Only ensures `subset_of(self@)`. Missing bidirectional spec (forall v: self@.contains(v) && f(v) ==> filtered@.contains(v)). |
| 8 | 41 | No map function | None | APAS explicitly excludes map from sets (Remark after Example 41.1). Correct omission. |

### Proof Holes Summary

| # | Chap | File | Hole Type | Count | Description |
|---|------|------|-----------|-------|-------------|
| 1 | 41 | ArraySetStEph.rs | external_body | 1 | filter |
| 2 | 41 | AVLTreeSetStEph.rs | assume | 1 | insert (len bound) |
| 3 | 41 | AVLTreeSetStEph.rs | external_body | 1 | filter |
| 4 | 41 | AVLTreeSetStPer.rs | external_body | 1 | filter |
| 5 | 41 | AVLTreeSetMtEph.rs | assume | 2 | iterator infrastructure |
| 6 | 41 | AVLTreeSetMtEph.rs | unsafe impl | 2 | Send/Sync markers |
| 7 | 41 | AVLTreeSetMtEph.rs | external_body | 5 | filter, intersection, union, difference, find |
| 8 | 41 | AVLTreeSetMtPer.rs | assume | 1 | eq body |
| 9 | 41 | AVLTreeSetMtPer.rs | external_body | 8 | from_seq, filter, intersection, difference, union, find, delete, insert |

## Phase 8: TOC Review

| # | Chap | File | TOC Present | Section Order | Issues |
|---|------|------|-------------|--------------|--------|
| 1 | 41 | ArraySetStEph.rs | Yes | 1,2,3,4,5,7,8,9,11,12,13 | Clean |
| 2 | 41 | ArraySetEnumMtEph.rs | Yes | 1,2,3,4,5,6,7,8,9,11,12,13 | Clean |
| 3 | 41 | AVLTreeSetStEph.rs | Yes | 1,2,3,4,5,8,9,11,12,13 | Duplicate "// 8. traits" and "// 9. impls" headers; "// 5. view impls" appears after section 9 start |
| 4 | 41 | AVLTreeSetStPer.rs | Yes | Standard ordering | Clean |
| 5 | 41 | AVLTreeSetMtEph.rs | Yes | Standard ordering | Clean |
| 6 | 41 | AVLTreeSetMtPer.rs | Yes | Standard ordering | Clean |

TOC issue in AVLTreeSetStEph.rs: Duplicate section headers (two "// 8. traits" at lines 84/86, two "// 9. impls" at lines 184/194, and a misplaced "// 5. view impls" at line 186). Minor formatting issue, does not affect verification.

## Summary

Chapter 41 implements all 12 APAS Set ADT operations across 6 files (2 representations x 3 persistence/threading variants). The ArraySetEnumMtEph (bit array for enumerable sets) is the cleanest file with 0 proof holes and full verification. The AVLTreeSet files have moderate proof hole counts, primarily `external_body` on filter and bulk operations. The main architectural deviation from APAS is that AVLTreeSet backs onto an index-ordered AVLTreeSeq rather than a value-ordered BST, resulting in O(n) insert/delete instead of O(log n). This is a known implementation choice, not a verification gap.
