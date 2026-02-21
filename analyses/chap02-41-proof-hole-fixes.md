<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap02–Chap41 Proof Hole Fixes (Agent1 Scope)

**Date:** 2026-02-19  
**Scope:** Chap02–Chap41 only (agent1 when multi-agent)  
**Priority:** Verusification — Prove Big Or Go Home

Veracity proof hole detector run on each directory. Table ordered by severity, then chapter.

## Proposed Fixes Table

| # | Sev | Chapter | File | Issue | Fix |
|---|:---:|---------|------|-------|-----|
| 1 | crit | Chap41 | 7 modules | 143 holes (100 assume, 43 ext_body) | Prove spec_wf, close assumes |
| 2 | crit | Chap37 | 7 modules | 90 holes (37 assume, 51 ext_body) | Close AVL/Splay/Treap specs |
| 3 | high | Chap40 | BSTKeyValue, Reduced, Size | 30 holes (4 assume, 26 ext_body) | Verify in_order, split_rank |
| 4 | high | Chap39 | BSTSetTreap, BSTTreap* | 26 holes (3 assume, 23 ext_body) | Verify treap ops |
| 5 | high | Chap12 | Exercise12_1, 12_5 | 18 holes (4 unsafe, 13 ext_body) | Lock-free expected; doc |
| 6 | high | Chap02 | HFSchedulerMtEph | 8 holes + 2 struct outside | Move PoolState, TaskState in |
| 7 | med | Chap26 | ETSPStEph, ETSPMtEph | 4 external_body | sort_and_split, find_best_swap |
| 8 | med | Chap35 | OrderStatSelectMt* | 2 external_body | parallel_three_way_partition |
| 9 | med | Chap18 | ArraySeq* | 2 external + 9 bare_impl | Iterator/Seq specs |
| 10 | med | Chap38 | BSTPara* | 2 verus_rwlock_ext_body | RwLock::new limitation |
| 11 | low | Chap11 | FibonacciMt* | 6 assume(false) | Thread-join idiom (OK) |
| 12 | low | Chap05,17,19,23 | assume_eq_clone | Verus workaround | Info only |

## Severity Legend

| Sev | Meaning |
|:---:|---------|
| crit | 50+ holes; blocks downstream verification |
| high | 10+ holes or structural (struct outside, unsafe) |
| med | 1–10 holes; verifiable with effort |
| low | Accepted idiom or Verus limitation |

## Summary by Chapter

| # | Chapter | Holed | Holes | Clean | Notes |
|---|---------|:-----:|:-----:|:-----:|-------|
| 1 | Chap02 | 1 | 8 | 1 | HFScheduler threading |
| 2 | Chap03 | 0 | 0 | 1 | Clean |
| 3 | Chap05 | 0 | 0 | 5 | assume_eq workaround |
| 4 | Chap06 | 0 | 0 | 20 | Clean |
| 5 | Chap11 | 3 | 6 | 2 | assume(false) join OK |
| 6 | Chap12 | 2 | 18 | 1 | Lock-free exercises |
| 7 | Chap17 | 0 | 0 | 1 | Clean |
| 8 | Chap18 | 1 | 2 | 6 | external + bare_impl |
| 9 | Chap19 | 0 | 0 | 4 | Clean |
| 10 | Chap21 | 0 | 0 | 12 | Clean |
| 11 | Chap23 | 0 | 0 | 2 | Clean |
| 12 | Chap26 | 2 | 4 | 6 | ETSP f64 external_body |
| 13 | Chap27 | 0 | 0 | 4 | Clean |
| 14 | Chap28 | 0 | 0 | 11 | Clean |
| 15 | Chap35 | 2 | 2 | 2 | parallel_three_way_partition |
| 16 | Chap36 | 0 | 0 | 3 | Clean |
| 17 | Chap37 | 7 | 90 | 12 | AVL, Splay, Treap |
| 18 | Chap38 | 0 | 0 | 2 | verus_rwlock only |
| 19 | Chap39 | 3 | 26 | 1 | Treap external_body |
| 20 | Chap40 | 3 | 30 | 0 | All holed |
| 21 | Chap41 | 7 | 143 | 0 | All holed |

## Accepted (Do Not Fix)

- **Chap11** assume(false) in thread join — assume-false-diverge idiom
- **Chap12** Exercise12_1, Exercise12_5 — lock-free/unsafe, expected
- **assume_eq_clone** — Verus PartialEq workaround, info only

## Verusification Priority Order

1. **Chap41** — 143 holes; blocks Chap42, Chap43, Chap52
2. **Chap37** — 90 holes; blocks Chap41 AVLTreeSet
3. **Chap40** — 30 holes; BST augmentations
4. **Chap39** — 26 holes; Treap
5. **Chap02** — struct/enum inside verus!
6. **Chap26** — 4 external_body (f64)
7. **Chap35** — 2 external_body (partition)
