# Chap42 Spec Audit Against ADT 42.1

Audited: TableStEph, TableStPer.
Prose source: ADT 42.1 (Tables).

## Per-Function Classification

| # | Function | Prose Spec | TableStEph | TableStPer | Status |
|---|----------|-----------|------------|------------|--------|
| 1 | size | \|a\| | count == self@.len() | same | Correct |
| 2 | empty | {} | empty@ == Map::empty() | same | Correct |
| 3 | singleton | {k->v} | Map::empty().insert(k@, v@) | same | Correct |
| 4 | domain | dom(a) | domain@ =~= self@.dom() | same | Correct |
| 5 | tabulate | {k->f(k)} | dom + f.ensures value | same | **Fixed R17** |
| 6 | map | {k->f(v)} | dom + f.ensures value | same | **Fixed R17** |
| 7 | filter | {(k->v) \| p(k,v)} | dom subset + value pres | same | Partial |
| 8 | intersection | {k->f(v1,v2)} | dom + combine.ensures value | same | **Fixed R17** |
| 9 | union | combine/self/other | dom + 3-case value spec | same | **Fixed R17** |
| 10 | difference | {(k->v) \| k not in b} | dom + value preservation | same | Correct |
| 11 | find | v if (k->v) in a | Some/None with value | same | Correct |
| 12 | delete | remove key | self@ =~= old@.remove(k@) | same | Correct |
| 13 | insert | union with singleton | dom + 3-case value spec | same | **Fixed R17** |
| 14 | restrict | dom intersect keys | dom + value preservation | same | Correct |
| 15 | subtract | dom diff keys | dom + value preservation | same | Correct |
| 16 | entries/collect | entry sequence | spec_entries_to_map == self@ | same | Correct |

## Fix Details

### tabulate (Fixed in Round 17)

**Before:** `tabulated@.dom() =~= keys@` — domain only.
**After:** Added `forall|k| tabulated@.contains_key(k) ==> (exists|key_arg, result| key_arg@ == k && f.ensures((&key_arg,), result) && tabulated@[k] == result@)`.
**Prose:** `tabulate(f, S) = {k -> f(k) : k in S}`.
**Impact:** +1 external_body per file (StEph, StPer). Existing body doesn't track f.ensures through loop.

### map (Fixed in Round 17)

**Before:** `self@.dom() == old(self)@.dom()` (StEph) / `mapped@.dom() == self@.dom()` (StPer) — domain only.
**After:** Added `forall|k| self@.contains_key(k) ==> (exists|old_val, result| old_val@ == old(self)@[k] && f.ensures((&old_val,), result) && self@[k] == result@)`.
**Prose:** `map(f, T) = {k -> f(v) : (k -> v) in T}`.
**Impact:** +1 external_body per file.

### intersection (Fixed in Round 17)

**Before:** `self@.dom() =~= old(self)@.dom().intersect(other@.dom())` — domain only.
**After:** Added `forall|k| self@.contains_key(k) ==> (exists|v1, v2, r| v1@ == old(self)@[k] && v2@ == other@[k] && combine.ensures((&v1, &v2), r) && self@[k] == r@)`.
**Prose:** `intersection(f, a, b) = {k -> f(find a k, find b k) : k in dom(a) ∩ dom(b)}`.
**Impact:** +1 external_body per file.

### union (Fixed in Round 17)

**Before (StEph):** Domain only. **Before (StPer):** Domain + self-only + other-only value specs.
**After:** Added combine.ensures for both-keys case. StEph also got self-only and other-only value specs.
**Prose:** `union(f, a, b) = intersection(f, a, b) ∪ difference(a, b) ∪ difference(b, a)`.
**Impact:** +1 external_body per file.

### insert StEph (Fixed in Round 17)

**Before:** `self@.contains_key(key@), self@.dom() =~= old(self)@.dom().insert(key@)` — domain only.
**After:** Added 3-case value spec matching StPer: non-key preservation, new-key value, existing-key combine.ensures.
**Prose:** `insert(f, a, k, v) = union(f, a, singleton(k, v))`.
**Impact:** +1 external_body (StEph only; StPer already had this spec).

### filter (Partial — No Fix)

**Current:** Domain subset + value preservation. Missing: which keys survive.
**Prose:** `filter(p, T) = {(k -> v) in T | p(k,v)}`.
**Gap:** Can't express predicate completeness without `Ghost(spec_fn(K, V) -> bool)` companion. The closure's `ensures` could express this, but the existential quantifier pattern is fragile for callers.
**Classification:** Partial. Value preservation is correct; domain filtering is the gap.
