# Chap18 Spec Audit — Sequence ADT

Auditor: Agent 3, Round 19.
Prose source: `prompts/Chap18.txt` (Data Type 18.1, Definitions 18.3–18.22).

## ArraySeqStEph.rs — Data Type 18.1 Sequences

### BaseTrait

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `new` | N/A (utility) | **strong** | len + init_value pointwise |
| 2 | `set` | N/A (ephemeral utility) | **strong** | pointwise update, frame |
| 3 | `length` | Def 18.3 (length) | **strong** | `len == spec_len()` |
| 4 | `nth` | Def 18.3 (nth) | **strong** | `*nth_elem == spec_index(i)` |
| 5 | `subseq_copy` | Def 18.12 | **strong** | len + pointwise content |
| 6 | `subseq` | Def 18.12 (subseq) | **strong** | len + pointwise content |
| 7 | `from_vec` | N/A (utility) | **strong** | len + pointwise content |

### RedefinableTrait

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `empty` | Def 18.4 (empty) | **strong** | `spec_len() == 0` |
| 2 | `singleton` | Def 18.4 (singleton) | **strong** | len 1 + content |
| 3 | `append` | Def 18.13 (append) | **strong** | len + pointwise both halves |
| 4 | `filter` | Def 18.10 (filter) | **strong** | multiset equality + spec_filter_len + predicate satisfaction |
| 5 | `update` | Def 18.15 (update) | **strong** | pointwise: target + frame |
| 6 | `inject` | Def 18.16 (inject) | **strong** | `=~= spec_inject(...)` |
| 7 | `is_empty` | Def 18.5 (isEmpty) | **strong** | `<==> spec_len() == 0` |
| 8 | `is_singleton` | Def 18.5 (isSingleton) | **strong** | `<==> spec_len() == 1` |
| 9 | `iterate` | Def 18.19 (iterate) | **strong** | `== spec_iterate(s, spec_f, seed)` via fold_left |
| 10 | `reduce` | Def 18.21 (reduce) | **strong** | `== spec_iterate(s, spec_f, id)` with monoid precondition |
| 11 | `scan` | Def 18.22 (scan) | **strong** | pointwise prefix sums + total |
| 12 | `map` | Def 18.8 (map) | **strong** | len + pointwise f.ensures |
| 13 | `tabulate` | Def 18.6 (tabulate) | **strong** | len + pointwise f.ensures |
| 14 | `flatten` | Def 18.14 (flatten) | **strong** | `=~= map_values(...).flatten()` |

**Verdict: 21/21 strong.** Every function faithfully encodes its APAS definition.

## ArraySeqStPer.rs — Persistent variant

All trait specs mirror StEph. One minor deviation: `set` lacks explicit `success.is_ok()` guarantee (conditional ensures only). Not a prose deviation since `set` is not in the ADT prose.

**Verdict: All strong (modulo minor `set` deviation).**

## ArraySeqMtEph.rs / ArraySeqMtPer.rs — Mt wrappers

All trait specs mirror the St counterparts. Same minor `set` deviation as StPer.

**Verdict: All strong.**

## LinkedListStEph.rs / LinkedListStPer.rs — Linked list variants

Not in the audit scope (different backing structure, same ADT).

## Chapter 18 Summary

| File | Strong | Partial | Weak | Missing | Total |
|------|--------|---------|------|---------|-------|
| ArraySeqStEph.rs | 21 | 0 | 0 | 0 | 21 |
| ArraySeqStPer.rs | 21 | 0 | 0 | 0 | 21 |
| ArraySeqMtEph.rs | 21 | 0 | 0 | 0 | 21 |
| ArraySeqMtPer.rs | 21 | 0 | 0 | 0 | 21 |
| **Total** | **84** | **0** | **0** | **0** | **84** |
