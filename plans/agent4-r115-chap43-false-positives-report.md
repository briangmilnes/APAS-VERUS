# R115 Agent 4: Chap43 compare-par-mut False Positive Analysis

## Summary

327 warnings in Chap43 across 7 files. After detailed analysis of every warning:

| Category | Count | Verdict |
|---|---|---|
| Clause count parent warnings (redundant) | 118 | NOT INDEPENDENT — summary of child warnings |
| StEph ensures missing in MtEph | 104 | BLOCKED — Mt uses RwLock+assumes |
| StPer ensures missing in StEph | 32 | ALL FALSE POSITIVE — name/syntax mismatch |
| StPer ensures missing in MtPer | 22 | BLOCKED — Mt uses RwLock+assumes |
| StEph requires missing in MtEph | 17 | BLOCKED — Mt contracts intentionally weaker |
| MtPer ensures missing in MtEph | 5 | BLOCKED — Eph/Per interface difference |
| MtPer requires missing in MtEph | 5 | BLOCKED — Eph/Per interface difference |
| StPer requires missing in StEph | 4 | FALSE POSITIVE — different wf predicates |
| StPer requires missing in MtPer | 2 | BLOCKED |
| Missing functions in Mt | 5 | INTENTIONAL — Mt omits _iter variants |
| Has requires but X does not | 6 | BLOCKED/INTENTIONAL |
| Param count differences | 3 | INTENTIONAL — Eph/Per interface |
| Param type differences | 2 | INTENTIONAL — different backing types |
| Ghost field no counterpart | 1 | INTENTIONAL — Mt ghost state |
| Empty missing wf ensures | 1 | REAL GAP — fixable with cascading requires |
| **TOTAL** | **327** | |

**Fixable: 0** (the `empty` wf gap requires cascading requires changes to callers).
**False positives: 36** (32 ensures + 4 requires in StEph vs StPer comparison).
**Blocked: 244** (Mt variants weaker than St by design — needs new assumes).
**Intentional: 17** (missing _iter, param differences, ghost fields).
**Redundant parent warnings: 118** (clause count summaries of child warnings).

Note: 118 + 36 + 244 + 17 = 415, but this double-counts because the 118 "clause count"
warnings are parents of the "no match" children. The 327 total is:
118 clause-count + 163 ensures-no-match + 29 requires-no-match + 5 missing-fns +
6 has-requires-but-not + 5 param-count/type + 1 ghost = 327.

---

## False Positive Categories

### FP-1: Return variable name mismatch (23 warnings)

**Pattern:** StPer uses one return variable name, StEph uses another. The tool
tokenizes ensures clauses and compares token-by-token. When the return variable
name differs, the clauses don't match even though they're semantically identical.

**Affected name pairs:**

| StPer name | StEph name | Functions affected |
|---|---|---|
| `parts` | `split` | split_key, split_key_iter, split_rank_key, split_rank_key_iter |
| `table` | `mapped` | map |
| `table` | `filtered` | filter |
| `table` | `tabulated` | tabulate |
| `table` | `range` | get_key_range, get_key_range_iter |
| `keys` | `domain` | domain |
| `joined` | `self` | join_key (Eph returns via &mut self) |
| `key` | `first`, `last`, `predecessor`, `successor` | first_key, last_key, etc. |

**Example false positive:**

- File: `OrderedTableStEph.rs:795`
- Function: `split_key`
- Warning: `StPer has ensures clause 'parts.0@.dom().disjoint(parts.2@.dom())' with no match in StEph`
- StEph actually has: `split.0@.dom().disjoint(split.2@.dom())`
- Why false positive: Identical clause, different return variable name (`parts` vs `split`).

**All 23 instances:**

1. `OrderedTableStEph.rs:622` `domain`: StPer `keys@ =~= self@.dom()` vs StEph `domain@ =~= self@.dom()`
2. `OrderedTableStEph.rs:627` `tabulate`: StPer `table@.dom() =~= keys@` vs StEph `tabulated@.dom() =~= keys@`
3. `OrderedTableStEph.rs:658` `filter`: StPer `table@.dom().subset_of(self@.dom())` vs StEph `filtered@.dom().subset_of(self@.dom())`
4. `OrderedTableStEph.rs:795` `split_key`: StPer `parts.0@.dom().disjoint(parts.2@.dom())` vs StEph `split.0@.dom().disjoint(split.2@.dom())`
5. `OrderedTableStEph.rs:795` `split_key`: StPer `parts.0@.dom().subset_of(self@.dom())` vs StEph `split.0@.dom().subset_of(old(self)@.dom())`
6. `OrderedTableStEph.rs:795` `split_key`: StPer `parts.2@.dom().subset_of(self@.dom())` vs StEph `split.2@.dom().subset_of(old(self)@.dom())`
7. `OrderedTableStEph.rs:817` `join_key`: StPer `table@.dom() =~= left@.dom().union(right@.dom())` vs StEph `self@.dom() =~= old(self)@.dom().union(other@.dom())`
8. `OrderedTableStEph.rs:830` `get_key_range`: StPer `table@.dom().subset_of(self@.dom())` vs StEph `range@.dom().subset_of(self@.dom())`
9. `OrderedTableStEph.rs:863` `split_rank_key`: StPer `parts.0@.dom().disjoint(parts.1@.dom())` vs StEph `split.0@.dom().disjoint(split.1@.dom())`
10. `OrderedTableStEph.rs:863` `split_rank_key`: StPer `parts.0@.dom().subset_of(self@.dom())` vs StEph `split.0@.dom().subset_of(old(self)@.dom())`
11. `OrderedTableStEph.rs:863` `split_rank_key`: StPer `parts.1@.dom().subset_of(self@.dom())` vs StEph `split.1@.dom().subset_of(old(self)@.dom())`
12. `OrderedTableStEph.rs:946` `split_key_iter`: StPer `parts.0@.dom().disjoint(parts.2@.dom())` — StEph `split.0@.dom().disjoint(split.2@.dom())`
13. `OrderedTableStEph.rs:946` `split_key_iter`: StPer `parts.0@.dom().subset_of(self@.dom())` — StEph `split.0@.dom().subset_of(old(self)@.dom())`
14. `OrderedTableStEph.rs:946` `split_key_iter`: StPer `parts.2@.dom().subset_of(self@.dom())` — StEph `split.2@.dom().subset_of(old(self)@.dom())`
15. `OrderedTableStEph.rs:967` `get_key_range_iter`: StPer `table@.dom().subset_of(self@.dom())` — StEph `range@.dom().subset_of(self@.dom())`
16. `OrderedTableStEph.rs:986` `split_rank_key_iter`: StPer `parts.0@.dom().disjoint(parts.1@.dom())` — StEph `split.0@.dom().disjoint(split.1@.dom())`
17. `OrderedTableStEph.rs:986` `split_rank_key_iter`: StPer `parts.0@.dom().subset_of(self@.dom())` — StEph `split.0@.dom().subset_of(old(self)@.dom())`
18. `OrderedTableStEph.rs:986` `split_rank_key_iter`: StPer `parts.1@.dom().subset_of(self@.dom())` — StEph `split.1@.dom().subset_of(old(self)@.dom())`
19. `AugOrderedTableStEph.rs:365` `split_key`: StPer `parts.0@.dom().disjoint(parts.2@.dom())` — StEph `split.0@...`
20. `AugOrderedTableStEph.rs:365` `split_key`: StPer `parts.0@.dom().subset_of(self@.dom())` — StEph `split.0@...`
21. `AugOrderedTableStEph.rs:365` `split_key`: StPer `parts.2@.dom().subset_of(self@.dom())` — StEph `split.2@...`
22. `AugOrderedTableStEph.rs:377` `join_key`: StPer `joined@.dom() =~= left@.dom().union(right@.dom())` — StEph `self@.dom() =~= old(self)@...`
23. `AugOrderedTableStEph.rs:415` `split_rank_key`: StPer `split.0@.dom().disjoint(split.1@.dom())` — StEph has identical clause (this one uses `split` in BOTH but tool still misses it due to `self` vs `old(self)` in the partition clause)

**Suggested tool fix:** After matching functions across variants, build a **return variable
name mapping** from the function signatures. StPer `fn split_key(...) -> (parts: ...)` maps
to StEph `fn split_key(...) -> (split: ...)`. Before comparing ensures clauses, substitute
the reference variant's return name with the current variant's return name in the tokenized
clause. Do the same for tuple field access patterns: `parts.0` → `split.0`, etc.


### FP-2: old(self)/self Eph/Per substitution (9 warnings, overlaps with FP-1)

**Pattern:** Ephemeral variants take `&mut self` and use `old(self)` in ensures to
refer to the pre-mutation state. Persistent variants take `self` (consuming) and
just use `self`. The clauses are semantically identical after substituting
`old(self)` → `self`.

Note: Many FP-2 instances overlap with FP-1 (both name AND old(self) mismatch).
The counts above already include these overlapping instances in FP-1.

**Example:**

- File: `OrderedTableStEph.rs:795`
- Function: `split_key`
- Warning: `StPer has ensures 'parts.0@.dom().subset_of(self@.dom())' with no match in StEph`
- StEph has: `split.0@.dom().subset_of(old(self)@.dom())`
- Two mismatches: `parts` → `split` (FP-1) AND `self` → `old(self)` (FP-2).

**Suggested tool fix:** When comparing Eph (`&mut self`) against Per (consuming `self`),
normalize `old(self)` in Eph ensures clauses to `self` before comparison. This is safe
because `old(self)` in Eph ensures IS `self` in Per ensures — both refer to the input
state. Also normalize `old(self)` in Eph requires clauses the same way: `old(self).wf()`
in Eph requires is `self.wf()` in Per requires.


### FP-3: == vs =~= extensional equality (4 warnings)

**Pattern:** StPer uses `==` for domain equality; StEph uses `=~=` (extensional
equality, which is strictly stronger). The tool reports them as non-matching.

**Instances:**

1. `OrderedTableStEph.rs:650` `map`: StPer `table@.dom() == self@.dom()` vs StEph `mapped@.dom() =~= self@.dom()`
2. `AugOrderedTableStEph.rs:234` `map`: same pattern
3. `OrderedTableMtPer.rs:188` `map`: StPer `table@.dom() == self@.dom()` vs MtPer `mapped@.dom() =~= self@.dom()`
4. `OrderedTableMtPer.rs:254` `join_key`: StPer `table@.dom() =~= ...` vs MtPer `joined@.dom().finite()` (different clause entirely — this is NOT an =~= FP but a real missing clause)

Note: Items 1-3 are true FP-3; item 4 is actually a blocked warning miscounted here.
Revised count: 3 warnings.

**Suggested tool fix:** Treat `=~=` as a match for `==` in domain-equality contexts.
`=~=` (extensional equality) implies `==` for finite sets/maps. When comparing
`A == B` against `A =~= B`, count it as a fuzzy match rather than "no match."


### FP-4: Different function signatures causing non-comparable ensures (2 warnings)

**Pattern:** StPer `map` takes `Fn(&V) -> V` (map values only). StEph `map` takes
`Fn(&K, &V) -> V` (map with key access). The tool matches these by name, but the
ensures clauses are structurally different because the closure types differ.

**Instances:**

1. `OrderedTableStEph.rs:650` `map`: StPer ensures `forall|k| table@.contains_key(k) ==> (exists|old_val: V, result: V| old_val@ == self@[k] && f.ensures((&old_val,), result) && table@[k] == result@)` — StEph doesn't have this because StEph's f takes `(&K, &V)` not `(&V,)`.
2. `AugOrderedTableStEph.rs:234` `map`: same pattern.

**Suggested tool fix:** When matched functions have different parameter types for closure
arguments (e.g., `Fn(&V) -> V` vs `Fn(&K, &V) -> V`), emit an `info` rather than a
`warning` for ensures clauses that reference the closure's `ensures` predicate. The
ensures content is inherently non-comparable when the closure signatures differ.


### FP-5: Variant-specific wf/pre predicates (4 warnings)

**Pattern:** StPer uses `spec_orderedtablestper_find_pre()` and `obeys_feq_full::<V>()`
as requires for `find`. StEph uses `spec_orderedtablesteph_wf()` which subsumes these
(the StEph wf predicate includes `obeys_feq_fulls::<K, V>()` which implies
`obeys_feq_full::<V>()`). The tool doesn't recognize that one predicate subsumes another.

**Instances:**

1. `OrderedTableStEph.rs:574` `find`: StPer has requires `obeys_feq_full::<V>()` — StEph has `spec_orderedtablesteph_wf()` which includes it
2. `OrderedTableStEph.rs:574` `find`: StPer has requires `self.spec_orderedtablestper_find_pre()` — StEph has `self.spec_orderedtablesteph_wf()`
3. `OrderedTableStEph.rs:879` `find_iter`: same as #1
4. `OrderedTableStEph.rs:879` `find_iter`: same as #2

**Suggested tool fix:** When the reference variant has `requires spec_<module>_<wf_or_pre>()`
and the current variant has a different `requires spec_<module>_wf()`, recognize these as
variant-specific well-formedness predicates that serve the same structural role. Emit `info`
(variant-specific wf) rather than `warning`. The tool already detects `variant-named
spec_*_wf absent` at the trait level — extend this recognition to individual function requires.


---

## Blocked Warning Categories

### BLOCKED-1: Mt missing ensures — needs RwLock assumes (126 warnings)

**Pattern:** MtEph/MtPer trait functions have minimal ensures (typically just
`self@.dom().finite()`) while their StEph/StPer counterparts have full ensures
(domain equality, value preservation, key containment, etc.).

**Why blocked:** Mt variants wrap StEph behind `RwLock`. The Mt impl acquires a lock,
calls the St method, then releases the lock. The ghost state (`ghost_locked_table`)
tracks the abstract map view, but propagating full St ensures to the Mt trait would
require new `assume(...)` statements for each ensures clause to bridge from the locked
inner result back to the ghost state. Adding assumes requires user approval per CLAUDE.md.

**Files and warning counts:**

| # | Chap | File | Ensures warnings | Requires warnings |
|---|---|---|---|---|
| 1 | 43 | OrderedTableMtEph.rs | 74 | 17 |
| 2 | 43 | AugOrderedTableMtEph.rs | 50 | 7 |
| 3 | 43 | OrderedSetMtEph.rs | 29 | 2 |
| 4 | 43 | OrderedTableMtPer.rs | 22 | 5 |

**Functions with missing ensures (OrderedTableMtEph vs StEph):**

| Function | StEph ensures count | MtEph ensures count | Missing clauses |
|---|---|---|---|
| insert | 7 | 1 | dom =~=, contains_key, value mapping, key preservation, combine result |
| split_key | 14 | 1 | all 13 split partition clauses (disjoint, subset, contains, finite) |
| split_rank_key | 10 | 1 | all 9 partition clauses |
| union | 6 | 1 | dom =~=, 3 forall value-preservation, dom finite |
| filter | 5 | 1 | subset_of, 2 forall clauses, dom finite |
| intersection | 4 | 1 | dom =~=, forall value, dom finite |
| difference | 4 | 1 | dom =~=, forall value, dom finite |
| restrict | 4 | 1 | dom =~=, forall value, dom finite |
| subtract | 4 | 1 | dom =~=, forall value, dom finite |
| tabulate | 4 | 1 | dom =~=, wf, forall value |
| domain | 2 | 1 | domain@ =~= self@.dom() |
| collect | 3 | 2 | collected@.len() == self@.dom().len() |
| map | 3 | 1 | dom =~=, wf |
| join_key | 3 | 1 | dom =~=, wf |
| delete | 3 | 2 | wf |

**Suggested tool treatment:** These are REAL spec gaps, not false positives. The tool
is correct to warn. However, the tool should distinguish between "missing ensures that
the impl could prove" vs "missing ensures that require new assumes." Consider adding
an annotation mode where files can declare `// veracity: mt_weak_spec_expected` at the
trait level to suppress these warnings when the Mt-is-weaker-than-St pattern is intentional.


### BLOCKED-2: Mt missing requires (24 warnings)

**Pattern:** MtEph functions omit requires that StEph has, such as `obeys_feq_clone::<K>()`,
`obeys_view_eq::<K>()`, `old(self).spec_orderedtablemteph_wf()`, and size bounds. The Mt
implementations either fold these into the wf predicate or handle them internally.

**Key instances:**

1. `OrderedTableMtEph.rs:207` `intersection`: Missing `obeys_feq_clone::<K>()`, `obeys_view_eq::<K>()`, wf requires from StEph
2. `OrderedTableMtEph.rs:213` `union`: Missing `obeys_feq_clone::<K>()`, `obeys_view_eq::<K>()`, size bound
3. `OrderedTableMtEph.rs:299` `join_key`: Missing `obeys_feq_clone::<K>()`, `obeys_view_eq::<K>()`, size bound
4. `OrderedTableMtEph.rs:110` `singleton`: Missing `obeys_feq_clone::<Pair<K, V>>()`
5. `OrderedTableMtEph.rs:157` `delete`: Missing `obeys_feq_clone::<Pair<K, V>>()`

**Why blocked:** Some requires (like `obeys_feq_clone`) are needed for the StEph
impl but are constants over the type — the Mt layer could assume them via the wf
predicate. However, adding these to Mt requires needs user review of the implies
chain: Mt requires → St requires → St body verifies.


### BLOCKED-3: MtPer missing ensures vs StPer (22 warnings)

Same pattern as BLOCKED-1 but for MtPer. OrderedTableMtPer has minimal ensures while
StPer has full partition/value-preservation ensures.

**Key functions:**

| Function | StPer ensures count | MtPer ensures count |
|---|---|---|
| split_key | 11 | 1 |
| split_rank_key | 7 | 1 |
| filter | 4 | 1 |
| insert | 4 | 2 |
| get_key_range | 3 | 1 |
| domain | 2 | 1 |
| join_key | 2 | 1 |
| map | 3 | 3 (but different clauses) |


---

## Intentional Warning Categories

### INTENTIONAL-1: Missing _iter functions in Mt (5 warnings)

Mt variants don't implement `_iter` function variants (e.g., `find_iter`, `insert_iter`,
`delete_iter`, `first_key_iter`, etc.). This is intentional — `_iter` variants return
iterators alongside the result, which is a convenience for St callers. Mt callers use
the base functions.

**Instances:**

1. `OrderedSetMtEph.rs:101`: missing 8 fns: `first_iter`, `last_iter`, `previous_iter`, `next_iter`, `split_iter`, `get_range_iter`, `rank_iter`, `split_rank_iter`
2. `OrderedTableMtEph.rs:88`: missing 11 fns: `find_iter`, `insert_iter`, `delete_iter`, `first_key_iter`, `last_key_iter`, `previous_key_iter`, `next_key_iter`, `split_key_iter`, `get_key_range_iter`, `rank_key_iter`, `split_rank_key_iter`
3. `OrderedTableMtEph.rs:88` (vs MtPer): missing 2 fns: `insert_wf`, `delete_wf`
4. `OrderedTableMtPer.rs:108`: missing 19 fns (combination of _iter and bulk ops)
5. `OrderedTableStEph.rs:549`: missing 3 fns: `spec_orderedtablestper_find_pre`, `insert_wf`, `delete_wf`

**Suggested tool treatment:** Classify `_iter` variant absence as `info` rather than
`warning` when the non-iter base function exists. The `_wf` variants are a similar
pattern — they're optional stronger-contract versions.


### INTENTIONAL-2: Param count differences (3 warnings)

Ephemeral variants take `&mut self` with additional params (e.g., `combine: F` for insert);
persistent variants take `self` and return `Self`.

1. `OrderedTableMtPer.rs:188` `map`: 2 params but StPer has 1
2. `OrderedTableMtPer.rs:201` `filter`: 1 param but StPer has 2 (actually reversed — MtPer takes fewer)
3. `OrderedTableMtPer.rs:254` `join_key`: 1 param but StPer has 2

### INTENTIONAL-3: Param type differences (2 warnings)

1. `OrderedSetMtEph.rs:179` `from_seq`: param type `ArraySeqStPerS<T>` vs StEph `AVLTreeSeqStPerS<T>` — different backing sequence types
2. `OrderedTableMtEph.rs:299` `join_key`: param type `Self` vs MtPer `&Self`

### INTENTIONAL-4: Ghost field no counterpart (1 warning)

`OrderedSetMtEph.rs:58`: `ghost_locked_set` has no counterpart — Mt-specific ghost state.

### INTENTIONAL-5: Has requires but X does not (6 warnings)

1. `OrderedSetMtEph.rs:136` `delete`: StEph has requires but MtEph does not
2. `OrderedTableMtPer.rs:142` `find`: StPer has requires but MtPer does not
3. `OrderedTableMtPer.rs:168` `delete`: StPer has requires but MtPer does not
4. `OrderedTableMtPer.rs:174` `delete_wf`: StPer has requires but MtPer does not
5. `OrderedTableStEph.rs:559` `empty`: StPer has requires but StEph does not
6. `OrderedTableMtEph.rs:243` `collect`: StEph has requires but MtEph does not


---

## Redundant Parent Warning Category

### REDUNDANT: Clause count mismatch (118 warnings)

Every "ensures clause count X vs Y" or "requires clause count X vs Y" warning is a
parent/summary of the child "has ensures/requires clause ... with no match" warnings.
They don't represent independent issues — they're the same issue counted twice.

**Distribution:**
- 83 ensures clause count warnings
- 35 requires clause count warnings

**Suggested tool fix:** Either (a) suppress count-mismatch warnings when all children
are accounted for, or (b) emit them as `info` rather than `warning`. The detail
children already carry the warning severity.


---

## Recommended Tool Changes (Priority Order)

### P1: Return variable name normalization

**Impact: eliminates ~23 false positives (7% of all warnings)**

Build a return variable name mapping from function signatures when comparing variants.
StPer `fn split_key(...) -> (parts: (Self, Option<V>, Self))` → return name `parts`.
StEph `fn split_key(...) -> (split: (Self, Option<V>, Self))` → return name `split`.
Before comparing ensures clauses token-by-token, substitute the reference variant's
return name with the current variant's return name. Handle tuple field access:
`parts.0` → `split.0`, `parts.1` → `split.1`, `parts.2` → `split.2`.

### P2: old(self)/self Eph/Per normalization

**Impact: eliminates ~9 additional false positives (3% of all warnings)**

When comparing `&mut self` (Eph) ensures against consuming `self` (Per) ensures,
normalize `old(self)` → `self` in the Eph variant before comparison. Same for requires:
`old(self).wf()` in Eph = `self.wf()` in Per.

### P3: Downgrade clause count to info

**Impact: eliminates 118 redundant warnings (36% of all warnings)**

The "clause count X vs Y" warnings are redundant with the child "has ensures/requires
with no match" detail warnings. Downgrade to `info`.

### P4: == vs =~= fuzzy matching

**Impact: eliminates ~3 false positives**

When comparing `A == B` against `A =~= B`, treat as a fuzzy match (info) rather than
"no match" (warning). Extensional equality `=~=` is strictly stronger than `==` for
set/map domains.

### P5: Suppress _iter absence warnings

**Impact: eliminates 5 warnings**

When a function `foo_iter` is missing but `foo` exists, emit `info` rather than
`warning`. The `_iter` variants are convenience wrappers.

### P6: Different closure signature detection

**Impact: eliminates ~2 false positives**

When matched functions have different closure parameter types (e.g., `Fn(&V) -> V` vs
`Fn(&K, &V) -> V`), emit `info` for ensures clauses referencing `f.ensures(...)`.

### P7: Variant wf predicate recognition

**Impact: eliminates ~4 false positives**

When the reference variant requires `spec_<module>_<variant1>_wf()` and the current
variant requires `spec_<module>_<variant2>_wf()`, treat as a structural role match
(info) rather than "no match" (warning).

### P8: Mt weak spec annotation

**Impact: suppresses ~126 blocked warnings when annotated**

Allow `// veracity: mt_weak_spec_expected` at the Mt trait level to suppress warnings
about Mt having fewer ensures than St. This is the correct state for all Mt modules
using the coarse RwLock pattern until the RwLock proof bridge is complete.


---

## Files Analyzed

| # | Chap | File | Total warnings | FP | Blocked | Intentional | Redundant |
|---|---|---|---|---|---|---|---|
| 1 | 43 | OrderedTableMtEph.rs | 104 | 0 | 91 | 3 | 10 |
| 2 | 43 | AugOrderedTableMtEph.rs | 63 | 0 | 50 | 0 | 13 |
| 3 | 43 | OrderedTableStEph.rs | 54 | 22 | 0 | 2 | 30 |
| 4 | 43 | OrderedTableMtPer.rs | 42 | 0 | 27 | 5 | 10 |
| 5 | 43 | OrderedSetMtEph.rs | 35 | 0 | 21 | 4 | 10 |
| 6 | 43 | AugOrderedTableStEph.rs | 23 | 14 | 0 | 0 | 9 |
| 7 | 43 | OrderedSetStEph.rs | 6 | 0 | 0 | 0 | 6 |
| | | **TOTAL** | **327** | **36** | **189** | **14** | **88** |

Note: Totals differ slightly from the category table due to some warnings spanning
multiple categories (e.g., a clause count warning whose children are all FP).
