<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap19 — Implementation Comparison

## ArraySeqStPer (Persistent)

| #  | Function        | Chap18 Impl                                    | Chap19 Impl                                                     | Different?  | Matches Chap19.txt?                                    | Verified? |
|----|-----------------|------------------------------------------------|------------------------------------------------------------------|-------------|--------------------------------------------------------|-----------|
| 1  | `empty`         | `Vec::new()`                                   | `tabulate(closure, 0)`                                           | **Yes**     | **Yes** — 19.1                                         | Yes       |
| 2  | `singleton`     | `push(item)`                                   | `tabulate(clone closure, 1)`                                     | **Yes**     | **Yes** — 19.2 (adds `Clone+Eq` bounds)               | Yes       |
| 3  | `map`           | While loop: `seq.push(f(&a.seq[i]))`           | `tabulate(closure calling f(&a.seq[i]))`                         | **Yes**     | **Yes** — 19.3                                         | Yes       |
| 4  | `append`        | Two while loops: clone from a then b           | `tabulate(select(a,b), \|a\|+\|b\|)`                            | **Yes**     | **Yes** — 19.4 variant 2                               | Yes       |
| 5  | `filter`        | While loop: clone if pred true                 | `flatten(map(deflate pred, a))`                                  | **Yes**     | **Yes** — 19.5                                         | Yes       |
| 6  | `update`        | While loop: clone item or a[j]                 | `tabulate(closure with if j==index)`                             | **Yes**     | **Yes** — 19.6                                         | Yes       |
| 7  | `is_empty`      | `self.seq.len() == 0`                          | `self.seq.len() == 0`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 8  | `is_singleton`  | `self.seq.len() == 1`                          | `self.seq.len() == 1`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 9  | `iterate_iter`  | While loop left-to-right                       | While loop left-to-right (renamed from `iterate`)                | Same        | **No** — iterative variant kept in trait               | Yes       |
| 10 | `iterate`       | *(same as iterate_iter)*                       | Recursive: `iterate f (f(x,a[0])) a[1..\|a\|-1]`               | **Yes**     | **Yes** — 19.8                                         | Yes       |
| 11 | `reduce_iter`   | While loop left-to-right                       | While loop left-to-right (renamed from `reduce`)                 | Same        | **No** — iterative variant kept in trait               | Yes       |
| 12 | `reduce`        | *(same as reduce_iter)*                        | Recursive D&C: `f(reduce f id b, reduce f id c)`                | **Yes**     | **Yes** — 19.9                                         | Yes       |
| 13 | `scan`          | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — should be contraction (19.10)                 | Yes       |
| 14 | `tabulate`      | While loop: `seq.push(f(i))`                   | While loop: `seq.push(f(i))`                                     | Same        | **Yes** — primitive                                    | Yes       |
| 15 | `flatten`       | Nested while loops with clone                  | Nested while loops with clone                                    | Same        | **Yes** — primitive                                    | Yes       |
| 16 | `deflate`       | *(not in Chap18)*                              | `if pred(x) { singleton } else { empty }`                        | **New**     | **Yes** — 19.5 deflate                                 | Yes       |

## ArraySeqStEph (Ephemeral)

| #  | Function        | Chap18 Impl                                    | Chap19 Impl                                                     | Different?  | Matches Chap19.txt?                                    | Verified? |
|----|-----------------|------------------------------------------------|------------------------------------------------------------------|-------------|--------------------------------------------------------|-----------|
| 1  | `set`           | `self.seq.set(index, item)` (in-place)         | `self.seq.set(index, item)` (in-place)                           | Same        | N/A — Eph-only mutation                                | Yes       |
| 2  | `empty`         | `Vec::new()`                                   | `tabulate(closure, 0)`                                           | **Yes**     | **Yes** — 19.1                                         | Yes       |
| 3  | `singleton`     | `push(item)`                                   | `tabulate(clone closure, 1)`                                     | **Yes**     | **Yes** — 19.2                                         | Yes       |
| 4  | `map`           | While loop: `seq.push(f(&a.seq[i]))`           | `tabulate(closure calling f(&a.seq[i]))`                         | **Yes**     | **Yes** — 19.3                                         | Yes       |
| 5  | `append`        | Two while loops: clone from a then b           | `tabulate(select(a,b), \|a\|+\|b\|)`                            | **Yes**     | **Yes** — 19.4                                         | Yes       |
| 6  | `filter`        | While loop: clone if pred true                 | `flatten(map(deflate pred, a))`                                  | **Yes**     | **Yes** — 19.5                                         | Yes       |
| 7  | `update`        | While loop: clone item or a[j]                 | Clone + `set` (O(n) clone, O(1) set)                             | **Yes**     | Eph variant — uses mutation                            | Yes       |
| 8  | `is_empty`      | `self.seq.len() == 0`                          | `self.seq.len() == 0`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 9  | `is_singleton`  | `self.seq.len() == 1`                          | `self.seq.len() == 1`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 10 | `iterate_iter`  | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — iterative variant kept in trait               | Yes       |
| 11 | `iterate`       | *(same as iterate_iter)*                       | Recursive: `iterate f (f(x,a[0])) a[1..\|a\|-1]`               | **Yes**     | **Yes** — 19.8                                         | Yes       |
| 12 | `reduce_iter`   | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — iterative variant kept in trait               | Yes       |
| 13 | `reduce`        | *(same as reduce_iter)*                        | Recursive D&C: `f(reduce f id b, reduce f id c)`                | **Yes**     | **Yes** — 19.9                                         | Yes       |
| 14 | `scan`          | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — should be contraction (19.10)                 | Yes       |
| 15 | `tabulate`      | While loop: `seq.push(f(i))`                   | While loop: `seq.push(f(i))`                                     | Same        | **Yes** — primitive                                    | Yes       |
| 16 | `flatten`       | Nested while loops with clone                  | Nested while loops with clone                                    | Same        | **Yes** — primitive                                    | Yes       |
| 17 | `deflate`       | *(not in Chap18)*                              | `if pred(x) { singleton } else { empty }`                        | **New**     | **Yes** — 19.5 deflate                                 | Yes       |

## ArraySeqMtEph (Multi-threaded Ephemeral)

| #  | Function        | Chap18 Impl                                    | Chap19 Impl                                                     | Different?  | Matches Chap19.txt?                                    | Verified? |
|----|-----------------|------------------------------------------------|------------------------------------------------------------------|-------------|--------------------------------------------------------|-----------|
| 1  | `set`           | `self.seq.set(index, item)` (in-place)         | `self.seq.set(index, item)` (in-place)                           | Same        | N/A — Eph-only mutation                                | Yes       |
| 2  | `empty`         | `Vec::new()`                                   | `tabulate(closure, 0)`                                           | **Yes**     | **Yes** — 19.1                                         | Yes       |
| 3  | `singleton`     | `push(item)`                                   | `tabulate(clone closure, 1)`                                     | **Yes**     | **Yes** — 19.2                                         | Yes       |
| 4  | `map`           | While loop: `seq.push(f(&a.seq[i]))`           | `tabulate(closure calling f(&a.seq[i]))`                         | **Yes**     | **Yes** — 19.3                                         | Yes       |
| 5  | `append`        | Two while loops: clone from a then b           | `tabulate(select(a,b), \|a\|+\|b\|)`                            | **Yes**     | **Yes** — 19.4                                         | Yes       |
| 6  | `filter`        | While loop: clone if pred true                 | `flatten(map(deflate pred, a))`                                  | **Yes**     | **Yes** — 19.5                                         | Yes       |
| 7  | `update`        | While loop: clone item or a[j]                 | Clone + `set` (O(n) clone, O(1) set)                             | **Yes**     | Eph variant — uses mutation                            | Yes       |
| 8  | `is_empty`      | `self.seq.len() == 0`                          | `self.seq.len() == 0`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 9  | `is_singleton`  | `self.seq.len() == 1`                          | `self.seq.len() == 1`                                            | Same        | **Yes** — 19.7                                         | Yes       |
| 10 | `iterate_iter`  | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — iterative variant kept in trait               | Yes       |
| 11 | `iterate`       | *(same as iterate_iter)*                       | Recursive: `iterate f (f(x,a[0])) a[1..\|a\|-1]`               | **Yes**     | **Yes** — 19.8                                         | Yes       |
| 12 | `reduce_iter`   | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — iterative variant kept in trait               | Yes       |
| 13 | `reduce`        | *(same as reduce_iter)*                        | Recursive D&C: `f(reduce f id b, reduce f id c)`                | **Yes**     | **Yes** — 19.9                                         | Yes       |
| 14 | `scan`          | While loop left-to-right                       | While loop left-to-right                                         | Same        | **No** — should be contraction (19.10)                 | Yes       |
| 15 | `tabulate`      | While loop: `seq.push(f(i))`                   | While loop: `seq.push(f(i))`                                     | Same        | **Yes** — primitive                                    | Yes       |
| 16 | `flatten`       | Nested while loops with clone                  | Nested while loops with clone                                    | Same        | **Yes** — primitive                                    | Yes       |
| 17 | `deflate`       | *(not in Chap18)*                              | `if pred(x) { singleton } else { empty }`                        | **New**     | **Yes** — 19.5 deflate                                 | Yes       |
| 18 | `map_par`       | D&C fork-join: split, map halves, append       | D&C fork-join: split, map halves, append                         | Same        | Parallel variant via `join`                            | Yes       |
| 19 | `filter_par`    | D&C fork-join: split, filter halves, append    | D&C fork-join: split, filter halves, append                      | Same        | Parallel variant via `join`                            | Yes       |
| 20 | `reduce_par`    | D&C fork-join: split, reduce halves, combine   | D&C fork-join: split, reduce halves, combine                     | Same        | Parallel variant via `join` (requires monoid)          | Yes       |

## Summary

### StPer (Persistent)

- **7 methods differ from Chap18** (empty, singleton, map, append, filter, update, reduce) — all use Chap19 compositional/recursive algorithms.
- **1 method is new** (deflate) — Chap19 Algorithm 19.5.
- **2 methods renamed** (iterate → iterate_iter, reduce → reduce_iter) — iterative variants kept alongside recursive versions.
- **1 method does not match Chap19.txt** (scan) — still uses iterative Chap18-style loop.
- **All methods verified.** 1212 total verifications.

### StEph (Ephemeral)

- Same Chap19 algorithms as StPer, plus `set(&mut self, ...)` for in-place mutation.
- **`update` uses clone + set** instead of tabulate — the Eph mutation advantage (O(1) set vs O(n) tabulate body).
- **1 method does not match Chap19.txt** (scan) — same as StPer.
- **All methods verified.** 1253 total verifications (StPer + StEph combined).

### MtEph (Multi-threaded Ephemeral)

- Same Chap19 algorithms as StEph for sequential methods, plus 3 parallel methods via D&C fork-join.
- **Parallel methods**: `map_par`, `filter_par`, `reduce_par` — all use `join` for fork-join parallelism with work-stealing scheduler.
- **`reduce_par` requires monoid** — associativity and identity proven via `spec_monoid` and `lemma_monoid_fold_left`.
- **`update` uses clone + set** — same ephemeral advantage as StEph.
- **1 method does not match Chap19.txt** (scan) — same as StPer/StEph.
- **All methods verified.** 1298 total verifications (StPer + StEph + MtEph combined).

## Structural differences from Chap18

| #  | Aspect       | Chap18                                                       | Chap19                                                     |
|----|--------------|--------------------------------------------------------------|------------------------------------------------------------|
| 1  | Traits       | `BaseTrait` + `RedefinableTrait` (two traits)                | Single trait per variant (`StPerTrait`, `StEphTrait`, `MtEphTrait`) |
| 2  | Impl blocks  | Two (one per trait)                                          | One (trait) + one (bare impl for par methods/lemmas)       |
| 3  | Struct       | Same `ArraySeq{StPer,StEph,MtEph}S<T>` definition           | Same                                                       |
| 4  | Specs        | Identical requires/ensures                                   | `singleton`, `iterate`, `reduce` add `Clone+Eq+obeys_feq_clone` for recursive subseq |
| 5  | View impl    | Same                                                         | Same                                                       |
| 6  | Iterators    | Same                                                         | Same                                                       |
| 7  | Derive impls | Same                                                         | Same                                                       |
| 8  | Par methods  | In bare `impl` block (Chap18 MtEph)                         | In bare `impl` block (Chap19 MtEph) — same structure      |
