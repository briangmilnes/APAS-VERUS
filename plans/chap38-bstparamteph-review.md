# Chap38 BSTParaMtEph Review Against Prose

Date: 2026-03-11
Reviewer: Claude Opus 4.6

Standard: `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
Prose: `prompts/Chap38.txt`
Files: `src/Chap38/BSTParaMtEph.rs`, `src/Chap38/BSTParaStEph.rs`

## Prose Inventory

| # | Prose Item | Type |
|---|-----------|------|
| 1 | Data Type 38.1 | ADT: K, T, E=Exposed, size, expose, joinMid |
| 2 | Algorithm 38.2 | empty, singleton, joinM |
| 3 | Algorithm 38.3 | split |
| 4 | Algorithm 38.4 | minKey, joinPair |
| 5 | Algorithm 38.5 | insert, delete (via split) |
| 6 | Algorithm 38.6 | union (parallel `||`) |
| 7 | Algorithm 38.7 | intersect (parallel `||`) |
| 8 | Algorithm 38.8 | difference (parallel `||`) |
| 9 | Algorithm 38.9 | filter (parallel `||`) |
| 10 | Algorithm 38.10 | reduce (parallel `||`) |
| 11 | Cost Spec 38.11 | Work/Span table |

## The Fine-Grained Locking Question

APAS says nothing about locking. The prose uses `||` to mean "run in parallel" — a
cost-model notation, not a threading prescription. The algorithms are clean
divide-and-conquer: split the tree, recurse on independent subtrees in parallel,
join the results. No shared mutable state during parallel execution.

Our code uses `Arc<RwLock<...>>` per tree node. This is NOT fine-grained locking.
It is a Verus limitation: you cannot dereference `Arc<T>` in Verus without going
through `Arc<RwLock<T>>`. Every tree node wraps its children in `Arc<RwLock<...>>`
just to read them. The RwLock is never used for concurrent write coordination — it
is a read-through mechanism.

The coarse RwLock standard does not apply here. BSTParaMtEph is not wrapping a
single inner struct in a coarse lock. The tree's structure IS Arcs — each node is
an `Arc<RwLock<Option<Box<NodeInner<T>>>>>`. Parallel workers share read access to
subtrees via Arc clone + acquire_read. This is structural sharing, not locking.

## Code vs Prose

| # | Function | Prose | St impl | Mt impl | Mt parallel? |
|---|----------|-------|---------|---------|-------------|
| 1 | new | 38.2 | verified | external_body | N/A |
| 2 | singleton | 38.2 | verified | external_body | N/A |
| 3 | expose | 38.1 | external_body | external_body | N/A |
| 4 | join_mid | 38.1 | verified | external_body | N/A |
| 5 | size | 38.1 | verified | external_body | N/A |
| 6 | is_empty | — | verified | external_body | N/A |
| 7 | find | 38.3 | verified | external_body | No (sequential) |
| 8 | split | 38.3 | verified | external_body | No (sequential) |
| 9 | insert | 38.5 | external_body | external_body | No |
| 10 | delete | 38.5 | external_body | external_body | No |
| 11 | min_key | 38.4 | verified | missing | — |
| 12 | join_pair | 38.4 | verified | external_body | No |
| 13 | union | 38.6 | external_body, seq | external_body, par | Yes (ParaPair!) |
| 14 | intersect | 38.7 | external_body, seq | external_body, par | Yes (ParaPair!) |
| 15 | difference | 38.8 | external_body, seq | external_body, par | Yes (ParaPair!) |
| 16 | filter | 38.9 | verified, seq | external_body, par | Yes (ParaPair!) |
| 17 | reduce | 38.10 | verified, seq | external_body, par | Yes (ParaPair!) |
| 18 | in_order | — | verified | external_body | No (sequential) |
| 19 | collect_in_order | — | verified | free fn, seq | No |

## Key Findings

1. Mt is 100% external_body — zero verified functions. Every trait method is
   external_body. The View itself is external_body returning `Set::empty()` (a
   specification lie).

2. St has good verification — split, find, join_pair, min_key, filter, reduce are
   all verified with real proofs. Union/intersect/difference are external_body
   even in St.

3. Mt IS genuinely parallel where APAS says parallel — union, intersect,
   difference, filter, reduce all use `ParaPair!`. This matches the prose's `||`
   notation.

4. `Arc<RwLock>` per node is a Verus artifact, not fine-grained locking. APAS does
   not discuss locking at all. The Arc is needed because parallel workers need
   shared read access to tree subtrees, and Verus requires RwLock to dereference
   Arc. No thread ever write-locks a subtree node during the parallel algorithms —
   they build new trees.

5. The insert/delete pattern is odd — they split, rebuild, then acquire_write on
   the root to swap in the new tree. This is mutation through interior mutability
   (`Arc<RwLock>`), which is why insert/delete have no `old(self)` specs.

## Should BSTParaMtEph Be in the Coarse RwLock Plan?

No. It does not fit the pattern. The coarse RwLock standard is for collections
that wrap a single St struct in one lock. BSTParaMtEph is a tree whose nodes ARE
Arcs — the tree structure is the sharing mechanism. It should stay excluded from
the coarse RwLock migration plan.

The real question is whether the `Arc<RwLock>` per node could be simplified to just
`Arc<Box<...>>` with an external_body deref helper, but that is a separate concern
from the coarse RwLock standard.

## BSTParaTreapMtEph (Chap39)

Same analysis applies. BSTParaTreapMtEph is the treap instantiation of the same
parametric BST pattern. It uses `Arc<RwLock>` per node for the same Verus reasons.
Not fine-grained locking, not a candidate for the coarse RwLock standard.
