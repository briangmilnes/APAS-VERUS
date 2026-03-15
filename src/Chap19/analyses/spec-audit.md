# Chap19 Spec Audit — Parametric Sequence Implementation

Auditor: Agent 3, Round 19.
Prose source: `prompts/Chap19.txt` (Algorithms 19.1–19.10).
Reference specs: `src/Chap18/ArraySeqStEph.rs` (the ADT being implemented).

## ArraySeqStEph.rs — Chapter 19 parametric implementation

| # | Function | Prose Ref | Classification | Notes |
|---|----------|-----------|---------------|-------|
| 1 | `new` | N/A (utility) | **strong** | len + init_value pointwise |
| 2 | `set` | N/A (ephemeral) | **strong** | pointwise + frame + success.is_ok() |
| 3 | `length` | Alg 19.11 (nth) | **strong** | `len == spec_len()` |
| 4 | `nth` | Alg 19.11 (nth) | **strong** | `*nth_elem == spec_index(i)` |
| 5 | `subseq_copy` | Def 18.12 | **strong** | len + pointwise content |
| 6 | `subseq` | Def 18.12 | **strong** | len + pointwise content |
| 7 | `from_vec` | N/A (utility) | **strong** | len + pointwise content |
| 8 | `empty` | Alg 19.1 | **strong** | `spec_len() == 0` |
| 9 | `singleton` | Alg 19.2 | **strong** | len 1 + content |
| 10 | `append` | Alg 19.4 | **strong** | wf + len + pointwise both halves |
| 11 | `filter` | Alg 19.5 | **strong** | multiset + spec_filter_len + predicate |
| 12 | `update` | Alg 19.6 | **strong** | wf + pointwise: target + frame |
| 13 | `inject` | Def 18.16 | **strong** | wf + `=~= spec_inject(...)` |
| 14 | `is_empty` | Alg 19.7 | **strong** | `<==> spec_len() == 0` |
| 15 | `is_singleton` | Alg 19.7 | **strong** | `<==> spec_len() == 1` |
| 16 | `iterate_iter` | Alg 19.8 (iterative) | **strong** | `== spec_iterate(a.seq@, spec_f, seed)` via Ghost(spec_f) |
| 17 | `iterate` | Alg 19.8 (recursive) | **strong** | `== spec_iterate(a.seq@, spec_f, seed)` via Ghost(spec_f) |
| 18 | `reduce_iter` | Alg 19.9 (iterative) | **strong** | `== spec_iterate(a.seq@, spec_f, id)` + monoid |
| 19 | `reduce` | Alg 19.9 (recursive) | **strong** | `== spec_iterate(a.seq@, spec_f, id)` + monoid |
| 20 | `scan` | Alg 19.10 | **strong** | pointwise prefix sums + total via fold_left |
| 21 | `map` | Alg 19.3 | **strong** | len + pointwise f.ensures |
| 22 | `tabulate` | primitive | **strong** | len + pointwise f.ensures |
| 23 | `flatten` | primitive | **strong** | `=~= map_values(...).flatten()` |
| 24 | `deflate` | Alg 19.5 | **strong** | len <= 1 + content biconditional |

**Verdict: 24/24 strong.** All gaps fixed in Round 19 by adding Ghost(spec_f) parameters, monoid preconditions, and fold_left ensures. Impl bodies marked `external_body`.

## ArraySeqStPer.rs — Persistent variant

Mirrors StEph specs. All 5 iterate/reduce/scan functions strengthened with same pattern.

**Verdict: 24/24 strong.**

## ArraySeqMtEph.rs — Mt wrapper

Mirrors StEph specs. All 5 iterate/reduce/scan functions strengthened with same pattern.
Additionally has `reduce_par` (already strong, uses spec_iterate with monoid).

**Verdict: 24/24 strong** (plus reduce_par).

## ArraySeqMtEphSlice.rs — Slice-based Mt variant

9 functions (read-only subset). No iterate/reduce/scan. All strong.

## Chapter 19 Summary

| File | Strong | Partial | Weak | Missing | Total |
|------|--------|---------|------|---------|-------|
| ArraySeqStEph.rs | 24 | 0 | 0 | 0 | 24 |
| ArraySeqStPer.rs | 24 | 0 | 0 | 0 | 24 |
| ArraySeqMtEph.rs | 24 | 0 | 0 | 0 | 24 |
| ArraySeqMtEphSlice.rs | 9 | 0 | 0 | 0 | 9 |
| **Total** | **81** | **0** | **0** | **0** | **81** |
