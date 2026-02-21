<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chap02–Chap41 Proof Hole Fixes (Agent1 Scope)

Veracity run on each directory. Verusification is highest priority.

## Proposed Fixes Table

| # | Sev | Chapter | File | Issue | Fix |
|---|-----|---------|------|-------|-----|
| 1 | high | Chap02 | HFSchedulerMtEph.rs | struct/enum outside verus! | Move PoolState, TaskState inside verus! |
| 2 | high | Chap26 | ETSPStEph.rs | bare_impl Point | Move Point helpers into ETSPStTrait |
| 3 | high | Chap26 | ETSPMtEph.rs | bare_impl Point | Move Point helpers into ETSPMtTrait |
| 4 | high | Chap38 | BSTParaStEph.rs | dummy RwLockPredicate | Replace inv=true with real BST inv |
| 5 | high | Chap38 | BSTParaMtEph.rs | dummy RwLockPredicate | Replace inv=true with real BST inv |
| 6 | med | Chap26 | ETSPStEph.rs | 2 assume() in lemma_combined_cycle | Prove f64 mod/point_eq chain |
| 7 | med | Chap26 | ETSPMtEph.rs | 2 assume() in lemma_combined_cycle | Prove f64 mod/point_eq chain |
| 8 | med | Chap26 | ETSP* | 2 external_body sort_and_split, find_best_swap | Verify or document f64/vec |
| 9 | med | Chap35 | OrderStatSelectMtEph.rs | 1 external_body parallel_three_way_partition | Verify partition logic |
| 10 | med | Chap35 | OrderStatSelectMtPer.rs | 1 external_body parallel_three_way_partition | Verify partition logic |
| 11 | med | Chap36 | QuickSortMtEphSlice.rs | 1 external_body quick_sort_mt_random | Verify random pivot test |
| 12 | med | Chap37 | AVLTreeSeq.rs | 15+ assume, 12 external_body | Close spec_wf, size_link, update_meta |
| 13 | med | Chap37 | BSTSplayStEph.rs | 5 assume, 6 external_body | Close height_link, update, insert |
| 14 | med | Chap37 | AVLTreeSeqMtPer.rs | 3 assume, 11 external_body | Same pattern as AVLTreeSeq |
| 15 | med | Chap39 | BSTSetTreapMtEph.rs | 14 external_body | Verify treap ops |
| 16 | med | Chap39 | BSTTreapStEph.rs | 3 assume, 7 external_body | Close height_link, update |
| 17 | med | Chap40 | BSTReducedStEph.rs | 1 assume, 11 external_body | Verify values, etc. |
| 18 | med | Chap40 | BSTSizeStEph.rs | 1 assume, 8 external_body | Close height_link, update_size |
| 19 | med | Chap41 | ArraySetStEph.rs | 22 assume (spec_wf) | Prove insert postcondition |
| 20 | med | Chap41 | AVLTreeSetStEph.rs | 22 assume | Prove spec_wf in ops |
| 21 | low | Chap05 | MappingStEph, SetStEph, SetMtEph | assume_eq_clone | Verus workaround, info |
| 22 | low | Chap17 | MathSeq.rs | assume_eq_clone | Verus workaround, info |
| 23 | low | Chap18–23 | ArraySeq*, LinkedList*, PrimTreeSeq* | assume_eq_clone | Verus workaround, info |

## Severity Legend

| Sev | Meaning |
|-----|---------|
| high | Structural: struct/enum outside verus!, bare_impl, dummy predicate |
| med | Proof gaps: assume(), external_body on algorithmic logic |
| low | Accepted: assume_eq_clone (Verus limitation), thread-join assume(false) |

## Verusification Status (Chap02–41)

All files in scope have `verus!` blocks. No `not_verusified` in current run.

## Accepted (Do Not Fix)

- **Chap11** assume(false) in thread join — assume-false-diverge idiom
- **Chap12** Exercise12_1, Exercise12_5 — lock-free/unsafe, expected external_body
- **assume_eq_clone_workaround** — Verus BUG, info only

## Priority Order

1. Chap38 RwLockPredicate (enables future BST verification)
2. Chap02 struct/enum inside verus!
3. Chap26 bare_impl + assume in ETSP
4. Chap37 AVLTreeSeq, BSTSplay (most assume + external_body)
5. Chap41 ArraySetStEph, AVLTreeSetStEph (spec_wf assumes)
6. Chap35, Chap36, Chap39, Chap40 external_body
