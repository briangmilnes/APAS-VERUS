<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 42: Tables — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
**Prose source:** `prompts/Chap42.txt` (Data Type 42.1, Algorithm 42.3, Cost Specification 42.5, Example 42.1)

## Phase 2: Semantic Fidelity

Data Type 42.1 specifies 15 operations. This phase checks whether each implementation faithfully captures the mathematical definition from the prose.

| # | APAS Operation | Prose Definition | StEph | StPer | MtEph | Fidelity Notes |
|---|---|---|:---:|:---:|:---:|---|
| 1 | `size(a)` | `\|a\|` | Yes | Yes | Yes | All return entry count via `entries.length()`. |
| 2 | `empty` | `∅` | Yes | Yes | Yes | All return table with empty backing array. |
| 3 | `singleton(k,v)` | `{k→v}` | Yes | Yes | Yes | All create single-entry table. |
| 4 | `domain(a)` | Set of all keys in `a` | Yes | Yes | Yes | All extract keys into `ArraySetStEph<K>`. Correct semantics. |
| 5 | `tabulate(f)(a:S)` | `{k→f(k) : k∈a}` | Yes | Yes | Yes | All take `keys: &ArraySetStEph<K>`, apply `f` to each key. Input is a set as required. |
| 6 | `map(f)(a)` | `{k→f(v) : (k→v)∈a}` | Yes | Yes | Yes | All apply `f` to values, preserving keys. |
| 7 | `filter(f)(a)` | `{(k→v)∈a \| f(k,v)}` | Yes | Yes | Yes | All take `f: Fn(&K, &V) -> B` matching the `K×V→B` signature. |
| 8 | `intersection(f)(a)(b)` | `{k→f(find a k, find b k) : k∈(dom(a)∩dom(b))}` | Yes | Yes | Yes | All produce entries whose keys appear in both tables, combined via `f`. |
| 9 | `union(f)(a)(b)` | `(intersection f a b) ∪ (diff a b) ∪ (diff b a)` | Equiv | Yes | Equiv | StPer follows the APAS decomposition (intersection + 2 differences). StEph and MtEph use a direct merge, which is semantically equivalent but structurally different from the prose. |
| 10 | `difference(a)(b)` | `{(k→v)∈a \| k∉dom(b)}` | Yes | Yes | Yes | All retain entries from `a` whose key is absent from `b`. |
| 11 | `find(a)(k)` | `v` if `(k→v)∈a`, else `⊥` | Yes | Yes | Yes | All use binary search on sorted entries. `Option<V>` models `V∪⊥`. |
| 12 | `delete(a)(k)` | `{(k'→v')∈a \| k≠k'}` | Yes | Yes | Yes | All filter out the matching key. |
| 13 | `insert(f)(a)(k,v)` | `union f a (singleton(k,v))` | Yes | Yes | Yes | All include a combine function `f` for duplicate keys, matching the APAS signature. |
| 14 | `restrict(a)(b:S)` | `{k→v∈a \| k∈b}` | Yes | Yes | Yes | All take `keys: &ArraySetStEph<K>` and filter entries whose key appears in the set. |
| 15 | `subtract(a)(b:S)` | `{(k→v)∈a \| k∉b}` | Yes | Yes | Yes | All take `keys: &ArraySetStEph<K>` and filter entries whose key does NOT appear in the set. |

**Interface fidelity observations:**

| # | Observation | Severity | Notes |
|---|---|---|---|
| 1 | StEph `map`/`filter` take `&mut self`; StPer returns `Self` | Correct | Ephemeral vs persistent semantics are properly distinguished. |
| 2 | `insert` includes combine function in all variants | Correct | Matches APAS `insert(f)(a)(k,v) = union f a (singleton(k,v))`. |
| 3 | `find` returns `Option<V>` | Correct | Rust `Option` models APAS `V∪⊥`. |
| 4 | `Pair<K,V>` used instead of tuples | Low | Prose uses `K×V`; implementation uses `Pair(K,V)` from `Types.rs`. Semantically identical. |
| 5 | `restrict`/`subtract` take `ArraySetStEph<K>` | Correct | Prose takes a set `S`; ArraySetStEph is the project's set implementation. |

**Semantic fidelity: 15/15 operations are semantically faithful to the prose.**

## Phase 3: Algorithm Coverage

| # | APAS Algorithm | Implemented | File | Fidelity |
|---|---|:---:|---|---|
| 1 | Algorithm 42.3 (collect) | No | — | **Not implemented.** The APAS collect takes a *sequence* of key-value pairs and groups values by key: `collect a = Seq.reduce (Table.union Seq.append) {} ⟨{k→⟨v⟩} : (k,v)∈a⟩`. This produces a `Table<K, Seq<V>>`. The implementations provide `collect(&self) -> Seq<Pair<K,V>>` which merely returns the table's entries — a completely different operation with a different type signature. |

**Extra operations not in Data Type 42.1:**

| # | Operation | Files | Notes |
|---|---|---|---|
| 1 | `collect(&self) -> Seq<Pair<K,V>>` | All 3 | Returns flat entries. Not the APAS Algorithm 42.3 collect. Better named `entries()` or `to_seq()`. |
| 2 | `from_sorted_entries(Vec<Pair<K,V>>)` | All 3 | Constructor helper. Not in ADT but useful for macros. |
| 3 | `TableStEphLit!` macro | TableStEph.rs | Literal syntax. Convenience, not in ADT. |
| 4 | `TableStPerLit!` macro | TableStPer.rs | Literal syntax. Convenience, not in ADT. |
| 5 | `TableMtEphLit!` macro | TableMtEph.rs | Literal syntax. Convenience, not in ADT. |

## Phase 4: Cost Annotation Audit

### Data Structure vs Cost Model

All three implementations use a **sorted array** (`ArraySeqStEphS` / `ArraySeqStPerS` / `ArraySeqMtEphS` of `Pair<K,V>`) as the backing store. The APAS Cost Specification 42.5 assumes a **balanced BST** (or equivalent log-depth structure), which gives O(lg n) for single-element operations. The sorted-array choice fundamentally limits achievable costs for insert and delete.

### TableStEph.rs — Single-threaded ephemeral

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match | Notes |
|---|---|---|---|---|---|:---:|---|
| 1 | `size` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 2 | `empty` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 3 | `singleton` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 4 | `domain` | O(\|a\|) | O(lg\|a\|) | O(n²) | O(n²) | No | Sequential insertion into `ArraySetStEph` is O(n) per insert × n keys. |
| 5 | `tabulate` | O(\|s\|·W(f)) | O(lg\|s\|+S(f)) | O(\|s\|·W(f)+\|s\|log\|s\|) | O(\|s\|·(W(f)+log\|s\|)) | Partial | Work matches (amortized). Span sequential rather than logarithmic. Extra O(n log n) sort at end. |
| 6 | `map` | O(Σ W(f(v))) | O(lg\|a\|+max S(f(v))) | O(n·W(f)) | O(n·S(f)) | Partial | Work equivalent. Span sequential (O(n)) rather than O(lg n). |
| 7 | `filter` | O(Σ W(p(k,v))) | O(lg\|a\|+max S(p)) | O(n·W(p)) | O(n·S(p)) | Partial | Same as map: work matches, span sequential. |
| 8 | `intersection` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | Merge is O(m+n); APAS is O(m·lg(1+n/m)) which is better when m≪n. |
| 9 | `union` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | Direct merge. Same issue as intersection. |
| 10 | `difference` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | Same issue. |
| 11 | `find` | O(lg\|a\|) | O(lg\|a\|) | O(log n) | O(log n) | Yes | Binary search. |
| 12 | `delete` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No | Linear scan to rebuild array. |
| 13 | `insert` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No | Linear rebuild + sort. |
| 14 | `restrict` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No | Per-entry binary search in the key set. |
| 15 | `subtract` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No | Same as restrict. |

**StEph cost match: 4/15 (size, empty, singleton, find).**

### TableStPer.rs — Single-threaded persistent

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match | Notes |
|---|---|---|---|---|---|:---:|---|
| 1 | `size` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 2 | `empty` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 3 | `singleton` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 4 | `domain` | O(\|a\|) | O(lg\|a\|) | O(n) | O(n) | Partial | Collects keys via `.map().collect()` then constructs `ArraySetStEph::from_seq`. Work is O(n) (correct). Span is O(n) not O(lg n). |
| 5 | `tabulate` | O(\|s\|·W(f)) | O(lg\|s\|+S(f)) | O(\|s\|·W(f)) | O(\|s\|·W(f)) | Partial | Uses `ArraySeqStPerS::tabulate`. Work matches. Span depends on tabulate implementation but sequential here. |
| 6 | `map` | O(Σ W(f(v))) | O(lg\|a\|+max S(f(v))) | O(n·W(f)) | O(n·W(f)) | Partial | Uses `tabulate`. Work matches. Span sequential. |
| 7 | `filter` | O(Σ W(p(k,v))) | O(lg\|a\|+max S(p)) | O(n·W(p)) | O(n·W(p)) | Partial | Sequential loop. Work matches. Span sequential. |
| 8 | `intersection` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | Sequential merge. O(m+n) is worse than O(m·lg(1+n/m)) when m≪n. The trait annotation claims APAS cost but the implementation is a simple merge. |
| 9 | `union` | O(m·lg(1+n/m)) | O(lg(n+m)) | O((m+n)log(m+n)) | O((m+n)log(m+n)) | No | Calls intersection + 2 differences (each O(m+n)), then concatenates and re-sorts. Total is dominated by the O((m+n)log(m+n)) sort. Actually WORSE than a direct merge. |
| 10 | `difference` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | Sequential merge. Same issue as intersection. |
| 11 | `find` | O(lg\|a\|) | O(lg\|a\|) | O(log n) | O(log n) | Yes | Binary search. |
| 12 | `delete` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No | Linear scan filtering. |
| 13 | `insert` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n) | No | Linear scan to find insertion point + delete + tabulate rebuild. |
| 14 | `restrict` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No | Per-entry lookup in key set. |
| 15 | `subtract` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n·log m) | No | Same as restrict. |

**StPer cost match: 4/15 (size, empty, singleton, find).**

Note: The trait annotations in StPer claim `claude-4-sonet: Work Θ(m log(1 + n/m))` for intersection, union, and difference, but the actual implementations are O(m+n) merge-based. The self-annotations are incorrect.

### TableMtEph.rs — Multi-threaded ephemeral

| # | Operation | APAS Work | APAS Span | Actual Work | Actual Span | Match | Notes |
|---|---|---|---|---|---|:---:|---|
| 1 | `size` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 2 | `empty` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 3 | `singleton` | O(1) | O(1) | O(1) | O(1) | Yes | |
| 4 | `domain` | O(\|a\|) | O(lg\|a\|) | O(n²) | O(n) | No | Parallel key extraction via spawn/join, but sequential insert into `ArraySetStEph` is still O(n²). Span improved by parallelism but work dominated by set insertion. |
| 5 | `tabulate` | O(\|s\|·W(f)) | O(lg\|s\|+S(f)) | O(\|s\|·W(f)+\|s\|log\|s\|) | O(\|s\|/2·W(f)+\|s\|log\|s\|) | Partial | Parallel via spawn/join (2-way). Extra O(n log n) sort at end. Only 2-way parallelism rather than full log-depth recursion. |
| 6 | `map` | O(Σ W(f(v))) | O(lg\|a\|+max S(f(v))) | O(n·W(f)) | O(n/2·W(f)) | Partial | 2-way spawn/join. Span halved but not logarithmic. |
| 7 | `filter` | O(Σ W(p(k,v))) | O(lg\|a\|+max S(p)) | O(n·W(p)) | O(n/2·W(p)) | Partial | Same as map: 2-way split, not full divide-and-conquer. |
| 8 | `intersection` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | **Sequential merge** despite being in the Mt file. No parallelism at all. |
| 9 | `union` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | **Sequential merge.** Same issue. |
| 10 | `difference` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(m+n) | O(m+n) | No | **Sequential merge.** Same issue. |
| 11 | `find` | O(lg\|a\|) | O(lg\|a\|) | O(log n) | O(log n) | Yes | Binary search. |
| 12 | `delete` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n/2) | No | Parallel filter via 2-way spawn/join. Work is O(n), not O(lg n). |
| 13 | `insert` | O(lg\|a\|) | O(lg\|a\|) | O(n) | O(n/2) | No | Parallel tabulate + sort for update. Work is O(n), not O(lg n). |
| 14 | `restrict` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n/2·log m) | No | 2-way parallel filter with per-entry set lookup. |
| 15 | `subtract` | O(m·lg(1+n/m)) | O(lg(n+m)) | O(n·log m) | O(n/2·log m) | No | Same as restrict. |

**MtEph cost match: 4/15 (size, empty, singleton, find).**

**Critical observation:** MtEph's intersection, union, and difference are entirely sequential despite being in the multi-threaded file. Only map, filter, tabulate, delete, restrict, and subtract use spawn/join parallelism, and even those use only a single 2-way split rather than recursive divide-and-conquer to achieve O(lg n) span.

### Cost Annotation Accuracy

The trait declarations contain both "APAS" and "claude-4-sonet" cost annotations. Several `claude-4-sonet` annotations are incorrect relative to the actual implementation:

| # | File | Operation | Annotated Cost | Actual Cost | Issue |
|---|---|---|---|---|---|
| 1 | StPer | `intersection` | `Work Θ(m log(1+n/m))` | `Work O(m+n)` | Annotation claims better-than-merge cost but impl is merge-based. |
| 2 | StPer | `union` | `Work Θ(m log(1+n/m))` | `Work O((m+n)log(m+n))` | Annotation wrong; impl does intersection + 2 diffs + sort. |
| 3 | StPer | `difference` | `Work Θ(m log(1+n/m))` | `Work O(m+n)` | Same merge mismatch. |
| 4 | MtEph | `intersection` | `Span Θ(log(m+n))` | `Span O(m+n)` | Sequential impl despite Mt file; no parallelism. |
| 5 | MtEph | `union` | `Span Θ(log(m+n))` | `Span O(m+n)` | Same sequential issue. |
| 6 | MtEph | `difference` | `Span Θ(log(m+n))` | `Span O(m+n)` | Same sequential issue. |

## Phase 5: Structural Review

### Data Representation

All three variants use a sorted array of `Pair<K,V>` entries. The APAS cost specification (42.5) is designed for a balanced BST (like a treap or AVL tree), which Chapter 43 provides. The sorted-array representation achieves O(log n) for `find` (binary search) but O(n) for `insert` and `delete` (array rebuild). This is the root cause of most cost mismatches.

### Ephemeral vs Persistent Semantics

| # | Property | StEph | StPer | MtEph | Correct |
|---|---|---|---|---|:---:|
| 1 | Mutating operations modify self | `&mut self` | Returns `Self` | `&mut self` | Yes |
| 2 | Old versions preserved | No | Yes | No | Yes |
| 3 | Test verifies semantics | `test_table_ephemeral_semantics` | `test_table_persistence` | `test_table_ephemeral_semantics` | Yes |

### Union Decomposition

The APAS defines union as: `union(f)(a)(b) = (intersection f a b) ∪ (difference a b) ∪ (difference b a)`.

| # | Variant | Follows APAS decomposition? | Notes |
|---|---|:---:|---|
| 1 | StPer | Yes | Calls `self.intersection()`, `self.difference()`, `other.difference()`, then concatenates and re-sorts. Structurally faithful but the re-sort adds O((m+n)log(m+n)) overhead. |
| 2 | StEph | No | Direct 3-way merge in a single pass. Semantically equivalent, algorithmically different. |
| 3 | MtEph | No | Direct 3-way merge, same as StEph. |

### Insert Decomposition

The APAS defines insert as: `insert(f)(a)(k,v) = union f a (singleton(k,v))`.

None of the implementations literally decompose insert into union+singleton. All three implement insert directly by checking for the existing key and combining or appending. This is a standard optimization — calling full union for a single-element insert would be wasteful. Semantically equivalent.

### Module Structure

| # | Property | TableStEph | TableStPer | TableMtEph | Example42_1 |
|---|---|:---:|:---:|:---:|:---:|
| 1 | Module header | Yes | Yes | Yes | Yes |
| 2 | TOC headers | No | No | No | No |
| 3 | `verus!` blocks | No | No | No | No |
| 4 | Trait definition | Yes | Yes | Yes | Yes (empty) |
| 5 | `Debug` impl | derive | derive | manual | — |
| 6 | `Clone` impl | derive | derive | derive | — |
| 7 | `PartialEq` impl | derive | derive | derive | — |
| 8 | Literal macro | Yes | Yes | Yes | — |

### Type Bounds

| # | Variant | Key bound | Value bound | Notes |
|---|---|---|---|---|
| 1 | StEph | `K: StT + Ord` | `V: StT` | `StT` is the project's single-threaded trait alias. |
| 2 | StPer | `K: StT + Ord` | `V: StT` | Same. |
| 3 | MtEph | `K: MtKey` | `V: MtVal` | `MtKey`/`MtVal` are the project's thread-safe trait aliases (includes `Send + Sync + 'static`). |

## Phase 6: Verus Verification Status

**No Verus code exists in Chapter 42.** All 4 source files are plain Rust without `verus!` blocks. This means:

| # | Verification Artifact | Present | Count |
|---|---|:---:|:---:|
| 1 | `verus!` blocks | No | 0 |
| 2 | `spec fn` definitions | No | 0 |
| 3 | `requires` / `ensures` clauses | No | 0 |
| 4 | Loop invariants | No | 0 |
| 5 | `proof fn` definitions | No | 0 |
| 6 | `View` implementations | No | 0 |
| 7 | Ghost state (`ghost`, `tracked`) | No | 0 |
| 8 | Trigger annotations | No | 0 |
| 9 | Broadcast groups | No | 0 |

**Spec strength summary (all functions):**

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | ~57 |

All functions lack Verus specifications. The chapter is entirely unverified.

## Phase 7: Test Coverage

### Runtime Tests (RTT)

| # | Test File | Test Count | Operations Covered |
|---|---|:---:|---|
| 1 | `TestTableStEph.rs` | 18 | empty, singleton, insert, find, delete, domain, map, filter, intersection, union, difference, restrict, subtract, tabulate, ephemeral semantics, macro, large ops |
| 2 | `TestTableStPer.rs` | 18 | empty, singleton, insert, find, delete, domain, map, filter, intersection, union, difference, restrict, subtract, tabulate, macro, empty ops, persistence, combine |
| 3 | `TestTableMtEph.rs` | 19 | empty, singleton, insert, find, delete, domain, map, filter, intersection, union, difference, restrict, subtract, tabulate, ephemeral semantics, macro, parallel ops, parallel tabulate |
| 4 | `TestExample42_1.rs` | 2 | Smoke tests: `example_42_1()` and `performance_comparison()` run without panic |
| | **Total RTT** | **57** | |

### RTT Quality Assessment

| # | Strength | Notes |
|---|---|---|
| 1 | All 15 ADT operations tested in all 3 variants | Good coverage of the full interface. |
| 2 | Ephemeral/persistent semantics explicitly tested | `test_table_ephemeral_semantics` and `test_table_persistence` verify the core semantic difference. |
| 3 | Combine function tested | `test_table_insert_with_combine` (StPer) verifies `f(old, new)` for duplicate keys. |
| 4 | Parallel operations tested at scale | `test_table_parallel_operations` (50 entries), `test_table_parallel_tabulate` (20 entries with sleep). |
| 5 | Large-scale test | `test_table_large_operations` tests 100-entry table with filter. |
| 6 | Macro tests | All three `*Lit!` macros tested including empty and with-data variants. |

| # | Gap | Notes |
|---|---|---|
| 1 | No edge-case tests for empty intersection/union/difference | E.g., intersecting with empty table, union of disjoint tables. |
| 2 | No tests verifying sorted invariant | No test checks that entries remain sorted after operations. |
| 3 | No tests for `collect` | The `collect` function is untested in all variants. |
| 4 | No tests matching APAS Example 42.1 values | The prose uses `{'a'→4, 'b'→11, 'c'→2}` etc.; tests use different values. |

### Proof-Time Tests (PTT)

| # | PTT Count | Notes |
|---|---|---|
| 1 | 0 | No PTTs exist. No Verus code to test. |

## Phase 8: Example Coverage

### Example 42.1

The APAS defines concrete tables:
- `a = {'a'→4, 'b'→11, 'c'→2}`
- `b = {'b'→3, 'd'→5}`
- `c = {3, 5, 7}`

And demonstrates specific results:

| # | APAS Operation | Expected Result | Tested in Example42_1.rs | Tested in RTT |
|---|---|---|:---:|:---:|
| 1 | `find: a['b'] = 11` | `Some(11)` | No | No |
| 2 | `filter: {k→x∈a \| x<7} = {'a'→4, 'c'→2}` | 2 entries | No | No |
| 3 | `map: {k→3×v : k→v∈b} = {'b'→9, 'd'→15}` | 2 entries | No | No |
| 4 | `tabulate: {k→k² : k∈c} = {3→9, 5→25, 7→49}` | 3 entries | No | No (tabulate tested with different values) |
| 5 | `union: a∪b = {'a'→4, 'b'→3, 'c'→2, 'd'→5}` (using second) | 4 entries | No | No |
| 6 | `union: union+(a,b) = {'a'→4, 'b'→14, 'c'→2, 'd'→5}` | 4 entries | No | No |
| 7 | `subtract: a\{'b','d','e'} = {'a'→4, 'c'→2}` | 2 entries | No | No |

**None of the APAS Example 42.1 computations are verified.** `Example42_1.rs` uses integer keys (`1,2,3`) with string values (`"Alice","Bob","Carol"`) and demonstrates the API via print statements without assertions. The prose's specific values and expected results are not tested.

Note: The APAS tabulate example says `{3→9, 5→25, 9→81}` but `c = {3,5,7}`, so the expected result should be `{3→9, 5→25, 7→49}`. The prose appears to contain a typo (9 instead of 7 in the last entry).

### Syntax 42.4 (Table Shorthands)

| # | APAS Shorthand | Meaning | Present in code |
|---|---|---|:---:|
| 1 | `a[k]` | `find a k` | No (uses `table.find(&k)`) |
| 2 | `{k→f(x) : (k→x)∈a}` | `map f a` | No |
| 3 | `{k→f(x) : k∈a}` | `tabulate f a` | No |
| 4 | `{(k→v)∈a \| p(k,v)}` | `filter p a` | No |
| 5 | `a \ m` | `subtract a m` | No |
| 6 | `a ∪ b` | `union second a b` | No |

None of the APAS shorthand notations are provided as Rust syntax sugar. This is acceptable — they are prose conveniences, not required API surface.

## Proof Holes

```
Modules:
   4 clean (no holes)
   0 holed (contains holes)
   4 total

Holes Found: 0 total
```

No proof holes because there is no Verus code. The "clean" status is vacuously true.

## Review TODOs

| # | Priority | Category | Description |
|---|---|---|---|
| 1 | High | Verification | Verusify all three table implementations — add `verus!` blocks, `View` impls, `spec fn` definitions, `requires`/`ensures` on all 15 ADT operations. |
| 2 | High | Verification | Add spec functions capturing the prose definitions (e.g., `spec fn spec_domain`, `spec fn spec_intersection`, etc.). |
| 3 | High | Cost | The sorted-array backing structure gives O(n) insert/delete instead of the APAS-required O(lg n). Consider whether Chapter 42 should remain array-backed (as an "interface chapter" with simple implementations) or be upgraded to a balanced BST. |
| 4 | High | Algorithm | Implement Algorithm 42.3 (collect) with the correct type signature: `Seq<(K,V)> → Table<K, Seq<V>>`. The current `collect` should be renamed to `entries` or `to_seq`. |
| 5 | Medium | Cost | Fix cost annotations in all trait declarations — several `claude-4-sonet` annotations claim better costs than the implementations achieve (see Phase 4 annotation accuracy table). |
| 6 | Medium | Parallelism | MtEph intersection, union, and difference are entirely sequential. These should use divide-and-conquer or split-based parallelism. |
| 7 | Medium | Parallelism | MtEph map, filter, tabulate, delete, restrict, subtract use only a single 2-way split. Full recursive divide-and-conquer would achieve O(lg n) span. |
| 8 | Medium | Cost | StPer union is O((m+n)log(m+n)) due to re-sorting after decomposition into intersection + 2 differences. Consider a merge-based union that avoids the re-sort. |
| 9 | Medium | Example | Implement Example 42.1 with the exact APAS values (`{'a'→4, 'b'→11, 'c'→2}`, etc.) and verify the expected results via assertions. This requires char keys. |
| 10 | Medium | Testing | Add PTTs once Verus code exists. |
| 11 | Low | Structure | Add TOC headers to all source files per the table-of-contents-standard rule. |
| 12 | Low | Naming | Rename `collect` to `entries` or `to_seq` to avoid confusion with APAS Algorithm 42.3. |
| 13 | Low | Testing | Add edge-case RTTs: empty table operations, disjoint intersection, `collect` function, sorted invariant checks. |
| 14 | Low | Structure | Module headers use `//!` for copyright line — should be `//` per module-header rule. |
| 15 | Low | Style | `domain` in StEph/MtEph uses sequential insertion into `ArraySetStEph` which is O(n²). Could use `ArraySetStEph::from_seq` for O(n log n). |
